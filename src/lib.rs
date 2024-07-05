#![cfg_attr(not(feature = "std"), no_std)]

mod elf_types;

use core::marker::PhantomData;

use binary_serde::{BinaryDeserializerFromBufSafe, Endianness};
pub use elf_types::*;
use thiserror_no_std::Error;

const SHN_UNDEF: u16 = 0;
const SHN_ABS: u16 = 0xfff1;

#[derive(Debug, Clone)]
pub struct ElfParser<'a> {
    data: DebugIgnore<&'a [u8]>,
    file_info: ElfFileInfo,
}
impl<'a> ElfParser<'a> {
    pub fn new(data: &'a [u8]) -> Result<Self> {
        // first extract the ident array to get some information about the binary and to make sure that it is valid
        let mut ident_deserializer = BinaryDeserializerFromBufSafe::new(
            data,
            // endianness doesn't matter here because we only use this deserialize to parse the elf ident, which is basically just
            // a byte array.
            Endianness::Big,
        );
        let ident: ElfIdent = ident_deserializer.deserialize()?;

        // verify elf magic
        if ident.header.magic != ELF_MAGIC {
            return Err(Error::ElfMagicIsMissing);
        }

        let mut parser = Self {
            data: data.into(),
            file_info: ElfFileInfo {
                endianness: ident.header.endianness.into(),
                bit_length: ident.header.bit_size,
                os_abi: ident.header.os_abi,

                // to know the architechture we must first parse the header, but to parse the header we need to have an instance of
                // a parser, so we use a placeholder here which will be filled later. the architechture shouldn't affect the way
                // the header is parsed, so it's ok to parse the header while the parser uses a placeholder architechture value.
                arch: elf_types::Architechture::None,
            },
        };

        // read the real architechture of the elf file
        parser.file_info.arch = *parser.header()?.arch();

        Ok(parser)
    }

    pub fn data(&self) -> &'a [u8] {
        &self.data
    }

    fn deserializer(&self) -> BinaryDeserializerFromBufSafe<'a> {
        BinaryDeserializerFromBufSafe::new(self.data.0, self.file_info.endianness)
    }

    fn deserializer_at_offset(&self, offset: usize) -> BinaryDeserializerFromBufSafe<'a> {
        let mut deserializer = self.deserializer();
        deserializer.set_position(offset);
        deserializer
    }

    pub fn file_info(&self) -> ElfFileInfo {
        self.file_info
    }

    pub fn header(&self) -> Result<ElfHeader> {
        let mut deserializer = self.deserializer();
        Ok(ElfHeader::deserialize(&mut deserializer, &self, ())?)
    }

    fn records_table<T: VariantStructBinarySerde<'a>>(
        &self,
        start_offset: usize,
        specified_record_len: u64,
        records_amount: usize,
        record_name: &'static str,
        context: T::Context,
    ) -> Result<ElfRecordsTable<'a, T>> {
        let record_len = T::record_len(&self.file_info);
        if specified_record_len != record_len as u64 {
            return Err(Error::UnexpectedEntrySize {
                record_name,
                expected_size: record_len as u64,
                specified_size: specified_record_len,
            });
        }
        Ok(ElfRecordsTable {
            parser: self.clone(),
            table_start_offset: start_offset,
            table_records_amount: records_amount,
            record_name,
            record_len,
            context,
            phantom: PhantomData,
        })
    }

    pub fn program_headers(&self) -> Result<ProgramHeaders<'a>> {
        let hdr = self.header()?;
        self.records_table(
            hdr.program_headers_off() as usize,
            hdr.program_header_entry_size() as u64,
            hdr.program_headers_amount() as usize,
            "program header",
            (),
        )
    }

    pub fn section_headers(&self) -> Result<SectionHeaders<'a>> {
        let hdr = self.header()?;
        self.records_table(
            hdr.section_headers_off() as usize,
            hdr.section_header_entry_size() as u64,
            hdr.section_headers_amount() as usize,
            "section header",
            (),
        )
    }

    fn get_offset_range_content(
        &self,
        offset: usize,
        len: usize,
        offset_range_of_what: &'static str,
    ) -> Result<&'a [u8]> {
        let offset_range = offset..offset + len;
        self.data
            .get(offset_range.clone())
            .ok_or(Error::OffsetRangeOutOfBounds {
                offset_range,
                file_len: self.data.len(),
                offset_range_of_what,
            })
    }

    pub fn section_names_string_table(&self) -> Result<StringTable<'a>> {
        let hdr = self.header()?;
        let section_names_section_index = hdr.section_names_section_index();
        if section_names_section_index == SHN_UNDEF {
            return Err(Error::NoSectionNamesStringTable);
        }
        match self
            .section_headers()?
            .get(section_names_section_index as usize)?
            .data()?
        {
            SectionData::StringTable(string_table) => Ok(string_table),
            _ => Err(Error::SectionNamesSectionIsNotAStringTable),
        }
    }
}

impl<'a> SectionHeaderRef<'a> {
    pub fn content(&self) -> Result<&'a [u8]> {
        self.parser.get_offset_range_content(
            self.offset() as usize,
            self.size() as usize,
            "section header content",
        )
    }

    pub fn name(&self) -> Result<&'a str> {
        self.parser
            .section_names_string_table()?
            .string_at_offset(self.name_offset() as usize, "section name")
    }

    fn generic_rel_section_build(
        &self,
        entries: GenericRelEntries<'a>,
    ) -> Result<GenericRelSection<'a>> {
        Ok(GenericRelSection {
            entries,
            relocated_section: self.parser.section_headers()?.get(self.info() as usize)?,
            parser: self.parser.clone(),
            linked_symbol_table_index: self.link() as usize,
        })
    }

    fn parse_as_symbol_table(&self) -> Result<SymbolEntries<'a>> {
        self.parser.records_table(
            self.offset() as usize,
            self.entry_size(),
            (self.size() / self.entry_size()) as usize,
            "symbol table entry",
            SymbolRefContext {
                string_table: match self
                    .parser
                    .section_headers()?
                    .get(self.link() as usize)?
                    .data()?
                {
                    SectionData::StringTable(string_table) => string_table,
                    _ => {
                        return Err(Error::LinkedSectionOfSymbolTableSectionIsNotAStringTable {
                            linked_section_index: self.link() as usize,
                        })
                    }
                },
            },
        )
    }

    pub fn data(&self) -> Result<SectionData<'a>> {
        match self.ty() {
            SectionHeaderType::Strtab => Ok(SectionData::StringTable(StringTable {
                content: self.content()?.into(),
            })),
            SectionHeaderType::Rela => Ok(SectionData::RelocationSection(
                self.generic_rel_section_build(GenericRelEntries::RelaEntries(
                    self.parser.records_table(
                        self.offset() as usize,
                        self.entry_size(),
                        (self.size() / self.entry_size()) as usize,
                        "relocation entry with addend",
                        (),
                    )?,
                ))?,
            )),
            SectionHeaderType::Rel => Ok(SectionData::RelocationSection(
                self.generic_rel_section_build(GenericRelEntries::RelEntries(
                    self.parser.records_table(
                        self.offset() as usize,
                        self.entry_size(),
                        (self.size() / self.entry_size()) as usize,
                        "relocation entry",
                        (),
                    )?,
                ))?,
            )),
            SectionHeaderType::Symtab => {
                Ok(SectionData::SymbolTable(self.parse_as_symbol_table()?))
            }
            SectionHeaderType::Dynsym => Ok(SectionData::DynamicSymbolTable(
                self.parse_as_symbol_table()?,
            )),
            _ => Ok(SectionData::UnknownSectionType),
        }
    }
}

#[derive(Debug, Clone)]
pub enum SectionData<'a> {
    StringTable(StringTable<'a>),
    SymbolTable(SymbolEntries<'a>),
    DynamicSymbolTable(SymbolEntries<'a>),
    RelocationSection(GenericRelSection<'a>),
    UnknownSectionType,
}

pub type SymbolEntries<'a> = ElfRecordsTable<'a, SymbolRef<'a>>;
pub type SymbolEntriesIter<'a> = ElfRecordsTableIter<'a, SymbolRef<'a>>;

impl<'a> SymbolRef<'a> {
    pub fn name(&self) -> Result<&'a str> {
        match self.info().ty {
            elf_types::SymbolType::Section => self
                .section()?
                .as_optional_section()
                .ok_or(Error::SectionSymbolHasNoSectionIndex)?
                .name(),
            _ => self
                .context
                .string_table
                .string_at_offset(self.name_index_in_string_table() as usize, "symbol name"),
        }
    }

    pub fn section(&self) -> Result<SymbolSection<'a>> {
        match self.related_section_index() {
            SHN_UNDEF => Ok(SymbolSection::UndefinedSection),
            SHN_ABS => Ok(SymbolSection::AbsoluteSymbol),
            section_index => Ok(SymbolSection::Section(
                self.parser.section_headers()?.get(section_index as usize)?,
            )),
        }
    }
}

pub enum SymbolSection<'a> {
    /// the symbol is not defined relative to any section
    UndefinedSection,

    /// the symbol is absolute.
    AbsoluteSymbol,

    Section(SectionHeaderRef<'a>),
}
impl<'a> SymbolSection<'a> {
    pub fn as_optional_section(self) -> Option<SectionHeaderRef<'a>> {
        match self {
            SymbolSection::UndefinedSection => None,
            SymbolSection::AbsoluteSymbol => None,
            SymbolSection::Section(section) => Some(section),
        }
    }
}

#[derive(Debug, Clone)]
pub struct GenericRelSection<'a> {
    parser: ElfParser<'a>,
    pub entries: GenericRelEntries<'a>,
    pub linked_symbol_table_index: usize,
    pub relocated_section: SectionHeaderRef<'a>,
}
impl<'a> GenericRelSection<'a> {
    pub fn linked_symbol_table(&self) -> Result<SymbolEntries<'a>> {
        match self
            .parser
            .section_headers()?
            .get(self.linked_symbol_table_index)?
            .data()?
        {
            SectionData::SymbolTable(symbols) | SectionData::DynamicSymbolTable(symbols) => {
                Ok(symbols)
            }
            _ => Err(Error::LinkedSectionOfRelocationSectionIsNotASymbolTable {
                linked_section_index: self.linked_symbol_table_index as usize,
            }),
        }
    }
}

#[derive(Debug, Clone)]
pub enum GenericRelEntries<'a> {
    RelEntries(RelEntries<'a>),
    RelaEntries(RelaEntries<'a>),
}
impl<'a> GenericRelEntries<'a> {
    pub fn get(&self, index: usize) -> Result<GenericRel> {
        match self {
            GenericRelEntries::RelEntries(x) => Ok(x.get(index)?.into()),
            GenericRelEntries::RelaEntries(x) => Ok(x.get(index)?.into()),
        }
    }

    pub fn iter(&self) -> GenericRelEntriesIter<'a> {
        match self {
            GenericRelEntries::RelEntries(x) => GenericRelEntriesIter::RelEntriesIter(x.iter()),
            GenericRelEntries::RelaEntries(x) => GenericRelEntriesIter::RelaEntriesIter(x.iter()),
        }
    }
}
impl<'a> IntoIterator for GenericRelEntries<'a> {
    type Item = Result<GenericRel>;

    type IntoIter = GenericRelEntriesIter<'a>;

    fn into_iter(self) -> Self::IntoIter {
        self.iter()
    }
}
impl<'a, 'r> IntoIterator for &'r GenericRelEntries<'a> {
    type Item = Result<GenericRel>;

    type IntoIter = GenericRelEntriesIter<'a>;

    fn into_iter(self) -> Self::IntoIter {
        self.iter()
    }
}

#[derive(Debug, Clone)]
pub enum GenericRelEntriesIter<'a> {
    RelEntriesIter(RelEntriesIter<'a>),
    RelaEntriesIter(RelaEntriesIter<'a>),
}
impl<'a> Iterator for GenericRelEntriesIter<'a> {
    type Item = Result<GenericRel>;

    fn next(&mut self) -> Option<Self::Item> {
        match self {
            GenericRelEntriesIter::RelEntriesIter(x) => Some(x.next()?.map(|rel| rel.into())),
            GenericRelEntriesIter::RelaEntriesIter(x) => Some(x.next()?.map(|rel| rel.into())),
        }
    }
}

pub type RelaEntries<'a> = ElfRecordsTable<'a, Rela>;
pub type RelaEntriesIter<'a> = ElfRecordsTableIter<'a, Rela>;

pub type RelEntries<'a> = ElfRecordsTable<'a, Rel>;
pub type RelEntriesIter<'a> = ElfRecordsTableIter<'a, Rel>;

#[derive(Debug, Clone)]
pub struct StringTable<'a> {
    content: DebugIgnore<&'a [u8]>,
}
impl<'a> StringTable<'a> {
    pub fn string_at_offset(&self, offset: usize, offset_of_what: &'static str) -> Result<&'a str> {
        let slice = self
            .content
            .get(offset..)
            .ok_or(Error::StringOffsetOutOfBoundsOfStrtab {
                offset,
                strtab_len: self.content.len(),
                offset_of_what,
            })?;
        let cstr = core::ffi::CStr::from_bytes_until_nul(slice)
            .map_err(|_| Error::StringTableNotNullTerminated)?;
        cstr.to_str().map_err(|_| Error::StringTableInvalidUtf8)
    }
}

impl<'a> ProgramHeaderRef<'a> {
    pub fn content_in_file(&self) -> Result<&'a [u8]> {
        self.parser.get_offset_range_content(
            self.offset() as usize,
            self.size_in_file() as usize,
            "program header content",
        )
    }
}

pub type ProgramHeaders<'a> = ElfRecordsTable<'a, ProgramHeaderRef<'a>>;
pub type ProgramHeadersIter<'a> = ElfRecordsTableIter<'a, ProgramHeaderRef<'a>>;

pub type SectionHeaders<'a> = ElfRecordsTable<'a, SectionHeaderRef<'a>>;
pub type SectionHeadersIter<'a> = ElfRecordsTableIter<'a, SectionHeaderRef<'a>>;

#[derive(Debug, Clone)]
pub struct ElfRecordsTable<'a, T: VariantStructBinarySerde<'a>> {
    parser: ElfParser<'a>,
    table_start_offset: usize,
    table_records_amount: usize,
    record_name: &'static str,
    record_len: usize,
    phantom: PhantomData<T>,
    context: T::Context,
}
impl<'a, T: VariantStructBinarySerde<'a>> ElfRecordsTable<'a, T> {
    pub fn len(&self) -> usize {
        self.table_records_amount
    }
    pub fn get(&self, index: usize) -> Result<T> {
        if index > self.table_records_amount {
            return Err(Error::RecordIndexOutOfBounds {
                record_name: self.record_name,
                index,
                records_amount: self.table_records_amount,
            });
        }
        let mut deserializer = self
            .parser
            .deserializer_at_offset(self.table_start_offset + self.record_len * index);
        Ok(T::deserialize(
            &mut deserializer,
            &self.parser,
            self.context.clone(),
        )?)
    }

    pub fn iter(&self) -> ElfRecordsTableIter<'a, T> {
        ElfRecordsTableIter {
            parser: self.parser.clone(),
            table_records_amount: self.table_records_amount,
            cur_record_index: 0,
            deserializer: self
                .parser
                .deserializer_at_offset(self.table_start_offset)
                .into(),
            context: self.context.clone(),
            phantom: PhantomData,
        }
    }
}
impl<'a, T: VariantStructBinarySerde<'a>> IntoIterator for ElfRecordsTable<'a, T> {
    type Item = Result<T>;

    type IntoIter = ElfRecordsTableIter<'a, T>;

    fn into_iter(self) -> Self::IntoIter {
        self.iter()
    }
}

impl<'r, 'a, T: VariantStructBinarySerde<'a>> IntoIterator for &'r ElfRecordsTable<'a, T> {
    type Item = Result<T>;

    type IntoIter = ElfRecordsTableIter<'a, T>;

    fn into_iter(self) -> Self::IntoIter {
        self.iter()
    }
}

#[derive(Debug, Clone)]
pub struct ElfRecordsTableIter<'a, T: VariantStructBinarySerde<'a>> {
    parser: ElfParser<'a>,
    table_records_amount: usize,
    cur_record_index: usize,
    deserializer: DebugIgnore<BinaryDeserializerFromBufSafe<'a>>,
    context: T::Context,
    phantom: PhantomData<T>,
}
impl<'a, T: VariantStructBinarySerde<'a>> Iterator for ElfRecordsTableIter<'a, T> {
    type Item = Result<T>;

    fn next(&mut self) -> Option<Self::Item> {
        if self.cur_record_index >= self.table_records_amount {
            return None;
        }
        self.cur_record_index += 1;
        Some(
            T::deserialize(&mut self.deserializer, &self.parser, self.context.clone())
                .map_err(|e| e.into()),
        )
    }
}

pub trait VariantStructBinarySerde<'a>: Sized {
    type Context: Clone;
    fn deserialize(
        deserializer: &mut BinaryDeserializerFromBufSafe<'a>,
        parser: &ElfParser<'a>,
        context: Self::Context,
    ) -> core::result::Result<Self, binary_serde::BinarySerdeBufSafeError>;
    fn serialize(&self, buf: &mut [u8], endianness: Endianness);
    fn record_len(file_info: &ElfFileInfo) -> usize;
}

#[derive(Debug, Error)]
pub enum Error {
    #[error("failed to deserialize binary data from elf file content")]
    BinaryDeserializeError(
        #[from]
        #[source]
        binary_serde::BinarySerdeBufSafeError,
    ),

    #[error("elf magic is missing")]
    ElfMagicIsMissing,

    #[error(
        "{record_name} {index} is out of bounds of {record_name} table with {records_amount} {record_name}s"
    )]
    RecordIndexOutOfBounds {
        record_name: &'static str,
        index: usize,
        records_amount: usize,
    },

    #[error(
        "file offset range {offset_range:?} of {offset_range_of_what} is out of bounds of file with length {file_len}"
    )]
    OffsetRangeOutOfBounds {
        offset_range: core::ops::Range<usize>,
        file_len: usize,
        offset_range_of_what: &'static str,
    },

    #[error("string offset {offset} of {offset_of_what} is out of bounds of string table with length {strtab_len}")]
    StringOffsetOutOfBoundsOfStrtab {
        offset: usize,
        strtab_len: usize,
        offset_of_what: &'static str,
    },

    #[error("string table is not null terminated")]
    StringTableNotNullTerminated,

    #[error("string from string table is not valid utf8")]
    StringTableInvalidUtf8,

    #[error("elf has no section names string table")]
    NoSectionNamesStringTable,

    #[error("section names section is not a string table")]
    SectionNamesSectionIsNotAStringTable,

    #[error("the size of a {record_name} specified in the elf file is {specified_size} but the expected size is {expected_size}")]
    UnexpectedEntrySize {
        record_name: &'static str,
        expected_size: u64,
        specified_size: u64,
    },

    #[error("section with index {linked_section_index} is sepcified as the linked section of a symbol table section but it is not a string table")]
    LinkedSectionOfSymbolTableSectionIsNotAStringTable { linked_section_index: usize },

    #[error("section with index {linked_section_index} is sepcified as the linked section of a relocation section but it is not a symbol table")]
    LinkedSectionOfRelocationSectionIsNotASymbolTable { linked_section_index: usize },

    #[error("symbol of type section has no section index")]
    SectionSymbolHasNoSectionIndex,
}

pub type Result<T> = core::result::Result<T, Error>;

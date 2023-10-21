pub mod elf_types;

use std::marker::PhantomData;

use binary_serde::{BinaryDeserializerFromBufSafe, Endianness};
use elf_types::{
    ArchBitLength, DebugIgnore, ElfFileInfo, ElfHeader, ElfIdent, ProgramHeader, ProgramHeaderRef,
    SectionHeader, SectionHeaderRef, SectionHeaderType, ELF_MAGIC,
};
use thiserror_no_std::Error;

pub const SHN_UNDEF: u16 = 0;

#[derive(Clone)]
pub struct ElfParser<'a> {
    data: &'a [u8],
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

        Ok(Self {
            data,
            file_info: ElfFileInfo {
                endianness: ident.header.endianness.into(),
                bit_length: ident.header.bit_size,
                os_abi: ident.header.os_abi,
            },
        })
    }

    fn deserializer(&self) -> BinaryDeserializerFromBufSafe<'a> {
        BinaryDeserializerFromBufSafe::new(self.data, self.file_info.endianness)
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
        Ok(ElfHeader::deserialize(&mut deserializer, &self)?)
    }

    fn records_table<T: VariantStructBinaryDeserialize<'a>>(
        &self,
        start_offset: usize,
        records_amount: usize,
        record_name: &'static str,
    ) -> ElfRecordsTable<'a, T> {
        ElfRecordsTable {
            parser: self.clone(),
            table_start_offset: start_offset,
            table_records_amount: records_amount,
            record_name,
            record_len: T::record_len(&self.file_info),
            phantom: PhantomData,
        }
    }

    pub fn program_headers(&self) -> Result<ProgramHeaders<'a>> {
        let hdr = self.header()?;
        Ok(self.records_table(
            hdr.program_headers_off() as usize,
            hdr.program_headers_amount() as usize,
            "program header",
        ))
    }

    pub fn section_headers(&self) -> Result<SectionHeaders<'a>> {
        let hdr = self.header()?;
        Ok(self.records_table(
            hdr.section_headers_off() as usize,
            hdr.section_headers_amount() as usize,
            "section header",
        ))
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
        self.section_headers()?
            .get(section_names_section_index as usize)?
            .as_string_table()?
            .ok_or(Error::SectionNamesSectionIsNotAStringTable)
    }
}

impl<'a> SectionHeaderRef<'a> {
    pub fn content(&self) -> Result<&'a [u8]> {
        self.parser.get_offset_range_content(
            self.offset() as usize,
            self.size() as usize,
            "section header",
        )
    }

    pub fn name(&self) -> Result<&str> {
        self.parser
            .section_names_string_table()?
            .string_at_offset(self.name_offset() as usize, "section name")
    }

    pub fn as_string_table(&self) -> Result<Option<StringTable<'a>>> {
        if *self.ty() != SectionHeaderType::Strtab {
            return Ok(None);
        }
        Ok(Some(StringTable {
            content: self.content()?.into(),
        }))
    }
}

#[derive(Clone)]
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
    pub fn content(&self) -> Result<&'a [u8]> {
        self.parser.get_offset_range_content(
            self.offset() as usize,
            self.size_in_file() as usize,
            "program header",
        )
    }
}

pub type ProgramHeaders<'a> = ElfRecordsTable<'a, ProgramHeaderRef<'a>>;
pub type ProgramHeadersIter<'a> = ElfRecordsTableIter<'a, ProgramHeaderRef<'a>>;

pub type SectionHeaders<'a> = ElfRecordsTable<'a, SectionHeaderRef<'a>>;
pub type SectionHeadersIter<'a> = ElfRecordsTableIter<'a, SectionHeaderRef<'a>>;

pub struct ElfRecordsTable<'a, T: VariantStructBinaryDeserialize<'a>> {
    parser: ElfParser<'a>,
    table_start_offset: usize,
    table_records_amount: usize,
    record_name: &'static str,
    record_len: usize,
    phantom: PhantomData<T>,
}
impl<'a, T: VariantStructBinaryDeserialize<'a>> ElfRecordsTable<'a, T> {
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
        Ok(T::deserialize(&mut deserializer, &self.parser)?)
    }

    pub fn iter(&self) -> ElfRecordsTableIter<'a, T> {
        ElfRecordsTableIter {
            parser: self.parser.clone(),
            table_records_amount: self.table_records_amount,
            cur_record_index: 0,
            deserializer: self.parser.deserializer_at_offset(self.table_start_offset),
            phantom: PhantomData,
        }
    }
}
impl<'a, T: VariantStructBinaryDeserialize<'a>> IntoIterator for ElfRecordsTable<'a, T> {
    type Item = Result<T>;

    type IntoIter = ElfRecordsTableIter<'a, T>;

    fn into_iter(self) -> Self::IntoIter {
        self.iter()
    }
}

impl<'r, 'a, T: VariantStructBinaryDeserialize<'a>> IntoIterator for &'r ElfRecordsTable<'a, T> {
    type Item = Result<T>;

    type IntoIter = ElfRecordsTableIter<'a, T>;

    fn into_iter(self) -> Self::IntoIter {
        self.iter()
    }
}

pub struct ElfRecordsTableIter<'a, T: VariantStructBinaryDeserialize<'a>> {
    parser: ElfParser<'a>,
    table_records_amount: usize,
    cur_record_index: usize,
    deserializer: BinaryDeserializerFromBufSafe<'a>,
    phantom: PhantomData<T>,
}
impl<'a, T: VariantStructBinaryDeserialize<'a>> Iterator for ElfRecordsTableIter<'a, T> {
    type Item = Result<T>;

    fn next(&mut self) -> Option<Self::Item> {
        if self.cur_record_index >= self.table_records_amount {
            return None;
        }
        self.cur_record_index += 1;
        Some(T::deserialize(&mut self.deserializer, &self.parser).map_err(|e| e.into()))
    }
}

pub trait VariantStructBinaryDeserialize<'a>: Sized {
    fn deserialize(
        deserializer: &mut BinaryDeserializerFromBufSafe<'a>,
        parser: &ElfParser<'a>,
    ) -> core::result::Result<Self, binary_serde::BinarySerdeBufSafeError>;
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
}

pub type Result<T> = core::result::Result<T, Error>;

use core::marker::PhantomData;

use binary_serde::{BinaryDeserializerFromBufSafe, BinarySerde, Endianness};
use bitflags::bitflags;
use elf_types::{
    ArchBitSize, ElfHeader, ElfHeaderOfBitSize, ElfIdent, OsAbi, ProgramHeader,
    SectionHeaderOfBitSize, SectionHeaderType, SectionIndex, EI_NIDENT, ELF_MAGIC,
};
use thiserror_no_std::Error;

use crate::elf_types::{ProgramHeader32Bit, ProgramHeader64Bit};
mod elf_types;

#[derive(Debug, Clone)]
pub struct ElfParser<'a> {
    data: &'a [u8],
    endianness: Endianness,
    bit_size: ArchBitSize,
    os_abi: OsAbi,
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
            endianness: ident.header.endianness.to_binary_serde_endianness(),
            bit_size: ident.header.bit_size,
            os_abi: ident.header.os_abi,
        })
    }

    fn deserializer(&self) -> BinaryDeserializerFromBufSafe<'a> {
        BinaryDeserializerFromBufSafe::new(self.data, self.endianness)
    }

    pub fn header(&mut self) -> Result<ElfHeader> {
        match self.bit_size {
            ArchBitSize::Arch32Bit => Ok(ElfHeader::B32(self.deserializer().deserialize()?)),
            ArchBitSize::Arch64Bit => Ok(ElfHeader::B64(self.deserializer().deserialize()?)),
        }
    }

    pub fn section_headers(&mut self) -> Result<SectionHeaders<'a>> {
        fn deserialize_section_header_32(
            serde: &mut BinaryDeserializerFromBufSafe,
        ) -> Result<SectionHeader> {
            Ok(SectionHeader::B32(serde.deserialize()?))
        }
        fn deserialize_section_header_64(
            serde: &mut BinaryDeserializerFromBufSafe,
        ) -> Result<SectionHeader> {
            Ok(SectionHeader::B64(serde.deserialize()?))
        }

        let elf_hdr = self.header()?;
        let (record_size, deserialize_fn) = match self.bit_size {
            ArchBitSize::Arch32Bit => (
                SectionHeaderOfBitSize::<u32>::SERIALIZED_SIZE,
                deserialize_section_header_32
                    as fn(&mut BinaryDeserializerFromBufSafe) -> Result<SectionHeader>,
            ),
            ArchBitSize::Arch64Bit => (
                SectionHeaderOfBitSize::<u64>::SERIALIZED_SIZE,
                deserialize_section_header_64
                    as fn(&mut BinaryDeserializerFromBufSafe) -> Result<SectionHeader>,
            ),
        };

        Ok(SectionHeaders(ElfRecordsTable {
            record_size,
            records_amount: elf_hdr.section_headers_amount() as usize,
            deserialize_fn,
            table_offset: elf_hdr.section_headers_off() as usize,
            phantom: PhantomData,
            parser: self.clone(),
        }))
    }

    pub fn program_headers<'p>(&'p mut self) -> Result<ProgramHeaders<'a>> {
        fn deserialize_program_header_32(
            serde: &mut BinaryDeserializerFromBufSafe,
        ) -> Result<ProgramHeader> {
            Ok(ProgramHeader::B32(serde.deserialize()?))
        }
        fn deserialize_program_header_64(
            serde: &mut BinaryDeserializerFromBufSafe,
        ) -> Result<ProgramHeader> {
            Ok(ProgramHeader::B64(serde.deserialize()?))
        }

        let elf_hdr = self.header()?;
        let (record_size, deserialize_fn) = match self.bit_size {
            ArchBitSize::Arch32Bit => (
                ProgramHeader32Bit::SERIALIZED_SIZE,
                deserialize_program_header_32
                    as fn(&mut BinaryDeserializerFromBufSafe) -> Result<ProgramHeader>,
            ),
            ArchBitSize::Arch64Bit => (
                ProgramHeader64Bit::SERIALIZED_SIZE,
                deserialize_program_header_64
                    as fn(&mut BinaryDeserializerFromBufSafe) -> Result<ProgramHeader>,
            ),
        };

        Ok(ProgramHeaders(ElfRecordsTable {
            record_size,
            records_amount: elf_hdr.program_headers_amount() as usize,
            deserialize_fn,
            table_offset: elf_hdr.program_headers_off() as usize,
            phantom: PhantomData,
            parser: self.clone(),
        }))
    }
}

macro_rules! impl_get_raw_passthrough {
    {$field_name: ident, $type: ty} => {
        pub fn $field_name(&self) -> $type {
            self.raw.$field_name()
        }
    };
}

pub struct SectionHeader<'a> {
    raw: elf_types::SectionHeader,
    parser: ElfParser<'a>,
}
impl<'a> SectionHeader<'a> {
    pub fn raw(&self) -> elf_types::SectionHeader {
        self.raw
    }
    pub fn name(&self) -> Result<&'a str> {
        self.parser.header()?.section_header_strings_section_index()
        self.raw.name_offset()
    }
}

pub struct SectionHeaders<'a>(ElfRecordsTable<'a, SectionHeader>);
impl<'a> SectionHeaders<'a> {
    pub fn index(&mut self, index: usize) -> Result<SectionHeader> {
        self.0.index(index)
    }
    pub fn iter(self) -> SectionHeadersIter<'a> {
        self.0.iter()
    }
}
pub type SectionHeadersIter<'a> = ElfRecordsTableIter<'a, SectionHeader>;

pub struct ProgramHeaders<'a>(ElfRecordsTable<'a, ProgramHeader>);
impl<'a> ProgramHeaders<'a> {
    pub fn index(&mut self, index: usize) -> Result<ProgramHeader> {
        self.0.index(index)
    }
    pub fn iter(self) -> ProgramHeadersIter<'a> {
        self.0.iter()
    }
}
pub type ProgramHeadersIter<'a> = ElfRecordsTableIter<'a, ProgramHeader>;

pub struct ElfRecordsTable<'a, E> {
    record_size: usize,
    records_amount: usize,
    parser: ElfParser<'a>,
    deserialize_fn: fn(&mut BinaryDeserializerFromBufSafe, ElfParser) -> Result<E>,
    table_offset: usize,
    phantom: PhantomData<E>,
}
impl<'a, E> ElfRecordsTable<'a, E> {
    pub fn index(&self, index: usize) -> Result<E> {
        if index >= self.records_amount {
            return Err(Error::RecordIndexOutOfBounds {
                index,
                records_amount: self.records_amount,
            });
        }
        let mut deserializer = self.parser.deserializer();
        deserializer.set_position(self.table_offset + index * self.record_size);
        (self.deserialize_fn)(&mut deserializer, self.parser.clone())
    }

    pub fn iter(&self) -> ElfRecordsTableIter<'a, E> {
        let mut deserializer = self.parser.deserializer();
        deserializer.set_position(self.table_offset);
        ElfRecordsTableIter {
            parser: self.parser,
            deserializer,
            deserialize_fn: self.deserialize_fn,
            cur_index: 0,
            records_amount: self.records_amount,
            phantom: PhantomData,
        }
    }
}

pub struct ElfRecordsTableIter<'a, E> {
    parser: ElfParser<'a>,
    deserializer: BinaryDeserializerFromBufSafe<'a>,
    deserialize_fn: fn(&mut BinaryDeserializerFromBufSafe, ElfParser) -> Result<E>,
    cur_index: usize,
    records_amount: usize,
    phantom: PhantomData<E>,
}
impl<'a, E> Iterator for ElfRecordsTableIter<'a, E> {
    type Item = Result<E>;

    fn next(&mut self) -> Option<Self::Item> {
        if self.cur_index >= self.records_amount {
            return None;
        }
        self.cur_index += 1;
        Some((self.deserialize_fn)(
            &mut self.deserializer,
            self.parser.clone(),
        ))
    }
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

    #[error("record index {index} is out of bounds of table with {records_amount} records")]
    RecordIndexOutOfBounds { index: usize, records_amount: usize },
}

pub type Result<T> = core::result::Result<T, Error>;

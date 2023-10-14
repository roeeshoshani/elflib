use core::marker::PhantomData;

use binary_serde::{BinarySerde, BinarySerdeBufSafe, Endianness};
use bitflags::bitflags;
use elf_types::{
    ArchBitSize, ElfHeader, ElfHeaderOfBitSize, ElfIdent, OsAbi, ProgramHeader, SectionHeader,
    SectionHeaderOfBitSize, EI_NIDENT, ELF_MAGIC,
};
use thiserror_no_std::Error;

use crate::elf_types::{ProgramHeader32Bit, ProgramHeader64Bit};
mod elf_types;

pub struct ElfParser<'a> {
    serde: BinarySerdeBufSafe<'a>,
    bit_size: ArchBitSize,
    os_abi: OsAbi,
}
impl<'a> ElfParser<'a> {
    pub fn new(data: &'a mut [u8]) -> Result<Self> {
        // first extract the ident array to get some information about the binary and to make sure that it is valid
        let mut ident_deserializer = BinarySerdeBufSafe::new(
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
            serde: BinarySerdeBufSafe::new(
                data,
                ident.header.endianness.to_binary_serde_endianness(),
            ),
            bit_size: ident.header.bit_size,
            os_abi: ident.header.os_abi,
        })
    }

    pub fn header(&mut self) -> Result<ElfHeader> {
        self.serde.set_position(0);
        match self.bit_size {
            ArchBitSize::Arch32Bit => Ok(ElfHeader::B32(self.serde.deserialize()?)),
            ArchBitSize::Arch64Bit => Ok(ElfHeader::B64(self.serde.deserialize()?)),
        }
    }

    pub fn set_header(&mut self, new_header: ElfHeader) -> Result<()> {
        self.serde.set_position(0);
        let endianness = new_header
            .ident()
            .header
            .endianness
            .to_binary_serde_endianness();
        self.serde.set_endianness(endianness);
        match new_header {
            ElfHeader::B32(hdr) => self.serde.serialize(&hdr)?,
            ElfHeader::B64(hdr) => self.serde.serialize(&hdr)?,
        }
        Ok(())
    }

    pub fn section_headers<'p>(&'p mut self) -> Result<SectionHeaders<'p, 'a>> {
        fn serialize_section_header(
            serde: &mut BinarySerdeBufSafe,
            value: &SectionHeader,
        ) -> Result<()> {
            match value {
                SectionHeader::B32(hdr) => serde.serialize(hdr)?,
                SectionHeader::B64(hdr) => serde.serialize(hdr)?,
            }
            Ok(())
        }
        fn deserialize_section_header_32(serde: &mut BinarySerdeBufSafe) -> Result<SectionHeader> {
            Ok(SectionHeader::B32(serde.deserialize()?))
        }
        fn deserialize_section_header_64(serde: &mut BinarySerdeBufSafe) -> Result<SectionHeader> {
            Ok(SectionHeader::B64(serde.deserialize()?))
        }

        let elf_hdr = self.header()?;
        let (record_size, deserialize_fn) = match self.bit_size {
            ArchBitSize::Arch32Bit => (
                SectionHeaderOfBitSize::<u32>::SERIALIZED_SIZE,
                deserialize_section_header_32
                    as fn(&mut BinarySerdeBufSafe) -> Result<SectionHeader>,
            ),
            ArchBitSize::Arch64Bit => (
                SectionHeaderOfBitSize::<u64>::SERIALIZED_SIZE,
                deserialize_section_header_64
                    as fn(&mut BinarySerdeBufSafe) -> Result<SectionHeader>,
            ),
        };

        Ok(SectionHeaders(ElfRecordsTable {
            record_size,
            records_amount: elf_hdr.section_headers_amount() as usize,
            serde: &mut self.serde,
            deserialize_fn,
            serialize_fn: serialize_section_header,
            table_offset: elf_hdr.section_headers_off() as usize,
            phantom: PhantomData,
        }))
    }

    pub fn program_headers<'p>(&'p mut self) -> Result<ProgramHeaders<'p, 'a>> {
        fn serialize_program_header(
            serde: &mut BinarySerdeBufSafe,
            value: &ProgramHeader,
        ) -> Result<()> {
            match value {
                ProgramHeader::B32(hdr) => serde.serialize(hdr)?,
                ProgramHeader::B64(hdr) => serde.serialize(hdr)?,
            }
            Ok(())
        }
        fn deserialize_program_header_32(serde: &mut BinarySerdeBufSafe) -> Result<ProgramHeader> {
            Ok(ProgramHeader::B32(serde.deserialize()?))
        }
        fn deserialize_program_header_64(serde: &mut BinarySerdeBufSafe) -> Result<ProgramHeader> {
            Ok(ProgramHeader::B64(serde.deserialize()?))
        }

        let elf_hdr = self.header()?;
        let (record_size, deserialize_fn) = match self.bit_size {
            ArchBitSize::Arch32Bit => (
                ProgramHeader32Bit::SERIALIZED_SIZE,
                deserialize_program_header_32
                    as fn(&mut BinarySerdeBufSafe) -> Result<ProgramHeader>,
            ),
            ArchBitSize::Arch64Bit => (
                ProgramHeader64Bit::SERIALIZED_SIZE,
                deserialize_program_header_64
                    as fn(&mut BinarySerdeBufSafe) -> Result<ProgramHeader>,
            ),
        };

        Ok(ProgramHeaders(ElfRecordsTable {
            record_size,
            records_amount: elf_hdr.program_headers_amount() as usize,
            serde: &mut self.serde,
            deserialize_fn,
            serialize_fn: serialize_program_header,
            table_offset: elf_hdr.program_headers_off() as usize,
            phantom: PhantomData,
        }))
    }
}

pub struct SectionHeaders<'p, 'a>(ElfRecordsTable<'p, 'a, SectionHeader>);
impl<'p, 'a> SectionHeaders<'p, 'a> {
    pub fn index(&mut self, index: usize) -> Result<SectionHeader> {
        self.0.index(index)
    }
    pub fn set_index(&mut self, index: usize, new_value: &SectionHeader) -> Result<()> {
        self.0.set_index(index, new_value)
    }
    pub fn iter(self) -> SectionHeadersIter<'p, 'a> {
        self.0.iter()
    }
}
pub type SectionHeadersIter<'p, 'a> = ElfRecordsTableIter<'p, 'a, SectionHeader>;

pub struct ProgramHeaders<'p, 'a>(ElfRecordsTable<'p, 'a, ProgramHeader>);
impl<'p, 'a> ProgramHeaders<'p, 'a> {
    pub fn index(&mut self, index: usize) -> Result<ProgramHeader> {
        self.0.index(index)
    }
    pub fn set_index(&mut self, index: usize, new_value: &ProgramHeader) -> Result<()> {
        self.0.set_index(index, new_value)
    }
    pub fn iter(self) -> ProgramHeadersIter<'p, 'a> {
        self.0.iter()
    }
}
pub type ProgramHeadersIter<'p, 'a> = ElfRecordsTableIter<'p, 'a, ProgramHeader>;

pub struct ElfRecordsTable<'p, 'a, E> {
    record_size: usize,
    records_amount: usize,
    serde: &'p mut BinarySerdeBufSafe<'a>,
    deserialize_fn: fn(&mut BinarySerdeBufSafe) -> Result<E>,
    serialize_fn: fn(&mut BinarySerdeBufSafe, &E) -> Result<()>,
    table_offset: usize,
    phantom: PhantomData<E>,
}
impl<'p, 'a, E> ElfRecordsTable<'p, 'a, E> {
    pub fn index(&mut self, index: usize) -> Result<E> {
        if index >= self.records_amount {
            return Err(Error::RecordIndexOutOfBounds {
                index,
                records_amount: self.records_amount,
            });
        }
        self.serde
            .set_position(self.table_offset + index * self.record_size);
        (self.deserialize_fn)(self.serde)
    }

    pub fn set_index(&mut self, index: usize, new_value: &E) -> Result<()> {
        if index >= self.records_amount {
            return Err(Error::RecordIndexOutOfBounds {
                index,
                records_amount: self.records_amount,
            });
        }
        self.serde
            .set_position(self.table_offset + index * self.record_size);
        (self.serialize_fn)(self.serde, new_value)
    }

    pub fn iter(self) -> ElfRecordsTableIter<'p, 'a, E> {
        self.serde.set_position(self.table_offset);
        ElfRecordsTableIter {
            serde: self.serde,
            deserialize_fn: self.deserialize_fn,
            cur_index: 0,
            records_amount: self.records_amount,
            phantom: PhantomData,
        }
    }
}

pub struct ElfRecordsTableIter<'p, 'a, E> {
    serde: &'p mut BinarySerdeBufSafe<'a>,
    deserialize_fn: fn(&mut BinarySerdeBufSafe) -> Result<E>,
    cur_index: usize,
    records_amount: usize,
    phantom: PhantomData<E>,
}
impl<'p, 'a, E> Iterator for ElfRecordsTableIter<'p, 'a, E> {
    type Item = Result<E>;

    fn next(&mut self) -> Option<Self::Item> {
        if self.cur_index >= self.records_amount {
            return None;
        }
        self.cur_index += 1;
        Some((self.deserialize_fn)(self.serde))
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

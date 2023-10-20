mod elf_types;

// use core::marker::PhantomData;

use binary_serde::{BinaryDeserializerFromBufSafe, BinarySerde, Endianness};
// use bitflags::bitflags;
use elf_types::{
    ArchBitSize, ElfHeader, ElfIdent, OsAbi, ProgramHeader, SectionHeaderType, EI_NIDENT, ELF_MAGIC,
};
use thiserror_no_std::Error;

// use crate::elf_types::{ProgramHeader32Bit, ProgramHeader64Bit};

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
            endianness: ident.header.endianness.into(),
            bit_size: ident.header.bit_size,
            os_abi: ident.header.os_abi,
        })
    }

    fn deserializer(&self) -> BinaryDeserializerFromBufSafe<'a> {
        BinaryDeserializerFromBufSafe::new(self.data, self.endianness)
    }

    pub fn header(&mut self) -> Result<ElfHeader> {
        match self.bit_size {
            ArchBitSize::Arch32Bit => {
                Ok(ElfHeader::ElfHeader32(self.deserializer().deserialize()?))
            }
            ArchBitSize::Arch64Bit => {
                Ok(ElfHeader::ElfHeader64(self.deserializer().deserialize()?))
            }
        }
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

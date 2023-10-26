use binary_serde::{binary_serde_bitfield, BinarySerde, BitfieldBitOrder};
use elflib_macros::define_raw_struct_by_variants;

use crate::{ElfParser, VariantStructBinaryDeserialize};

use super::{ArchBitLength, Architechture, ElfFileInfo, RelocationType, RelocationTypeU8};

define_raw_struct_by_variants! {
    #[binary_serde_bitfield(order = BitfieldBitOrder::LsbFirst)]
    #[derive(Copy)]
    struct RelocationEntryInfoRegular32 {
        #[bits(8)]
        ty: RelocationTypeU8,

        #[bits(24)]
        symbol_index: u32,
    }

    #[derive(Copy)]
    struct RelocationEntryInfoRegular64 {
        ty: RelocationType,
        symbol_index: u32,
    }
}

impl TryFrom<RelocationEntryInfoRegular64> for RelocationEntryInfoRegular32 {
    type Error = <RelocationTypeU8 as TryFrom<RelocationType>>::Error;

    fn try_from(value: RelocationEntryInfoRegular64) -> Result<Self, Self::Error> {
        Ok(Self {
            ty: value.ty.try_into()?,
            symbol_index: value.symbol_index,
        })
    }
}
impl From<RelocationEntryInfoRegular32> for RelocationEntryInfoRegular64 {
    fn from(value: RelocationEntryInfoRegular32) -> Self {
        Self {
            ty: value.ty.into(),
            symbol_index: value.symbol_index,
        }
    }
}

define_raw_struct_by_variants! {
    struct RelocationEntryRegular32 {
        offset: u32,
        info: RelocationEntryInfoRegular32,
    }
    struct RelocationEntryRegular64 {
        offset: u64,
        info: RelocationEntryInfoRegular64,
    }
}

define_raw_struct_by_variants! {
    struct RelocationEntryWithAddendRegular32 {
        offset: u32,
        info: RelocationEntryInfoRegular32,
    }
    struct RelocationEntryWithAddendRegular64 {
        offset: u64,
        info: RelocationEntryInfoRegular64,
    }
}

#[derive(Debug, BinarySerde, PartialEq, Eq, Clone, Hash)]
pub struct RelocationEntryMips64 {
    pub offset: u64,
    pub symbol_index: u32,
    pub special_symbol: u8,
    pub ty3: RelocationTypeU8,
    pub ty2: RelocationTypeU8,
    pub ty: RelocationTypeU8,
}

#[derive(Debug, BinarySerde, PartialEq, Eq, Clone, Hash)]
pub struct RelocationEntryWithAddendMips64 {
    pub offset: u64,
    pub symbol_index: u32,
    pub special_symbol: u8,
    pub ty3: RelocationTypeU8,
    pub ty2: RelocationTypeU8,
    pub ty: RelocationTypeU8,
    pub addend: i64,
}

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub enum RelocationEntry {
    RelocationEntryMips64(RelocationEntryMips64),
    RelocationEntryRegular(RelocationEntryRegular),
}
impl<'a> VariantStructBinaryDeserialize<'a> for RelocationEntry {
    fn deserialize(
        deserializer: &mut binary_serde::BinaryDeserializerFromBufSafe<'a>,
        parser: &ElfParser<'a>,
    ) -> core::result::Result<Self, binary_serde::BinarySerdeBufSafeError> {
        match (parser.file_info.arch, parser.file_info.bit_length) {
            (Architechture::Mips, ArchBitLength::Arch64Bit) => {
                Ok(Self::RelocationEntryMips64(deserializer.deserialize()?))
            }
            _ => Ok(Self::RelocationEntryRegular(
                RelocationEntryRegular::deserialize(deserializer, parser)?,
            )),
        }
    }

    fn record_len(file_info: &ElfFileInfo) -> usize {
        match (file_info.arch, file_info.bit_length) {
            (Architechture::Mips, ArchBitLength::Arch64Bit) => {
                RelocationEntryMips64::SERIALIZED_SIZE
            }
            _ => RelocationEntryRegular::record_len(file_info),
        }
    }
}

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub enum RelocationEntryWithAddend {
    RelocationEntryWithAddendMips64(RelocationEntryWithAddendMips64),
    RelocationEntryWithAddendRegular(RelocationEntryWithAddendRegular),
}
impl<'a> VariantStructBinaryDeserialize<'a> for RelocationEntryWithAddend {
    fn deserialize(
        deserializer: &mut binary_serde::BinaryDeserializerFromBufSafe<'a>,
        parser: &ElfParser<'a>,
    ) -> core::result::Result<Self, binary_serde::BinarySerdeBufSafeError> {
        match (parser.file_info.arch, parser.file_info.bit_length) {
            (Architechture::Mips, ArchBitLength::Arch64Bit) => Ok(
                Self::RelocationEntryWithAddendMips64(deserializer.deserialize()?),
            ),
            _ => Ok(Self::RelocationEntryWithAddendRegular(
                RelocationEntryWithAddendRegular::deserialize(deserializer, parser)?,
            )),
        }
    }

    fn record_len(file_info: &ElfFileInfo) -> usize {
        match (file_info.arch, file_info.bit_length) {
            (Architechture::Mips, ArchBitLength::Arch64Bit) => {
                RelocationEntryWithAddendMips64::SERIALIZED_SIZE
            }
            _ => RelocationEntryWithAddendRegular::record_len(file_info),
        }
    }
}

#[derive(Debug, BinarySerde, Clone, Copy, PartialEq, Eq, Hash)]
#[repr(u8)]
pub enum RelocationSpecialSymbolMips64 {
    /// none
    Undefined = 0,
    /// value of gp
    GP = 1,
    /// value of gp used to create object being relocated
    GP0 = 2,
    /// address of location being relocated
    Loc = 3,
}

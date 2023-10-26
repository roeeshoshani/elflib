use binary_serde::{binary_serde_bitfield, BinarySerde, BitfieldBitOrder};

use crate::{ElfParser, VariantStructBinaryDeserialize};

use super::{ArchBitLength, Architechture, ElfFileInfo, RelocationType, RelocationTypeU8};

#[binary_serde_bitfield(order = BitfieldBitOrder::LsbFirst)]
#[derive(Debug, PartialEq, Eq, Clone, Hash)]
pub struct RelocationEntryInfo32 {
    #[bits(8)]
    pub ty: RelocationTypeU8,

    #[bits(24)]
    pub symbol_index: u32,
}

#[derive(Debug, BinarySerde, PartialEq, Eq, Clone, Hash)]
pub struct RelocationEntry32 {
    pub offset: u32,
    pub info: RelocationEntryInfo32,
}
#[derive(Debug, BinarySerde, PartialEq, Eq, Clone, Hash)]
pub struct RelocationEntryWithAddend32 {
    pub offset: u32,
    pub info: RelocationEntryInfo32,
    pub addend: i32,
}

#[derive(Debug, BinarySerde, PartialEq, Eq, Clone, Hash)]
pub struct RelocationEntry64 {
    pub offset: u64,
    pub ty: RelocationType,
    pub symbol_index: u32,
}

#[derive(Debug, BinarySerde, PartialEq, Eq, Clone, Hash)]
pub struct RelocationEntryWithAddend64 {
    pub offset: u64,
    pub ty: RelocationType,
    pub symbol_index: u32,
    pub addend: i64,
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

#[derive(Debug, PartialEq, Eq, Clone, Hash)]
pub enum RegularRelocationEntry {
    RelocationEntry32(RelocationEntry32),
    RelocationEntry64(RelocationEntry64),
}
impl RegularRelocationEntry {
    pub fn offset(&self) -> u64 {
        match self {
            RegularRelocationEntry::RelocationEntry32(x) => x.offset as u64,
            RegularRelocationEntry::RelocationEntry64(x) => x.offset,
        }
    }
    pub fn set_offset(&mut self, new_value: u64) {
        match self {
            RegularRelocationEntry::RelocationEntry32(x) => x.offset = new_value as u32,
            RegularRelocationEntry::RelocationEntry64(x) => x.offset = new_value,
        }
    }
    pub fn ty(&self) -> RelocationType {
        match self {
            RegularRelocationEntry::RelocationEntry32(x) => x.info.ty.get(),
            RegularRelocationEntry::RelocationEntry64(x) => x.ty,
        }
    }
    pub fn set_ty(&mut self, new_value: RelocationType) {
        match self {
            RegularRelocationEntry::RelocationEntry32(x) => {
                x.info.ty = RelocationTypeU8::new(new_value)
            }
            RegularRelocationEntry::RelocationEntry64(x) => x.ty = new_value,
        }
    }
    pub fn symbol_index(&self) -> u32 {
        match self {
            RegularRelocationEntry::RelocationEntry32(x) => x.info.symbol_index,
            RegularRelocationEntry::RelocationEntry64(x) => x.symbol_index,
        }
    }
    pub fn set_symbol_index(&mut self, new_value: u32) {
        match self {
            RegularRelocationEntry::RelocationEntry32(x) => x.info.symbol_index = new_value,
            RegularRelocationEntry::RelocationEntry64(x) => x.symbol_index = new_value,
        }
    }
}

#[derive(Debug, PartialEq, Eq, Clone, Hash)]
pub enum RelocationEntry {
    RegularRelocationEntry(RegularRelocationEntry),
    RelocationEntryMips64(RelocationEntryMips64),
}

impl RelocationEntry {
    pub fn offset(&self) -> u64 {
        match self {
            RelocationEntry::RegularRelocationEntry(x) => x.offset(),
            RelocationEntry::RelocationEntryMips64(x) => x.offset,
        }
    }
    pub fn set_offset(&mut self, new_value: u64) {
        match self {
            RelocationEntry::RegularRelocationEntry(x) => x.set_offset(new_value),
            RelocationEntry::RelocationEntryMips64(x) => x.offset = new_value,
        }
    }
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
            (_, ArchBitLength::Arch32Bit) => Ok(Self::RegularRelocationEntry(
                RegularRelocationEntry::RelocationEntry32(deserializer.deserialize()?),
            )),
            (_, ArchBitLength::Arch64Bit) => Ok(Self::RegularRelocationEntry(
                RegularRelocationEntry::RelocationEntry64(deserializer.deserialize()?),
            )),
        }
    }

    fn record_len(file_info: &ElfFileInfo) -> usize {
        match (file_info.arch, file_info.bit_length) {
            (Architechture::Mips, ArchBitLength::Arch64Bit) => {
                RelocationEntryMips64::SERIALIZED_SIZE
            }
            (_, ArchBitLength::Arch32Bit) => RelocationEntry32::SERIALIZED_SIZE,
            (_, ArchBitLength::Arch64Bit) => RelocationEntry64::SERIALIZED_SIZE,
        }
    }
}

#[derive(Debug, PartialEq, Eq, Clone, Hash)]
pub enum RegularRelocationEntryWithAddend {
    RelocationEntryWithAddend32(RelocationEntryWithAddend32),
    RelocationEntryWithAddend64(RelocationEntryWithAddend64),
}
impl RegularRelocationEntryWithAddend {
    pub fn offset(&self) -> u64 {
        match self {
            RegularRelocationEntryWithAddend::RelocationEntryWithAddend32(x) => x.offset as u64,
            RegularRelocationEntryWithAddend::RelocationEntryWithAddend64(x) => x.offset,
        }
    }
    pub fn set_offset(&mut self, new_value: u64) {
        match self {
            RegularRelocationEntryWithAddend::RelocationEntryWithAddend32(x) => {
                x.offset = new_value as u32
            }
            RegularRelocationEntryWithAddend::RelocationEntryWithAddend64(x) => {
                x.offset = new_value
            }
        }
    }
    pub fn ty(&self) -> RelocationType {
        match self {
            RegularRelocationEntryWithAddend::RelocationEntryWithAddend32(x) => x.info.ty.get(),
            RegularRelocationEntryWithAddend::RelocationEntryWithAddend64(x) => x.ty,
        }
    }
    pub fn set_ty(&mut self, new_value: RelocationType) {
        match self {
            RegularRelocationEntryWithAddend::RelocationEntryWithAddend32(x) => {
                x.info.ty = RelocationTypeU8::new(new_value)
            }
            RegularRelocationEntryWithAddend::RelocationEntryWithAddend64(x) => x.ty = new_value,
        }
    }
    pub fn symbol_index(&self) -> u32 {
        match self {
            RegularRelocationEntryWithAddend::RelocationEntryWithAddend32(x) => x.info.symbol_index,
            RegularRelocationEntryWithAddend::RelocationEntryWithAddend64(x) => x.symbol_index,
        }
    }
    pub fn set_symbol_index(&mut self, new_value: u32) {
        match self {
            RegularRelocationEntryWithAddend::RelocationEntryWithAddend32(x) => {
                x.info.symbol_index = new_value
            }
            RegularRelocationEntryWithAddend::RelocationEntryWithAddend64(x) => {
                x.symbol_index = new_value
            }
        }
    }
    pub fn addend(&self) -> i64 {
        match self {
            RegularRelocationEntryWithAddend::RelocationEntryWithAddend32(x) => x.addend as i64,
            RegularRelocationEntryWithAddend::RelocationEntryWithAddend64(x) => x.addend,
        }
    }
    pub fn set_addend(&mut self, new_value: i64) {
        match self {
            RegularRelocationEntryWithAddend::RelocationEntryWithAddend32(x) => {
                x.addend = new_value as i32
            }
            RegularRelocationEntryWithAddend::RelocationEntryWithAddend64(x) => {
                x.addend = new_value
            }
        }
    }
}

#[derive(Debug, PartialEq, Eq, Clone, Hash)]
pub enum RelocationEntryWithAddend {
    RegularRelocationEntryWithAddend(RegularRelocationEntryWithAddend),
    RelocationEntryWithAddendMips64(RelocationEntryWithAddendMips64),
}

impl RelocationEntryWithAddend {
    pub fn offset(&self) -> u64 {
        match self {
            RelocationEntryWithAddend::RegularRelocationEntryWithAddend(x) => x.offset(),
            RelocationEntryWithAddend::RelocationEntryWithAddendMips64(x) => x.offset,
        }
    }
    pub fn set_offset(&mut self, new_value: u64) {
        match self {
            RelocationEntryWithAddend::RegularRelocationEntryWithAddend(x) => {
                x.set_offset(new_value)
            }
            RelocationEntryWithAddend::RelocationEntryWithAddendMips64(x) => x.offset = new_value,
        }
    }
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
            (_, ArchBitLength::Arch32Bit) => Ok(Self::RegularRelocationEntryWithAddend(
                RegularRelocationEntryWithAddend::RelocationEntryWithAddend32(
                    deserializer.deserialize()?,
                ),
            )),
            (_, ArchBitLength::Arch64Bit) => Ok(Self::RegularRelocationEntryWithAddend(
                RegularRelocationEntryWithAddend::RelocationEntryWithAddend64(
                    deserializer.deserialize()?,
                ),
            )),
        }
    }

    fn record_len(file_info: &ElfFileInfo) -> usize {
        match (file_info.arch, file_info.bit_length) {
            (Architechture::Mips, ArchBitLength::Arch64Bit) => {
                RelocationEntryWithAddendMips64::SERIALIZED_SIZE
            }
            (_, ArchBitLength::Arch32Bit) => RelocationEntryWithAddend32::SERIALIZED_SIZE,
            (_, ArchBitLength::Arch64Bit) => RelocationEntryWithAddend64::SERIALIZED_SIZE,
        }
    }
}

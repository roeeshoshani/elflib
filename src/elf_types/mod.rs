mod codegen;
mod relocations;

pub use codegen::*;
pub use relocations::*;

use crate::{ElfParser, VariantStructBinaryDeserialize};
use binary_serde::{
    binary_serde_bitfield, impl_binary_serde_for_bitflags_ty, BinarySerde, BitfieldBitOrder,
    Endianness,
};
use bitflags::bitflags;
use elflib_macros::{define_raw_struct_by_variants, define_raw_struct_generic_bitlen};

pub const ELF_MAGIC: &[u8] = &[0x7f, b'E', b'L', b'F'];
pub const EI_NIDENT: usize = 16;
pub const ELF_IDENT_PADDING_SIZE: usize = EI_NIDENT - ElfIdentHeader::SERIALIZED_SIZE;

#[derive(Clone, Copy, PartialEq, Eq, Hash)]
pub(crate) struct DebugIgnore<T>(pub(crate) T);
impl<T> core::fmt::Debug for DebugIgnore<T> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "...")
    }
}
impl<T> From<T> for DebugIgnore<T> {
    fn from(value: T) -> Self {
        Self(value)
    }
}
impl<T> core::ops::Deref for DebugIgnore<T> {
    type Target = T;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl<T> core::ops::DerefMut for DebugIgnore<T> {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.0
    }
}

#[derive(Debug, PartialEq, Eq, Clone, Copy, Hash)]
pub struct ElfFileInfo {
    pub endianness: Endianness,
    pub bit_length: ArchBitLength,
    pub os_abi: OsAbi,
    pub arch: Architechture,
}

#[derive(Debug, PartialEq, Eq, Clone, Hash, BinarySerde)]
pub struct ElfIdentHeader {
    pub magic: [u8; ELF_MAGIC.len()],
    pub bit_size: ArchBitLength,
    pub endianness: ElfEndianness,
    pub elf_version: ElfVersionInIdent,
    pub os_abi: OsAbi,
    pub abi_version: AbiVersion,
}

#[derive(Debug, PartialEq, Eq, Clone, Hash, BinarySerde)]
pub struct ElfIdent {
    pub header: ElfIdentHeader,
    pub padding: [u8; ELF_IDENT_PADDING_SIZE],
}

define_raw_struct_by_variants! {
    struct ProgramHeader32 {
        ty: ProgramHeaderType,
        offset: u32,
        virt_addr: u32,
        phys_addr: u32,
        size_in_file: u32,
        size_in_memory: u32,
        flags: ProgramHeaderFlags,
        alignment: u32,
    }
    struct ProgramHeader64 {
        ty: ProgramHeaderType,
        flags: ProgramHeaderFlags,
        offset: u64,
        virt_addr: u64,
        phys_addr: u64,
        size_in_file: u64,
        size_in_memory: u64,
        alignment: u64,
    }
}

define_raw_struct_by_variants! {
    struct SectionHeader32 {
        name_offset: u32,
        ty: SectionHeaderType,
        flags: SectionHeaderFlags,
        address: u64,
        offset: u64,
        size: u64,
        link: u32,
        info: u32,
        address_alignemnt: u64,
        entry_size: u64,
    }
    struct SectionHeader64 {
        name_offset: u32,
        ty: SectionHeaderType,
        flags: SectionHeaderFlagsU32,
        address: u32,
        offset: u32,
        size: u32,
        link: u32,
        info: u32,
        address_alignemnt: u32,
        entry_size: u32,
    }
}

define_raw_struct_generic_bitlen! {
    struct ElfHeader {
        ident: ElfIdent,
        ty: ElfFileType,
        arch: Architechture,
        version: ElfVersion,
        entry: U,
        program_headers_off: U,
        section_headers_off: U,
        flags: ElfFlags,
        header_size: u16,
        program_header_entry_size: u16,
        program_headers_amount: u16,
        section_header_entry_size: u16,
        section_headers_amount: u16,
        section_names_section_index: u16,
    }
}

#[derive(Debug, BinarySerde, Clone, Copy, PartialEq, Eq, Hash)]
#[repr(u8)]
pub enum AbiVersion {
    Valid = 0,
}

#[derive(Debug, BinarySerde, Clone, Copy, PartialEq, Eq, Hash)]
#[repr(u8)]
pub enum ElfVersionInIdent {
    Current = 1,
}

#[derive(Debug, BinarySerde, Clone, Copy, PartialEq, Eq, Hash)]
#[repr(u32)]
pub enum ElfVersion {
    Current = 1,
}

#[derive(Debug, BinarySerde, Clone, Copy, PartialEq, Eq, Hash)]
#[repr(u8)]
pub enum ElfEndianness {
    Little = 1,
    Big = 2,
}
impl From<ElfEndianness> for binary_serde::Endianness {
    fn from(value: ElfEndianness) -> Self {
        match value {
            ElfEndianness::Little => Endianness::Little,
            ElfEndianness::Big => Endianness::Big,
        }
    }
}
impl From<binary_serde::Endianness> for ElfEndianness {
    fn from(value: binary_serde::Endianness) -> Self {
        match value {
            Endianness::Little => ElfEndianness::Little,
            Endianness::Big => ElfEndianness::Big,
        }
    }
}

#[derive(Debug, BinarySerde, Clone, Copy, PartialEq, Eq, Hash)]
#[repr(u8)]
pub enum ArchBitLength {
    Arch32Bit = 1,
    Arch64Bit = 2,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct RelocationTypeU8(RelocationType);
impl RelocationTypeU8 {
    pub fn new(value: RelocationType) -> Self {
        // make sure that the relocation type fits in a u8
        let _ = value as u8;
        Self(value)
    }
    pub fn get(&self) -> RelocationType {
        self.0
    }
}
impl BinarySerde for RelocationTypeU8 {
    const SERIALIZED_SIZE: usize = 1;

    type RecursiveArray = <u8 as BinarySerde>::RecursiveArray;

    fn binary_serialize(&self, buf: &mut [u8], endianness: Endianness) {
        let value = self.0 as u8;
        value.binary_serialize(buf, endianness)
    }

    fn binary_deserialize(
        buf: &[u8],
        endianness: Endianness,
    ) -> Result<Self, binary_serde::DeserializeError> {
        let bytes = (buf[0] as u32).binary_serialize_to_array(endianness);
        Ok(Self(RelocationType::binary_deserialize(
            bytes.as_ref(),
            endianness,
        )?))
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct SectionHeaderFlagsU32(SectionHeaderFlags);
impl SectionHeaderFlagsU32 {
    pub fn new(value: SectionHeaderFlags) -> Self {
        // make sure that the relocation type fits in a u32
        let _ = value.bits() as u32;
        Self(value)
    }
    pub fn get(&self) -> SectionHeaderFlags {
        self.0
    }
}
impl BinarySerde for SectionHeaderFlagsU32 {
    const SERIALIZED_SIZE: usize = 1;

    type RecursiveArray = <u32 as BinarySerde>::RecursiveArray;

    fn binary_serialize(&self, buf: &mut [u8], endianness: Endianness) {
        let value = self.0.bits() as u32;
        value.binary_serialize(buf, endianness)
    }

    fn binary_deserialize(
        buf: &[u8],
        endianness: Endianness,
    ) -> Result<Self, binary_serde::DeserializeError> {
        let bytes = (buf[0] as u32).binary_serialize_to_array(endianness);
        Ok(Self(SectionHeaderFlags::binary_deserialize(
            bytes.as_ref(),
            endianness,
        )?))
    }
}
impl TryFrom<SectionHeaderFlags> for SectionHeaderFlagsU32 {
    type Error = <u32 as TryFrom<u64>>::Error;

    fn try_from(value: SectionHeaderFlags) -> Result<Self, Self::Error> {
        let _ = u32::try_from(value.bits())?;
        Ok(Self(value))
    }
}
impl From<SectionHeaderFlagsU32> for SectionHeaderFlags {
    fn from(value: SectionHeaderFlagsU32) -> Self {
        value.0
    }
}

mod codegen;
mod relocations;

pub use codegen::*;
pub use relocations::*;

use crate::{ElfParser, StringTable, VariantStructBinarySerde};
use binary_serde::{binary_serde_bitfield, BinarySerde, BitfieldBitOrder, Endianness};
use elflib_macros::{define_raw_struct_by_variants, define_raw_struct_generic_bitlen};

pub const ELF_MAGIC: &[u8] = &[0x7f, b'E', b'L', b'F'];
pub const EI_NIDENT: usize = 16;
pub const ELF_IDENT_PADDING_SIZE: usize = EI_NIDENT - ElfIdentHeader::SERIALIZED_SIZE;

#[derive(Clone, Copy, PartialEq, Eq, Hash)]
pub(crate) struct DebugIgnore<T>(pub(crate) T);
impl<T> core::fmt::Debug for DebugIgnore<T> {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
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
    => ()
}

define_raw_struct_by_variants! {
    struct SectionHeader64 {
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
    struct SectionHeader32 {
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
    => ()
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
    => ()
}

#[derive(Debug, PartialEq, Eq, Clone, Hash)]
#[binary_serde_bitfield(order = BitfieldBitOrder::LsbFirst)]
pub struct SymbolInfo {
    #[bits(4)]
    pub ty: SymbolType,

    #[bits(4)]
    pub binding: SymbolBinding,
}

#[derive(Debug, PartialEq, Eq, Clone, Hash)]
#[binary_serde_bitfield(order = BitfieldBitOrder::LsbFirst)]
pub struct SymbolOtherInfo {
    #[bits(2)]
    pub visibility: SymbolVisibility,

    #[bits(6)]
    pub padding: u8,
}

#[derive(Debug, Clone)]
pub struct SymbolRefContext<'a> {
    pub(crate) string_table: StringTable<'a>,
}

define_raw_struct_by_variants! {
    struct Symbol32 {
        name_index_in_string_table: u32,
        value: u32,
        size: u32,
        info: SymbolInfo,
        other_info: SymbolOtherInfo,
        related_section_index: u16,
    }
    struct Symbol64 {
        name_index_in_string_table: u32,
        info: SymbolInfo,
        other_info: SymbolOtherInfo,
        related_section_index: u16,
        value: u64,
        size: u64,
    }
    => SymbolRefContext<'a>
}

#[derive(Debug, BinarySerde, Clone, Copy, PartialEq, Eq, Hash)]
#[repr(u8)]
pub enum AbiVersion {
    Valid = 0,
    Unknown = 1,
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

macro_rules! gen_enum_size_truncating_wrapper {
    {$wrapper_name: ident, $inner_ty: ty, $truncated_uint: ty, $original_uint: ty, $convert_inner_to_bits_input_var_name: ident, $convert_inner_to_bits_body: block} => {
        #[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
        pub struct $wrapper_name($inner_ty);
        impl BinarySerde for $wrapper_name {
            const SERIALIZED_SIZE: usize = <$truncated_uint as BinarySerde>::SERIALIZED_SIZE;

            type RecursiveArray = <$truncated_uint as BinarySerde>::RecursiveArray;

            fn binary_serialize(&self, buf: &mut [u8], endianness: Endianness) {
                let converted = {
                    let $convert_inner_to_bits_input_var_name = &self.0;
                    $convert_inner_to_bits_body
                };
                converted.binary_serialize(buf, endianness)
            }

            fn binary_deserialize(
                buf: &[u8],
                endianness: Endianness,
            ) -> Result<Self, binary_serde::DeserializeError> {
                let value = <$truncated_uint>::binary_deserialize(buf, endianness)?;
                let bytes = (value as $original_uint).binary_serialize_to_array(endianness);
                Ok(Self(<$inner_ty>::binary_deserialize(
                    bytes.as_ref(),
                    endianness,
                )?))
            }
        }
        impl TryFrom<$inner_ty> for $wrapper_name {
            type Error = <$truncated_uint as TryFrom<$original_uint>>::Error;

            fn try_from(value: $inner_ty) -> Result<Self, Self::Error> {
                let converted = {
                    let $convert_inner_to_bits_input_var_name = &value;
                    $convert_inner_to_bits_body
                };
                let _ = <$truncated_uint>::try_from(converted)?;
                Ok(Self(value))
            }
        }
        impl From<$wrapper_name> for $inner_ty {
            fn from(value: $wrapper_name) -> Self {
                value.0
            }
        }
    };
}

gen_enum_size_truncating_wrapper! {RelocationTypeU8, RelocationType, u8, u32, x, {*x as u32}}
gen_enum_size_truncating_wrapper! {SectionHeaderFlagsU32, SectionHeaderFlags, u32, u64, x, {x.bits()}}

use binary_serde::{binary_serde_bitfield, BinarySerde, BitfieldBitOrder};
use elflib_macros::define_raw_struct_by_variants;

use crate::{ElfParser, VariantStructBinaryDeserialize};

use super::{ArchBitLength, Architechture, ElfFileInfo, RelocationType, RelocationTypeU8};

define_raw_struct_by_variants! {
    #[binary_serde_bitfield(order = BitfieldBitOrder::LsbFirst)]
    #[derive(Copy)]
    struct RelInfoRegular32 {
        #[bits(8)]
        ty: RelocationTypeU8,

        #[bits(24)]
        symbol_index: u32,
    }

    #[derive(Copy)]
    struct RelInfoRegular64 {
        ty: RelocationType,
        symbol_index: u32,
    }
}

impl TryFrom<RelInfoRegular64> for RelInfoRegular32 {
    type Error = <RelocationTypeU8 as TryFrom<RelocationType>>::Error;

    fn try_from(value: RelInfoRegular64) -> Result<Self, Self::Error> {
        Ok(Self {
            ty: value.ty.try_into()?,
            symbol_index: value.symbol_index,
        })
    }
}
impl From<RelInfoRegular32> for RelInfoRegular64 {
    fn from(value: RelInfoRegular32) -> Self {
        Self {
            ty: value.ty.into(),
            symbol_index: value.symbol_index,
        }
    }
}

define_raw_struct_by_variants! {
    struct RelRegular32 {
        offset: u32,
        info: RelInfoRegular32,
    }
    struct RelRegular64 {
        offset: u64,
        info: RelInfoRegular64,
    }
}

define_raw_struct_by_variants! {
    struct RelaRegular32 {
        offset: u32,
        info: RelInfoRegular32,
        addend: i32,
    }
    struct RelaRegular64 {
        offset: u64,
        info: RelInfoRegular64,
        addend: i64,
    }
}

#[derive(Debug, BinarySerde, PartialEq, Eq, Clone, Hash)]
pub struct RelMips64 {
    pub offset: u64,
    pub symbol_index: u32,
    pub special_symbol: u8,
    pub ty3: RelocationTypeU8,
    pub ty2: RelocationTypeU8,
    pub ty: RelocationTypeU8,
}

#[derive(Debug, BinarySerde, PartialEq, Eq, Clone, Hash)]
pub struct RelaMips64 {
    pub offset: u64,
    pub symbol_index: u32,
    pub special_symbol: u8,
    pub ty3: RelocationTypeU8,
    pub ty2: RelocationTypeU8,
    pub ty: RelocationTypeU8,
    pub addend: i64,
}

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub enum Rel {
    RelMips64(RelMips64),
    RelRegular(RelRegular),
}
impl<'a> VariantStructBinaryDeserialize<'a> for Rel {
    fn deserialize(
        deserializer: &mut binary_serde::BinaryDeserializerFromBufSafe<'a>,
        parser: &ElfParser<'a>,
    ) -> core::result::Result<Self, binary_serde::BinarySerdeBufSafeError> {
        match (parser.file_info.arch, parser.file_info.bit_length) {
            (Architechture::Mips, ArchBitLength::Arch64Bit) => {
                Ok(Self::RelMips64(deserializer.deserialize()?))
            }
            _ => Ok(Self::RelRegular(RelRegular::deserialize(
                deserializer,
                parser,
            )?)),
        }
    }

    fn record_len(file_info: &ElfFileInfo) -> usize {
        match (file_info.arch, file_info.bit_length) {
            (Architechture::Mips, ArchBitLength::Arch64Bit) => RelMips64::SERIALIZED_SIZE,
            _ => RelRegular::record_len(file_info),
        }
    }
}

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub enum Rela {
    RelaMips64(RelaMips64),
    RelaRegular(RelaRegular),
}
impl<'a> VariantStructBinaryDeserialize<'a> for Rela {
    fn deserialize(
        deserializer: &mut binary_serde::BinaryDeserializerFromBufSafe<'a>,
        parser: &ElfParser<'a>,
    ) -> core::result::Result<Self, binary_serde::BinarySerdeBufSafeError> {
        match (parser.file_info.arch, parser.file_info.bit_length) {
            (Architechture::Mips, ArchBitLength::Arch64Bit) => {
                Ok(Self::RelaMips64(deserializer.deserialize()?))
            }
            _ => Ok(Self::RelaRegular(RelaRegular::deserialize(
                deserializer,
                parser,
            )?)),
        }
    }

    fn record_len(file_info: &ElfFileInfo) -> usize {
        match (file_info.arch, file_info.bit_length) {
            (Architechture::Mips, ArchBitLength::Arch64Bit) => RelaMips64::SERIALIZED_SIZE,
            _ => RelaRegular::record_len(file_info),
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

macro_rules! gen_generic_rel_ty {
    {$rel_postfix: ident, $addend_ty: ty, $convert_rela_to_rel_input_var_name: ident, $convert_rela_to_rel_block: block} => {
        paste::paste!{
            #[derive(Debug, Clone, PartialEq, Eq, Hash)]
            pub enum [<GenericRel $rel_postfix>] {
                [<Rel $rel_postfix>]([<Rel $rel_postfix>]),
                [<Rela $rel_postfix>]([<Rela $rel_postfix>]),
            }
            impl [<GenericRel $rel_postfix>] {
                pub fn as_rel_with_opt_addend(self) -> [<RelOptAddend $rel_postfix>] {
                    match self {
                        Self::[<Rel $rel_postfix>](x) => [<RelOptAddend $rel_postfix>] {
                            rel: x,
                            addend: None,
                        },
                        Self::[<Rela $rel_postfix>](x) => [<RelOptAddend $rel_postfix>] {
                            rel: {
                                let $convert_rela_to_rel_input_var_name = &x;
                                $convert_rela_to_rel_block
                            },
                            addend: Some(x.addend),
                        },
                    }
                }
            }
            #[derive(Debug, Clone, PartialEq, Eq, Hash)]
            pub struct [<RelOptAddend $rel_postfix>] {
                pub rel: [<Rel $rel_postfix>],
                pub addend: Option<$addend_ty>,
            }
        }
    };
}

gen_generic_rel_ty! {Regular32, i32, x, {
    RelRegular32 {
        offset: x.offset,
        info: x.info,
    }
}}
gen_generic_rel_ty! {Regular64, i64, x, {
    RelRegular64 {
        offset: x.offset,
        info: x.info,
    }
}}
gen_generic_rel_ty! {Mips64, i64, x, {
    RelMips64 {
        offset: x.offset,
        symbol_index: x.symbol_index,
        special_symbol: x.special_symbol,
        ty3: x.ty3,
        ty2: x.ty2,
        ty: x.ty,
    }
}}

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub enum GenericRelRegular {
    GenericRelRegular32(GenericRelRegular32),
    GenericRelRegular64(GenericRelRegular64),
}
impl GenericRelRegular {
    pub fn as_rel_with_opt_addend(self) -> RelOptAddendRegular {
        match self {
            GenericRelRegular::GenericRelRegular32(x) => {
                let rel_with_opt_addend = x.as_rel_with_opt_addend();
                RelOptAddendRegular {
                    rel: RelRegular::RelRegular32(rel_with_opt_addend.rel),
                    addend: rel_with_opt_addend.addend.map(|x| x as i64),
                }
            }
            GenericRelRegular::GenericRelRegular64(x) => {
                let rel_with_opt_addend = x.as_rel_with_opt_addend();
                RelOptAddendRegular {
                    rel: RelRegular::RelRegular64(rel_with_opt_addend.rel),
                    addend: rel_with_opt_addend.addend,
                }
            }
        }
    }
}
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct RelOptAddendRegular {
    pub rel: RelRegular,
    pub addend: Option<i64>,
}

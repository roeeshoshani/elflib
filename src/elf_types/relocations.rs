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
    => ()
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
    => ()
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
    => ()
}

impl RelaRegular32 {
    pub fn to_rel_and_addend(self) -> (RelRegular32, i32) {
        (
            RelRegular32 {
                offset: self.offset,
                info: self.info,
            },
            self.addend,
        )
    }
    pub fn from_rel_and_addend(rel: RelRegular32, addend: i32) -> Self {
        Self {
            offset: rel.offset,
            info: rel.info,
            addend,
        }
    }
}

impl RelaRegular64 {
    pub fn to_rel_and_addend(self) -> (RelRegular64, i64) {
        (
            RelRegular64 {
                offset: self.offset,
                info: self.info,
            },
            self.addend,
        )
    }
    pub fn from_rel_and_addend(rel: RelRegular64, addend: i64) -> Self {
        Self {
            offset: rel.offset,
            info: rel.info,
            addend,
        }
    }
}

impl RelaRegular {
    pub fn to_rel_and_addend(self) -> (RelRegular, i64) {
        match self {
            RelaRegular::RelaRegular32(x) => {
                let (rel, addend) = x.to_rel_and_addend();
                (RelRegular::RelRegular32(rel), addend as i64)
            }
            RelaRegular::RelaRegular64(x) => {
                let (rel, addend) = x.to_rel_and_addend();
                (RelRegular::RelRegular64(rel), addend)
            }
        }
    }
    pub fn from_rel_and_addend(rel: RelRegular, addend: i64) -> Self {
        match rel {
            RelRegular::RelRegular32(x) => {
                Self::RelaRegular32(RelaRegular32::from_rel_and_addend(x, addend as i32))
            }
            RelRegular::RelRegular64(x) => {
                Self::RelaRegular64(RelaRegular64::from_rel_and_addend(x, addend))
            }
        }
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
impl RelaMips64 {
    pub fn to_rel_and_addend(self) -> (RelMips64, i64) {
        (
            RelMips64 {
                offset: self.offset,
                symbol_index: self.symbol_index,
                special_symbol: self.special_symbol,
                ty3: self.ty3,
                ty2: self.ty2,
                ty: self.ty,
            },
            self.addend,
        )
    }
    pub fn from_rel_and_addend(rel: RelMips64, addend: i64) -> Self {
        Self {
            offset: rel.offset,
            symbol_index: rel.symbol_index,
            special_symbol: rel.special_symbol,
            ty3: rel.ty3,
            ty2: rel.ty2,
            ty: rel.ty,
            addend,
        }
    }
}

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub enum Rel {
    RelMips64(RelMips64),
    RelRegular(RelRegular),
}
impl<'a> VariantStructBinaryDeserialize<'a> for Rel {
    type Context = ();
    fn deserialize(
        deserializer: &mut binary_serde::BinaryDeserializerFromBufSafe<'a>,
        parser: &ElfParser<'a>,
        _context: (),
    ) -> core::result::Result<Self, binary_serde::BinarySerdeBufSafeError> {
        match (parser.file_info.arch, parser.file_info.bit_length) {
            (Architechture::Mips, ArchBitLength::Arch64Bit) => {
                Ok(Self::RelMips64(deserializer.deserialize()?))
            }
            _ => Ok(Self::RelRegular(RelRegular::deserialize(
                deserializer,
                parser,
                (),
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
impl Rel {
    pub fn to_generic_rel(self) -> GenericRel {
        GenericRel {
            rel: self,
            addend: None,
        }
    }
}
impl From<Rel> for GenericRel {
    fn from(value: Rel) -> Self {
        value.to_generic_rel()
    }
}

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub enum Rela {
    RelaMips64(RelaMips64),
    RelaRegular(RelaRegular),
}
impl Rela {
    pub fn to_rel_and_addend(self) -> (Rel, i64) {
        match self {
            Rela::RelaMips64(rela_mips64) => {
                let (rel, addend) = rela_mips64.to_rel_and_addend();
                (Rel::RelMips64(rel), addend)
            }
            Rela::RelaRegular(rela_regular) => {
                let (rel, addend) = rela_regular.to_rel_and_addend();
                (Rel::RelRegular(rel), addend)
            }
        }
    }
    pub fn from_rel_and_addend(rel: Rel, addend: i64) -> Self {
        match rel {
            Rel::RelMips64(x) => Self::RelaMips64(RelaMips64::from_rel_and_addend(x, addend)),
            Rel::RelRegular(x) => Self::RelaRegular(RelaRegular::from_rel_and_addend(x, addend)),
        }
    }
    pub fn to_generic_rel(self) -> GenericRel {
        let (rel, addend) = self.to_rel_and_addend();
        GenericRel {
            rel,
            addend: Some(addend),
        }
    }
}
impl From<Rela> for GenericRel {
    fn from(value: Rela) -> Self {
        value.to_generic_rel()
    }
}
impl<'a> VariantStructBinaryDeserialize<'a> for Rela {
    type Context = ();
    fn deserialize(
        deserializer: &mut binary_serde::BinaryDeserializerFromBufSafe<'a>,
        parser: &ElfParser<'a>,
        _context: (),
    ) -> core::result::Result<Self, binary_serde::BinarySerdeBufSafeError> {
        match (parser.file_info.arch, parser.file_info.bit_length) {
            (Architechture::Mips, ArchBitLength::Arch64Bit) => {
                Ok(Self::RelaMips64(deserializer.deserialize()?))
            }
            _ => Ok(Self::RelaRegular(RelaRegular::deserialize(
                deserializer,
                parser,
                (),
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

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct GenericRel {
    pub rel: Rel,
    pub addend: Option<i64>,
}
impl GenericRel {
    pub fn as_rel(self) -> Option<Rel> {
        if self.addend.is_none() {
            Some(self.rel)
        } else {
            None
        }
    }

    pub fn as_rela(self) -> Option<Rela> {
        Some(Rela::from_rel_and_addend(self.rel, self.addend?))
    }

    pub fn as_rel_or_rela(self) -> RelOrRela {
        match self.addend {
            Some(addend) => RelOrRela::Rela(Rela::from_rel_and_addend(self.rel, addend)),
            None => RelOrRela::Rel(self.rel),
        }
    }
}

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub enum RelOrRela {
    Rel(Rel),
    Rela(Rela),
}

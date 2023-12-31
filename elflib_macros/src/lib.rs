use std::collections::HashMap;

use quote::{quote, ToTokens};

#[derive(Clone)]
struct WithWrapperContext<T: syn::parse::Parse> {
    inner: T,
    context_ty: syn::Type,
}
impl<T: syn::parse::Parse> syn::parse::Parse for WithWrapperContext<T> {
    fn parse(input: syn::parse::ParseStream) -> syn::Result<Self> {
        let inner = input.parse()?;
        let _: syn::Token![=>] = input.parse()?;
        let context_ty = input.parse()?;
        Ok(Self { inner, context_ty })
    }
}

struct RawStructVariants {
    variants: Vec<syn::ItemStruct>,
}
impl syn::parse::Parse for RawStructVariants {
    fn parse(input: syn::parse::ParseStream) -> syn::Result<Self> {
        let mut variants = Vec::new();
        while !input.lookahead1().peek(syn::Token![=>]) {
            variants.push(input.parse()?);
        }
        Ok(Self { variants })
    }
}

#[proc_macro]
pub fn define_raw_struct_generic_bitlen(
    item_tokens: proc_macro::TokenStream,
) -> proc_macro::TokenStream {
    let WithWrapperContext {
        inner: generic_struct,
        context_ty,
    } = syn::parse_macro_input!(item_tokens as WithWrapperContext<syn::ItemStruct>);
    let variants = [32_usize, 64_usize].iter().map(|bitlen| {
        let bitlen_uint_type_str = format!("u{}", bitlen);
        let bitlen_uint_type: syn::Type = syn::parse_str(&bitlen_uint_type_str).unwrap();
        let mut bitlen_specific_struct = generic_struct.clone();
        for field in &mut bitlen_specific_struct.fields {
            if field.ty.to_token_stream().to_string() == "U" {
                field.ty = bitlen_uint_type.clone();
            }
        }
        bitlen_specific_struct.ident =
            quote::format_ident!("{}{}", bitlen_specific_struct.ident, bitlen);
        bitlen_specific_struct
    });
    define_raw_struct_by_variants(
        quote! {
            #(#variants)*
            => #context_ty
        }
        .into(),
    )
}

#[proc_macro]
pub fn define_raw_struct_by_variants(
    item_tokens: proc_macro::TokenStream,
) -> proc_macro::TokenStream {
    let WithWrapperContext {
        inner: RawStructVariants { variants },
        context_ty,
    } = syn::parse_macro_input!(item_tokens as WithWrapperContext<RawStructVariants>);
    if variants.is_empty() {
        return proc_macro::TokenStream::new();
    }
    for variant in &variants {
        if !matches!(variant.fields, syn::Fields::Named(_)) {
            return quote! {
                compile_error!("only structs with named fields are supported");
            }
            .into();
        }
    }

    // make sure that all variants have the same field names
    let field_names = struct_field_names_sorted(&variants[0].fields);
    for variant in &variants[1..] {
        if struct_field_names_sorted(&variant.fields) != field_names {
            return quote! {
                compile_error!("all variants of a struct must have the same field names");
            }
            .into();
        }
    }

    // make sure that all variants have the same field types for fields with the same name, or at least field types of the
    // same kind, which can be converted to each other (integers of different bit length).
    // additionally, while iterating over all variants, decide the maximum bit length required for each field.
    let mut field_type_by_name: HashMap<String, VariantsFieldTypeInfo> =
        struct_field_names(&variants[0].fields)
            .into_iter()
            .zip(
                struct_variant_field_types(&variants[0].fields)
                    .into_iter()
                    .map(|field_type| VariantsFieldTypeInfo {
                        field_type,
                        are_all_fields_of_the_same_type: true,
                    }),
            )
            .collect();
    for variant in &variants[1..] {
        for field in &variant.fields {
            let name = field.ident.as_ref().unwrap().to_string();
            let VariantsFieldTypeInfo {
                field_type: expected_type,
                are_all_fields_of_the_same_type: are_all_fields_the_same,
            } = field_type_by_name.get_mut(&name).unwrap();
            let actual_type = VariantFieldType::from_syn_type(field.ty.clone());

            match (expected_type, actual_type) {
                (
                    VariantFieldType::Int {
                        bit_length: max_bit_length,
                        is_signed: expected_is_signed,
                    },
                    VariantFieldType::Int {
                        bit_length: cur_bit_length,
                        is_signed: actual_is_signed,
                    },
                ) if *expected_is_signed == actual_is_signed => {
                    if cur_bit_length != *max_bit_length {
                        *are_all_fields_the_same = false;

                        if cur_bit_length > *max_bit_length {
                            *max_bit_length = cur_bit_length;
                        }
                    }
                }
                (VariantFieldType::Other(expected_ty), VariantFieldType::Other(actual_ty)) => {
                    let expected_ty_str = expected_ty.to_token_stream().to_string();
                    let actual_ty_str = actual_ty.to_token_stream().to_string();
                    if expected_ty_str != actual_ty_str {
                        if expected_ty_str.starts_with(actual_ty_str.as_str()) {
                            *expected_ty = actual_ty;
                            *are_all_fields_the_same = false
                        } else if actual_ty_str.starts_with(expected_ty_str.as_str()) {
                            // it's ok, it just the same type but with some prefix
                            *are_all_fields_the_same = false
                        } else {
                            let sorted_types_strings = {
                                let mut strs =
                                    [actual_ty_str.to_string(), expected_ty_str.to_string()];
                                strs.sort();
                                strs
                            };
                            let prefix = strings_find_common_prefix(&sorted_types_strings);
                            if sorted_types_strings
                                == [format!("{prefix}32"), format!("{prefix}64")]
                            {
                                // all good, it's just 2 different size variants of the same struct.
                                // use the larger variant as the common case.
                                *expected_ty = syn::parse_str(&format!("{prefix}64")).unwrap();
                                *are_all_fields_the_same = false;
                            } else {
                                // type mismatch
                                let error_msg =
                                    format!("field {name} has multiple different types");
                                return quote! {
                                    compile_error!(#error_msg);
                                }
                                .into();
                            }
                        }
                    }
                }
                _ => {
                    // type mismatch
                    let error_msg = format!("field {name} has multiple different types");
                    return quote! {
                        compile_error!(#error_msg);
                    }
                    .into();
                }
            }
        }
    }

    gen_raw_struct_by_variants(field_names, field_type_by_name, variants, context_ty).into()
}

fn strings_find_common_prefix(strs: &[String]) -> String {
    let mut result = String::new();
    let mut strs_chars: Vec<std::str::Chars> = strs.iter().map(|str| str.chars()).collect();
    'add_chars_loop: loop {
        let Some(cur_char_of_first_str) = strs_chars[0].next() else {
            break 'add_chars_loop;
        };
        for str_chars in &mut strs_chars[1..] {
            let Some(cur_char) = str_chars.next() else {
                break 'add_chars_loop;
            };
            if cur_char != cur_char_of_first_str {
                break 'add_chars_loop;
            }
        }
        result.push(cur_char_of_first_str);
    }
    result
}

fn variants_find_common_name_prefix(variants: &[syn::ItemStruct]) -> String {
    let variant_names: Vec<String> = variants
        .iter()
        .map(|variant| variant.ident.to_string())
        .collect();
    strings_find_common_prefix(&variant_names)
}

fn gen_variant_struct_derives(variant: &syn::ItemStruct) -> proc_macro2::TokenStream {
    if variant.attrs.iter().any(|attr| {
        matches!(attr, syn::Attribute { style: syn::AttrStyle::Outer, meta: syn::Meta::List(syn::MetaList {
            path, ..
        }), .. } if path.to_token_stream().to_string() == "binary_serde_bitfield")
    }) {
        quote! {
            #[derive(Debug, PartialEq, Eq, Clone, Hash)]
        }
    } else {
        quote! {
            #[derive(Debug, BinarySerde, PartialEq, Eq, Clone, Hash)]
        }
    }
}

fn gen_ref_wrapper_derives() -> proc_macro2::TokenStream {
    quote! {
        #[derive(Clone, Debug)]
    }
}

fn gen_enum_derives() -> proc_macro2::TokenStream {
    quote! {
        #[derive(Debug, PartialEq, Eq, Clone, Hash)]
    }
}

fn expr_for_each_enum_variant_of_self(
    expr: proc_macro2::TokenStream,
    variants: &[syn::ItemStruct],
) -> proc_macro2::TokenStream {
    let branches = variants.iter().map(|variant| {
        let ident = &variant.ident;
        quote! {
            Self::#ident(x) => #expr,
        }
    });
    quote! {
        match self {
            #(#branches)*
        }
    }
}

fn gen_ref_wrapper(
    wrapped_type_ident: &syn::Ident,
    context_ty: syn::Type,
) -> proc_macro2::TokenStream {
    let wrapper_ident = quote::format_ident!("{}Ref", wrapped_type_ident);
    let wrapper_derives = gen_ref_wrapper_derives();
    let deref_impl = quote! {
        impl<'a> ::core::ops::Deref for #wrapper_ident<'a> {
            type Target = #wrapped_type_ident;

            fn deref(&self) -> &Self::Target {
                &self.raw
            }
        }
    };
    let deserialize_impl = quote! {
        #[automatically_derived]
        impl<'a> VariantStructBinarySerde<'a> for #wrapper_ident<'a>
            where #wrapped_type_ident: VariantStructBinarySerde<'a, Context = ()>
        {
            type Context = #context_ty;
            fn deserialize(
                deserializer: &mut ::binary_serde::BinaryDeserializerFromBufSafe<'a>,
                parser: &ElfParser<'a>,
                context: #context_ty,
            ) -> ::core::result::Result<Self, ::binary_serde::BinarySerdeBufSafeError> {
                Ok(Self {
                    raw: #wrapped_type_ident::deserialize(deserializer, parser, ())?,
                    parser: parser.clone(),
                    context,
                })
            }
            fn record_len(file_info: &ElfFileInfo) -> usize {
                #wrapped_type_ident::record_len(file_info)
            }
            fn serialize(&self, buf: &mut [u8], endianness: ::binary_serde::Endianness) {
                self.raw.serialize(buf, endianness);
            }
        }
    };
    quote! {
        #wrapper_derives
        pub struct #wrapper_ident<'a> {
            pub(crate) raw: #wrapped_type_ident,
            pub(crate) parser: ElfParser<'a>,
            pub(crate) context: #context_ty,
        }

        impl<'a> #wrapper_ident<'a> {
            pub fn raw(&self) -> &#wrapped_type_ident {
                &self.raw
            }
        }

        #deref_impl
        #deserialize_impl
    }
}

fn are_variants_of_generic_bitlen_struct(variants: &[syn::ItemStruct]) -> bool {
    let sorted_names = {
        let mut names: Vec<String> = variants
            .iter()
            .map(|variant| variant.ident.to_string())
            .collect();
        names.sort();
        names
    };
    let prefix = variants_find_common_name_prefix(variants);
    let generic_bitlen_names = [format!("{}32", prefix), format!("{}64", prefix)];
    sorted_names.as_slice() == generic_bitlen_names.as_slice()
}

fn gen_raw_struct_by_variants(
    field_names: Vec<String>,
    field_type_by_name: HashMap<String, VariantsFieldTypeInfo>,
    mut variants: Vec<syn::ItemStruct>,
    context_ty: syn::Type,
) -> proc_macro2::TokenStream {
    let variant_names_common_prefix = variants_find_common_name_prefix(&variants);
    if variant_names_common_prefix.is_empty() {
        return quote! {
            compile_error!("variants must have a shared name prefix");
        };
    }

    // make all variant structs and their fields public
    let public_visibility =
        syn::Visibility::Public(syn::Token![pub](proc_macro2::Span::mixed_site()));
    for variant in &mut variants {
        variant.vis = public_visibility.clone();
        for field in &mut variant.fields {
            field.vis = public_visibility.clone();
        }
    }
    let enum_variants = variants.iter().map(|variant| {
        let ident = &variant.ident;
        quote! {
            #ident(#ident),
        }
    });
    let field_access_fns = field_names.iter().map(|field_name| {
        let field_name_ident = quote::format_ident!("{}", field_name);
        let type_info = &field_type_by_name[field_name];
        let ty = type_info.field_type.to_syn_type();
        let field_access_methods =
            type_info
                .field_access_methods()
                .iter()
                .map(|field_access_method| {
                    field_access_method.build_method(
                    field_name,
                    &ty,
                    match field_access_method {
                        FieldAccessMethod::Get => {
                            if type_info.are_all_fields_of_the_same_type {
                                expr_for_each_enum_variant_of_self(
                                    quote! {x.#field_name_ident},
                                    &variants,
                                )
                            } else {
                                expr_for_each_enum_variant_of_self(
                                    quote! {x.#field_name_ident.try_into().unwrap()},
                                    &variants,
                                )
                            }
                        }
                        FieldAccessMethod::Set => {
                            if type_info.are_all_fields_of_the_same_type {
                                expr_for_each_enum_variant_of_self(
                                    quote! {x.#field_name_ident = new_value},
                                    &variants,
                                )
                            } else {
                                expr_for_each_enum_variant_of_self(
                                    quote! {x.#field_name_ident = new_value.try_into().unwrap()},
                                    &variants,
                                )
                            }
                        }
                        FieldAccessMethod::GetByRef => expr_for_each_enum_variant_of_self(
                            quote! {&x.#field_name_ident},
                            &variants,
                        ),
                        FieldAccessMethod::GetMut => expr_for_each_enum_variant_of_self(
                            quote! {&mut x.#field_name_ident},
                            &variants,
                        ),
                    },
                )
                });
        quote! {
            #(#field_access_methods)*
        }
    });
    let enum_derives = gen_enum_derives();
    let enum_ident = quote::format_ident!("{}", &variant_names_common_prefix,);
    let is_generic_bitlen_struct = are_variants_of_generic_bitlen_struct(&variants);
    let deserialize_impl = if is_generic_bitlen_struct {
        let self_32 = quote::format_ident!("{}32", enum_ident);
        let self_64 = quote::format_ident!("{}64", enum_ident);
        quote! {
            #[automatically_derived]
            impl<'a> VariantStructBinarySerde<'a> for #enum_ident {
                type Context = ();
                fn deserialize(
                    deserializer: &mut ::binary_serde::BinaryDeserializerFromBufSafe<'a>,
                    parser: &ElfParser<'a>,
                    context: (),
                ) -> ::core::result::Result<Self, ::binary_serde::BinarySerdeBufSafeError> {
                    match parser.file_info().bit_length {
                        ArchBitLength::Arch32Bit => Ok(Self::#self_32(deserializer.deserialize()?)),
                        ArchBitLength::Arch64Bit => Ok(Self::#self_64(deserializer.deserialize()?)),
                    }
                }
                fn record_len(file_info: &ElfFileInfo) -> usize {
                    match file_info.bit_length {
                        ArchBitLength::Arch32Bit => <#self_32 as BinarySerde>::SERIALIZED_SIZE,
                        ArchBitLength::Arch64Bit => <#self_64 as BinarySerde>::SERIALIZED_SIZE,
                    }
                }
                fn serialize(&self, buf: &mut [u8], endianness: ::binary_serde::Endianness) {
                    match self {
                        Self::#self_32(x) => x.binary_serialize(buf, endianness),
                        Self::#self_64(x) => x.binary_serialize(buf, endianness),
                    }
                }
            }
        }
    } else {
        quote! {}
    };
    let combined_enum = quote! {
        #enum_derives
        pub enum #enum_ident {
            #(#enum_variants)*
        }
        impl #enum_ident {
            #(#field_access_fns)*
        }
        #deserialize_impl
    };
    let ref_wrapper = gen_ref_wrapper(&enum_ident, context_ty);
    let variants_with_derives = variants.into_iter().map(|variant| {
        let derives = gen_variant_struct_derives(&variant);
        quote! {
            #derives
            #variant
        }
    });
    quote! {
        #(
            #variants_with_derives
        )*
        #combined_enum
        #ref_wrapper
    }
}

fn struct_field_names(fields: &syn::Fields) -> Vec<String> {
    fields
        .iter()
        .map(|field| field.ident.as_ref().unwrap().to_string())
        .collect()
}

fn struct_field_names_sorted(fields: &syn::Fields) -> Vec<String> {
    let mut field_names = struct_field_names(fields);
    field_names.sort();
    field_names
}

fn struct_variant_field_types(fields: &syn::Fields) -> Vec<VariantFieldType> {
    fields
        .iter()
        .map(|field| VariantFieldType::from_syn_type(field.ty.clone()))
        .collect()
}

enum FieldAccessMethod {
    Get,
    Set,
    GetByRef,
    GetMut,
}
impl FieldAccessMethod {
    pub fn method_ident(&self, field_name: &str) -> syn::Ident {
        match self {
            FieldAccessMethod::Get => quote::format_ident!("{}", field_name),
            FieldAccessMethod::Set => quote::format_ident!("set_{}", field_name),
            FieldAccessMethod::GetByRef => quote::format_ident!("{}", field_name),
            FieldAccessMethod::GetMut => quote::format_ident!("{}_mut", field_name),
        }
    }
    pub fn build_method(
        &self,
        field_name: &str,
        field_ty: &syn::Type,
        body: proc_macro2::TokenStream,
    ) -> proc_macro2::TokenStream {
        let method_ident = self.method_ident(field_name);
        match self {
            FieldAccessMethod::Get => {
                quote! {
                    pub fn #method_ident(&self) -> #field_ty {
                        #body
                    }
                }
            }
            FieldAccessMethod::Set => {
                quote! {
                    pub fn #method_ident(&mut self, new_value: #field_ty) {
                        #body
                    }
                }
            }
            FieldAccessMethod::GetByRef => {
                quote! {
                    pub fn #method_ident(&self) -> &#field_ty {
                        #body
                    }
                }
            }
            FieldAccessMethod::GetMut => {
                quote! {
                    pub fn #method_ident(&mut self) -> &mut #field_ty {
                        #body
                    }
                }
            }
        }
    }
}

struct VariantsFieldTypeInfo {
    field_type: VariantFieldType,
    are_all_fields_of_the_same_type: bool,
}
impl VariantsFieldTypeInfo {
    pub fn field_access_methods(&self) -> &'static [FieldAccessMethod] {
        if self.are_all_fields_of_the_same_type {
            match self.field_type {
                VariantFieldType::Int { .. } => &[
                    FieldAccessMethod::Get,
                    FieldAccessMethod::GetMut,
                    FieldAccessMethod::Set,
                ],
                VariantFieldType::Other(_) => &[
                    FieldAccessMethod::GetByRef,
                    FieldAccessMethod::GetMut,
                    FieldAccessMethod::Set,
                ],
            }
        } else {
            &[FieldAccessMethod::Get, FieldAccessMethod::Set]
        }
    }
}

enum VariantFieldType {
    Int { bit_length: usize, is_signed: bool },
    Other(syn::Type),
}
impl VariantFieldType {
    fn from_syn_type(ty: syn::Type) -> Self {
        let type_str = ty.to_token_stream().to_string();
        let Some(&first_char) = type_str.as_bytes().get(0) else {
            return Self::Other(ty);
        };
        if first_char == b'u' || first_char == b'i' {
            let Ok(bit_length) = type_str[1..].parse() else {
                return Self::Other(ty);
            };
            Self::Int {
                bit_length,
                is_signed: first_char == b'i',
            }
        } else {
            Self::Other(ty)
        }
    }
    fn to_syn_type(&self) -> syn::Type {
        match &self {
            VariantFieldType::Int {
                bit_length,
                is_signed,
            } => {
                let first_char = if *is_signed { 'i' } else { 'u' };
                syn::parse_str(&format!("{}{}", first_char, bit_length)).unwrap()
            }
            VariantFieldType::Other(ty) => ty.clone(),
        }
    }
}

use std::collections::HashMap;

use quote::{quote, quote_spanned, ToTokens};

struct RawStructVariants {
    variants: Vec<syn::ItemStruct>,
}
impl syn::parse::Parse for RawStructVariants {
    fn parse(input: syn::parse::ParseStream) -> syn::Result<Self> {
        let mut variants = Vec::new();
        while !input.is_empty() {
            variants.push(input.parse()?);
        }
        Ok(Self { variants })
    }
}

#[proc_macro]
pub fn define_raw_struct_generic_bitlen(
    item_tokens: proc_macro::TokenStream,
) -> proc_macro::TokenStream {
    let generic_struct = syn::parse_macro_input!(item_tokens as syn::ItemStruct);
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
        }
        .into(),
    )
}

#[proc_macro]
pub fn define_raw_struct_by_variants(
    item_tokens: proc_macro::TokenStream,
) -> proc_macro::TokenStream {
    let variants = syn::parse_macro_input!(item_tokens as RawStructVariants).variants;
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
                (VariantFieldType::Other(expected_ty), VariantFieldType::Other(actual_ty))
                    if expected_ty.to_token_stream().to_string()
                        == actual_ty.to_token_stream().to_string() => {}
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

    gen_raw_struct_by_variants(field_names, field_type_by_name, variants).into()
}

fn variants_find_common_name_prefix(variants: &[syn::ItemStruct]) -> String {
    let mut result = String::new();
    let variants_names: Vec<String> = variants
        .iter()
        .map(|variant| variant.ident.to_string())
        .collect();
    let mut variant_names_chars: Vec<std::str::Chars> = variants_names
        .iter()
        .map(|variant_name| variant_name.chars())
        .collect();
    'add_chars_loop: loop {
        let Some(cur_char_of_first_variant) = variant_names_chars[0].next() else {
            break 'add_chars_loop;
        };
        for variant_name_chars in &mut variant_names_chars[1..] {
            let Some(cur_char) = variant_name_chars.next() else {
                break 'add_chars_loop;
            };
            if cur_char != cur_char_of_first_variant {
                break 'add_chars_loop;
            }
        }
        result.push(cur_char_of_first_variant);
    }
    result
}

fn gen_struct_derives() -> proc_macro2::TokenStream {
    quote! {
        #[derive(Debug, BinarySerde, PartialEq, Eq, Clone, Hash)]
    }
}

fn gen_wrapper_derives() -> proc_macro2::TokenStream {
    quote! {
        #[derive(Debug)]
    }
}

fn gen_enum_derives() -> proc_macro2::TokenStream {
    quote! {
        #[derive(Debug)]
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
    field_names: &[String],
    field_type_by_name: &HashMap<String, VariantsFieldTypeInfo>,
) -> proc_macro2::TokenStream {
    let wrapper_ident = quote::format_ident!("{}Ref", wrapped_type_ident);
    let wrapper_derives = gen_wrapper_derives();
    let methods = field_names.iter().map(|field_name| {
        let field_type_info = &field_type_by_name[field_name];
        let field_ty = field_type_info.field_type.to_syn_type();
        let field_methods = field_type_info
            .field_access_methods()
            .iter()
            .map(|access_method| {
                let method_ident = access_method.method_ident(field_name);
                access_method.build_method(
                    field_name,
                    &field_ty,
                    match access_method {
                        FieldAccessMethod::Get => quote! {self.raw.#method_ident()},
                        FieldAccessMethod::Set => quote! {self.raw.#method_ident(new_value)},
                        FieldAccessMethod::GetByRef => quote! {self.raw.#method_ident()},
                        FieldAccessMethod::GetMut => quote! {self.raw.#method_ident()},
                    },
                )
            });
        quote! {
            #(#field_methods)*
        }
    });
    quote! {
        #wrapper_derives
        pub struct #wrapper_ident<'a> {
            raw: #wrapped_type_ident,
            bytes: &'a [u8],
        }

        impl<'a> #wrapper_ident<'a> {
            #(#methods)*
        }
    }
}

fn gen_raw_struct_by_variants(
    field_names: Vec<String>,
    field_type_by_name: HashMap<String, VariantsFieldTypeInfo>,
    mut variants: Vec<syn::ItemStruct>,
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
                            FieldAccessMethod::Get => expr_for_each_enum_variant_of_self(
                                quote! {x.#field_name_ident as #ty},
                                &variants,
                            ),
                            FieldAccessMethod::Set => expr_for_each_enum_variant_of_self(
                                quote! {x.#field_name_ident = new_value as _},
                                &variants,
                            ),
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
    let combined_enum = quote! {
        #enum_derives
        pub enum #enum_ident {
            #(#enum_variants)*
        }
        impl #enum_ident {
            #(#field_access_fns)*
        }
    };
    let ref_wrapper = gen_ref_wrapper(&enum_ident, &field_names, &field_type_by_name);
    let struct_derives = gen_struct_derives();
    quote! {
        #(
            #struct_derives
            #variants
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
                    pub fn #method_ident(self) -> #field_ty {
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
            &[
                FieldAccessMethod::GetByRef,
                FieldAccessMethod::GetMut,
                FieldAccessMethod::Set,
            ]
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

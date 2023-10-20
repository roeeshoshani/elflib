macro_rules! generate_raw_struct_ref {
    {generate_ref} => {
        compile_error!("fuck");
    };
    {} => {};
}

macro_rules! generic_bitness_for_each_bit_size {
    {$value: expr, $inner_ident: ident, $do: block} => {
        match $value {
            Self::B32($inner_ident) => $do,
            Self::B64($inner_ident) => $do,
        }
    };
}

macro_rules! define_raw_struct_generic_bitness_field_ty {
    {U, $bitness_uint_type: ty} => {
        $bitness_uint_type
    };
    {$field_ty: ty, $bitness_uint_type: ty} => {
        $field_ty
    };
}

macro_rules! define_raw_struct_generic_bitness_field_mut_set {
    {$field_name: ident : $field_ty: ty} => {
        paste::paste! {
            pub fn [<$field_name _mut>](&mut self) -> &mut $field_ty {
                generic_bitness_for_each_bit_size!{self, x, {
                    &mut x.$field_name
                }}
            }
            pub fn [<set_ $field_name>](&mut self, new_value: $field_ty) {
                generic_bitness_for_each_bit_size!{self, x, {
                    x.$field_name = new_value;
                }}
            }
        }
    };
}

macro_rules! define_raw_struct_generic_bitness_field_getset {
    {$field_name: ident : U} => {
        pub fn $field_name(&self) -> u64 {
            generic_bitness_for_each_bit_size!{self, x, {
                x.$field_name as u64
            }}
        }

        paste::paste! {
            pub fn [<set_ $field_name>](&mut self, new_value: u64) {
                generic_bitness_for_each_bit_size!{self, x, {
                    x.$field_name = new_value.try_into().unwrap();
                }}
            }
        }
    };
    {$field_name: ident : $field_ty: ty} => {
        pub fn $field_name(&self) -> $field_ty {
            generic_bitness_for_each_bit_size!{self, x, {
                x.$field_name
            }}
        }

        define_raw_struct_generic_bitness_field_mut_set!{$field_name : $field_ty}
    };

    {$field_name: ident : $field_ty: ty [ref]} => {
        pub fn $field_name(&self) -> &$field_ty {
            generic_bitness_for_each_bit_size!{self, x, {
                &x.$field_name
            }}
        }

        define_raw_struct_generic_bitness_field_mut_set!{$field_name : $field_ty}
    };
}

macro_rules! define_raw_struct_generic_bitness {
    {
        struct $struct_name: ident {
            $(
                $field_name: ident : $field_ty: ty $([$ref:tt])?,
            )+
        }
        $($generate_ref: tt)?
    } => {
        paste::paste! {
            #[derive(Debug)]
            pub struct [<$struct_name 32>] {
                $(
                    $field_name: define_raw_struct_generic_bitness_field_ty! {$field_ty, u32},
                )+
            }

            #[derive(Debug)]
            pub struct [<$struct_name 64>] {
                $(
                    $field_name: define_raw_struct_generic_bitness_field_ty! {$field_ty, u64},
                )+
            }

            #[derive(Debug)]
            pub enum $struct_name {
                B32([<$struct_name 32>]),
                B64([<$struct_name 64>]),
            }

            impl $struct_name {
                $(
                    define_raw_struct_generic_bitness_field_getset!{$field_name : $field_ty $([$ref])?}
                )+
            }
        }
        generate_raw_struct_ref!{$($generate_ref)?}
    };
}

macro_rules! raw_struct_for_each_variant {
    {$value: expr, [$($variant_name: ident,)+], $inner_ident: ident, $do: block} => {
        match $value {
            $(
                Self::$variant_name($inner_ident) => $do,
            )+
        }
    };
}

macro_rules! define_raw_struct_by_variants_field_getset {
    {$field_name: ident : $field_ty: ty, $($variant_name: ident,)+} => {
        pub fn $field_name(&self) -> $field_ty {
            raw_struct_for_each_variant!{self, [$($variant_name,)+], x, {
                x.$field_name
            }}
        }

        paste::paste!{
            pub fn [<$field_name _mut>](&mut self) -> &mut $field_ty {
                raw_struct_for_each_variant!{self, [$($variant_name,)+], x, {
                    &mut x.$field_name
                }}
            }
            pub fn [<set_ $field_name>](&mut self, new_value: $field_ty) {
                raw_struct_for_each_variant!{self, [$($variant_name,)+], x, {
                    x.$field_name = new_value;
                }}
            }
        }
    };

    {$field_name: ident : $field_ty: ty [diff], $($variant_name: ident,)+} => {
        pub fn $field_name(&self) -> $field_ty {
            raw_struct_for_each_variant!{self, [$($variant_name,)+], x, {
                x.$field_name as $field_ty
            }}
        }

        paste::paste!{
            pub fn [<set_ $field_name>](&mut self, new_value: $field_ty) {
                raw_struct_for_each_variant!{self, [$($variant_name,)+], x, {
                    x.$field_name = new_value.try_into().unwrap();
                }}
            }
        }
    };

    {$field_name: ident : $field_ty: ty [ref], $($variant_name: ident,)+ } => {
        pub fn $field_name(&self) -> &$field_ty {
            raw_struct_for_each_variant!{self, [$($variant_name,)+], x, {
                &x.$field_name
            }}
        }

        paste::paste!{
            pub fn [<$field_name _mut>](&mut self) -> &mut $field_ty {
                raw_struct_for_each_variant!{self, [$($variant_name,)+], x, {
                    &mut x.$field_name
                }}
            }
            pub fn [<set_ $field_name>](&mut self, new_value: $field_ty) {
                raw_struct_for_each_variant!{self, [$($variant_name,)+], x, {
                    x.$field_name = new_value;
                }}
            }
        }
    };
}

macro_rules! define_raw_struct_by_variants_fields_getset {
    {
        {
            $first_field_name: ident : $first_field_ty: ty $([$first_attr: tt])?,
            $($field_name: ident : $field_ty: ty $([$attr: tt])?,)*
        }
        {
            $($variant_name: ident,)+
        }
        {
            $($cur_content: item)*
        }
    } => {
        define_raw_struct_by_variants_fields_getset!{
            {
                $($field_name: $field_ty $([$attr])?,)*
            }
            { $($variant_name,)+ }
            {
                $($cur_content)*
                define_raw_struct_by_variants_field_getset! {$first_field_name : $first_field_ty $([$first_attr])?, $($variant_name,)+}
            }
        }
    };

    {
        { }
        {
            $($variant_name: ident,)+
        }
        {
            $($cur_content: item)*
        }
    } => {
        $($cur_content)*
    };
}

macro_rules! define_raw_struct_by_variants {
    {
        struct $struct_name: ident {
            $(
                $field_name: ident : $field_ty: ty $([$attr: tt])?,
            )+
        }
        variants {
            $(
                struct $postfix: ident {
                    $(
                        $variant_field_name: ident : $variant_field_ty: ty,
                    )+
                }
            )+
        }
        $($generate_ref:tt)?
    } => {
        paste::paste!{
            $(
                #[derive(Debug)]
                pub struct [<$struct_name $postfix>] {
                    $(
                        $variant_field_name : $variant_field_ty,
                    )+
                }
            )+

            #[derive(Debug)]
            pub enum $struct_name {
                $(
                    [<$struct_name $postfix>]([<$struct_name $postfix>]),
                )+
            }
            impl $struct_name {
                define_raw_struct_by_variants_fields_getset!{
                    {
                        $($field_name: $field_ty $([$attr])?,)*
                    }
                    { $([<$struct_name $postfix>],)+ }
                    {
                    }
                }
            }
        }
        generate_raw_struct_ref!{$($generate_ref)?}
    };
}

use proc_macro2::TokenStream;
use quote::ToTokens;
use rustdoc_types::{Function, GenericArgs, Impl, ItemEnum, Path, Struct, Type, Variant};

macro_rules! matches {
    ($s: expr, $pat: pat => $inner: expr) => {
        match $s {
            $pat => Some($inner),
            _ => None,
        }
    };
}

pub trait OptionExt {
    fn quote(&self) -> TokenStream;
}

impl<T: ToTokens> OptionExt for Option<T> {
    fn quote(&self) -> TokenStream {
        match self {
            Some(o) => quote::quote!(Some(#o)),
            None => quote::quote!(None),
        }
    }
}

pub trait ItemEnumExt {
    fn as_impl(&self) -> Option<&Impl>;
    fn as_struct(&self) -> Option<&Struct>;
    fn as_struct_field(&self) -> Option<&Type>;
    fn as_variant(&self) -> Option<&Variant>;
    fn as_fn(&self) -> Option<&Function>;
}

impl ItemEnumExt for ItemEnum {
    fn as_impl(&self) -> Option<&Impl> {
        matches!(self, Self::Impl(i) => i)
    }

    fn as_struct(&self) -> Option<&Struct> {
        matches!(self, Self::Struct(i) => i)
    }

    fn as_struct_field(&self) -> Option<&Type> {
        matches!(self, Self::StructField(i) => i)
    }

    fn as_variant(&self) -> Option<&Variant> {
        matches!(self, Self::Variant(i) => i)
    }

    fn as_fn(&self) -> Option<&Function> {
        matches!(self, Self::Function(i) => i)
    }
}

pub trait TypeExt {
    fn as_resolved_path(&self) -> Option<&Path>;
}

impl TypeExt for Type {
    fn as_resolved_path(&self) -> Option<&Path> {
        matches!(self, Self::ResolvedPath(i) => i)
    }
}

pub trait GenericArgsExt {
    fn is_empty(&self) -> bool;
}

impl GenericArgsExt for GenericArgs {
    fn is_empty(&self) -> bool {
        match self {
            GenericArgs::AngleBracketed { args, bindings } => {
                args.is_empty() && bindings.is_empty()
            }
            GenericArgs::Parenthesized { .. } => false,
        }
    }
}

pub trait OptionBoxGenericArgsExt {
    fn as_non_empty(&self) -> Option<&Box<GenericArgs>>;
}

impl OptionBoxGenericArgsExt for Option<Box<GenericArgs>> {
    fn as_non_empty(&self) -> Option<&Box<GenericArgs>> {
        self.as_ref().filter(|a| !a.is_empty())
    }
}

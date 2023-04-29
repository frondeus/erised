use proc_macro2::TokenStream;
use quote::quote;

use crate::heap_types::Item;

impl Item {
    pub(crate) fn access(&self) -> TokenStream {
        match self {
            Item::Module(_)
            | Item::ExternCrate { .. }
            | Item::Import(_)
            | Item::Union(_)
            | Item::Function(_)
            | Item::Trait(_)
            | Item::TraitAlias(_)
            | Item::Impl(_)
            | Item::Typedef(_)
            | Item::OpaqueTy(_)
            | Item::Constant(_)
            | Item::Static(_)
            | Item::ForeignType
            | Item::Macro(_)
            | Item::ProcMacro(_)
            | Item::Primitive(_)
            | Item::AssocConst { .. }
            | Item::AssocType { .. } => quote!(todo!()),
            Item::Struct(strukt) => {
                let path = strukt.meta.get_formatted_path();
                quote!(
                    <#path as Reflect>::TYPE_INFO
                )
            }
            Item::Enum(enum_) => {
                let path = enum_.meta.get_formatted_path();
                quote!(
                    <#path as Reflect>::TYPE_INFO
                )
            }
        }
    }
}

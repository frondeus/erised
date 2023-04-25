use proc_macro2::TokenStream;
use quote::quote;

use crate::heap_types::Item;

impl Item {
    pub(crate) fn access(&self) -> TokenStream {
        match self {
            Item::Module(_) => quote!(),
            Item::ExternCrate { meta, name, rename } => quote!(),
            Item::Import(_) => quote!(),
            Item::Union(_) => quote!(),
            Item::Struct(_) => quote!(),
            Item::Enum(_) => quote!(),
            Item::Function(_) => quote!(),
            Item::Trait(_) => quote!(),
            Item::TraitAlias(_) => quote!(),
            Item::Impl(_) => quote!(),
            Item::Typedef(_) => quote!(),
            Item::OpaqueTy(_) => quote!(),
            Item::Constant(_) => quote!(),
            Item::Static(_) => quote!(),
            Item::ForeignType => quote!(),
            Item::Macro(_) => quote!(),
            Item::ProcMacro(_) => quote!(),
            Item::Primitive(_) => quote!(),
            Item::AssocConst {
                meta,
                type_,
                default,
            } => quote!(),
            Item::AssocType {
                meta,
                generics,
                bounds,
                default,
            } => quote!(),
        };

        quote!(todo!())
    }
}

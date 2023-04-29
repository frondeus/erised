use inflector::Inflector;
use proc_macro2::TokenStream;
use quote::format_ident;

use crate::heap_types::{Crate, ItemMeta};

impl ItemMeta {
    pub(crate) fn get_formatted_path(&self) -> TokenStream {
        let summary = self.summary.as_ref().unwrap();
        let first: Option<&str> = summary.path.first().map(|t| t.as_str());
        let is_crate = first.unwrap_or_default() == "crate";

        let paths = summary
            .path
            .iter()
            .map(|segment| format_ident!("{}", segment));

        if is_crate {
            quote::quote!(#(#paths)::*)
        } else {
            quote::quote!(::#(#paths)::*)
        }
    }
}
impl Crate {
    pub fn generate_static(&self) -> TokenStream {
        let name = &self.root.name;
        let _uppercase_name = format_ident!("{}", name.to_screaming_snake_case());
        let lowercase_name = format_ident!("{}", name.to_snake_case());

        let mut items = vec![];

        for item in &self.all_items {
            let item = &**item;
            match item {
                crate::heap_types::Item::Module(_)
                | crate::heap_types::Item::ExternCrate { .. }
                | crate::heap_types::Item::Import(_)
                | crate::heap_types::Item::Union(_)
                | crate::heap_types::Item::Function(_)
                | crate::heap_types::Item::Trait(_)
                | crate::heap_types::Item::TraitAlias(_)
                | crate::heap_types::Item::Impl(_)
                | crate::heap_types::Item::Typedef(_)
                | crate::heap_types::Item::OpaqueTy(_)
                | crate::heap_types::Item::Constant(_)
                | crate::heap_types::Item::Static(_)
                | crate::heap_types::Item::ForeignType
                | crate::heap_types::Item::Macro(_)
                | crate::heap_types::Item::ProcMacro(_)
                | crate::heap_types::Item::Primitive(_)
                | crate::heap_types::Item::AssocConst { .. }
                | crate::heap_types::Item::AssocType { .. } => (),
                crate::heap_types::Item::Enum(enum_) => {
                    // let uppercase_name = format_ident!("{}", enum_.name.to_screaming_snake_case());
                    let path = enum_.meta.get_formatted_path();
                    items.push(quote::quote!(
                        impl Reflect for #path {
                            const TYPE_INFO: erised::types::Item = #item;
                        }
                    ));
                }
                crate::heap_types::Item::Struct(strukt) => {
                    // let uppercase_name = format_ident!("{}", strukt.name.to_screaming_snake_case());
                    let path = strukt.meta.get_formatted_path();
                    items.push(quote::quote!(
                        impl Reflect for #path {
                            const TYPE_INFO: erised::types::Item = #item;
                        }
                    ));
                }
            }
        }

        quote::quote!(
            pub trait Reflect {
                const TYPE_INFO: erised::types::Item;
            }
            pub mod #lowercase_name {
                use super::*;

                #(#items)*
            }
            // pub const #uppercase_name: erised::types::Crate = #self;
        )
    }
}

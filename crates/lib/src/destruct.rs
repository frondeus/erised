use std::{
    collections::{BTreeMap, HashMap},
    path::PathBuf,
    sync::{Arc, Weak},
};

use inflector::Inflector;
use quote::quote;

use crate::heap_types::{Id, Item};

#[derive(Default)]
pub struct ItemsPaths {
    defs: HashMap<Id, ItemDef>,
    counter: usize,
}

impl ItemsPaths {
    pub(crate) fn defs(self) -> BTreeMap<String, proc_macro2::TokenStream> {
        self.defs
            .into_values()
            .map(|def| (def.accessor.to_string(), def.def))
            .collect()
    }
}

pub struct ItemDef {
    pub def: proc_macro2::TokenStream,
    pub accessor: proc_macro2::TokenStream,
}

pub trait ToTokens {
    fn to_tokens(&self, paths: &mut ItemsPaths) -> proc_macro2::TokenStream;
}

//---- Concrete

impl ToTokens for PathBuf {
    fn to_tokens(&self, _paths: &mut ItemsPaths) -> proc_macro2::TokenStream {
        let p = self.display().to_string();
        quote!(
            #p
        )
    }
}

impl ToTokens for String {
    fn to_tokens(&self, _paths: &mut ItemsPaths) -> proc_macro2::TokenStream {
        quote!(
            #self
        )
    }
}

//---- Abstract

impl<T: ToTokens> ToTokens for Vec<T> {
    fn to_tokens(&self, paths: &mut ItemsPaths) -> proc_macro2::TokenStream {
        let inner = self.iter().map(|inner| ToTokens::to_tokens(inner, paths));
        quote!(
            &[ #(#inner),* ]
        )
    }
}

impl<T: ToTokens> ToTokens for Option<T> {
    fn to_tokens(&self, paths: &mut ItemsPaths) -> proc_macro2::TokenStream {
        match self {
            Some(t) => {
                // let t = ToTokens::to_tokens(t);
                // let inner: &T = t;
                let inner = ToTokens::to_tokens(t, paths);
                // let inner = inner.access();
                quote!(Some(#inner))
            }
            None => quote!(None),
        }
    }
}

impl<T: ToTokens> ToTokens for Box<T> {
    fn to_tokens(&self, paths: &mut ItemsPaths) -> proc_macro2::TokenStream {
        let inner: &T = self;
        let inner = ToTokens::to_tokens(inner, paths);
        // let inner = inner.access();
        quote!(
            || #inner
        )
    }
}

impl<T: ToTokens> ToTokens for Arc<T> {
    fn to_tokens(&self, paths: &mut ItemsPaths) -> proc_macro2::TokenStream {
        let inner: &T = self;
        let inner = ToTokens::to_tokens(inner, paths);
        // let inner = inner.access();
        quote!(
            || #inner
        )
    }
}

impl ToTokens for Weak<Item> {
    fn to_tokens(&self, paths: &mut ItemsPaths) -> proc_macro2::TokenStream {
        match self.upgrade() {
            Some(inner) => {
                let inner: &Item = &inner;
                let meta = inner.meta();
                // println!("LOOKING FOR CACHED {} ", meta.id.0);

                if let Some(cached) = paths.defs.get(&meta.id) {
                    // println!("FOUND CACHED {}", meta.id.0);
                    let inner = &cached.accessor;
                    return quote!( || #inner );
                }

                let uppercase_name = quote::format_ident!(
                    "{}",
                    inner.name(&mut paths.counter).to_screaming_snake_case()
                );

                // println!("INSERTING {uppercase_name} TO CACHE AS {}", meta.id.0);

                paths.defs.insert(
                    meta.id.clone(),
                    ItemDef {
                        def: proc_macro2::TokenStream::new(),
                        accessor: quote!(#uppercase_name),
                    },
                );

                // println!("{{ BUILDING DEFINITION");

                let def = ToTokens::to_tokens(inner, paths);

                // println!("}} FINISHED DEFINITION");

                let def = quote!(const #uppercase_name: erised::types::Item = #def;);

                let item_def = paths.defs.get_mut(&meta.id).unwrap(); // we just inserted it
                let inner = &item_def.accessor;
                item_def.def = def;

                quote!(
                    || #inner
                )
            }
            None => {
                quote!(|| todo!("Weak pointer to Item is dangling"))
            }
        }
    }
}

impl<A: quote::ToTokens, B: quote::ToTokens> ToTokens for (A, B) {
    fn to_tokens(&self, paths: &mut ItemsPaths) -> proc_macro2::TokenStream {
        let (a, b) = self;
        // let a = Destructive::to_tokens(a);
        // let b = Destructive::to_tokens(b);
        quote!(
            (#a, #b)
        )
    }
}

impl<K: quote::ToTokens, V: quote::ToTokens> ToTokens for HashMap<K, V> {
    fn to_tokens(&self, paths: &mut ItemsPaths) -> proc_macro2::TokenStream {
        let v = self.iter().map(|(k, v)| {
            quote!(
                ( #k, #v )
            )
        });
        quote!(
            &[#( #v ),*]
        )
    }
}

macro_rules! primitive_to_tokens {
    ($a: ty) => {
        impl ToTokens for $a {
            fn to_tokens(&self, paths: &mut ItemsPaths) -> proc_macro2::TokenStream {
                quote!( #self )
            }
        }
    };
    ($a: ty, $($rest: tt)+) => {
        primitive_to_tokens!($a);
        primitive_to_tokens!($($rest)*);
    };
}

primitive_to_tokens!(bool);

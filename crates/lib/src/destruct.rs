use std::{
    collections::HashMap,
    path::PathBuf,
    sync::{Arc, Weak},
};

use quote::quote;

use crate::{access::Access, heap_types::Item};

pub trait ToTokens {
    fn to_tokens(&self) -> proc_macro2::TokenStream;
}

//---- Concrete

impl ToTokens for PathBuf {
    fn to_tokens(&self) -> proc_macro2::TokenStream {
        let p = self.display().to_string();
        quote!(
            #p
        )
    }
}

impl ToTokens for String {
    fn to_tokens(&self) -> proc_macro2::TokenStream {
        quote!(
            #self
        )
    }
}

//---- Abstract

impl<T: ToTokens> ToTokens for Vec<T> {
    fn to_tokens(&self) -> proc_macro2::TokenStream {
        let inner = self.iter().map(|inner| ToTokens::to_tokens(inner));
        quote!(
            &[ #(#inner),* ]
        )
    }
}

impl<T: ToTokens> ToTokens for Option<T> {
    fn to_tokens(&self) -> proc_macro2::TokenStream {
        match self {
            Some(t) => {
                let t = ToTokens::to_tokens(t);
                quote!(Some(#t))
            }
            None => quote!(None),
        }
    }
}

impl<T: Access> ToTokens for Box<T> {
    fn to_tokens(&self) -> proc_macro2::TokenStream {
        let inner: &T = self;
        let inner = inner.access();
        quote!(
            || #inner
        )
    }
}

impl<T: Access> ToTokens for Arc<T> {
    fn to_tokens(&self) -> proc_macro2::TokenStream {
        let inner: &T = self;
        let inner = inner.access();
        quote!(
            || #inner
        )
    }
}

impl ToTokens for Weak<Item> {
    fn to_tokens(&self) -> proc_macro2::TokenStream {
        match self.upgrade() {
            Some(inner) => {
                let inner: &Item = &inner;
                let inner = inner.access();
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
    fn to_tokens(&self) -> proc_macro2::TokenStream {
        let (a, b) = self;
        // let a = Destructive::to_tokens(a);
        // let b = Destructive::to_tokens(b);
        quote!(
            (#a, #b)
        )
    }
}

impl<K: quote::ToTokens, V: quote::ToTokens> ToTokens for HashMap<K, V> {
    fn to_tokens(&self) -> proc_macro2::TokenStream {
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

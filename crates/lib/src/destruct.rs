use std::{collections::HashMap, path::PathBuf, sync::Arc};

use quote::quote;

pub trait ToTokens {
    fn to_tokens(&self) -> proc_macro2::TokenStream;
}

impl ToTokens for PathBuf {
    fn to_tokens(&self) -> proc_macro2::TokenStream {
        let p = self.display().to_string();
        quote!(
            #p
        )
    }
}

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

impl<T: ToTokens> ToTokens for Arc<T> {
    fn to_tokens(&self) -> proc_macro2::TokenStream {
        let inner: &T = &*self;
        let inner = ToTokens::to_tokens(inner);
        quote!(
            || #inner
        )
    }
}

impl<T: ToTokens> ToTokens for Box<T> {
    fn to_tokens(&self) -> proc_macro2::TokenStream {
        let inner: &T = &*self;
        let inner = ToTokens::to_tokens(inner);
        quote!(
            || #inner
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

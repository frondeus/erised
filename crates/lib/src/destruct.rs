use std::{collections::HashMap, path::PathBuf};

use quote::{quote, ToTokens};

pub struct Destruct<T>(pub T);

impl ToTokens for Destruct<&PathBuf> {
    fn to_tokens(&self, tokens: &mut proc_macro2::TokenStream) {
        use quote::TokenStreamExt;
        let p = self.0.display().to_string();
        tokens.append_all(quote!(
            #p
        ))
    }
}

impl<T: ToTokens> ToTokens for Destruct<&Option<T>> {
    fn to_tokens(&self, tokens: &mut proc_macro2::TokenStream) {
        use quote::TokenStreamExt;
        tokens.append_all(match &self.0 {
            Some(t) => quote!(Some(#t)),
            None => quote!(None),
        })
    }
}

impl<T: ToTokens> ToTokens for Destruct<&Vec<T>> {
    fn to_tokens(&self, tokens: &mut proc_macro2::TokenStream) {
        use quote::TokenStreamExt;
        let v = &self.0;
        tokens.append_all(quote!(
            &[#(#v),*]
        ))
    }
}

// impl<A: ToTokens, B: ToTokens> ToTokens for Destruct<&Vec<(A, B)>> {
//     fn to_tokens(&self, tokens: &mut proc_macro2::TokenStream) {
//         use quote::TokenStreamExt;
//         let v = &self.0;
//         tokens.append_all(quote!(
//             &[#(#v),*]
//         ))
//     }
// }

impl<A: ToTokens, B: ToTokens> ToTokens for Destruct<&(A, B)> {
    fn to_tokens(&self, tokens: &mut proc_macro2::TokenStream) {
        use quote::TokenStreamExt;
        let (a, b) = &self.0;
        tokens.append_all(quote!(
            ( #a, #b )
        ))
    }
}

impl<K: ToTokens, V: ToTokens> ToTokens for Destruct<&HashMap<K, V>> {
    fn to_tokens(&self, tokens: &mut proc_macro2::TokenStream) {
        use quote::TokenStreamExt;
        let v = self.0.iter().map(|(k, v)| {
            quote!(
                ( #k, #v )
            )
        });
        tokens.append_all(quote!(
            &[#( #v ),*]
        ))
    }
}

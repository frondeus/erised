use darling::FromField;
use proc_macro2::Punct;
use proc_macro2::Spacing;
use proc_macro2::TokenStream as TokenStream2;
use quote::format_ident;
use quote::quote;
use syn::Ident;
use syn::Type;

use crate::type_info::matcher::TokenMatcher;

#[derive(Debug, FromField)]
#[darling(attributes(type_info))]
pub struct TypeInfoField {
    ident: Option<Ident>,
    ty: Type,
}

impl TypeInfoField {
    pub fn ident_quote(&self) -> TokenStream2 {
        match &self.ident {
            Some(ident) => quote!(#ident:),
            None => quote!(),
        }
    }

    pub fn destruct(&self, idx: usize) -> TokenStream2 {
        let ident = match &self.ident {
            Some(ident) => ident.clone(),
            None => format_ident!("_{}", idx),
        };

        quote!(#ident)
    }

    pub fn map(&self, idx: usize) -> TokenStream2 {
        let destructed = self.destruct(idx);

        // TokenMatcher.gen_destruct(self.ty_quote())

        if TokenMatcher.is_destruct(self.ty_quote()) {
            return quote!(erised::Destruct(#destructed));
        }

        quote!(#destructed)
    }

    pub fn ty_quote(&self) -> TokenStream2 {
        let ty = &self.ty;
        quote!(#ty)
    }

    pub fn gen(&self) -> TokenStream2 {
        let ident = self.ident_quote();
        let ty = self.ty_quote();

        let ty = TokenMatcher.gen(ty);
        // if let Some(alias) = self.alias.as_ref() {
        //     return quote!(#ident #alias);
        // };

        quote!(#ident #ty)
    }

    pub fn gen_to_tokens(&self, idx: usize) -> TokenStream2 {
        let ident = self.ident_quote();
        let val = self.destruct(idx);
        let hash = Punct::new('#', Spacing::Alone);
        // let ident_access = self.ident_access(idx);
        quote!(
            #ident #hash #val
        )
    }
}

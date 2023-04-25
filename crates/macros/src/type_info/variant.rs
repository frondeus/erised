use darling::{ast, FromVariant};
use inflector::Inflector;
use proc_macro2::TokenStream;
use quote::{format_ident, quote};
use syn::Ident;

use super::field::TypeInfoField;

#[derive(Debug, FromVariant)]
#[darling(attributes(type_info))]
pub struct TypeInfoVariant {
    ident: Ident,
    fields: ast::Fields<TypeInfoField>,
}

impl TypeInfoVariant {
    pub fn destruct(&self) -> TokenStream {
        let ident = &self.ident;
        let style = &self.fields.style;
        let fields = self
            .fields
            .iter()
            .enumerate()
            .map(|(idx, f)| f.destruct(idx));

        match style {
            ast::Style::Tuple => quote!(
                Self::#ident ( #(#fields),* )
            ),
            ast::Style::Struct => quote!(
                Self::#ident { #(#fields),* }
            ),
            ast::Style::Unit => quote!(
                Self::#ident
            ),
        }
    }

    pub fn gen_as(&self) -> Option<TokenStream> {
        let ident = &self.ident;
        let style = &self.fields.style;

        let as_ident = format_ident!("as_{}", ident.to_string().to_camel_case());
        let fields = &self.fields;

        match style {
            ast::Style::Tuple => {
                if fields.len() == 1 {
                    let field = self.fields.fields.first().unwrap();
                    let field_ty = &field.ty;

                    Some(quote!(
                        pub fn #as_ident(self) -> Option<#field_ty> {
                            match self {
                                Self::#ident(i) => Some(i),
                                _ => None
                            }
                        }
                    ))
                } else {
                    None
                }
            }
            ast::Style::Struct => None,
            ast::Style::Unit => Some(quote!(

                pub fn #as_ident(self) -> Option<()> {
                    match self {
                        Self::#ident => Some(()),
                        _ => None
                    }
                }
            )),
        }
    }

    pub fn gen(&self) -> TokenStream {
        let ident = &self.ident;
        let style = &self.fields.style;
        let fields = self.fields.iter().map(|f| f.gen());

        match style {
            ast::Style::Tuple => quote!(
                #ident ( #(#fields),* )
            ),
            ast::Style::Struct => quote!(
                #ident { #(#fields),* }
            ),
            ast::Style::Unit => quote!(
                #ident
            ),
        }
    }

    pub fn gen_to_tokens(&self, en: &Ident) -> TokenStream {
        let ident = &self.ident;
        let fields = &self.fields;
        let style = &self.fields.style;
        let mapped_fields = fields.iter().enumerate().map(|(idx, f)| f.map(idx));
        let construct = fields
            .iter()
            .enumerate()
            .map(|(idx, f)| f.gen_to_tokens(idx));
        let fields: Vec<_> = fields
            .iter()
            .enumerate()
            .map(|(idx, f)| f.destruct(idx))
            .collect();
        match style {
            ast::Style::Tuple => quote!(
                // let &Self ( #(#fields),* ) = &self;
                let ( #(#fields),* ) = (#(#mapped_fields),*);
                quote::quote!(
                    erised::types::#en::#ident (
                        #(#construct),*
                    )
                )
            ),
            ast::Style::Struct => quote!(
                // let &Self { #(#fields),* } = &self;
                let ( #(#fields),* ) = (#(#mapped_fields),*);
                quote::quote!(
                    erised::types::#en::#ident {
                        #(#construct),*
                    }
                )
            ),
            ast::Style::Unit => quote!(quote::quote!(
                erised::types::#en::#ident
            )),
        }
    }
}

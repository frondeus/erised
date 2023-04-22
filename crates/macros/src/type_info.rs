use darling::{ast, FromDeriveInput};
use proc_macro2::TokenStream as TokenStream2;
use quote::format_ident;
use quote::quote;
use syn::Ident;

use self::field::TypeInfoField;
use self::variant::TypeInfoVariant;

mod variant;

mod field;

mod matcher;

#[derive(Debug, FromDeriveInput)]
#[darling(attributes(type_info))]
pub struct TypeInfo {
    ident: Ident,
    data: ast::Data<TypeInfoVariant, TypeInfoField>,
}

impl TypeInfo {
    fn gen_derive(&self) -> TokenStream2 {
        quote!(#[derive(Debug, PartialEq, Eq, Hash, Clone, Copy)])
    }
    fn gen_to_tokens(&self) -> TokenStream2 {
        let ident = &self.ident;

        let destruct = match &self.data {
            ast::Data::Enum(variants) => {
                let construct = variants.iter().map(|v| v.gen_to_tokens(ident));
                let variants = variants.iter().map(|v| v.destruct());
                quote!(
                    match &self {
                        #(#variants => {#construct}),*
                    }
                )
            }
            ast::Data::Struct(fields) => {
                let style = fields.style;
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
                        let &Self ( #(#fields),* ) = &self;
                        let ( #(#fields),* ) = (#(#mapped_fields),*);
                        quote::quote!(
                            erised::types::#ident (
                                #(#construct),*
                            )
                        )
                    ),
                    ast::Style::Struct => quote!(
                        let &Self { #(#fields),* } = &self;
                        let ( #(#fields),* ) = (#(#mapped_fields),*);
                        // let #static_ident {
                        //     #(#fields),*
                        // } = &self.into();
                        quote::quote!(
                            erised::types::#ident {
                                #(#construct),*
                            }
                        )
                    ),
                    ast::Style::Unit => quote!(quote::quote!(
                    erised::types::#ident
                    )),
                }
            }
        };

        quote!(
            impl erised::destruct::ToTokens for #ident {
                fn to_tokens(&self) -> proc_macro2::TokenStream {
                    #destruct
                }
            }
            impl quote::ToTokens for #ident {
                fn to_tokens(&self, tokens: &mut proc_macro2::TokenStream) {
                    use quote::TokenStreamExt;
                    tokens.append_all(erised::destruct::ToTokens::to_tokens(self))
                }
            }
        )
    }

    fn static_ident(&self) -> Ident {
        format_ident!("Static{}", self.ident)
    }

    pub fn gen(&self) -> TokenStream2 {
        let derive = self.gen_derive();
        let ident = self.static_ident();
        let to_tokens = self.gen_to_tokens();

        let data = match &self.data {
            ast::Data::Enum(variants) => {
                let variants = variants.iter().map(|v| v.gen());
                quote!(
                    pub enum #ident {
                        #(#variants),*
                    }
                )
            }
            ast::Data::Struct(fields) => {
                let style = fields.style;
                let fields = fields.iter().map(|f| f.gen());
                match style {
                    ast::Style::Tuple => quote!(
                        pub struct #ident (
                            #(#fields),*
                        );
                    ),
                    ast::Style::Struct => quote!(
                        pub struct #ident {
                            #(#fields),*
                        }
                    ),
                    ast::Style::Unit => quote!(
                        pub struct #ident;
                    ),
                }
            }
        };

        quote!(
            #derive
            #data
            #to_tokens
        )
    }
}

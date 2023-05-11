use proc_macro::TokenStream;

extern crate proc_macro;

#[proc_macro_derive(TypeInfo, attributes(type_info))]
pub fn type_info(_item: TokenStream) -> TokenStream {
    TokenStream::default()
}


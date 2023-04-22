use darling::FromDeriveInput;
use proc_macro::TokenStream;
use type_info::TypeInfo;

extern crate proc_macro;

#[proc_macro_derive(TypeInfo, attributes(type_info))]
pub fn type_info(item: TokenStream) -> TokenStream {
    let type_info = TypeInfo::from_derive_input(&syn::parse_macro_input!(item));
    let type_info = match type_info {
        Err(e) => e.write_errors(),
        Ok(t) => t.gen(),
    };
    // println!("{}", type_info.to_string());
    TokenStream::from(type_info)
}

mod type_info;
#[cfg(test)]
mod tests {
    use erised_macros::TypeInfo;

    #[test]
    fn struct_info() {
        #[derive(TypeInfo)]
        pub struct TypeInfo {
            name: String,
        }

        let dyn_info = TypeInfo {
            name: "Foo".to_owned(),
        };

        let static_info = quote::quote!(
          #dyn_info
        );

        insta::assert_display_snapshot!(static_info);
    }

    #[test]
    fn tuple_struct_info() {
        #[derive(TypeInfo)]
        pub struct TypeInfo(String);

        let dyn_info = TypeInfo("Foo".to_owned());

        let static_info = quote::quote!(
          #dyn_info
        );

        insta::assert_display_snapshot!(static_info);
    }

    #[test]
    fn unit_struct_info() {
        #[derive(TypeInfo)]
        pub struct TypeInfo;

        let dyn_info = TypeInfo;

        let static_info = quote::quote!(
          #dyn_info
        );

        insta::assert_display_snapshot!(static_info);
    }

    #[test]
    fn option_info() {
        #[derive(TypeInfo)]
        pub struct TypeInfo {
            pub docs: Option<String>,
        }

        let dyn_info = TypeInfo {
            docs: Some("Foo".to_owned()),
        };

        let static_info = quote::quote!(
          #dyn_info
        );

        insta::assert_display_snapshot!(static_info);
    }

    #[test]
    #[allow(dead_code)]
    fn unit_enum_info() {
        #[derive(TypeInfo)]
        pub enum TypeInfo {
            Unit,
        }

        let dyn_info = TypeInfo::Unit;

        let static_info = quote::quote!(
          #dyn_info
        );

        insta::assert_display_snapshot!(static_info);
    }

    #[test]
    fn tuple_enum_info() {
        #[derive(TypeInfo)]
        pub enum TypeInfo {
            Tuple(String, String),
        }

        let dyn_info = TypeInfo::Tuple("foo".to_owned(), "bar".to_owned());

        let static_info = quote::quote!(
          #dyn_info
        );

        insta::assert_display_snapshot!(static_info);
    }

    #[test]
    fn struct_enum_info() {
        #[derive(TypeInfo)]
        pub enum TypeInfo {
            Map { foo: String },
        }

        let dyn_info = TypeInfo::Map {
            foo: "baz".to_owned(),
        };

        let static_info = quote::quote!(
          #dyn_info
        );

        insta::assert_display_snapshot!(static_info);
    }
}

mod experiments {

    use std::sync::Arc;

    #[allow(dead_code)]
    struct Foo {
        i: Vec<Arc<String>>,
        bar: Bar,
    }

    #[allow(dead_code)]
    struct StaticFoo {
        i: &'static [fn() -> &'static str],
        bar: StaticBar,
    }

    impl erised::destruct::ToTokens for Foo {
        fn to_tokens(&self) -> proc_macro2::TokenStream {
            let &Self { i, bar } = &self;

            let (i, bar) = (
                erised::destruct::ToTokens::to_tokens(i),
                erised::destruct::ToTokens::to_tokens(bar),
            );

            quote::quote!(StaticFoo { i: #i, bar: #bar })
        }
    }
    impl quote::ToTokens for Foo {
        fn to_tokens(&self, tokens: &mut proc_macro2::TokenStream) {
            use quote::TokenStreamExt;

            tokens.append_all(erised::destruct::ToTokens::to_tokens(self));
        }
    }

    struct Bar {
        a: String,
    }

    #[allow(dead_code)]
    struct StaticBar {
        a: &'static str,
    }

    impl erised::destruct::ToTokens for Bar {
        fn to_tokens(&self) -> proc_macro2::TokenStream {
            let &Self { a } = &self;

            let a = erised::destruct::ToTokens::to_tokens(a);

            // tokens.append_all(quote::quote!(StaticFoo { i: &[|| "Foo"] }));
            quote::quote!(StaticBar { a: #a })
        }
    }
    impl quote::ToTokens for Bar {
        fn to_tokens(&self, tokens: &mut proc_macro2::TokenStream) {
            use quote::TokenStreamExt;

            tokens.append_all(erised::destruct::ToTokens::to_tokens(self));
        }
    }
}

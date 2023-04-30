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

    #[test]
    fn struct_enum_destruct_field() {
        use erised::heap_types::StaticType;
        use erised::heap_types::Type;

        #[derive(TypeInfo)]
        pub enum TypeInfo {
            Map { foo: Box<Type> },
        }

        let dyn_info = TypeInfo::Map {
            foo: Box::new(Type::Infer),
        };

        let static_info = quote::quote!(
          #dyn_info
        );

        insta::assert_display_snapshot!(static_info);
    }
}

pub trait Reflect {
    const TYPE_INFO: erised::types::Item;
}
pub mod erised_tests {
    use super::*;
    impl Reflect for crate::MyStruct {
        const TYPE_INFO: erised::types::Item = erised::types::Item::Struct(erised::types::Struct {
            name: "MyStruct",
            meta: erised::types::ItemMeta {
                krate: || erised::types::ExternalCrate {
                    name: "erised_tests",
                    html_root_url: None,
                },
                summary: Some(erised::types::ItemSummary {
                    krate: || erised::types::ExternalCrate {
                        name: "erised_tests",
                        html_root_url: None,
                    },
                    path: &["erised_tests", "MyStruct"],
                    kind: erised::types::ItemKind::Struct,
                }),
                span: Some(erised::types::Span {
                    filename: "src/lib.rs",
                    begin: (57usize, 0usize),
                    end: (60usize, 1usize),
                }),
                visibility: erised::types::Visibility::Public,
                docs: None,
                attrs: &[],
                deprecation: None,
            },
            kind: erised::types::StructKind::Plain {
                fields: &[
                    erised::types::StructField {
                        name: Some("name"),
                        meta: erised::types::ItemMeta {
                            krate: || erised::types::ExternalCrate {
                                name: "erised_tests",
                                html_root_url: None,
                            },
                            summary: None,
                            span: Some(erised::types::Span {
                                filename: "src/lib.rs",
                                begin: (58usize, 4usize),
                                end: (58usize, 16usize),
                            }),
                            visibility: erised::types::Visibility::Crate,
                            docs: None,
                            attrs: &[],
                            deprecation: None,
                        },
                        ty: erised::types::Type::ResolvedPath(erised::types::Path {
                            name: "String",
                            prefix: "",
                            target: erised::types::Identifiable::Summary(|| {
                                erised::types::ItemSummary {
                                    krate: || erised::types::ExternalCrate {
                                        name: "alloc",
                                        html_root_url: Some("https://doc.rust-lang.org/nightly/"),
                                    },
                                    path: &["alloc", "string", "String"],
                                    kind: erised::types::ItemKind::Struct,
                                }
                            }),
                            args: Some(|| erised::types::GenericArgs::AngleBracketed {
                                args: &[],
                                bindings: &[],
                            }),
                        }),
                    },
                    erised::types::StructField {
                        name: Some("en"),
                        meta: erised::types::ItemMeta {
                            krate: || erised::types::ExternalCrate {
                                name: "erised_tests",
                                html_root_url: None,
                            },
                            summary: None,
                            span: Some(erised::types::Span {
                                filename: "src/lib.rs",
                                begin: (59usize, 4usize),
                                end: (59usize, 14usize),
                            }),
                            visibility: erised::types::Visibility::Crate,
                            docs: None,
                            attrs: &[],
                            deprecation: None,
                        },
                        ty: erised::types::Type::ResolvedPath(erised::types::Path {
                            name: "MyEnum",
                            prefix: "",
                            target: erised::types::Identifiable::Item(|| {
                                <crate::MyEnum as Reflect>::TYPE_INFO
                            }),
                            args: Some(|| erised::types::GenericArgs::AngleBracketed {
                                args: &[],
                                bindings: &[],
                            }),
                        }),
                    },
                ],
                fields_stripped: false,
            },
            generics: erised::types::Generics {
                params: &[],
                where_predicates: &[],
            },
            impls: &[
                erised::types::Identifiable::Item(|| todo!()),
                erised::types::Identifiable::Item(|| todo!()),
                erised::types::Identifiable::Item(|| todo!()),
                erised::types::Identifiable::Item(|| todo!()),
                erised::types::Identifiable::Item(|| todo!()),
                erised::types::Identifiable::Item(|| todo!()),
                erised::types::Identifiable::Item(|| todo!()),
                erised::types::Identifiable::Item(|| todo!()),
                erised::types::Identifiable::Item(|| todo!()),
                erised::types::Identifiable::Item(|| todo!()),
                erised::types::Identifiable::Item(|| todo!()),
                erised::types::Identifiable::Item(|| todo!()),
            ],
        });
    }
    impl Reflect for crate::MyEnum {
        const TYPE_INFO: erised::types::Item = erised::types::Item::Enum(erised::types::Enum {
            name: "MyEnum",
            meta: erised::types::ItemMeta {
                krate: || erised::types::ExternalCrate {
                    name: "erised_tests",
                    html_root_url: None,
                },
                summary: Some(erised::types::ItemSummary {
                    krate: || erised::types::ExternalCrate {
                        name: "erised_tests",
                        html_root_url: None,
                    },
                    path: &["erised_tests", "MyEnum"],
                    kind: erised::types::ItemKind::Enum,
                }),
                span: Some(erised::types::Span {
                    filename: "src/lib.rs",
                    begin: (62usize, 0usize),
                    end: (64usize, 1usize),
                }),
                visibility: erised::types::Visibility::Public,
                docs: None,
                attrs: &[],
                deprecation: None,
            },
            generics: erised::types::Generics {
                params: &[],
                where_predicates: &[],
            },
            variants_stripped: false,
            variants: &[erised::types::Variant {
                name: "Unit",
                meta: erised::types::ItemMeta {
                    krate: || erised::types::ExternalCrate {
                        name: "erised_tests",
                        html_root_url: None,
                    },
                    summary: Some(erised::types::ItemSummary {
                        krate: || erised::types::ExternalCrate {
                            name: "erised_tests",
                            html_root_url: None,
                        },
                        path: &["erised_tests", "MyEnum", "Unit"],
                        kind: erised::types::ItemKind::Variant,
                    }),
                    span: Some(erised::types::Span {
                        filename: "src/lib.rs",
                        begin: (63usize, 4usize),
                        end: (63usize, 8usize),
                    }),
                    visibility: erised::types::Visibility::Default,
                    docs: None,
                    attrs: &[],
                    deprecation: None,
                },
                kind: erised::types::VariantKind::Plain,
                discriminant: None,
            }],
            impls: &[
                erised::types::Identifiable::Item(|| todo!()),
                erised::types::Identifiable::Item(|| todo!()),
                erised::types::Identifiable::Item(|| todo!()),
                erised::types::Identifiable::Item(|| todo!()),
                erised::types::Identifiable::Item(|| todo!()),
                erised::types::Identifiable::Item(|| todo!()),
                erised::types::Identifiable::Item(|| todo!()),
                erised::types::Identifiable::Item(|| todo!()),
                erised::types::Identifiable::Item(|| todo!()),
                erised::types::Identifiable::Item(|| todo!()),
                erised::types::Identifiable::Item(|| todo!()),
                erised::types::Identifiable::Item(|| todo!()),
            ],
        });
    }
}

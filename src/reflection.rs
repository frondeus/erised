#![allow(dead_code)]
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
                    path: &["crate", "MyStruct"],
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
                                end: (58usize, 20usize),
                            }),
                            visibility: erised::types::Visibility::Public,
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
                                end: (59usize, 18usize),
                            }),
                            visibility: erised::types::Visibility::Public,
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
                erised::types::Identifiable::Item(|| {
                    erised::types::Item::Impl(erised::types::Impl {
                        meta: erised::types::ItemMeta {
                            krate: || erised::types::ExternalCrate {
                                name: "erised_tests",
                                html_root_url: None,
                            },
                            summary: None,
                            span: None,
                            visibility: erised::types::Visibility::Default,
                            docs: None,
                            attrs: &[],
                            deprecation: None,
                        },
                        is_unsafe: false,
                        generics: erised::types::Generics {
                            params: &[],
                            where_predicates: &[],
                        },
                        provided_trait_methods: &[],
                        trait_: Some(erised::types::Path {
                            name: "Send",
                            prefix: "",
                            target: erised::types::Identifiable::Summary(|| {
                                erised::types::ItemSummary {
                                    krate: || erised::types::ExternalCrate {
                                        name: "core",
                                        html_root_url: Some("https://doc.rust-lang.org/nightly/"),
                                    },
                                    path: &["core", "marker", "Send"],
                                    kind: erised::types::ItemKind::Trait,
                                }
                            }),
                            args: Some(|| erised::types::GenericArgs::AngleBracketed {
                                args: &[],
                                bindings: &[],
                            }),
                        }),
                        for_: erised::types::Type::ResolvedPath(erised::types::Path {
                            name: "MyStruct",
                            prefix: "",
                            target: erised::types::Identifiable::Item(|| {
                                <crate::MyStruct as Reflect>::TYPE_INFO
                            }),
                            args: Some(|| erised::types::GenericArgs::AngleBracketed {
                                args: &[],
                                bindings: &[],
                            }),
                        }),
                        items: &[],
                        negative: false,
                        synthetic: true,
                        blanket_impl: None,
                    })
                }),
                erised::types::Identifiable::Item(|| {
                    erised::types::Item::Impl(erised::types::Impl {
                        meta: erised::types::ItemMeta {
                            krate: || erised::types::ExternalCrate {
                                name: "erised_tests",
                                html_root_url: None,
                            },
                            summary: None,
                            span: None,
                            visibility: erised::types::Visibility::Default,
                            docs: None,
                            attrs: &[],
                            deprecation: None,
                        },
                        is_unsafe: false,
                        generics: erised::types::Generics {
                            params: &[],
                            where_predicates: &[],
                        },
                        provided_trait_methods: &[],
                        trait_: Some(erised::types::Path {
                            name: "Unpin",
                            prefix: "",
                            target: erised::types::Identifiable::Summary(|| {
                                erised::types::ItemSummary {
                                    krate: || erised::types::ExternalCrate {
                                        name: "core",
                                        html_root_url: Some("https://doc.rust-lang.org/nightly/"),
                                    },
                                    path: &["core", "marker", "Unpin"],
                                    kind: erised::types::ItemKind::Trait,
                                }
                            }),
                            args: Some(|| erised::types::GenericArgs::AngleBracketed {
                                args: &[],
                                bindings: &[],
                            }),
                        }),
                        for_: erised::types::Type::ResolvedPath(erised::types::Path {
                            name: "MyStruct",
                            prefix: "",
                            target: erised::types::Identifiable::Item(|| {
                                <crate::MyStruct as Reflect>::TYPE_INFO
                            }),
                            args: Some(|| erised::types::GenericArgs::AngleBracketed {
                                args: &[],
                                bindings: &[],
                            }),
                        }),
                        items: &[],
                        negative: false,
                        synthetic: true,
                        blanket_impl: None,
                    })
                }),
                erised::types::Identifiable::Item(|| {
                    erised::types::Item::Impl(erised::types::Impl {
                        meta: erised::types::ItemMeta {
                            krate: || erised::types::ExternalCrate {
                                name: "erised_tests",
                                html_root_url: None,
                            },
                            summary: None,
                            span: None,
                            visibility: erised::types::Visibility::Default,
                            docs: None,
                            attrs: &[],
                            deprecation: None,
                        },
                        is_unsafe: false,
                        generics: erised::types::Generics {
                            params: &[],
                            where_predicates: &[],
                        },
                        provided_trait_methods: &[],
                        trait_: Some(erised::types::Path {
                            name: "RefUnwindSafe",
                            prefix: "",
                            target: erised::types::Identifiable::Summary(|| {
                                erised::types::ItemSummary {
                                    krate: || erised::types::ExternalCrate {
                                        name: "core",
                                        html_root_url: Some("https://doc.rust-lang.org/nightly/"),
                                    },
                                    path: &["core", "panic", "unwind_safe", "RefUnwindSafe"],
                                    kind: erised::types::ItemKind::Trait,
                                }
                            }),
                            args: Some(|| erised::types::GenericArgs::AngleBracketed {
                                args: &[],
                                bindings: &[],
                            }),
                        }),
                        for_: erised::types::Type::ResolvedPath(erised::types::Path {
                            name: "MyStruct",
                            prefix: "",
                            target: erised::types::Identifiable::Item(|| {
                                <crate::MyStruct as Reflect>::TYPE_INFO
                            }),
                            args: Some(|| erised::types::GenericArgs::AngleBracketed {
                                args: &[],
                                bindings: &[],
                            }),
                        }),
                        items: &[],
                        negative: false,
                        synthetic: true,
                        blanket_impl: None,
                    })
                }),
                erised::types::Identifiable::Item(|| {
                    erised::types::Item::Impl(erised::types::Impl {
                        meta: erised::types::ItemMeta {
                            krate: || erised::types::ExternalCrate {
                                name: "erised_tests",
                                html_root_url: None,
                            },
                            summary: None,
                            span: None,
                            visibility: erised::types::Visibility::Default,
                            docs: None,
                            attrs: &[],
                            deprecation: None,
                        },
                        is_unsafe: false,
                        generics: erised::types::Generics {
                            params: &[],
                            where_predicates: &[],
                        },
                        provided_trait_methods: &[],
                        trait_: Some(erised::types::Path {
                            name: "Sync",
                            prefix: "",
                            target: erised::types::Identifiable::Summary(|| {
                                erised::types::ItemSummary {
                                    krate: || erised::types::ExternalCrate {
                                        name: "core",
                                        html_root_url: Some("https://doc.rust-lang.org/nightly/"),
                                    },
                                    path: &["core", "marker", "Sync"],
                                    kind: erised::types::ItemKind::Trait,
                                }
                            }),
                            args: Some(|| erised::types::GenericArgs::AngleBracketed {
                                args: &[],
                                bindings: &[],
                            }),
                        }),
                        for_: erised::types::Type::ResolvedPath(erised::types::Path {
                            name: "MyStruct",
                            prefix: "",
                            target: erised::types::Identifiable::Item(|| {
                                <crate::MyStruct as Reflect>::TYPE_INFO
                            }),
                            args: Some(|| erised::types::GenericArgs::AngleBracketed {
                                args: &[],
                                bindings: &[],
                            }),
                        }),
                        items: &[],
                        negative: false,
                        synthetic: true,
                        blanket_impl: None,
                    })
                }),
                erised::types::Identifiable::Item(|| {
                    erised::types::Item::Impl(erised::types::Impl {
                        meta: erised::types::ItemMeta {
                            krate: || erised::types::ExternalCrate {
                                name: "erised_tests",
                                html_root_url: None,
                            },
                            summary: None,
                            span: None,
                            visibility: erised::types::Visibility::Default,
                            docs: None,
                            attrs: &[],
                            deprecation: None,
                        },
                        is_unsafe: false,
                        generics: erised::types::Generics {
                            params: &[],
                            where_predicates: &[],
                        },
                        provided_trait_methods: &[],
                        trait_: Some(erised::types::Path {
                            name: "UnwindSafe",
                            prefix: "",
                            target: erised::types::Identifiable::Summary(|| {
                                erised::types::ItemSummary {
                                    krate: || erised::types::ExternalCrate {
                                        name: "core",
                                        html_root_url: Some("https://doc.rust-lang.org/nightly/"),
                                    },
                                    path: &["core", "panic", "unwind_safe", "UnwindSafe"],
                                    kind: erised::types::ItemKind::Trait,
                                }
                            }),
                            args: Some(|| erised::types::GenericArgs::AngleBracketed {
                                args: &[],
                                bindings: &[],
                            }),
                        }),
                        for_: erised::types::Type::ResolvedPath(erised::types::Path {
                            name: "MyStruct",
                            prefix: "",
                            target: erised::types::Identifiable::Item(|| {
                                <crate::MyStruct as Reflect>::TYPE_INFO
                            }),
                            args: Some(|| erised::types::GenericArgs::AngleBracketed {
                                args: &[],
                                bindings: &[],
                            }),
                        }),
                        items: &[],
                        negative: false,
                        synthetic: true,
                        blanket_impl: None,
                    })
                }),
                erised::types::Identifiable::Item(|| {
                    erised::types::Item::Impl(erised::types::Impl {
                        meta: erised::types::ItemMeta {
                            krate: || erised::types::ExternalCrate {
                                name: "erised_tests",
                                html_root_url: None,
                            },
                            summary: None,
                            span: None,
                            visibility: erised::types::Visibility::Default,
                            docs: None,
                            attrs: &[],
                            deprecation: None,
                        },
                        is_unsafe: false,
                        generics: erised::types::Generics {
                            params: &[
                                erised::types::GenericParamDef {
                                    name: "T",
                                    kind: erised::types::GenericParamDefKind::Type {
                                        bounds: &[],
                                        default: None,
                                        synthetic: false,
                                    },
                                },
                                erised::types::GenericParamDef {
                                    name: "U",
                                    kind: erised::types::GenericParamDefKind::Type {
                                        bounds: &[],
                                        default: None,
                                        synthetic: false,
                                    },
                                },
                            ],
                            where_predicates: &[erised::types::WherePredicate::BoundPredicate {
                                type_: erised::types::Type::Generic("U"),
                                bounds: &[erised::types::GenericBound::TraitBound {
                                    trait_: erised::types::Path {
                                        name: "From",
                                        prefix: "",
                                        target: erised::types::Identifiable::Summary(|| {
                                            erised::types::ItemSummary {
                                                krate: || erised::types::ExternalCrate {
                                                    name: "core",
                                                    html_root_url: Some(
                                                        "https://doc.rust-lang.org/nightly/",
                                                    ),
                                                },
                                                path: &["core", "convert", "From"],
                                                kind: erised::types::ItemKind::Trait,
                                            }
                                        }),
                                        args: Some(|| erised::types::GenericArgs::AngleBracketed {
                                            args: &[erised::types::GenericArg::Type(
                                                erised::types::Type::Generic("T"),
                                            )],
                                            bindings: &[],
                                        }),
                                    },
                                    generic_params: &[],
                                    modifier: erised::types::TraitBoundModifier::None,
                                }],
                                generic_params: &[],
                            }],
                        },
                        provided_trait_methods: &[],
                        trait_: Some(erised::types::Path {
                            name: "Into",
                            prefix: "",
                            target: erised::types::Identifiable::Summary(|| {
                                erised::types::ItemSummary {
                                    krate: || erised::types::ExternalCrate {
                                        name: "core",
                                        html_root_url: Some("https://doc.rust-lang.org/nightly/"),
                                    },
                                    path: &["core", "convert", "Into"],
                                    kind: erised::types::ItemKind::Trait,
                                }
                            }),
                            args: Some(|| erised::types::GenericArgs::AngleBracketed {
                                args: &[erised::types::GenericArg::Type(
                                    erised::types::Type::Generic("U"),
                                )],
                                bindings: &[],
                            }),
                        }),
                        for_: erised::types::Type::ResolvedPath(erised::types::Path {
                            name: "MyStruct",
                            prefix: "",
                            target: erised::types::Identifiable::Item(|| {
                                <crate::MyStruct as Reflect>::TYPE_INFO
                            }),
                            args: Some(|| erised::types::GenericArgs::AngleBracketed {
                                args: &[],
                                bindings: &[],
                            }),
                        }),
                        items: &[|| {
                            erised :: types :: Item :: Function (erised :: types :: Function { name : "into" , meta : erised :: types :: ItemMeta { krate : || erised :: types :: ExternalCrate { name : "core" , html_root_url : Some ("https://doc.rust-lang.org/nightly/") } , summary : None , span : None , visibility : erised :: types :: Visibility :: Default , docs : Some ("Calls `U::from(self)`.\n\nThat is, this conversion is whatever the implementation of\n<code>[From]&lt;T&gt; for U</code> chooses to do.") , attrs : & [] , deprecation : None } , decl : erised :: types :: FnDecl { inputs : & [erised :: types :: FnInput { pat : "self" , ty : erised :: types :: Type :: Generic ("Self") }] , output : Some (erised :: types :: Type :: Generic ("U")) , c_variadic : false } , generics : erised :: types :: Generics { params : & [] , where_predicates : & [] } , header : erised :: types :: Header { const_ : false , unsafe_ : false , async_ : false , abi : erised :: types :: Abi :: Rust } , has_body : true })
                        }],
                        negative: false,
                        synthetic: false,
                        blanket_impl: Some(erised::types::Type::Generic("T")),
                    })
                }),
                erised::types::Identifiable::Item(|| {
                    erised::types::Item::Impl(erised::types::Impl {
                        meta: erised::types::ItemMeta {
                            krate: || erised::types::ExternalCrate {
                                name: "erised_tests",
                                html_root_url: None,
                            },
                            summary: None,
                            span: None,
                            visibility: erised::types::Visibility::Default,
                            docs: None,
                            attrs: &[],
                            deprecation: None,
                        },
                        is_unsafe: false,
                        generics: erised::types::Generics {
                            params: &[erised::types::GenericParamDef {
                                name: "T",
                                kind: erised::types::GenericParamDefKind::Type {
                                    bounds: &[],
                                    default: None,
                                    synthetic: false,
                                },
                            }],
                            where_predicates: &[erised::types::WherePredicate::BoundPredicate {
                                type_: erised::types::Type::Generic("T"),
                                bounds: &[
                                    erised::types::GenericBound::Outlives("'static"),
                                    erised::types::GenericBound::TraitBound {
                                        trait_: erised::types::Path {
                                            name: "Sized",
                                            prefix: "",
                                            target: erised::types::Identifiable::Summary(|| {
                                                erised::types::ItemSummary {
                                                    krate: || erised::types::ExternalCrate {
                                                        name: "core",
                                                        html_root_url: Some(
                                                            "https://doc.rust-lang.org/nightly/",
                                                        ),
                                                    },
                                                    path: &["core", "marker", "Sized"],
                                                    kind: erised::types::ItemKind::Trait,
                                                }
                                            }),
                                            args: Some(|| {
                                                erised::types::GenericArgs::AngleBracketed {
                                                    args: &[],
                                                    bindings: &[],
                                                }
                                            }),
                                        },
                                        generic_params: &[],
                                        modifier: erised::types::TraitBoundModifier::Maybe,
                                    },
                                ],
                                generic_params: &[],
                            }],
                        },
                        provided_trait_methods: &[],
                        trait_: Some(erised::types::Path {
                            name: "Any",
                            prefix: "",
                            target: erised::types::Identifiable::Summary(|| {
                                erised::types::ItemSummary {
                                    krate: || erised::types::ExternalCrate {
                                        name: "core",
                                        html_root_url: Some("https://doc.rust-lang.org/nightly/"),
                                    },
                                    path: &["core", "any", "Any"],
                                    kind: erised::types::ItemKind::Trait,
                                }
                            }),
                            args: Some(|| erised::types::GenericArgs::AngleBracketed {
                                args: &[],
                                bindings: &[],
                            }),
                        }),
                        for_: erised::types::Type::ResolvedPath(erised::types::Path {
                            name: "MyStruct",
                            prefix: "",
                            target: erised::types::Identifiable::Item(|| {
                                <crate::MyStruct as Reflect>::TYPE_INFO
                            }),
                            args: Some(|| erised::types::GenericArgs::AngleBracketed {
                                args: &[],
                                bindings: &[],
                            }),
                        }),
                        items: &[|| {
                            erised::types::Item::Function(erised::types::Function {
                                name: "type_id",
                                meta: erised::types::ItemMeta {
                                    krate: || erised::types::ExternalCrate {
                                        name: "core",
                                        html_root_url: Some("https://doc.rust-lang.org/nightly/"),
                                    },
                                    summary: None,
                                    span: None,
                                    visibility: erised::types::Visibility::Default,
                                    docs: None,
                                    attrs: &[],
                                    deprecation: None,
                                },
                                decl: erised::types::FnDecl {
                                    inputs: &[erised::types::FnInput {
                                        pat: "self",
                                        ty: erised::types::Type::BorrowedRef {
                                            lifetime: None,
                                            mutable: false,
                                            type_: || erised::types::Type::Generic("Self"),
                                        },
                                    }],
                                    output: Some(erised::types::Type::ResolvedPath(
                                        erised::types::Path {
                                            name: "TypeId",
                                            prefix: "",
                                            target: erised::types::Identifiable::Summary(|| {
                                                erised::types::ItemSummary {
                                                    krate: || erised::types::ExternalCrate {
                                                        name: "core",
                                                        html_root_url: Some(
                                                            "https://doc.rust-lang.org/nightly/",
                                                        ),
                                                    },
                                                    path: &["core", "any", "TypeId"],
                                                    kind: erised::types::ItemKind::Struct,
                                                }
                                            }),
                                            args: Some(|| {
                                                erised::types::GenericArgs::AngleBracketed {
                                                    args: &[],
                                                    bindings: &[],
                                                }
                                            }),
                                        },
                                    )),
                                    c_variadic: false,
                                },
                                generics: erised::types::Generics {
                                    params: &[],
                                    where_predicates: &[],
                                },
                                header: erised::types::Header {
                                    const_: false,
                                    unsafe_: false,
                                    async_: false,
                                    abi: erised::types::Abi::Rust,
                                },
                                has_body: true,
                            })
                        }],
                        negative: false,
                        synthetic: false,
                        blanket_impl: Some(erised::types::Type::Generic("T")),
                    })
                }),
                erised::types::Identifiable::Item(|| {
                    erised::types::Item::Impl(erised::types::Impl {
                        meta: erised::types::ItemMeta {
                            krate: || erised::types::ExternalCrate {
                                name: "erised_tests",
                                html_root_url: None,
                            },
                            summary: None,
                            span: None,
                            visibility: erised::types::Visibility::Default,
                            docs: None,
                            attrs: &[],
                            deprecation: None,
                        },
                        is_unsafe: false,
                        generics: erised::types::Generics {
                            params: &[erised::types::GenericParamDef {
                                name: "T",
                                kind: erised::types::GenericParamDefKind::Type {
                                    bounds: &[],
                                    default: None,
                                    synthetic: false,
                                },
                            }],
                            where_predicates: &[erised::types::WherePredicate::BoundPredicate {
                                type_: erised::types::Type::Generic("T"),
                                bounds: &[erised::types::GenericBound::TraitBound {
                                    trait_: erised::types::Path {
                                        name: "Sized",
                                        prefix: "",
                                        target: erised::types::Identifiable::Summary(|| {
                                            erised::types::ItemSummary {
                                                krate: || erised::types::ExternalCrate {
                                                    name: "core",
                                                    html_root_url: Some(
                                                        "https://doc.rust-lang.org/nightly/",
                                                    ),
                                                },
                                                path: &["core", "marker", "Sized"],
                                                kind: erised::types::ItemKind::Trait,
                                            }
                                        }),
                                        args: Some(|| erised::types::GenericArgs::AngleBracketed {
                                            args: &[],
                                            bindings: &[],
                                        }),
                                    },
                                    generic_params: &[],
                                    modifier: erised::types::TraitBoundModifier::Maybe,
                                }],
                                generic_params: &[],
                            }],
                        },
                        provided_trait_methods: &[],
                        trait_: Some(erised::types::Path {
                            name: "BorrowMut",
                            prefix: "",
                            target: erised::types::Identifiable::Summary(|| {
                                erised::types::ItemSummary {
                                    krate: || erised::types::ExternalCrate {
                                        name: "core",
                                        html_root_url: Some("https://doc.rust-lang.org/nightly/"),
                                    },
                                    path: &["core", "borrow", "BorrowMut"],
                                    kind: erised::types::ItemKind::Trait,
                                }
                            }),
                            args: Some(|| erised::types::GenericArgs::AngleBracketed {
                                args: &[erised::types::GenericArg::Type(
                                    erised::types::Type::Generic("T"),
                                )],
                                bindings: &[],
                            }),
                        }),
                        for_: erised::types::Type::ResolvedPath(erised::types::Path {
                            name: "MyStruct",
                            prefix: "",
                            target: erised::types::Identifiable::Item(|| {
                                <crate::MyStruct as Reflect>::TYPE_INFO
                            }),
                            args: Some(|| erised::types::GenericArgs::AngleBracketed {
                                args: &[],
                                bindings: &[],
                            }),
                        }),
                        items: &[|| {
                            erised::types::Item::Function(erised::types::Function {
                                name: "borrow_mut",
                                meta: erised::types::ItemMeta {
                                    krate: || erised::types::ExternalCrate {
                                        name: "core",
                                        html_root_url: Some("https://doc.rust-lang.org/nightly/"),
                                    },
                                    summary: None,
                                    span: None,
                                    visibility: erised::types::Visibility::Default,
                                    docs: None,
                                    attrs: &[],
                                    deprecation: None,
                                },
                                decl: erised::types::FnDecl {
                                    inputs: &[erised::types::FnInput {
                                        pat: "self",
                                        ty: erised::types::Type::BorrowedRef {
                                            lifetime: None,
                                            mutable: true,
                                            type_: || erised::types::Type::Generic("Self"),
                                        },
                                    }],
                                    output: Some(erised::types::Type::BorrowedRef {
                                        lifetime: None,
                                        mutable: true,
                                        type_: || erised::types::Type::Generic("T"),
                                    }),
                                    c_variadic: false,
                                },
                                generics: erised::types::Generics {
                                    params: &[],
                                    where_predicates: &[],
                                },
                                header: erised::types::Header {
                                    const_: false,
                                    unsafe_: false,
                                    async_: false,
                                    abi: erised::types::Abi::Rust,
                                },
                                has_body: true,
                            })
                        }],
                        negative: false,
                        synthetic: false,
                        blanket_impl: Some(erised::types::Type::Generic("T")),
                    })
                }),
                erised::types::Identifiable::Item(|| {
                    erised::types::Item::Impl(erised::types::Impl {
                        meta: erised::types::ItemMeta {
                            krate: || erised::types::ExternalCrate {
                                name: "erised_tests",
                                html_root_url: None,
                            },
                            summary: None,
                            span: None,
                            visibility: erised::types::Visibility::Default,
                            docs: None,
                            attrs: &[],
                            deprecation: None,
                        },
                        is_unsafe: false,
                        generics: erised::types::Generics {
                            params: &[
                                erised::types::GenericParamDef {
                                    name: "T",
                                    kind: erised::types::GenericParamDefKind::Type {
                                        bounds: &[],
                                        default: None,
                                        synthetic: false,
                                    },
                                },
                                erised::types::GenericParamDef {
                                    name: "U",
                                    kind: erised::types::GenericParamDefKind::Type {
                                        bounds: &[],
                                        default: None,
                                        synthetic: false,
                                    },
                                },
                            ],
                            where_predicates: &[erised::types::WherePredicate::BoundPredicate {
                                type_: erised::types::Type::Generic("U"),
                                bounds: &[erised::types::GenericBound::TraitBound {
                                    trait_: erised::types::Path {
                                        name: "Into",
                                        prefix: "",
                                        target: erised::types::Identifiable::Summary(|| {
                                            erised::types::ItemSummary {
                                                krate: || erised::types::ExternalCrate {
                                                    name: "core",
                                                    html_root_url: Some(
                                                        "https://doc.rust-lang.org/nightly/",
                                                    ),
                                                },
                                                path: &["core", "convert", "Into"],
                                                kind: erised::types::ItemKind::Trait,
                                            }
                                        }),
                                        args: Some(|| erised::types::GenericArgs::AngleBracketed {
                                            args: &[erised::types::GenericArg::Type(
                                                erised::types::Type::Generic("T"),
                                            )],
                                            bindings: &[],
                                        }),
                                    },
                                    generic_params: &[],
                                    modifier: erised::types::TraitBoundModifier::None,
                                }],
                                generic_params: &[],
                            }],
                        },
                        provided_trait_methods: &[],
                        trait_: Some(erised::types::Path {
                            name: "TryFrom",
                            prefix: "",
                            target: erised::types::Identifiable::Summary(|| {
                                erised::types::ItemSummary {
                                    krate: || erised::types::ExternalCrate {
                                        name: "core",
                                        html_root_url: Some("https://doc.rust-lang.org/nightly/"),
                                    },
                                    path: &["core", "convert", "TryFrom"],
                                    kind: erised::types::ItemKind::Trait,
                                }
                            }),
                            args: Some(|| erised::types::GenericArgs::AngleBracketed {
                                args: &[erised::types::GenericArg::Type(
                                    erised::types::Type::Generic("U"),
                                )],
                                bindings: &[],
                            }),
                        }),
                        for_: erised::types::Type::ResolvedPath(erised::types::Path {
                            name: "MyStruct",
                            prefix: "",
                            target: erised::types::Identifiable::Item(|| {
                                <crate::MyStruct as Reflect>::TYPE_INFO
                            }),
                            args: Some(|| erised::types::GenericArgs::AngleBracketed {
                                args: &[],
                                bindings: &[],
                            }),
                        }),
                        items: &[
                            || erised::types::Item::AssocType {
                                meta: erised::types::ItemMeta {
                                    krate: || erised::types::ExternalCrate {
                                        name: "core",
                                        html_root_url: Some("https://doc.rust-lang.org/nightly/"),
                                    },
                                    summary: None,
                                    span: None,
                                    visibility: erised::types::Visibility::Default,
                                    docs: None,
                                    attrs: &[],
                                    deprecation: None,
                                },
                                generics: erised::types::Generics {
                                    params: &[],
                                    where_predicates: &[],
                                },
                                bounds: &[],
                                default: Some(erised::types::Type::ResolvedPath(
                                    erised::types::Path {
                                        name: "Infallible",
                                        prefix: "",
                                        target: erised::types::Identifiable::Summary(|| {
                                            erised::types::ItemSummary {
                                                krate: || erised::types::ExternalCrate {
                                                    name: "core",
                                                    html_root_url: Some(
                                                        "https://doc.rust-lang.org/nightly/",
                                                    ),
                                                },
                                                path: &["core", "convert", "Infallible"],
                                                kind: erised::types::ItemKind::Enum,
                                            }
                                        }),
                                        args: Some(|| erised::types::GenericArgs::AngleBracketed {
                                            args: &[],
                                            bindings: &[],
                                        }),
                                    },
                                )),
                            },
                            || {
                                erised::types::Item::Function(erised::types::Function {
                                    name: "try_from",
                                    meta: erised::types::ItemMeta {
                                        krate: || erised::types::ExternalCrate {
                                            name: "core",
                                            html_root_url: Some(
                                                "https://doc.rust-lang.org/nightly/",
                                            ),
                                        },
                                        summary: None,
                                        span: None,
                                        visibility: erised::types::Visibility::Default,
                                        docs: None,
                                        attrs: &[],
                                        deprecation: None,
                                    },
                                    decl: erised::types::FnDecl {
                                        inputs: &[erised::types::FnInput {
                                            pat: "value",
                                            ty: erised::types::Type::Generic("U"),
                                        }],
                                        output: Some(erised::types::Type::ResolvedPath(
                                            erised::types::Path {
                                                name: "Result",
                                                prefix: "",
                                                target: erised::types::Identifiable::Summary(
                                                    || erised::types::ItemSummary {
                                                        krate: || {
                                                            erised :: types :: ExternalCrate { name : "core" , html_root_url : Some ("https://doc.rust-lang.org/nightly/") }
                                                        },
                                                        path: &["core", "result", "Result"],
                                                        kind: erised::types::ItemKind::Enum,
                                                    },
                                                ),
                                                args: Some(|| {
                                                    erised :: types :: GenericArgs :: AngleBracketed { args : & [erised :: types :: GenericArg :: Type (erised :: types :: Type :: Generic ("T")) , erised :: types :: GenericArg :: Type (erised :: types :: Type :: QualifiedPath { name : "Error" , args : || erised :: types :: GenericArgs :: AngleBracketed { args : & [] , bindings : & [] } , self_type : || erised :: types :: Type :: Generic ("T") , trait_ : erised :: types :: Path { name : "TryFrom" , prefix : "" , target : erised :: types :: Identifiable :: Summary (|| erised :: types :: ItemSummary { krate : || erised :: types :: ExternalCrate { name : "core" , html_root_url : Some ("https://doc.rust-lang.org/nightly/") } , path : & ["core" , "convert" , "TryFrom"] , kind : erised :: types :: ItemKind :: Trait }) , args : Some (|| erised :: types :: GenericArgs :: AngleBracketed { args : & [erised :: types :: GenericArg :: Type (erised :: types :: Type :: Generic ("U"))] , bindings : & [] }) } })] , bindings : & [] }
                                                }),
                                            },
                                        )),
                                        c_variadic: false,
                                    },
                                    generics: erised::types::Generics {
                                        params: &[],
                                        where_predicates: &[],
                                    },
                                    header: erised::types::Header {
                                        const_: false,
                                        unsafe_: false,
                                        async_: false,
                                        abi: erised::types::Abi::Rust,
                                    },
                                    has_body: true,
                                })
                            },
                        ],
                        negative: false,
                        synthetic: false,
                        blanket_impl: Some(erised::types::Type::Generic("T")),
                    })
                }),
                erised::types::Identifiable::Item(|| {
                    erised::types::Item::Impl(erised::types::Impl {
                        meta: erised::types::ItemMeta {
                            krate: || erised::types::ExternalCrate {
                                name: "erised_tests",
                                html_root_url: None,
                            },
                            summary: None,
                            span: None,
                            visibility: erised::types::Visibility::Default,
                            docs: None,
                            attrs: &[],
                            deprecation: None,
                        },
                        is_unsafe: false,
                        generics: erised::types::Generics {
                            params: &[erised::types::GenericParamDef {
                                name: "T",
                                kind: erised::types::GenericParamDefKind::Type {
                                    bounds: &[],
                                    default: None,
                                    synthetic: false,
                                },
                            }],
                            where_predicates: &[],
                        },
                        provided_trait_methods: &[],
                        trait_: Some(erised::types::Path {
                            name: "From",
                            prefix: "",
                            target: erised::types::Identifiable::Summary(|| {
                                erised::types::ItemSummary {
                                    krate: || erised::types::ExternalCrate {
                                        name: "core",
                                        html_root_url: Some("https://doc.rust-lang.org/nightly/"),
                                    },
                                    path: &["core", "convert", "From"],
                                    kind: erised::types::ItemKind::Trait,
                                }
                            }),
                            args: Some(|| erised::types::GenericArgs::AngleBracketed {
                                args: &[erised::types::GenericArg::Type(
                                    erised::types::Type::Generic("T"),
                                )],
                                bindings: &[],
                            }),
                        }),
                        for_: erised::types::Type::ResolvedPath(erised::types::Path {
                            name: "MyStruct",
                            prefix: "",
                            target: erised::types::Identifiable::Item(|| {
                                <crate::MyStruct as Reflect>::TYPE_INFO
                            }),
                            args: Some(|| erised::types::GenericArgs::AngleBracketed {
                                args: &[],
                                bindings: &[],
                            }),
                        }),
                        items: &[|| {
                            erised::types::Item::Function(erised::types::Function {
                                name: "from",
                                meta: erised::types::ItemMeta {
                                    krate: || erised::types::ExternalCrate {
                                        name: "core",
                                        html_root_url: Some("https://doc.rust-lang.org/nightly/"),
                                    },
                                    summary: None,
                                    span: None,
                                    visibility: erised::types::Visibility::Default,
                                    docs: Some("Returns the argument unchanged."),
                                    attrs: &[],
                                    deprecation: None,
                                },
                                decl: erised::types::FnDecl {
                                    inputs: &[erised::types::FnInput {
                                        pat: "t",
                                        ty: erised::types::Type::Generic("T"),
                                    }],
                                    output: Some(erised::types::Type::Generic("T")),
                                    c_variadic: false,
                                },
                                generics: erised::types::Generics {
                                    params: &[],
                                    where_predicates: &[],
                                },
                                header: erised::types::Header {
                                    const_: false,
                                    unsafe_: false,
                                    async_: false,
                                    abi: erised::types::Abi::Rust,
                                },
                                has_body: true,
                            })
                        }],
                        negative: false,
                        synthetic: false,
                        blanket_impl: Some(erised::types::Type::Generic("T")),
                    })
                }),
                erised::types::Identifiable::Item(|| {
                    erised::types::Item::Impl(erised::types::Impl {
                        meta: erised::types::ItemMeta {
                            krate: || erised::types::ExternalCrate {
                                name: "erised_tests",
                                html_root_url: None,
                            },
                            summary: None,
                            span: None,
                            visibility: erised::types::Visibility::Default,
                            docs: None,
                            attrs: &[],
                            deprecation: None,
                        },
                        is_unsafe: false,
                        generics: erised::types::Generics {
                            params: &[
                                erised::types::GenericParamDef {
                                    name: "T",
                                    kind: erised::types::GenericParamDefKind::Type {
                                        bounds: &[],
                                        default: None,
                                        synthetic: false,
                                    },
                                },
                                erised::types::GenericParamDef {
                                    name: "U",
                                    kind: erised::types::GenericParamDefKind::Type {
                                        bounds: &[],
                                        default: None,
                                        synthetic: false,
                                    },
                                },
                            ],
                            where_predicates: &[erised::types::WherePredicate::BoundPredicate {
                                type_: erised::types::Type::Generic("U"),
                                bounds: &[erised::types::GenericBound::TraitBound {
                                    trait_: erised::types::Path {
                                        name: "TryFrom",
                                        prefix: "",
                                        target: erised::types::Identifiable::Summary(|| {
                                            erised::types::ItemSummary {
                                                krate: || erised::types::ExternalCrate {
                                                    name: "core",
                                                    html_root_url: Some(
                                                        "https://doc.rust-lang.org/nightly/",
                                                    ),
                                                },
                                                path: &["core", "convert", "TryFrom"],
                                                kind: erised::types::ItemKind::Trait,
                                            }
                                        }),
                                        args: Some(|| erised::types::GenericArgs::AngleBracketed {
                                            args: &[erised::types::GenericArg::Type(
                                                erised::types::Type::Generic("T"),
                                            )],
                                            bindings: &[],
                                        }),
                                    },
                                    generic_params: &[],
                                    modifier: erised::types::TraitBoundModifier::None,
                                }],
                                generic_params: &[],
                            }],
                        },
                        provided_trait_methods: &[],
                        trait_: Some(erised::types::Path {
                            name: "TryInto",
                            prefix: "",
                            target: erised::types::Identifiable::Summary(|| {
                                erised::types::ItemSummary {
                                    krate: || erised::types::ExternalCrate {
                                        name: "core",
                                        html_root_url: Some("https://doc.rust-lang.org/nightly/"),
                                    },
                                    path: &["core", "convert", "TryInto"],
                                    kind: erised::types::ItemKind::Trait,
                                }
                            }),
                            args: Some(|| erised::types::GenericArgs::AngleBracketed {
                                args: &[erised::types::GenericArg::Type(
                                    erised::types::Type::Generic("U"),
                                )],
                                bindings: &[],
                            }),
                        }),
                        for_: erised::types::Type::ResolvedPath(erised::types::Path {
                            name: "MyStruct",
                            prefix: "",
                            target: erised::types::Identifiable::Item(|| {
                                <crate::MyStruct as Reflect>::TYPE_INFO
                            }),
                            args: Some(|| erised::types::GenericArgs::AngleBracketed {
                                args: &[],
                                bindings: &[],
                            }),
                        }),
                        items: &[
                            || erised::types::Item::AssocType {
                                meta: erised::types::ItemMeta {
                                    krate: || erised::types::ExternalCrate {
                                        name: "core",
                                        html_root_url: Some("https://doc.rust-lang.org/nightly/"),
                                    },
                                    summary: None,
                                    span: None,
                                    visibility: erised::types::Visibility::Default,
                                    docs: None,
                                    attrs: &[],
                                    deprecation: None,
                                },
                                generics: erised::types::Generics {
                                    params: &[],
                                    where_predicates: &[],
                                },
                                bounds: &[],
                                default: Some(erised::types::Type::QualifiedPath {
                                    name: "Error",
                                    args: || erised::types::GenericArgs::AngleBracketed {
                                        args: &[],
                                        bindings: &[],
                                    },
                                    self_type: || erised::types::Type::Generic("U"),
                                    trait_: erised::types::Path {
                                        name: "TryFrom",
                                        prefix: "",
                                        target: erised::types::Identifiable::Summary(|| {
                                            erised::types::ItemSummary {
                                                krate: || erised::types::ExternalCrate {
                                                    name: "core",
                                                    html_root_url: Some(
                                                        "https://doc.rust-lang.org/nightly/",
                                                    ),
                                                },
                                                path: &["core", "convert", "TryFrom"],
                                                kind: erised::types::ItemKind::Trait,
                                            }
                                        }),
                                        args: Some(|| erised::types::GenericArgs::AngleBracketed {
                                            args: &[erised::types::GenericArg::Type(
                                                erised::types::Type::Generic("T"),
                                            )],
                                            bindings: &[],
                                        }),
                                    },
                                }),
                            },
                            || {
                                erised::types::Item::Function(erised::types::Function {
                                    name: "try_into",
                                    meta: erised::types::ItemMeta {
                                        krate: || erised::types::ExternalCrate {
                                            name: "core",
                                            html_root_url: Some(
                                                "https://doc.rust-lang.org/nightly/",
                                            ),
                                        },
                                        summary: None,
                                        span: None,
                                        visibility: erised::types::Visibility::Default,
                                        docs: None,
                                        attrs: &[],
                                        deprecation: None,
                                    },
                                    decl: erised::types::FnDecl {
                                        inputs: &[erised::types::FnInput {
                                            pat: "self",
                                            ty: erised::types::Type::Generic("Self"),
                                        }],
                                        output: Some(erised::types::Type::ResolvedPath(
                                            erised::types::Path {
                                                name: "Result",
                                                prefix: "",
                                                target: erised::types::Identifiable::Summary(
                                                    || erised::types::ItemSummary {
                                                        krate: || {
                                                            erised :: types :: ExternalCrate { name : "core" , html_root_url : Some ("https://doc.rust-lang.org/nightly/") }
                                                        },
                                                        path: &["core", "result", "Result"],
                                                        kind: erised::types::ItemKind::Enum,
                                                    },
                                                ),
                                                args: Some(|| {
                                                    erised :: types :: GenericArgs :: AngleBracketed { args : & [erised :: types :: GenericArg :: Type (erised :: types :: Type :: Generic ("U")) , erised :: types :: GenericArg :: Type (erised :: types :: Type :: QualifiedPath { name : "Error" , args : || erised :: types :: GenericArgs :: AngleBracketed { args : & [] , bindings : & [] } , self_type : || erised :: types :: Type :: Generic ("U") , trait_ : erised :: types :: Path { name : "TryFrom" , prefix : "" , target : erised :: types :: Identifiable :: Summary (|| erised :: types :: ItemSummary { krate : || erised :: types :: ExternalCrate { name : "core" , html_root_url : Some ("https://doc.rust-lang.org/nightly/") } , path : & ["core" , "convert" , "TryFrom"] , kind : erised :: types :: ItemKind :: Trait }) , args : Some (|| erised :: types :: GenericArgs :: AngleBracketed { args : & [erised :: types :: GenericArg :: Type (erised :: types :: Type :: Generic ("T"))] , bindings : & [] }) } })] , bindings : & [] }
                                                }),
                                            },
                                        )),
                                        c_variadic: false,
                                    },
                                    generics: erised::types::Generics {
                                        params: &[],
                                        where_predicates: &[],
                                    },
                                    header: erised::types::Header {
                                        const_: false,
                                        unsafe_: false,
                                        async_: false,
                                        abi: erised::types::Abi::Rust,
                                    },
                                    has_body: true,
                                })
                            },
                        ],
                        negative: false,
                        synthetic: false,
                        blanket_impl: Some(erised::types::Type::Generic("T")),
                    })
                }),
                erised::types::Identifiable::Item(|| {
                    erised::types::Item::Impl(erised::types::Impl {
                        meta: erised::types::ItemMeta {
                            krate: || erised::types::ExternalCrate {
                                name: "erised_tests",
                                html_root_url: None,
                            },
                            summary: None,
                            span: None,
                            visibility: erised::types::Visibility::Default,
                            docs: None,
                            attrs: &[],
                            deprecation: None,
                        },
                        is_unsafe: false,
                        generics: erised::types::Generics {
                            params: &[erised::types::GenericParamDef {
                                name: "T",
                                kind: erised::types::GenericParamDefKind::Type {
                                    bounds: &[],
                                    default: None,
                                    synthetic: false,
                                },
                            }],
                            where_predicates: &[erised::types::WherePredicate::BoundPredicate {
                                type_: erised::types::Type::Generic("T"),
                                bounds: &[erised::types::GenericBound::TraitBound {
                                    trait_: erised::types::Path {
                                        name: "Sized",
                                        prefix: "",
                                        target: erised::types::Identifiable::Summary(|| {
                                            erised::types::ItemSummary {
                                                krate: || erised::types::ExternalCrate {
                                                    name: "core",
                                                    html_root_url: Some(
                                                        "https://doc.rust-lang.org/nightly/",
                                                    ),
                                                },
                                                path: &["core", "marker", "Sized"],
                                                kind: erised::types::ItemKind::Trait,
                                            }
                                        }),
                                        args: Some(|| erised::types::GenericArgs::AngleBracketed {
                                            args: &[],
                                            bindings: &[],
                                        }),
                                    },
                                    generic_params: &[],
                                    modifier: erised::types::TraitBoundModifier::Maybe,
                                }],
                                generic_params: &[],
                            }],
                        },
                        provided_trait_methods: &[],
                        trait_: Some(erised::types::Path {
                            name: "Borrow",
                            prefix: "",
                            target: erised::types::Identifiable::Summary(|| {
                                erised::types::ItemSummary {
                                    krate: || erised::types::ExternalCrate {
                                        name: "core",
                                        html_root_url: Some("https://doc.rust-lang.org/nightly/"),
                                    },
                                    path: &["core", "borrow", "Borrow"],
                                    kind: erised::types::ItemKind::Trait,
                                }
                            }),
                            args: Some(|| erised::types::GenericArgs::AngleBracketed {
                                args: &[erised::types::GenericArg::Type(
                                    erised::types::Type::Generic("T"),
                                )],
                                bindings: &[],
                            }),
                        }),
                        for_: erised::types::Type::ResolvedPath(erised::types::Path {
                            name: "MyStruct",
                            prefix: "",
                            target: erised::types::Identifiable::Item(|| {
                                <crate::MyStruct as Reflect>::TYPE_INFO
                            }),
                            args: Some(|| erised::types::GenericArgs::AngleBracketed {
                                args: &[],
                                bindings: &[],
                            }),
                        }),
                        items: &[|| {
                            erised::types::Item::Function(erised::types::Function {
                                name: "borrow",
                                meta: erised::types::ItemMeta {
                                    krate: || erised::types::ExternalCrate {
                                        name: "core",
                                        html_root_url: Some("https://doc.rust-lang.org/nightly/"),
                                    },
                                    summary: None,
                                    span: None,
                                    visibility: erised::types::Visibility::Default,
                                    docs: None,
                                    attrs: &["#[rustc_diagnostic_item = \"noop_method_borrow\"]"],
                                    deprecation: None,
                                },
                                decl: erised::types::FnDecl {
                                    inputs: &[erised::types::FnInput {
                                        pat: "self",
                                        ty: erised::types::Type::BorrowedRef {
                                            lifetime: None,
                                            mutable: false,
                                            type_: || erised::types::Type::Generic("Self"),
                                        },
                                    }],
                                    output: Some(erised::types::Type::BorrowedRef {
                                        lifetime: None,
                                        mutable: false,
                                        type_: || erised::types::Type::Generic("T"),
                                    }),
                                    c_variadic: false,
                                },
                                generics: erised::types::Generics {
                                    params: &[],
                                    where_predicates: &[],
                                },
                                header: erised::types::Header {
                                    const_: false,
                                    unsafe_: false,
                                    async_: false,
                                    abi: erised::types::Abi::Rust,
                                },
                                has_body: true,
                            })
                        }],
                        negative: false,
                        synthetic: false,
                        blanket_impl: Some(erised::types::Type::Generic("T")),
                    })
                }),
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
                    path: &["crate", "MyEnum"],
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
                        path: &["crate", "MyEnum", "Unit"],
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
                erised::types::Identifiable::Item(|| {
                    erised::types::Item::Impl(erised::types::Impl {
                        meta: erised::types::ItemMeta {
                            krate: || erised::types::ExternalCrate {
                                name: "erised_tests",
                                html_root_url: None,
                            },
                            summary: None,
                            span: None,
                            visibility: erised::types::Visibility::Default,
                            docs: None,
                            attrs: &[],
                            deprecation: None,
                        },
                        is_unsafe: false,
                        generics: erised::types::Generics {
                            params: &[],
                            where_predicates: &[],
                        },
                        provided_trait_methods: &[],
                        trait_: Some(erised::types::Path {
                            name: "Send",
                            prefix: "",
                            target: erised::types::Identifiable::Summary(|| {
                                erised::types::ItemSummary {
                                    krate: || erised::types::ExternalCrate {
                                        name: "core",
                                        html_root_url: Some("https://doc.rust-lang.org/nightly/"),
                                    },
                                    path: &["core", "marker", "Send"],
                                    kind: erised::types::ItemKind::Trait,
                                }
                            }),
                            args: Some(|| erised::types::GenericArgs::AngleBracketed {
                                args: &[],
                                bindings: &[],
                            }),
                        }),
                        for_: erised::types::Type::ResolvedPath(erised::types::Path {
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
                        items: &[],
                        negative: false,
                        synthetic: true,
                        blanket_impl: None,
                    })
                }),
                erised::types::Identifiable::Item(|| {
                    erised::types::Item::Impl(erised::types::Impl {
                        meta: erised::types::ItemMeta {
                            krate: || erised::types::ExternalCrate {
                                name: "erised_tests",
                                html_root_url: None,
                            },
                            summary: None,
                            span: None,
                            visibility: erised::types::Visibility::Default,
                            docs: None,
                            attrs: &[],
                            deprecation: None,
                        },
                        is_unsafe: false,
                        generics: erised::types::Generics {
                            params: &[],
                            where_predicates: &[],
                        },
                        provided_trait_methods: &[],
                        trait_: Some(erised::types::Path {
                            name: "Unpin",
                            prefix: "",
                            target: erised::types::Identifiable::Summary(|| {
                                erised::types::ItemSummary {
                                    krate: || erised::types::ExternalCrate {
                                        name: "core",
                                        html_root_url: Some("https://doc.rust-lang.org/nightly/"),
                                    },
                                    path: &["core", "marker", "Unpin"],
                                    kind: erised::types::ItemKind::Trait,
                                }
                            }),
                            args: Some(|| erised::types::GenericArgs::AngleBracketed {
                                args: &[],
                                bindings: &[],
                            }),
                        }),
                        for_: erised::types::Type::ResolvedPath(erised::types::Path {
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
                        items: &[],
                        negative: false,
                        synthetic: true,
                        blanket_impl: None,
                    })
                }),
                erised::types::Identifiable::Item(|| {
                    erised::types::Item::Impl(erised::types::Impl {
                        meta: erised::types::ItemMeta {
                            krate: || erised::types::ExternalCrate {
                                name: "erised_tests",
                                html_root_url: None,
                            },
                            summary: None,
                            span: None,
                            visibility: erised::types::Visibility::Default,
                            docs: None,
                            attrs: &[],
                            deprecation: None,
                        },
                        is_unsafe: false,
                        generics: erised::types::Generics {
                            params: &[],
                            where_predicates: &[],
                        },
                        provided_trait_methods: &[],
                        trait_: Some(erised::types::Path {
                            name: "RefUnwindSafe",
                            prefix: "",
                            target: erised::types::Identifiable::Summary(|| {
                                erised::types::ItemSummary {
                                    krate: || erised::types::ExternalCrate {
                                        name: "core",
                                        html_root_url: Some("https://doc.rust-lang.org/nightly/"),
                                    },
                                    path: &["core", "panic", "unwind_safe", "RefUnwindSafe"],
                                    kind: erised::types::ItemKind::Trait,
                                }
                            }),
                            args: Some(|| erised::types::GenericArgs::AngleBracketed {
                                args: &[],
                                bindings: &[],
                            }),
                        }),
                        for_: erised::types::Type::ResolvedPath(erised::types::Path {
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
                        items: &[],
                        negative: false,
                        synthetic: true,
                        blanket_impl: None,
                    })
                }),
                erised::types::Identifiable::Item(|| {
                    erised::types::Item::Impl(erised::types::Impl {
                        meta: erised::types::ItemMeta {
                            krate: || erised::types::ExternalCrate {
                                name: "erised_tests",
                                html_root_url: None,
                            },
                            summary: None,
                            span: None,
                            visibility: erised::types::Visibility::Default,
                            docs: None,
                            attrs: &[],
                            deprecation: None,
                        },
                        is_unsafe: false,
                        generics: erised::types::Generics {
                            params: &[],
                            where_predicates: &[],
                        },
                        provided_trait_methods: &[],
                        trait_: Some(erised::types::Path {
                            name: "Sync",
                            prefix: "",
                            target: erised::types::Identifiable::Summary(|| {
                                erised::types::ItemSummary {
                                    krate: || erised::types::ExternalCrate {
                                        name: "core",
                                        html_root_url: Some("https://doc.rust-lang.org/nightly/"),
                                    },
                                    path: &["core", "marker", "Sync"],
                                    kind: erised::types::ItemKind::Trait,
                                }
                            }),
                            args: Some(|| erised::types::GenericArgs::AngleBracketed {
                                args: &[],
                                bindings: &[],
                            }),
                        }),
                        for_: erised::types::Type::ResolvedPath(erised::types::Path {
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
                        items: &[],
                        negative: false,
                        synthetic: true,
                        blanket_impl: None,
                    })
                }),
                erised::types::Identifiable::Item(|| {
                    erised::types::Item::Impl(erised::types::Impl {
                        meta: erised::types::ItemMeta {
                            krate: || erised::types::ExternalCrate {
                                name: "erised_tests",
                                html_root_url: None,
                            },
                            summary: None,
                            span: None,
                            visibility: erised::types::Visibility::Default,
                            docs: None,
                            attrs: &[],
                            deprecation: None,
                        },
                        is_unsafe: false,
                        generics: erised::types::Generics {
                            params: &[],
                            where_predicates: &[],
                        },
                        provided_trait_methods: &[],
                        trait_: Some(erised::types::Path {
                            name: "UnwindSafe",
                            prefix: "",
                            target: erised::types::Identifiable::Summary(|| {
                                erised::types::ItemSummary {
                                    krate: || erised::types::ExternalCrate {
                                        name: "core",
                                        html_root_url: Some("https://doc.rust-lang.org/nightly/"),
                                    },
                                    path: &["core", "panic", "unwind_safe", "UnwindSafe"],
                                    kind: erised::types::ItemKind::Trait,
                                }
                            }),
                            args: Some(|| erised::types::GenericArgs::AngleBracketed {
                                args: &[],
                                bindings: &[],
                            }),
                        }),
                        for_: erised::types::Type::ResolvedPath(erised::types::Path {
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
                        items: &[],
                        negative: false,
                        synthetic: true,
                        blanket_impl: None,
                    })
                }),
                erised::types::Identifiable::Item(|| {
                    erised::types::Item::Impl(erised::types::Impl {
                        meta: erised::types::ItemMeta {
                            krate: || erised::types::ExternalCrate {
                                name: "erised_tests",
                                html_root_url: None,
                            },
                            summary: None,
                            span: None,
                            visibility: erised::types::Visibility::Default,
                            docs: None,
                            attrs: &[],
                            deprecation: None,
                        },
                        is_unsafe: false,
                        generics: erised::types::Generics {
                            params: &[
                                erised::types::GenericParamDef {
                                    name: "T",
                                    kind: erised::types::GenericParamDefKind::Type {
                                        bounds: &[],
                                        default: None,
                                        synthetic: false,
                                    },
                                },
                                erised::types::GenericParamDef {
                                    name: "U",
                                    kind: erised::types::GenericParamDefKind::Type {
                                        bounds: &[],
                                        default: None,
                                        synthetic: false,
                                    },
                                },
                            ],
                            where_predicates: &[erised::types::WherePredicate::BoundPredicate {
                                type_: erised::types::Type::Generic("U"),
                                bounds: &[erised::types::GenericBound::TraitBound {
                                    trait_: erised::types::Path {
                                        name: "From",
                                        prefix: "",
                                        target: erised::types::Identifiable::Summary(|| {
                                            erised::types::ItemSummary {
                                                krate: || erised::types::ExternalCrate {
                                                    name: "core",
                                                    html_root_url: Some(
                                                        "https://doc.rust-lang.org/nightly/",
                                                    ),
                                                },
                                                path: &["core", "convert", "From"],
                                                kind: erised::types::ItemKind::Trait,
                                            }
                                        }),
                                        args: Some(|| erised::types::GenericArgs::AngleBracketed {
                                            args: &[erised::types::GenericArg::Type(
                                                erised::types::Type::Generic("T"),
                                            )],
                                            bindings: &[],
                                        }),
                                    },
                                    generic_params: &[],
                                    modifier: erised::types::TraitBoundModifier::None,
                                }],
                                generic_params: &[],
                            }],
                        },
                        provided_trait_methods: &[],
                        trait_: Some(erised::types::Path {
                            name: "Into",
                            prefix: "",
                            target: erised::types::Identifiable::Summary(|| {
                                erised::types::ItemSummary {
                                    krate: || erised::types::ExternalCrate {
                                        name: "core",
                                        html_root_url: Some("https://doc.rust-lang.org/nightly/"),
                                    },
                                    path: &["core", "convert", "Into"],
                                    kind: erised::types::ItemKind::Trait,
                                }
                            }),
                            args: Some(|| erised::types::GenericArgs::AngleBracketed {
                                args: &[erised::types::GenericArg::Type(
                                    erised::types::Type::Generic("U"),
                                )],
                                bindings: &[],
                            }),
                        }),
                        for_: erised::types::Type::ResolvedPath(erised::types::Path {
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
                        items: &[|| {
                            erised :: types :: Item :: Function (erised :: types :: Function { name : "into" , meta : erised :: types :: ItemMeta { krate : || erised :: types :: ExternalCrate { name : "core" , html_root_url : Some ("https://doc.rust-lang.org/nightly/") } , summary : None , span : None , visibility : erised :: types :: Visibility :: Default , docs : Some ("Calls `U::from(self)`.\n\nThat is, this conversion is whatever the implementation of\n<code>[From]&lt;T&gt; for U</code> chooses to do.") , attrs : & [] , deprecation : None } , decl : erised :: types :: FnDecl { inputs : & [erised :: types :: FnInput { pat : "self" , ty : erised :: types :: Type :: Generic ("Self") }] , output : Some (erised :: types :: Type :: Generic ("U")) , c_variadic : false } , generics : erised :: types :: Generics { params : & [] , where_predicates : & [] } , header : erised :: types :: Header { const_ : false , unsafe_ : false , async_ : false , abi : erised :: types :: Abi :: Rust } , has_body : true })
                        }],
                        negative: false,
                        synthetic: false,
                        blanket_impl: Some(erised::types::Type::Generic("T")),
                    })
                }),
                erised::types::Identifiable::Item(|| {
                    erised::types::Item::Impl(erised::types::Impl {
                        meta: erised::types::ItemMeta {
                            krate: || erised::types::ExternalCrate {
                                name: "erised_tests",
                                html_root_url: None,
                            },
                            summary: None,
                            span: None,
                            visibility: erised::types::Visibility::Default,
                            docs: None,
                            attrs: &[],
                            deprecation: None,
                        },
                        is_unsafe: false,
                        generics: erised::types::Generics {
                            params: &[erised::types::GenericParamDef {
                                name: "T",
                                kind: erised::types::GenericParamDefKind::Type {
                                    bounds: &[],
                                    default: None,
                                    synthetic: false,
                                },
                            }],
                            where_predicates: &[erised::types::WherePredicate::BoundPredicate {
                                type_: erised::types::Type::Generic("T"),
                                bounds: &[
                                    erised::types::GenericBound::Outlives("'static"),
                                    erised::types::GenericBound::TraitBound {
                                        trait_: erised::types::Path {
                                            name: "Sized",
                                            prefix: "",
                                            target: erised::types::Identifiable::Summary(|| {
                                                erised::types::ItemSummary {
                                                    krate: || erised::types::ExternalCrate {
                                                        name: "core",
                                                        html_root_url: Some(
                                                            "https://doc.rust-lang.org/nightly/",
                                                        ),
                                                    },
                                                    path: &["core", "marker", "Sized"],
                                                    kind: erised::types::ItemKind::Trait,
                                                }
                                            }),
                                            args: Some(|| {
                                                erised::types::GenericArgs::AngleBracketed {
                                                    args: &[],
                                                    bindings: &[],
                                                }
                                            }),
                                        },
                                        generic_params: &[],
                                        modifier: erised::types::TraitBoundModifier::Maybe,
                                    },
                                ],
                                generic_params: &[],
                            }],
                        },
                        provided_trait_methods: &[],
                        trait_: Some(erised::types::Path {
                            name: "Any",
                            prefix: "",
                            target: erised::types::Identifiable::Summary(|| {
                                erised::types::ItemSummary {
                                    krate: || erised::types::ExternalCrate {
                                        name: "core",
                                        html_root_url: Some("https://doc.rust-lang.org/nightly/"),
                                    },
                                    path: &["core", "any", "Any"],
                                    kind: erised::types::ItemKind::Trait,
                                }
                            }),
                            args: Some(|| erised::types::GenericArgs::AngleBracketed {
                                args: &[],
                                bindings: &[],
                            }),
                        }),
                        for_: erised::types::Type::ResolvedPath(erised::types::Path {
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
                        items: &[|| {
                            erised::types::Item::Function(erised::types::Function {
                                name: "type_id",
                                meta: erised::types::ItemMeta {
                                    krate: || erised::types::ExternalCrate {
                                        name: "core",
                                        html_root_url: Some("https://doc.rust-lang.org/nightly/"),
                                    },
                                    summary: None,
                                    span: None,
                                    visibility: erised::types::Visibility::Default,
                                    docs: None,
                                    attrs: &[],
                                    deprecation: None,
                                },
                                decl: erised::types::FnDecl {
                                    inputs: &[erised::types::FnInput {
                                        pat: "self",
                                        ty: erised::types::Type::BorrowedRef {
                                            lifetime: None,
                                            mutable: false,
                                            type_: || erised::types::Type::Generic("Self"),
                                        },
                                    }],
                                    output: Some(erised::types::Type::ResolvedPath(
                                        erised::types::Path {
                                            name: "TypeId",
                                            prefix: "",
                                            target: erised::types::Identifiable::Summary(|| {
                                                erised::types::ItemSummary {
                                                    krate: || erised::types::ExternalCrate {
                                                        name: "core",
                                                        html_root_url: Some(
                                                            "https://doc.rust-lang.org/nightly/",
                                                        ),
                                                    },
                                                    path: &["core", "any", "TypeId"],
                                                    kind: erised::types::ItemKind::Struct,
                                                }
                                            }),
                                            args: Some(|| {
                                                erised::types::GenericArgs::AngleBracketed {
                                                    args: &[],
                                                    bindings: &[],
                                                }
                                            }),
                                        },
                                    )),
                                    c_variadic: false,
                                },
                                generics: erised::types::Generics {
                                    params: &[],
                                    where_predicates: &[],
                                },
                                header: erised::types::Header {
                                    const_: false,
                                    unsafe_: false,
                                    async_: false,
                                    abi: erised::types::Abi::Rust,
                                },
                                has_body: true,
                            })
                        }],
                        negative: false,
                        synthetic: false,
                        blanket_impl: Some(erised::types::Type::Generic("T")),
                    })
                }),
                erised::types::Identifiable::Item(|| {
                    erised::types::Item::Impl(erised::types::Impl {
                        meta: erised::types::ItemMeta {
                            krate: || erised::types::ExternalCrate {
                                name: "erised_tests",
                                html_root_url: None,
                            },
                            summary: None,
                            span: None,
                            visibility: erised::types::Visibility::Default,
                            docs: None,
                            attrs: &[],
                            deprecation: None,
                        },
                        is_unsafe: false,
                        generics: erised::types::Generics {
                            params: &[erised::types::GenericParamDef {
                                name: "T",
                                kind: erised::types::GenericParamDefKind::Type {
                                    bounds: &[],
                                    default: None,
                                    synthetic: false,
                                },
                            }],
                            where_predicates: &[erised::types::WherePredicate::BoundPredicate {
                                type_: erised::types::Type::Generic("T"),
                                bounds: &[erised::types::GenericBound::TraitBound {
                                    trait_: erised::types::Path {
                                        name: "Sized",
                                        prefix: "",
                                        target: erised::types::Identifiable::Summary(|| {
                                            erised::types::ItemSummary {
                                                krate: || erised::types::ExternalCrate {
                                                    name: "core",
                                                    html_root_url: Some(
                                                        "https://doc.rust-lang.org/nightly/",
                                                    ),
                                                },
                                                path: &["core", "marker", "Sized"],
                                                kind: erised::types::ItemKind::Trait,
                                            }
                                        }),
                                        args: Some(|| erised::types::GenericArgs::AngleBracketed {
                                            args: &[],
                                            bindings: &[],
                                        }),
                                    },
                                    generic_params: &[],
                                    modifier: erised::types::TraitBoundModifier::Maybe,
                                }],
                                generic_params: &[],
                            }],
                        },
                        provided_trait_methods: &[],
                        trait_: Some(erised::types::Path {
                            name: "BorrowMut",
                            prefix: "",
                            target: erised::types::Identifiable::Summary(|| {
                                erised::types::ItemSummary {
                                    krate: || erised::types::ExternalCrate {
                                        name: "core",
                                        html_root_url: Some("https://doc.rust-lang.org/nightly/"),
                                    },
                                    path: &["core", "borrow", "BorrowMut"],
                                    kind: erised::types::ItemKind::Trait,
                                }
                            }),
                            args: Some(|| erised::types::GenericArgs::AngleBracketed {
                                args: &[erised::types::GenericArg::Type(
                                    erised::types::Type::Generic("T"),
                                )],
                                bindings: &[],
                            }),
                        }),
                        for_: erised::types::Type::ResolvedPath(erised::types::Path {
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
                        items: &[|| {
                            erised::types::Item::Function(erised::types::Function {
                                name: "borrow_mut",
                                meta: erised::types::ItemMeta {
                                    krate: || erised::types::ExternalCrate {
                                        name: "core",
                                        html_root_url: Some("https://doc.rust-lang.org/nightly/"),
                                    },
                                    summary: None,
                                    span: None,
                                    visibility: erised::types::Visibility::Default,
                                    docs: None,
                                    attrs: &[],
                                    deprecation: None,
                                },
                                decl: erised::types::FnDecl {
                                    inputs: &[erised::types::FnInput {
                                        pat: "self",
                                        ty: erised::types::Type::BorrowedRef {
                                            lifetime: None,
                                            mutable: true,
                                            type_: || erised::types::Type::Generic("Self"),
                                        },
                                    }],
                                    output: Some(erised::types::Type::BorrowedRef {
                                        lifetime: None,
                                        mutable: true,
                                        type_: || erised::types::Type::Generic("T"),
                                    }),
                                    c_variadic: false,
                                },
                                generics: erised::types::Generics {
                                    params: &[],
                                    where_predicates: &[],
                                },
                                header: erised::types::Header {
                                    const_: false,
                                    unsafe_: false,
                                    async_: false,
                                    abi: erised::types::Abi::Rust,
                                },
                                has_body: true,
                            })
                        }],
                        negative: false,
                        synthetic: false,
                        blanket_impl: Some(erised::types::Type::Generic("T")),
                    })
                }),
                erised::types::Identifiable::Item(|| {
                    erised::types::Item::Impl(erised::types::Impl {
                        meta: erised::types::ItemMeta {
                            krate: || erised::types::ExternalCrate {
                                name: "erised_tests",
                                html_root_url: None,
                            },
                            summary: None,
                            span: None,
                            visibility: erised::types::Visibility::Default,
                            docs: None,
                            attrs: &[],
                            deprecation: None,
                        },
                        is_unsafe: false,
                        generics: erised::types::Generics {
                            params: &[
                                erised::types::GenericParamDef {
                                    name: "T",
                                    kind: erised::types::GenericParamDefKind::Type {
                                        bounds: &[],
                                        default: None,
                                        synthetic: false,
                                    },
                                },
                                erised::types::GenericParamDef {
                                    name: "U",
                                    kind: erised::types::GenericParamDefKind::Type {
                                        bounds: &[],
                                        default: None,
                                        synthetic: false,
                                    },
                                },
                            ],
                            where_predicates: &[erised::types::WherePredicate::BoundPredicate {
                                type_: erised::types::Type::Generic("U"),
                                bounds: &[erised::types::GenericBound::TraitBound {
                                    trait_: erised::types::Path {
                                        name: "Into",
                                        prefix: "",
                                        target: erised::types::Identifiable::Summary(|| {
                                            erised::types::ItemSummary {
                                                krate: || erised::types::ExternalCrate {
                                                    name: "core",
                                                    html_root_url: Some(
                                                        "https://doc.rust-lang.org/nightly/",
                                                    ),
                                                },
                                                path: &["core", "convert", "Into"],
                                                kind: erised::types::ItemKind::Trait,
                                            }
                                        }),
                                        args: Some(|| erised::types::GenericArgs::AngleBracketed {
                                            args: &[erised::types::GenericArg::Type(
                                                erised::types::Type::Generic("T"),
                                            )],
                                            bindings: &[],
                                        }),
                                    },
                                    generic_params: &[],
                                    modifier: erised::types::TraitBoundModifier::None,
                                }],
                                generic_params: &[],
                            }],
                        },
                        provided_trait_methods: &[],
                        trait_: Some(erised::types::Path {
                            name: "TryFrom",
                            prefix: "",
                            target: erised::types::Identifiable::Summary(|| {
                                erised::types::ItemSummary {
                                    krate: || erised::types::ExternalCrate {
                                        name: "core",
                                        html_root_url: Some("https://doc.rust-lang.org/nightly/"),
                                    },
                                    path: &["core", "convert", "TryFrom"],
                                    kind: erised::types::ItemKind::Trait,
                                }
                            }),
                            args: Some(|| erised::types::GenericArgs::AngleBracketed {
                                args: &[erised::types::GenericArg::Type(
                                    erised::types::Type::Generic("U"),
                                )],
                                bindings: &[],
                            }),
                        }),
                        for_: erised::types::Type::ResolvedPath(erised::types::Path {
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
                        items: &[
                            || erised::types::Item::AssocType {
                                meta: erised::types::ItemMeta {
                                    krate: || erised::types::ExternalCrate {
                                        name: "core",
                                        html_root_url: Some("https://doc.rust-lang.org/nightly/"),
                                    },
                                    summary: None,
                                    span: None,
                                    visibility: erised::types::Visibility::Default,
                                    docs: None,
                                    attrs: &[],
                                    deprecation: None,
                                },
                                generics: erised::types::Generics {
                                    params: &[],
                                    where_predicates: &[],
                                },
                                bounds: &[],
                                default: Some(erised::types::Type::ResolvedPath(
                                    erised::types::Path {
                                        name: "Infallible",
                                        prefix: "",
                                        target: erised::types::Identifiable::Summary(|| {
                                            erised::types::ItemSummary {
                                                krate: || erised::types::ExternalCrate {
                                                    name: "core",
                                                    html_root_url: Some(
                                                        "https://doc.rust-lang.org/nightly/",
                                                    ),
                                                },
                                                path: &["core", "convert", "Infallible"],
                                                kind: erised::types::ItemKind::Enum,
                                            }
                                        }),
                                        args: Some(|| erised::types::GenericArgs::AngleBracketed {
                                            args: &[],
                                            bindings: &[],
                                        }),
                                    },
                                )),
                            },
                            || {
                                erised::types::Item::Function(erised::types::Function {
                                    name: "try_from",
                                    meta: erised::types::ItemMeta {
                                        krate: || erised::types::ExternalCrate {
                                            name: "core",
                                            html_root_url: Some(
                                                "https://doc.rust-lang.org/nightly/",
                                            ),
                                        },
                                        summary: None,
                                        span: None,
                                        visibility: erised::types::Visibility::Default,
                                        docs: None,
                                        attrs: &[],
                                        deprecation: None,
                                    },
                                    decl: erised::types::FnDecl {
                                        inputs: &[erised::types::FnInput {
                                            pat: "value",
                                            ty: erised::types::Type::Generic("U"),
                                        }],
                                        output: Some(erised::types::Type::ResolvedPath(
                                            erised::types::Path {
                                                name: "Result",
                                                prefix: "",
                                                target: erised::types::Identifiable::Summary(
                                                    || erised::types::ItemSummary {
                                                        krate: || {
                                                            erised :: types :: ExternalCrate { name : "core" , html_root_url : Some ("https://doc.rust-lang.org/nightly/") }
                                                        },
                                                        path: &["core", "result", "Result"],
                                                        kind: erised::types::ItemKind::Enum,
                                                    },
                                                ),
                                                args: Some(|| {
                                                    erised :: types :: GenericArgs :: AngleBracketed { args : & [erised :: types :: GenericArg :: Type (erised :: types :: Type :: Generic ("T")) , erised :: types :: GenericArg :: Type (erised :: types :: Type :: QualifiedPath { name : "Error" , args : || erised :: types :: GenericArgs :: AngleBracketed { args : & [] , bindings : & [] } , self_type : || erised :: types :: Type :: Generic ("T") , trait_ : erised :: types :: Path { name : "TryFrom" , prefix : "" , target : erised :: types :: Identifiable :: Summary (|| erised :: types :: ItemSummary { krate : || erised :: types :: ExternalCrate { name : "core" , html_root_url : Some ("https://doc.rust-lang.org/nightly/") } , path : & ["core" , "convert" , "TryFrom"] , kind : erised :: types :: ItemKind :: Trait }) , args : Some (|| erised :: types :: GenericArgs :: AngleBracketed { args : & [erised :: types :: GenericArg :: Type (erised :: types :: Type :: Generic ("U"))] , bindings : & [] }) } })] , bindings : & [] }
                                                }),
                                            },
                                        )),
                                        c_variadic: false,
                                    },
                                    generics: erised::types::Generics {
                                        params: &[],
                                        where_predicates: &[],
                                    },
                                    header: erised::types::Header {
                                        const_: false,
                                        unsafe_: false,
                                        async_: false,
                                        abi: erised::types::Abi::Rust,
                                    },
                                    has_body: true,
                                })
                            },
                        ],
                        negative: false,
                        synthetic: false,
                        blanket_impl: Some(erised::types::Type::Generic("T")),
                    })
                }),
                erised::types::Identifiable::Item(|| {
                    erised::types::Item::Impl(erised::types::Impl {
                        meta: erised::types::ItemMeta {
                            krate: || erised::types::ExternalCrate {
                                name: "erised_tests",
                                html_root_url: None,
                            },
                            summary: None,
                            span: None,
                            visibility: erised::types::Visibility::Default,
                            docs: None,
                            attrs: &[],
                            deprecation: None,
                        },
                        is_unsafe: false,
                        generics: erised::types::Generics {
                            params: &[erised::types::GenericParamDef {
                                name: "T",
                                kind: erised::types::GenericParamDefKind::Type {
                                    bounds: &[],
                                    default: None,
                                    synthetic: false,
                                },
                            }],
                            where_predicates: &[],
                        },
                        provided_trait_methods: &[],
                        trait_: Some(erised::types::Path {
                            name: "From",
                            prefix: "",
                            target: erised::types::Identifiable::Summary(|| {
                                erised::types::ItemSummary {
                                    krate: || erised::types::ExternalCrate {
                                        name: "core",
                                        html_root_url: Some("https://doc.rust-lang.org/nightly/"),
                                    },
                                    path: &["core", "convert", "From"],
                                    kind: erised::types::ItemKind::Trait,
                                }
                            }),
                            args: Some(|| erised::types::GenericArgs::AngleBracketed {
                                args: &[erised::types::GenericArg::Type(
                                    erised::types::Type::Generic("T"),
                                )],
                                bindings: &[],
                            }),
                        }),
                        for_: erised::types::Type::ResolvedPath(erised::types::Path {
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
                        items: &[|| {
                            erised::types::Item::Function(erised::types::Function {
                                name: "from",
                                meta: erised::types::ItemMeta {
                                    krate: || erised::types::ExternalCrate {
                                        name: "core",
                                        html_root_url: Some("https://doc.rust-lang.org/nightly/"),
                                    },
                                    summary: None,
                                    span: None,
                                    visibility: erised::types::Visibility::Default,
                                    docs: Some("Returns the argument unchanged."),
                                    attrs: &[],
                                    deprecation: None,
                                },
                                decl: erised::types::FnDecl {
                                    inputs: &[erised::types::FnInput {
                                        pat: "t",
                                        ty: erised::types::Type::Generic("T"),
                                    }],
                                    output: Some(erised::types::Type::Generic("T")),
                                    c_variadic: false,
                                },
                                generics: erised::types::Generics {
                                    params: &[],
                                    where_predicates: &[],
                                },
                                header: erised::types::Header {
                                    const_: false,
                                    unsafe_: false,
                                    async_: false,
                                    abi: erised::types::Abi::Rust,
                                },
                                has_body: true,
                            })
                        }],
                        negative: false,
                        synthetic: false,
                        blanket_impl: Some(erised::types::Type::Generic("T")),
                    })
                }),
                erised::types::Identifiable::Item(|| {
                    erised::types::Item::Impl(erised::types::Impl {
                        meta: erised::types::ItemMeta {
                            krate: || erised::types::ExternalCrate {
                                name: "erised_tests",
                                html_root_url: None,
                            },
                            summary: None,
                            span: None,
                            visibility: erised::types::Visibility::Default,
                            docs: None,
                            attrs: &[],
                            deprecation: None,
                        },
                        is_unsafe: false,
                        generics: erised::types::Generics {
                            params: &[
                                erised::types::GenericParamDef {
                                    name: "T",
                                    kind: erised::types::GenericParamDefKind::Type {
                                        bounds: &[],
                                        default: None,
                                        synthetic: false,
                                    },
                                },
                                erised::types::GenericParamDef {
                                    name: "U",
                                    kind: erised::types::GenericParamDefKind::Type {
                                        bounds: &[],
                                        default: None,
                                        synthetic: false,
                                    },
                                },
                            ],
                            where_predicates: &[erised::types::WherePredicate::BoundPredicate {
                                type_: erised::types::Type::Generic("U"),
                                bounds: &[erised::types::GenericBound::TraitBound {
                                    trait_: erised::types::Path {
                                        name: "TryFrom",
                                        prefix: "",
                                        target: erised::types::Identifiable::Summary(|| {
                                            erised::types::ItemSummary {
                                                krate: || erised::types::ExternalCrate {
                                                    name: "core",
                                                    html_root_url: Some(
                                                        "https://doc.rust-lang.org/nightly/",
                                                    ),
                                                },
                                                path: &["core", "convert", "TryFrom"],
                                                kind: erised::types::ItemKind::Trait,
                                            }
                                        }),
                                        args: Some(|| erised::types::GenericArgs::AngleBracketed {
                                            args: &[erised::types::GenericArg::Type(
                                                erised::types::Type::Generic("T"),
                                            )],
                                            bindings: &[],
                                        }),
                                    },
                                    generic_params: &[],
                                    modifier: erised::types::TraitBoundModifier::None,
                                }],
                                generic_params: &[],
                            }],
                        },
                        provided_trait_methods: &[],
                        trait_: Some(erised::types::Path {
                            name: "TryInto",
                            prefix: "",
                            target: erised::types::Identifiable::Summary(|| {
                                erised::types::ItemSummary {
                                    krate: || erised::types::ExternalCrate {
                                        name: "core",
                                        html_root_url: Some("https://doc.rust-lang.org/nightly/"),
                                    },
                                    path: &["core", "convert", "TryInto"],
                                    kind: erised::types::ItemKind::Trait,
                                }
                            }),
                            args: Some(|| erised::types::GenericArgs::AngleBracketed {
                                args: &[erised::types::GenericArg::Type(
                                    erised::types::Type::Generic("U"),
                                )],
                                bindings: &[],
                            }),
                        }),
                        for_: erised::types::Type::ResolvedPath(erised::types::Path {
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
                        items: &[
                            || erised::types::Item::AssocType {
                                meta: erised::types::ItemMeta {
                                    krate: || erised::types::ExternalCrate {
                                        name: "core",
                                        html_root_url: Some("https://doc.rust-lang.org/nightly/"),
                                    },
                                    summary: None,
                                    span: None,
                                    visibility: erised::types::Visibility::Default,
                                    docs: None,
                                    attrs: &[],
                                    deprecation: None,
                                },
                                generics: erised::types::Generics {
                                    params: &[],
                                    where_predicates: &[],
                                },
                                bounds: &[],
                                default: Some(erised::types::Type::QualifiedPath {
                                    name: "Error",
                                    args: || erised::types::GenericArgs::AngleBracketed {
                                        args: &[],
                                        bindings: &[],
                                    },
                                    self_type: || erised::types::Type::Generic("U"),
                                    trait_: erised::types::Path {
                                        name: "TryFrom",
                                        prefix: "",
                                        target: erised::types::Identifiable::Summary(|| {
                                            erised::types::ItemSummary {
                                                krate: || erised::types::ExternalCrate {
                                                    name: "core",
                                                    html_root_url: Some(
                                                        "https://doc.rust-lang.org/nightly/",
                                                    ),
                                                },
                                                path: &["core", "convert", "TryFrom"],
                                                kind: erised::types::ItemKind::Trait,
                                            }
                                        }),
                                        args: Some(|| erised::types::GenericArgs::AngleBracketed {
                                            args: &[erised::types::GenericArg::Type(
                                                erised::types::Type::Generic("T"),
                                            )],
                                            bindings: &[],
                                        }),
                                    },
                                }),
                            },
                            || {
                                erised::types::Item::Function(erised::types::Function {
                                    name: "try_into",
                                    meta: erised::types::ItemMeta {
                                        krate: || erised::types::ExternalCrate {
                                            name: "core",
                                            html_root_url: Some(
                                                "https://doc.rust-lang.org/nightly/",
                                            ),
                                        },
                                        summary: None,
                                        span: None,
                                        visibility: erised::types::Visibility::Default,
                                        docs: None,
                                        attrs: &[],
                                        deprecation: None,
                                    },
                                    decl: erised::types::FnDecl {
                                        inputs: &[erised::types::FnInput {
                                            pat: "self",
                                            ty: erised::types::Type::Generic("Self"),
                                        }],
                                        output: Some(erised::types::Type::ResolvedPath(
                                            erised::types::Path {
                                                name: "Result",
                                                prefix: "",
                                                target: erised::types::Identifiable::Summary(
                                                    || erised::types::ItemSummary {
                                                        krate: || {
                                                            erised :: types :: ExternalCrate { name : "core" , html_root_url : Some ("https://doc.rust-lang.org/nightly/") }
                                                        },
                                                        path: &["core", "result", "Result"],
                                                        kind: erised::types::ItemKind::Enum,
                                                    },
                                                ),
                                                args: Some(|| {
                                                    erised :: types :: GenericArgs :: AngleBracketed { args : & [erised :: types :: GenericArg :: Type (erised :: types :: Type :: Generic ("U")) , erised :: types :: GenericArg :: Type (erised :: types :: Type :: QualifiedPath { name : "Error" , args : || erised :: types :: GenericArgs :: AngleBracketed { args : & [] , bindings : & [] } , self_type : || erised :: types :: Type :: Generic ("U") , trait_ : erised :: types :: Path { name : "TryFrom" , prefix : "" , target : erised :: types :: Identifiable :: Summary (|| erised :: types :: ItemSummary { krate : || erised :: types :: ExternalCrate { name : "core" , html_root_url : Some ("https://doc.rust-lang.org/nightly/") } , path : & ["core" , "convert" , "TryFrom"] , kind : erised :: types :: ItemKind :: Trait }) , args : Some (|| erised :: types :: GenericArgs :: AngleBracketed { args : & [erised :: types :: GenericArg :: Type (erised :: types :: Type :: Generic ("T"))] , bindings : & [] }) } })] , bindings : & [] }
                                                }),
                                            },
                                        )),
                                        c_variadic: false,
                                    },
                                    generics: erised::types::Generics {
                                        params: &[],
                                        where_predicates: &[],
                                    },
                                    header: erised::types::Header {
                                        const_: false,
                                        unsafe_: false,
                                        async_: false,
                                        abi: erised::types::Abi::Rust,
                                    },
                                    has_body: true,
                                })
                            },
                        ],
                        negative: false,
                        synthetic: false,
                        blanket_impl: Some(erised::types::Type::Generic("T")),
                    })
                }),
                erised::types::Identifiable::Item(|| {
                    erised::types::Item::Impl(erised::types::Impl {
                        meta: erised::types::ItemMeta {
                            krate: || erised::types::ExternalCrate {
                                name: "erised_tests",
                                html_root_url: None,
                            },
                            summary: None,
                            span: None,
                            visibility: erised::types::Visibility::Default,
                            docs: None,
                            attrs: &[],
                            deprecation: None,
                        },
                        is_unsafe: false,
                        generics: erised::types::Generics {
                            params: &[erised::types::GenericParamDef {
                                name: "T",
                                kind: erised::types::GenericParamDefKind::Type {
                                    bounds: &[],
                                    default: None,
                                    synthetic: false,
                                },
                            }],
                            where_predicates: &[erised::types::WherePredicate::BoundPredicate {
                                type_: erised::types::Type::Generic("T"),
                                bounds: &[erised::types::GenericBound::TraitBound {
                                    trait_: erised::types::Path {
                                        name: "Sized",
                                        prefix: "",
                                        target: erised::types::Identifiable::Summary(|| {
                                            erised::types::ItemSummary {
                                                krate: || erised::types::ExternalCrate {
                                                    name: "core",
                                                    html_root_url: Some(
                                                        "https://doc.rust-lang.org/nightly/",
                                                    ),
                                                },
                                                path: &["core", "marker", "Sized"],
                                                kind: erised::types::ItemKind::Trait,
                                            }
                                        }),
                                        args: Some(|| erised::types::GenericArgs::AngleBracketed {
                                            args: &[],
                                            bindings: &[],
                                        }),
                                    },
                                    generic_params: &[],
                                    modifier: erised::types::TraitBoundModifier::Maybe,
                                }],
                                generic_params: &[],
                            }],
                        },
                        provided_trait_methods: &[],
                        trait_: Some(erised::types::Path {
                            name: "Borrow",
                            prefix: "",
                            target: erised::types::Identifiable::Summary(|| {
                                erised::types::ItemSummary {
                                    krate: || erised::types::ExternalCrate {
                                        name: "core",
                                        html_root_url: Some("https://doc.rust-lang.org/nightly/"),
                                    },
                                    path: &["core", "borrow", "Borrow"],
                                    kind: erised::types::ItemKind::Trait,
                                }
                            }),
                            args: Some(|| erised::types::GenericArgs::AngleBracketed {
                                args: &[erised::types::GenericArg::Type(
                                    erised::types::Type::Generic("T"),
                                )],
                                bindings: &[],
                            }),
                        }),
                        for_: erised::types::Type::ResolvedPath(erised::types::Path {
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
                        items: &[|| {
                            erised::types::Item::Function(erised::types::Function {
                                name: "borrow",
                                meta: erised::types::ItemMeta {
                                    krate: || erised::types::ExternalCrate {
                                        name: "core",
                                        html_root_url: Some("https://doc.rust-lang.org/nightly/"),
                                    },
                                    summary: None,
                                    span: None,
                                    visibility: erised::types::Visibility::Default,
                                    docs: None,
                                    attrs: &["#[rustc_diagnostic_item = \"noop_method_borrow\"]"],
                                    deprecation: None,
                                },
                                decl: erised::types::FnDecl {
                                    inputs: &[erised::types::FnInput {
                                        pat: "self",
                                        ty: erised::types::Type::BorrowedRef {
                                            lifetime: None,
                                            mutable: false,
                                            type_: || erised::types::Type::Generic("Self"),
                                        },
                                    }],
                                    output: Some(erised::types::Type::BorrowedRef {
                                        lifetime: None,
                                        mutable: false,
                                        type_: || erised::types::Type::Generic("T"),
                                    }),
                                    c_variadic: false,
                                },
                                generics: erised::types::Generics {
                                    params: &[],
                                    where_predicates: &[],
                                },
                                header: erised::types::Header {
                                    const_: false,
                                    unsafe_: false,
                                    async_: false,
                                    abi: erised::types::Abi::Rust,
                                },
                                has_body: true,
                            })
                        }],
                        negative: false,
                        synthetic: false,
                        blanket_impl: Some(erised::types::Type::Generic("T")),
                    })
                }),
            ],
        });
    }
}
pub const ERISED_TESTS: erised::types::Crate = erised::types::Crate {
    root: || erised::types::Module {
        name: "erised_tests",
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
                path: &["crate"],
                kind: erised::types::ItemKind::Module,
            }),
            span: Some(erised::types::Span {
                filename: "src/lib.rs",
                begin: (1usize, 0usize),
                end: (73usize, 1usize),
            }),
            visibility: erised::types::Visibility::Public,
            docs: None,
            attrs: &[],
            deprecation: None,
        },
        is_crate: true,
        items: &[
            erised::types::Identifiable::Item(|| {
                erised::types::Item::Function(erised::types::Function {
                    name: "pretty_print_item",
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
                            path: &["crate", "pretty_print_item"],
                            kind: erised::types::ItemKind::Function,
                        }),
                        span: Some(erised::types::Span {
                            filename: "src/lib.rs",
                            begin: (10usize, 0usize),
                            end: (16usize, 1usize),
                        }),
                        visibility: erised::types::Visibility::Public,
                        docs: None,
                        attrs: &[],
                        deprecation: None,
                    },
                    decl: erised::types::FnDecl {
                        inputs: &[erised::types::FnInput {
                            pat: "tokens",
                            ty: erised::types::Type::ImplTrait(&[
                                erised::types::GenericBound::TraitBound {
                                    trait_: erised::types::Path {
                                        name: "ToTokens",
                                        prefix: "",
                                        target: erised::types::Identifiable::Summary(|| {
                                            erised::types::ItemSummary {
                                                krate: || erised::types::ExternalCrate {
                                                    name: "quote",
                                                    html_root_url: Some(
                                                        "https://docs.rs/quote/1.0.26/",
                                                    ),
                                                },
                                                path: &["quote", "to_tokens", "ToTokens"],
                                                kind: erised::types::ItemKind::Trait,
                                            }
                                        }),
                                        args: Some(|| erised::types::GenericArgs::AngleBracketed {
                                            args: &[],
                                            bindings: &[],
                                        }),
                                    },
                                    generic_params: &[],
                                    modifier: erised::types::TraitBoundModifier::None,
                                },
                            ]),
                        }],
                        output: Some(erised::types::Type::ResolvedPath(erised::types::Path {
                            name: "Result",
                            prefix: "",
                            target: erised::types::Identifiable::Summary(|| {
                                erised::types::ItemSummary {
                                    krate: || erised::types::ExternalCrate {
                                        name: "core",
                                        html_root_url: Some("https://doc.rust-lang.org/nightly/"),
                                    },
                                    path: &["core", "result", "Result"],
                                    kind: erised::types::ItemKind::Enum,
                                }
                            }),
                            args: Some(|| erised::types::GenericArgs::AngleBracketed {
                                args: &[
                                    erised::types::GenericArg::Type(
                                        erised::types::Type::ResolvedPath(erised::types::Path {
                                            name: "String",
                                            prefix: "",
                                            target: erised::types::Identifiable::Summary(|| {
                                                erised::types::ItemSummary {
                                                    krate: || erised::types::ExternalCrate {
                                                        name: "alloc",
                                                        html_root_url: Some(
                                                            "https://doc.rust-lang.org/nightly/",
                                                        ),
                                                    },
                                                    path: &["alloc", "string", "String"],
                                                    kind: erised::types::ItemKind::Struct,
                                                }
                                            }),
                                            args: Some(|| {
                                                erised::types::GenericArgs::AngleBracketed {
                                                    args: &[],
                                                    bindings: &[],
                                                }
                                            }),
                                        }),
                                    ),
                                    erised::types::GenericArg::Type(
                                        erised::types::Type::ResolvedPath(erised::types::Path {
                                            name: "Error",
                                            prefix: "",
                                            target: erised::types::Identifiable::Summary(|| {
                                                erised::types::ItemSummary {
                                                    krate: || erised::types::ExternalCrate {
                                                        name: "anyhow",
                                                        html_root_url: Some(
                                                            "https://docs.rs/anyhow/1.0.70/",
                                                        ),
                                                    },
                                                    path: &["anyhow", "Error"],
                                                    kind: erised::types::ItemKind::Struct,
                                                }
                                            }),
                                            args: Some(|| {
                                                erised::types::GenericArgs::AngleBracketed {
                                                    args: &[],
                                                    bindings: &[],
                                                }
                                            }),
                                        }),
                                    ),
                                ],
                                bindings: &[],
                            }),
                        })),
                        c_variadic: false,
                    },
                    generics: erised::types::Generics {
                        params: &[erised::types::GenericParamDef {
                            name: "impl ToTokens",
                            kind: erised::types::GenericParamDefKind::Type {
                                bounds: &[erised::types::GenericBound::TraitBound {
                                    trait_: erised::types::Path {
                                        name: "ToTokens",
                                        prefix: "",
                                        target: erised::types::Identifiable::Summary(|| {
                                            erised::types::ItemSummary {
                                                krate: || erised::types::ExternalCrate {
                                                    name: "quote",
                                                    html_root_url: Some(
                                                        "https://docs.rs/quote/1.0.26/",
                                                    ),
                                                },
                                                path: &["quote", "to_tokens", "ToTokens"],
                                                kind: erised::types::ItemKind::Trait,
                                            }
                                        }),
                                        args: Some(|| erised::types::GenericArgs::AngleBracketed {
                                            args: &[],
                                            bindings: &[],
                                        }),
                                    },
                                    generic_params: &[],
                                    modifier: erised::types::TraitBoundModifier::None,
                                }],
                                default: None,
                                synthetic: true,
                            },
                        }],
                        where_predicates: &[],
                    },
                    header: erised::types::Header {
                        const_: false,
                        unsafe_: false,
                        async_: false,
                        abi: erised::types::Abi::Rust,
                    },
                    has_body: true,
                })
            }),
            erised::types::Identifiable::Item(|| {
                erised::types::Item::Function(erised::types::Function {
                    name: "pretty_print",
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
                            path: &["crate", "pretty_print"],
                            kind: erised::types::ItemKind::Function,
                        }),
                        span: Some(erised::types::Span {
                            filename: "src/lib.rs",
                            begin: (19usize, 0usize),
                            end: (55usize, 1usize),
                        }),
                        visibility: erised::types::Visibility::Public,
                        docs: Some("Use `rustfmt` to pretty-print the tokens."),
                        attrs: &[],
                        deprecation: None,
                    },
                    decl: erised::types::FnDecl {
                        inputs: &[erised::types::FnInput {
                            pat: "tokens",
                            ty: erised::types::Type::ImplTrait(&[
                                erised::types::GenericBound::TraitBound {
                                    trait_: erised::types::Path {
                                        name: "ToTokens",
                                        prefix: "",
                                        target: erised::types::Identifiable::Summary(|| {
                                            erised::types::ItemSummary {
                                                krate: || erised::types::ExternalCrate {
                                                    name: "quote",
                                                    html_root_url: Some(
                                                        "https://docs.rs/quote/1.0.26/",
                                                    ),
                                                },
                                                path: &["quote", "to_tokens", "ToTokens"],
                                                kind: erised::types::ItemKind::Trait,
                                            }
                                        }),
                                        args: Some(|| erised::types::GenericArgs::AngleBracketed {
                                            args: &[],
                                            bindings: &[],
                                        }),
                                    },
                                    generic_params: &[],
                                    modifier: erised::types::TraitBoundModifier::None,
                                },
                            ]),
                        }],
                        output: Some(erised::types::Type::ResolvedPath(erised::types::Path {
                            name: "Result",
                            prefix: "",
                            target: erised::types::Identifiable::Summary(|| {
                                erised::types::ItemSummary {
                                    krate: || erised::types::ExternalCrate {
                                        name: "core",
                                        html_root_url: Some("https://doc.rust-lang.org/nightly/"),
                                    },
                                    path: &["core", "result", "Result"],
                                    kind: erised::types::ItemKind::Enum,
                                }
                            }),
                            args: Some(|| erised::types::GenericArgs::AngleBracketed {
                                args: &[
                                    erised::types::GenericArg::Type(
                                        erised::types::Type::ResolvedPath(erised::types::Path {
                                            name: "String",
                                            prefix: "",
                                            target: erised::types::Identifiable::Summary(|| {
                                                erised::types::ItemSummary {
                                                    krate: || erised::types::ExternalCrate {
                                                        name: "alloc",
                                                        html_root_url: Some(
                                                            "https://doc.rust-lang.org/nightly/",
                                                        ),
                                                    },
                                                    path: &["alloc", "string", "String"],
                                                    kind: erised::types::ItemKind::Struct,
                                                }
                                            }),
                                            args: Some(|| {
                                                erised::types::GenericArgs::AngleBracketed {
                                                    args: &[],
                                                    bindings: &[],
                                                }
                                            }),
                                        }),
                                    ),
                                    erised::types::GenericArg::Type(
                                        erised::types::Type::ResolvedPath(erised::types::Path {
                                            name: "Error",
                                            prefix: "",
                                            target: erised::types::Identifiable::Summary(|| {
                                                erised::types::ItemSummary {
                                                    krate: || erised::types::ExternalCrate {
                                                        name: "anyhow",
                                                        html_root_url: Some(
                                                            "https://docs.rs/anyhow/1.0.70/",
                                                        ),
                                                    },
                                                    path: &["anyhow", "Error"],
                                                    kind: erised::types::ItemKind::Struct,
                                                }
                                            }),
                                            args: Some(|| {
                                                erised::types::GenericArgs::AngleBracketed {
                                                    args: &[],
                                                    bindings: &[],
                                                }
                                            }),
                                        }),
                                    ),
                                ],
                                bindings: &[],
                            }),
                        })),
                        c_variadic: false,
                    },
                    generics: erised::types::Generics {
                        params: &[erised::types::GenericParamDef {
                            name: "impl ToTokens",
                            kind: erised::types::GenericParamDefKind::Type {
                                bounds: &[erised::types::GenericBound::TraitBound {
                                    trait_: erised::types::Path {
                                        name: "ToTokens",
                                        prefix: "",
                                        target: erised::types::Identifiable::Summary(|| {
                                            erised::types::ItemSummary {
                                                krate: || erised::types::ExternalCrate {
                                                    name: "quote",
                                                    html_root_url: Some(
                                                        "https://docs.rs/quote/1.0.26/",
                                                    ),
                                                },
                                                path: &["quote", "to_tokens", "ToTokens"],
                                                kind: erised::types::ItemKind::Trait,
                                            }
                                        }),
                                        args: Some(|| erised::types::GenericArgs::AngleBracketed {
                                            args: &[],
                                            bindings: &[],
                                        }),
                                    },
                                    generic_params: &[],
                                    modifier: erised::types::TraitBoundModifier::None,
                                }],
                                default: None,
                                synthetic: true,
                            },
                        }],
                        where_predicates: &[],
                    },
                    header: erised::types::Header {
                        const_: false,
                        unsafe_: false,
                        async_: false,
                        abi: erised::types::Abi::Rust,
                    },
                    has_body: true,
                })
            }),
            erised::types::Identifiable::Item(|| <crate::MyStruct as Reflect>::TYPE_INFO),
            erised::types::Identifiable::Item(|| <crate::MyEnum as Reflect>::TYPE_INFO),
            erised::types::Identifiable::Item(|| {
                erised::types::Item::Import(erised::types::Import {
                    meta: erised::types::ItemMeta {
                        krate: || erised::types::ExternalCrate {
                            name: "erised_tests",
                            html_root_url: None,
                        },
                        summary: None,
                        span: Some(erised::types::Span {
                            filename: "src/lib.rs",
                            begin: (8usize, 0usize),
                            end: (8usize, 30usize),
                        }),
                        visibility: erised::types::Visibility::Public,
                        docs: None,
                        attrs: &[],
                        deprecation: None,
                    },
                    source: "erised::heap_types",
                    name: "heap_types",
                    target: Some(erised::types::Identifiable::Summary(|| {
                        erised::types::ItemSummary {
                            krate: || erised::types::ExternalCrate {
                                name: "erised",
                                html_root_url: None,
                            },
                            path: &["erised", "heap_types"],
                            kind: erised::types::ItemKind::Module,
                        }
                    })),
                    glob: true,
                })
            }),
        ],
        is_stripped: false,
    },
    crate_version: Some("0.1.0"),
    all_items: &[
        || {
            erised::types::Item::Module(erised::types::Module {
                name: "erised_tests",
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
                        path: &["crate"],
                        kind: erised::types::ItemKind::Module,
                    }),
                    span: Some(erised::types::Span {
                        filename: "src/lib.rs",
                        begin: (1usize, 0usize),
                        end: (73usize, 1usize),
                    }),
                    visibility: erised::types::Visibility::Public,
                    docs: None,
                    attrs: &[],
                    deprecation: None,
                },
                is_crate: true,
                items: &[
                    erised::types::Identifiable::Item(|| {
                        erised::types::Item::Function(erised::types::Function {
                            name: "pretty_print_item",
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
                                    path: &["crate", "pretty_print_item"],
                                    kind: erised::types::ItemKind::Function,
                                }),
                                span: Some(erised::types::Span {
                                    filename: "src/lib.rs",
                                    begin: (10usize, 0usize),
                                    end: (16usize, 1usize),
                                }),
                                visibility: erised::types::Visibility::Public,
                                docs: None,
                                attrs: &[],
                                deprecation: None,
                            },
                            decl: erised::types::FnDecl {
                                inputs: &[erised::types::FnInput {
                                    pat: "tokens",
                                    ty: erised::types::Type::ImplTrait(&[
                                        erised::types::GenericBound::TraitBound {
                                            trait_: erised::types::Path {
                                                name: "ToTokens",
                                                prefix: "",
                                                target: erised::types::Identifiable::Summary(
                                                    || erised::types::ItemSummary {
                                                        krate: || erised::types::ExternalCrate {
                                                            name: "quote",
                                                            html_root_url: Some(
                                                                "https://docs.rs/quote/1.0.26/",
                                                            ),
                                                        },
                                                        path: &["quote", "to_tokens", "ToTokens"],
                                                        kind: erised::types::ItemKind::Trait,
                                                    },
                                                ),
                                                args: Some(|| {
                                                    erised::types::GenericArgs::AngleBracketed {
                                                        args: &[],
                                                        bindings: &[],
                                                    }
                                                }),
                                            },
                                            generic_params: &[],
                                            modifier: erised::types::TraitBoundModifier::None,
                                        },
                                    ]),
                                }],
                                output: Some(erised::types::Type::ResolvedPath(
                                    erised::types::Path {
                                        name: "Result",
                                        prefix: "",
                                        target: erised::types::Identifiable::Summary(|| {
                                            erised::types::ItemSummary {
                                                krate: || erised::types::ExternalCrate {
                                                    name: "core",
                                                    html_root_url: Some(
                                                        "https://doc.rust-lang.org/nightly/",
                                                    ),
                                                },
                                                path: &["core", "result", "Result"],
                                                kind: erised::types::ItemKind::Enum,
                                            }
                                        }),
                                        args: Some(|| erised::types::GenericArgs::AngleBracketed {
                                            args: &[
                                                erised::types::GenericArg::Type(
                                                    erised::types::Type::ResolvedPath(
                                                        erised::types::Path {
                                                            name: "String",
                                                            prefix: "",
                                                            target:
                                                                erised::types::Identifiable::Summary(
                                                                    || {
                                                                        erised :: types :: ItemSummary { krate : || erised :: types :: ExternalCrate { name : "alloc" , html_root_url : Some ("https://doc.rust-lang.org/nightly/") } , path : & ["alloc" , "string" , "String"] , kind : erised :: types :: ItemKind :: Struct }
                                                                    },
                                                                ),
                                                            args: Some(|| {
                                                                erised :: types :: GenericArgs :: AngleBracketed { args : & [] , bindings : & [] }
                                                            }),
                                                        },
                                                    ),
                                                ),
                                                erised::types::GenericArg::Type(
                                                    erised::types::Type::ResolvedPath(
                                                        erised::types::Path {
                                                            name: "Error",
                                                            prefix: "",
                                                            target:
                                                                erised::types::Identifiable::Summary(
                                                                    || {
                                                                        erised :: types :: ItemSummary { krate : || erised :: types :: ExternalCrate { name : "anyhow" , html_root_url : Some ("https://docs.rs/anyhow/1.0.70/") } , path : & ["anyhow" , "Error"] , kind : erised :: types :: ItemKind :: Struct }
                                                                    },
                                                                ),
                                                            args: Some(|| {
                                                                erised :: types :: GenericArgs :: AngleBracketed { args : & [] , bindings : & [] }
                                                            }),
                                                        },
                                                    ),
                                                ),
                                            ],
                                            bindings: &[],
                                        }),
                                    },
                                )),
                                c_variadic: false,
                            },
                            generics: erised::types::Generics {
                                params: &[erised::types::GenericParamDef {
                                    name: "impl ToTokens",
                                    kind: erised::types::GenericParamDefKind::Type {
                                        bounds: &[erised::types::GenericBound::TraitBound {
                                            trait_: erised::types::Path {
                                                name: "ToTokens",
                                                prefix: "",
                                                target: erised::types::Identifiable::Summary(
                                                    || erised::types::ItemSummary {
                                                        krate: || erised::types::ExternalCrate {
                                                            name: "quote",
                                                            html_root_url: Some(
                                                                "https://docs.rs/quote/1.0.26/",
                                                            ),
                                                        },
                                                        path: &["quote", "to_tokens", "ToTokens"],
                                                        kind: erised::types::ItemKind::Trait,
                                                    },
                                                ),
                                                args: Some(|| {
                                                    erised::types::GenericArgs::AngleBracketed {
                                                        args: &[],
                                                        bindings: &[],
                                                    }
                                                }),
                                            },
                                            generic_params: &[],
                                            modifier: erised::types::TraitBoundModifier::None,
                                        }],
                                        default: None,
                                        synthetic: true,
                                    },
                                }],
                                where_predicates: &[],
                            },
                            header: erised::types::Header {
                                const_: false,
                                unsafe_: false,
                                async_: false,
                                abi: erised::types::Abi::Rust,
                            },
                            has_body: true,
                        })
                    }),
                    erised::types::Identifiable::Item(|| {
                        erised::types::Item::Function(erised::types::Function {
                            name: "pretty_print",
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
                                    path: &["crate", "pretty_print"],
                                    kind: erised::types::ItemKind::Function,
                                }),
                                span: Some(erised::types::Span {
                                    filename: "src/lib.rs",
                                    begin: (19usize, 0usize),
                                    end: (55usize, 1usize),
                                }),
                                visibility: erised::types::Visibility::Public,
                                docs: Some("Use `rustfmt` to pretty-print the tokens."),
                                attrs: &[],
                                deprecation: None,
                            },
                            decl: erised::types::FnDecl {
                                inputs: &[erised::types::FnInput {
                                    pat: "tokens",
                                    ty: erised::types::Type::ImplTrait(&[
                                        erised::types::GenericBound::TraitBound {
                                            trait_: erised::types::Path {
                                                name: "ToTokens",
                                                prefix: "",
                                                target: erised::types::Identifiable::Summary(
                                                    || erised::types::ItemSummary {
                                                        krate: || erised::types::ExternalCrate {
                                                            name: "quote",
                                                            html_root_url: Some(
                                                                "https://docs.rs/quote/1.0.26/",
                                                            ),
                                                        },
                                                        path: &["quote", "to_tokens", "ToTokens"],
                                                        kind: erised::types::ItemKind::Trait,
                                                    },
                                                ),
                                                args: Some(|| {
                                                    erised::types::GenericArgs::AngleBracketed {
                                                        args: &[],
                                                        bindings: &[],
                                                    }
                                                }),
                                            },
                                            generic_params: &[],
                                            modifier: erised::types::TraitBoundModifier::None,
                                        },
                                    ]),
                                }],
                                output: Some(erised::types::Type::ResolvedPath(
                                    erised::types::Path {
                                        name: "Result",
                                        prefix: "",
                                        target: erised::types::Identifiable::Summary(|| {
                                            erised::types::ItemSummary {
                                                krate: || erised::types::ExternalCrate {
                                                    name: "core",
                                                    html_root_url: Some(
                                                        "https://doc.rust-lang.org/nightly/",
                                                    ),
                                                },
                                                path: &["core", "result", "Result"],
                                                kind: erised::types::ItemKind::Enum,
                                            }
                                        }),
                                        args: Some(|| erised::types::GenericArgs::AngleBracketed {
                                            args: &[
                                                erised::types::GenericArg::Type(
                                                    erised::types::Type::ResolvedPath(
                                                        erised::types::Path {
                                                            name: "String",
                                                            prefix: "",
                                                            target:
                                                                erised::types::Identifiable::Summary(
                                                                    || {
                                                                        erised :: types :: ItemSummary { krate : || erised :: types :: ExternalCrate { name : "alloc" , html_root_url : Some ("https://doc.rust-lang.org/nightly/") } , path : & ["alloc" , "string" , "String"] , kind : erised :: types :: ItemKind :: Struct }
                                                                    },
                                                                ),
                                                            args: Some(|| {
                                                                erised :: types :: GenericArgs :: AngleBracketed { args : & [] , bindings : & [] }
                                                            }),
                                                        },
                                                    ),
                                                ),
                                                erised::types::GenericArg::Type(
                                                    erised::types::Type::ResolvedPath(
                                                        erised::types::Path {
                                                            name: "Error",
                                                            prefix: "",
                                                            target:
                                                                erised::types::Identifiable::Summary(
                                                                    || {
                                                                        erised :: types :: ItemSummary { krate : || erised :: types :: ExternalCrate { name : "anyhow" , html_root_url : Some ("https://docs.rs/anyhow/1.0.70/") } , path : & ["anyhow" , "Error"] , kind : erised :: types :: ItemKind :: Struct }
                                                                    },
                                                                ),
                                                            args: Some(|| {
                                                                erised :: types :: GenericArgs :: AngleBracketed { args : & [] , bindings : & [] }
                                                            }),
                                                        },
                                                    ),
                                                ),
                                            ],
                                            bindings: &[],
                                        }),
                                    },
                                )),
                                c_variadic: false,
                            },
                            generics: erised::types::Generics {
                                params: &[erised::types::GenericParamDef {
                                    name: "impl ToTokens",
                                    kind: erised::types::GenericParamDefKind::Type {
                                        bounds: &[erised::types::GenericBound::TraitBound {
                                            trait_: erised::types::Path {
                                                name: "ToTokens",
                                                prefix: "",
                                                target: erised::types::Identifiable::Summary(
                                                    || erised::types::ItemSummary {
                                                        krate: || erised::types::ExternalCrate {
                                                            name: "quote",
                                                            html_root_url: Some(
                                                                "https://docs.rs/quote/1.0.26/",
                                                            ),
                                                        },
                                                        path: &["quote", "to_tokens", "ToTokens"],
                                                        kind: erised::types::ItemKind::Trait,
                                                    },
                                                ),
                                                args: Some(|| {
                                                    erised::types::GenericArgs::AngleBracketed {
                                                        args: &[],
                                                        bindings: &[],
                                                    }
                                                }),
                                            },
                                            generic_params: &[],
                                            modifier: erised::types::TraitBoundModifier::None,
                                        }],
                                        default: None,
                                        synthetic: true,
                                    },
                                }],
                                where_predicates: &[],
                            },
                            header: erised::types::Header {
                                const_: false,
                                unsafe_: false,
                                async_: false,
                                abi: erised::types::Abi::Rust,
                            },
                            has_body: true,
                        })
                    }),
                    erised::types::Identifiable::Item(|| <crate::MyStruct as Reflect>::TYPE_INFO),
                    erised::types::Identifiable::Item(|| <crate::MyEnum as Reflect>::TYPE_INFO),
                    erised::types::Identifiable::Item(|| {
                        erised::types::Item::Import(erised::types::Import {
                            meta: erised::types::ItemMeta {
                                krate: || erised::types::ExternalCrate {
                                    name: "erised_tests",
                                    html_root_url: None,
                                },
                                summary: None,
                                span: Some(erised::types::Span {
                                    filename: "src/lib.rs",
                                    begin: (8usize, 0usize),
                                    end: (8usize, 30usize),
                                }),
                                visibility: erised::types::Visibility::Public,
                                docs: None,
                                attrs: &[],
                                deprecation: None,
                            },
                            source: "erised::heap_types",
                            name: "heap_types",
                            target: Some(erised::types::Identifiable::Summary(|| {
                                erised::types::ItemSummary {
                                    krate: || erised::types::ExternalCrate {
                                        name: "erised",
                                        html_root_url: None,
                                    },
                                    path: &["erised", "heap_types"],
                                    kind: erised::types::ItemKind::Module,
                                }
                            })),
                            glob: true,
                        })
                    }),
                ],
                is_stripped: false,
            })
        },
        || {
            erised::types::Item::Import(erised::types::Import {
                meta: erised::types::ItemMeta {
                    krate: || erised::types::ExternalCrate {
                        name: "erised_tests",
                        html_root_url: None,
                    },
                    summary: None,
                    span: Some(erised::types::Span {
                        filename: "src/lib.rs",
                        begin: (8usize, 0usize),
                        end: (8usize, 30usize),
                    }),
                    visibility: erised::types::Visibility::Public,
                    docs: None,
                    attrs: &[],
                    deprecation: None,
                },
                source: "erised::heap_types",
                name: "heap_types",
                target: Some(erised::types::Identifiable::Summary(|| {
                    erised::types::ItemSummary {
                        krate: || erised::types::ExternalCrate {
                            name: "erised",
                            html_root_url: None,
                        },
                        path: &["erised", "heap_types"],
                        kind: erised::types::ItemKind::Module,
                    }
                })),
                glob: true,
            })
        },
        || {
            erised::types::Item::Function(erised::types::Function {
                name: "pretty_print_item",
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
                        path: &["crate", "pretty_print_item"],
                        kind: erised::types::ItemKind::Function,
                    }),
                    span: Some(erised::types::Span {
                        filename: "src/lib.rs",
                        begin: (10usize, 0usize),
                        end: (16usize, 1usize),
                    }),
                    visibility: erised::types::Visibility::Public,
                    docs: None,
                    attrs: &[],
                    deprecation: None,
                },
                decl: erised::types::FnDecl {
                    inputs: &[erised::types::FnInput {
                        pat: "tokens",
                        ty: erised::types::Type::ImplTrait(&[
                            erised::types::GenericBound::TraitBound {
                                trait_: erised::types::Path {
                                    name: "ToTokens",
                                    prefix: "",
                                    target: erised::types::Identifiable::Summary(|| {
                                        erised::types::ItemSummary {
                                            krate: || erised::types::ExternalCrate {
                                                name: "quote",
                                                html_root_url: Some(
                                                    "https://docs.rs/quote/1.0.26/",
                                                ),
                                            },
                                            path: &["quote", "to_tokens", "ToTokens"],
                                            kind: erised::types::ItemKind::Trait,
                                        }
                                    }),
                                    args: Some(|| erised::types::GenericArgs::AngleBracketed {
                                        args: &[],
                                        bindings: &[],
                                    }),
                                },
                                generic_params: &[],
                                modifier: erised::types::TraitBoundModifier::None,
                            },
                        ]),
                    }],
                    output: Some(erised::types::Type::ResolvedPath(erised::types::Path {
                        name: "Result",
                        prefix: "",
                        target: erised::types::Identifiable::Summary(|| {
                            erised::types::ItemSummary {
                                krate: || erised::types::ExternalCrate {
                                    name: "core",
                                    html_root_url: Some("https://doc.rust-lang.org/nightly/"),
                                },
                                path: &["core", "result", "Result"],
                                kind: erised::types::ItemKind::Enum,
                            }
                        }),
                        args: Some(|| erised::types::GenericArgs::AngleBracketed {
                            args: &[
                                erised::types::GenericArg::Type(erised::types::Type::ResolvedPath(
                                    erised::types::Path {
                                        name: "String",
                                        prefix: "",
                                        target: erised::types::Identifiable::Summary(|| {
                                            erised::types::ItemSummary {
                                                krate: || erised::types::ExternalCrate {
                                                    name: "alloc",
                                                    html_root_url: Some(
                                                        "https://doc.rust-lang.org/nightly/",
                                                    ),
                                                },
                                                path: &["alloc", "string", "String"],
                                                kind: erised::types::ItemKind::Struct,
                                            }
                                        }),
                                        args: Some(|| erised::types::GenericArgs::AngleBracketed {
                                            args: &[],
                                            bindings: &[],
                                        }),
                                    },
                                )),
                                erised::types::GenericArg::Type(erised::types::Type::ResolvedPath(
                                    erised::types::Path {
                                        name: "Error",
                                        prefix: "",
                                        target: erised::types::Identifiable::Summary(|| {
                                            erised::types::ItemSummary {
                                                krate: || erised::types::ExternalCrate {
                                                    name: "anyhow",
                                                    html_root_url: Some(
                                                        "https://docs.rs/anyhow/1.0.70/",
                                                    ),
                                                },
                                                path: &["anyhow", "Error"],
                                                kind: erised::types::ItemKind::Struct,
                                            }
                                        }),
                                        args: Some(|| erised::types::GenericArgs::AngleBracketed {
                                            args: &[],
                                            bindings: &[],
                                        }),
                                    },
                                )),
                            ],
                            bindings: &[],
                        }),
                    })),
                    c_variadic: false,
                },
                generics: erised::types::Generics {
                    params: &[erised::types::GenericParamDef {
                        name: "impl ToTokens",
                        kind: erised::types::GenericParamDefKind::Type {
                            bounds: &[erised::types::GenericBound::TraitBound {
                                trait_: erised::types::Path {
                                    name: "ToTokens",
                                    prefix: "",
                                    target: erised::types::Identifiable::Summary(|| {
                                        erised::types::ItemSummary {
                                            krate: || erised::types::ExternalCrate {
                                                name: "quote",
                                                html_root_url: Some(
                                                    "https://docs.rs/quote/1.0.26/",
                                                ),
                                            },
                                            path: &["quote", "to_tokens", "ToTokens"],
                                            kind: erised::types::ItemKind::Trait,
                                        }
                                    }),
                                    args: Some(|| erised::types::GenericArgs::AngleBracketed {
                                        args: &[],
                                        bindings: &[],
                                    }),
                                },
                                generic_params: &[],
                                modifier: erised::types::TraitBoundModifier::None,
                            }],
                            default: None,
                            synthetic: true,
                        },
                    }],
                    where_predicates: &[],
                },
                header: erised::types::Header {
                    const_: false,
                    unsafe_: false,
                    async_: false,
                    abi: erised::types::Abi::Rust,
                },
                has_body: true,
            })
        },
        || {
            erised::types::Item::Function(erised::types::Function {
                name: "pretty_print",
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
                        path: &["crate", "pretty_print"],
                        kind: erised::types::ItemKind::Function,
                    }),
                    span: Some(erised::types::Span {
                        filename: "src/lib.rs",
                        begin: (19usize, 0usize),
                        end: (55usize, 1usize),
                    }),
                    visibility: erised::types::Visibility::Public,
                    docs: Some("Use `rustfmt` to pretty-print the tokens."),
                    attrs: &[],
                    deprecation: None,
                },
                decl: erised::types::FnDecl {
                    inputs: &[erised::types::FnInput {
                        pat: "tokens",
                        ty: erised::types::Type::ImplTrait(&[
                            erised::types::GenericBound::TraitBound {
                                trait_: erised::types::Path {
                                    name: "ToTokens",
                                    prefix: "",
                                    target: erised::types::Identifiable::Summary(|| {
                                        erised::types::ItemSummary {
                                            krate: || erised::types::ExternalCrate {
                                                name: "quote",
                                                html_root_url: Some(
                                                    "https://docs.rs/quote/1.0.26/",
                                                ),
                                            },
                                            path: &["quote", "to_tokens", "ToTokens"],
                                            kind: erised::types::ItemKind::Trait,
                                        }
                                    }),
                                    args: Some(|| erised::types::GenericArgs::AngleBracketed {
                                        args: &[],
                                        bindings: &[],
                                    }),
                                },
                                generic_params: &[],
                                modifier: erised::types::TraitBoundModifier::None,
                            },
                        ]),
                    }],
                    output: Some(erised::types::Type::ResolvedPath(erised::types::Path {
                        name: "Result",
                        prefix: "",
                        target: erised::types::Identifiable::Summary(|| {
                            erised::types::ItemSummary {
                                krate: || erised::types::ExternalCrate {
                                    name: "core",
                                    html_root_url: Some("https://doc.rust-lang.org/nightly/"),
                                },
                                path: &["core", "result", "Result"],
                                kind: erised::types::ItemKind::Enum,
                            }
                        }),
                        args: Some(|| erised::types::GenericArgs::AngleBracketed {
                            args: &[
                                erised::types::GenericArg::Type(erised::types::Type::ResolvedPath(
                                    erised::types::Path {
                                        name: "String",
                                        prefix: "",
                                        target: erised::types::Identifiable::Summary(|| {
                                            erised::types::ItemSummary {
                                                krate: || erised::types::ExternalCrate {
                                                    name: "alloc",
                                                    html_root_url: Some(
                                                        "https://doc.rust-lang.org/nightly/",
                                                    ),
                                                },
                                                path: &["alloc", "string", "String"],
                                                kind: erised::types::ItemKind::Struct,
                                            }
                                        }),
                                        args: Some(|| erised::types::GenericArgs::AngleBracketed {
                                            args: &[],
                                            bindings: &[],
                                        }),
                                    },
                                )),
                                erised::types::GenericArg::Type(erised::types::Type::ResolvedPath(
                                    erised::types::Path {
                                        name: "Error",
                                        prefix: "",
                                        target: erised::types::Identifiable::Summary(|| {
                                            erised::types::ItemSummary {
                                                krate: || erised::types::ExternalCrate {
                                                    name: "anyhow",
                                                    html_root_url: Some(
                                                        "https://docs.rs/anyhow/1.0.70/",
                                                    ),
                                                },
                                                path: &["anyhow", "Error"],
                                                kind: erised::types::ItemKind::Struct,
                                            }
                                        }),
                                        args: Some(|| erised::types::GenericArgs::AngleBracketed {
                                            args: &[],
                                            bindings: &[],
                                        }),
                                    },
                                )),
                            ],
                            bindings: &[],
                        }),
                    })),
                    c_variadic: false,
                },
                generics: erised::types::Generics {
                    params: &[erised::types::GenericParamDef {
                        name: "impl ToTokens",
                        kind: erised::types::GenericParamDefKind::Type {
                            bounds: &[erised::types::GenericBound::TraitBound {
                                trait_: erised::types::Path {
                                    name: "ToTokens",
                                    prefix: "",
                                    target: erised::types::Identifiable::Summary(|| {
                                        erised::types::ItemSummary {
                                            krate: || erised::types::ExternalCrate {
                                                name: "quote",
                                                html_root_url: Some(
                                                    "https://docs.rs/quote/1.0.26/",
                                                ),
                                            },
                                            path: &["quote", "to_tokens", "ToTokens"],
                                            kind: erised::types::ItemKind::Trait,
                                        }
                                    }),
                                    args: Some(|| erised::types::GenericArgs::AngleBracketed {
                                        args: &[],
                                        bindings: &[],
                                    }),
                                },
                                generic_params: &[],
                                modifier: erised::types::TraitBoundModifier::None,
                            }],
                            default: None,
                            synthetic: true,
                        },
                    }],
                    where_predicates: &[],
                },
                header: erised::types::Header {
                    const_: false,
                    unsafe_: false,
                    async_: false,
                    abi: erised::types::Abi::Rust,
                },
                has_body: true,
            })
        },
        || <crate::MyStruct as Reflect>::TYPE_INFO,
        || <crate::MyEnum as Reflect>::TYPE_INFO,
        || {
            erised::types::Item::Function(erised::types::Function {
                name: "borrow",
                meta: erised::types::ItemMeta {
                    krate: || erised::types::ExternalCrate {
                        name: "core",
                        html_root_url: Some("https://doc.rust-lang.org/nightly/"),
                    },
                    summary: None,
                    span: None,
                    visibility: erised::types::Visibility::Default,
                    docs: None,
                    attrs: &["#[rustc_diagnostic_item = \"noop_method_borrow\"]"],
                    deprecation: None,
                },
                decl: erised::types::FnDecl {
                    inputs: &[erised::types::FnInput {
                        pat: "self",
                        ty: erised::types::Type::BorrowedRef {
                            lifetime: None,
                            mutable: false,
                            type_: || erised::types::Type::Generic("Self"),
                        },
                    }],
                    output: Some(erised::types::Type::BorrowedRef {
                        lifetime: None,
                        mutable: false,
                        type_: || erised::types::Type::Generic("T"),
                    }),
                    c_variadic: false,
                },
                generics: erised::types::Generics {
                    params: &[],
                    where_predicates: &[],
                },
                header: erised::types::Header {
                    const_: false,
                    unsafe_: false,
                    async_: false,
                    abi: erised::types::Abi::Rust,
                },
                has_body: true,
            })
        },
        || {
            erised::types::Item::Function(erised::types::Function {
                name: "borrow_mut",
                meta: erised::types::ItemMeta {
                    krate: || erised::types::ExternalCrate {
                        name: "core",
                        html_root_url: Some("https://doc.rust-lang.org/nightly/"),
                    },
                    summary: None,
                    span: None,
                    visibility: erised::types::Visibility::Default,
                    docs: None,
                    attrs: &[],
                    deprecation: None,
                },
                decl: erised::types::FnDecl {
                    inputs: &[erised::types::FnInput {
                        pat: "self",
                        ty: erised::types::Type::BorrowedRef {
                            lifetime: None,
                            mutable: true,
                            type_: || erised::types::Type::Generic("Self"),
                        },
                    }],
                    output: Some(erised::types::Type::BorrowedRef {
                        lifetime: None,
                        mutable: true,
                        type_: || erised::types::Type::Generic("T"),
                    }),
                    c_variadic: false,
                },
                generics: erised::types::Generics {
                    params: &[],
                    where_predicates: &[],
                },
                header: erised::types::Header {
                    const_: false,
                    unsafe_: false,
                    async_: false,
                    abi: erised::types::Abi::Rust,
                },
                has_body: true,
            })
        },
        || {
            erised :: types :: Item :: Function (erised :: types :: Function { name : "into" , meta : erised :: types :: ItemMeta { krate : || erised :: types :: ExternalCrate { name : "core" , html_root_url : Some ("https://doc.rust-lang.org/nightly/") } , summary : None , span : None , visibility : erised :: types :: Visibility :: Default , docs : Some ("Calls `U::from(self)`.\n\nThat is, this conversion is whatever the implementation of\n<code>[From]&lt;T&gt; for U</code> chooses to do.") , attrs : & [] , deprecation : None } , decl : erised :: types :: FnDecl { inputs : & [erised :: types :: FnInput { pat : "self" , ty : erised :: types :: Type :: Generic ("Self") }] , output : Some (erised :: types :: Type :: Generic ("U")) , c_variadic : false } , generics : erised :: types :: Generics { params : & [] , where_predicates : & [] } , header : erised :: types :: Header { const_ : false , unsafe_ : false , async_ : false , abi : erised :: types :: Abi :: Rust } , has_body : true })
        },
        || {
            erised::types::Item::Function(erised::types::Function {
                name: "from",
                meta: erised::types::ItemMeta {
                    krate: || erised::types::ExternalCrate {
                        name: "core",
                        html_root_url: Some("https://doc.rust-lang.org/nightly/"),
                    },
                    summary: None,
                    span: None,
                    visibility: erised::types::Visibility::Default,
                    docs: Some("Returns the argument unchanged."),
                    attrs: &[],
                    deprecation: None,
                },
                decl: erised::types::FnDecl {
                    inputs: &[erised::types::FnInput {
                        pat: "t",
                        ty: erised::types::Type::Generic("T"),
                    }],
                    output: Some(erised::types::Type::Generic("T")),
                    c_variadic: false,
                },
                generics: erised::types::Generics {
                    params: &[],
                    where_predicates: &[],
                },
                header: erised::types::Header {
                    const_: false,
                    unsafe_: false,
                    async_: false,
                    abi: erised::types::Abi::Rust,
                },
                has_body: true,
            })
        },
        || erised::types::Item::AssocType {
            meta: erised::types::ItemMeta {
                krate: || erised::types::ExternalCrate {
                    name: "core",
                    html_root_url: Some("https://doc.rust-lang.org/nightly/"),
                },
                summary: None,
                span: None,
                visibility: erised::types::Visibility::Default,
                docs: None,
                attrs: &[],
                deprecation: None,
            },
            generics: erised::types::Generics {
                params: &[],
                where_predicates: &[],
            },
            bounds: &[],
            default: Some(erised::types::Type::QualifiedPath {
                name: "Error",
                args: || erised::types::GenericArgs::AngleBracketed {
                    args: &[],
                    bindings: &[],
                },
                self_type: || erised::types::Type::Generic("U"),
                trait_: erised::types::Path {
                    name: "TryFrom",
                    prefix: "",
                    target: erised::types::Identifiable::Summary(|| erised::types::ItemSummary {
                        krate: || erised::types::ExternalCrate {
                            name: "core",
                            html_root_url: Some("https://doc.rust-lang.org/nightly/"),
                        },
                        path: &["core", "convert", "TryFrom"],
                        kind: erised::types::ItemKind::Trait,
                    }),
                    args: Some(|| erised::types::GenericArgs::AngleBracketed {
                        args: &[erised::types::GenericArg::Type(
                            erised::types::Type::Generic("T"),
                        )],
                        bindings: &[],
                    }),
                },
            }),
        },
        || {
            erised::types::Item::Function(erised::types::Function {
                name: "try_into",
                meta: erised::types::ItemMeta {
                    krate: || erised::types::ExternalCrate {
                        name: "core",
                        html_root_url: Some("https://doc.rust-lang.org/nightly/"),
                    },
                    summary: None,
                    span: None,
                    visibility: erised::types::Visibility::Default,
                    docs: None,
                    attrs: &[],
                    deprecation: None,
                },
                decl: erised::types::FnDecl {
                    inputs: &[erised::types::FnInput {
                        pat: "self",
                        ty: erised::types::Type::Generic("Self"),
                    }],
                    output: Some(erised::types::Type::ResolvedPath(erised::types::Path {
                        name: "Result",
                        prefix: "",
                        target: erised::types::Identifiable::Summary(|| {
                            erised::types::ItemSummary {
                                krate: || erised::types::ExternalCrate {
                                    name: "core",
                                    html_root_url: Some("https://doc.rust-lang.org/nightly/"),
                                },
                                path: &["core", "result", "Result"],
                                kind: erised::types::ItemKind::Enum,
                            }
                        }),
                        args: Some(|| erised::types::GenericArgs::AngleBracketed {
                            args: &[
                                erised::types::GenericArg::Type(erised::types::Type::Generic("U")),
                                erised::types::GenericArg::Type(
                                    erised::types::Type::QualifiedPath {
                                        name: "Error",
                                        args: || erised::types::GenericArgs::AngleBracketed {
                                            args: &[],
                                            bindings: &[],
                                        },
                                        self_type: || erised::types::Type::Generic("U"),
                                        trait_: erised::types::Path {
                                            name: "TryFrom",
                                            prefix: "",
                                            target: erised::types::Identifiable::Summary(|| {
                                                erised::types::ItemSummary {
                                                    krate: || erised::types::ExternalCrate {
                                                        name: "core",
                                                        html_root_url: Some(
                                                            "https://doc.rust-lang.org/nightly/",
                                                        ),
                                                    },
                                                    path: &["core", "convert", "TryFrom"],
                                                    kind: erised::types::ItemKind::Trait,
                                                }
                                            }),
                                            args: Some(|| {
                                                erised::types::GenericArgs::AngleBracketed {
                                                    args: &[erised::types::GenericArg::Type(
                                                        erised::types::Type::Generic("T"),
                                                    )],
                                                    bindings: &[],
                                                }
                                            }),
                                        },
                                    },
                                ),
                            ],
                            bindings: &[],
                        }),
                    })),
                    c_variadic: false,
                },
                generics: erised::types::Generics {
                    params: &[],
                    where_predicates: &[],
                },
                header: erised::types::Header {
                    const_: false,
                    unsafe_: false,
                    async_: false,
                    abi: erised::types::Abi::Rust,
                },
                has_body: true,
            })
        },
        || erised::types::Item::AssocType {
            meta: erised::types::ItemMeta {
                krate: || erised::types::ExternalCrate {
                    name: "core",
                    html_root_url: Some("https://doc.rust-lang.org/nightly/"),
                },
                summary: None,
                span: None,
                visibility: erised::types::Visibility::Default,
                docs: None,
                attrs: &[],
                deprecation: None,
            },
            generics: erised::types::Generics {
                params: &[],
                where_predicates: &[],
            },
            bounds: &[],
            default: Some(erised::types::Type::ResolvedPath(erised::types::Path {
                name: "Infallible",
                prefix: "",
                target: erised::types::Identifiable::Summary(|| erised::types::ItemSummary {
                    krate: || erised::types::ExternalCrate {
                        name: "core",
                        html_root_url: Some("https://doc.rust-lang.org/nightly/"),
                    },
                    path: &["core", "convert", "Infallible"],
                    kind: erised::types::ItemKind::Enum,
                }),
                args: Some(|| erised::types::GenericArgs::AngleBracketed {
                    args: &[],
                    bindings: &[],
                }),
            })),
        },
        || {
            erised::types::Item::Function(erised::types::Function {
                name: "try_from",
                meta: erised::types::ItemMeta {
                    krate: || erised::types::ExternalCrate {
                        name: "core",
                        html_root_url: Some("https://doc.rust-lang.org/nightly/"),
                    },
                    summary: None,
                    span: None,
                    visibility: erised::types::Visibility::Default,
                    docs: None,
                    attrs: &[],
                    deprecation: None,
                },
                decl: erised::types::FnDecl {
                    inputs: &[erised::types::FnInput {
                        pat: "value",
                        ty: erised::types::Type::Generic("U"),
                    }],
                    output: Some(erised::types::Type::ResolvedPath(erised::types::Path {
                        name: "Result",
                        prefix: "",
                        target: erised::types::Identifiable::Summary(|| {
                            erised::types::ItemSummary {
                                krate: || erised::types::ExternalCrate {
                                    name: "core",
                                    html_root_url: Some("https://doc.rust-lang.org/nightly/"),
                                },
                                path: &["core", "result", "Result"],
                                kind: erised::types::ItemKind::Enum,
                            }
                        }),
                        args: Some(|| erised::types::GenericArgs::AngleBracketed {
                            args: &[
                                erised::types::GenericArg::Type(erised::types::Type::Generic("T")),
                                erised::types::GenericArg::Type(
                                    erised::types::Type::QualifiedPath {
                                        name: "Error",
                                        args: || erised::types::GenericArgs::AngleBracketed {
                                            args: &[],
                                            bindings: &[],
                                        },
                                        self_type: || erised::types::Type::Generic("T"),
                                        trait_: erised::types::Path {
                                            name: "TryFrom",
                                            prefix: "",
                                            target: erised::types::Identifiable::Summary(|| {
                                                erised::types::ItemSummary {
                                                    krate: || erised::types::ExternalCrate {
                                                        name: "core",
                                                        html_root_url: Some(
                                                            "https://doc.rust-lang.org/nightly/",
                                                        ),
                                                    },
                                                    path: &["core", "convert", "TryFrom"],
                                                    kind: erised::types::ItemKind::Trait,
                                                }
                                            }),
                                            args: Some(|| {
                                                erised::types::GenericArgs::AngleBracketed {
                                                    args: &[erised::types::GenericArg::Type(
                                                        erised::types::Type::Generic("U"),
                                                    )],
                                                    bindings: &[],
                                                }
                                            }),
                                        },
                                    },
                                ),
                            ],
                            bindings: &[],
                        }),
                    })),
                    c_variadic: false,
                },
                generics: erised::types::Generics {
                    params: &[],
                    where_predicates: &[],
                },
                header: erised::types::Header {
                    const_: false,
                    unsafe_: false,
                    async_: false,
                    abi: erised::types::Abi::Rust,
                },
                has_body: true,
            })
        },
        || {
            erised::types::Item::Function(erised::types::Function {
                name: "type_id",
                meta: erised::types::ItemMeta {
                    krate: || erised::types::ExternalCrate {
                        name: "core",
                        html_root_url: Some("https://doc.rust-lang.org/nightly/"),
                    },
                    summary: None,
                    span: None,
                    visibility: erised::types::Visibility::Default,
                    docs: None,
                    attrs: &[],
                    deprecation: None,
                },
                decl: erised::types::FnDecl {
                    inputs: &[erised::types::FnInput {
                        pat: "self",
                        ty: erised::types::Type::BorrowedRef {
                            lifetime: None,
                            mutable: false,
                            type_: || erised::types::Type::Generic("Self"),
                        },
                    }],
                    output: Some(erised::types::Type::ResolvedPath(erised::types::Path {
                        name: "TypeId",
                        prefix: "",
                        target: erised::types::Identifiable::Summary(|| {
                            erised::types::ItemSummary {
                                krate: || erised::types::ExternalCrate {
                                    name: "core",
                                    html_root_url: Some("https://doc.rust-lang.org/nightly/"),
                                },
                                path: &["core", "any", "TypeId"],
                                kind: erised::types::ItemKind::Struct,
                            }
                        }),
                        args: Some(|| erised::types::GenericArgs::AngleBracketed {
                            args: &[],
                            bindings: &[],
                        }),
                    })),
                    c_variadic: false,
                },
                generics: erised::types::Generics {
                    params: &[],
                    where_predicates: &[],
                },
                header: erised::types::Header {
                    const_: false,
                    unsafe_: false,
                    async_: false,
                    abi: erised::types::Abi::Rust,
                },
                has_body: true,
            })
        },
        || {
            erised::types::Item::Impl(erised::types::Impl {
                meta: erised::types::ItemMeta {
                    krate: || erised::types::ExternalCrate {
                        name: "erised_tests",
                        html_root_url: None,
                    },
                    summary: None,
                    span: None,
                    visibility: erised::types::Visibility::Default,
                    docs: None,
                    attrs: &[],
                    deprecation: None,
                },
                is_unsafe: false,
                generics: erised::types::Generics {
                    params: &[],
                    where_predicates: &[],
                },
                provided_trait_methods: &[],
                trait_: Some(erised::types::Path {
                    name: "Send",
                    prefix: "",
                    target: erised::types::Identifiable::Summary(|| erised::types::ItemSummary {
                        krate: || erised::types::ExternalCrate {
                            name: "core",
                            html_root_url: Some("https://doc.rust-lang.org/nightly/"),
                        },
                        path: &["core", "marker", "Send"],
                        kind: erised::types::ItemKind::Trait,
                    }),
                    args: Some(|| erised::types::GenericArgs::AngleBracketed {
                        args: &[],
                        bindings: &[],
                    }),
                }),
                for_: erised::types::Type::ResolvedPath(erised::types::Path {
                    name: "MyStruct",
                    prefix: "",
                    target: erised::types::Identifiable::Item(|| {
                        <crate::MyStruct as Reflect>::TYPE_INFO
                    }),
                    args: Some(|| erised::types::GenericArgs::AngleBracketed {
                        args: &[],
                        bindings: &[],
                    }),
                }),
                items: &[],
                negative: false,
                synthetic: true,
                blanket_impl: None,
            })
        },
        || {
            erised::types::Item::Impl(erised::types::Impl {
                meta: erised::types::ItemMeta {
                    krate: || erised::types::ExternalCrate {
                        name: "erised_tests",
                        html_root_url: None,
                    },
                    summary: None,
                    span: None,
                    visibility: erised::types::Visibility::Default,
                    docs: None,
                    attrs: &[],
                    deprecation: None,
                },
                is_unsafe: false,
                generics: erised::types::Generics {
                    params: &[],
                    where_predicates: &[],
                },
                provided_trait_methods: &[],
                trait_: Some(erised::types::Path {
                    name: "Send",
                    prefix: "",
                    target: erised::types::Identifiable::Summary(|| erised::types::ItemSummary {
                        krate: || erised::types::ExternalCrate {
                            name: "core",
                            html_root_url: Some("https://doc.rust-lang.org/nightly/"),
                        },
                        path: &["core", "marker", "Send"],
                        kind: erised::types::ItemKind::Trait,
                    }),
                    args: Some(|| erised::types::GenericArgs::AngleBracketed {
                        args: &[],
                        bindings: &[],
                    }),
                }),
                for_: erised::types::Type::ResolvedPath(erised::types::Path {
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
                items: &[],
                negative: false,
                synthetic: true,
                blanket_impl: None,
            })
        },
        || {
            erised::types::Item::Impl(erised::types::Impl {
                meta: erised::types::ItemMeta {
                    krate: || erised::types::ExternalCrate {
                        name: "erised_tests",
                        html_root_url: None,
                    },
                    summary: None,
                    span: None,
                    visibility: erised::types::Visibility::Default,
                    docs: None,
                    attrs: &[],
                    deprecation: None,
                },
                is_unsafe: false,
                generics: erised::types::Generics {
                    params: &[],
                    where_predicates: &[],
                },
                provided_trait_methods: &[],
                trait_: Some(erised::types::Path {
                    name: "Sync",
                    prefix: "",
                    target: erised::types::Identifiable::Summary(|| erised::types::ItemSummary {
                        krate: || erised::types::ExternalCrate {
                            name: "core",
                            html_root_url: Some("https://doc.rust-lang.org/nightly/"),
                        },
                        path: &["core", "marker", "Sync"],
                        kind: erised::types::ItemKind::Trait,
                    }),
                    args: Some(|| erised::types::GenericArgs::AngleBracketed {
                        args: &[],
                        bindings: &[],
                    }),
                }),
                for_: erised::types::Type::ResolvedPath(erised::types::Path {
                    name: "MyStruct",
                    prefix: "",
                    target: erised::types::Identifiable::Item(|| {
                        <crate::MyStruct as Reflect>::TYPE_INFO
                    }),
                    args: Some(|| erised::types::GenericArgs::AngleBracketed {
                        args: &[],
                        bindings: &[],
                    }),
                }),
                items: &[],
                negative: false,
                synthetic: true,
                blanket_impl: None,
            })
        },
        || {
            erised::types::Item::Impl(erised::types::Impl {
                meta: erised::types::ItemMeta {
                    krate: || erised::types::ExternalCrate {
                        name: "erised_tests",
                        html_root_url: None,
                    },
                    summary: None,
                    span: None,
                    visibility: erised::types::Visibility::Default,
                    docs: None,
                    attrs: &[],
                    deprecation: None,
                },
                is_unsafe: false,
                generics: erised::types::Generics {
                    params: &[],
                    where_predicates: &[],
                },
                provided_trait_methods: &[],
                trait_: Some(erised::types::Path {
                    name: "Sync",
                    prefix: "",
                    target: erised::types::Identifiable::Summary(|| erised::types::ItemSummary {
                        krate: || erised::types::ExternalCrate {
                            name: "core",
                            html_root_url: Some("https://doc.rust-lang.org/nightly/"),
                        },
                        path: &["core", "marker", "Sync"],
                        kind: erised::types::ItemKind::Trait,
                    }),
                    args: Some(|| erised::types::GenericArgs::AngleBracketed {
                        args: &[],
                        bindings: &[],
                    }),
                }),
                for_: erised::types::Type::ResolvedPath(erised::types::Path {
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
                items: &[],
                negative: false,
                synthetic: true,
                blanket_impl: None,
            })
        },
        || {
            erised::types::Item::Impl(erised::types::Impl {
                meta: erised::types::ItemMeta {
                    krate: || erised::types::ExternalCrate {
                        name: "erised_tests",
                        html_root_url: None,
                    },
                    summary: None,
                    span: None,
                    visibility: erised::types::Visibility::Default,
                    docs: None,
                    attrs: &[],
                    deprecation: None,
                },
                is_unsafe: false,
                generics: erised::types::Generics {
                    params: &[],
                    where_predicates: &[],
                },
                provided_trait_methods: &[],
                trait_: Some(erised::types::Path {
                    name: "Unpin",
                    prefix: "",
                    target: erised::types::Identifiable::Summary(|| erised::types::ItemSummary {
                        krate: || erised::types::ExternalCrate {
                            name: "core",
                            html_root_url: Some("https://doc.rust-lang.org/nightly/"),
                        },
                        path: &["core", "marker", "Unpin"],
                        kind: erised::types::ItemKind::Trait,
                    }),
                    args: Some(|| erised::types::GenericArgs::AngleBracketed {
                        args: &[],
                        bindings: &[],
                    }),
                }),
                for_: erised::types::Type::ResolvedPath(erised::types::Path {
                    name: "MyStruct",
                    prefix: "",
                    target: erised::types::Identifiable::Item(|| {
                        <crate::MyStruct as Reflect>::TYPE_INFO
                    }),
                    args: Some(|| erised::types::GenericArgs::AngleBracketed {
                        args: &[],
                        bindings: &[],
                    }),
                }),
                items: &[],
                negative: false,
                synthetic: true,
                blanket_impl: None,
            })
        },
        || {
            erised::types::Item::Impl(erised::types::Impl {
                meta: erised::types::ItemMeta {
                    krate: || erised::types::ExternalCrate {
                        name: "erised_tests",
                        html_root_url: None,
                    },
                    summary: None,
                    span: None,
                    visibility: erised::types::Visibility::Default,
                    docs: None,
                    attrs: &[],
                    deprecation: None,
                },
                is_unsafe: false,
                generics: erised::types::Generics {
                    params: &[],
                    where_predicates: &[],
                },
                provided_trait_methods: &[],
                trait_: Some(erised::types::Path {
                    name: "Unpin",
                    prefix: "",
                    target: erised::types::Identifiable::Summary(|| erised::types::ItemSummary {
                        krate: || erised::types::ExternalCrate {
                            name: "core",
                            html_root_url: Some("https://doc.rust-lang.org/nightly/"),
                        },
                        path: &["core", "marker", "Unpin"],
                        kind: erised::types::ItemKind::Trait,
                    }),
                    args: Some(|| erised::types::GenericArgs::AngleBracketed {
                        args: &[],
                        bindings: &[],
                    }),
                }),
                for_: erised::types::Type::ResolvedPath(erised::types::Path {
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
                items: &[],
                negative: false,
                synthetic: true,
                blanket_impl: None,
            })
        },
        || {
            erised::types::Item::Impl(erised::types::Impl {
                meta: erised::types::ItemMeta {
                    krate: || erised::types::ExternalCrate {
                        name: "erised_tests",
                        html_root_url: None,
                    },
                    summary: None,
                    span: None,
                    visibility: erised::types::Visibility::Default,
                    docs: None,
                    attrs: &[],
                    deprecation: None,
                },
                is_unsafe: false,
                generics: erised::types::Generics {
                    params: &[],
                    where_predicates: &[],
                },
                provided_trait_methods: &[],
                trait_: Some(erised::types::Path {
                    name: "UnwindSafe",
                    prefix: "",
                    target: erised::types::Identifiable::Summary(|| erised::types::ItemSummary {
                        krate: || erised::types::ExternalCrate {
                            name: "core",
                            html_root_url: Some("https://doc.rust-lang.org/nightly/"),
                        },
                        path: &["core", "panic", "unwind_safe", "UnwindSafe"],
                        kind: erised::types::ItemKind::Trait,
                    }),
                    args: Some(|| erised::types::GenericArgs::AngleBracketed {
                        args: &[],
                        bindings: &[],
                    }),
                }),
                for_: erised::types::Type::ResolvedPath(erised::types::Path {
                    name: "MyStruct",
                    prefix: "",
                    target: erised::types::Identifiable::Item(|| {
                        <crate::MyStruct as Reflect>::TYPE_INFO
                    }),
                    args: Some(|| erised::types::GenericArgs::AngleBracketed {
                        args: &[],
                        bindings: &[],
                    }),
                }),
                items: &[],
                negative: false,
                synthetic: true,
                blanket_impl: None,
            })
        },
        || {
            erised::types::Item::Impl(erised::types::Impl {
                meta: erised::types::ItemMeta {
                    krate: || erised::types::ExternalCrate {
                        name: "erised_tests",
                        html_root_url: None,
                    },
                    summary: None,
                    span: None,
                    visibility: erised::types::Visibility::Default,
                    docs: None,
                    attrs: &[],
                    deprecation: None,
                },
                is_unsafe: false,
                generics: erised::types::Generics {
                    params: &[],
                    where_predicates: &[],
                },
                provided_trait_methods: &[],
                trait_: Some(erised::types::Path {
                    name: "UnwindSafe",
                    prefix: "",
                    target: erised::types::Identifiable::Summary(|| erised::types::ItemSummary {
                        krate: || erised::types::ExternalCrate {
                            name: "core",
                            html_root_url: Some("https://doc.rust-lang.org/nightly/"),
                        },
                        path: &["core", "panic", "unwind_safe", "UnwindSafe"],
                        kind: erised::types::ItemKind::Trait,
                    }),
                    args: Some(|| erised::types::GenericArgs::AngleBracketed {
                        args: &[],
                        bindings: &[],
                    }),
                }),
                for_: erised::types::Type::ResolvedPath(erised::types::Path {
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
                items: &[],
                negative: false,
                synthetic: true,
                blanket_impl: None,
            })
        },
        || {
            erised::types::Item::Impl(erised::types::Impl {
                meta: erised::types::ItemMeta {
                    krate: || erised::types::ExternalCrate {
                        name: "erised_tests",
                        html_root_url: None,
                    },
                    summary: None,
                    span: None,
                    visibility: erised::types::Visibility::Default,
                    docs: None,
                    attrs: &[],
                    deprecation: None,
                },
                is_unsafe: false,
                generics: erised::types::Generics {
                    params: &[],
                    where_predicates: &[],
                },
                provided_trait_methods: &[],
                trait_: Some(erised::types::Path {
                    name: "RefUnwindSafe",
                    prefix: "",
                    target: erised::types::Identifiable::Summary(|| erised::types::ItemSummary {
                        krate: || erised::types::ExternalCrate {
                            name: "core",
                            html_root_url: Some("https://doc.rust-lang.org/nightly/"),
                        },
                        path: &["core", "panic", "unwind_safe", "RefUnwindSafe"],
                        kind: erised::types::ItemKind::Trait,
                    }),
                    args: Some(|| erised::types::GenericArgs::AngleBracketed {
                        args: &[],
                        bindings: &[],
                    }),
                }),
                for_: erised::types::Type::ResolvedPath(erised::types::Path {
                    name: "MyStruct",
                    prefix: "",
                    target: erised::types::Identifiable::Item(|| {
                        <crate::MyStruct as Reflect>::TYPE_INFO
                    }),
                    args: Some(|| erised::types::GenericArgs::AngleBracketed {
                        args: &[],
                        bindings: &[],
                    }),
                }),
                items: &[],
                negative: false,
                synthetic: true,
                blanket_impl: None,
            })
        },
        || {
            erised::types::Item::Impl(erised::types::Impl {
                meta: erised::types::ItemMeta {
                    krate: || erised::types::ExternalCrate {
                        name: "erised_tests",
                        html_root_url: None,
                    },
                    summary: None,
                    span: None,
                    visibility: erised::types::Visibility::Default,
                    docs: None,
                    attrs: &[],
                    deprecation: None,
                },
                is_unsafe: false,
                generics: erised::types::Generics {
                    params: &[],
                    where_predicates: &[],
                },
                provided_trait_methods: &[],
                trait_: Some(erised::types::Path {
                    name: "RefUnwindSafe",
                    prefix: "",
                    target: erised::types::Identifiable::Summary(|| erised::types::ItemSummary {
                        krate: || erised::types::ExternalCrate {
                            name: "core",
                            html_root_url: Some("https://doc.rust-lang.org/nightly/"),
                        },
                        path: &["core", "panic", "unwind_safe", "RefUnwindSafe"],
                        kind: erised::types::ItemKind::Trait,
                    }),
                    args: Some(|| erised::types::GenericArgs::AngleBracketed {
                        args: &[],
                        bindings: &[],
                    }),
                }),
                for_: erised::types::Type::ResolvedPath(erised::types::Path {
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
                items: &[],
                negative: false,
                synthetic: true,
                blanket_impl: None,
            })
        },
        || {
            erised::types::Item::Impl(erised::types::Impl {
                meta: erised::types::ItemMeta {
                    krate: || erised::types::ExternalCrate {
                        name: "erised_tests",
                        html_root_url: None,
                    },
                    summary: None,
                    span: None,
                    visibility: erised::types::Visibility::Default,
                    docs: None,
                    attrs: &[],
                    deprecation: None,
                },
                is_unsafe: false,
                generics: erised::types::Generics {
                    params: &[erised::types::GenericParamDef {
                        name: "T",
                        kind: erised::types::GenericParamDefKind::Type {
                            bounds: &[],
                            default: None,
                            synthetic: false,
                        },
                    }],
                    where_predicates: &[erised::types::WherePredicate::BoundPredicate {
                        type_: erised::types::Type::Generic("T"),
                        bounds: &[erised::types::GenericBound::TraitBound {
                            trait_: erised::types::Path {
                                name: "Sized",
                                prefix: "",
                                target: erised::types::Identifiable::Summary(|| {
                                    erised::types::ItemSummary {
                                        krate: || erised::types::ExternalCrate {
                                            name: "core",
                                            html_root_url: Some(
                                                "https://doc.rust-lang.org/nightly/",
                                            ),
                                        },
                                        path: &["core", "marker", "Sized"],
                                        kind: erised::types::ItemKind::Trait,
                                    }
                                }),
                                args: Some(|| erised::types::GenericArgs::AngleBracketed {
                                    args: &[],
                                    bindings: &[],
                                }),
                            },
                            generic_params: &[],
                            modifier: erised::types::TraitBoundModifier::Maybe,
                        }],
                        generic_params: &[],
                    }],
                },
                provided_trait_methods: &[],
                trait_: Some(erised::types::Path {
                    name: "Borrow",
                    prefix: "",
                    target: erised::types::Identifiable::Summary(|| erised::types::ItemSummary {
                        krate: || erised::types::ExternalCrate {
                            name: "core",
                            html_root_url: Some("https://doc.rust-lang.org/nightly/"),
                        },
                        path: &["core", "borrow", "Borrow"],
                        kind: erised::types::ItemKind::Trait,
                    }),
                    args: Some(|| erised::types::GenericArgs::AngleBracketed {
                        args: &[erised::types::GenericArg::Type(
                            erised::types::Type::Generic("T"),
                        )],
                        bindings: &[],
                    }),
                }),
                for_: erised::types::Type::ResolvedPath(erised::types::Path {
                    name: "MyStruct",
                    prefix: "",
                    target: erised::types::Identifiable::Item(|| {
                        <crate::MyStruct as Reflect>::TYPE_INFO
                    }),
                    args: Some(|| erised::types::GenericArgs::AngleBracketed {
                        args: &[],
                        bindings: &[],
                    }),
                }),
                items: &[|| {
                    erised::types::Item::Function(erised::types::Function {
                        name: "borrow",
                        meta: erised::types::ItemMeta {
                            krate: || erised::types::ExternalCrate {
                                name: "core",
                                html_root_url: Some("https://doc.rust-lang.org/nightly/"),
                            },
                            summary: None,
                            span: None,
                            visibility: erised::types::Visibility::Default,
                            docs: None,
                            attrs: &["#[rustc_diagnostic_item = \"noop_method_borrow\"]"],
                            deprecation: None,
                        },
                        decl: erised::types::FnDecl {
                            inputs: &[erised::types::FnInput {
                                pat: "self",
                                ty: erised::types::Type::BorrowedRef {
                                    lifetime: None,
                                    mutable: false,
                                    type_: || erised::types::Type::Generic("Self"),
                                },
                            }],
                            output: Some(erised::types::Type::BorrowedRef {
                                lifetime: None,
                                mutable: false,
                                type_: || erised::types::Type::Generic("T"),
                            }),
                            c_variadic: false,
                        },
                        generics: erised::types::Generics {
                            params: &[],
                            where_predicates: &[],
                        },
                        header: erised::types::Header {
                            const_: false,
                            unsafe_: false,
                            async_: false,
                            abi: erised::types::Abi::Rust,
                        },
                        has_body: true,
                    })
                }],
                negative: false,
                synthetic: false,
                blanket_impl: Some(erised::types::Type::Generic("T")),
            })
        },
        || {
            erised::types::Item::Impl(erised::types::Impl {
                meta: erised::types::ItemMeta {
                    krate: || erised::types::ExternalCrate {
                        name: "erised_tests",
                        html_root_url: None,
                    },
                    summary: None,
                    span: None,
                    visibility: erised::types::Visibility::Default,
                    docs: None,
                    attrs: &[],
                    deprecation: None,
                },
                is_unsafe: false,
                generics: erised::types::Generics {
                    params: &[erised::types::GenericParamDef {
                        name: "T",
                        kind: erised::types::GenericParamDefKind::Type {
                            bounds: &[],
                            default: None,
                            synthetic: false,
                        },
                    }],
                    where_predicates: &[erised::types::WherePredicate::BoundPredicate {
                        type_: erised::types::Type::Generic("T"),
                        bounds: &[erised::types::GenericBound::TraitBound {
                            trait_: erised::types::Path {
                                name: "Sized",
                                prefix: "",
                                target: erised::types::Identifiable::Summary(|| {
                                    erised::types::ItemSummary {
                                        krate: || erised::types::ExternalCrate {
                                            name: "core",
                                            html_root_url: Some(
                                                "https://doc.rust-lang.org/nightly/",
                                            ),
                                        },
                                        path: &["core", "marker", "Sized"],
                                        kind: erised::types::ItemKind::Trait,
                                    }
                                }),
                                args: Some(|| erised::types::GenericArgs::AngleBracketed {
                                    args: &[],
                                    bindings: &[],
                                }),
                            },
                            generic_params: &[],
                            modifier: erised::types::TraitBoundModifier::Maybe,
                        }],
                        generic_params: &[],
                    }],
                },
                provided_trait_methods: &[],
                trait_: Some(erised::types::Path {
                    name: "Borrow",
                    prefix: "",
                    target: erised::types::Identifiable::Summary(|| erised::types::ItemSummary {
                        krate: || erised::types::ExternalCrate {
                            name: "core",
                            html_root_url: Some("https://doc.rust-lang.org/nightly/"),
                        },
                        path: &["core", "borrow", "Borrow"],
                        kind: erised::types::ItemKind::Trait,
                    }),
                    args: Some(|| erised::types::GenericArgs::AngleBracketed {
                        args: &[erised::types::GenericArg::Type(
                            erised::types::Type::Generic("T"),
                        )],
                        bindings: &[],
                    }),
                }),
                for_: erised::types::Type::ResolvedPath(erised::types::Path {
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
                items: &[|| {
                    erised::types::Item::Function(erised::types::Function {
                        name: "borrow",
                        meta: erised::types::ItemMeta {
                            krate: || erised::types::ExternalCrate {
                                name: "core",
                                html_root_url: Some("https://doc.rust-lang.org/nightly/"),
                            },
                            summary: None,
                            span: None,
                            visibility: erised::types::Visibility::Default,
                            docs: None,
                            attrs: &["#[rustc_diagnostic_item = \"noop_method_borrow\"]"],
                            deprecation: None,
                        },
                        decl: erised::types::FnDecl {
                            inputs: &[erised::types::FnInput {
                                pat: "self",
                                ty: erised::types::Type::BorrowedRef {
                                    lifetime: None,
                                    mutable: false,
                                    type_: || erised::types::Type::Generic("Self"),
                                },
                            }],
                            output: Some(erised::types::Type::BorrowedRef {
                                lifetime: None,
                                mutable: false,
                                type_: || erised::types::Type::Generic("T"),
                            }),
                            c_variadic: false,
                        },
                        generics: erised::types::Generics {
                            params: &[],
                            where_predicates: &[],
                        },
                        header: erised::types::Header {
                            const_: false,
                            unsafe_: false,
                            async_: false,
                            abi: erised::types::Abi::Rust,
                        },
                        has_body: true,
                    })
                }],
                negative: false,
                synthetic: false,
                blanket_impl: Some(erised::types::Type::Generic("T")),
            })
        },
        || {
            erised::types::Item::Impl(erised::types::Impl {
                meta: erised::types::ItemMeta {
                    krate: || erised::types::ExternalCrate {
                        name: "erised_tests",
                        html_root_url: None,
                    },
                    summary: None,
                    span: None,
                    visibility: erised::types::Visibility::Default,
                    docs: None,
                    attrs: &[],
                    deprecation: None,
                },
                is_unsafe: false,
                generics: erised::types::Generics {
                    params: &[erised::types::GenericParamDef {
                        name: "T",
                        kind: erised::types::GenericParamDefKind::Type {
                            bounds: &[],
                            default: None,
                            synthetic: false,
                        },
                    }],
                    where_predicates: &[erised::types::WherePredicate::BoundPredicate {
                        type_: erised::types::Type::Generic("T"),
                        bounds: &[erised::types::GenericBound::TraitBound {
                            trait_: erised::types::Path {
                                name: "Sized",
                                prefix: "",
                                target: erised::types::Identifiable::Summary(|| {
                                    erised::types::ItemSummary {
                                        krate: || erised::types::ExternalCrate {
                                            name: "core",
                                            html_root_url: Some(
                                                "https://doc.rust-lang.org/nightly/",
                                            ),
                                        },
                                        path: &["core", "marker", "Sized"],
                                        kind: erised::types::ItemKind::Trait,
                                    }
                                }),
                                args: Some(|| erised::types::GenericArgs::AngleBracketed {
                                    args: &[],
                                    bindings: &[],
                                }),
                            },
                            generic_params: &[],
                            modifier: erised::types::TraitBoundModifier::Maybe,
                        }],
                        generic_params: &[],
                    }],
                },
                provided_trait_methods: &[],
                trait_: Some(erised::types::Path {
                    name: "BorrowMut",
                    prefix: "",
                    target: erised::types::Identifiable::Summary(|| erised::types::ItemSummary {
                        krate: || erised::types::ExternalCrate {
                            name: "core",
                            html_root_url: Some("https://doc.rust-lang.org/nightly/"),
                        },
                        path: &["core", "borrow", "BorrowMut"],
                        kind: erised::types::ItemKind::Trait,
                    }),
                    args: Some(|| erised::types::GenericArgs::AngleBracketed {
                        args: &[erised::types::GenericArg::Type(
                            erised::types::Type::Generic("T"),
                        )],
                        bindings: &[],
                    }),
                }),
                for_: erised::types::Type::ResolvedPath(erised::types::Path {
                    name: "MyStruct",
                    prefix: "",
                    target: erised::types::Identifiable::Item(|| {
                        <crate::MyStruct as Reflect>::TYPE_INFO
                    }),
                    args: Some(|| erised::types::GenericArgs::AngleBracketed {
                        args: &[],
                        bindings: &[],
                    }),
                }),
                items: &[|| {
                    erised::types::Item::Function(erised::types::Function {
                        name: "borrow_mut",
                        meta: erised::types::ItemMeta {
                            krate: || erised::types::ExternalCrate {
                                name: "core",
                                html_root_url: Some("https://doc.rust-lang.org/nightly/"),
                            },
                            summary: None,
                            span: None,
                            visibility: erised::types::Visibility::Default,
                            docs: None,
                            attrs: &[],
                            deprecation: None,
                        },
                        decl: erised::types::FnDecl {
                            inputs: &[erised::types::FnInput {
                                pat: "self",
                                ty: erised::types::Type::BorrowedRef {
                                    lifetime: None,
                                    mutable: true,
                                    type_: || erised::types::Type::Generic("Self"),
                                },
                            }],
                            output: Some(erised::types::Type::BorrowedRef {
                                lifetime: None,
                                mutable: true,
                                type_: || erised::types::Type::Generic("T"),
                            }),
                            c_variadic: false,
                        },
                        generics: erised::types::Generics {
                            params: &[],
                            where_predicates: &[],
                        },
                        header: erised::types::Header {
                            const_: false,
                            unsafe_: false,
                            async_: false,
                            abi: erised::types::Abi::Rust,
                        },
                        has_body: true,
                    })
                }],
                negative: false,
                synthetic: false,
                blanket_impl: Some(erised::types::Type::Generic("T")),
            })
        },
        || {
            erised::types::Item::Impl(erised::types::Impl {
                meta: erised::types::ItemMeta {
                    krate: || erised::types::ExternalCrate {
                        name: "erised_tests",
                        html_root_url: None,
                    },
                    summary: None,
                    span: None,
                    visibility: erised::types::Visibility::Default,
                    docs: None,
                    attrs: &[],
                    deprecation: None,
                },
                is_unsafe: false,
                generics: erised::types::Generics {
                    params: &[erised::types::GenericParamDef {
                        name: "T",
                        kind: erised::types::GenericParamDefKind::Type {
                            bounds: &[],
                            default: None,
                            synthetic: false,
                        },
                    }],
                    where_predicates: &[erised::types::WherePredicate::BoundPredicate {
                        type_: erised::types::Type::Generic("T"),
                        bounds: &[erised::types::GenericBound::TraitBound {
                            trait_: erised::types::Path {
                                name: "Sized",
                                prefix: "",
                                target: erised::types::Identifiable::Summary(|| {
                                    erised::types::ItemSummary {
                                        krate: || erised::types::ExternalCrate {
                                            name: "core",
                                            html_root_url: Some(
                                                "https://doc.rust-lang.org/nightly/",
                                            ),
                                        },
                                        path: &["core", "marker", "Sized"],
                                        kind: erised::types::ItemKind::Trait,
                                    }
                                }),
                                args: Some(|| erised::types::GenericArgs::AngleBracketed {
                                    args: &[],
                                    bindings: &[],
                                }),
                            },
                            generic_params: &[],
                            modifier: erised::types::TraitBoundModifier::Maybe,
                        }],
                        generic_params: &[],
                    }],
                },
                provided_trait_methods: &[],
                trait_: Some(erised::types::Path {
                    name: "BorrowMut",
                    prefix: "",
                    target: erised::types::Identifiable::Summary(|| erised::types::ItemSummary {
                        krate: || erised::types::ExternalCrate {
                            name: "core",
                            html_root_url: Some("https://doc.rust-lang.org/nightly/"),
                        },
                        path: &["core", "borrow", "BorrowMut"],
                        kind: erised::types::ItemKind::Trait,
                    }),
                    args: Some(|| erised::types::GenericArgs::AngleBracketed {
                        args: &[erised::types::GenericArg::Type(
                            erised::types::Type::Generic("T"),
                        )],
                        bindings: &[],
                    }),
                }),
                for_: erised::types::Type::ResolvedPath(erised::types::Path {
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
                items: &[|| {
                    erised::types::Item::Function(erised::types::Function {
                        name: "borrow_mut",
                        meta: erised::types::ItemMeta {
                            krate: || erised::types::ExternalCrate {
                                name: "core",
                                html_root_url: Some("https://doc.rust-lang.org/nightly/"),
                            },
                            summary: None,
                            span: None,
                            visibility: erised::types::Visibility::Default,
                            docs: None,
                            attrs: &[],
                            deprecation: None,
                        },
                        decl: erised::types::FnDecl {
                            inputs: &[erised::types::FnInput {
                                pat: "self",
                                ty: erised::types::Type::BorrowedRef {
                                    lifetime: None,
                                    mutable: true,
                                    type_: || erised::types::Type::Generic("Self"),
                                },
                            }],
                            output: Some(erised::types::Type::BorrowedRef {
                                lifetime: None,
                                mutable: true,
                                type_: || erised::types::Type::Generic("T"),
                            }),
                            c_variadic: false,
                        },
                        generics: erised::types::Generics {
                            params: &[],
                            where_predicates: &[],
                        },
                        header: erised::types::Header {
                            const_: false,
                            unsafe_: false,
                            async_: false,
                            abi: erised::types::Abi::Rust,
                        },
                        has_body: true,
                    })
                }],
                negative: false,
                synthetic: false,
                blanket_impl: Some(erised::types::Type::Generic("T")),
            })
        },
        || {
            erised::types::Item::Impl(erised::types::Impl {
                meta: erised::types::ItemMeta {
                    krate: || erised::types::ExternalCrate {
                        name: "erised_tests",
                        html_root_url: None,
                    },
                    summary: None,
                    span: None,
                    visibility: erised::types::Visibility::Default,
                    docs: None,
                    attrs: &[],
                    deprecation: None,
                },
                is_unsafe: false,
                generics: erised::types::Generics {
                    params: &[
                        erised::types::GenericParamDef {
                            name: "T",
                            kind: erised::types::GenericParamDefKind::Type {
                                bounds: &[],
                                default: None,
                                synthetic: false,
                            },
                        },
                        erised::types::GenericParamDef {
                            name: "U",
                            kind: erised::types::GenericParamDefKind::Type {
                                bounds: &[],
                                default: None,
                                synthetic: false,
                            },
                        },
                    ],
                    where_predicates: &[erised::types::WherePredicate::BoundPredicate {
                        type_: erised::types::Type::Generic("U"),
                        bounds: &[erised::types::GenericBound::TraitBound {
                            trait_: erised::types::Path {
                                name: "From",
                                prefix: "",
                                target: erised::types::Identifiable::Summary(|| {
                                    erised::types::ItemSummary {
                                        krate: || erised::types::ExternalCrate {
                                            name: "core",
                                            html_root_url: Some(
                                                "https://doc.rust-lang.org/nightly/",
                                            ),
                                        },
                                        path: &["core", "convert", "From"],
                                        kind: erised::types::ItemKind::Trait,
                                    }
                                }),
                                args: Some(|| erised::types::GenericArgs::AngleBracketed {
                                    args: &[erised::types::GenericArg::Type(
                                        erised::types::Type::Generic("T"),
                                    )],
                                    bindings: &[],
                                }),
                            },
                            generic_params: &[],
                            modifier: erised::types::TraitBoundModifier::None,
                        }],
                        generic_params: &[],
                    }],
                },
                provided_trait_methods: &[],
                trait_: Some(erised::types::Path {
                    name: "Into",
                    prefix: "",
                    target: erised::types::Identifiable::Summary(|| erised::types::ItemSummary {
                        krate: || erised::types::ExternalCrate {
                            name: "core",
                            html_root_url: Some("https://doc.rust-lang.org/nightly/"),
                        },
                        path: &["core", "convert", "Into"],
                        kind: erised::types::ItemKind::Trait,
                    }),
                    args: Some(|| erised::types::GenericArgs::AngleBracketed {
                        args: &[erised::types::GenericArg::Type(
                            erised::types::Type::Generic("U"),
                        )],
                        bindings: &[],
                    }),
                }),
                for_: erised::types::Type::ResolvedPath(erised::types::Path {
                    name: "MyStruct",
                    prefix: "",
                    target: erised::types::Identifiable::Item(|| {
                        <crate::MyStruct as Reflect>::TYPE_INFO
                    }),
                    args: Some(|| erised::types::GenericArgs::AngleBracketed {
                        args: &[],
                        bindings: &[],
                    }),
                }),
                items: &[|| {
                    erised :: types :: Item :: Function (erised :: types :: Function { name : "into" , meta : erised :: types :: ItemMeta { krate : || erised :: types :: ExternalCrate { name : "core" , html_root_url : Some ("https://doc.rust-lang.org/nightly/") } , summary : None , span : None , visibility : erised :: types :: Visibility :: Default , docs : Some ("Calls `U::from(self)`.\n\nThat is, this conversion is whatever the implementation of\n<code>[From]&lt;T&gt; for U</code> chooses to do.") , attrs : & [] , deprecation : None } , decl : erised :: types :: FnDecl { inputs : & [erised :: types :: FnInput { pat : "self" , ty : erised :: types :: Type :: Generic ("Self") }] , output : Some (erised :: types :: Type :: Generic ("U")) , c_variadic : false } , generics : erised :: types :: Generics { params : & [] , where_predicates : & [] } , header : erised :: types :: Header { const_ : false , unsafe_ : false , async_ : false , abi : erised :: types :: Abi :: Rust } , has_body : true })
                }],
                negative: false,
                synthetic: false,
                blanket_impl: Some(erised::types::Type::Generic("T")),
            })
        },
        || {
            erised::types::Item::Impl(erised::types::Impl {
                meta: erised::types::ItemMeta {
                    krate: || erised::types::ExternalCrate {
                        name: "erised_tests",
                        html_root_url: None,
                    },
                    summary: None,
                    span: None,
                    visibility: erised::types::Visibility::Default,
                    docs: None,
                    attrs: &[],
                    deprecation: None,
                },
                is_unsafe: false,
                generics: erised::types::Generics {
                    params: &[
                        erised::types::GenericParamDef {
                            name: "T",
                            kind: erised::types::GenericParamDefKind::Type {
                                bounds: &[],
                                default: None,
                                synthetic: false,
                            },
                        },
                        erised::types::GenericParamDef {
                            name: "U",
                            kind: erised::types::GenericParamDefKind::Type {
                                bounds: &[],
                                default: None,
                                synthetic: false,
                            },
                        },
                    ],
                    where_predicates: &[erised::types::WherePredicate::BoundPredicate {
                        type_: erised::types::Type::Generic("U"),
                        bounds: &[erised::types::GenericBound::TraitBound {
                            trait_: erised::types::Path {
                                name: "From",
                                prefix: "",
                                target: erised::types::Identifiable::Summary(|| {
                                    erised::types::ItemSummary {
                                        krate: || erised::types::ExternalCrate {
                                            name: "core",
                                            html_root_url: Some(
                                                "https://doc.rust-lang.org/nightly/",
                                            ),
                                        },
                                        path: &["core", "convert", "From"],
                                        kind: erised::types::ItemKind::Trait,
                                    }
                                }),
                                args: Some(|| erised::types::GenericArgs::AngleBracketed {
                                    args: &[erised::types::GenericArg::Type(
                                        erised::types::Type::Generic("T"),
                                    )],
                                    bindings: &[],
                                }),
                            },
                            generic_params: &[],
                            modifier: erised::types::TraitBoundModifier::None,
                        }],
                        generic_params: &[],
                    }],
                },
                provided_trait_methods: &[],
                trait_: Some(erised::types::Path {
                    name: "Into",
                    prefix: "",
                    target: erised::types::Identifiable::Summary(|| erised::types::ItemSummary {
                        krate: || erised::types::ExternalCrate {
                            name: "core",
                            html_root_url: Some("https://doc.rust-lang.org/nightly/"),
                        },
                        path: &["core", "convert", "Into"],
                        kind: erised::types::ItemKind::Trait,
                    }),
                    args: Some(|| erised::types::GenericArgs::AngleBracketed {
                        args: &[erised::types::GenericArg::Type(
                            erised::types::Type::Generic("U"),
                        )],
                        bindings: &[],
                    }),
                }),
                for_: erised::types::Type::ResolvedPath(erised::types::Path {
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
                items: &[|| {
                    erised :: types :: Item :: Function (erised :: types :: Function { name : "into" , meta : erised :: types :: ItemMeta { krate : || erised :: types :: ExternalCrate { name : "core" , html_root_url : Some ("https://doc.rust-lang.org/nightly/") } , summary : None , span : None , visibility : erised :: types :: Visibility :: Default , docs : Some ("Calls `U::from(self)`.\n\nThat is, this conversion is whatever the implementation of\n<code>[From]&lt;T&gt; for U</code> chooses to do.") , attrs : & [] , deprecation : None } , decl : erised :: types :: FnDecl { inputs : & [erised :: types :: FnInput { pat : "self" , ty : erised :: types :: Type :: Generic ("Self") }] , output : Some (erised :: types :: Type :: Generic ("U")) , c_variadic : false } , generics : erised :: types :: Generics { params : & [] , where_predicates : & [] } , header : erised :: types :: Header { const_ : false , unsafe_ : false , async_ : false , abi : erised :: types :: Abi :: Rust } , has_body : true })
                }],
                negative: false,
                synthetic: false,
                blanket_impl: Some(erised::types::Type::Generic("T")),
            })
        },
        || {
            erised::types::Item::Impl(erised::types::Impl {
                meta: erised::types::ItemMeta {
                    krate: || erised::types::ExternalCrate {
                        name: "erised_tests",
                        html_root_url: None,
                    },
                    summary: None,
                    span: None,
                    visibility: erised::types::Visibility::Default,
                    docs: None,
                    attrs: &[],
                    deprecation: None,
                },
                is_unsafe: false,
                generics: erised::types::Generics {
                    params: &[erised::types::GenericParamDef {
                        name: "T",
                        kind: erised::types::GenericParamDefKind::Type {
                            bounds: &[],
                            default: None,
                            synthetic: false,
                        },
                    }],
                    where_predicates: &[],
                },
                provided_trait_methods: &[],
                trait_: Some(erised::types::Path {
                    name: "From",
                    prefix: "",
                    target: erised::types::Identifiable::Summary(|| erised::types::ItemSummary {
                        krate: || erised::types::ExternalCrate {
                            name: "core",
                            html_root_url: Some("https://doc.rust-lang.org/nightly/"),
                        },
                        path: &["core", "convert", "From"],
                        kind: erised::types::ItemKind::Trait,
                    }),
                    args: Some(|| erised::types::GenericArgs::AngleBracketed {
                        args: &[erised::types::GenericArg::Type(
                            erised::types::Type::Generic("T"),
                        )],
                        bindings: &[],
                    }),
                }),
                for_: erised::types::Type::ResolvedPath(erised::types::Path {
                    name: "MyStruct",
                    prefix: "",
                    target: erised::types::Identifiable::Item(|| {
                        <crate::MyStruct as Reflect>::TYPE_INFO
                    }),
                    args: Some(|| erised::types::GenericArgs::AngleBracketed {
                        args: &[],
                        bindings: &[],
                    }),
                }),
                items: &[|| {
                    erised::types::Item::Function(erised::types::Function {
                        name: "from",
                        meta: erised::types::ItemMeta {
                            krate: || erised::types::ExternalCrate {
                                name: "core",
                                html_root_url: Some("https://doc.rust-lang.org/nightly/"),
                            },
                            summary: None,
                            span: None,
                            visibility: erised::types::Visibility::Default,
                            docs: Some("Returns the argument unchanged."),
                            attrs: &[],
                            deprecation: None,
                        },
                        decl: erised::types::FnDecl {
                            inputs: &[erised::types::FnInput {
                                pat: "t",
                                ty: erised::types::Type::Generic("T"),
                            }],
                            output: Some(erised::types::Type::Generic("T")),
                            c_variadic: false,
                        },
                        generics: erised::types::Generics {
                            params: &[],
                            where_predicates: &[],
                        },
                        header: erised::types::Header {
                            const_: false,
                            unsafe_: false,
                            async_: false,
                            abi: erised::types::Abi::Rust,
                        },
                        has_body: true,
                    })
                }],
                negative: false,
                synthetic: false,
                blanket_impl: Some(erised::types::Type::Generic("T")),
            })
        },
        || {
            erised::types::Item::Impl(erised::types::Impl {
                meta: erised::types::ItemMeta {
                    krate: || erised::types::ExternalCrate {
                        name: "erised_tests",
                        html_root_url: None,
                    },
                    summary: None,
                    span: None,
                    visibility: erised::types::Visibility::Default,
                    docs: None,
                    attrs: &[],
                    deprecation: None,
                },
                is_unsafe: false,
                generics: erised::types::Generics {
                    params: &[erised::types::GenericParamDef {
                        name: "T",
                        kind: erised::types::GenericParamDefKind::Type {
                            bounds: &[],
                            default: None,
                            synthetic: false,
                        },
                    }],
                    where_predicates: &[],
                },
                provided_trait_methods: &[],
                trait_: Some(erised::types::Path {
                    name: "From",
                    prefix: "",
                    target: erised::types::Identifiable::Summary(|| erised::types::ItemSummary {
                        krate: || erised::types::ExternalCrate {
                            name: "core",
                            html_root_url: Some("https://doc.rust-lang.org/nightly/"),
                        },
                        path: &["core", "convert", "From"],
                        kind: erised::types::ItemKind::Trait,
                    }),
                    args: Some(|| erised::types::GenericArgs::AngleBracketed {
                        args: &[erised::types::GenericArg::Type(
                            erised::types::Type::Generic("T"),
                        )],
                        bindings: &[],
                    }),
                }),
                for_: erised::types::Type::ResolvedPath(erised::types::Path {
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
                items: &[|| {
                    erised::types::Item::Function(erised::types::Function {
                        name: "from",
                        meta: erised::types::ItemMeta {
                            krate: || erised::types::ExternalCrate {
                                name: "core",
                                html_root_url: Some("https://doc.rust-lang.org/nightly/"),
                            },
                            summary: None,
                            span: None,
                            visibility: erised::types::Visibility::Default,
                            docs: Some("Returns the argument unchanged."),
                            attrs: &[],
                            deprecation: None,
                        },
                        decl: erised::types::FnDecl {
                            inputs: &[erised::types::FnInput {
                                pat: "t",
                                ty: erised::types::Type::Generic("T"),
                            }],
                            output: Some(erised::types::Type::Generic("T")),
                            c_variadic: false,
                        },
                        generics: erised::types::Generics {
                            params: &[],
                            where_predicates: &[],
                        },
                        header: erised::types::Header {
                            const_: false,
                            unsafe_: false,
                            async_: false,
                            abi: erised::types::Abi::Rust,
                        },
                        has_body: true,
                    })
                }],
                negative: false,
                synthetic: false,
                blanket_impl: Some(erised::types::Type::Generic("T")),
            })
        },
        || {
            erised::types::Item::Impl(erised::types::Impl {
                meta: erised::types::ItemMeta {
                    krate: || erised::types::ExternalCrate {
                        name: "erised_tests",
                        html_root_url: None,
                    },
                    summary: None,
                    span: None,
                    visibility: erised::types::Visibility::Default,
                    docs: None,
                    attrs: &[],
                    deprecation: None,
                },
                is_unsafe: false,
                generics: erised::types::Generics {
                    params: &[
                        erised::types::GenericParamDef {
                            name: "T",
                            kind: erised::types::GenericParamDefKind::Type {
                                bounds: &[],
                                default: None,
                                synthetic: false,
                            },
                        },
                        erised::types::GenericParamDef {
                            name: "U",
                            kind: erised::types::GenericParamDefKind::Type {
                                bounds: &[],
                                default: None,
                                synthetic: false,
                            },
                        },
                    ],
                    where_predicates: &[erised::types::WherePredicate::BoundPredicate {
                        type_: erised::types::Type::Generic("U"),
                        bounds: &[erised::types::GenericBound::TraitBound {
                            trait_: erised::types::Path {
                                name: "TryFrom",
                                prefix: "",
                                target: erised::types::Identifiable::Summary(|| {
                                    erised::types::ItemSummary {
                                        krate: || erised::types::ExternalCrate {
                                            name: "core",
                                            html_root_url: Some(
                                                "https://doc.rust-lang.org/nightly/",
                                            ),
                                        },
                                        path: &["core", "convert", "TryFrom"],
                                        kind: erised::types::ItemKind::Trait,
                                    }
                                }),
                                args: Some(|| erised::types::GenericArgs::AngleBracketed {
                                    args: &[erised::types::GenericArg::Type(
                                        erised::types::Type::Generic("T"),
                                    )],
                                    bindings: &[],
                                }),
                            },
                            generic_params: &[],
                            modifier: erised::types::TraitBoundModifier::None,
                        }],
                        generic_params: &[],
                    }],
                },
                provided_trait_methods: &[],
                trait_: Some(erised::types::Path {
                    name: "TryInto",
                    prefix: "",
                    target: erised::types::Identifiable::Summary(|| erised::types::ItemSummary {
                        krate: || erised::types::ExternalCrate {
                            name: "core",
                            html_root_url: Some("https://doc.rust-lang.org/nightly/"),
                        },
                        path: &["core", "convert", "TryInto"],
                        kind: erised::types::ItemKind::Trait,
                    }),
                    args: Some(|| erised::types::GenericArgs::AngleBracketed {
                        args: &[erised::types::GenericArg::Type(
                            erised::types::Type::Generic("U"),
                        )],
                        bindings: &[],
                    }),
                }),
                for_: erised::types::Type::ResolvedPath(erised::types::Path {
                    name: "MyStruct",
                    prefix: "",
                    target: erised::types::Identifiable::Item(|| {
                        <crate::MyStruct as Reflect>::TYPE_INFO
                    }),
                    args: Some(|| erised::types::GenericArgs::AngleBracketed {
                        args: &[],
                        bindings: &[],
                    }),
                }),
                items: &[
                    || erised::types::Item::AssocType {
                        meta: erised::types::ItemMeta {
                            krate: || erised::types::ExternalCrate {
                                name: "core",
                                html_root_url: Some("https://doc.rust-lang.org/nightly/"),
                            },
                            summary: None,
                            span: None,
                            visibility: erised::types::Visibility::Default,
                            docs: None,
                            attrs: &[],
                            deprecation: None,
                        },
                        generics: erised::types::Generics {
                            params: &[],
                            where_predicates: &[],
                        },
                        bounds: &[],
                        default: Some(erised::types::Type::QualifiedPath {
                            name: "Error",
                            args: || erised::types::GenericArgs::AngleBracketed {
                                args: &[],
                                bindings: &[],
                            },
                            self_type: || erised::types::Type::Generic("U"),
                            trait_: erised::types::Path {
                                name: "TryFrom",
                                prefix: "",
                                target: erised::types::Identifiable::Summary(|| {
                                    erised::types::ItemSummary {
                                        krate: || erised::types::ExternalCrate {
                                            name: "core",
                                            html_root_url: Some(
                                                "https://doc.rust-lang.org/nightly/",
                                            ),
                                        },
                                        path: &["core", "convert", "TryFrom"],
                                        kind: erised::types::ItemKind::Trait,
                                    }
                                }),
                                args: Some(|| erised::types::GenericArgs::AngleBracketed {
                                    args: &[erised::types::GenericArg::Type(
                                        erised::types::Type::Generic("T"),
                                    )],
                                    bindings: &[],
                                }),
                            },
                        }),
                    },
                    || {
                        erised::types::Item::Function(erised::types::Function {
                            name: "try_into",
                            meta: erised::types::ItemMeta {
                                krate: || erised::types::ExternalCrate {
                                    name: "core",
                                    html_root_url: Some("https://doc.rust-lang.org/nightly/"),
                                },
                                summary: None,
                                span: None,
                                visibility: erised::types::Visibility::Default,
                                docs: None,
                                attrs: &[],
                                deprecation: None,
                            },
                            decl: erised::types::FnDecl {
                                inputs: &[erised::types::FnInput {
                                    pat: "self",
                                    ty: erised::types::Type::Generic("Self"),
                                }],
                                output: Some(erised::types::Type::ResolvedPath(
                                    erised::types::Path {
                                        name: "Result",
                                        prefix: "",
                                        target: erised::types::Identifiable::Summary(|| {
                                            erised::types::ItemSummary {
                                                krate: || erised::types::ExternalCrate {
                                                    name: "core",
                                                    html_root_url: Some(
                                                        "https://doc.rust-lang.org/nightly/",
                                                    ),
                                                },
                                                path: &["core", "result", "Result"],
                                                kind: erised::types::ItemKind::Enum,
                                            }
                                        }),
                                        args: Some(|| erised::types::GenericArgs::AngleBracketed {
                                            args: &[
                                                erised::types::GenericArg::Type(
                                                    erised::types::Type::Generic("U"),
                                                ),
                                                erised::types::GenericArg::Type(
                                                    erised::types::Type::QualifiedPath {
                                                        name: "Error",
                                                        args: || {
                                                            erised :: types :: GenericArgs :: AngleBracketed { args : & [] , bindings : & [] }
                                                        },
                                                        self_type: || {
                                                            erised::types::Type::Generic("U")
                                                        },
                                                        trait_: erised::types::Path {
                                                            name: "TryFrom",
                                                            prefix: "",
                                                            target:
                                                                erised::types::Identifiable::Summary(
                                                                    || {
                                                                        erised :: types :: ItemSummary { krate : || erised :: types :: ExternalCrate { name : "core" , html_root_url : Some ("https://doc.rust-lang.org/nightly/") } , path : & ["core" , "convert" , "TryFrom"] , kind : erised :: types :: ItemKind :: Trait }
                                                                    },
                                                                ),
                                                            args: Some(|| {
                                                                erised :: types :: GenericArgs :: AngleBracketed { args : & [erised :: types :: GenericArg :: Type (erised :: types :: Type :: Generic ("T"))] , bindings : & [] }
                                                            }),
                                                        },
                                                    },
                                                ),
                                            ],
                                            bindings: &[],
                                        }),
                                    },
                                )),
                                c_variadic: false,
                            },
                            generics: erised::types::Generics {
                                params: &[],
                                where_predicates: &[],
                            },
                            header: erised::types::Header {
                                const_: false,
                                unsafe_: false,
                                async_: false,
                                abi: erised::types::Abi::Rust,
                            },
                            has_body: true,
                        })
                    },
                ],
                negative: false,
                synthetic: false,
                blanket_impl: Some(erised::types::Type::Generic("T")),
            })
        },
        || {
            erised::types::Item::Impl(erised::types::Impl {
                meta: erised::types::ItemMeta {
                    krate: || erised::types::ExternalCrate {
                        name: "erised_tests",
                        html_root_url: None,
                    },
                    summary: None,
                    span: None,
                    visibility: erised::types::Visibility::Default,
                    docs: None,
                    attrs: &[],
                    deprecation: None,
                },
                is_unsafe: false,
                generics: erised::types::Generics {
                    params: &[
                        erised::types::GenericParamDef {
                            name: "T",
                            kind: erised::types::GenericParamDefKind::Type {
                                bounds: &[],
                                default: None,
                                synthetic: false,
                            },
                        },
                        erised::types::GenericParamDef {
                            name: "U",
                            kind: erised::types::GenericParamDefKind::Type {
                                bounds: &[],
                                default: None,
                                synthetic: false,
                            },
                        },
                    ],
                    where_predicates: &[erised::types::WherePredicate::BoundPredicate {
                        type_: erised::types::Type::Generic("U"),
                        bounds: &[erised::types::GenericBound::TraitBound {
                            trait_: erised::types::Path {
                                name: "TryFrom",
                                prefix: "",
                                target: erised::types::Identifiable::Summary(|| {
                                    erised::types::ItemSummary {
                                        krate: || erised::types::ExternalCrate {
                                            name: "core",
                                            html_root_url: Some(
                                                "https://doc.rust-lang.org/nightly/",
                                            ),
                                        },
                                        path: &["core", "convert", "TryFrom"],
                                        kind: erised::types::ItemKind::Trait,
                                    }
                                }),
                                args: Some(|| erised::types::GenericArgs::AngleBracketed {
                                    args: &[erised::types::GenericArg::Type(
                                        erised::types::Type::Generic("T"),
                                    )],
                                    bindings: &[],
                                }),
                            },
                            generic_params: &[],
                            modifier: erised::types::TraitBoundModifier::None,
                        }],
                        generic_params: &[],
                    }],
                },
                provided_trait_methods: &[],
                trait_: Some(erised::types::Path {
                    name: "TryInto",
                    prefix: "",
                    target: erised::types::Identifiable::Summary(|| erised::types::ItemSummary {
                        krate: || erised::types::ExternalCrate {
                            name: "core",
                            html_root_url: Some("https://doc.rust-lang.org/nightly/"),
                        },
                        path: &["core", "convert", "TryInto"],
                        kind: erised::types::ItemKind::Trait,
                    }),
                    args: Some(|| erised::types::GenericArgs::AngleBracketed {
                        args: &[erised::types::GenericArg::Type(
                            erised::types::Type::Generic("U"),
                        )],
                        bindings: &[],
                    }),
                }),
                for_: erised::types::Type::ResolvedPath(erised::types::Path {
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
                items: &[
                    || erised::types::Item::AssocType {
                        meta: erised::types::ItemMeta {
                            krate: || erised::types::ExternalCrate {
                                name: "core",
                                html_root_url: Some("https://doc.rust-lang.org/nightly/"),
                            },
                            summary: None,
                            span: None,
                            visibility: erised::types::Visibility::Default,
                            docs: None,
                            attrs: &[],
                            deprecation: None,
                        },
                        generics: erised::types::Generics {
                            params: &[],
                            where_predicates: &[],
                        },
                        bounds: &[],
                        default: Some(erised::types::Type::QualifiedPath {
                            name: "Error",
                            args: || erised::types::GenericArgs::AngleBracketed {
                                args: &[],
                                bindings: &[],
                            },
                            self_type: || erised::types::Type::Generic("U"),
                            trait_: erised::types::Path {
                                name: "TryFrom",
                                prefix: "",
                                target: erised::types::Identifiable::Summary(|| {
                                    erised::types::ItemSummary {
                                        krate: || erised::types::ExternalCrate {
                                            name: "core",
                                            html_root_url: Some(
                                                "https://doc.rust-lang.org/nightly/",
                                            ),
                                        },
                                        path: &["core", "convert", "TryFrom"],
                                        kind: erised::types::ItemKind::Trait,
                                    }
                                }),
                                args: Some(|| erised::types::GenericArgs::AngleBracketed {
                                    args: &[erised::types::GenericArg::Type(
                                        erised::types::Type::Generic("T"),
                                    )],
                                    bindings: &[],
                                }),
                            },
                        }),
                    },
                    || {
                        erised::types::Item::Function(erised::types::Function {
                            name: "try_into",
                            meta: erised::types::ItemMeta {
                                krate: || erised::types::ExternalCrate {
                                    name: "core",
                                    html_root_url: Some("https://doc.rust-lang.org/nightly/"),
                                },
                                summary: None,
                                span: None,
                                visibility: erised::types::Visibility::Default,
                                docs: None,
                                attrs: &[],
                                deprecation: None,
                            },
                            decl: erised::types::FnDecl {
                                inputs: &[erised::types::FnInput {
                                    pat: "self",
                                    ty: erised::types::Type::Generic("Self"),
                                }],
                                output: Some(erised::types::Type::ResolvedPath(
                                    erised::types::Path {
                                        name: "Result",
                                        prefix: "",
                                        target: erised::types::Identifiable::Summary(|| {
                                            erised::types::ItemSummary {
                                                krate: || erised::types::ExternalCrate {
                                                    name: "core",
                                                    html_root_url: Some(
                                                        "https://doc.rust-lang.org/nightly/",
                                                    ),
                                                },
                                                path: &["core", "result", "Result"],
                                                kind: erised::types::ItemKind::Enum,
                                            }
                                        }),
                                        args: Some(|| erised::types::GenericArgs::AngleBracketed {
                                            args: &[
                                                erised::types::GenericArg::Type(
                                                    erised::types::Type::Generic("U"),
                                                ),
                                                erised::types::GenericArg::Type(
                                                    erised::types::Type::QualifiedPath {
                                                        name: "Error",
                                                        args: || {
                                                            erised :: types :: GenericArgs :: AngleBracketed { args : & [] , bindings : & [] }
                                                        },
                                                        self_type: || {
                                                            erised::types::Type::Generic("U")
                                                        },
                                                        trait_: erised::types::Path {
                                                            name: "TryFrom",
                                                            prefix: "",
                                                            target:
                                                                erised::types::Identifiable::Summary(
                                                                    || {
                                                                        erised :: types :: ItemSummary { krate : || erised :: types :: ExternalCrate { name : "core" , html_root_url : Some ("https://doc.rust-lang.org/nightly/") } , path : & ["core" , "convert" , "TryFrom"] , kind : erised :: types :: ItemKind :: Trait }
                                                                    },
                                                                ),
                                                            args: Some(|| {
                                                                erised :: types :: GenericArgs :: AngleBracketed { args : & [erised :: types :: GenericArg :: Type (erised :: types :: Type :: Generic ("T"))] , bindings : & [] }
                                                            }),
                                                        },
                                                    },
                                                ),
                                            ],
                                            bindings: &[],
                                        }),
                                    },
                                )),
                                c_variadic: false,
                            },
                            generics: erised::types::Generics {
                                params: &[],
                                where_predicates: &[],
                            },
                            header: erised::types::Header {
                                const_: false,
                                unsafe_: false,
                                async_: false,
                                abi: erised::types::Abi::Rust,
                            },
                            has_body: true,
                        })
                    },
                ],
                negative: false,
                synthetic: false,
                blanket_impl: Some(erised::types::Type::Generic("T")),
            })
        },
        || {
            erised::types::Item::Impl(erised::types::Impl {
                meta: erised::types::ItemMeta {
                    krate: || erised::types::ExternalCrate {
                        name: "erised_tests",
                        html_root_url: None,
                    },
                    summary: None,
                    span: None,
                    visibility: erised::types::Visibility::Default,
                    docs: None,
                    attrs: &[],
                    deprecation: None,
                },
                is_unsafe: false,
                generics: erised::types::Generics {
                    params: &[
                        erised::types::GenericParamDef {
                            name: "T",
                            kind: erised::types::GenericParamDefKind::Type {
                                bounds: &[],
                                default: None,
                                synthetic: false,
                            },
                        },
                        erised::types::GenericParamDef {
                            name: "U",
                            kind: erised::types::GenericParamDefKind::Type {
                                bounds: &[],
                                default: None,
                                synthetic: false,
                            },
                        },
                    ],
                    where_predicates: &[erised::types::WherePredicate::BoundPredicate {
                        type_: erised::types::Type::Generic("U"),
                        bounds: &[erised::types::GenericBound::TraitBound {
                            trait_: erised::types::Path {
                                name: "Into",
                                prefix: "",
                                target: erised::types::Identifiable::Summary(|| {
                                    erised::types::ItemSummary {
                                        krate: || erised::types::ExternalCrate {
                                            name: "core",
                                            html_root_url: Some(
                                                "https://doc.rust-lang.org/nightly/",
                                            ),
                                        },
                                        path: &["core", "convert", "Into"],
                                        kind: erised::types::ItemKind::Trait,
                                    }
                                }),
                                args: Some(|| erised::types::GenericArgs::AngleBracketed {
                                    args: &[erised::types::GenericArg::Type(
                                        erised::types::Type::Generic("T"),
                                    )],
                                    bindings: &[],
                                }),
                            },
                            generic_params: &[],
                            modifier: erised::types::TraitBoundModifier::None,
                        }],
                        generic_params: &[],
                    }],
                },
                provided_trait_methods: &[],
                trait_: Some(erised::types::Path {
                    name: "TryFrom",
                    prefix: "",
                    target: erised::types::Identifiable::Summary(|| erised::types::ItemSummary {
                        krate: || erised::types::ExternalCrate {
                            name: "core",
                            html_root_url: Some("https://doc.rust-lang.org/nightly/"),
                        },
                        path: &["core", "convert", "TryFrom"],
                        kind: erised::types::ItemKind::Trait,
                    }),
                    args: Some(|| erised::types::GenericArgs::AngleBracketed {
                        args: &[erised::types::GenericArg::Type(
                            erised::types::Type::Generic("U"),
                        )],
                        bindings: &[],
                    }),
                }),
                for_: erised::types::Type::ResolvedPath(erised::types::Path {
                    name: "MyStruct",
                    prefix: "",
                    target: erised::types::Identifiable::Item(|| {
                        <crate::MyStruct as Reflect>::TYPE_INFO
                    }),
                    args: Some(|| erised::types::GenericArgs::AngleBracketed {
                        args: &[],
                        bindings: &[],
                    }),
                }),
                items: &[
                    || erised::types::Item::AssocType {
                        meta: erised::types::ItemMeta {
                            krate: || erised::types::ExternalCrate {
                                name: "core",
                                html_root_url: Some("https://doc.rust-lang.org/nightly/"),
                            },
                            summary: None,
                            span: None,
                            visibility: erised::types::Visibility::Default,
                            docs: None,
                            attrs: &[],
                            deprecation: None,
                        },
                        generics: erised::types::Generics {
                            params: &[],
                            where_predicates: &[],
                        },
                        bounds: &[],
                        default: Some(erised::types::Type::ResolvedPath(erised::types::Path {
                            name: "Infallible",
                            prefix: "",
                            target: erised::types::Identifiable::Summary(|| {
                                erised::types::ItemSummary {
                                    krate: || erised::types::ExternalCrate {
                                        name: "core",
                                        html_root_url: Some("https://doc.rust-lang.org/nightly/"),
                                    },
                                    path: &["core", "convert", "Infallible"],
                                    kind: erised::types::ItemKind::Enum,
                                }
                            }),
                            args: Some(|| erised::types::GenericArgs::AngleBracketed {
                                args: &[],
                                bindings: &[],
                            }),
                        })),
                    },
                    || {
                        erised::types::Item::Function(erised::types::Function {
                            name: "try_from",
                            meta: erised::types::ItemMeta {
                                krate: || erised::types::ExternalCrate {
                                    name: "core",
                                    html_root_url: Some("https://doc.rust-lang.org/nightly/"),
                                },
                                summary: None,
                                span: None,
                                visibility: erised::types::Visibility::Default,
                                docs: None,
                                attrs: &[],
                                deprecation: None,
                            },
                            decl: erised::types::FnDecl {
                                inputs: &[erised::types::FnInput {
                                    pat: "value",
                                    ty: erised::types::Type::Generic("U"),
                                }],
                                output: Some(erised::types::Type::ResolvedPath(
                                    erised::types::Path {
                                        name: "Result",
                                        prefix: "",
                                        target: erised::types::Identifiable::Summary(|| {
                                            erised::types::ItemSummary {
                                                krate: || erised::types::ExternalCrate {
                                                    name: "core",
                                                    html_root_url: Some(
                                                        "https://doc.rust-lang.org/nightly/",
                                                    ),
                                                },
                                                path: &["core", "result", "Result"],
                                                kind: erised::types::ItemKind::Enum,
                                            }
                                        }),
                                        args: Some(|| erised::types::GenericArgs::AngleBracketed {
                                            args: &[
                                                erised::types::GenericArg::Type(
                                                    erised::types::Type::Generic("T"),
                                                ),
                                                erised::types::GenericArg::Type(
                                                    erised::types::Type::QualifiedPath {
                                                        name: "Error",
                                                        args: || {
                                                            erised :: types :: GenericArgs :: AngleBracketed { args : & [] , bindings : & [] }
                                                        },
                                                        self_type: || {
                                                            erised::types::Type::Generic("T")
                                                        },
                                                        trait_: erised::types::Path {
                                                            name: "TryFrom",
                                                            prefix: "",
                                                            target:
                                                                erised::types::Identifiable::Summary(
                                                                    || {
                                                                        erised :: types :: ItemSummary { krate : || erised :: types :: ExternalCrate { name : "core" , html_root_url : Some ("https://doc.rust-lang.org/nightly/") } , path : & ["core" , "convert" , "TryFrom"] , kind : erised :: types :: ItemKind :: Trait }
                                                                    },
                                                                ),
                                                            args: Some(|| {
                                                                erised :: types :: GenericArgs :: AngleBracketed { args : & [erised :: types :: GenericArg :: Type (erised :: types :: Type :: Generic ("U"))] , bindings : & [] }
                                                            }),
                                                        },
                                                    },
                                                ),
                                            ],
                                            bindings: &[],
                                        }),
                                    },
                                )),
                                c_variadic: false,
                            },
                            generics: erised::types::Generics {
                                params: &[],
                                where_predicates: &[],
                            },
                            header: erised::types::Header {
                                const_: false,
                                unsafe_: false,
                                async_: false,
                                abi: erised::types::Abi::Rust,
                            },
                            has_body: true,
                        })
                    },
                ],
                negative: false,
                synthetic: false,
                blanket_impl: Some(erised::types::Type::Generic("T")),
            })
        },
        || {
            erised::types::Item::Impl(erised::types::Impl {
                meta: erised::types::ItemMeta {
                    krate: || erised::types::ExternalCrate {
                        name: "erised_tests",
                        html_root_url: None,
                    },
                    summary: None,
                    span: None,
                    visibility: erised::types::Visibility::Default,
                    docs: None,
                    attrs: &[],
                    deprecation: None,
                },
                is_unsafe: false,
                generics: erised::types::Generics {
                    params: &[
                        erised::types::GenericParamDef {
                            name: "T",
                            kind: erised::types::GenericParamDefKind::Type {
                                bounds: &[],
                                default: None,
                                synthetic: false,
                            },
                        },
                        erised::types::GenericParamDef {
                            name: "U",
                            kind: erised::types::GenericParamDefKind::Type {
                                bounds: &[],
                                default: None,
                                synthetic: false,
                            },
                        },
                    ],
                    where_predicates: &[erised::types::WherePredicate::BoundPredicate {
                        type_: erised::types::Type::Generic("U"),
                        bounds: &[erised::types::GenericBound::TraitBound {
                            trait_: erised::types::Path {
                                name: "Into",
                                prefix: "",
                                target: erised::types::Identifiable::Summary(|| {
                                    erised::types::ItemSummary {
                                        krate: || erised::types::ExternalCrate {
                                            name: "core",
                                            html_root_url: Some(
                                                "https://doc.rust-lang.org/nightly/",
                                            ),
                                        },
                                        path: &["core", "convert", "Into"],
                                        kind: erised::types::ItemKind::Trait,
                                    }
                                }),
                                args: Some(|| erised::types::GenericArgs::AngleBracketed {
                                    args: &[erised::types::GenericArg::Type(
                                        erised::types::Type::Generic("T"),
                                    )],
                                    bindings: &[],
                                }),
                            },
                            generic_params: &[],
                            modifier: erised::types::TraitBoundModifier::None,
                        }],
                        generic_params: &[],
                    }],
                },
                provided_trait_methods: &[],
                trait_: Some(erised::types::Path {
                    name: "TryFrom",
                    prefix: "",
                    target: erised::types::Identifiable::Summary(|| erised::types::ItemSummary {
                        krate: || erised::types::ExternalCrate {
                            name: "core",
                            html_root_url: Some("https://doc.rust-lang.org/nightly/"),
                        },
                        path: &["core", "convert", "TryFrom"],
                        kind: erised::types::ItemKind::Trait,
                    }),
                    args: Some(|| erised::types::GenericArgs::AngleBracketed {
                        args: &[erised::types::GenericArg::Type(
                            erised::types::Type::Generic("U"),
                        )],
                        bindings: &[],
                    }),
                }),
                for_: erised::types::Type::ResolvedPath(erised::types::Path {
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
                items: &[
                    || erised::types::Item::AssocType {
                        meta: erised::types::ItemMeta {
                            krate: || erised::types::ExternalCrate {
                                name: "core",
                                html_root_url: Some("https://doc.rust-lang.org/nightly/"),
                            },
                            summary: None,
                            span: None,
                            visibility: erised::types::Visibility::Default,
                            docs: None,
                            attrs: &[],
                            deprecation: None,
                        },
                        generics: erised::types::Generics {
                            params: &[],
                            where_predicates: &[],
                        },
                        bounds: &[],
                        default: Some(erised::types::Type::ResolvedPath(erised::types::Path {
                            name: "Infallible",
                            prefix: "",
                            target: erised::types::Identifiable::Summary(|| {
                                erised::types::ItemSummary {
                                    krate: || erised::types::ExternalCrate {
                                        name: "core",
                                        html_root_url: Some("https://doc.rust-lang.org/nightly/"),
                                    },
                                    path: &["core", "convert", "Infallible"],
                                    kind: erised::types::ItemKind::Enum,
                                }
                            }),
                            args: Some(|| erised::types::GenericArgs::AngleBracketed {
                                args: &[],
                                bindings: &[],
                            }),
                        })),
                    },
                    || {
                        erised::types::Item::Function(erised::types::Function {
                            name: "try_from",
                            meta: erised::types::ItemMeta {
                                krate: || erised::types::ExternalCrate {
                                    name: "core",
                                    html_root_url: Some("https://doc.rust-lang.org/nightly/"),
                                },
                                summary: None,
                                span: None,
                                visibility: erised::types::Visibility::Default,
                                docs: None,
                                attrs: &[],
                                deprecation: None,
                            },
                            decl: erised::types::FnDecl {
                                inputs: &[erised::types::FnInput {
                                    pat: "value",
                                    ty: erised::types::Type::Generic("U"),
                                }],
                                output: Some(erised::types::Type::ResolvedPath(
                                    erised::types::Path {
                                        name: "Result",
                                        prefix: "",
                                        target: erised::types::Identifiable::Summary(|| {
                                            erised::types::ItemSummary {
                                                krate: || erised::types::ExternalCrate {
                                                    name: "core",
                                                    html_root_url: Some(
                                                        "https://doc.rust-lang.org/nightly/",
                                                    ),
                                                },
                                                path: &["core", "result", "Result"],
                                                kind: erised::types::ItemKind::Enum,
                                            }
                                        }),
                                        args: Some(|| erised::types::GenericArgs::AngleBracketed {
                                            args: &[
                                                erised::types::GenericArg::Type(
                                                    erised::types::Type::Generic("T"),
                                                ),
                                                erised::types::GenericArg::Type(
                                                    erised::types::Type::QualifiedPath {
                                                        name: "Error",
                                                        args: || {
                                                            erised :: types :: GenericArgs :: AngleBracketed { args : & [] , bindings : & [] }
                                                        },
                                                        self_type: || {
                                                            erised::types::Type::Generic("T")
                                                        },
                                                        trait_: erised::types::Path {
                                                            name: "TryFrom",
                                                            prefix: "",
                                                            target:
                                                                erised::types::Identifiable::Summary(
                                                                    || {
                                                                        erised :: types :: ItemSummary { krate : || erised :: types :: ExternalCrate { name : "core" , html_root_url : Some ("https://doc.rust-lang.org/nightly/") } , path : & ["core" , "convert" , "TryFrom"] , kind : erised :: types :: ItemKind :: Trait }
                                                                    },
                                                                ),
                                                            args: Some(|| {
                                                                erised :: types :: GenericArgs :: AngleBracketed { args : & [erised :: types :: GenericArg :: Type (erised :: types :: Type :: Generic ("U"))] , bindings : & [] }
                                                            }),
                                                        },
                                                    },
                                                ),
                                            ],
                                            bindings: &[],
                                        }),
                                    },
                                )),
                                c_variadic: false,
                            },
                            generics: erised::types::Generics {
                                params: &[],
                                where_predicates: &[],
                            },
                            header: erised::types::Header {
                                const_: false,
                                unsafe_: false,
                                async_: false,
                                abi: erised::types::Abi::Rust,
                            },
                            has_body: true,
                        })
                    },
                ],
                negative: false,
                synthetic: false,
                blanket_impl: Some(erised::types::Type::Generic("T")),
            })
        },
        || {
            erised::types::Item::Impl(erised::types::Impl {
                meta: erised::types::ItemMeta {
                    krate: || erised::types::ExternalCrate {
                        name: "erised_tests",
                        html_root_url: None,
                    },
                    summary: None,
                    span: None,
                    visibility: erised::types::Visibility::Default,
                    docs: None,
                    attrs: &[],
                    deprecation: None,
                },
                is_unsafe: false,
                generics: erised::types::Generics {
                    params: &[erised::types::GenericParamDef {
                        name: "T",
                        kind: erised::types::GenericParamDefKind::Type {
                            bounds: &[],
                            default: None,
                            synthetic: false,
                        },
                    }],
                    where_predicates: &[erised::types::WherePredicate::BoundPredicate {
                        type_: erised::types::Type::Generic("T"),
                        bounds: &[
                            erised::types::GenericBound::Outlives("'static"),
                            erised::types::GenericBound::TraitBound {
                                trait_: erised::types::Path {
                                    name: "Sized",
                                    prefix: "",
                                    target: erised::types::Identifiable::Summary(|| {
                                        erised::types::ItemSummary {
                                            krate: || erised::types::ExternalCrate {
                                                name: "core",
                                                html_root_url: Some(
                                                    "https://doc.rust-lang.org/nightly/",
                                                ),
                                            },
                                            path: &["core", "marker", "Sized"],
                                            kind: erised::types::ItemKind::Trait,
                                        }
                                    }),
                                    args: Some(|| erised::types::GenericArgs::AngleBracketed {
                                        args: &[],
                                        bindings: &[],
                                    }),
                                },
                                generic_params: &[],
                                modifier: erised::types::TraitBoundModifier::Maybe,
                            },
                        ],
                        generic_params: &[],
                    }],
                },
                provided_trait_methods: &[],
                trait_: Some(erised::types::Path {
                    name: "Any",
                    prefix: "",
                    target: erised::types::Identifiable::Summary(|| erised::types::ItemSummary {
                        krate: || erised::types::ExternalCrate {
                            name: "core",
                            html_root_url: Some("https://doc.rust-lang.org/nightly/"),
                        },
                        path: &["core", "any", "Any"],
                        kind: erised::types::ItemKind::Trait,
                    }),
                    args: Some(|| erised::types::GenericArgs::AngleBracketed {
                        args: &[],
                        bindings: &[],
                    }),
                }),
                for_: erised::types::Type::ResolvedPath(erised::types::Path {
                    name: "MyStruct",
                    prefix: "",
                    target: erised::types::Identifiable::Item(|| {
                        <crate::MyStruct as Reflect>::TYPE_INFO
                    }),
                    args: Some(|| erised::types::GenericArgs::AngleBracketed {
                        args: &[],
                        bindings: &[],
                    }),
                }),
                items: &[|| {
                    erised::types::Item::Function(erised::types::Function {
                        name: "type_id",
                        meta: erised::types::ItemMeta {
                            krate: || erised::types::ExternalCrate {
                                name: "core",
                                html_root_url: Some("https://doc.rust-lang.org/nightly/"),
                            },
                            summary: None,
                            span: None,
                            visibility: erised::types::Visibility::Default,
                            docs: None,
                            attrs: &[],
                            deprecation: None,
                        },
                        decl: erised::types::FnDecl {
                            inputs: &[erised::types::FnInput {
                                pat: "self",
                                ty: erised::types::Type::BorrowedRef {
                                    lifetime: None,
                                    mutable: false,
                                    type_: || erised::types::Type::Generic("Self"),
                                },
                            }],
                            output: Some(erised::types::Type::ResolvedPath(erised::types::Path {
                                name: "TypeId",
                                prefix: "",
                                target: erised::types::Identifiable::Summary(|| {
                                    erised::types::ItemSummary {
                                        krate: || erised::types::ExternalCrate {
                                            name: "core",
                                            html_root_url: Some(
                                                "https://doc.rust-lang.org/nightly/",
                                            ),
                                        },
                                        path: &["core", "any", "TypeId"],
                                        kind: erised::types::ItemKind::Struct,
                                    }
                                }),
                                args: Some(|| erised::types::GenericArgs::AngleBracketed {
                                    args: &[],
                                    bindings: &[],
                                }),
                            })),
                            c_variadic: false,
                        },
                        generics: erised::types::Generics {
                            params: &[],
                            where_predicates: &[],
                        },
                        header: erised::types::Header {
                            const_: false,
                            unsafe_: false,
                            async_: false,
                            abi: erised::types::Abi::Rust,
                        },
                        has_body: true,
                    })
                }],
                negative: false,
                synthetic: false,
                blanket_impl: Some(erised::types::Type::Generic("T")),
            })
        },
        || {
            erised::types::Item::Impl(erised::types::Impl {
                meta: erised::types::ItemMeta {
                    krate: || erised::types::ExternalCrate {
                        name: "erised_tests",
                        html_root_url: None,
                    },
                    summary: None,
                    span: None,
                    visibility: erised::types::Visibility::Default,
                    docs: None,
                    attrs: &[],
                    deprecation: None,
                },
                is_unsafe: false,
                generics: erised::types::Generics {
                    params: &[erised::types::GenericParamDef {
                        name: "T",
                        kind: erised::types::GenericParamDefKind::Type {
                            bounds: &[],
                            default: None,
                            synthetic: false,
                        },
                    }],
                    where_predicates: &[erised::types::WherePredicate::BoundPredicate {
                        type_: erised::types::Type::Generic("T"),
                        bounds: &[
                            erised::types::GenericBound::Outlives("'static"),
                            erised::types::GenericBound::TraitBound {
                                trait_: erised::types::Path {
                                    name: "Sized",
                                    prefix: "",
                                    target: erised::types::Identifiable::Summary(|| {
                                        erised::types::ItemSummary {
                                            krate: || erised::types::ExternalCrate {
                                                name: "core",
                                                html_root_url: Some(
                                                    "https://doc.rust-lang.org/nightly/",
                                                ),
                                            },
                                            path: &["core", "marker", "Sized"],
                                            kind: erised::types::ItemKind::Trait,
                                        }
                                    }),
                                    args: Some(|| erised::types::GenericArgs::AngleBracketed {
                                        args: &[],
                                        bindings: &[],
                                    }),
                                },
                                generic_params: &[],
                                modifier: erised::types::TraitBoundModifier::Maybe,
                            },
                        ],
                        generic_params: &[],
                    }],
                },
                provided_trait_methods: &[],
                trait_: Some(erised::types::Path {
                    name: "Any",
                    prefix: "",
                    target: erised::types::Identifiable::Summary(|| erised::types::ItemSummary {
                        krate: || erised::types::ExternalCrate {
                            name: "core",
                            html_root_url: Some("https://doc.rust-lang.org/nightly/"),
                        },
                        path: &["core", "any", "Any"],
                        kind: erised::types::ItemKind::Trait,
                    }),
                    args: Some(|| erised::types::GenericArgs::AngleBracketed {
                        args: &[],
                        bindings: &[],
                    }),
                }),
                for_: erised::types::Type::ResolvedPath(erised::types::Path {
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
                items: &[|| {
                    erised::types::Item::Function(erised::types::Function {
                        name: "type_id",
                        meta: erised::types::ItemMeta {
                            krate: || erised::types::ExternalCrate {
                                name: "core",
                                html_root_url: Some("https://doc.rust-lang.org/nightly/"),
                            },
                            summary: None,
                            span: None,
                            visibility: erised::types::Visibility::Default,
                            docs: None,
                            attrs: &[],
                            deprecation: None,
                        },
                        decl: erised::types::FnDecl {
                            inputs: &[erised::types::FnInput {
                                pat: "self",
                                ty: erised::types::Type::BorrowedRef {
                                    lifetime: None,
                                    mutable: false,
                                    type_: || erised::types::Type::Generic("Self"),
                                },
                            }],
                            output: Some(erised::types::Type::ResolvedPath(erised::types::Path {
                                name: "TypeId",
                                prefix: "",
                                target: erised::types::Identifiable::Summary(|| {
                                    erised::types::ItemSummary {
                                        krate: || erised::types::ExternalCrate {
                                            name: "core",
                                            html_root_url: Some(
                                                "https://doc.rust-lang.org/nightly/",
                                            ),
                                        },
                                        path: &["core", "any", "TypeId"],
                                        kind: erised::types::ItemKind::Struct,
                                    }
                                }),
                                args: Some(|| erised::types::GenericArgs::AngleBracketed {
                                    args: &[],
                                    bindings: &[],
                                }),
                            })),
                            c_variadic: false,
                        },
                        generics: erised::types::Generics {
                            params: &[],
                            where_predicates: &[],
                        },
                        header: erised::types::Header {
                            const_: false,
                            unsafe_: false,
                            async_: false,
                            abi: erised::types::Abi::Rust,
                        },
                        has_body: true,
                    })
                }],
                negative: false,
                synthetic: false,
                blanket_impl: Some(erised::types::Type::Generic("T")),
            })
        },
    ],
    summaries: &[],
    external_crates: &[],
};

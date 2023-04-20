#[allow(dead_code)]
pub fn to_reflect(_: impl crate::MyTrait, _: crate::PlainStruct) -> erised::ToReflect {
    erised::ToReflect
}
pub trait Reflect {
    const TYPE_INFO: erised::TypeInfo;
}
impl Reflect for crate::PlainStruct {
    const TYPE_INFO: erised::TypeInfo = erised::TypeInfo::Struct(erised::StructInfo {
        name: "crate::PlainStruct",
        docs: None,
        fields: &[
            erised::StructFieldInfo {
                name: "tuple_str",
                docs: Some("This is comment"),
                ty: <crate::TupleStruct as Reflect>::TYPE_INFO,
            },
            erised::StructFieldInfo {
                name: "unit",
                docs: None,
                ty: <crate::UnitStruct as Reflect>::TYPE_INFO,
            },
            erised::StructFieldInfo {
                name: "tuple",
                docs: None,
                ty: erised::TypeInfo::Tuple(erised::TupleInfo {
                    fields: &[
                        erised::TypeInfo::Primitive(erised::Primitive {
                            name: "alloc::string::String",
                            docs: None,
                        }),
                        erised::TypeInfo::Primitive(erised::Primitive {
                            name: "alloc::string::String",
                            docs: None,
                        }),
                    ],
                }),
            },
            erised::StructFieldInfo {
                name: "array",
                docs: None,
                ty: erised::TypeInfo::Array(erised::ArrayInfo {
                    len: 4usize,
                    ty: || {
                        erised::TypeInfo::Primitive(erised::Primitive {
                            name: "i32",
                            docs: None,
                        })
                    },
                }),
            },
            erised::StructFieldInfo {
                name: "primitive",
                docs: None,
                ty: erised::TypeInfo::Primitive(erised::Primitive {
                    name: "i32",
                    docs: None,
                }),
            },
            erised::StructFieldInfo {
                name: "recurse",
                docs: None,
                ty: erised::TypeInfo::WithGeneric(erised::WithGenericInfo {
                    name: "alloc::boxed::Box",
                    args: || &[<crate::PlainStruct as Reflect>::TYPE_INFO],
                    bindings: &[],
                }),
            },
            erised::StructFieldInfo {
                name: "borrowed",
                docs: None,
                ty: erised::TypeInfo::Borrow(erised::BorrowInfo {
                    lifetime: Some("static"),
                    mutable: false,
                    ty: || {
                        erised::TypeInfo::Primitive(erised::Primitive {
                            name: "str",
                            docs: None,
                        })
                    },
                }),
            },
            erised::StructFieldInfo {
                name: "mut_borrowed",
                docs: None,
                ty: erised::TypeInfo::Borrow(erised::BorrowInfo {
                    lifetime: Some("static"),
                    mutable: true,
                    ty: || {
                        erised::TypeInfo::Primitive(erised::Primitive {
                            name: "str",
                            docs: None,
                        })
                    },
                }),
            },
            erised::StructFieldInfo {
                name: "enumerated",
                docs: None,
                ty: <crate::Enum as Reflect>::TYPE_INFO,
            },
            erised::StructFieldInfo {
                name: "enum_with_disc",
                docs: None,
                ty: <crate::EnumWithDisc as Reflect>::TYPE_INFO,
            },
        ],
    });
}
impl Reflect for crate::EnumWithDisc {
    const TYPE_INFO: erised::TypeInfo = erised::TypeInfo::Enum(erised::EnumInfo {
        name: "crate::EnumWithDisc",
        docs: None,
        variants: &[erised::VariantInfo::Unit {
            name: "WithDiscr",
            docs: None,
            discr: Some("2"),
        }],
    });
}
impl Reflect for crate::Enum {
    const TYPE_INFO: erised::TypeInfo = erised::TypeInfo::Enum(erised::EnumInfo {
        name: "crate::Enum",
        docs: None,
        variants: &[
            erised::VariantInfo::Unit {
                name: "Unit",
                docs: Some("This is variant doc"),
                discr: None,
            },
            erised::VariantInfo::Tuple {
                name: "One",
                docs: None,
                fields: &[erised::TypeInfo::Primitive(erised::Primitive {
                    name: "i32",
                    docs: None,
                })],
            },
            erised::VariantInfo::Tuple {
                name: "Seq",
                docs: None,
                fields: &[
                    erised::TypeInfo::Primitive(erised::Primitive {
                        name: "i32",
                        docs: None,
                    }),
                    erised::TypeInfo::Primitive(erised::Primitive {
                        name: "i32",
                        docs: None,
                    }),
                ],
            },
            erised::VariantInfo::Struct {
                name: "Map",
                docs: None,
                fields: &[
                    erised::StructFieldInfo {
                        name: "a",
                        docs: None,
                        ty: erised::TypeInfo::Primitive(erised::Primitive {
                            name: "i32",
                            docs: None,
                        }),
                    },
                    erised::StructFieldInfo {
                        name: "b",
                        docs: Some("This is field doc"),
                        ty: erised::TypeInfo::Primitive(erised::Primitive {
                            name: "i32",
                            docs: None,
                        }),
                    },
                ],
            },
        ],
    });
}
impl Reflect for crate::UnitStruct {
    const TYPE_INFO: erised::TypeInfo = erised::TypeInfo::Primitive(erised::Primitive {
        name: "crate::UnitStruct",
        docs: Some("This is unit comment"),
    });
}
impl Reflect for crate::TupleStruct {
    const TYPE_INFO: erised::TypeInfo = erised::TypeInfo::TupleStruct(erised::TupleStructInfo {
        name: "crate::TupleStruct",
        docs: Some("This is struct comment"),
        fields: &[erised::TypeInfo::Primitive(erised::Primitive {
            name: "alloc::string::String",
            docs: None,
        })],
    });
}
pub enum MyTrait {}
impl Reflect for MyTrait {
    const TYPE_INFO: erised::TypeInfo = erised::TypeInfo::Trait(erised::TraitInfo {
        name: "crate::MyTrait",
        docs: Some("This is trait doc"),
        methods: &[
            erised::FunctionInfo {
                name: "my_trait_method",
                docs: Some("This is method doc"),
                inputs: &[
                    erised::FunctionArg {
                        name: "self",
                        ty: erised::TypeInfo::Borrow(erised::BorrowInfo {
                            lifetime: None,
                            mutable: false,
                            ty: || erised::TypeInfo::Receiver,
                        }),
                    },
                    erised::FunctionArg {
                        name: "arg",
                        ty: <crate::PlainStruct as Reflect>::TYPE_INFO,
                    },
                ],
                output: Some(<crate::EnumWithDisc as Reflect>::TYPE_INFO),
                generics: &[],
            },
            erised::FunctionInfo {
                name: "static_method",
                docs: None,
                inputs: &[erised::FunctionArg {
                    name: "arg",
                    ty: <crate::PlainStruct as Reflect>::TYPE_INFO,
                }],
                output: None,
                generics: &[],
            },
            erised::FunctionInfo {
                name: "my_generic_method",
                docs: None,
                inputs: &[
                    erised::FunctionArg {
                        name: "self",
                        ty: erised::TypeInfo::Borrow(erised::BorrowInfo {
                            lifetime: None,
                            mutable: false,
                            ty: || erised::TypeInfo::Receiver,
                        }),
                    },
                    erised::FunctionArg {
                        name: "arg",
                        ty: erised::TypeInfo::Generic(erised::GenericInfo { name: "F" }),
                    },
                ],
                output: None,
                generics: &[erised::FunctionGeneric {
                    name: "F",
                    kind: erised::GenericParamKind::Type(erised::GenericParamType {
                        bounds: &[erised::GenericBound {
                            trait_: || <MyTrait as Reflect>::TYPE_INFO.as_trait().unwrap(),
                        }],
                    }),
                }],
            },
            erised::FunctionInfo {
                name: "my_generic_method_with_where",
                docs: None,
                inputs: &[
                    erised::FunctionArg {
                        name: "self",
                        ty: erised::TypeInfo::Borrow(erised::BorrowInfo {
                            lifetime: None,
                            mutable: false,
                            ty: || erised::TypeInfo::Receiver,
                        }),
                    },
                    erised::FunctionArg {
                        name: "arg",
                        ty: erised::TypeInfo::Generic(erised::GenericInfo { name: "F" }),
                    },
                ],
                output: None,
                generics: &[erised::FunctionGeneric {
                    name: "F",
                    kind: erised::GenericParamKind::Type(erised::GenericParamType { bounds: &[] }),
                }],
            },
        ],
        consts: &[erised::ConstInfo {
            name: "FOO",
            docs: Some("This is const doc"),
            ty: erised::TypeInfo::Borrow(erised::BorrowInfo {
                lifetime: Some("static"),
                mutable: false,
                ty: || {
                    erised::TypeInfo::Primitive(erised::Primitive {
                        name: "str",
                        docs: None,
                    })
                },
            }),
        }],
        assoc_types: &[erised::AssocTypeInfo {
            name: "Foo",
            docs: Some("This is type doc"),
        }],
    });
}

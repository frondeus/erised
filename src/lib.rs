#[allow(dead_code)]
pub struct PlainStruct {
    /// This is comment
    tuple_str: TupleStruct,
    unit: UnitStruct,

    tuple: (String, String),
    array: [i32; 4],
    primitive: i32,
    recurse: Box<PlainStruct>,

    borrowed: &'static str,
    mut_borrowed: &'static mut str,

    enumerated: Enum,
    enum_with_disc: EnumWithDisc,
}

/// This is struct comment
pub struct TupleStruct(String);

/// This is unit comment
pub struct UnitStruct;

pub enum Enum {
    /// This is variant doc
    Unit,
    One(i32),
    Seq(i32, i32),
    Map {
        a: i32,
        /// This is field doc
        b: i32,
    },
}

pub enum EnumWithDisc {
    WithDiscr = 2,
}

pub trait MyTrait {
    type Foo;
    const FOO: &'static str;
    fn my_trait_method(&self, arg: PlainStruct) -> EnumWithDisc;
    fn static_method(arg: PlainStruct);
}

pub fn to_reflect(_: impl MyTrait, _: PlainStruct) -> erised::ToReflect {
    erised::ToReflect
}

#[test]
fn test() {
    use crate::reflected::Reflect;
    match PlainStruct::TYPE_INFO {
        erised::TypeInfo::Struct(strukt) => {
            assert_eq!(strukt.name, "crate::PlainStruct");
            assert_eq!(strukt.fields.len(), 10);
            assert_eq!(
                strukt.fields[0],
                erised::StructFieldInfo {
                    name: "tuple_str",
                    docs: Some("This is comment"),
                    ty: erised::TypeInfo::TupleStruct(erised::TupleStructInfo {
                        name: "crate::TupleStruct",
                        docs: Some("This is struct comment"),
                        fields: &[erised::TypeInfo::Primitive(erised::Primitive {
                            name: "alloc::string::String",
                            docs: None
                        })]
                    })
                }
            );
            assert_eq!(
                strukt.fields[1],
                erised::StructFieldInfo {
                    name: "unit",
                    docs: None,
                    ty: erised::TypeInfo::Primitive(erised::Primitive {
                        docs: Some("This is unit comment"),
                        name: "crate::UnitStruct"
                    })
                }
            );
            assert_eq!(
                strukt.fields[2],
                erised::StructFieldInfo {
                    name: "tuple",
                    docs: None,
                    ty: erised::TypeInfo::Tuple(erised::TupleInfo {
                        fields: &[
                            erised::TypeInfo::Primitive(erised::Primitive {
                                name: "alloc::string::String",
                                docs: None
                            }),
                            erised::TypeInfo::Primitive(erised::Primitive {
                                name: "alloc::string::String",
                                docs: None
                            })
                        ]
                    })
                }
            );
            assert_eq!(strukt.fields[3].name, "array");
            let array_ty = match strukt.fields[3].ty {
                erised::TypeInfo::Array(erised::ArrayInfo { len, ty }) => {
                    assert_eq!(len, 4);
                    (ty)()
                }
                _ => panic!("Expected array"),
            };
            assert_eq!(
                array_ty,
                erised::TypeInfo::Primitive(erised::Primitive {
                    name: "i32",
                    docs: None
                })
            );
            assert_eq!(
                strukt.fields[4],
                erised::StructFieldInfo {
                    name: "primitive",
                    docs: None,
                    ty: erised::TypeInfo::Primitive(erised::Primitive {
                        name: "i32",
                        docs: None
                    })
                }
            );
            assert_eq!(strukt.fields[5].name, "recurse");
            let generic_args = match strukt.fields[5].ty {
                erised::TypeInfo::Generic(erised::GenericInfo { name, args }) => {
                    assert_eq!(name, "alloc::boxed::Box");
                    (args)()
                }
                t => panic!("Expected generic, found {t:?}"),
            };
            assert_eq!(generic_args.len(), 1);
            assert_eq!(generic_args[0], PlainStruct::TYPE_INFO);

            assert_eq!(strukt.fields[6].name, "borrowed");
            let borrow_ty = match strukt.fields[6].ty {
                erised::TypeInfo::Borrow(erised::BorrowInfo {
                    lifetime,
                    mutable,
                    ty,
                }) => {
                    assert_eq!(lifetime, Some("static"));
                    assert_eq!(mutable, false);
                    (ty)()
                }
                _ => panic!("Expected borrow"),
            };
            assert_eq!(
                borrow_ty,
                erised::TypeInfo::Primitive(erised::Primitive {
                    name: "str",
                    docs: None
                })
            );
            assert_eq!(strukt.fields[7].name, "mut_borrowed");
            let borrow_ty = match strukt.fields[7].ty {
                erised::TypeInfo::Borrow(erised::BorrowInfo {
                    lifetime,
                    mutable,
                    ty,
                }) => {
                    assert_eq!(lifetime, Some("static"));
                    assert_eq!(mutable, true);
                    (ty)()
                }
                _ => panic!("Expected borrow"),
            };
            assert_eq!(
                borrow_ty,
                erised::TypeInfo::Primitive(erised::Primitive {
                    name: "str",
                    docs: None
                })
            );
            assert_eq!(
                strukt.fields[8],
                erised::StructFieldInfo {
                    name: "enumerated",
                    docs: None,
                    ty: erised::TypeInfo::Enum(erised::EnumInfo {
                        name: "crate::Enum",
                        docs: None,
                        variants: &[
                            erised::VariantInfo::Unit {
                                name: "Unit",
                                discr: None,
                                docs: Some("This is variant doc")
                            },
                            erised::VariantInfo::Tuple {
                                name: "One",
                                docs: None,
                                fields: &[erised::TypeInfo::Primitive(erised::Primitive {
                                    name: "i32",
                                    docs: None
                                })]
                            },
                            erised::VariantInfo::Tuple {
                                name: "Seq",
                                docs: None,
                                fields: &[
                                    erised::TypeInfo::Primitive(erised::Primitive {
                                        name: "i32",
                                        docs: None
                                    }),
                                    erised::TypeInfo::Primitive(erised::Primitive {
                                        name: "i32",
                                        docs: None
                                    })
                                ]
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
                                            docs: None
                                        })
                                    },
                                    erised::StructFieldInfo {
                                        name: "b",
                                        docs: Some("This is field doc"),
                                        ty: erised::TypeInfo::Primitive(erised::Primitive {
                                            name: "i32",
                                            docs: None
                                        })
                                    },
                                ]
                            }
                        ]
                    })
                }
            );
            assert_eq!(
                strukt.fields[9],
                erised::StructFieldInfo {
                    name: "enum_with_disc",
                    docs: None,
                    ty: erised::TypeInfo::Enum(erised::EnumInfo {
                        name: "crate::EnumWithDisc",
                        docs: None,
                        variants: &[erised::VariantInfo::Unit {
                            name: "WithDiscr",
                            docs: None,
                            discr: Some("2")
                        }]
                    })
                }
            );
        }
        _ => panic!("Expected struct"),
    }
}

mod reflected;

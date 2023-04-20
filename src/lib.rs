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

pub trait BorrowedTrait<'a> {}

/// This is trait doc
pub trait MyTrait {
    /// This is type doc
    type Foo;
    /// This is const doc
    const FOO: &'static str;
    /// This is method doc
    fn my_trait_method(&self, arg: PlainStruct) -> EnumWithDisc;
    fn static_method(arg: PlainStruct);
    fn my_generic_method<F: MyTrait>(&self, arg: F);

    fn my_generic_method_with_where<F: MyTrait>(&self, arg: F)
    where
        F: for<'a> BorrowedTrait<'a>;
}

mod reflected;

#[test]
fn test_trait() {
    use reflected::Reflect;
    match crate::reflected::MyTrait::TYPE_INFO {
        erised::TypeInfo::Trait(trait_) => {
            assert_eq!(trait_.name, "crate::MyTrait");
            assert_eq!(trait_.docs, Some("This is trait doc"));
            assert_eq!(trait_.assoc_types[0].name, "Foo");
            assert_eq!(trait_.assoc_types[0].docs, Some("This is type doc"));
            assert_eq!(trait_.consts[0].name, "FOO");
            assert_eq!(trait_.consts[0].docs, Some("This is const doc"));
            let const_ty = trait_.consts[0].ty.as_borrow().expect("Borrow");
            assert_eq!(const_ty.lifetime, Some("static"));
            assert_eq!(const_ty.mutable, false);
            assert_eq!(
                (const_ty.ty)(),
                erised::TypeInfo::Primitive(erised::Primitive {
                    name: "str",
                    docs: None
                })
            );
            assert_eq!(trait_.methods.len(), 4);
            assert_eq!(trait_.methods[0].name, "my_trait_method");
            assert_eq!(trait_.methods[0].docs, Some("This is method doc"));
            assert_eq!(trait_.methods[0].inputs[0].name, "self");
            let erised::BorrowInfo {
                lifetime,
                mutable,
                ty,
            } = trait_.methods[0].inputs[0].ty.as_borrow().expect("Borrow");
            assert_eq!(lifetime, None);
            assert_eq!(mutable, false);
            assert_eq!((ty)(), erised::TypeInfo::Receiver);

            assert_eq!(trait_.methods[0].inputs[1].name, "arg");
            assert_eq!(trait_.methods[0].inputs[1].ty, PlainStruct::TYPE_INFO);

            assert_eq!(trait_.methods[1].name, "static_method");
            assert_eq!(trait_.methods[1].docs, None);
            assert_eq!(trait_.methods[1].inputs[0].name, "arg");
            assert_eq!(trait_.methods[1].inputs[0].ty, PlainStruct::TYPE_INFO);

            assert_eq!(trait_.methods[2].name, "my_generic_method");
            assert_eq!(trait_.methods[2].docs, None);
            assert_eq!(trait_.methods[2].generics[0].name, "F");
            let generic_kind = trait_.methods[2].generics[0]
                .kind
                .as_type()
                .expect("Type generic");
            assert_eq!(
                (generic_kind.bounds[0].trait_)(),
                crate::reflected::MyTrait::TYPE_INFO
                    .as_trait()
                    .expect("Trait")
            );
            let erised::BorrowInfo {
                lifetime,
                mutable,
                ty,
            } = trait_.methods[2].inputs[0].ty.as_borrow().expect("Borrow");
            assert_eq!(lifetime, None);
            assert_eq!(mutable, false);
            assert_eq!((ty)(), erised::TypeInfo::Receiver);
            assert_eq!(trait_.methods[2].inputs[1].name, "arg");
            assert_eq!(
                trait_.methods[2].inputs[1]
                    .ty
                    .as_generic()
                    .expect("Generic")
                    .name,
                "F"
            );
        }
        _ => panic!("Expected trait_"),
    }
}

#[test]
fn test_type() {
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
            let array_ty = {
                let arr = strukt.fields[3].ty.as_array().expect("Array");
                assert_eq!(arr.len, 4);
                (arr.ty)()
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
            let erised::WithGenericInfo {
                name,
                args,
                bindings,
            } = strukt.fields[5].ty.as_withgeneric().expect("With generic");
            assert_eq!(name, "alloc::boxed::Box");
            let generic_args = (args)();
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

use erised::ToReflect;

#[allow(dead_code)]
pub struct PlainStruct {
    tuple_str: TupleStruct,
    unit: UnitStruct,

    tuple: (String, String),
    array: [i32; 4],
    primitive: i32,
    recurse: Box<PlainStruct>,

    borrowed: &'static str,
    mut_borrowed: &'static mut str,

    enumerated: Enum,
}

pub struct TupleStruct(String);

pub struct UnitStruct;

pub enum Enum {
    // WithDiscr = 2,
    Unit,
    One(i32),
    Seq(i32, i32),
    Map { a: i32, b: i32 },
}

impl ToReflect for PlainStruct {}

#[test]
fn test() {
    use crate::reflected::Reflect;
    match PlainStruct::TYPE_INFO {
        erised::TypeInfo::Struct(strukt) => {
            assert_eq!(strukt.name, "crate::PlainStruct");
            assert_eq!(strukt.fields.len(), 9);
            assert_eq!(
                strukt.fields[0],
                erised::StructFieldInfo {
                    name: "tuple_str",
                    ty: erised::TypeInfo::TupleStruct(erised::TupleStructInfo {
                        name: "crate::TupleStruct",
                        fields: &[erised::TypeInfo::Primitive(erised::Primitive {
                            name: "alloc::string::String"
                        })]
                    })
                }
            );
            assert_eq!(
                strukt.fields[1],
                erised::StructFieldInfo {
                    name: "unit",
                    ty: erised::TypeInfo::Primitive(erised::Primitive {
                        name: "crate::UnitStruct"
                    })
                }
            );
            assert_eq!(
                strukt.fields[2],
                erised::StructFieldInfo {
                    name: "tuple",
                    ty: erised::TypeInfo::Tuple(erised::TupleInfo {
                        fields: &[
                            erised::TypeInfo::Primitive(erised::Primitive {
                                name: "alloc::string::String"
                            }),
                            erised::TypeInfo::Primitive(erised::Primitive {
                                name: "alloc::string::String"
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
                erised::TypeInfo::Primitive(erised::Primitive { name: "i32" })
            );
            assert_eq!(
                strukt.fields[4],
                erised::StructFieldInfo {
                    name: "primitive",
                    ty: erised::TypeInfo::Primitive(erised::Primitive { name: "i32" })
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
                erised::TypeInfo::Primitive(erised::Primitive { name: "str" })
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
                erised::TypeInfo::Primitive(erised::Primitive { name: "str" })
            );
            assert_eq!(
                strukt.fields[8],
                erised::StructFieldInfo {
                    name: "enumerated",
                    ty: erised::TypeInfo::Enum(erised::EnumInfo {
                        name: "crate::Enum",
                        variants: &[
                            erised::VariantInfo::Unit { name: "Unit" },
                            erised::VariantInfo::Tuple {
                                name: "One",
                                fields: &[erised::TypeInfo::Primitive(erised::Primitive {
                                    name: "i32"
                                })]
                            },
                            erised::VariantInfo::Tuple {
                                name: "Seq",
                                fields: &[
                                    erised::TypeInfo::Primitive(erised::Primitive { name: "i32" }),
                                    erised::TypeInfo::Primitive(erised::Primitive { name: "i32" })
                                ]
                            },
                            erised::VariantInfo::Struct {
                                name: "Map",
                                fields: &[
                                    erised::StructFieldInfo {
                                        name: "a",
                                        ty: erised::TypeInfo::Primitive(erised::Primitive {
                                            name: "i32"
                                        })
                                    },
                                    erised::StructFieldInfo {
                                        name: "b",
                                        ty: erised::TypeInfo::Primitive(erised::Primitive {
                                            name: "i32"
                                        })
                                    },
                                ]
                            }
                        ]
                    })
                }
            );
        }
        _ => panic!("Expected struct"),
    }
}

mod reflected;

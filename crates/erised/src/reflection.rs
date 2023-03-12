pub trait ToReflect {}

#[derive(PartialEq, Clone, Copy, Debug)]
pub enum TypeInfo {
    Struct(StructInfo),
    Tuple(TupleInfo),
    TupleStruct(TupleStructInfo),
    Enum(EnumInfo),
    Array(ArrayInfo),
    Generic(GenericInfo),
    Borrow(BorrowInfo),
    Primitive(Primitive),
}

// impl const From<&'static str> for TypeInfo {
//     fn from(name: &'static str) -> Self {
//         Self::Primitive(Primitive { name })
//     }
// }

#[derive(PartialEq, Clone, Copy, Debug)]
pub struct Primitive {
    pub name: &'static str,
}

#[derive(PartialEq, Clone, Copy, Debug)]
pub struct StructInfo {
    pub name: &'static str,
    pub fields: &'static [StructFieldInfo],
}

#[derive(PartialEq, Clone, Copy, Debug)]
pub struct TupleStructInfo {
    pub name: &'static str,
    pub fields: &'static [TypeInfo],
}

#[derive(PartialEq, Clone, Copy, Debug)]
pub struct TupleInfo {
    pub fields: &'static [TypeInfo],
}

#[derive(PartialEq, Clone, Copy, Debug)]
pub struct ArrayInfo {
    pub ty: fn() -> TypeInfo,
    pub len: usize,
}

#[derive(PartialEq, Clone, Copy, Debug)]
pub struct GenericInfo {
    pub name: &'static str,
    pub args: fn() -> &'static [TypeInfo],
}

#[derive(PartialEq, Clone, Copy, Debug)]
pub struct BorrowInfo {
    pub lifetime: Option<&'static str>,
    pub mutable: bool,
    pub ty: fn() -> TypeInfo,
}

#[derive(PartialEq, Clone, Copy, Debug)]
pub struct StructFieldInfo {
    pub name: &'static str,
    pub ty: TypeInfo,
}

#[derive(PartialEq, Clone, Copy, Debug)]
pub struct EnumInfo {
    pub name: &'static str,
    pub variants: &'static [VariantInfo],
}

#[derive(PartialEq, Clone, Copy, Debug)]
pub enum VariantInfo {
    Unit {
        name: &'static str,
    },
    Tuple {
        name: &'static str,
        fields: &'static [TypeInfo],
    },
    Struct {
        name: &'static str,
        fields: &'static [StructFieldInfo],
    },
}

pub trait Reflect {
    fn type_info() -> TypeInfo;
}

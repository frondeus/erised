pub struct ToReflect;
pub struct Trait<T>(T);

#[derive(PartialEq, Clone, Copy, Debug)]
pub enum TypeInfo {
    Struct(StructInfo),
    Tuple(TupleInfo),
    TupleStruct(TupleStructInfo),
    Enum(EnumInfo),
    Array(ArrayInfo),
    WithGeneric(WithGenericInfo),
    Borrow(BorrowInfo),
    Primitive(Primitive),
    Trait(TraitInfo),
    DynTrait(DynTraitInfo),
    Generic(GenericInfo),
    Receiver,
}

macro_rules! as_ty {
    ($({$name: ident, $var: ident, $ty: ty}),*) => {
        impl TypeInfo {
            $(

            pub fn $name(self) -> Option<$ty> {
                match self {
                    Self::$var(v) => Some(v),
                    _ => None,
                }
            }
            )*
        }
    };
}

as_ty!(
    {as_struct, Struct, StructInfo},
    {as_tuple, Tuple, TupleInfo},
    {as_tuplestruct, TupleStruct, TupleStructInfo},
    {as_enum, Enum, EnumInfo},
    {as_array, Array, ArrayInfo},
    {as_withgeneric, WithGeneric, WithGenericInfo},
    {as_borrow, Borrow, BorrowInfo},
    {as_primitive, Primitive, Primitive},
    {as_trait, Trait, TraitInfo},
    {as_generic, Generic, GenericInfo}
);

// impl const From<&'static str> for TypeInfo {
//     fn from(name: &'static str) -> Self {
//         Self::Primitive(Primitive { name })
//     }
// }

#[derive(PartialEq, Clone, Copy, Debug)]
pub struct Primitive {
    pub name: &'static str,
    pub docs: Option<&'static str>,
}

#[derive(PartialEq, Clone, Copy, Debug)]
pub struct StructInfo {
    pub docs: Option<&'static str>,
    pub name: &'static str,
    pub fields: &'static [StructFieldInfo],
}

#[derive(PartialEq, Clone, Copy, Debug)]
pub struct TupleStructInfo {
    pub docs: Option<&'static str>,
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
}

#[derive(PartialEq, Clone, Copy, Debug)]
pub struct WithGenericInfo {
    pub name: &'static str,
    pub args: fn() -> &'static [TypeInfo],
    pub bindings: &'static [WithGenericBindingInfo],
}

#[derive(PartialEq, Clone, Copy, Debug)]
pub struct WithGenericBindingInfo {
    pub name: &'static str,
    pub binding: TypeBindingKind,
}

#[derive(PartialEq, Clone, Copy, Debug)]
pub enum TypeBindingKind {
    Equality { term: Term },
    Constraint { bounds: &'static [GenericBound] },
}

#[derive(PartialEq, Clone, Copy, Debug)]
pub enum Term {
    Type(TypeInfo),
    //Constant
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
    pub docs: Option<&'static str>,
    pub ty: TypeInfo,
}

#[derive(PartialEq, Clone, Copy, Debug)]
pub struct EnumInfo {
    pub docs: Option<&'static str>,
    pub name: &'static str,
    pub variants: &'static [VariantInfo],
}

#[derive(PartialEq, Clone, Copy, Debug)]
pub enum VariantInfo {
    Unit {
        name: &'static str,
        docs: Option<&'static str>,
        discr: Option<&'static str>,
    },
    Tuple {
        name: &'static str,
        docs: Option<&'static str>,
        fields: &'static [TypeInfo],
    },
    Struct {
        name: &'static str,
        docs: Option<&'static str>,
        fields: &'static [StructFieldInfo],
    },
}

#[derive(PartialEq, Clone, Copy, Debug)]
pub struct DynTraitInfo {
    pub traits: &'static [TypeInfo],
    pub lifetime: Option<&'static str>,
}

#[derive(PartialEq, Clone, Copy, Debug)]
pub struct TraitInfo {
    pub name: &'static str,
    pub docs: Option<&'static str>,
    pub consts: &'static [ConstInfo],
    pub assoc_types: &'static [AssocTypeInfo],
    pub methods: &'static [FunctionInfo],
}

#[derive(PartialEq, Clone, Copy, Debug)]
pub struct ConstInfo {
    pub name: &'static str,
    pub docs: Option<&'static str>,
    pub ty: TypeInfo,
}

#[derive(PartialEq, Clone, Copy, Debug)]
pub struct FunctionInfo {
    pub name: &'static str,
    pub docs: Option<&'static str>,
    pub inputs: &'static [FunctionArg],
    pub output: Option<TypeInfo>,
    pub generics: &'static [FunctionGeneric],
}

#[derive(PartialEq, Clone, Copy, Debug)]
pub struct FunctionGeneric {
    pub name: &'static str,
    pub kind: GenericParamKind,
}

#[derive(PartialEq, Clone, Copy, Debug)]
pub enum GenericParamKind {
    Type(GenericParamType),
    Lifetime(LifetimeParamType),
}

impl GenericParamKind {
    pub fn as_type(self) -> Option<GenericParamType> {
        match self {
            GenericParamKind::Type(t) => Some(t),
            _ => None,
        }
    }

    pub fn as_lifetime(self) -> Option<LifetimeParamType> {
        match self {
            GenericParamKind::Lifetime(l) => Some(l),
            _ => None,
        }
    }
}

#[derive(PartialEq, Clone, Copy, Debug)]
pub struct GenericParamType {
    pub bounds: &'static [GenericBound],
}

#[derive(PartialEq, Clone, Copy, Debug)]
pub struct LifetimeParamType {
    pub outlives: &'static [&'static str],
}

#[derive(PartialEq, Clone, Copy, Debug)]
pub struct GenericBound {
    pub trait_: fn() -> TraitInfo,
}

#[derive(PartialEq, Clone, Copy, Debug)]
pub struct FunctionArg {
    pub name: &'static str,
    pub ty: TypeInfo,
}

#[derive(PartialEq, Clone, Copy, Debug)]
pub struct AssocTypeInfo {
    pub name: &'static str,
    pub docs: Option<&'static str>,
}

pub trait Reflect {
    fn type_info() -> TypeInfo;
}

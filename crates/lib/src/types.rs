#![allow(clippy::type_complexity, clippy::just_underscores_and_digits)]
#[derive(Debug, PartialEq, Eq, Hash, Clone, Copy)]
pub struct Crate {
    pub root: fn() -> Module,
    pub crate_version: Option<&'static str>,
    pub all_items: &'static [fn() -> Item],
}
#[derive(Debug, PartialEq, Eq, Hash, Clone, Copy)]
pub enum Identifiable {
    Item(fn() -> Item),
    Summary(fn() -> ItemSummary),
}
impl Identifiable {
    pub fn as_item(self) -> Option<fn() -> Item> {
        match self {
            Self::Item(_0) => Some(_0),
            _ => None,
        }
    }
    pub fn as_summary(self) -> Option<fn() -> ItemSummary> {
        match self {
            Self::Summary(_0) => Some(_0),
            _ => None,
        }
    }
}
#[derive(Debug, PartialEq, Eq, Hash, Clone, Copy)]
pub struct ExternalCrate {
    pub name: &'static str,
    pub html_root_url: Option<&'static str>,
}
#[derive(Debug, PartialEq, Eq, Hash, Clone, Copy)]
pub struct ItemSummary {
    pub krate: fn() -> ExternalCrate,
    pub path: &'static [&'static str],
    pub kind: ItemKind,
}
#[derive(Debug, PartialEq, Eq, Hash, Clone, Copy)]
pub struct ItemMeta {
    pub krate: fn() -> ExternalCrate,
    pub summary: Option<ItemSummary>,
    pub span: Option<Span>,
    pub visibility: Visibility,
    pub docs: Option<&'static str>,
    pub attrs: &'static [&'static str],
    pub deprecation: Option<Deprecation>,
}
#[derive(Debug, PartialEq, Eq, Hash, Clone, Copy)]
pub struct Span {
    pub filename: &'static str,
    pub begin: (usize, usize),
    pub end: (usize, usize),
}
#[derive(Debug, PartialEq, Eq, Hash, Clone, Copy)]
pub struct Deprecation {
    pub since: Option<&'static str>,
    pub note: Option<&'static str>,
}
#[derive(Debug, PartialEq, Eq, Hash, Clone, Copy)]
pub enum Visibility {
    Public,
    Default,
    Crate,
    Restricted {
        parent: Identifiable,
        path: &'static str,
    },
}
impl Visibility {
    pub fn as_public(self) -> Option<()> {
        match self {
            Self::Public => Some(()),
            _ => None,
        }
    }
    pub fn as_default(self) -> Option<()> {
        match self {
            Self::Default => Some(()),
            _ => None,
        }
    }
    pub fn as_crate(self) -> Option<()> {
        match self {
            Self::Crate => Some(()),
            _ => None,
        }
    }
    pub fn as_restricted(self) -> Option<(Identifiable, &'static str)> {
        match self {
            Self::Restricted { parent, path } => Some((parent, path)),
            _ => None,
        }
    }
}
#[derive(Debug, PartialEq, Eq, Hash, Clone, Copy)]
pub struct DynTrait {
    pub traits: &'static [PolyTrait],
    pub lifetime: Option<&'static str>,
}
#[derive(Debug, PartialEq, Eq, Hash, Clone, Copy)]
pub struct PolyTrait {
    pub trait_: Path,
    pub generic_params: &'static [GenericParamDef],
}
#[derive(Debug, PartialEq, Eq, Hash, Clone, Copy)]
pub enum GenericArgs {
    AngleBracketed {
        args: &'static [GenericArg],
        bindings: &'static [TypeBinding],
    },
    Parenthesized {
        inputs: &'static [Type],
        output: Option<Type>,
    },
}
impl GenericArgs {
    pub fn as_angle_bracketed(self) -> Option<(&'static [GenericArg], &'static [TypeBinding])> {
        match self {
            Self::AngleBracketed { args, bindings } => Some((args, bindings)),
            _ => None,
        }
    }
    pub fn as_parenthesized(self) -> Option<(&'static [Type], Option<Type>)> {
        match self {
            Self::Parenthesized { inputs, output } => Some((inputs, output)),
            _ => None,
        }
    }
}
#[derive(Debug, PartialEq, Eq, Hash, Clone, Copy)]
pub enum GenericArg {
    Lifetime(&'static str),
    Type(Type),
    Const(Constant),
    Infer,
}
impl GenericArg {
    pub fn as_lifetime(self) -> Option<&'static str> {
        match self {
            Self::Lifetime(_0) => Some(_0),
            _ => None,
        }
    }
    pub fn as_type(self) -> Option<Type> {
        match self {
            Self::Type(_0) => Some(_0),
            _ => None,
        }
    }
    pub fn as_const(self) -> Option<Constant> {
        match self {
            Self::Const(_0) => Some(_0),
            _ => None,
        }
    }
    pub fn as_infer(self) -> Option<()> {
        match self {
            Self::Infer => Some(()),
            _ => None,
        }
    }
}
#[derive(Debug, PartialEq, Eq, Hash, Clone, Copy)]
pub struct ConstantItem {
    pub name: &'static str,
    pub meta: ItemMeta,
    pub constant: Constant,
}
#[derive(Debug, PartialEq, Eq, Hash, Clone, Copy)]
pub struct Constant {
    pub type_: Type,
    pub expr: &'static str,
    pub value: Option<&'static str>,
    pub is_literal: bool,
}
#[derive(Debug, PartialEq, Eq, Hash, Clone, Copy)]
pub struct TypeBinding {
    pub name: &'static str,
    pub args: GenericArgs,
    pub binding: TypeBindingKind,
}
#[derive(Debug, PartialEq, Eq, Hash, Clone, Copy)]
pub enum TypeBindingKind {
    Equality(Term),
    Constraint(&'static [GenericBound]),
}
impl TypeBindingKind {
    pub fn as_equality(self) -> Option<Term> {
        match self {
            Self::Equality(_0) => Some(_0),
            _ => None,
        }
    }
    pub fn as_constraint(self) -> Option<&'static [GenericBound]> {
        match self {
            Self::Constraint(_0) => Some(_0),
            _ => None,
        }
    }
}
#[derive(Debug, PartialEq, Eq, Hash, Clone, Copy)]
pub enum ItemKind {
    Module,
    ExternCrate,
    Import,
    Struct,
    StructField,
    Union,
    Enum,
    Variant,
    Function,
    Typedef,
    OpaqueTy,
    Constant,
    Trait,
    TraitAlias,
    Impl,
    Static,
    ForeignType,
    Macro,
    ProcAttribute,
    ProcDerive,
    AssocConst,
    AssocType,
    Primitive,
    Keyword,
}
impl ItemKind {
    pub fn as_module(self) -> Option<()> {
        match self {
            Self::Module => Some(()),
            _ => None,
        }
    }
    pub fn as_extern_crate(self) -> Option<()> {
        match self {
            Self::ExternCrate => Some(()),
            _ => None,
        }
    }
    pub fn as_import(self) -> Option<()> {
        match self {
            Self::Import => Some(()),
            _ => None,
        }
    }
    pub fn as_struct(self) -> Option<()> {
        match self {
            Self::Struct => Some(()),
            _ => None,
        }
    }
    pub fn as_struct_field(self) -> Option<()> {
        match self {
            Self::StructField => Some(()),
            _ => None,
        }
    }
    pub fn as_union(self) -> Option<()> {
        match self {
            Self::Union => Some(()),
            _ => None,
        }
    }
    pub fn as_enum(self) -> Option<()> {
        match self {
            Self::Enum => Some(()),
            _ => None,
        }
    }
    pub fn as_variant(self) -> Option<()> {
        match self {
            Self::Variant => Some(()),
            _ => None,
        }
    }
    pub fn as_function(self) -> Option<()> {
        match self {
            Self::Function => Some(()),
            _ => None,
        }
    }
    pub fn as_typedef(self) -> Option<()> {
        match self {
            Self::Typedef => Some(()),
            _ => None,
        }
    }
    pub fn as_opaque_ty(self) -> Option<()> {
        match self {
            Self::OpaqueTy => Some(()),
            _ => None,
        }
    }
    pub fn as_constant(self) -> Option<()> {
        match self {
            Self::Constant => Some(()),
            _ => None,
        }
    }
    pub fn as_trait(self) -> Option<()> {
        match self {
            Self::Trait => Some(()),
            _ => None,
        }
    }
    pub fn as_trait_alias(self) -> Option<()> {
        match self {
            Self::TraitAlias => Some(()),
            _ => None,
        }
    }
    pub fn as_impl(self) -> Option<()> {
        match self {
            Self::Impl => Some(()),
            _ => None,
        }
    }
    pub fn as_static(self) -> Option<()> {
        match self {
            Self::Static => Some(()),
            _ => None,
        }
    }
    pub fn as_foreign_type(self) -> Option<()> {
        match self {
            Self::ForeignType => Some(()),
            _ => None,
        }
    }
    pub fn as_macro(self) -> Option<()> {
        match self {
            Self::Macro => Some(()),
            _ => None,
        }
    }
    pub fn as_proc_attribute(self) -> Option<()> {
        match self {
            Self::ProcAttribute => Some(()),
            _ => None,
        }
    }
    pub fn as_proc_derive(self) -> Option<()> {
        match self {
            Self::ProcDerive => Some(()),
            _ => None,
        }
    }
    pub fn as_assoc_const(self) -> Option<()> {
        match self {
            Self::AssocConst => Some(()),
            _ => None,
        }
    }
    pub fn as_assoc_type(self) -> Option<()> {
        match self {
            Self::AssocType => Some(()),
            _ => None,
        }
    }
    pub fn as_primitive(self) -> Option<()> {
        match self {
            Self::Primitive => Some(()),
            _ => None,
        }
    }
    pub fn as_keyword(self) -> Option<()> {
        match self {
            Self::Keyword => Some(()),
            _ => None,
        }
    }
}
#[derive(Debug, PartialEq, Eq, Hash, Clone, Copy)]
pub enum Item {
    Module(Module),
    ExternCrate {
        meta: ItemMeta,
        name: &'static str,
        rename: Option<&'static str>,
    },
    Import(Import),
    Union(Union),
    Struct(Struct),
    Enum(Enum),
    Function(Function),
    Trait(Trait),
    TraitAlias(TraitAlias),
    Impl(Impl),
    Typedef(Typedef),
    OpaqueTy(OpaqueTy),
    Constant(ConstantItem),
    Static(Static),
    ForeignType,
    Macro {
        name: &'static str,
        meta: ItemMeta,
        expr: &'static str,
    },
    ProcMacro(ProcMacro),
    Primitive(Primitive),
    AssocConst {
        meta: ItemMeta,
        type_: Type,
        default: Option<&'static str>,
    },
    AssocType {
        meta: ItemMeta,
        generics: Generics,
        bounds: &'static [GenericBound],
        default: Option<Type>,
    },
}
impl Item {
    pub fn as_module(self) -> Option<Module> {
        match self {
            Self::Module(_0) => Some(_0),
            _ => None,
        }
    }
    pub fn as_extern_crate(self) -> Option<(ItemMeta, &'static str, Option<&'static str>)> {
        match self {
            Self::ExternCrate { meta, name, rename } => Some((meta, name, rename)),
            _ => None,
        }
    }
    pub fn as_import(self) -> Option<Import> {
        match self {
            Self::Import(_0) => Some(_0),
            _ => None,
        }
    }
    pub fn as_union(self) -> Option<Union> {
        match self {
            Self::Union(_0) => Some(_0),
            _ => None,
        }
    }
    pub fn as_struct(self) -> Option<Struct> {
        match self {
            Self::Struct(_0) => Some(_0),
            _ => None,
        }
    }
    pub fn as_enum(self) -> Option<Enum> {
        match self {
            Self::Enum(_0) => Some(_0),
            _ => None,
        }
    }
    pub fn as_function(self) -> Option<Function> {
        match self {
            Self::Function(_0) => Some(_0),
            _ => None,
        }
    }
    pub fn as_trait(self) -> Option<Trait> {
        match self {
            Self::Trait(_0) => Some(_0),
            _ => None,
        }
    }
    pub fn as_trait_alias(self) -> Option<TraitAlias> {
        match self {
            Self::TraitAlias(_0) => Some(_0),
            _ => None,
        }
    }
    pub fn as_impl(self) -> Option<Impl> {
        match self {
            Self::Impl(_0) => Some(_0),
            _ => None,
        }
    }
    pub fn as_typedef(self) -> Option<Typedef> {
        match self {
            Self::Typedef(_0) => Some(_0),
            _ => None,
        }
    }
    pub fn as_opaque_ty(self) -> Option<OpaqueTy> {
        match self {
            Self::OpaqueTy(_0) => Some(_0),
            _ => None,
        }
    }
    pub fn as_constant(self) -> Option<ConstantItem> {
        match self {
            Self::Constant(_0) => Some(_0),
            _ => None,
        }
    }
    pub fn as_static(self) -> Option<Static> {
        match self {
            Self::Static(_0) => Some(_0),
            _ => None,
        }
    }
    pub fn as_foreign_type(self) -> Option<()> {
        match self {
            Self::ForeignType => Some(()),
            _ => None,
        }
    }
    pub fn as_macro(self) -> Option<(&'static str, ItemMeta, &'static str)> {
        match self {
            Self::Macro { name, meta, expr } => Some((name, meta, expr)),
            _ => None,
        }
    }
    pub fn as_proc_macro(self) -> Option<ProcMacro> {
        match self {
            Self::ProcMacro(_0) => Some(_0),
            _ => None,
        }
    }
    pub fn as_primitive(self) -> Option<Primitive> {
        match self {
            Self::Primitive(_0) => Some(_0),
            _ => None,
        }
    }
    pub fn as_assoc_const(self) -> Option<(ItemMeta, Type, Option<&'static str>)> {
        match self {
            Self::AssocConst {
                meta,
                type_,
                default,
            } => Some((meta, type_, default)),
            _ => None,
        }
    }
    pub fn as_assoc_type(
        self,
    ) -> Option<(ItemMeta, Generics, &'static [GenericBound], Option<Type>)> {
        match self {
            Self::AssocType {
                meta,
                generics,
                bounds,
                default,
            } => Some((meta, generics, bounds, default)),
            _ => None,
        }
    }
}
#[derive(Debug, PartialEq, Eq, Hash, Clone, Copy)]
pub struct Module {
    pub name: &'static str,
    pub meta: ItemMeta,
    pub is_crate: bool,
    pub items: &'static [Identifiable],
    pub is_stripped: bool,
}
#[derive(Debug, PartialEq, Eq, Hash, Clone, Copy)]
pub struct Union {
    pub generics: Generics,
    pub fields_stripped: bool,
    pub fields: &'static [Identifiable],
    pub impls: &'static [Identifiable],
}
#[derive(Debug, PartialEq, Eq, Hash, Clone, Copy)]
pub struct Struct {
    pub name: &'static str,
    pub meta: ItemMeta,
    pub kind: StructKind,
    pub generics: Generics,
    pub impls: &'static [Identifiable],
}
#[derive(Debug, PartialEq, Eq, Hash, Clone, Copy)]
pub enum StructKind {
    Unit,
    Tuple(&'static [Option<StructField>]),
    Plain {
        fields: &'static [StructField],
        fields_stripped: bool,
    },
}
impl StructKind {
    pub fn as_unit(self) -> Option<()> {
        match self {
            Self::Unit => Some(()),
            _ => None,
        }
    }
    pub fn as_tuple(self) -> Option<&'static [Option<StructField>]> {
        match self {
            Self::Tuple(_0) => Some(_0),
            _ => None,
        }
    }
    pub fn as_plain(self) -> Option<(&'static [StructField], bool)> {
        match self {
            Self::Plain {
                fields,
                fields_stripped,
            } => Some((fields, fields_stripped)),
            _ => None,
        }
    }
}
#[derive(Debug, PartialEq, Eq, Hash, Clone, Copy)]
pub struct StructField {
    pub name: &'static str,
    pub is_part_of_tuple: bool,
    pub meta: ItemMeta,
    pub ty: Type,
}
#[derive(Debug, PartialEq, Eq, Hash, Clone, Copy)]
pub struct Enum {
    pub name: &'static str,
    pub meta: ItemMeta,
    pub generics: Generics,
    pub variants_stripped: bool,
    pub variants: &'static [Variant],
    pub impls: &'static [Identifiable],
}
#[derive(Debug, PartialEq, Eq, Hash, Clone, Copy)]
pub struct Variant {
    pub name: &'static str,
    pub meta: ItemMeta,
    pub kind: VariantKind,
    pub discriminant: Option<Discriminant>,
}
#[derive(Debug, PartialEq, Eq, Hash, Clone, Copy)]
pub enum VariantKind {
    Plain,
    Tuple(&'static [Option<StructField>]),
    Struct {
        fields: &'static [StructField],
        fields_stripped: bool,
    },
}
impl VariantKind {
    pub fn as_plain(self) -> Option<()> {
        match self {
            Self::Plain => Some(()),
            _ => None,
        }
    }
    pub fn as_tuple(self) -> Option<&'static [Option<StructField>]> {
        match self {
            Self::Tuple(_0) => Some(_0),
            _ => None,
        }
    }
    pub fn as_struct(self) -> Option<(&'static [StructField], bool)> {
        match self {
            Self::Struct {
                fields,
                fields_stripped,
            } => Some((fields, fields_stripped)),
            _ => None,
        }
    }
}
#[derive(Debug, PartialEq, Eq, Hash, Clone, Copy)]
pub struct Discriminant {
    pub expr: &'static str,
    pub value: &'static str,
}
#[derive(Debug, PartialEq, Eq, Hash, Clone, Copy)]
pub struct Header {
    pub const_: bool,
    pub unsafe_: bool,
    pub async_: bool,
    pub abi: Abi,
}
#[derive(Debug, PartialEq, Eq, Hash, Clone, Copy)]
pub enum Abi {
    Rust,
    C { unwind: bool },
    Cdecl { unwind: bool },
    Stdcall { unwind: bool },
    Fastcall { unwind: bool },
    Aapcs { unwind: bool },
    Win64 { unwind: bool },
    SysV64 { unwind: bool },
    System { unwind: bool },
    Other(&'static str),
}
impl Abi {
    pub fn as_rust(self) -> Option<()> {
        match self {
            Self::Rust => Some(()),
            _ => None,
        }
    }
    pub fn as_c(self) -> Option<bool> {
        match self {
            Self::C { unwind } => Some(unwind),
            _ => None,
        }
    }
    pub fn as_cdecl(self) -> Option<bool> {
        match self {
            Self::Cdecl { unwind } => Some(unwind),
            _ => None,
        }
    }
    pub fn as_stdcall(self) -> Option<bool> {
        match self {
            Self::Stdcall { unwind } => Some(unwind),
            _ => None,
        }
    }
    pub fn as_fastcall(self) -> Option<bool> {
        match self {
            Self::Fastcall { unwind } => Some(unwind),
            _ => None,
        }
    }
    pub fn as_aapcs(self) -> Option<bool> {
        match self {
            Self::Aapcs { unwind } => Some(unwind),
            _ => None,
        }
    }
    pub fn as_win_64(self) -> Option<bool> {
        match self {
            Self::Win64 { unwind } => Some(unwind),
            _ => None,
        }
    }
    pub fn as_sys_v64(self) -> Option<bool> {
        match self {
            Self::SysV64 { unwind } => Some(unwind),
            _ => None,
        }
    }
    pub fn as_system(self) -> Option<bool> {
        match self {
            Self::System { unwind } => Some(unwind),
            _ => None,
        }
    }
    pub fn as_other(self) -> Option<&'static str> {
        match self {
            Self::Other(_0) => Some(_0),
            _ => None,
        }
    }
}
#[derive(Debug, PartialEq, Eq, Hash, Clone, Copy)]
pub struct Function {
    pub name: &'static str,
    pub meta: ItemMeta,
    pub decl: FnDecl,
    pub generics: Generics,
    pub header: Header,
    pub has_body: bool,
}
#[derive(Debug, PartialEq, Eq, Hash, Clone, Copy)]
pub struct Generics {
    pub params: &'static [GenericParamDef],
    pub where_predicates: &'static [WherePredicate],
}
#[derive(Debug, PartialEq, Eq, Hash, Clone, Copy)]
pub struct GenericParamDef {
    pub name: &'static str,
    pub kind: GenericParamDefKind,
}
#[derive(Debug, PartialEq, Eq, Hash, Clone, Copy)]
pub enum GenericParamDefKind {
    Lifetime {
        outlives: &'static [&'static str],
    },
    Type {
        bounds: &'static [GenericBound],
        default: Option<Type>,
        synthetic: bool,
    },
    Const {
        type_: Type,
        default: Option<&'static str>,
    },
}
impl GenericParamDefKind {
    pub fn as_lifetime(self) -> Option<&'static [&'static str]> {
        match self {
            Self::Lifetime { outlives } => Some(outlives),
            _ => None,
        }
    }
    pub fn as_type(self) -> Option<(&'static [GenericBound], Option<Type>, bool)> {
        match self {
            Self::Type {
                bounds,
                default,
                synthetic,
            } => Some((bounds, default, synthetic)),
            _ => None,
        }
    }
    pub fn as_const(self) -> Option<(Type, Option<&'static str>)> {
        match self {
            Self::Const { type_, default } => Some((type_, default)),
            _ => None,
        }
    }
}
#[derive(Debug, PartialEq, Eq, Hash, Clone, Copy)]
pub enum WherePredicate {
    BoundPredicate {
        type_: Type,
        bounds: &'static [GenericBound],
        generic_params: &'static [GenericParamDef],
    },
    RegionPredicate {
        lifetime: &'static str,
        bounds: &'static [GenericBound],
    },
    EqPredicate {
        lhs: Type,
        rhs: Term,
    },
}
impl WherePredicate {
    pub fn as_bound_predicate(
        self,
    ) -> Option<(Type, &'static [GenericBound], &'static [GenericParamDef])> {
        match self {
            Self::BoundPredicate {
                type_,
                bounds,
                generic_params,
            } => Some((type_, bounds, generic_params)),
            _ => None,
        }
    }
    pub fn as_region_predicate(self) -> Option<(&'static str, &'static [GenericBound])> {
        match self {
            Self::RegionPredicate { lifetime, bounds } => Some((lifetime, bounds)),
            _ => None,
        }
    }
    pub fn as_eq_predicate(self) -> Option<(Type, Term)> {
        match self {
            Self::EqPredicate { lhs, rhs } => Some((lhs, rhs)),
            _ => None,
        }
    }
}
#[derive(Debug, PartialEq, Eq, Hash, Clone, Copy)]
pub enum GenericBound {
    TraitBound {
        trait_: Path,
        generic_params: &'static [GenericParamDef],
        modifier: TraitBoundModifier,
    },
    Outlives(&'static str),
}
impl GenericBound {
    pub fn as_trait_bound(self) -> Option<(Path, &'static [GenericParamDef], TraitBoundModifier)> {
        match self {
            Self::TraitBound {
                trait_,
                generic_params,
                modifier,
            } => Some((trait_, generic_params, modifier)),
            _ => None,
        }
    }
    pub fn as_outlives(self) -> Option<&'static str> {
        match self {
            Self::Outlives(_0) => Some(_0),
            _ => None,
        }
    }
}
#[derive(Debug, PartialEq, Eq, Hash, Clone, Copy)]
pub enum TraitBoundModifier {
    None,
    Maybe,
    MaybeConst,
}
impl TraitBoundModifier {
    pub fn as_none(self) -> Option<()> {
        match self {
            Self::None => Some(()),
            _ => None,
        }
    }
    pub fn as_maybe(self) -> Option<()> {
        match self {
            Self::Maybe => Some(()),
            _ => None,
        }
    }
    pub fn as_maybe_const(self) -> Option<()> {
        match self {
            Self::MaybeConst => Some(()),
            _ => None,
        }
    }
}
#[derive(Debug, PartialEq, Eq, Hash, Clone, Copy)]
pub enum Term {
    Type(Type),
    Constant(Constant),
}
impl Term {
    pub fn as_type(self) -> Option<Type> {
        match self {
            Self::Type(_0) => Some(_0),
            _ => None,
        }
    }
    pub fn as_constant(self) -> Option<Constant> {
        match self {
            Self::Constant(_0) => Some(_0),
            _ => None,
        }
    }
}
#[derive(Debug, PartialEq, Eq, Hash, Clone, Copy)]
pub enum Type {
    ResolvedPath(Path),
    DynTrait(DynTrait),
    Generic(&'static str),
    Primitive(&'static str),
    FunctionPointer(fn() -> FunctionPointer),
    Tuple(&'static [Type]),
    Slice(fn() -> Type),
    Array {
        type_: fn() -> Type,
        len: &'static str,
    },
    ImplTrait(&'static [GenericBound]),
    Infer,
    RawPointer {
        mutable: bool,
        type_: fn() -> Type,
    },
    BorrowedRef {
        lifetime: Option<&'static str>,
        mutable: bool,
        type_: fn() -> Type,
    },
    QualifiedPath {
        name: &'static str,
        args: fn() -> GenericArgs,
        self_type: fn() -> Type,
        trait_: Path,
    },
}
impl Type {
    pub fn as_resolved_path(self) -> Option<Path> {
        match self {
            Self::ResolvedPath(_0) => Some(_0),
            _ => None,
        }
    }
    pub fn as_dyn_trait(self) -> Option<DynTrait> {
        match self {
            Self::DynTrait(_0) => Some(_0),
            _ => None,
        }
    }
    pub fn as_generic(self) -> Option<&'static str> {
        match self {
            Self::Generic(_0) => Some(_0),
            _ => None,
        }
    }
    pub fn as_primitive(self) -> Option<&'static str> {
        match self {
            Self::Primitive(_0) => Some(_0),
            _ => None,
        }
    }
    pub fn as_function_pointer(self) -> Option<fn() -> FunctionPointer> {
        match self {
            Self::FunctionPointer(_0) => Some(_0),
            _ => None,
        }
    }
    pub fn as_tuple(self) -> Option<&'static [Type]> {
        match self {
            Self::Tuple(_0) => Some(_0),
            _ => None,
        }
    }
    pub fn as_slice(self) -> Option<fn() -> Type> {
        match self {
            Self::Slice(_0) => Some(_0),
            _ => None,
        }
    }
    pub fn as_array(self) -> Option<(fn() -> Type, &'static str)> {
        match self {
            Self::Array { type_, len } => Some((type_, len)),
            _ => None,
        }
    }
    pub fn as_impl_trait(self) -> Option<&'static [GenericBound]> {
        match self {
            Self::ImplTrait(_0) => Some(_0),
            _ => None,
        }
    }
    pub fn as_infer(self) -> Option<()> {
        match self {
            Self::Infer => Some(()),
            _ => None,
        }
    }
    pub fn as_raw_pointer(self) -> Option<(bool, fn() -> Type)> {
        match self {
            Self::RawPointer { mutable, type_ } => Some((mutable, type_)),
            _ => None,
        }
    }
    pub fn as_borrowed_ref(self) -> Option<(Option<&'static str>, bool, fn() -> Type)> {
        match self {
            Self::BorrowedRef {
                lifetime,
                mutable,
                type_,
            } => Some((lifetime, mutable, type_)),
            _ => None,
        }
    }
    pub fn as_qualified_path(
        self,
    ) -> Option<(&'static str, fn() -> GenericArgs, fn() -> Type, Path)> {
        match self {
            Self::QualifiedPath {
                name,
                args,
                self_type,
                trait_,
            } => Some((name, args, self_type, trait_)),
            _ => None,
        }
    }
}
#[derive(Debug, PartialEq, Eq, Hash, Clone, Copy)]
pub struct Path {
    pub name: &'static str,
    pub prefix: &'static str,
    pub target: Identifiable,
    pub args: Option<fn() -> GenericArgs>,
}
#[derive(Debug, PartialEq, Eq, Hash, Clone, Copy)]
pub struct FunctionPointer {
    pub decl: FnDecl,
    pub generic_params: &'static [GenericParamDef],
    pub header: Header,
}
#[derive(Debug, PartialEq, Eq, Hash, Clone, Copy)]
pub struct FnDecl {
    pub inputs: &'static [FnInput],
    pub output: Option<Type>,
    pub c_variadic: bool,
}
#[derive(Debug, PartialEq, Eq, Hash, Clone, Copy)]
pub struct FnInput {
    pub pat: &'static str,
    pub ty: Type,
}
#[derive(Debug, PartialEq, Eq, Hash, Clone, Copy)]
pub struct Trait {
    pub name: &'static str,
    pub meta: ItemMeta,
    pub is_auto: bool,
    pub is_unsafe: bool,
    pub items: &'static [Identifiable],
    pub generics: Generics,
    pub bounds: &'static [GenericBound],
    pub implementations: &'static [Identifiable],
}
#[derive(Debug, PartialEq, Eq, Hash, Clone, Copy)]
pub struct TraitAlias {
    pub generics: Generics,
    pub params: &'static [GenericBound],
}
#[derive(Debug, PartialEq, Eq, Hash, Clone, Copy)]
pub struct Impl {
    pub meta: ItemMeta,
    pub is_unsafe: bool,
    pub generics: Generics,
    pub provided_trait_methods: &'static [&'static str],
    pub trait_: Option<Path>,
    pub for_: Type,
    pub items: &'static [fn() -> Item],
    pub negative: bool,
    pub synthetic: bool,
    pub blanket_impl: Option<Type>,
}
#[derive(Debug, PartialEq, Eq, Hash, Clone, Copy)]
pub struct Import {
    pub meta: ItemMeta,
    pub source: &'static str,
    pub name: &'static str,
    pub target: Option<Identifiable>,
    pub glob: bool,
}
#[derive(Debug, PartialEq, Eq, Hash, Clone, Copy)]
pub struct ProcMacro {
    pub meta: ItemMeta,
    pub name: &'static str,
    pub kind: MacroKind,
    pub helpers: &'static [&'static str],
}
#[derive(Debug, PartialEq, Eq, Hash, Clone, Copy)]
pub enum MacroKind {
    Bang,
    Attr,
    Derive,
}
impl MacroKind {
    pub fn as_bang(self) -> Option<()> {
        match self {
            Self::Bang => Some(()),
            _ => None,
        }
    }
    pub fn as_attr(self) -> Option<()> {
        match self {
            Self::Attr => Some(()),
            _ => None,
        }
    }
    pub fn as_derive(self) -> Option<()> {
        match self {
            Self::Derive => Some(()),
            _ => None,
        }
    }
}
#[derive(Debug, PartialEq, Eq, Hash, Clone, Copy)]
pub struct Typedef {
    pub name: &'static str,
    pub meta: ItemMeta,
    pub type_: Type,
    pub generics: Generics,
}
#[derive(Debug, PartialEq, Eq, Hash, Clone, Copy)]
pub struct OpaqueTy {
    pub name: &'static str,
    pub meta: ItemMeta,
    pub bounds: &'static [GenericBound],
    pub generics: Generics,
}
#[derive(Debug, PartialEq, Eq, Hash, Clone, Copy)]
pub struct Static {
    pub name: &'static str,
    pub meta: ItemMeta,
    pub type_: Type,
    pub mutable: bool,
    pub expr: &'static str,
}
#[derive(Debug, PartialEq, Eq, Hash, Clone, Copy)]
pub struct Primitive {
    pub name: &'static str,
    pub impls: &'static [Identifiable],
}

use crate as erised;
use erised_macros::TypeInfo;

use std::collections::HashMap;
use std::path::PathBuf;
use std::sync::{Arc, Weak};

/// rustdoc format-version.
pub(crate) const FORMAT_VERSION: u32 = 24;

/// A `Crate` is the root of the emitted JSON blob. It contains all type/documentation information
/// about the language items in the local crate, as well as info about external items to allow
/// tools to find or link to them.
#[derive(Debug, Clone, TypeInfo)]
pub struct Crate {
    /// The id of the root [`Module`] item of the local crate.
    pub root: Arc<Module>,
    /// The version string given to `--crate-version`, if any.
    pub crate_version: Option<String>,
    /// A collection of all items in the local crate as well as some external traits and their
    /// items that are referenced locally.
    pub all_items: Vec<Arc<Item>>,
    /// Maps IDs to fully qualified paths and other info helpful for generating links.
    pub summaries: Vec<Arc<ItemSummary>>,
    /// Maps `crate_id` of items to a crate name and html_root_url if it exists.
    pub external_crates: Vec<Arc<ExternalCrate>>,
}

#[derive(Debug, Clone, TypeInfo)]
pub enum Identifiable {
    Item(Weak<Item>),
    Summary(Arc<ItemSummary>),
}

#[derive(Debug, Clone, TypeInfo)]
pub struct ExternalCrate {
    pub name: String,
    pub html_root_url: Option<String>,
}

/// For external (not defined in the local crate) items, you don't get the same level of
/// information. This struct should contain enough to generate a link/reference to the item in
/// question, or can be used by a tool that takes the json output of multiple crates to find
/// the actual item definition with all the relevant info.
#[derive(Debug, Clone, TypeInfo)]
pub struct ItemSummary {
    /// Can be used to look up the name and html_root_url of the crate this item came from in the
    /// `external_crates` map.
    pub krate: Arc<ExternalCrate>,
    /// The list of path components for the fully qualified path of this item (e.g.
    /// `["std", "io", "lazy", "Lazy"]` for `std::io::lazy::Lazy`).
    ///
    /// Note that items can appear in multiple paths, and the one chosen is implementation
    /// defined. Currently, this is the full path to where the item was defined. Eg
    /// [`String`] is currently `["alloc", "string", "String"]` and [`HashMap`] is
    /// `["std", "collections", "hash", "map", "HashMap"]`, but this is subject to change.
    pub path: Vec<String>,
    /// Whether this item is a struct, trait, macro, etc.
    pub kind: ItemKind,
}

#[derive(Debug, Clone, TypeInfo)]
pub struct ItemMeta {
    /// This can be used as a key to the `external_crates` map of [`Crate`] to see which crate
    /// this item came from.
    pub krate: Arc<ExternalCrate>,
    pub summary: Option<ItemSummary>,
    /// The source location of this item (absent if it came from a macro expansion or inline
    /// assembly).
    pub span: Option<Span>,
    /// By default all documented items are public, but you can tell rustdoc to output private items
    /// so this field is needed to differentiate.
    pub visibility: Visibility,
    /// The full markdown docstring of this item. Absent if there is no documentation at all,
    /// Some("") if there is some documentation but it is empty (EG `#[doc = ""]`).
    pub docs: Option<String>,
    /// Stringified versions of the attributes on this item (e.g. `"#[inline]"`)
    pub attrs: Vec<String>,
    pub deprecation: Option<Deprecation>,
}

// #[derive(Debug, Clone, TypeInfo)]
// pub struct Item {
//     /// This can be used as a key to the `external_crates` map of [`Crate`] to see which crate
//     /// this item came from.
//     pub krate: Arc<ExternalCrate>,
//     /// Some items such as impls don't have names.
//     pub name: Option<String>,
//     /// The source location of this item (absent if it came from a macro expansion or inline
//     /// assembly).
//     pub span: Option<Span>,
//     /// By default all documented items are public, but you can tell rustdoc to output private items
//     /// so this field is needed to differentiate.
//     pub visibility: Visibility,
//     /// The full markdown docstring of this item. Absent if there is no documentation at all,
//     /// Some("") if there is some documentation but it is empty (EG `#[doc = ""]`).
//     pub docs: Option<String>,
//     /// Stringified versions of the attributes on this item (e.g. `"#[inline]"`)
//     pub attrs: Vec<String>,
//     pub deprecation: Option<Deprecation>,
//     pub inner: ItemEnum,
// }

#[derive(Debug, Clone, TypeInfo)]
pub struct Span {
    /// The path to the source file for this span relative to the path `rustdoc` was invoked with.
    pub filename: PathBuf,
    /// Zero indexed Line and Column of the first character of the `Span`
    pub begin: (usize, usize),
    /// Zero indexed Line and Column of the last character of the `Span`
    pub end: (usize, usize),
}

#[derive(Debug, Clone, TypeInfo)]
pub struct Deprecation {
    pub since: Option<String>,
    pub note: Option<String>,
}

#[derive(Debug, Clone, TypeInfo)]
pub enum Visibility {
    Public,
    /// For the most part items are private by default. The exceptions are associated items of
    /// public traits and variants of public enums.
    Default,
    Crate,
    /// For `pub(in path)` visibility. `parent` is the module it's restricted to and `path` is how
    /// that module was referenced (like `"super::super"` or `"crate::foo::bar"`).
    Restricted {
        parent: Identifiable,
        path: String,
    },
}

#[derive(Debug, Clone, TypeInfo)]
pub struct DynTrait {
    /// All the traits implemented. One of them is the vtable, and the rest must be auto traits.
    pub traits: Vec<PolyTrait>,
    /// The lifetime of the whole dyn object
    /// ```text
    /// dyn Debug + 'static
    ///             ^^^^^^^
    ///             |
    ///             this part
    /// ```
    pub lifetime: Option<String>,
}

#[derive(Debug, Clone, TypeInfo)]
/// A trait and potential HRTBs
pub struct PolyTrait {
    pub trait_: Path,
    /// Used for Higher-Rank Trait Bounds (HRTBs)
    /// ```text
    /// dyn for<'a> Fn() -> &'a i32"
    ///     ^^^^^^^
    ///       |
    ///       this part
    /// ```
    pub generic_params: Vec<GenericParamDef>,
}

#[derive(Debug, Clone, TypeInfo)]
pub enum GenericArgs {
    /// <'a, 32, B: Copy, C = u32>
    AngleBracketed {
        args: Vec<GenericArg>,
        bindings: Vec<TypeBinding>,
    },
    /// Fn(A, B) -> C
    Parenthesized {
        inputs: Vec<Type>,
        output: Option<Type>,
    },
}

#[derive(Debug, Clone, TypeInfo)]
pub enum GenericArg {
    Lifetime(String),
    Type(Type),
    Const(Constant),
    Infer,
}

#[derive(Debug, Clone, TypeInfo)]
pub struct ConstantItem {
    pub name: String,
    pub meta: ItemMeta,
    pub constant: Constant,
}

#[derive(Debug, Clone, TypeInfo)]
pub struct Constant {
    pub type_: Type,
    pub expr: String,
    pub value: Option<String>,
    pub is_literal: bool,
}

#[derive(Debug, Clone, TypeInfo)]
pub struct TypeBinding {
    pub name: String,
    pub args: GenericArgs,
    pub binding: TypeBindingKind,
}

#[derive(Debug, Clone, TypeInfo)]
pub enum TypeBindingKind {
    Equality(Term),
    Constraint(Vec<GenericBound>),
}

#[derive(Debug, Clone, TypeInfo, Copy)]
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

#[derive(Debug, Clone, TypeInfo, Default)]
pub enum Item {
    Module(Module),
    ExternCrate {
        meta: ItemMeta,
        name: String,
        rename: Option<String>,
    },
    Import(Import),

    Union(Union),
    Struct(Struct),
    // StructField(StructField),
    Enum(Enum),
    // Variant(Variant),
    Function(Function),

    Trait(Trait),
    TraitAlias(TraitAlias),
    Impl(Impl),

    Typedef(Typedef),
    OpaqueTy(OpaqueTy),
    Constant(ConstantItem),

    Static(Static),

    /// `type`s from an extern block
    #[default]
    ForeignType,

    /// Declarative macro_rules! macro
    Macro(String),
    ProcMacro(ProcMacro),

    Primitive(Primitive),

    AssocConst {
        meta: ItemMeta,
        type_: Type,
        /// e.g. `const X: usize = 5;`
        default: Option<String>,
    },
    AssocType {
        meta: ItemMeta,
        generics: Generics,
        bounds: Vec<GenericBound>,
        /// e.g. `type X = usize;`
        default: Option<Type>,
    },
}

#[derive(Debug, Clone, TypeInfo)]
pub struct Module {
    pub name: String,
    pub meta: ItemMeta,
    pub is_crate: bool,
    pub items: Vec<Identifiable>,
    /// If `true`, this module is not part of the public API, but it contains
    /// items that are re-exported as public API.
    pub is_stripped: bool,
}

#[derive(Debug, Clone, TypeInfo)]
pub struct Union {
    pub generics: Generics,
    pub fields_stripped: bool,
    pub fields: Vec<Identifiable>,
    pub impls: Vec<Identifiable>,
}

#[derive(Debug, Clone, TypeInfo)]
pub struct Struct {
    pub name: String,
    pub meta: ItemMeta,
    pub kind: StructKind,
    pub generics: Generics,
    pub impls: Vec<Identifiable>,
}

#[derive(Debug, Clone, TypeInfo)]
pub enum StructKind {
    /// A struct with no fields and no parentheses.
    ///
    /// ```rust
    /// pub struct Unit;
    /// ```
    Unit,
    /// A struct with unnamed fields.
    ///
    /// ```rust
    /// pub struct TupleStruct(i32);
    /// pub struct EmptyTupleStruct();
    /// ```
    ///
    /// All [`Id`]'s will point to [`ItemEnum::StructField`]. Private and
    /// `#[doc(hidden)]` fields will be given as `None`
    Tuple(Vec<Option<StructField>>),
    /// A struct with nammed fields.
    ///
    /// ```rust
    /// pub struct PlainStruct { x: i32 }
    /// pub struct EmptyPlainStruct {}
    /// ```
    Plain {
        fields: Vec<StructField>,
        fields_stripped: bool,
    },
}

#[derive(Debug, Clone, TypeInfo)]
pub struct StructField {
    pub name: Option<String>,
    pub meta: ItemMeta,
    pub ty: Type,
}

#[derive(Debug, Clone, TypeInfo)]
pub struct Enum {
    pub name: String,
    pub meta: ItemMeta,
    pub generics: Generics,
    pub variants_stripped: bool,
    pub variants: Vec<Variant>,
    pub impls: Vec<Identifiable>,
}

#[derive(Debug, Clone, TypeInfo)]
pub struct Variant {
    pub name: String,
    pub meta: ItemMeta,
    /// Whether the variant is plain, a tuple-like, or struct-like. Contains the fields.
    pub kind: VariantKind,
    /// The discriminant, if explicitly specified.
    pub discriminant: Option<Discriminant>,
}

#[derive(Debug, Clone, TypeInfo)]
pub enum VariantKind {
    /// A variant with no parentheses
    ///
    /// ```rust
    /// enum Demo {
    ///     PlainVariant,
    ///     PlainWithDiscriminant = 1,
    /// }
    /// ```
    Plain,
    /// A variant with unnamed fields.
    ///
    /// Unlike most of json, `#[doc(hidden)]` fields will be given as `None`
    /// instead of being omitted, because order matters.
    ///
    /// ```rust
    /// enum Demo {
    ///     TupleVariant(i32),
    ///     EmptyTupleVariant(),
    /// }
    /// ```
    Tuple(Vec<Option<StructField>>),
    /// A variant with named fields.
    ///
    /// ```rust
    /// enum Demo {
    ///     StructVariant { x: i32 },
    ///     EmptyStructVariant {},
    /// }
    /// ```
    Struct {
        fields: Vec<StructField>,
        fields_stripped: bool,
    },
}

#[derive(Debug, Clone, TypeInfo)]
pub struct Discriminant {
    /// The expression that produced the discriminant.
    ///
    /// Unlike `value`, this preserves the original formatting (eg suffixes,
    /// hexadecimal, and underscores), making it unsuitable to be machine
    /// interpreted.
    ///
    /// In some cases, when the value is to complex, this may be `"{ _ }"`.
    /// When this occurs is unstable, and may change without notice.
    pub expr: String,
    /// The numerical value of the discriminant. Stored as a string due to
    /// JSON's poor support for large integers, and the fact that it would need
    /// to store from [`i128::MIN`] to [`u128::MAX`].
    pub value: String,
}

#[derive(Debug, Clone, TypeInfo)]
pub struct Header {
    pub const_: bool,
    pub unsafe_: bool,
    pub async_: bool,
    pub abi: Abi,
}

#[derive(Debug, Clone, TypeInfo)]
pub enum Abi {
    // We only have a concrete listing here for stable ABI's because their are so many
    // See rustc_ast_passes::feature_gate::PostExpansionVisitor::check_abi for the list
    Rust,
    C { unwind: bool },
    Cdecl { unwind: bool },
    Stdcall { unwind: bool },
    Fastcall { unwind: bool },
    Aapcs { unwind: bool },
    Win64 { unwind: bool },
    SysV64 { unwind: bool },
    System { unwind: bool },
    Other(String),
}

/// Represents a function (including methods and other associated functions)
#[derive(Debug, Clone, TypeInfo)]
pub struct Function {
    pub name: String,
    pub meta: ItemMeta,
    pub decl: FnDecl,
    pub generics: Generics,
    pub header: Header,
    pub has_body: bool,
}

#[derive(Debug, Clone, TypeInfo)]
pub struct Generics {
    pub params: Vec<GenericParamDef>,
    pub where_predicates: Vec<WherePredicate>,
}

#[derive(Debug, Clone, TypeInfo)]
pub struct GenericParamDef {
    pub name: String,
    pub kind: GenericParamDefKind,
}

#[derive(Debug, Clone, TypeInfo)]
pub enum GenericParamDefKind {
    Lifetime {
        outlives: Vec<String>,
    },
    Type {
        bounds: Vec<GenericBound>,
        default: Option<Type>,
        /// This is normally `false`, which means that this generic parameter is
        /// declared in the Rust source text.
        ///
        /// If it is `true`, this generic parameter has been introduced by the
        /// compiler behind the scenes.
        ///
        /// # Example
        ///
        /// Consider
        ///
        /// ```ignore (pseudo-rust)
        /// pub fn f(_: impl Trait) {}
        /// ```
        ///
        /// The compiler will transform this behind the scenes to
        ///
        /// ```ignore (pseudo-rust)
        /// pub fn f<impl Trait: Trait>(_: impl Trait) {}
        /// ```
        ///
        /// In this example, the generic parameter named `impl Trait` (and which
        /// is bound by `Trait`) is synthetic, because it was not originally in
        /// the Rust source text.
        synthetic: bool,
    },
    Const {
        type_: Type,
        default: Option<String>,
    },
}

#[derive(Debug, Clone, TypeInfo)]
pub enum WherePredicate {
    BoundPredicate {
        type_: Type,
        bounds: Vec<GenericBound>,
        /// Used for Higher-Rank Trait Bounds (HRTBs)
        /// ```text
        /// where for<'a> &'a T: Iterator,"
        ///       ^^^^^^^
        ///       |
        ///       this part
        /// ```
        generic_params: Vec<GenericParamDef>,
    },
    RegionPredicate {
        lifetime: String,
        bounds: Vec<GenericBound>,
    },
    EqPredicate {
        lhs: Type,
        rhs: Term,
    },
}

#[derive(Debug, Clone, TypeInfo)]
pub enum GenericBound {
    TraitBound {
        trait_: Path,
        /// Used for Higher-Rank Trait Bounds (HRTBs)
        /// ```text
        /// where F: for<'a, 'b> Fn(&'a u8, &'b u8)
        ///          ^^^^^^^^^^^
        ///          |
        ///          this part
        /// ```
        generic_params: Vec<GenericParamDef>,
        modifier: TraitBoundModifier,
    },
    Outlives(String),
}

#[derive(Debug, Clone, TypeInfo)]
pub enum TraitBoundModifier {
    None,
    Maybe,
    MaybeConst,
}

#[derive(Debug, Clone, TypeInfo)]
pub enum Term {
    Type(Type),
    Constant(Constant),
}

#[derive(Debug, Clone, TypeInfo)]
pub enum Type {
    /// Structs, enums, and unions
    ResolvedPath(Path),
    DynTrait(DynTrait),
    /// Parameterized types
    Generic(String),
    /// Built in numberic (i*, u*, f*) types, bool, and char
    Primitive(String),
    /// `extern "ABI" fn`
    FunctionPointer(Box<FunctionPointer>),
    /// `(String, u32, Box<usize>)`
    Tuple(Vec<Type>),
    /// `[u32]`
    Slice(Box<Type>),
    /// [u32; 15]
    Array {
        type_: Box<Type>,
        len: String,
    },
    /// `impl TraitA + TraitB + ...`
    ImplTrait(Vec<GenericBound>),
    /// `_`
    Infer,
    /// `*mut u32`, `*u8`, etc.
    RawPointer {
        mutable: bool,
        type_: Box<Type>,
    },
    /// `&'a mut String`, `&str`, etc.
    BorrowedRef {
        lifetime: Option<String>,
        mutable: bool,
        type_: Box<Type>,
    },
    /// `<Type as Trait>::Name` or associated types like `T::Item` where `T: Iterator`
    QualifiedPath {
        name: String,
        args: Box<GenericArgs>,
        self_type: Box<Type>,
        trait_: Path,
    },
}

#[derive(Debug, Clone, TypeInfo)]
pub struct Path {
    /// Last segment of the path,
    /// for example `Cow` in `std::borrow::Cow`
    pub name: String,
    /// Everything before the name,
    /// for example `std::borrow` in `std::borrow::Cow`
    pub prefix: String,
    pub target: Identifiable,
    /// Generic arguments to the type
    /// ```test
    /// std::borrow::Cow<'static, str>
    ///                 ^^^^^^^^^^^^^^
    ///                 |
    ///                 this part
    /// ```
    pub args: Option<Box<GenericArgs>>,
}

#[derive(Debug, Clone, TypeInfo)]
pub struct FunctionPointer {
    pub decl: FnDecl,
    /// Used for Higher-Rank Trait Bounds (HRTBs)
    /// ```text
    /// for<'c> fn(val: &'c i32) -> i32
    /// ^^^^^^^
    ///       |
    ///       this part
    /// ```
    pub generic_params: Vec<GenericParamDef>,
    pub header: Header,
}

#[derive(Debug, Clone, TypeInfo)]
pub struct FnDecl {
    /// List of argument names and their type.
    ///
    /// Note that not all names will be valid identifiers, as some of
    /// them may be patterns.
    pub inputs: Vec<FnInput>,
    pub output: Option<Type>,
    pub c_variadic: bool,
}

#[derive(Debug, Clone, TypeInfo)]
pub struct FnInput {
    pub pat: String,
    pub ty: Type,
}

#[derive(Debug, Clone, TypeInfo)]
pub struct Trait {
    pub name: String,
    pub meta: ItemMeta,
    pub is_auto: bool,
    pub is_unsafe: bool,
    pub items: Vec<Identifiable>,
    pub generics: Generics,
    pub bounds: Vec<GenericBound>,
    pub implementations: Vec<Identifiable>,
}

#[derive(Debug, Clone, TypeInfo)]
pub struct TraitAlias {
    pub generics: Generics,
    pub params: Vec<GenericBound>,
}

#[derive(Debug, Clone, TypeInfo)]
pub struct Impl {
    pub meta: ItemMeta,
    pub is_unsafe: bool,
    pub generics: Generics,
    pub provided_trait_methods: Vec<String>,
    pub trait_: Option<Path>,
    pub for_: Type,
    pub items: Vec<Weak<Item>>,
    pub negative: bool,
    pub synthetic: bool,
    pub blanket_impl: Option<Type>,
}

#[derive(Debug, Clone, TypeInfo)]
pub struct Import {
    pub meta: ItemMeta,
    /// The full path being imported.
    pub source: String,
    /// May be different from the last segment of `source` when renaming imports:
    /// `use source as name;`
    pub name: String,
    /// The ID of the item being imported. Will be `None` in case of re-exports of primitives:
    /// ```rust
    /// pub use i32 as my_i32;
    /// ```
    pub target: Option<Identifiable>,
    /// Whether this import uses a glob: `use source::*;`
    pub glob: bool,
}

#[derive(Debug, Clone, TypeInfo)]
pub struct ProcMacro {
    pub kind: MacroKind,
    pub helpers: Vec<String>,
}

#[derive(Debug, Clone, TypeInfo)]
pub enum MacroKind {
    /// A bang macro `foo!()`.
    Bang,
    /// An attribute macro `#[foo]`.
    Attr,
    /// #[derive(Debug)]
    Derive,
}

#[derive(Debug, Clone, TypeInfo)]
pub struct Typedef {
    pub name: String,
    pub meta: ItemMeta,
    pub type_: Type,
    pub generics: Generics,
}

#[derive(Debug, Clone, TypeInfo)]
pub struct OpaqueTy {
    pub bounds: Vec<GenericBound>,
    pub generics: Generics,
}

#[derive(Debug, Clone, TypeInfo)]
pub struct Static {
    pub type_: Type,
    pub mutable: bool,
    pub expr: String,
}

#[derive(Debug, Clone, TypeInfo)]
pub struct Primitive {
    pub name: String,
    pub impls: Vec<Identifiable>,
}

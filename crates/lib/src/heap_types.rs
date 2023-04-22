use crate as erised;
use erised_macros::TypeInfo;

use std::collections::HashMap;
use std::path::PathBuf;
use std::sync::Arc;

/// rustdoc format-version.
pub(crate) const FORMAT_VERSION: u32 = 24;

/// A `Crate` is the root of the emitted JSON blob. It contains all type/documentation information
/// about the language items in the local crate, as well as info about external items to allow
/// tools to find or link to them.
#[derive(TypeInfo)]
pub struct Crate {
    /// The id of the root [`Module`] item of the local crate.
    pub root: Id,
    /// The version string given to `--crate-version`, if any.
    pub crate_version: Option<String>,
    /// Whether or not the output includes private items.
    pub includes_private: bool,
    /// A collection of all items in the local crate as well as some external traits and their
    /// items that are referenced locally.
    pub index: HashMap<Id, Item>,
    /// Maps IDs to fully qualified paths and other info helpful for generating links.
    pub paths: HashMap<Id, ItemSummary>,
    /// Maps `crate_id` of items to a crate name and html_root_url if it exists.
    pub external_crates: Vec<Arc<ExternalCrate>>,
    /// A single version number to be used in the future when making backwards incompatible changes
    /// to the JSON output.
    pub format_version: u32,
}

#[derive(TypeInfo)]
pub struct ExternalCrate {
    pub name: String,
    pub html_root_url: Option<String>,
}

/// For external (not defined in the local crate) items, you don't get the same level of
/// information. This struct should contain enough to generate a link/reference to the item in
/// question, or can be used by a tool that takes the json output of multiple crates to find
/// the actual item definition with all the relevant info.
#[derive(TypeInfo)]
pub struct ItemSummary {
    /// Can be used to look up the name and html_root_url of the crate this item came from in the
    /// `external_crates` map.
    pub crate_id: u32,
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

#[derive(TypeInfo)]
pub struct Item {
    /// The unique identifier of this item. Can be used to find this item in various mappings.
    pub id: Id,
    /// This can be used as a key to the `external_crates` map of [`Crate`] to see which crate
    /// this item came from.
    pub crate_id: u32,
    /// Some items such as impls don't have names.
    pub name: Option<String>,
    /// The source location of this item (absent if it came from a macro expansion or inline
    /// assembly).
    pub span: Option<Span>,
    /// By default all documented items are public, but you can tell rustdoc to output private items
    /// so this field is needed to differentiate.
    pub visibility: Visibility,
    /// The full markdown docstring of this item. Absent if there is no documentation at all,
    /// Some("") if there is some documentation but it is empty (EG `#[doc = ""]`).
    pub docs: Option<String>,
    /// This mapping resolves [intra-doc links](https://github.com/rust-lang/rfcs/blob/master/text/1946-intra-rustdoc-links.md) from the docstring to their IDs
    pub links: HashMap<String, Id>,
    /// Stringified versions of the attributes on this item (e.g. `"#[inline]"`)
    pub attrs: Vec<String>,
    pub deprecation: Option<Deprecation>,
    pub inner: ItemEnum,
}

#[derive(TypeInfo)]
pub struct Span {
    /// The path to the source file for this span relative to the path `rustdoc` was invoked with.
    pub filename: PathBuf,
    /// Zero indexed Line and Column of the first character of the `Span`
    pub begin: (usize, usize),
    /// Zero indexed Line and Column of the last character of the `Span`
    pub end: (usize, usize),
}

#[derive(TypeInfo)]
pub struct Deprecation {
    pub since: Option<String>,
    pub note: Option<String>,
}

#[derive(TypeInfo)]
pub enum Visibility {
    Public,
    /// For the most part items are private by default. The exceptions are associated items of
    /// public traits and variants of public enums.
    Default,
    Crate,
    /// For `pub(in path)` visibility. `parent` is the module it's restricted to and `path` is how
    /// that module was referenced (like `"super::super"` or `"crate::foo::bar"`).
    Restricted {
        parent: Id,
        path: String,
    },
}

#[derive(TypeInfo)]
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

#[derive(TypeInfo)]
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

#[derive(TypeInfo)]
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

#[derive(TypeInfo)]
pub enum GenericArg {
    Lifetime(String),
    Type(Type),
    Const(Constant),
    Infer,
}

#[derive(TypeInfo)]
pub struct Constant {
    pub type_: Type,
    pub expr: String,
    pub value: Option<String>,
    pub is_literal: bool,
}

#[derive(TypeInfo)]
pub struct TypeBinding {
    pub name: String,
    pub args: GenericArgs,
    pub binding: TypeBindingKind,
}

#[derive(TypeInfo)]
pub enum TypeBindingKind {
    Equality(Term),
    Constraint(Vec<GenericBound>),
}

#[derive(TypeInfo)]
pub struct Id(pub String);

#[derive(TypeInfo)]
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

#[derive(TypeInfo)]
pub enum ItemEnum {
    Module(Module),
    ExternCrate {
        name: String,
        rename: Option<String>,
    },
    Import(Import),

    Union(Union),
    Struct(Struct),
    StructField(Type),
    Enum(Enum),
    Variant(Variant),

    Function(Function),

    Trait(Trait),
    TraitAlias(TraitAlias),
    Impl(Impl),

    Typedef(Typedef),
    OpaqueTy(OpaqueTy),
    Constant(Constant),

    Static(Static),

    /// `type`s from an extern block
    ForeignType,

    /// Declarative macro_rules! macro
    Macro(String),
    ProcMacro(ProcMacro),

    Primitive(Primitive),

    AssocConst {
        type_: Type,
        /// e.g. `const X: usize = 5;`
        default: Option<String>,
    },
    AssocType {
        generics: Generics,
        bounds: Vec<GenericBound>,
        /// e.g. `type X = usize;`
        default: Option<Type>,
    },
}

#[derive(TypeInfo)]
pub struct Module {
    pub is_crate: bool,
    pub items: Vec<Id>,
    /// If `true`, this module is not part of the public API, but it contains
    /// items that are re-exported as public API.
    pub is_stripped: bool,
}

#[derive(TypeInfo)]
pub struct Union {
    pub generics: Generics,
    pub fields_stripped: bool,
    pub fields: Vec<Id>,
    pub impls: Vec<Id>,
}

#[derive(TypeInfo)]
pub struct Struct {
    pub kind: StructKind,
    pub generics: Generics,
    pub impls: Vec<Id>,
}

#[derive(TypeInfo)]
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
    Tuple(Vec<Option<Id>>),
    /// A struct with nammed fields.
    ///
    /// ```rust
    /// pub struct PlainStruct { x: i32 }
    /// pub struct EmptyPlainStruct {}
    /// ```
    Plain {
        fields: Vec<Id>,
        fields_stripped: bool,
    },
}

#[derive(TypeInfo)]
pub struct Enum {
    pub generics: Generics,
    pub variants_stripped: bool,
    pub variants: Vec<Id>,
    pub impls: Vec<Id>,
}

#[derive(TypeInfo)]
pub struct Variant {
    /// Whether the variant is plain, a tuple-like, or struct-like. Contains the fields.
    pub kind: VariantKind,
    /// The discriminant, if explicitly specified.
    pub discriminant: Option<Discriminant>,
}

#[derive(TypeInfo)]
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
    Tuple(Vec<Option<Id>>),
    /// A variant with named fields.
    ///
    /// ```rust
    /// enum Demo {
    ///     StructVariant { x: i32 },
    ///     EmptyStructVariant {},
    /// }
    /// ```
    Struct {
        fields: Vec<Id>,
        fields_stripped: bool,
    },
}

#[derive(TypeInfo)]
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

#[derive(TypeInfo)]
pub struct Header {
    pub const_: bool,
    pub unsafe_: bool,
    pub async_: bool,
    pub abi: Abi,
}

#[derive(TypeInfo)]
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
#[derive(TypeInfo)]
pub struct Function {
    pub decl: FnDecl,
    pub generics: Generics,
    pub header: Header,
    pub has_body: bool,
}

#[derive(TypeInfo)]
pub struct Generics {
    pub params: Vec<GenericParamDef>,
    pub where_predicates: Vec<WherePredicate>,
}

#[derive(TypeInfo)]
pub struct GenericParamDef {
    pub name: String,
    pub kind: GenericParamDefKind,
}

#[derive(TypeInfo)]
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

#[derive(TypeInfo)]
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

#[derive(TypeInfo)]
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

#[derive(TypeInfo)]
pub enum TraitBoundModifier {
    None,
    Maybe,
    MaybeConst,
}

#[derive(TypeInfo)]
pub enum Term {
    Type(Type),
    Constant(Constant),
}

#[derive(TypeInfo)]
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

#[derive(TypeInfo)]
pub struct Path {
    pub name: String,
    pub id: Id,
    /// Generic arguments to the type
    /// ```test
    /// std::borrow::Cow<'static, str>
    ///                 ^^^^^^^^^^^^^^
    ///                 |
    ///                 this part
    /// ```
    pub args: Option<Box<GenericArgs>>,
}

#[derive(TypeInfo)]
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

#[derive(TypeInfo)]
pub struct FnDecl {
    /// List of argument names and their type.
    ///
    /// Note that not all names will be valid identifiers, as some of
    /// them may be patterns.
    pub inputs: Vec<FnInput>,
    pub output: Option<Type>,
    pub c_variadic: bool,
}

#[derive(TypeInfo)]
pub struct FnInput {
    pat: String,
    ty: Type,
}

#[derive(TypeInfo)]
pub struct Trait {
    pub is_auto: bool,
    pub is_unsafe: bool,
    pub items: Vec<Id>,
    pub generics: Generics,
    pub bounds: Vec<GenericBound>,
    pub implementations: Vec<Id>,
}

#[derive(TypeInfo)]
pub struct TraitAlias {
    pub generics: Generics,
    pub params: Vec<GenericBound>,
}

#[derive(TypeInfo)]
pub struct Impl {
    pub is_unsafe: bool,
    pub generics: Generics,
    pub provided_trait_methods: Vec<String>,
    pub trait_: Option<Path>,
    pub for_: Type,
    pub items: Vec<Id>,
    pub negative: bool,
    pub synthetic: bool,
    pub blanket_impl: Option<Type>,
}

#[derive(TypeInfo)]
pub struct Import {
    /// The full path being imported.
    pub source: String,
    /// May be different from the last segment of `source` when renaming imports:
    /// `use source as name;`
    pub name: String,
    /// The ID of the item being imported. Will be `None` in case of re-exports of primitives:
    /// ```rust
    /// pub use i32 as my_i32;
    /// ```
    pub id: Option<Id>,
    /// Whether this import uses a glob: `use source::*;`
    pub glob: bool,
}

#[derive(TypeInfo)]
pub struct ProcMacro {
    pub kind: MacroKind,
    pub helpers: Vec<String>,
}

#[derive(TypeInfo)]
pub enum MacroKind {
    /// A bang macro `foo!()`.
    Bang,
    /// An attribute macro `#[foo]`.
    Attr,
    /// #[derive(Debug)]
    Derive,
}

#[derive(TypeInfo)]
pub struct Typedef {
    pub type_: Type,
    pub generics: Generics,
}

#[derive(TypeInfo)]
pub struct OpaqueTy {
    pub bounds: Vec<GenericBound>,
    pub generics: Generics,
}

#[derive(TypeInfo)]
pub struct Static {
    pub type_: Type,
    pub mutable: bool,
    pub expr: String,
}

#[derive(TypeInfo)]
pub struct Primitive {
    pub name: String,
    pub impls: Vec<Id>,
}

/*
#[derive(TypeInfo)]
pub enum ItemInfo {
    Struct(StructInfo),
}

#[derive(TypeInfo)]
pub enum TypeInfo {
    /// Structs, enums, unions etc.
    ResolvedPath(PathInfo),
    /// `<Type as Trait>::Name` or `T::Item` where T: Iterator
    QualifiedPath(QualifiedPathInfo),
    /// capital letters between < and >
    Generic(String),
    /// Built in numeric (i*, u*, f*), bool and char
    Primitive(String),
    /// extern ABI fn
    FunctionPointer(Box<FunctionPointerInfo>),
    /// (String, i32),
    Tuple(Vec<TypeInfo>),
    /// [u32]
    Slice(Box<TypeInfo>),
    /// [u32; 15]
    Array(ArrayInfo),
    /// dyn Trait
    DynTrait(DynTraitInfo),
    /// impl Trait
    ImplTrait(ImplTraitInfo),
    /// *mut u32, *u8
    RawPointer(RawPointerInfo),
    /// &'a mut String, &'str
    BorrowedRef(BorrowedRefInfo),
    /// Infer: `_`
    Infer,
}

#[derive(TypeInfo)]
pub struct PathInfo {
    /// Last segment of the path.
    /// For example `Path` in `std::path::Path`
    pub name: String,
    /// Everything that led to this path
    /// For example `std::path` in `std::path::Path`
    pub prefix: String,
    /// The item that this path points to
    pub item: ItemInfo,
    /// Generic arguments
    /// For example `std::borrow::Cow<'static, str>`
    ///                              ^^^^^^^^^^^^^^
    pub args: Option<GenericArgsInfo>,
}

#[derive(TypeInfo)]
pub enum GenericArgsInfo {
    /// <'a, 32, B: Copy, C = u32>
    AngleBracketed { args: Vec<GenericArgInfo> },
    /// Fn(A, B) -> C
    Parenthesized {
        inputs: Vec<TypeInfo>,
        output: Option<Box<TypeInfo>>,
    },
}

#[derive(TypeInfo)]
pub struct StructInfo {
    pub docs: Option<String>,
    pub name: String,
    pub fields: Vec<StructFieldInfo>,
}

#[derive(TypeInfo)]
pub struct StructFieldInfo {
    pub name: String,
    pub docs: Option<String>,
    pub ty: TypeInfo,
}

#[derive(TypeInfo)]
pub struct ArrayInfo {
    pub ty: Box<TypeInfo>,
    pub len: usize,
}
*/

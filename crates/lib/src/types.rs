// pub use crate::dyn_types::StaticTypeInfo as TypeInfo;
// pub use crate::dyn_types::StaticItem as Item;

pub use crate::heap_types::{
    StaticAbi as Abi, StaticConstant as Constant, StaticCrate as Crate,
    StaticDeprecation as Deprecation, StaticDiscriminant as Discriminant,
    StaticDynTrait as DynTrait, StaticEnum as Enum, StaticExternalCrate as ExternalCrate,
    StaticFnDecl as FnDecl, StaticFnInput as FnInput, StaticFunction as Function,
    StaticFunctionPointer as FunctionPointer, StaticGenericArg as GenericArg,
    StaticGenericArgs as GenericArgs, StaticGenericBound as GenericBound,
    StaticGenericParamDef as GenericParamDef, StaticGenericParamDefKind as GenericParamDefKind,
    StaticGenerics as Generics, StaticHeader as Header, StaticImpl as Impl, StaticImport as Import,
    StaticItem as Item, StaticItemEnum as ItemEnum, StaticItemKind as ItemKind,
    StaticItemSummary as ItemSummary, StaticMacroKind as MacroKind, StaticModule as Module,
    StaticOpaqueTy as OpaqueTy, StaticPath as Path, StaticPolyTrait as PolyTrait,
    StaticPrimitive as Primitive, StaticProcMacro as ProcMacro, StaticSpan as Span,
    StaticStatic as Static, StaticStruct as Struct, StaticStructKind as StructKind,
    StaticTerm as Term, StaticTrait as Trait, StaticTraitAlias as TraitAlias,
    StaticTraitBoundModifier as TraitBoundModifier, StaticType as Type,
    StaticTypeBinding as TypeBinding, StaticTypeBindingKind as TypeBindingKind,
    StaticTypedef as Typedef, StaticUnion as Union, StaticVariant as Variant,
    StaticVariantKind as VariantKind, StaticVisibility as Visibility,
    StaticWherePredicate as WherePredicate,
};

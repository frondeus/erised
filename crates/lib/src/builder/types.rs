use crate::heap_types::*;

use super::{Builder, Cache, Result};

impl Builder {
    pub(crate) fn build_type(&self, cache: &mut Cache, ty: &rustdoc_types::Type) -> Result<Type> {
        match ty {
            rustdoc_types::Type::ResolvedPath(path) => {
                Ok(Type::ResolvedPath(self.build_path(cache, path)?))
            }
            rustdoc_types::Type::DynTrait(trait_) => {
                Ok(Type::DynTrait(self.build_dyn_trait(cache, trait_)?))
            }
            rustdoc_types::Type::Generic(gen) => Ok(Type::Generic(gen.clone())),
            rustdoc_types::Type::Primitive(primitive) => Ok(Type::Primitive(primitive.clone())),
            rustdoc_types::Type::FunctionPointer(fp) => Ok(Type::FunctionPointer(Box::new(
                self.build_function_pointer(cache, fp)?,
            ))),
            rustdoc_types::Type::Tuple(tuple) => {
                let mut types = vec![];
                for ty in tuple {
                    types.push(self.build_type(cache, ty)?);
                }
                Ok(Type::Tuple(types))
            }
            rustdoc_types::Type::Slice(slice) => {
                Ok(Type::Slice(Box::new(self.build_type(cache, slice)?)))
            }
            rustdoc_types::Type::Array { type_, len } => Ok(Type::Array {
                type_: Box::new(self.build_type(cache, type_)?),
                len: len.clone(),
            }),
            rustdoc_types::Type::ImplTrait(generic_bounds) => {
                let mut imp = vec![];
                for bound in generic_bounds {
                    imp.push(self.build_generic_bound(cache, bound)?);
                }
                Ok(Type::ImplTrait(imp))
            }
            rustdoc_types::Type::Infer => Ok(Type::Infer),
            rustdoc_types::Type::RawPointer { mutable, type_ } => Ok(Type::RawPointer {
                mutable: *mutable,
                type_: Box::new(self.build_type(cache, type_)?),
            }),
            rustdoc_types::Type::BorrowedRef {
                lifetime,
                mutable,
                type_,
            } => Ok(Type::BorrowedRef {
                lifetime: lifetime.clone(),
                mutable: *mutable,
                type_: Box::new(self.build_type(cache, type_)?),
            }),
            rustdoc_types::Type::QualifiedPath {
                name,
                args,
                self_type,
                trait_,
            } => Ok(Type::QualifiedPath {
                name: name.clone(),
                args: Box::new(self.build_generic_args(cache, args)?),
                self_type: Box::new(self.build_type(cache, self_type)?),
                trait_: self.build_path(cache, trait_)?,
            }),
        }
    }

    pub(crate) fn build_generic_bound(
        &self,
        cache: &mut Cache,
        bound: &rustdoc_types::GenericBound,
    ) -> Result<GenericBound> {
        Ok(match bound {
            rustdoc_types::GenericBound::TraitBound {
                trait_,
                generic_params: source_generic_params,
                modifier,
            } => {
                let mut generic_params = vec![];
                for generic_param in source_generic_params {
                    generic_params.push(self.build_generic_param_def(cache, generic_param)?);
                }
                GenericBound::TraitBound {
                    trait_: self.build_path(cache, trait_)?,
                    generic_params,
                    modifier: self.build_trait_bound_modifier(cache, modifier)?,
                }
            }
            rustdoc_types::GenericBound::Outlives(s) => GenericBound::Outlives(s.clone()),
        })
    }

    pub(crate) fn build_trait_bound_modifier(
        &self,
        _cache: &mut Cache,
        modifier: &rustdoc_types::TraitBoundModifier,
    ) -> Result<TraitBoundModifier> {
        Ok(match modifier {
            rustdoc_types::TraitBoundModifier::None => TraitBoundModifier::None,
            rustdoc_types::TraitBoundModifier::Maybe => TraitBoundModifier::Maybe,
            rustdoc_types::TraitBoundModifier::MaybeConst => TraitBoundModifier::MaybeConst,
        })
    }

    pub(crate) fn build_generic_param_def(
        &self,
        cache: &mut Cache,
        source: &rustdoc_types::GenericParamDef,
    ) -> Result<GenericParamDef> {
        Ok(GenericParamDef {
            name: source.name.clone(),
            kind: self.build_generic_param_def_kind(cache, &source.kind)?,
        })
    }

    pub(crate) fn build_generic_param_def_kind(
        &self,
        cache: &mut Cache,
        source: &rustdoc_types::GenericParamDefKind,
    ) -> Result<GenericParamDefKind> {
        Ok(match source {
            rustdoc_types::GenericParamDefKind::Lifetime { outlives } => {
                GenericParamDefKind::Lifetime {
                    outlives: outlives.clone(),
                }
            }
            rustdoc_types::GenericParamDefKind::Type {
                bounds: source_bounds,
                default,
                synthetic,
            } => {
                let mut bounds = vec![];
                for bound in source_bounds {
                    bounds.push(self.build_generic_bound(cache, bound)?);
                }
                GenericParamDefKind::Type {
                    bounds,
                    default: match default.as_ref() {
                        Some(default) => Some(self.build_type(cache, default)?),
                        None => None,
                    },
                    synthetic: *synthetic,
                }
            }
            rustdoc_types::GenericParamDefKind::Const { type_, default } => {
                GenericParamDefKind::Const {
                    type_: self.build_type(cache, type_)?,
                    default: default.clone(),
                }
            }
        })
    }

    pub(crate) fn build_function_pointer(
        &self,
        cache: &mut Cache,
        source: &rustdoc_types::FunctionPointer,
    ) -> Result<FunctionPointer> {
        let mut generic_params = vec![];
        for param in &source.generic_params {
            generic_params.push(self.build_generic_param_def(cache, param)?);
        }

        Ok(FunctionPointer {
            decl: self.build_function_decl(cache, &source.decl)?,
            generic_params,
            header: self.build_header(cache, &source.header)?,
        })
    }

    pub(crate) fn build_dyn_trait(
        &self,
        cache: &mut Cache,
        source: &rustdoc_types::DynTrait,
    ) -> Result<DynTrait> {
        let mut traits = vec![];
        for trait_ in &source.traits {
            traits.push(self.build_poly_trait(cache, trait_)?);
        }
        Ok(DynTrait {
            traits,
            lifetime: source.lifetime.clone(),
        })
    }

    pub(crate) fn build_poly_trait(
        &self,
        cache: &mut Cache,
        source: &rustdoc_types::PolyTrait,
    ) -> Result<PolyTrait> {
        let mut generic_params = vec![];
        for param in &source.generic_params {
            generic_params.push(self.build_generic_param_def(cache, param)?);
        }
        Ok(PolyTrait {
            trait_: self.build_path(cache, &source.trait_)?,
            generic_params,
        })
    }
}

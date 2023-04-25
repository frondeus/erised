use std::sync::Arc;

use crate::heap_types::*;
use rustdoc_types::Id;

use super::{Builder, Cache, Error, Result};

impl Builder {
    pub(crate) fn build_type(&self, cache: &mut Cache, ty: &rustdoc_types::Type) -> Result<Type> {
        match ty {
            rustdoc_types::Type::ResolvedPath(path) => {
                Ok(Type::ResolvedPath(self.build_path(cache, path)?))
            }
            rustdoc_types::Type::DynTrait(_) => todo!(),
            rustdoc_types::Type::Generic(_) => todo!(),
            rustdoc_types::Type::Primitive(_) => todo!(),
            rustdoc_types::Type::FunctionPointer(_) => todo!(),
            rustdoc_types::Type::Tuple(_) => todo!(),
            rustdoc_types::Type::Slice(_) => todo!(),
            rustdoc_types::Type::Array { type_, len } => todo!(),
            rustdoc_types::Type::ImplTrait(generic_bounds) => {
                let mut imp = vec![];
                for bound in generic_bounds {
                    imp.push(self.build_generic_bound(cache, bound)?);
                }
                Ok(Type::ImplTrait(imp))
            }
            rustdoc_types::Type::Infer => todo!(),
            rustdoc_types::Type::RawPointer { mutable, type_ } => todo!(),
            rustdoc_types::Type::BorrowedRef {
                lifetime,
                mutable,
                type_,
            } => todo!(),
            rustdoc_types::Type::QualifiedPath {
                name,
                args,
                self_type,
                trait_,
            } => todo!(),
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
            rustdoc_types::GenericBound::Outlives(_) => todo!(),
        })
    }

    pub(crate) fn build_trait_bound_modifier(
        &self,
        cache: &mut Cache,
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
            rustdoc_types::GenericParamDefKind::Const { type_, default } => todo!(),
        })
    }
}

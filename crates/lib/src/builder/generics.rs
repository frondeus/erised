use std::sync::Arc;

use crate::heap_types::*;
use rustdoc_types::Id;

use super::{Builder, Cache, Error, Result};

impl Builder {
    pub(crate) fn build_generics(
        &self,
        cache: &mut Cache,
        source: &rustdoc_types::Generics,
    ) -> Result<Generics> {
        let mut params = vec![];
        for param in &source.params {
            params.push(self.build_generic_param_def(cache, param)?);
        }

        let mut where_predicates = vec![];
        for where_predicate in &source.where_predicates {
            where_predicates.push(self.build_where_predicate(cache, where_predicate)?);
        }

        Ok(Generics {
            params,
            where_predicates,
        })
    }

    pub(crate) fn build_where_predicate(
        &self,
        cache: &mut Cache,
        source: &rustdoc_types::WherePredicate,
    ) -> Result<WherePredicate> {
        Ok(match source {
            rustdoc_types::WherePredicate::BoundPredicate {
                type_,
                bounds: source_bounds,
                generic_params: source_generic_params,
            } => {
                let mut bounds = vec![];
                for bound in source_bounds {
                    bounds.push(self.build_generic_bound(cache, bound)?);
                }
                let mut generic_params = vec![];
                for generic_param in source_generic_params {
                    generic_params.push(self.build_generic_param_def(cache, generic_param)?);
                }
                WherePredicate::BoundPredicate {
                    type_: self.build_type(cache, type_)?,
                    bounds,
                    generic_params,
                }
            }
            rustdoc_types::WherePredicate::RegionPredicate { lifetime, bounds } => todo!(),
            rustdoc_types::WherePredicate::EqPredicate { lhs, rhs } => todo!(),
        })
    }
}

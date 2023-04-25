use std::sync::Weak;

use crate::heap_types::*;

use super::{Builder, Cache, Result};

impl Builder {
    pub(crate) fn build_trait(
        &self,
        cache: &mut Cache,
        name: Option<String>,
        meta: ItemMeta,
        source: &rustdoc_types::Trait,
    ) -> Result<Trait> {
        let mut items = vec![];
        for item in &source.items {
            items.push(self.get_identifiable(cache, item)?);
        }
        let generics = self.build_generics(cache, &source.generics)?;
        let mut bounds = vec![];
        for bound in &source.bounds {
            bounds.push(self.build_generic_bound(cache, bound)?);
        }
        let mut implementations = vec![];
        for implementation in &source.implementations {
            implementations.push(self.get_identifiable(cache, implementation)?);
        }
        Ok(Trait {
            is_auto: source.is_auto,
            is_unsafe: source.is_unsafe,
            items,
            generics,
            bounds,
            implementations,
            name: name.unwrap(),
            meta,
        })
    }
}

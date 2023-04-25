use crate::heap_types::*;

use super::{Builder, Cache, Result};

impl Builder {
    pub(crate) fn build_impl(
        &self,
        cache: &mut Cache,
        meta: ItemMeta,
        source: &rustdoc_types::Impl,
    ) -> Result<Impl> {
        let mut items = vec![];
        for item in &source.items {
            items.push(self.get_item(cache, item)?);
        }
        Ok(Impl {
            meta,
            is_unsafe: source.is_unsafe,
            generics: self.build_generics(cache, &source.generics)?,
            provided_trait_methods: source.provided_trait_methods.clone(),
            trait_: match source.trait_.as_ref() {
                Some(path) => Some(self.build_path(cache, path)?),
                None => None,
            },
            for_: self.build_type(cache, &source.for_)?,
            items,
            negative: source.negative,
            synthetic: source.synthetic,
            blanket_impl: match source.blanket_impl.as_ref() {
                Some(ty) => Some(self.build_type(cache, ty)?),
                None => None,
            },
        })
    }
}

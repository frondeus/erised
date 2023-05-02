use crate::heap_types::*;

use super::{Builder, Cache, Result};

impl Builder {
    pub(crate) fn build_opaque_ty(
        &self,
        cache: &mut Cache,
        name: Option<String>,
        meta: ItemMeta,
        source: &rustdoc_types::OpaqueTy,
    ) -> Result<OpaqueTy> {
        let mut bounds = vec![];
        for bound in &source.bounds {
            bounds.push(self.build_generic_bound(cache, bound)?);
        }
        Ok(OpaqueTy {
            name: name.unwrap(),
            meta,
            bounds,
            generics: self.build_generics(cache, &source.generics)?,
        })
    }
}

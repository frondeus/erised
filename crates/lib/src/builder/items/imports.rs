use crate::heap_types::*;

use super::{Builder, Cache, Result};

impl Builder {
    pub(crate) fn build_import(
        &self,
        cache: &mut Cache,
        meta: ItemMeta,
        source: &rustdoc_types::Import,
    ) -> Result<Import> {
        Ok(Import {
            name: source.name.clone(),
            meta,
            source: source.source.clone(),
            target: source
                .id
                .as_ref()
                .map(|id| self.get_identifiable(cache, id))
                .transpose()?,
            glob: source.glob,
        })
    }
}

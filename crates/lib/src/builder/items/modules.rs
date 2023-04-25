use crate::heap_types::*;

use super::{Builder, Cache, Result};

impl Builder {
    pub(crate) fn build_module(
        &self,
        cache: &mut Cache,
        name: Option<String>,
        meta: ItemMeta,
        module: &rustdoc_types::Module,
    ) -> Result<Module> {
        let mut items = vec![];
        for id in &module.items {
            items.push(self.get_identifiable(cache, id)?);
        }

        Ok(Module {
            name: name.unwrap_or_default(),
            meta,
            is_crate: module.is_crate,
            items,
            is_stripped: module.is_stripped,
        })
    }
}

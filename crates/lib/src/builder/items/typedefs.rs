use crate::heap_types::*;

use super::{Builder, Cache, Result};

impl Builder {
    pub(crate) fn build_typedef(
        &self,
        cache: &mut Cache,
        name: Option<String>,
        meta: ItemMeta,
        source: &rustdoc_types::Typedef,
    ) -> Result<Typedef> {
        Ok(Typedef {
            type_: self.build_type(cache, &source.type_)?,
            generics: self.build_generics(cache, &source.generics)?,
            name: name.unwrap(),
            meta,
        })
    }
}

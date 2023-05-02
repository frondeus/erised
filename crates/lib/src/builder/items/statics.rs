use crate::heap_types::*;

use super::{Builder, Cache, Result};

impl Builder {
    pub(crate) fn build_static(
        &self,
        cache: &mut Cache,
        name: Option<String>,
        meta: ItemMeta,
        source: &rustdoc_types::Static,
    ) -> Result<Static> {
        Ok(Static {
            type_: self.build_type(cache, &source.type_)?,
            name: name.unwrap(),
            meta,
            mutable: source.mutable,
            expr: source.expr.clone(),
        })
    }
}

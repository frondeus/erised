use crate::heap_types::*;

use super::{Builder, Cache, Result};

impl Builder {
    pub(crate) fn build_constant_item(
        &self,
        cache: &mut Cache,
        name: Option<String>,
        meta: ItemMeta,
        constant: &rustdoc_types::Constant,
    ) -> Result<ConstantItem> {
        Ok(ConstantItem {
            name: name.unwrap_or_default(),
            meta,
            constant: self.build_constant(cache, constant)?,
        })
    }

    pub(crate) fn build_constant(
        &self,
        cache: &mut Cache,
        source: &rustdoc_types::Constant,
    ) -> Result<Constant> {
        Ok(Constant {
            type_: self.build_type(cache, &source.type_)?,
            expr: source.expr.clone(),
            value: source.value.clone(),
            is_literal: source.is_literal,
        })
    }
}

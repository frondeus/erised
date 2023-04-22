use std::sync::Arc;

use crate::heap_types::*;
use rustdoc_types::Id;

use super::{Builder, Cache, Error, Result};

mod deprecation;
mod modules;
mod spans;
mod visibility;

impl Builder {
    pub(crate) fn get_item(&self, cache: &mut Cache, id: &Id) -> Result<Arc<Item>> {
        if let Some(cached) = cache.items.get(id) {
            return Ok(cached.clone());
        }

        let item = self
            .source
            .index
            .get(id)
            .ok_or_else(|| Error::CouldNotFind(id.clone()))?;

        let item = self.build_item(cache, item)?;
        let item = Arc::new(item);

        cache.items.insert(id.clone(), item.clone());

        Ok(item)
    }

    pub(crate) fn build_item(&self, cache: &mut Cache, item: &rustdoc_types::Item) -> Result<Item> {
        Ok(Item {
            krate: self.get_crate(cache, item.crate_id)?,
            name: item.name.clone(),
            span: self.build_span(cache, item.span.as_ref())?,
            visibility: self.build_visibility(cache, &item.visibility)?,
            docs: item.docs.clone(),
            attrs: item.attrs.clone(),
            deprecation: self.build_deprecation(cache, item.deprecation.as_ref())?,
            inner: self.build_item_enum(cache, &item.inner)?,
        })
    }

    pub(crate) fn build_item_enum(
        &self,
        cache: &mut Cache,
        item: &rustdoc_types::ItemEnum,
    ) -> Result<ItemEnum> {
        match item {
            rustdoc_types::ItemEnum::Module(module) => {
                Ok(ItemEnum::Module(self.build_module(cache, module)?))
            }
            rustdoc_types::ItemEnum::ExternCrate { name, rename } => todo!(),
            rustdoc_types::ItemEnum::Import(_) => todo!(),
            rustdoc_types::ItemEnum::Union(_) => todo!(),
            rustdoc_types::ItemEnum::Struct(_) => todo!(),
            rustdoc_types::ItemEnum::StructField(_) => todo!(),
            rustdoc_types::ItemEnum::Enum(_) => todo!(),
            rustdoc_types::ItemEnum::Variant(_) => todo!(),
            rustdoc_types::ItemEnum::Function(_) => todo!(),
            rustdoc_types::ItemEnum::Trait(_) => todo!(),
            rustdoc_types::ItemEnum::TraitAlias(_) => todo!(),
            rustdoc_types::ItemEnum::Impl(_) => todo!(),
            rustdoc_types::ItemEnum::Typedef(_) => todo!(),
            rustdoc_types::ItemEnum::OpaqueTy(_) => todo!(),
            rustdoc_types::ItemEnum::Constant(_) => todo!(),
            rustdoc_types::ItemEnum::Static(_) => todo!(),
            rustdoc_types::ItemEnum::ForeignType => todo!(),
            rustdoc_types::ItemEnum::Macro(_) => todo!(),
            rustdoc_types::ItemEnum::ProcMacro(_) => todo!(),
            rustdoc_types::ItemEnum::Primitive(_) => todo!(),
            rustdoc_types::ItemEnum::AssocConst { type_, default } => todo!(),
            rustdoc_types::ItemEnum::AssocType {
                generics,
                bounds,
                default,
            } => todo!(),
        }
    }
}

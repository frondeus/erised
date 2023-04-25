use std::sync::Arc;

use crate::heap_types::*;
use rustdoc_types::Id;

use super::{Builder, Cache, Error, Result};

mod deprecation;
mod spans;
mod visibility;

mod functions;
mod imports;
mod modules;

impl Builder {
    pub(crate) fn build_item_meta(
        &self,
        cache: &mut Cache,
        item: &rustdoc_types::Item,
    ) -> Result<ItemMeta> {
        Ok(ItemMeta {
            krate: self.get_crate(cache, item.crate_id)?,
            span: self.build_span(cache, item.span.as_ref())?,
            visibility: self.build_visibility(cache, &item.visibility)?,
            docs: item.docs.clone(),
            attrs: item.attrs.clone(),
            deprecation: self.build_deprecation(cache, item.deprecation.as_ref())?,
        })
    }

    pub(crate) fn get_item(&self, cache: &mut Cache, id: &Id) -> Result<Arc<Item>> {
        if let Some(cached) = cache.items.get(id) {
            return Ok(cached.clone());
        }

        let item = self
            .source
            .index
            .get(id)
            .ok_or_else(|| Error::CouldNotFind(id.clone()))?;

        let meta = self.build_item_meta(cache, item)?;
        let name = item.name.clone();
        let item = self.build_item(cache, name, meta, &item.inner)?;
        let item = Arc::new(item);

        cache.items.insert(id.clone(), item.clone());

        Ok(item)
    }

    pub(crate) fn build_item(
        &self,
        cache: &mut Cache,
        name: Option<String>,
        meta: ItemMeta,
        item: &rustdoc_types::ItemEnum,
    ) -> Result<Item> {
        match item {
            rustdoc_types::ItemEnum::Module(module) => {
                Ok(Item::Module(self.build_module(cache, name, meta, module)?))
            }
            rustdoc_types::ItemEnum::ExternCrate { name, rename } => todo!(),
            rustdoc_types::ItemEnum::Import(import) => {
                Ok(Item::Import(self.build_import(cache, meta, import)?))
            }
            rustdoc_types::ItemEnum::Union(_) => todo!(),
            rustdoc_types::ItemEnum::Struct(_) => todo!(),
            rustdoc_types::ItemEnum::StructField(_) => todo!(),
            rustdoc_types::ItemEnum::Enum(_) => todo!(),
            rustdoc_types::ItemEnum::Variant(_) => todo!(),
            rustdoc_types::ItemEnum::Function(func) => Ok(Item::Function(
                self.build_function(cache, name, meta, func)?,
            )),
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

    pub(crate) fn get_item_summary(&self, cache: &mut Cache, id: &Id) -> Result<Arc<ItemSummary>> {
        if let Some(cached) = cache.summaries.get(id) {
            return Ok(cached.clone());
        }

        let item = self
            .source
            .paths
            .get(id)
            .ok_or_else(|| Error::CouldNotFind(id.clone()))?;

        let item = self.build_item_summary(cache, &item)?;
        let item = Arc::new(item);

        cache.summaries.insert(id.clone(), item.clone());

        Ok(item)
    }

    pub(crate) fn build_item_summary(
        &self,
        cache: &mut Cache,
        item: &rustdoc_types::ItemSummary,
    ) -> Result<ItemSummary> {
        Ok(ItemSummary {
            krate: self.get_crate(cache, item.crate_id)?,
            path: item.path.clone(),
            kind: match item.kind {
                rustdoc_types::ItemKind::Module => ItemKind::Module,
                rustdoc_types::ItemKind::ExternCrate => ItemKind::ExternCrate,
                rustdoc_types::ItemKind::Import => ItemKind::Import,
                rustdoc_types::ItemKind::Struct => ItemKind::Struct,
                rustdoc_types::ItemKind::StructField => ItemKind::StructField,
                rustdoc_types::ItemKind::Union => ItemKind::Union,
                rustdoc_types::ItemKind::Enum => ItemKind::Enum,
                rustdoc_types::ItemKind::Variant => ItemKind::Variant,
                rustdoc_types::ItemKind::Function => ItemKind::Function,
                rustdoc_types::ItemKind::Typedef => ItemKind::Typedef,
                rustdoc_types::ItemKind::OpaqueTy => ItemKind::OpaqueTy,
                rustdoc_types::ItemKind::Constant => ItemKind::Constant,
                rustdoc_types::ItemKind::Trait => ItemKind::Trait,
                rustdoc_types::ItemKind::TraitAlias => ItemKind::TraitAlias,
                rustdoc_types::ItemKind::Impl => ItemKind::Impl,
                rustdoc_types::ItemKind::Static => ItemKind::Static,
                rustdoc_types::ItemKind::ForeignType => ItemKind::ForeignType,
                rustdoc_types::ItemKind::Macro => ItemKind::Macro,
                rustdoc_types::ItemKind::ProcAttribute => ItemKind::ProcAttribute,
                rustdoc_types::ItemKind::ProcDerive => ItemKind::ProcDerive,
                rustdoc_types::ItemKind::AssocConst => ItemKind::AssocConst,
                rustdoc_types::ItemKind::AssocType => ItemKind::AssocType,
                rustdoc_types::ItemKind::Primitive => ItemKind::Primitive,
                rustdoc_types::ItemKind::Keyword => ItemKind::Keyword,
            },
        })
    }

    pub(crate) fn get_identifiable(&self, cache: &mut Cache, id: &Id) -> Result<Identifiable> {
        let item = self.get_item(cache, id);
        match item {
            Ok(item) => Ok(Identifiable::Item(item)),
            Err(Error::CouldNotFind(_)) => {
                let summary = self.get_item_summary(cache, id)?;
                Ok(Identifiable::Summary(summary))
            }
            Err(e) => Err(e),
        }
    }
}

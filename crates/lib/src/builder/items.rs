use std::sync::{Arc, Weak};

use crate::{heap_types::*, utils::ArcExt};
use rustdoc_types::Id;

use super::{Builder, Cache, Error, Result};

mod deprecation;
mod spans;
mod visibility;

mod constants;
mod enums;
mod functions;
mod impls;
mod imports;
mod modules;
mod structs;
mod traits;
mod typedefs;

impl Builder {
    pub(crate) fn build_item_meta(
        &self,
        cache: &mut Cache,
        item: &rustdoc_types::Item,
    ) -> Result<ItemMeta> {
        let summary = self
            .get_item_summary(cache, &item.id)
            .ok()
            .map(|i| ItemSummary::clone(&i));
        Ok(ItemMeta {
            krate: self.get_crate(cache, item.crate_id)?,
            span: self.build_span(cache, item.span.as_ref())?,
            visibility: self.build_visibility(cache, &item.visibility)?,
            docs: item.docs.clone(),
            attrs: item.attrs.clone(),
            deprecation: self.build_deprecation(cache, item.deprecation.as_ref())?,
            summary,
        })
    }

    pub(crate) fn get_item(&self, cache: &mut Cache, id: &Id) -> Result<Weak<Item>> {
        if let Some(cached) = cache.weak_items.get(id) {
            return Ok(cached.clone());
        }
        let item = self
            .source
            .index
            .get(id)
            .ok_or_else(|| Error::CouldNotFind(id.clone()))?;

        let meta = self.build_item_meta(cache, item)?;
        let name = item.name.clone();
        let item = Arc::create_cyclic(|weak| {
            cache.weak_items.insert(id.clone(), weak.clone());

            self.build_item(cache, name, meta, &item.inner)
        })?;
        let weak = Arc::downgrade(&item);
        cache.items.insert(id.clone(), item);
        Ok(weak)
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
            rustdoc_types::ItemEnum::ExternCrate { name: _, rename: _ } => todo!(),
            rustdoc_types::ItemEnum::Import(import) => {
                Ok(Item::Import(self.build_import(cache, meta, import)?))
            }
            rustdoc_types::ItemEnum::Union(_) => todo!(),
            rustdoc_types::ItemEnum::Struct(strukt) => {
                Ok(Item::Struct(self.build_struct(cache, name, meta, strukt)?))
            }
            rustdoc_types::ItemEnum::Enum(enum_) => {
                Ok(Item::Enum(self.build_enum(cache, name, meta, enum_)?))
            }
            rustdoc_types::ItemEnum::Function(func) => Ok(Item::Function(
                self.build_function(cache, name, meta, func)?,
            )),
            rustdoc_types::ItemEnum::Trait(trait_) => {
                Ok(Item::Trait(self.build_trait(cache, name, meta, trait_)?))
            }
            rustdoc_types::ItemEnum::TraitAlias(_) => todo!(),
            rustdoc_types::ItemEnum::Impl(i) => Ok(Item::Impl(self.build_impl(cache, meta, i)?)),
            rustdoc_types::ItemEnum::Typedef(typedf) => Ok(Item::Typedef(
                self.build_typedef(cache, name, meta, typedf)?,
            )),
            rustdoc_types::ItemEnum::OpaqueTy(_) => todo!(),
            rustdoc_types::ItemEnum::Constant(c) => Ok(Item::Constant(
                self.build_constant_item(cache, name, meta, c)?,
            )),
            rustdoc_types::ItemEnum::Static(_) => todo!(),
            rustdoc_types::ItemEnum::ForeignType => todo!(),
            rustdoc_types::ItemEnum::Macro(_) => todo!(),
            rustdoc_types::ItemEnum::ProcMacro(_) => todo!(),
            rustdoc_types::ItemEnum::Primitive(_) => todo!(),
            rustdoc_types::ItemEnum::AssocConst { type_, default } => Ok(Item::AssocConst {
                meta,
                type_: self.build_type(cache, type_)?,
                default: default.clone(),
            }),
            rustdoc_types::ItemEnum::AssocType {
                generics,
                bounds: source_bounds,
                default,
            } => {
                let mut bounds = vec![];
                for bound in source_bounds {
                    bounds.push(self.build_generic_bound(cache, bound)?);
                }
                Ok(Item::AssocType {
                    meta,
                    generics: self.build_generics(cache, generics)?,
                    bounds,
                    default: match default.as_ref() {
                        None => None,
                        Some(def) => Some(self.build_type(cache, def)?),
                    },
                })
            }
            rustdoc_types::ItemEnum::StructField(_) => unreachable!(),
            rustdoc_types::ItemEnum::Variant(_) => unreachable!(),
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

        let item = self.build_item_summary(cache, item)?;
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

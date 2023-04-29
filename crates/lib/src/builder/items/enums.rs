use crate::{builder::Error, heap_types::*};

use super::{Builder, Cache, Result};

impl Builder {
    pub(crate) fn build_enum(
        &self,
        cache: &mut Cache,
        name: Option<String>,
        meta: ItemMeta,
        source: &rustdoc_types::Enum,
    ) -> Result<Enum> {
        let mut impls = vec![];
        for id in &source.impls {
            let imp = self.get_identifiable(cache, id)?;
            impls.push(imp);
        }
        let mut variants = vec![];
        for variant in &source.variants {
            variants.push(self.build_enum_variant(cache, variant)?);
        }
        Ok(Enum {
            name: name.unwrap(),
            meta,
            generics: self.build_generics(cache, &source.generics)?,
            variants_stripped: source.variants_stripped,
            variants,
            impls,
        })
    }

    pub(crate) fn build_enum_variant(
        &self,
        cache: &mut Cache,
        id: &rustdoc_types::Id,
    ) -> Result<Variant> {
        match self.source.index.get(id) {
            Some(item) => {
                let variant = match &item.inner {
                    rustdoc_types::ItemEnum::Variant(ty) => ty,
                    t => panic!("Expected variant, found {t:?}"),
                };
                Ok(Variant {
                    name: item.name.clone().unwrap(),
                    meta: self.build_item_meta(cache, item)?,
                    kind: self.build_enum_variant_kind(cache, &variant.kind)?,
                    discriminant: match variant.discriminant.as_ref() {
                        Some(d) => Some(self.build_discriminant(cache, d)?),
                        None => None,
                    },
                })
            }
            None => Err(Error::CouldNotFind(id.clone())),
        }
    }

    pub(crate) fn build_enum_variant_kind(
        &self,
        cache: &mut Cache,
        kind: &rustdoc_types::VariantKind,
    ) -> Result<VariantKind> {
        Ok(match kind {
            rustdoc_types::VariantKind::Plain => VariantKind::Plain,
            rustdoc_types::VariantKind::Tuple(source_fields) => {
                let mut fields = vec![];
                for id in source_fields {
                    match id.as_ref() {
                        Some(id) => {
                            let field = self.build_struct_field(cache, id)?;
                            fields.push(Some(field));
                        }
                        None => fields.push(None),
                    }
                }
                VariantKind::Tuple(fields)
            }
            rustdoc_types::VariantKind::Struct {
                fields: source_fields,
                fields_stripped,
            } => {
                let mut fields = vec![];
                for id in source_fields {
                    let field = self.build_struct_field(cache, id)?;
                    fields.push(field);
                }
                VariantKind::Struct {
                    fields,
                    fields_stripped: *fields_stripped,
                }
            }
        })
    }

    pub(crate) fn build_discriminant(
        &self,
        cache: &mut Cache,
        source: &rustdoc_types::Discriminant,
    ) -> Result<Discriminant> {
        Ok(Discriminant {
            expr: source.expr.clone(),
            value: source.value.clone(),
        })
    }
}

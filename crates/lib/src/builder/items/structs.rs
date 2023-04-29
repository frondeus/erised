use crate::{builder::Error::CouldNotFind, heap_types::*};

use super::{Builder, Cache, Result};

impl Builder {
    pub(crate) fn build_struct(
        &self,
        cache: &mut Cache,
        name: Option<String>,
        meta: ItemMeta,
        source: &rustdoc_types::Struct,
    ) -> Result<Struct> {
        let mut impls = vec![];
        for id in &source.impls {
            let imp = self.get_identifiable(cache, id)?;
            impls.push(imp);
        }
        Ok(Struct {
            name: name.unwrap_or_default(),
            meta,
            kind: self.build_struct_kind(cache, &source.kind)?,
            generics: self.build_generics(cache, &source.generics)?,
            impls,
        })
    }

    pub(crate) fn build_struct_kind(
        &self,
        cache: &mut Cache,
        source: &rustdoc_types::StructKind,
    ) -> Result<StructKind> {
        Ok(match source {
            rustdoc_types::StructKind::Unit => StructKind::Unit,
            rustdoc_types::StructKind::Tuple(source_fields) => {
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
                StructKind::Tuple(fields)
            }
            rustdoc_types::StructKind::Plain {
                fields: source_fields,
                fields_stripped,
            } => {
                let mut fields = vec![];
                for id in source_fields {
                    let field = self.build_struct_field(cache, id)?;
                    fields.push(field);
                }
                StructKind::Plain {
                    fields,
                    fields_stripped: *fields_stripped,
                }
            }
        })
    }

    pub(crate) fn build_struct_field(
        &self,
        cache: &mut Cache,
        source: &rustdoc_types::Id,
    ) -> Result<StructField> {
        match self.source.index.get(source) {
            Some(item) => {
                let ty = match &item.inner {
                    rustdoc_types::ItemEnum::StructField(ty) => ty,
                    t => panic!("Expected struct field, found {t:?}"),
                };
                Ok(StructField {
                    name: item.name.clone(),
                    meta: self.build_item_meta(cache, item)?,
                    ty: self.build_type(cache, ty)?,
                })
            }
            None => Err(CouldNotFind(source.clone())),
        }
    }
}

use std::sync::Arc;
use std::sync::Weak;

use crate::heap_types::Item;
use crate::heap_types::ItemMeta;

pub trait ArcExt<T>: Sized {
    fn create_cyclic<F, E>(func: F) -> Result<Self, E>
    where
        F: FnOnce(&Weak<T>) -> Result<T, E>;
}

impl<T: Default> ArcExt<T> for Arc<T> {
    fn create_cyclic<F, E>(func: F) -> Result<Self, E>
    where
        F: FnOnce(&Weak<T>) -> Result<T, E>,
    {
        let mut err = None;
        let o = Arc::new_cyclic(|weak| match func(weak) {
            Ok(o) => o,
            Err(e) => {
                err = Some(e);
                Default::default()
            }
        });

        match err {
            None => Ok(o),
            Some(err) => Err(err),
        }
    }
}

impl Item {
    pub fn meta(&self) -> &ItemMeta {
        match self {
            Item::Module(v) => &v.meta,
            Item::ExternCrate { meta, name, rename } => meta,
            Item::Import(v) => &v.meta,
            Item::Union(v) => todo!(),
            Item::Struct(v) => &v.meta,
            Item::Enum(v) => &v.meta,
            Item::Function(v) => &v.meta,
            Item::Trait(v) => &v.meta,
            Item::TraitAlias(v) => todo!(),
            Item::Impl(v) => &v.meta,
            Item::Typedef(v) => &v.meta,
            Item::OpaqueTy(v) => &v.meta,
            Item::Constant(v) => &v.meta,
            Item::Static(v) => &v.meta,
            Item::ForeignType => todo!(),
            Item::Macro { name, meta, expr } => meta,
            Item::ProcMacro(v) => todo!(),
            Item::Primitive(v) => todo!(),
            Item::AssocConst {
                meta,
                type_,
                default,
            } => meta,
            Item::AssocType {
                meta,
                generics,
                bounds,
                default,
            } => meta,
        }
    }
}

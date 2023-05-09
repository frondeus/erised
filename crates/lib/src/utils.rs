use std::collections::HashSet;
use std::sync::Arc;
use std::sync::Weak;

use crate::codegen_input::GenericArgs;
use crate::heap_types::Item;
use crate::heap_types::ItemMeta;

#[derive(Default, Clone)]
pub struct CycleDetector {
    visited: HashSet<crate::heap_types::Id>,
}

impl CycleDetector {
    pub fn was_visited(&mut self, item: &Item) -> bool {
        // Because there are cycles in the graph we want to make sure we won't run out of the stack.
        let id = &item.meta().id;
        if self.visited.get(id).is_some() {
            return true;
        }

        self.visited.insert(id.clone());
        false
    }

    pub fn remove_from_visited(&mut self, item: &Item) {
        let id = &item.meta().id;
        self.visited.remove(id);
    }
}

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
            Item::ExternCrate { meta, .. } => meta,
            Item::Import(v) => &v.meta,
            Item::Union(_v) => todo!(),
            Item::Struct(v) => &v.meta,
            Item::Enum(v) => &v.meta,
            Item::Function(v) => &v.meta,
            Item::Trait(v) => &v.meta,
            Item::TraitAlias(_v) => todo!(),
            Item::Impl(v) => &v.meta,
            Item::Typedef(v) => &v.meta,
            Item::OpaqueTy(v) => &v.meta,
            Item::Constant(v) => &v.meta,
            Item::Static(v) => &v.meta,
            Item::ForeignType => todo!(),
            Item::Macro { meta, .. } => meta,
            Item::ProcMacro(_v) => todo!(),
            Item::Primitive(_v) => todo!(),
            Item::AssocConst { meta, .. } => meta,
            Item::AssocType { meta, .. } => meta,
        }
    }
}

impl GenericArgs {
    pub fn is_empty(&self) -> bool {
        match self {
            GenericArgs::AngleBracketed { args, bindings } => {
                args.is_empty() && bindings.is_empty()
            }
            GenericArgs::Parenthesized { .. } => false, //inputs.is_empty() && output,
        }
    }
}

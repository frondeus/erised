use crate::heap_types::*;

use super::{Builder, Cache, Result};

impl Builder {
    pub(crate) fn build_module(
        &self,
        cache: &mut Cache,
        module: &rustdoc_types::Module,
    ) -> Result<Module> {
        Ok(Module {
            is_crate: module.is_crate,
            items: vec![],
            is_stripped: module.is_stripped,
        })
    }
}

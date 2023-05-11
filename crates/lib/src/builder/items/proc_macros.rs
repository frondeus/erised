use crate::heap_types::*;

use super::{Builder, Cache, Result};

impl Builder {
    pub(crate) fn build_proc_macro(
        &self,
        _cache: &mut Cache,
        name: Option<String>,
        meta: ItemMeta,
        source: &rustdoc_types::ProcMacro,
    ) -> Result<ProcMacro> {
        Ok(ProcMacro {
            name: name.unwrap(),
            meta,
            kind: match source.kind {
                rustdoc_types::MacroKind::Bang => MacroKind::Bang,
                rustdoc_types::MacroKind::Attr => MacroKind::Attr,
                rustdoc_types::MacroKind::Derive => MacroKind::Derive,
            },
            helpers: source.helpers.clone(),
        })
    }
}

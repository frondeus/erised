use crate::heap_types::*;

use super::{Builder, Cache, Result};
impl Builder {
    pub(crate) fn build_visibility(
        &self,
        _cache: &mut Cache,
        vis: &rustdoc_types::Visibility,
    ) -> Result<Visibility> {
        match vis {
            rustdoc_types::Visibility::Public => Ok(Visibility::Public),
            rustdoc_types::Visibility::Default => Ok(Visibility::Default),
            rustdoc_types::Visibility::Crate => Ok(Visibility::Crate),
            rustdoc_types::Visibility::Restricted { parent: _, path: _ } => {
                todo!()
            }
        }
    }
}

use crate::heap_types::*;

use super::{Builder, Cache, Result};
impl Builder {
    pub(crate) fn build_deprecation(
        &self,
        _cache: &mut Cache,
        deprecation: Option<&rustdoc_types::Deprecation>,
    ) -> Result<Option<Deprecation>> {
        let deprecation = match deprecation {
            None => return Ok(None),
            Some(span) => span,
        };

        Ok(Some(Deprecation {
            since: deprecation.since.clone(),
            note: deprecation.note.clone(),
        }))
    }
}

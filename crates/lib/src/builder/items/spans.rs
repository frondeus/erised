use crate::heap_types::*;

use super::{Builder, Cache, Result};
impl Builder {
    pub(crate) fn build_span(
        &self,
        _cache: &mut Cache,
        span: Option<&rustdoc_types::Span>,
    ) -> Result<Option<Span>> {
        let span = match span {
            None => return Ok(None),
            Some(span) => span,
        };

        Ok(Some(Span {
            filename: span.filename.clone(),
            begin: span.begin,
            end: span.end,
        }))
    }
}

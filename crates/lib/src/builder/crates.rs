use std::sync::Arc;

use crate::heap_types::*;

use super::{Builder, Cache, Error, Result};

impl Builder {
    pub(crate) fn get_crate(&self, cache: &mut Cache, id: u32) -> Result<Arc<ExternalCrate>> {
        if let Some(cached) = cache.crates.get(&id) {
            return Ok(cached.clone());
        }

        if (id == self.root.crate_id) {
            let krate = Arc::new(ExternalCrate {
                name: self.root.name.clone().unwrap(),
                html_root_url: None,
            });
            cache.crates.insert(id, krate.clone());
            return Ok(krate);
        }

        let krate = self
            .source
            .external_crates
            .get(&id)
            .ok_or_else(|| Error::CouldNotFindCrate(id))?;

        let krate = Arc::new(ExternalCrate {
            name: krate.name.clone(),
            html_root_url: krate.html_root_url.clone(),
        });

        cache.crates.insert(id, krate.clone());

        Ok(krate)
    }
}

use std::{
    collections::HashMap,
    sync::{Arc, Weak},
};

use crate::heap_types::*;
use rustdoc_types::Id;
use thiserror::Error;

pub use rustdoc_json::PackageTarget;

#[derive(Debug, Error)]
pub enum Error {
    #[error(transparent)]
    BuildError(#[from] rustdoc_json::BuildError),

    #[error(transparent)]
    IO(#[from] std::io::Error),

    #[error(transparent)]
    JSON(#[from] serde_json::Error),

    #[error("Could not find item with id: {0:?}")]
    CouldNotFind(rustdoc_types::Id),

    #[error("Could not find crate with id: {0:?}")]
    CouldNotFindCrate(u32),
}

pub type Result<T, E = Error> = std::result::Result<T, E>;

pub struct Builder {
    source: rustdoc_types::Crate,
    // reflect_id: rustdoc_types::Id,
    root: rustdoc_types::Item,
}

#[derive(Default)]
pub(crate) struct Cache {
    weak_items: HashMap<Id, Weak<Item>>,
    items: HashMap<Id, Arc<Item>>,
    summaries: HashMap<Id, Arc<ItemSummary>>,
    crates: HashMap<u32, Arc<ExternalCrate>>,
}

mod crates;
mod generics;
mod items;
mod paths;
mod types;

impl Builder {
    pub fn load(
        manifest_dir: impl AsRef<std::path::Path>,
        options: impl Fn(rustdoc_json::Builder) -> rustdoc_json::Builder,
    ) -> Result<Option<Crate>> {
        let manifest_dir = manifest_dir.as_ref();

        let builder = rustdoc_json::Builder::default()
            .toolchain("nightly")
            .manifest_path(manifest_dir.join("Cargo.toml"))
            .target_dir(manifest_dir.join("erised_target"))
            .document_private_items(true);

        let json_path = options(builder).build()?;
        let file = std::fs::OpenOptions::new().read(true).open(&json_path)?;

        let source: rustdoc_types::Crate = serde_json::from_reader(file)?;

        // let reflect_id = source
        //     .paths
        //     .iter()
        //     .find(|(_path, val)| val.path == &["erised", "ToReflect"])
        //     .map(|(p, _)| p)
        //     .cloned();

        // let reflect_id = match reflect_id {
        //     Some(id) => id,
        //     None => {
        //         eprintln!(
        //             "cargo-warning=Reflection crate did not find a single usage of erised::ToReflect"
        //         );
        //         eprintln!("cargo-warning=It tried to load {json_path:?}");
        //         return Ok(None);
        //     }
        // };

        let root = source
            .index
            .get(&source.root)
            .ok_or_else(|| Error::CouldNotFind(source.root.clone()))?
            .clone();

        let builder = Builder {
            source,
            // reflect_id,
            root,
        };

        builder.build().map(Some)
    }

    pub fn build(self) -> Result<Crate> {
        let mut cache = Default::default();
        let root = (*self
            .get_item(&mut cache, &self.source.root)?
            .upgrade()
            .expect("Module"))
        .clone()
        .as_module()
        .expect("Module")
        .into();

        Ok(Crate {
            root,
            crate_version: self.source.crate_version,
            all_items: cache.items.into_iter().map(|(_k, v)| v).collect(),
            summaries: vec![],
            external_crates: vec![],
        })
    }
}

use std::{
    collections::{BTreeMap, HashMap},
    path::PathBuf,
    sync::{Arc, Weak},
};

use crate::heap_types::*;
use rustdoc_types::Id;
use thiserror::Error;

#[derive(Debug, Error)]
pub enum Error {
    #[error("Expected JSON doc in version {expected} but found {found}")]
    VersionMismatch { expected: u32, found: u32 },

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
    crate_replacement: Option<CrateReplacement>,
}

#[derive(Clone)]
pub enum CrateReplacement {
    Root,
    OtherByName(String),
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

pub struct BuilderOpts {
    pub toolchain: Option<String>,
    pub manifest_path: PathBuf,
    pub target_dir: Option<PathBuf>,
    pub target: Option<String>,
    pub quiet: bool,
    pub silent: bool,
    pub no_default_features: bool,
    pub all_features: bool,
    pub features: Vec<String>,
    pub package: Option<String>,
    pub package_target: PackageTarget,
    pub document_private_items: bool,
    pub cap_lints: Option<String>,
    pub crate_replacement: Option<CrateReplacement>,
}

#[derive(Default, Debug, Clone)]
pub enum PackageTarget {
    #[default]
    Lib,
    Bin(String),
    Example(String),
    Test(String),
    Bench(String),
}

impl From<PackageTarget> for rustdoc_json::PackageTarget {
    fn from(value: PackageTarget) -> Self {
        match value {
            PackageTarget::Lib => Self::Lib,
            PackageTarget::Bin(b) => Self::Bin(b),
            PackageTarget::Example(e) => Self::Example(e),
            PackageTarget::Test(t) => Self::Test(t),
            PackageTarget::Bench(b) => Self::Bench(b),
        }
    }
}

impl Default for BuilderOpts {
    fn default() -> Self {
        Self {
            manifest_path: "Cargo.toml".into(),
            toolchain: Some("nightly".to_owned()),
            target_dir: None,
            target: None,
            quiet: false,
            silent: false,
            no_default_features: false,
            all_features: false,
            features: vec![],
            package: None,
            package_target: PackageTarget::default(),
            document_private_items: false,
            cap_lints: Some("warn".to_owned()),
            crate_replacement: None,
        }
    }
}

impl From<BuilderOpts> for rustdoc_json::Builder {
    fn from(
        BuilderOpts {
            toolchain,
            manifest_path,
            target_dir,
            target,
            quiet,
            silent,
            no_default_features,
            all_features,
            features,
            package,
            package_target,
            document_private_items,
            cap_lints,
            crate_replacement: _,
        }: BuilderOpts,
    ) -> Self {
        let mut builder = rustdoc_json::Builder::default()
            .manifest_path(manifest_path)
            .quiet(quiet)
            .silent(silent)
            .no_default_features(no_default_features)
            .all_features(all_features)
            .features(features)
            .document_private_items(document_private_items)
            .cap_lints(cap_lints)
            .package_target(package_target.into());

        if let Some(target_dir) = target_dir {
            builder = builder.target_dir(target_dir);
        }
        if let Some(target) = target {
            builder = builder.target(target);
        }
        if let Some(package) = package {
            builder = builder.package(package);
        }

        if let Some(toolchain) = toolchain {
            builder = builder.toolchain(toolchain);
        }

        builder
    }
}

impl BuilderOpts {
    pub fn use_default_toolchain(mut self) -> Self {
        self.toolchain = None;
        self
    }

    pub fn manifest_dir(mut self, dir: impl AsRef<std::path::Path>) -> Self {
        let dir = dir.as_ref();
        self.manifest_path = dir.join("Cargo.toml");
        self.target_dir = Some(dir.join("erised_target"));
        self
    }

    pub fn replace_with_crate(mut self, replace: CrateReplacement) -> Self {
        self.crate_replacement = Some(replace);
        self
    }

    /// Set the relative or absolute path to `Cargo.toml`. Default: `Cargo.toml`
    #[must_use]
    pub fn manifest_path(mut self, manifest_path: impl AsRef<std::path::Path>) -> Self {
        self.manifest_path = manifest_path.as_ref().to_owned();
        self
    }

    /// Set what `--target-dir` to pass to `cargo`. Typically only needed if you
    /// want to be able to build rustdoc JSON for the same crate concurrently,
    /// for example to parallelize regression tests.
    #[must_use]
    pub fn target_dir(mut self, target_dir: impl AsRef<std::path::Path>) -> Self {
        self.target_dir = Some(target_dir.as_ref().to_owned());
        self
    }

    /// Clear a target dir previously set with [`Self::target_dir`].
    #[must_use]
    pub fn clear_target_dir(mut self) -> Self {
        self.target_dir = None;
        self
    }

    /// Whether or not to pass `--quiet` to `cargo rustdoc`. Default: `false`
    #[must_use]
    pub const fn quiet(mut self, quiet: bool) -> Self {
        self.quiet = quiet;
        self
    }

    /// Whether or not to redirect stdout and stderr to /dev/null. Default: `false`
    #[must_use]
    pub const fn silent(mut self, silent: bool) -> Self {
        self.silent = silent;
        self
    }

    /// Whether or not to pass `--target` to `cargo rustdoc`. Default: `None`
    #[must_use]
    pub fn target(mut self, target: String) -> Self {
        self.target = Some(target);
        self
    }

    /// Whether to pass `--no-default-features` to `cargo rustdoc`. Default: `false`
    #[must_use]
    pub const fn no_default_features(mut self, no_default_features: bool) -> Self {
        self.no_default_features = no_default_features;
        self
    }

    /// Whether to pass `--all-features` to `cargo rustdoc`. Default: `false`
    #[must_use]
    pub const fn all_features(mut self, all_features: bool) -> Self {
        self.all_features = all_features;
        self
    }

    /// Features to pass to `cargo rustdoc` via `--features`. Default to an empty vector
    #[must_use]
    pub fn features<I: IntoIterator<Item = S>, S: AsRef<str>>(mut self, features: I) -> Self {
        self.features = features
            .into_iter()
            .map(|item| item.as_ref().to_owned())
            .collect();
        self
    }

    /// Package to use for `cargo rustdoc` via `-p`. Default: `None`
    #[must_use]
    pub fn package(mut self, package: impl AsRef<str>) -> Self {
        self.package = Some(package.as_ref().to_owned());
        self
    }

    /// What part of the package to document. Default: `PackageTarget::Lib`
    #[must_use]
    pub fn package_target(mut self, package_target: PackageTarget) -> Self {
        self.package_target = package_target;
        self
    }

    /// Whether to pass `--document-private-items` to `cargo rustdoc`. Default: `false`
    #[must_use]
    pub fn document_private_items(mut self, document_private_items: bool) -> Self {
        self.document_private_items = document_private_items;
        self
    }

    /// What to pass as `--cap-lints` to rustdoc JSON build command
    #[must_use]
    pub fn cap_lints(mut self, cap_lints: Option<impl AsRef<str>>) -> Self {
        self.cap_lints = cap_lints.map(|c| c.as_ref().to_owned());
        self
    }

    pub fn load(self) -> Result<Builder> {
        let crate_replacement = self.crate_replacement.clone();
        let builder: rustdoc_json::Builder = self.into();
        let json_path = builder.build()?;
        let file = std::fs::OpenOptions::new().read(true).open(json_path)?;

        let source: rustdoc_types::Crate = serde_json::from_reader(file)?;
        if source.format_version != FORMAT_VERSION {
            return Err(Error::VersionMismatch {
                expected: FORMAT_VERSION,
                found: source.format_version,
            });
        }

        let root = source
            .index
            .get(&source.root)
            .ok_or_else(|| Error::CouldNotFind(source.root.clone()))?
            .clone();

        let builder = Builder {
            source,
            root,
            crate_replacement,
        };

        Ok(builder)
    }
}

impl Builder {
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

        let all_items: BTreeMap<String, Arc<Item>> =
            cache.items.into_iter().map(|(k, v)| (k.0, v)).collect();

        Ok(Crate {
            root,
            crate_version: self.source.crate_version,
            all_items: all_items.into_values().collect(),
            summaries: vec![],
            external_crates: vec![],
        })
    }
}

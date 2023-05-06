use std::{
    io::Write,
    path::{Path, PathBuf},
};

use thiserror::Error;

use crate::builder::{Builder, BuilderOpts};

impl BuilderOpts {
    pub fn for_build_rs() -> Option<Self> {
        let manifest_dir = PathBuf::from(std::env::var("CARGO_MANIFEST_DIR").unwrap_or_default());
        if !ensure_there_is_no_cycle() {
            return None;
        }
        Some(
            Self::default()
                .target_dir(manifest_dir.join("erised_target"))
                .manifest_dir(manifest_dir),
        )
    }
}

#[derive(Error, Debug)]
pub enum StaticError {
    #[error("Could not generate rust reflection")]
    CouldNotGenerateReflection(#[from] crate::builder::Error),
    #[error(transparent)]
    IO(#[from] std::io::Error),
}

impl Builder {
    pub fn build_static_reflection(self, target: impl AsRef<Path>) -> Result<(), StaticError> {
        let krate = self.build()?;
        let info = krate.generate_static();

        let mut file = std::fs::OpenOptions::new()
            .read(true)
            .write(true)
            .create(true)
            .truncate(true)
            .open(target)?;

        write!(&mut file, "{info}")?;

        file.flush()?;
        Ok(())
    }
}
fn ensure_there_is_no_cycle() -> bool {
    // This part checks if we are not trying to build the reflection during building reflection.
    let rust_recursion_count = std::env::var("RUST_RECURSION_COUNT").unwrap_or_default();
    let rust_recursion_count: usize = rust_recursion_count.parse().unwrap_or_default();
    rust_recursion_count <= 1
}

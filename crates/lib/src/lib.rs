pub mod heap_types;

pub mod types;

pub mod destruct;

pub mod builder;

mod access;
mod static_gen;

pub(crate) mod utils;

pub mod build {
    use std::{
        io::Write,
        path::{Path, PathBuf},
    };

    use crate::builder::{Builder, BuilderOpts};

    impl BuilderOpts {
        pub fn for_build_rs() -> Option<Self> {
            let manifest_dir =
                PathBuf::from(std::env::var("CARGO_MANIFEST_DIR").unwrap_or_default());
            if !ensure_there_is_no_cycle() {
                return None;
            }
            Some(Self::default().manifest_dir(manifest_dir))
        }
    }

    impl Builder {
        pub fn build_static(self, target: impl AsRef<Path>) {
            let krate = self.build().expect("Could not generate rust reflection");
            let info = krate.generate_static();

            let mut file = std::fs::OpenOptions::new()
                .read(true)
                .write(true)
                .create(true)
                .truncate(true)
                .open(target)
                .expect("Could not open target file");

            write!(&mut file, "{info}").expect("Could not write to target file");

            file.flush().expect("Could not flush the target file");
        }
    }
    fn ensure_there_is_no_cycle() -> bool {
        // This part checks if we are not trying to build the reflection during building reflection.
        let rust_recursion_count = std::env::var("RUST_RECURSION_COUNT").unwrap_or_default();
        let rust_recursion_count: usize = rust_recursion_count.parse().unwrap_or_default();
        rust_recursion_count <= 1
    }
}

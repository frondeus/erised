use erised::builder::{BuilderOpts, CrateReplacement};

fn main() {
    // Returns none when build.rs is called from rustdoc itself to avoid recursion
    if let Some(builder) = BuilderOpts::for_build_rs() {
        builder
            .replace_with_crate(CrateReplacement::Root)
            .load()
            .expect("Could not generate JSON")
            .build_static_reflection("./src/reflection.rs");
    }
}

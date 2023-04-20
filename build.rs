use std::path::PathBuf;

fn main() {
    let manifest_dir = PathBuf::from(std::env::var("CARGO_MANIFEST_DIR").unwrap());
    let rust_recursion_count = std::env::var("RUST_RECURSION_COUNT").unwrap();
    let rust_recursion_count: usize = rust_recursion_count.parse().unwrap();
    if rust_recursion_count > 1 {
        return;
    }

    erised::build::build_reflection_opts(|b| {
        b.manifest_path(manifest_dir.join("Cargo.toml"))
            .target_dir(manifest_dir.join("erised_target"))
    })
    .unwrap();
}

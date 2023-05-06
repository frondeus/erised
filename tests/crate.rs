use std::path::PathBuf;

use erised::builder::BuilderOpts;
use erised_tests::pretty_print;

#[test]
fn erised() {
    let manifest_dir = PathBuf::from(std::env::var("CARGO_MANIFEST_DIR").unwrap_or_default());

    let krate = BuilderOpts::default()
        .manifest_dir(manifest_dir)
        .package("erised")
        .load()
        .expect("JSON info")
        .build()
        .expect("Rust info");

    let info = krate.generate_static();

    let info = pretty_print(info).expect("formatted");

    insta::assert_display_snapshot!(info);
}

#[test]
fn erised_tests() {
    let manifest_dir = PathBuf::from(std::env::var("CARGO_MANIFEST_DIR").unwrap_or_default());

    let krate = BuilderOpts::default()
        .manifest_dir(manifest_dir)
        .package("erised")
        .load()
        .expect("JSON info")
        .build()
        .expect("Rust info");

    let info = krate.generate_static();

    let info = pretty_print(info).expect("formatted");

    insta::assert_display_snapshot!(info);
}

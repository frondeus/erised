use std::path::PathBuf;

use erised::builder::Builder;
use erised_tests::pretty_print;

#[test]
fn erised() {
    let manifest_dir = PathBuf::from(std::env::var("CARGO_MANIFEST_DIR").unwrap_or_default());

    let krate: erised_tests::Crate = Builder::load(manifest_dir, |o| o.package("erised"))
        .unwrap()
        .expect("Some crate");

    dbg!(&krate);
    let info = krate.generate_static();

    // let info = pretty_print(info).expect("formatted");

    insta::assert_display_snapshot!(info);
}

#[test]
fn erised_tests() {
    let manifest_dir = PathBuf::from(std::env::var("CARGO_MANIFEST_DIR").unwrap_or_default());

    let krate: erised_tests::Crate = Builder::load(manifest_dir, |o| o)
        .unwrap()
        .expect("Some crate");

    dbg!(&krate);
    let info = krate.generate_static();

    let info = pretty_print(info).expect("formatted");

    insta::assert_display_snapshot!(info);
}

use std::path::PathBuf;

use erised::{builder::Builder, destruct::ToTokens};
use erised_tests::pretty_print_item;

#[test]
fn todo() {
    let manifest_dir = PathBuf::from(std::env::var("CARGO_MANIFEST_DIR").unwrap_or_default());

    let krate: erised_tests::Crate = Builder::load(manifest_dir, |o| o.package("erised"))
        .unwrap()
        .expect("Some crate");

    dbg!(&krate);
    let info = krate.to_tokens();

    let info = pretty_print_item(info).expect("formatted");

    insta::assert_display_snapshot!(info);
}

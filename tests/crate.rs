use std::path::PathBuf;

use erised::{builder::Builder, destruct::ToTokens};
use erised_tests::pretty_print_item;

#[test]
fn todo() {
    let manifest_dir = PathBuf::from(std::env::var("CARGO_MANIFEST_DIR").unwrap_or_default());

    let info = Builder::load(manifest_dir, |o| o)
        .unwrap()
        .expect("Some crate")
        .to_tokens();

    let info = pretty_print_item(info).expect("formatted");

    insta::assert_display_snapshot!(info);
}

use std::{collections::HashSet, path::PathBuf};

use erised::{builder::BuilderOpts, visitor::Visitor};
use erised_tests::pretty_print;

#[test]
fn visitor() {
    #[derive(Default)]
    struct MyVisitor {
        visited: HashSet<erised::heap_types::Id>,
    }
    impl Visitor for MyVisitor {
        fn visit_item(&mut self, item: &erised_tests::Item) {
            let id = &item.meta().id;
            let visited = self.visited.get(id).is_some();
            if !visited {
                self.visited.insert(id.clone());
                erised::visitor::visit_item(self, item);
            }
        }
    }

    let manifest_dir = PathBuf::from(std::env::var("CARGO_MANIFEST_DIR").unwrap_or_default());

    let krate = BuilderOpts::default()
        .manifest_dir(manifest_dir)
        .package("erised")
        .load()
        .expect("JSON info")
        .build()
        .expect("Rust info");
    MyVisitor::default().visit_crate(&krate);
}

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

    dbg!(&krate);
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

    dbg!(&krate);
    let info = krate.generate_static();

    let info = pretty_print(info).expect("formatted");

    insta::assert_display_snapshot!(info);
}

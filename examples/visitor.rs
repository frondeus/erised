use std::{collections::HashSet, path::PathBuf};

use erised::heap_types::{Item, ItemMeta};
use erised::{builder::BuilderOpts, visitor::Visitor};

#[derive(Default)]
struct MyVisitor {
    visited: HashSet<erised::heap_types::Id>,
}

impl Visitor for MyVisitor {
    fn visit_item_meta(&mut self, meta: &ItemMeta) {
        if let Some(summary) = meta.summary.as_ref() {
            let path = summary.path.join("::");
            println!("Visited: {path}");
        }
    }

    fn visit_item(&mut self, item: &Item) {
        // Because there are cycles in the graph we want to make sure we won't run out of the stack.
        let id = &item.meta().id;
        if self.visited.get(id).is_some() {
            return;
        }

        // If not visited, then use default visitor implementation
        self.visited.insert(id.clone());
        erised::visitor::visit_item(self, item);
    }
}

fn main() {
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

use erised::{
    heap_types::*,
    visitor::{CycleDetector, Visitor},
};

use crate::{
    extra_gen::ExtraGenerator, static_items_gen::StaticItemsGenerator,
    to_tokens_gen::ToTokensGenerator, visitor::VisitorGenerator,
};

#[derive(Default)]
pub struct ModuleFinder {
    cycles: CycleDetector,
    pub static_items: StaticItemsGenerator,
    pub to_tokens: ToTokensGenerator,
    pub extra: ExtraGenerator,
    pub visitor: VisitorGenerator,
}

impl Visitor for ModuleFinder {
    fn visit_module(&mut self, module: &Module) {
        if module.name == "codegen_input" {
            self.static_items.visit_module(module);
            self.to_tokens.visit_module(module);
            self.extra.visit_module(module);
            self.visitor.visit_module(module);
        } else {
            erised::visitor::visit_module(self, module);
        }
    }

    fn visit_item(&mut self, item: &Item) {
        if self.cycles.was_visited(item) {
            return;
        }
        erised::visitor::visit_item(self, item);
    }
}

use std::collections::HashMap;

use erised::heap_types::*;
use erised::visitor::{CycleDetector, Visitor};
use inflector::Inflector;
use proc_macro2::{Ident, Literal, TokenStream, TokenTree};
use quote::format_ident;
use quote::TokenStreamExt;

use crate::Generator;

struct Keywords;

impl Keywords {
    fn replace(&self, s: &mut String) {
        let mut keywords = HashMap::new();
        keywords.insert("static", "statik");
        keywords.insert("impl", "imp");
        keywords.insert("trait", "trait_");
        keywords.insert("enum", "enum_");
        keywords.insert("struct", "struct_");
        keywords.insert("crate", "krate");
        keywords.insert("type", "ty");

        if let Some(kw) = keywords.get(s.as_str()) {
            *s = kw.to_string();
        }
    }
}

#[derive(Default)]
pub struct VisitorGenerator {
    pub output: TokenStream,
}

macro_rules! quote {
    ($output: expr, $($tokens: tt)*) => {{
        $output.append_all(quote::quote!($($tokens)*));
    }}
}

impl Generator for VisitorGenerator {
    fn output(&mut self) -> &mut TokenStream {
        &mut self.output
    }
}

impl Visitor for VisitorGenerator {
    fn visit_module(&mut self, module: &Module) {
        let mut state = Default::default();
        let mut default = DefaultVisitor::new(&mut state);
        // default.visit_module(module);

        for item in &module.items {
            default.visit_identifiable(item);
        }

        let mut out = TokenStream::default();
        for action in &default.state.actions {
            match action {
                Action::GenFuncWithDefault(name) => {
                    quote!(out, fn gen_func_with_default(#name: ()) {});
                }
                Action::GenOnlyFunc(name) => {
                    quote!(out, fn gen_only_func(#name: ()) {});
                }
                Action::GenUse(name) => {
                    quote!(out, fn gen_use(#name: ()) {});
                }
                Action::EndFunc => {
                    quote!(out, fn gen_end() {});
                }
            }
        }
        // let default = default.output;

        quote!(self.output,
            use crate::heap_types::*;
            pub use crate::utils::CycleDetector;

            #out
            // pub trait Visitor {
            // }
        );
    }
    // fn visit_item(&mut self, item: &Item) {

    // }
}

#[derive(Default)]
pub struct DefaultVisitorState {
    cycles: CycleDetector,
    actions: Vec<Action>,
    cache: HashMap<Id, Action>,
}
pub struct DefaultVisitor<'a> {
    state: &'a mut DefaultVisitorState,
}

impl<'a> DefaultVisitor<'a> {
    pub fn new(state: &'a mut DefaultVisitorState) -> Self {
        Self { state }
    }

    pub fn branch<F>(&mut self, f: F) -> DefaultVisitorState
    where
        F: for<'b> FnOnce(&mut DefaultVisitor<'b>),
    {
        let mut state: DefaultVisitorState = Default::default();
        state.cycles = self.state.cycles.clone();
        state.cache = self.state.cache.clone();
        let mut inner = DefaultVisitor { state: &mut state };
        f(&mut inner);
        self.state.cycles = state.cycles.clone();
        self.state.cache = state.cache.clone();
        state
    }

    fn visit_it(&mut self, name: Ident, id: &Id, state: DefaultVisitorState) {
        self.state
            .cache
            .insert(id.clone(), Action::GenUse(name.clone()));
        if !state.actions.is_empty() {
            self.state.actions.push(Action::GenFuncWithDefault(name));
            self.state.actions.extend(state.actions);
            self.state.actions.push(Action::EndFunc);
        } else {
            self.state.actions.push(Action::GenOnlyFunc(name));
        }
    }
}
impl<'a> Visitor for DefaultVisitor<'a> {
    fn visit_module(&mut self, _module: &Module) {}
    fn visit_impl(&mut self, _imp: &Impl) {}
    fn visit_item(&mut self, item: &Item) {
        if let Some(cached) = self.state.cache.get(&item.meta().id) {
            self.state.actions.push(cached.clone());
        }
        if self.state.cycles.was_visited(item) {
            return;
        }
        erised::visitor::visit_item(self, item);
    }
    fn visit_struct_field(&mut self, struct_field: &StructField) {
        println!("* {}", &struct_field.name);
        erised::visitor::visit_struct_field(self, struct_field);
        println!("* end {}", &struct_field.name);
    }
    fn visit_struct(&mut self, struct_: &Struct) {
        println!("{}", &struct_.name);
        let state = self.branch(|f| {
            erised::visitor::visit_struct(f, struct_);
        });
        println!("end {}", &struct_.name);

        self.visit_it(format_ident!("{}", struct_.name), &struct_.meta.id, state);
    }
    fn visit_enum(&mut self, enum_: &Enum) {
        println!("{}", &enum_.name);
        let state = self.branch(|f| {
            erised::visitor::visit_enum(f, enum_);
        });
        println!("end {}", &enum_.name);

        self.visit_it(format_ident!("{}", enum_.name), &enum_.meta.id, state);
    }
}

#[derive(Clone)]
enum Action {
    GenFuncWithDefault(Ident),
    GenOnlyFunc(Ident),
    GenUse(Ident),
    EndFunc,
}

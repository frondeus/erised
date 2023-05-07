use std::collections::HashMap;

use erised::heap_types::*;
use erised::visitor::{CycleDetector, Visitor};
use inflector::Inflector;
use proc_macro2::{Ident, Literal, TokenStream, TokenTree};
use quote::format_ident;
use quote::TokenStreamExt;

use crate::Generator;

#[derive(Default)]
pub struct VisitorGenerator {
    cycles: CycleDetector,
    pub output: TokenStream,
    pub default: DefaultVisitorGenerator,
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
        self.default.visit_module(module);
        erised::visitor::visit_module(self, module);
    }
    fn visit_struct(&mut self, s: &Struct) {
        let mut s_name = s.name.to_snake_case();

        Keywords.replace(&mut s_name);

        let arg_name = format_ident!("{}", s_name);
        let name = format_ident!("{}", s.name);
        let fn_name = format_ident!("visit_{}", s.name.to_snake_case());

        quote!(self.output, fn #fn_name(&mut self, #arg_name: &#name) {
            #fn_name(self, #arg_name);
        })
    }
    fn visit_enum(&mut self, e: &Enum) {
        let mut e_name = e.name.to_snake_case();

        Keywords.replace(&mut e_name);

        let arg_name = format_ident!("{}", e_name);
        let name = format_ident!("{}", e.name);
        let fn_name = format_ident!("visit_{}", e.name.to_snake_case());

        quote!(self.output, fn #fn_name(&mut self, #arg_name: &#name) {
            #fn_name(self, #arg_name);
        })
    }
    fn visit_item(&mut self, item: &Item) {
        if self.cycles.was_visited(item) {
            return;
        }
        erised::visitor::visit_item(self, item);
    }
}
#[derive(Default)]
pub struct DefaultVisitorGenerator {
    cycles: CycleDetector,
    pub output: TokenStream,
    recursion: usize,
}

impl Generator for DefaultVisitorGenerator {
    fn output(&mut self) -> &mut TokenStream {
        &mut self.output
    }
}

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

impl Visitor for DefaultVisitorGenerator {
    fn visit_struct(&mut self, s: &Struct) {
        if self.recursion != 0 {
            return;
        }

        self.recursion += 1;

        let mut s_name = s.name.to_snake_case();

        Keywords.replace(&mut s_name);

        let arg_name = format_ident!("{}", s_name);
        let name = format_ident!("{}", s.name);
        let fn_name = format_ident!("visit_{}", s.name.to_snake_case());

        let inner = self.branch(|codegen| {
            DefaultFieldVisitorGenerator::new(codegen, Some(arg_name.clone())).visit_struct(s);
        });

        quote!(self.output, pub fn #fn_name(vis: &mut (impl Visitor + ?Sized), #arg_name: &#name) {
            #inner
        });

        self.recursion -= 1;
    }
    fn visit_enum(&mut self, e: &Enum) {
        if self.recursion != 0 {
            return;
        }

        self.recursion += 1;

        let mut e_name = e.name.to_snake_case();

        Keywords.replace(&mut e_name);

        let arg_name = format_ident!("{}", e_name);
        let name = format_ident!("{}", e.name);
        let fn_name = format_ident!("visit_{}", e.name.to_snake_case());

        let mut variants_processed = 0;

        let inner = self.branch(|codegen| {
            let mut fv = DefaultVariantVisitorGenerator::new(codegen, name.clone());
            fv.visit_enum(e);
            variants_processed = fv.processed;
            quote!(codegen.output(), _ => ());
        });

        if variants_processed > 0 {
            quote!(self.output, pub fn #fn_name(vis: &mut (impl Visitor + ?Sized), #arg_name: &#name) {
                match #arg_name {
                    #inner
                }
            });
        }

        self.recursion -= 1;
    }
    fn visit_item(&mut self, item: &Item) {
        if self.cycles.was_visited(item) {
            return;
        }
        erised::visitor::visit_item(self, item);
    }
}

struct DefaultVariantVisitorGenerator<'a> {
    codegen: &'a mut DefaultVisitorGenerator,
    name: Ident,
    processed: usize,
    counter: usize,
}

impl<'a> DefaultVariantVisitorGenerator<'a> {
    fn new(codegen: &'a mut DefaultVisitorGenerator, name: Ident) -> Self {
        Self {
            codegen,
            name,
            processed: 0,
            counter: 0,
        }
    }
    fn branch<F>(&mut self, f: F) -> TokenStream
    where
        F: for<'b> FnOnce(&mut DefaultVariantVisitorGenerator<'b>),
    {
        self.codegen.branch(|codegen| {
            let mut fv = DefaultVariantVisitorGenerator::new(codegen, self.name.clone());
            fv.processed = self.processed;
            f(&mut fv);
            self.processed = fv.processed;
        })
    }
    fn get_idx(&mut self) -> usize {
        let idx = self.counter;
        self.counter += 1;
        idx
    }
}

impl<'a> Visitor for DefaultVariantVisitorGenerator<'a> {
    fn visit_variant(&mut self, variant: &Variant) {
        let variant_name = format_ident!("{}", &variant.name);
        let mut this_processed = false;

        let variant_args = self.branch(|codegen| match &variant.kind {
            VariantKind::Plain => (),
            VariantKind::Tuple(_) => {
                this_processed = true;
                let args = codegen.branch(|codegen| {
                    erised::visitor::visit_variant(codegen, variant);
                });
                quote!(codegen.codegen.output(), ( #args ));
            }
            VariantKind::Struct { .. } => {
                this_processed = true;
                let args = codegen.branch(|codegen| {
                    erised::visitor::visit_variant(codegen, variant);
                });
                quote!(codegen.codegen.output(), { #args });
            }
        });

        let variant_call = self.codegen.branch(|codegen| {
            let mut fv = DefaultFieldVisitorGenerator::new(codegen, None);
            fv.visit_variant(variant);
        });

        let name = &self.name;
        if this_processed && !variant_call.is_empty() {
            self.processed += 1;
            quote!(self.codegen.output(), #name::#variant_name #variant_args => {
                    #variant_call
                },);
        }
    }
    fn visit_struct_field(&mut self, field: &StructField) {
        let name = if !field.is_part_of_tuple {
            format_ident!("{}", field.name)
            // quote
        } else {
            format_ident!("_{}", self.get_idx())
            // quote!(self.codegen.output, #idx )
        };

        quote!(self.codegen.output(), #name,);
        // dbg!(&field);
        // panic!();
    }
    fn visit_item(&mut self, _item: &Item) {}
}

struct DefaultFieldVisitorGenerator<'a> {
    codegen: &'a mut DefaultVisitorGenerator,
    name: Option<Ident>,
    counter: usize,
}

impl<'a> DefaultFieldVisitorGenerator<'a> {
    fn new(codegen: &'a mut DefaultVisitorGenerator, name: Option<Ident>) -> Self {
        Self {
            codegen,
            name,
            counter: 0,
        }
    }
    fn get_idx(&mut self) -> usize {
        let idx = self.counter;
        self.counter += 1;
        idx
    }
}

impl<'a> Visitor for DefaultFieldVisitorGenerator<'a> {
    fn visit_struct_field(&mut self, field: &StructField) {
        if field.meta.attrs.contains(&"#[type_info(skip)]".to_owned()) {
            return;
        }

        let mut is_worthy = WorthinessChecker::default();
        is_worthy.visit_struct_field(field);
        let accessor_name = self.name.as_ref().map(|n| quote::quote!(#n.));
        let amp = self.name.as_ref().map(|_| quote::quote!(&));

        if let Some(ty) = is_worthy.path {
            let mut field_name = field.name.clone();
            Keywords.replace(&mut field_name);
            let field_name = if !field.is_part_of_tuple {
                field_name
            } else {
                format!("_{}", self.get_idx())
            };

            let mut singular_name = field_name.clone().to_singular();
            let field_name = format_ident!("{}", field_name);

            let ty_name = ty.name.to_snake_case();
            let ty_name = format_ident!("visit_{}", ty_name);

            match is_worthy.kind {
                FieldKind::Iterable => {
                    Keywords.replace(&mut singular_name);
                    let singular_name = format_ident!("{}", singular_name);

                    quote!(self.codegen.output, for #singular_name in #amp #accessor_name #field_name {
                        vis.#ty_name(#singular_name);
                    })
                }
                FieldKind::Optional => {
                    quote!(self.codegen.output, if let Some(#field_name) = #accessor_name #field_name.as_ref() {
                        vis.#ty_name(#field_name);
                    })
                }
                FieldKind::Standard => {
                    quote!(self.codegen.output,
                        vis.#ty_name(
                            #amp #accessor_name #field_name
                        );
                    );
                }
            }
        }
    }
}

#[derive(Default)]
struct WorthinessChecker {
    path: Option<Path>,
    kind: FieldKind,
    first: bool,
}

#[derive(Default)]
enum FieldKind {
    #[default]
    Standard,
    Iterable,
    Optional,
}

impl Visitor for WorthinessChecker {
    fn visit_resolved_path(&mut self, r: &Path) {
        if self.path.is_none() {
            if r.target.clone().as_item().is_some() {
                self.path = Some(r.clone());
            } else if !self.first {
                self.first = true;
                if r.name == "Vec" {
                    self.kind = FieldKind::Iterable;
                }
                if r.name == "Option" {
                    self.kind = FieldKind::Optional;
                }
                return erised::visitor::visit_resolved_path(self, r);
            }
        }
    }
}

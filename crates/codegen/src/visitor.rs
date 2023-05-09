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
        keywords.insert("for", "for_");
        keywords.insert("unsafe", "unsafe_");
        keywords.insert("const", "const_");

        if let Some(kw) = keywords.get(s.as_str()) {
            *s = kw.to_string();
        }
    }
    fn snake_case(&self, s: &String) -> String {
        let mut sn = s.to_snake_case();
        self.replace(&mut sn);
        sn
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

impl VisitorGenerator {}
impl Visitor for VisitorGenerator {
    fn visit_module(&mut self, module: &Module) {
        let mut state = Default::default();
        let mut out = TokenStream::default();
        let mut default = DefaultVisitor::new(&mut state, &mut out);

        for item in &module.items {
            default.visit_identifiable(item);
        }

        quote!(self.output,
            use crate::heap_types::*;
            pub use crate::utils::CycleDetector;

            #out
        );

        let mut inner = TokenStream::default();
        for item in &module.items {
            VisitorGen {
                out: &mut inner,
                saved: &state.saved,
            }
            .visit_identifiable(item);
        }

        quote!(self.output,
            pub trait Visitor {
                #inner
            }
        );
    }
}

pub struct VisitorGen<'a> {
    out: &'a mut TokenStream,
    saved: &'a HashMap<Id, String>,
}

impl<'a> VisitorGen<'a> {
    fn process_entity(&mut self, name: &String, id: &Id) {
        let fn_name = format_ident!("visit_{}", &name.to_snake_case());
        let attr_name = format_ident!("{}", Keywords.snake_case(name));
        let ty = format_ident!("{}", name);

        let body = self
            .saved
            .get(id)
            .map(|_| quote::quote!(#fn_name(self, #attr_name)));

        quote!(self.out,
            fn #fn_name(&mut self, #attr_name: &#ty) { #body }
        );
    }
}
impl<'a> Visitor for VisitorGen<'a> {
    fn visit_struct(&mut self, s: &Struct) {
        let name = &s.name;
        let id = &s.meta.id;
        if s.meta.attrs.contains(&"#[type_info(skip)]".to_owned()) {
            return;
        }
        self.process_entity(name, id);
    }

    fn visit_enum(&mut self, e: &Enum) {
        let name = &e.name;
        let id = &e.meta.id;
        if e.meta.attrs.contains(&"#[type_info(skip)]".to_owned()) {
            return;
        }
        self.process_entity(name, id);
    }
}

#[derive(Default, Clone)]
pub struct DefaultVisitorState {
    cycles: CycleDetector,
    saved: HashMap<Id, String>,
}
pub struct DefaultVisitor<'a> {
    state: &'a mut DefaultVisitorState,
    out: &'a mut TokenStream,
}

impl<'a> DefaultVisitor<'a> {
    pub fn new(state: &'a mut DefaultVisitorState, out: &'a mut TokenStream) -> Self {
        Self { state, out }
    }

    pub fn branch<F>(&mut self, f: F) -> TokenStream
    where
        F: for<'b> FnOnce(&mut DefaultVisitor<'b>),
    {
        let mut state = self.state.clone();
        let out = std::mem::take(self.out);
        let mut inner = DefaultVisitor {
            state: &mut state,
            out: self.out,
        };
        f(&mut inner);
        let out = std::mem::replace(self.out, out);
        *self.state = state;
        out
    }

    fn process_type(&mut self, ty: FieldInfo, attr: Option<&Ident>, field_name: &String) {
        let fn_name = format_ident!("visit_{}", &ty.name.to_snake_case());
        let field_name: TokenStream = match field_name.parse::<usize>() {
            Ok(idx) => {
                let idx = TokenTree::Literal(Literal::usize_unsuffixed(idx));
                idx.into()
            }
            Err(_) => {
                let q = format_ident!("{}", field_name);
                quote::quote!(#q)
            }
        };

        let len = ty.process.len();
        let mut inner = TokenStream::default();
        // Iteration from bottom to the top
        for (idx, p) in ty.process.into_iter().enumerate() {
            let is_last = idx == len - 1;

            let access = if is_last && attr.is_some() {
                quote::quote!(&#attr.#field_name)
            } else {
                quote::quote!(#field_name)
            };

            inner = match p {
                FieldProcess::Use => {
                    quote::quote!(
                        vis.#fn_name(#access);
                    )
                }
                FieldProcess::Iterate => {
                    quote::quote!(for #field_name in #access {
                        #inner
                    })
                }
                FieldProcess::IfLet => {
                    quote::quote!(if let Some(#field_name) = #access.as_ref() {
                        #inner
                    })
                }
                FieldProcess::Upgrade => {
                    quote::quote!(if let Some(#field_name) = #access.upgrade().as_ref() {
                        #inner
                    })
                }
            };
        }

        quote!(self.out, #inner);
    }
}
impl<'a> Visitor for DefaultVisitor<'a> {
    fn visit_module(&mut self, _module: &Module) {}
    fn visit_impl(&mut self, _imp: &Impl) {}
    fn visit_item(&mut self, item: &Item) {
        if self.state.cycles.was_visited(item) {
            return;
        }
        erised::visitor::visit_item(self, item);
    }
    fn visit_struct(&mut self, struct_: &Struct) {
        let fn_name = format_ident!("visit_{}", &struct_.name.to_snake_case());
        let attr = format_ident!("{}", Keywords.snake_case(&struct_.name));
        let ty = format_ident!("{}", struct_.name);
        let mut at_least_one_field = false;
        let fields: Vec<_> = match &struct_.kind {
            StructKind::Unit => Default::default(),
            StructKind::Tuple(fields) => fields.into_iter().filter_map(|f| f.as_ref()).collect(),
            StructKind::Plain { fields, .. } => fields.iter().collect(),
        };
        let body = self.branch(|f| {
            for field in fields {
                let field_name = &field.name;
                let mut vis = TypeVisitor::default();
                vis.visit_struct_field(field);
                if let Some(ty) = vis.ty {
                    at_least_one_field = true;
                    f.process_type(ty, Some(&attr), field_name);
                }
            }
        });

        if at_least_one_field {
            self.state
                .saved
                .insert(struct_.meta.id.clone(), struct_.name.clone());
            quote!(self.out, pub fn #fn_name(vis: &mut (impl Visitor + ?Sized), #attr: &#ty) {
                #body
            });
        }
    }
    fn visit_enum(&mut self, enum_: &Enum) {
        let fn_name = format_ident!("visit_{}", &enum_.name.to_snake_case());
        let attr = format_ident!("{}", Keywords.snake_case(&enum_.name));
        let ty = format_ident!("{}", enum_.name);
        let mut at_least_one_variant = false;
        let body = self.branch(|f| {
            for variant in &enum_.variants {
                let variant_name = format_ident!("{}", variant.name);

                let field_info: Vec<Option<(String, FieldInfo)>> = match &variant.kind {
                    VariantKind::Plain => Default::default(),
                    VariantKind::Tuple(fields) => fields
                        .iter()
                        .map(|field| match field.as_ref() {
                            Some(field) => {
                                let mut vis = TypeVisitor::default();
                                vis.visit_struct_field(field);
                                vis.ty.map(|ty| (field.name.clone(), ty))
                            }
                            None => None,
                        })
                        .collect(),
                    VariantKind::Struct { fields, .. } => fields
                        .iter()
                        .map(|field| {
                            let mut vis = TypeVisitor::default();
                            vis.visit_struct_field(field);
                            vis.ty.map(|ty| (field.name.clone(), ty))
                        })
                        .collect(),
                };

                let mut at_least_one_field = false;
                let variant_destruct = f.branch(|f| {
                    let inner = f.branch(|f| {
                        for field in &field_info {
                            match field {
                                Some((field_name, info)) => {
                                    at_least_one_field = true;
                                    // let field_name = Keywords.snake_case(field_name);
                                    let field_name = match field_name.parse::<usize>() {
                                        Ok(_) => {
                                            format_ident!("{}", Keywords.snake_case(&info.name))
                                        }
                                        Err(_) => {
                                            // if let VariantKind::Tuple
                                            format_ident!("{}", field_name)
                                        }
                                    };
                                    quote!(f.out, #field_name,);
                                }
                                None => {
                                    if let VariantKind::Tuple(_) = &variant.kind {
                                        quote!(f.out, _,);
                                    }
                                }
                            }
                        }
                    });
                    match variant.kind {
                        VariantKind::Plain => (),
                        VariantKind::Tuple(_) => quote!(f.out, (#inner)),
                        VariantKind::Struct { .. } => quote!(f.out, { #inner .. }),
                    }
                });
                let variant_construct = f.branch(|f| {
                    for field in &field_info {
                        if let Some((field_name, field)) = field.as_ref() {
                            let field_name = match field_name.parse::<usize>() {
                                Ok(_) => Keywords.snake_case(&field.name),
                                Err(_) => {
                                    // if let VariantKind::Tuple
                                    field_name.clone()
                                }
                            };
                            f.process_type(field.clone(), None, &field_name);
                        }
                    }
                });
                if at_least_one_field {
                    at_least_one_variant = true;
                    quote!(f.out, #ty::#variant_name #variant_destruct => {
                        #variant_construct
                    },);
                }
            }
        });

        if at_least_one_variant {
            quote!(self.out, pub fn #fn_name(vis: &mut (impl Visitor + ?Sized), #attr: &#ty) {
                match &#attr {
                    #body
                    _ => ()
                }
            });
            self.state
                .saved
                .insert(enum_.meta.id.clone(), enum_.name.clone());
        }
    }
}

#[derive(Default)]
struct TypeVisitor {
    ty: Option<FieldInfo>,
}

#[derive(Clone)]
struct FieldInfo {
    name: String,
    process: Vec<FieldProcess>,
}

#[derive(Clone)]
enum FieldProcess {
    Iterate,
    IfLet,
    Upgrade,

    Use, // Artificial one that helps with iteration
}

impl Visitor for TypeVisitor {
    fn visit_item(&mut self, _: &Item) {}
    fn visit_struct_field(&mut self, struct_field: &StructField) {
        if struct_field
            .meta
            .attrs
            .contains(&"#[type_info(skip)]".to_owned())
        {
            return;
        }

        erised::visitor::visit_struct_field(self, struct_field);
    }
    fn visit_path(&mut self, r: &Path) {
        if self.ty.is_some() {
            return;
        }
        if let Some(_) = r.target.clone().as_item() {
            self.ty = Some(FieldInfo {
                name: r.name.clone(),
                process: vec![FieldProcess::Use],
            });
            return;
        }
        if let Some(args) = r.args.as_ref() {
            self.visit_generic_args(&args);
        }

        if let Some(ty) = self.ty.as_mut() {
            match r.name.as_str() {
                "Vec" => {
                    ty.process.push(FieldProcess::Iterate);
                }
                "Weak" => {
                    ty.process.push(FieldProcess::Upgrade);
                }
                "Option" => {
                    ty.process.push(FieldProcess::IfLet);
                }
                _ => (),
            }
        }
    }
}

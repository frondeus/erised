use erised::{
    heap_types::*,
    visitor::{CycleDetector, Visitor},
};
use inflector::Inflector;
use proc_macro2::{Punct, Spacing, TokenStream};
use quote::format_ident;
use quote::TokenStreamExt;

#[derive(Default)]
pub struct ExtraGenerator {
    cycles: CycleDetector,
    pub output: TokenStream,
}

macro_rules! quote {
    ($output: expr, $($tokens: tt)*) => {
        $output.append_all(quote::quote!($($tokens)*));
    }
}

impl ExtraGenerator {
    fn branch(&mut self, f: impl FnOnce(&mut Self)) -> TokenStream {
        let saved = std::mem::take(&mut self.output);
        f(self);
        let inner = std::mem::replace(&mut self.output, saved);
        inner
    }
}

impl Visitor for ExtraGenerator {
    fn visit_module(&mut self, module: &Module) {
        erised::visitor::visit_module(self, module);
    }
    fn visit_item(&mut self, item: &Item) {
        if self.cycles.was_visited(item) {
            return;
        }
        erised::visitor::visit_item(self, item);
    }

    fn visit_enum(&mut self, e: &Enum) {
        let name = &e.name;
        // Let's check if the enum has been codegenerated
        if name.starts_with("Static") && name != "Static" {
            return;
        }
        if e.meta.attrs.contains(&"#[type_info(skip)]".to_owned()) {
            return;
        }
        let name = format_ident!("{name}");

        let as_variant = self.branch(|codegen| {
            for variant in &e.variants {
                let variant_name = format_ident!("{}", variant.name);
                let fn_name = format_ident!("as_{}", variant.name.to_snake_case());
                let ty = codegen.branch(|codegen| match &variant.kind {
                    VariantKind::Plain => {
                        quote!(codegen.output, ());
                    }
                    VariantKind::Tuple(fields) => {
                        let inner = codegen.branch(|codegen| {
                            let mut fv = Typper {
                                codegen,
                                trailing_comma: true,
                            };
                            for field in fields {
                                if let Some(field) = field.as_ref() {
                                    fv.visit_struct_field(field);
                                }
                            }
                        });

                        if fields.len() == 1 {
                            quote!(codegen.output, #inner);
                        } else {
                            quote!(codegen.output, ( #inner ));
                        }
                    }
                    VariantKind::Struct { fields, .. } => {
                        let inner = codegen.branch(|codegen| {
                            let mut fv = Typper {
                                codegen,
                                trailing_comma: true,
                            };

                            for field in fields {
                                fv.visit_struct_field(field);
                            }
                        });

                        if fields.len() == 1 {
                            quote!(codegen.output, #inner);
                        } else {
                            quote!(codegen.output, ( #inner ));
                        }
                    }
                });
                let destruct = codegen.branch(|codegen| match &variant.kind {
                    VariantKind::Plain => (),
                    VariantKind::Tuple(fields) => {
                        let inner = codegen.branch(|codegen| {
                            let mut fv = Destructor {
                                codegen,
                                trailing_comma: true,
                                counter: 0,
                            };
                            for field in fields {
                                if let Some(field) = field.as_ref() {
                                    fv.visit_struct_field(field);
                                }
                            }
                        });

                        quote!(codegen.output, ( #inner ));
                    }
                    VariantKind::Struct { fields, .. } => {
                        let inner = codegen.branch(|codegen| {
                            let mut fv = Destructor {
                                codegen,
                                trailing_comma: true,
                                counter: 0,
                            };

                            for field in fields {
                                fv.visit_struct_field(field);
                            }
                        });

                        quote!(codegen.output, { #inner });
                    }
                });
                let construct = codegen.branch(|codegen| match &variant.kind {
                    VariantKind::Plain => {
                        quote!(codegen.output, ());
                    }
                    VariantKind::Tuple(fields) => {
                        let inner = codegen.branch(|codegen| {
                            let mut fv = Constructor {
                                codegen,
                                trailing_comma: true,
                                counter: 0,
                            };
                            for field in fields {
                                if let Some(field) = field.as_ref() {
                                    fv.visit_struct_field(field);
                                }
                            }
                        });

                        if fields.len() == 1 {
                            quote!(codegen.output, #inner);
                        } else {
                            quote!(codegen.output, ( #inner ));
                        }
                    }
                    VariantKind::Struct { fields, .. } => {
                        let inner = codegen.branch(|codegen| {
                            let mut fv = Constructor {
                                codegen,
                                trailing_comma: true,
                                counter: 0,
                            };

                            for field in fields {
                                fv.visit_struct_field(field);
                            }
                        });

                        if fields.len() == 1 {
                            quote!(codegen.output, #inner);
                        } else {
                            quote!(codegen.output, ( #inner ));
                        }
                    }
                });

                quote!(codegen.output,
                    pub fn #fn_name(self) -> Option<#ty> {
                        match self {
                            Self::#variant_name #destruct => Some(#construct),
                            _ => None
                        }
                    }
                );
            }
        });

        quote!(self.output,
            impl #name {
                #as_variant
            }
        );
    }
}

struct Destructor<'a> {
    codegen: &'a mut ExtraGenerator,
    trailing_comma: bool,
    counter: usize,
}

impl<'a> Destructor<'a> {
    fn trailing_comma(&mut self) {
        if self.trailing_comma {
            quote!(self.codegen.output, ,);
        }
    }
    fn get_idx(&mut self) -> usize {
        let idx = self.counter;
        self.counter += 1;
        idx
    }
}

impl<'a> Visitor for Destructor<'a> {
    fn visit_struct_field(&mut self, field: &StructField) {
        if field.meta.attrs.contains(&"#[type_info(skip)]".to_owned()) {
            return;
        }

        if !field.is_part_of_tuple {
            let name = format_ident!("{}", field.name);
            quote!(self.codegen.output, #name);
        } else {
            let idx = format_ident!("_{}", self.get_idx());
            quote!(self.codegen.output, #idx);
        }

        self.trailing_comma();
        // self.visit_item_meta(&field.meta);
        // self.visit_type(&field.ty);
    }
    fn visit_item(&mut self, _: &Item) {}
}

struct Constructor<'a> {
    codegen: &'a mut ExtraGenerator,
    trailing_comma: bool,
    counter: usize,
}

impl<'a> Constructor<'a> {
    fn trailing_comma(&mut self) {
        if self.trailing_comma {
            quote!(self.codegen.output, ,);
        }
    }
    fn get_idx(&mut self) -> usize {
        let idx = self.counter;
        self.counter += 1;
        idx
    }
}

impl<'a> Visitor for Constructor<'a> {
    fn visit_struct_field(&mut self, field: &StructField) {
        if field.meta.attrs.contains(&"#[type_info(skip)]".to_owned()) {
            return;
        }

        let name = if !field.is_part_of_tuple {
            format_ident!("{}", field.name)
        } else {
            format_ident!("_{}", self.get_idx())
        };

        // let hash = Punct::new('#', Spacing::Joint);
        // let hashed_name = quote::quote!(#hash #name);

        quote!(self.codegen.output, #name );

        self.trailing_comma();
    }
    fn visit_item(&mut self, _: &Item) {}
}

struct Typper<'a> {
    codegen: &'a mut ExtraGenerator,
    trailing_comma: bool,
}

impl<'a> Typper<'a> {
    fn trailing_comma(&mut self) {
        if self.trailing_comma {
            quote!(self.codegen.output, ,);
        }
    }
    fn branch<F>(&mut self, f: F) -> TokenStream
    where
        F: for<'b> FnOnce(&mut Typper<'b>),
    {
        self.codegen.branch(|codegen| {
            let mut field_visitor = Typper {
                codegen,
                trailing_comma: true,
            };
            f(&mut field_visitor);
        })
    }
}

impl<'a> Visitor for Typper<'a> {
    fn visit_struct_field(&mut self, field: &StructField) {
        if field.meta.attrs.contains(&"#[type_info(skip)]".to_owned()) {
            return;
        }

        self.visit_type(&field.ty);
    }
    fn visit_item(&mut self, item: &Item) {}
    fn visit_type(&mut self, ty: &Type) {
        match ty {
            Type::ResolvedPath(path) => match &path.target {
                Identifiable::Item(i) => {
                    let name = format_ident!("{}", path.name);
                    quote!(self.codegen.output, #name);
                    dbg!(&path);
                }
                Identifiable::Summary(_) => {
                    let args = match &path.args {
                        None => None,
                        Some(args) if args.is_empty() => None,
                        Some(args) => {
                            let inner = self.branch(|fv| {
                                erised::visitor::visit_generic_args(fv, args);
                            });
                            Some(quote::quote!(< #inner >))
                        }
                    };
                    let name = format_ident!("{}", path.name);
                    quote!(self.codegen.output, #name #args);
                }
            },
            Type::DynTrait(_) => todo!(),
            Type::Generic(s) => {
                let s = format_ident!("{}", s);
                quote!(
                    self.codegen.output,
                    #s
                );
            }
            Type::Primitive(u) => {
                let u = format_ident!("{u}");
                quote!(self.codegen.output, #u);
            }
            Type::FunctionPointer(_) => todo!(),
            Type::Tuple(t) => {
                let inner = self.branch(|fv| {
                    erised::visitor::visit_tuple(fv, t);
                });
                quote!(self.codegen.output, ( #inner ));
            }
            Type::Slice(_) => todo!(),
            Type::Array { type_, len } => todo!(),
            Type::ImplTrait(_) => todo!(),
            Type::Infer => todo!(),
            Type::RawPointer { mutable, type_ } => todo!(),
            Type::BorrowedRef {
                lifetime,
                mutable,
                type_,
            } => todo!(),
            Type::QualifiedPath {
                name,
                args,
                self_type,
                trait_,
            } => {
                todo!()
            }
        }
        self.trailing_comma();
    }
}

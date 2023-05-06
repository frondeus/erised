use erised::{
    heap_types::*,
    visitor::{CycleDetector, Visitor},
};
use inflector::Inflector;
use proc_macro2::TokenStream;
use quote::format_ident;
use quote::TokenStreamExt;

#[derive(Default)]
pub struct StaticItemsGenerator {
    cycles: CycleDetector,
    pub output: TokenStream,
}

macro_rules! quote {
    ($output: expr, $($tokens: tt)*) => {
        $output.append_all(quote::quote!($($tokens)*));
    }
}

impl StaticItemsGenerator {
    fn branch(&mut self, f: impl FnOnce(&mut Self)) -> TokenStream {
        let saved = std::mem::take(&mut self.output);
        f(self);
        std::mem::replace(&mut self.output, saved)
    }
}

impl Visitor for StaticItemsGenerator {
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
        let static_name = format_ident!("{name}");
        let inner = self.branch(|codegen| {
            for variant in &e.variants {
                let variant_name = format_ident!("{}", &variant.name);
                quote!(codegen.output, #variant_name);

                match &variant.kind {
                    VariantKind::Plain => {
                        quote!(codegen.output, ,);
                    }
                    VariantKind::Tuple(fields) => {
                        let inner = codegen.branch(|codegen| {
                            let mut field_visitor = FieldVisitor {
                                codegen,
                                trailing_comma: true,
                            };
                            for field in fields {
                                if let Some(field) = field.as_ref() {
                                    field_visitor.visit_struct_field(field);
                                }
                            }
                        });
                        quote!(codegen.output, ( #inner ), );
                    }
                    VariantKind::Struct { fields, .. } => {
                        let inner = codegen.branch(|codegen| {
                            let mut field_visitor = FieldVisitor {
                                codegen,
                                trailing_comma: true,
                            };
                            for field in fields {
                                field_visitor.visit_struct_field(field);
                            }
                        });
                        quote!(codegen.output, { #inner }, );
                    }
                }
            }
        });

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

        #[derive(Debug, PartialEq, Eq, Hash, Clone, Copy)]
        pub enum #static_name {
            #inner
        }

        impl #static_name {
            #as_variant
        }
        );
    }

    fn visit_struct(&mut self, strukt: &Struct) {
        let name = &strukt.name;
        // Let's check if the struct has been codegenerated
        if name.starts_with("Static") && name != "Static" {
            return;
        }
        if strukt.meta.attrs.contains(&"#[type_info(skip)]".to_owned()) {
            return;
        }
        let static_name = format_ident!("{name}");
        quote!(self.output,
            #[derive(Debug, PartialEq, Eq, Hash, Clone, Copy)]
            pub struct #static_name);

        match &strukt.kind {
            StructKind::Tuple(_) => {
                let inner = self.branch(|codegen| {
                    let mut field_visitor = FieldVisitor {
                        codegen,
                        trailing_comma: true,
                    };
                    erised::visitor::visit_struct(&mut field_visitor, strukt);
                });
                quote!(self.output, ( #inner ); );
            }
            StructKind::Plain { .. } => {
                let inner = self.branch(|codegen| {
                    let mut field_visitor = FieldVisitor {
                        codegen,
                        trailing_comma: true,
                    };
                    erised::visitor::visit_struct(&mut field_visitor, strukt);
                });

                quote!(self.output, { #inner });
            }
            StructKind::Unit => (),
        }
    }
}

struct FieldVisitor<'a> {
    codegen: &'a mut StaticItemsGenerator,
    trailing_comma: bool,
}

impl<'a> FieldVisitor<'a> {
    fn branch<F>(&mut self, f: F) -> TokenStream
    where
        F: for<'b> FnOnce(&mut FieldVisitor<'b>),
    {
        self.codegen.branch(|codegen| {
            let mut field_visitor = FieldVisitor {
                codegen,
                trailing_comma: false,
            };
            f(&mut field_visitor);
        })
    }
    fn branch_with_comma<F>(&mut self, f: F) -> TokenStream
    where
        F: for<'b> FnOnce(&mut FieldVisitor<'b>),
    {
        self.codegen.branch(|codegen| {
            let mut field_visitor = FieldVisitor {
                codegen,
                trailing_comma: true,
            };
            f(&mut field_visitor);
        })
    }

    fn trailing_comma(&mut self) {
        if self.trailing_comma {
            quote!(self.codegen.output, ,);
        }
    }
}

impl<'a> Visitor for FieldVisitor<'a> {
    fn visit_struct_field(&mut self, field: &StructField) {
        if field.meta.attrs.contains(&"#[type_info(skip)]".to_owned()) {
            return;
        }
        match &field.meta.visibility {
            Visibility::Public => {
                quote!(self.codegen.output, pub);
            }
            Visibility::Default => (),
            Visibility::Crate => {
                quote!(self.codegen.output, pub(crate));
            }
            Visibility::Restricted { parent: _, path: _ } => todo!(),
        }
        if !field.is_part_of_tuple {
            let name = format_ident!("{}", field.name);
            quote!(self.codegen.output, #name:);
        }
        self.visit_type(&field.ty);
    }
    fn visit_type(&mut self, ty: &Type) {
        match ty {
            Type::ResolvedPath(path) => match &path.target {
                Identifiable::Item(_) => {
                    let static_name = format_ident!("{}", path.name);
                    quote!(self.codegen.output, #static_name);
                }
                Identifiable::Summary(_) => {
                    if path.name == "String" || path.name == "PathBuf" {
                        quote!(self.codegen.output, &'static str);
                        return self.trailing_comma();
                    }
                    if path.name == "Vec" {
                        let inner = path.args.as_ref().map(|args| {
                            self.branch(|fv| {
                                erised::visitor::visit_generic_args(fv, args);
                            })
                        });
                        quote!(self.codegen.output, &'static [
                            #inner
                        ]);
                        return self.trailing_comma();
                    }
                    if path.name == "Arc" || path.name == "Weak" || path.name == "Box" {
                        let inner = path.args.as_ref().map(|args| {
                            self.branch(|fv| {
                                erised::visitor::visit_generic_args(fv, args);
                            })
                        });
                        quote!(self.codegen.output, fn() ->
                            #inner
                        );
                        return self.trailing_comma();
                    }

                    let name = format_ident!("{}", path.name);
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
                    quote!(self.codegen.output, #name #args);
                }
            },
            Type::DynTrait(_) => todo!(),
            Type::Generic(_) => todo!(),
            Type::Primitive(u) => {
                let u = format_ident!("{u}");
                quote!(self.codegen.output, #u);
            }
            Type::FunctionPointer(_) => todo!(),
            Type::Tuple(t) => {
                let inner = self.branch_with_comma(|fv| {
                    erised::visitor::visit_tuple(fv, t);
                });
                quote!(self.codegen.output, ( #inner ));
            }
            Type::Slice(_) => todo!(),
            Type::Array { type_: _, len: _ } => todo!(),
            Type::ImplTrait(_) => todo!(),
            Type::Infer => todo!(),
            Type::RawPointer {
                mutable: _,
                type_: _,
            } => todo!(),
            Type::BorrowedRef {
                lifetime: _,
                mutable: _,
                type_: _,
            } => todo!(),
            Type::QualifiedPath {
                name: _,
                args: _,
                self_type: _,
                trait_: _,
            } => todo!(),
        }
        self.trailing_comma();
    }
    fn visit_item(&mut self, _: &Item) {}
}
struct Destructor<'a> {
    codegen: &'a mut StaticItemsGenerator,
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
    codegen: &'a mut StaticItemsGenerator,
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
    codegen: &'a mut StaticItemsGenerator,
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
    fn visit_item(&mut self, _item: &Item) {}
    fn visit_type(&mut self, ty: &Type) {
        match ty {
            Type::ResolvedPath(path) => match &path.target {
                Identifiable::Item(_) => {
                    let static_name = format_ident!("{}", path.name);
                    quote!(self.codegen.output, #static_name);
                }
                Identifiable::Summary(_) => {
                    if path.name == "String" || path.name == "PathBuf" {
                        quote!(self.codegen.output, &'static str);
                        return self.trailing_comma();
                    }
                    if path.name == "Vec" {
                        let inner = path.args.as_ref().map(|args| {
                            self.branch(|fv| {
                                fv.trailing_comma = false;
                                erised::visitor::visit_generic_args(fv, args);
                            })
                        });
                        quote!(self.codegen.output, &'static [
                            #inner
                        ]);
                        return self.trailing_comma();
                    }
                    if path.name == "Arc" || path.name == "Weak" || path.name == "Box" {
                        let inner = path.args.as_ref().map(|args| {
                            self.branch(|fv| {
                                fv.trailing_comma = false;
                                erised::visitor::visit_generic_args(fv, args);
                            })
                        });
                        quote!(self.codegen.output, fn() ->
                            #inner
                        );
                        return self.trailing_comma();
                    }

                    let name = format_ident!("{}", path.name);
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
                    quote!(self.codegen.output, #name #args);
                }
            },
            Type::DynTrait(_) => todo!(),
            Type::Generic(_) => todo!(),
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
            Type::Array { type_: _, len: _ } => todo!(),
            Type::ImplTrait(_) => todo!(),
            Type::Infer => todo!(),
            Type::RawPointer {
                mutable: _,
                type_: _,
            } => todo!(),
            Type::BorrowedRef {
                lifetime: _,
                mutable: _,
                type_: _,
            } => todo!(),
            Type::QualifiedPath {
                name: _,
                args: _,
                self_type: _,
                trait_: _,
            } => todo!(),
        }
        self.trailing_comma();
    }
}

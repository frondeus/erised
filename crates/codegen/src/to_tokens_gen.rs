use erised::{
    heap_types::*,
    visitor::{CycleDetector, Visitor},
};
use proc_macro2::{Punct, Spacing, TokenStream};
use quote::format_ident;
use quote::TokenStreamExt;

#[derive(Default)]
pub struct ToTokensGenerator {
    cycles: CycleDetector,
    pub output: TokenStream,
}

macro_rules! quote {
    ($output: expr, $($tokens: tt)*) => {
        $output.append_all(quote::quote!($($tokens)*));
    }
}

impl ToTokensGenerator {
    fn branch(&mut self, f: impl FnOnce(&mut Self)) -> TokenStream {
        let saved = std::mem::take(&mut self.output);
        f(self);
        let inner = std::mem::replace(&mut self.output, saved);
        inner
    }
}

impl Visitor for ToTokensGenerator {
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
        let inner = self.branch(|codegen| {
            for variant in &e.variants {
                let variant_name = format_ident!("{}", &variant.name);
                quote!(codegen.output, Self::#variant_name);

                match &variant.kind {
                    VariantKind::Plain => {
                        quote!(codegen.output, =>
                            quote::quote!(
                                erised::types::#name::#variant_name
                            ),
                        );
                    }
                    VariantKind::Tuple(fields) => {
                        let destruct = codegen.branch(|codegen| {
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
                        let mapped_fields = codegen.branch(|codegen| {
                            let mut fv = Mapper {
                                codegen,
                                counter: 0,
                            };
                            for field in fields {
                                if let Some(field) = field.as_ref() {
                                    fv.visit_struct_field(field);
                                }
                            }
                        });
                        let construct = codegen.branch(|codegen| {
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
                        quote!(codegen.output, ( #destruct ) => {
                                #mapped_fields
                                quote::quote!(
                                    erised::types::#name::#variant_name (
                                        #construct
                                    )
                                )
                            },
                        );
                    }
                    VariantKind::Struct { fields, .. } => {
                        let destruct = codegen.branch(|codegen| {
                            let mut fv = Destructor {
                                codegen,
                                trailing_comma: true,
                                counter: 0,
                            };
                            for field in fields {
                                fv.visit_struct_field(field);
                            }
                        });
                        let mapped_fields = codegen.branch(|codegen| {
                            let mut fv = Mapper {
                                codegen,
                                counter: 0,
                            };
                            for field in fields {
                                fv.visit_struct_field(field);
                            }
                        });
                        let construct = codegen.branch(|codegen| {
                            let mut fv = Constructor {
                                codegen,
                                trailing_comma: true,
                                counter: 0,
                            };
                            for field in fields {
                                fv.visit_struct_field(field);
                            }
                        });
                        quote!(codegen.output, { #destruct } => {
                            #mapped_fields
                            quote::quote!(
                                    erised::types::#name::#variant_name {
                                        #construct
                                    }
                                )
                            },
                        );
                    }
                }
            }
        });

        quote!(self.output,
        impl erised::destruct::ToTokens for #name {
            fn to_tokens(&self, paths: &mut erised::destruct::ItemsPaths) -> proc_macro2::TokenStream {
                match &self {
                    #inner
                }
            }
        }
        impl quote::ToTokens for #name {
            fn to_tokens(&self, tokens: &mut proc_macro2::TokenStream) {
                use quote::TokenStreamExt;
                let mut paths = erised::destruct::ItemsPaths::default();
                tokens.append_all(erised::destruct::ToTokens::to_tokens(self, &mut paths))
            }
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
        let name = format_ident!("{name}");

        let inner = self.branch(|codegen| match &strukt.kind {
            StructKind::Unit => {
                quote!(
                    codegen.output,
                    quote::quote!(
                        erised::types::#name
                    )
                );
            }
            StructKind::Tuple(_) => todo!(),
            StructKind::Plain {
                fields,
                fields_stripped,
            } => {
                let destruct = codegen.branch(|codegen| {
                    Destructor {
                        codegen,
                        trailing_comma: true,
                        counter: 0,
                    }
                    .visit_struct(strukt);
                });
                let mapped_fields = codegen.branch(|codegen| {
                    Mapper {
                        codegen,
                        counter: 0,
                    }
                    .visit_struct(strukt);
                });
                let construct = codegen.branch(|codegen| {
                    Constructor {
                        codegen,
                        trailing_comma: true,
                        counter: 0,
                    }
                    .visit_struct(strukt);
                });

                quote!(codegen.output,
                    let &Self { #destruct .. }  = &self;
                    #mapped_fields
                    quote::quote!(
                        erised::types::#name {
                            #construct
                        }
                    )
                );
            }
        });

        quote!(self.output,
        impl erised::destruct::ToTokens for #name {
            fn to_tokens(&self, paths: &mut erised::destruct::ItemsPaths) -> proc_macro2::TokenStream {
                #inner
            }
        }
        impl quote::ToTokens for #name {
            fn to_tokens(&self, tokens: &mut proc_macro2::TokenStream) {
                use quote::TokenStreamExt;
                let mut paths = erised::destruct::ItemsPaths::default();
                tokens.append_all(erised::destruct::ToTokens::to_tokens(self, &mut paths))
            }
        }
        );
    }
}

struct Destructor<'a> {
    codegen: &'a mut ToTokensGenerator,
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
    codegen: &'a mut ToTokensGenerator,
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

        let hash = Punct::new('#', Spacing::Joint);
        let hashed_name = quote::quote!(#hash #name);

        quote!(self.codegen.output, #name : #hashed_name );

        self.trailing_comma();
    }
    fn visit_item(&mut self, _: &Item) {}
}

struct Mapper<'a> {
    codegen: &'a mut ToTokensGenerator,
    counter: usize,
}

impl<'a> Mapper<'a> {
    fn get_idx(&mut self) -> usize {
        let idx = self.counter;
        self.counter += 1;
        idx
    }
}

impl<'a> Visitor for Mapper<'a> {
    fn visit_struct_field(&mut self, field: &StructField) {
        if field.meta.attrs.contains(&"#[type_info(skip)]".to_owned()) {
            return;
        }

        let name = if !field.is_part_of_tuple {
            format_ident!("{}", field.name)
        } else {
            format_ident!("_{}", self.get_idx())
        };

        quote!(self.codegen.output,
            let #name = erised::destruct::ToTokens::to_tokens(#name, paths);
        );
    }
    fn visit_item(&mut self, _: &Item) {}
}

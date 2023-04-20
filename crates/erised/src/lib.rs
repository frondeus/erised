use std::path::PathBuf;

use anyhow::Context;
use ext::{ItemEnumExt, OptionBoxGenericArgsExt};
use proc_macro2::TokenStream;
use quote::{format_ident, quote};
use rustdoc_types::{
    Crate, Enum, Function, GenericArg, GenericArgs, Id, Item, ItemEnum, Path, Struct, StructKind,
    Type, VariantKind,
};

use crate::ext::{OptionExt, TypeExt};

pub mod build;
mod ext;
mod reflection;

pub use reflection::*;

pub struct Mirror {
    krate: Crate,
    reflect_id: Id,
    root_crate_id: u32,
}

pub struct Reflection {
    path: PathBuf,
    reflection: TokenStream,
}

impl Mirror {
    pub fn build_opts(
        opts: impl Fn(rustdoc_json::Builder) -> rustdoc_json::Builder,
    ) -> anyhow::Result<Option<Self>> {
        let builder = rustdoc_json::Builder::default()
            .toolchain("nightly")
            // .package_target(rustdoc_json::PackageTarget::Example("codegen".to_owned()))
            .document_private_items(true);

        let json_path = opts(builder).build()?;

        let file = std::fs::OpenOptions::new()
            .read(true)
            .open(&json_path)
            .context("Could not open target JSON documentation")?;

        let krate: Crate = serde_json::from_reader(file)?;

        let reflect_id = krate
            .paths
            .iter()
            .find(|(_path, val)| val.path == &["erised", "reflection", "ToReflect"])
            .map(|(p, _)| p)
            .cloned();

        let reflect_id = match reflect_id {
            Some(id) => id,
            None => {
                eprintln!(
                    "warn: Reflection crate did not find a single usage of erised::ToReflect"
                );
                eprintln!("It tried to load {json_path:?}");
                return Ok(None);
            }
        };

        let root_crate = krate.index.get(&krate.root).context("Root crate")?;
        let root_crate_id = root_crate.crate_id;

        Ok(Some(Mirror {
            krate,
            reflect_id,
            root_crate_id,
        }))
    }

    pub fn build() -> anyhow::Result<Option<Self>> {
        Self::build_opts(|o| o)
    }

    fn items_to_reflect(krate: &Crate, reflect_id: &Id) -> Vec<(PathBuf, Vec<Item>)> {
        krate
            .index
            .iter()
            .filter_map(|(_, item)| {
                let fn_ = item.inner.as_fn()?;
                if let Some(out) = fn_.decl.output.as_ref() {
                    let out = out.as_resolved_path()?;
                    if &out.id == reflect_id && item.span.is_some() {
                        return Some((fn_, item.span.clone().unwrap()));
                    }
                }
                None
            })
            .map(|(fn_, span)| {
                let items = fn_
                    .decl
                    .inputs
                    .iter()
                    .flat_map(|(_, input)| match input {
                        Type::ImplTrait(bounds) => bounds
                            .iter()
                            .filter_map(|bound| match bound {
                                rustdoc_types::GenericBound::TraitBound {
                                    trait_,
                                    generic_params: _,
                                    modifier: _,
                                } => Some(trait_.id.clone()),
                                _ => None,
                            })
                            .collect(),
                        Type::ResolvedPath(path) => vec![path.id.clone()],
                        _ => vec![],
                    })
                    .filter_map(|id| krate.index.get(&id))
                    .cloned()
                    .collect();

                (span.filename, items)
            })
            .collect()
    }

    pub fn prelude(&self, items: &[Item]) -> anyhow::Result<TokenStream> {
        let mut reflection_input: Vec<TokenStream> = vec![];
        for item in items {
            let is_trait = match item.inner {
                rustdoc_types::ItemEnum::Trait(_) => true,
                _ => false,
            };
            let name = self.get_path(&item.id).context("Item path")?;
            if is_trait {
                reflection_input.push(quote!(_: impl #name));
            } else {
                reflection_input.push(quote!(_: #name));
            }
        }
        Ok(quote!(
            #[allow(dead_code)]
            pub fn to_reflect(#(#reflection_input),*) -> erised::ToReflect {
                erised::ToReflect
            }

            pub trait Reflect {
                const TYPE_INFO: erised::TypeInfo;
            }
        ))
    }

    pub fn get_path(&self, id: &Id) -> anyhow::Result<TokenStream> {
        let item_path = self.krate.paths.get(id).context("Item path")?;
        let item_crate = &item_path.crate_id;
        let mut item_path: Vec<_> = item_path
            .path
            .iter()
            .map(|t| format_ident!("{t}"))
            .collect();
        if item_crate == &self.root_crate_id {
            if let Some(krate) = item_path.first_mut() {
                *krate = format_ident!("crate");
            }
        }
        let item_path = quote!(#(#item_path)::*);
        Ok(item_path)
    }

    pub fn get_local_path(&self, id: &Id) -> anyhow::Result<TokenStream> {
        let item_path = self.krate.paths.get(id).context("Item path")?;
        let item_crate = &item_path.crate_id;
        let mut item_path: Vec<_> = item_path
            .path
            .iter()
            .map(|t| format_ident!("{t}"))
            .collect();
        if item_crate == &self.root_crate_id {
            if let Some(_krate) = item_path.first_mut() {
                // *krate = format_ident!("crate");
                item_path.remove(0);
            }
        }
        let item_path = quote!(#(#item_path)::*);
        Ok(item_path)
    }

    pub fn get_full_name(&self, id: &Id, name: &Option<String>) -> anyhow::Result<String> {
        let item_path = match self.krate.paths.get(id) {
            None => return Ok(name.as_deref().unwrap_or_default().to_string()),
            Some(p) => p,
        };

        let item_crate = &item_path.crate_id;
        let mut item_path: Vec<_> = item_path.path.iter().cloned().collect();
        if item_crate == &self.root_crate_id {
            if let Some(krate) = item_path.first_mut() {
                *krate = "crate".to_string();
            }
        }
        Ok(item_path.join("::"))
    }

    fn reflect_path(&self, queue: &mut Vec<Item>, path: &Path) -> anyhow::Result<TokenStream> {
        // let field_ty = &path.name;
        let field_ty = self
            .get_full_name(&path.id, &Some(path.name.clone()))
            .context("Item name")?;

        if let Some(item) = self.krate.index.get(&path.id) {
            let is_trait = match item.inner {
                ItemEnum::Trait(_) => true,
                _ => false,
            };
            let item_path = self.get_path(&path.id).context("Item path")?;
            queue.push(item.clone());
            if is_trait {
                let local_path = self.get_local_path(&path.id).context("Item local path")?;
                Ok(quote!(
                    <#local_path as Reflect>::TYPE_INFO
                ))
            } else {
                Ok(quote!(
                    <#item_path as Reflect>::TYPE_INFO
                ))
            }
        } else if let Some(args) = path.args.as_non_empty() {
            let (args, bindings) = match &**args {
                GenericArgs::AngleBracketed { args, bindings } => {
                    let a = args
                        .iter()
                        .map(|arg| match arg {
                            GenericArg::Lifetime(_) => todo!(),
                            GenericArg::Type(ty) => self.reflect_type(queue, ty),
                            GenericArg::Const(_) => todo!(),
                            GenericArg::Infer => todo!(),
                        })
                        .collect::<Result<Vec<_>, _>>()?;

                    let b: Vec<_> = bindings
                        .iter()
                        .map(|binding| {
                            let name = &binding.name;
                            let binding = match &binding.binding {
                                rustdoc_types::TypeBindingKind::Equality(
                                    rustdoc_types::Term::Type(_type),
                                ) => {
                                    let ty = self.reflect_type(queue, _type)?;
                                    quote!(
                                        erised::TypeBindingKind::Equality {
                                            term: erised::Term::Type(#ty)
                                        }
                                    )
                                }
                                rustdoc_types::TypeBindingKind::Equality(
                                    rustdoc_types::Term::Constant(_),
                                ) => {
                                    todo!()
                                }
                                rustdoc_types::TypeBindingKind::Constraint(bounds) => {
                                    let bounds = self.reflect_generic_bounds(queue, bounds)?;
                                    quote!(
                                        erised::TypeBindingKind::Constraint {
                                            bounds: &[#(#bounds),*]
                                        }
                                    )
                                }
                            };
                            Ok(quote!(
                                erised::WithGenericBindingInfo {
                                    name: #name,
                                    binding: #binding
                                }
                            ))
                        })
                        .collect::<anyhow::Result<_>>()?;

                    (a, b)
                }
                GenericArgs::Parenthesized { .. } => todo!(),
            };
            Ok(quote!(
                erised::TypeInfo::WithGeneric(
                    erised::WithGenericInfo {
                        name: #field_ty,
                        args: || &[#(#args),*],
                        bindings: &[#(#bindings),*]
                    }
                )
            ))
        } else {
            Ok(quote!(
                erised::TypeInfo::Primitive(
                    erised::Primitive {
                        name: #field_ty,
                        docs: None
                    }
                )
            ))
        }
    }

    pub fn reflect_type(&self, queue: &mut Vec<Item>, ty: &Type) -> anyhow::Result<TokenStream> {
        match ty {
            Type::ResolvedPath(path) => self.reflect_path(queue, path),
            Type::DynTrait(d) => {
                let mut traits: Vec<TokenStream> = vec![];
                let lifetime = d.lifetime.quote();
                for trait_ in &d.traits {
                    let it = self.reflect_path(queue, &trait_.trait_)?;
                    traits.push(it);
                }

                Ok(quote!(erised::TypeInfo::DynTrait(
                  erised::DynTraitInfo {
                      traits: &[#(#traits),*],
                      lifetime: #lifetime
                  }
                )))
            }
            Type::Generic(gen) if gen == "Self" => Ok(quote!(erised::TypeInfo::Receiver)),
            Type::Generic(gen) => Ok(quote!(
                erised::TypeInfo::Generic(erised::GenericInfo { name: #gen })
            )),
            Type::Primitive(field_ty) => Ok(quote!(
                    erised::TypeInfo::Primitive(
                        erised::Primitive {
                            name: #field_ty,
                            docs: None
                        }
                    )
            )),
            Type::FunctionPointer(_) => todo!("Function pointer"),
            Type::Tuple(fields) => {
                let fields: Result<Vec<_>, _> = fields
                    .iter()
                    .map(|field| self.reflect_type(queue, field))
                    .collect();
                let fields = fields?;
                Ok(quote!(
                    erised::TypeInfo::Tuple(
                        erised::TupleInfo {
                            fields: &[#(#fields),*]
                        }
                    )
                ))
            }
            Type::Slice(_) => todo!("Slice"),
            Type::Array { type_, len } => {
                let ty = self.reflect_type(queue, type_)?;
                let len: usize = len.parse().context("Could not parse array length")?;
                Ok(quote!(
                    erised::TypeInfo::Array(
                        erised::ArrayInfo {
                            len: #len,
                            ty: || #ty
                        }
                    )
                ))
            }
            Type::ImplTrait(_) => todo!("Impl trait"),
            Type::Infer => todo!("Infer"),
            Type::RawPointer { .. } => todo!("Raw pointer"),
            Type::BorrowedRef {
                lifetime,
                mutable,
                type_,
            } => {
                let lifetime = lifetime.as_ref().map(|lif| lif.trim_matches('\'')).quote();
                let ty = self.reflect_type(queue, type_)?;
                Ok(quote!(
                    erised::TypeInfo::Borrow(
                        erised::BorrowInfo {
                            lifetime: #lifetime,
                            mutable: #mutable,
                            ty: || #ty
                        }
                    )
                ))
            }
            Type::QualifiedPath { .. } => todo!("Qualified Path"),
        }
    }

    fn reflect_struct(
        &self,
        queue: &mut Vec<Item>,
        strukt: &Struct,
        name: &String,
        docs: TokenStream,
    ) -> anyhow::Result<TokenStream> {
        match &strukt.kind {
            StructKind::Unit => Ok(quote!(
                erised::TypeInfo::Primitive(
                    erised::Primitive {
                        name: #name,
                        docs: #docs
                    }
                )
            )),
            StructKind::Tuple(fields) => {
                let mut field_info: Vec<TokenStream> = vec![];
                for field in fields.iter().filter_map(|f| f.as_ref()) {
                    let item = self.krate.index.get(&field).context("struct field")?;
                    let field_inner = item.inner.as_struct_field().context("Struct field")?;

                    field_info.push(self.reflect_type(queue, field_inner)?);
                }
                Ok(quote!(
                    erised::TypeInfo::TupleStruct(erised::TupleStructInfo {
                        name: #name,
                        docs: #docs,
                        fields: &[
                            #(#field_info),*
                        ]
                    })
                ))
            }
            StructKind::Plain { fields, .. } => {
                let mut field_info = vec![];

                for field in fields {
                    if let Some(field) = self.krate.index.get(&field) {
                        let field_name = self
                            .get_full_name(&field.id, &field.name)
                            .context("Field name")?;
                        let field_inner = field.inner.as_struct_field().context("Struct field")?;

                        let docs = field.docs.quote();
                        let ty = self.reflect_type(queue, field_inner)?;

                        field_info.push(quote!(
                           erised::StructFieldInfo {
                                name: #field_name,
                                docs: #docs,
                                ty: #ty
                            }
                        ));
                    }
                }

                Ok(quote!(
                    erised::TypeInfo::Struct(erised::StructInfo {
                        name: #name,
                        docs: #docs,
                        fields: &[
                            #(#field_info),*
                        ]
                    })
                ))
            }
        }
    }

    fn reflect_enum(
        &self,
        queue: &mut Vec<Item>,
        enumerated: &Enum,
        name: &String,
        docs: TokenStream,
    ) -> anyhow::Result<TokenStream> {
        let mut variant_info = vec![];
        for variant in &enumerated.variants {
            if let Some(variant) = self.krate.index.get(&variant) {
                let variant_name = variant.name.as_deref().unwrap_or_default();
                let variant_inner = variant.inner.as_variant().context("Variant")?;
                let discr = variant_inner.discriminant.as_ref().map(|d| &d.expr).quote();
                let docs = variant.docs.quote();

                match &variant_inner.kind {
                    VariantKind::Plain => {
                        variant_info.push(quote!(
                            erised::VariantInfo::Unit {
                                name: #variant_name,
                                docs: #docs,
                                discr: #discr
                            }
                        ));
                    }
                    VariantKind::Tuple(fields) => {
                        let mut field_info: Vec<TokenStream> = vec![];
                        for field in fields.iter().filter_map(|f| f.as_ref()) {
                            let item = self.krate.index.get(&field).context("struct field")?;
                            let field_inner =
                                item.inner.as_struct_field().context("Struct field")?;

                            field_info.push(self.reflect_type(queue, field_inner)?);
                        }
                        variant_info.push(quote! {
                            erised::VariantInfo::Tuple {
                                name: #variant_name,
                                docs: #docs,
                                fields: &[
                                    #(#field_info),*
                                ]
                            }
                        })
                    }
                    VariantKind::Struct {
                        fields,
                        fields_stripped: _,
                    } => {
                        let mut field_info: Vec<TokenStream> = vec![];
                        for field in fields {
                            if let Some(field) = self.krate.index.get(&field) {
                                let field_name = self
                                    .get_full_name(&field.id, &field.name)
                                    .context("Field name")?;
                                let field_inner =
                                    field.inner.as_struct_field().context("Struct field")?;

                                let ty = self.reflect_type(queue, field_inner)?;

                                let docs = field.docs.quote();

                                field_info.push(quote!(
                                   erised::StructFieldInfo {
                                        name: #field_name,
                                        docs: #docs,
                                        ty: #ty
                                    }
                                ));
                            }
                        }
                        variant_info.push(quote!(
                           erised::VariantInfo::Struct {
                                name: #variant_name,
                                docs: #docs,
                                fields: &[
                                    #(#field_info),*
                                ]
                            }
                        ));
                    }
                }
            }
        }
        Ok(quote!(
            erised::TypeInfo::Enum(erised::EnumInfo {
                name: #name,
                docs: #docs,
                variants: &[
                    #(#variant_info),*
                ]
            })
        ))
    }

    fn reflect_generic_bounds(
        &self,
        queue: &mut Vec<Item>,
        param_bounds: &Vec<rustdoc_types::GenericBound>,
    ) -> anyhow::Result<Vec<TokenStream>> {
        let mut bounds: Vec<TokenStream> = vec![];
        for bound in param_bounds.iter() {
            match bound {
                rustdoc_types::GenericBound::TraitBound {
                    trait_,
                    generic_params: _,
                    modifier: _,
                } => {
                    let trait_info = self.reflect_path(queue, &trait_)?;
                    bounds.push(quote!(
                        erised::GenericBound {
                            trait_: || #trait_info.as_trait().unwrap()
                        }
                    ));
                }
                rustdoc_types::GenericBound::Outlives(_) => todo!(),
            }
        }
        Ok(bounds)
    }

    fn reflect_function(
        &self,
        queue: &mut Vec<Item>,
        func: &Function,
        name: &String,
        docs: TokenStream,
    ) -> anyhow::Result<TokenStream> {
        let mut inputs: Vec<TokenStream> = vec![];
        let mut generics: Vec<TokenStream> = vec![];
        for (input_name, input) in &func.decl.inputs {
            let ty = self.reflect_type(queue, input)?;
            inputs.push(quote!(
               erised::FunctionArg {
                   name: #input_name,
                   ty: #ty
               }
            ));
        }
        let output = func
            .decl
            .output
            .as_ref()
            .map(|out| self.reflect_type(queue, &out))
            .transpose()?
            .quote();

        for param in &func.generics.params {
            let generic_name = &param.name;
            let kind: TokenStream;

            match &param.kind {
                rustdoc_types::GenericParamDefKind::Lifetime { outlives } => {
                    // todo!("Generic Param Def Kind::Lifetime: {param:#?}")
                    kind = quote!(
                        erised::GenericParamKind::Lifetime(
                            erised::LifetimeParamType {
                                outlives: &[ #(#outlives),* ]
                            }
                        )
                    );
                }
                rustdoc_types::GenericParamDefKind::Type {
                    bounds: param_bounds,
                    ..
                } => {
                    let bounds = self.reflect_generic_bounds(queue, param_bounds)?;
                    kind = quote!(
                        erised::GenericParamKind::Type(
                            erised::GenericParamType {
                                bounds: &[#(#bounds),*]
                            }
                        )
                    );
                }
                rustdoc_types::GenericParamDefKind::Const { .. } => todo!(),
            }

            generics.push(quote!(
               erised::FunctionGeneric {
                    name: #generic_name,
                    kind: #kind
                }
            ));
        }

        Ok(quote!(
            erised::FunctionInfo {
                name: #name,
                docs: #docs,
                inputs: &[#(#inputs),*],
                output: #output,
                generics: &[#(#generics),*]
            }
        ))
    }

    fn reflect_trait(
        &self,
        queue: &mut Vec<Item>,
        trait_: &rustdoc_types::Trait,
        name: &String,
        docs: TokenStream,
    ) -> anyhow::Result<TokenStream> {
        let mut methods = vec![];
        let mut consts: Vec<TokenStream> = vec![];
        let mut assoc_types: Vec<TokenStream> = vec![];
        for item in &trait_.items {
            if let Some(item) = self.krate.index.get(item) {
                let name = item.name.as_deref().unwrap_or_default();
                let docs = item.docs.quote();

                match &item.inner {
                    ItemEnum::Function(func) => {
                        methods.push(self.reflect_function(
                            queue,
                            func,
                            &name.to_string(),
                            docs,
                        )?);
                    }
                    ItemEnum::AssocType {
                        generics: _,
                        bounds: _,
                        default: _,
                    } => {
                        assoc_types.push(quote!(erised::AssocTypeInfo {
                            name: #name,
                            docs: #docs,
                        }));
                    }
                    ItemEnum::AssocConst { type_, default: _ } => {
                        let ty = self.reflect_type(queue, &type_)?;
                        consts.push(quote!(erised::ConstInfo {
                            name: #name,
                            docs: #docs,
                            ty: #ty
                        }));
                    }
                    _ => todo!(),
                }
            }
        }
        Ok(quote!(erised::TypeInfo::Trait(erised::TraitInfo {
            name: #name,
            docs: #docs,
            methods: &[#(#methods),*],
            consts: &[#(#consts),*],
            assoc_types: &[#(#assoc_types),*],
        })))
    }

    fn reflect_item(&self, queue: &mut Vec<Item>, item: &Item) -> anyhow::Result<TokenStream> {
        let name = self
            .get_full_name(&item.id, &item.name)
            .context("Item name")?;

        let item_path = self.get_path(&item.id).context("Item path")?;

        let docs = item.docs.quote();

        match &item.inner {
            ItemEnum::Struct(strukt) => {
                let type_info = self.reflect_struct(queue, strukt, &name, docs)?;
                Ok(quote!(
                    impl Reflect for #item_path {
                        const TYPE_INFO: erised::TypeInfo = #type_info;
                    }
                ))
            }
            ItemEnum::Enum(enumerated) => {
                let type_info = self.reflect_enum(queue, enumerated, &name, docs)?;
                Ok(quote!(
                    impl Reflect for #item_path {
                        const TYPE_INFO: erised::TypeInfo = #type_info;
                    }
                ))
            }
            ItemEnum::Trait(trait_) => {
                let type_info = self.reflect_trait(queue, trait_, &name, docs)?;
                let struct_name =
                    format_ident!("{}", name.clone().replace("::", "_").replace("crate_", ""));
                Ok(quote!(
                    pub enum #struct_name {}
                   impl Reflect for #struct_name {
                        const TYPE_INFO: erised::TypeInfo = #type_info;
                    }
                ))
            }
            ItemEnum::StructField(_) => todo!(),
            ItemEnum::Variant(_) => todo!(),
            ItemEnum::Function(_) => todo!(),
            ItemEnum::TraitAlias(_) => todo!(),
            ItemEnum::Impl(_) => todo!(),
            ItemEnum::Typedef(_) => todo!(),
            ItemEnum::OpaqueTy(_) => todo!(),
            ItemEnum::Constant(_) => todo!(),
            ItemEnum::Static(_) => todo!(),
            ItemEnum::ForeignType => todo!(),
            ItemEnum::Macro(_) => todo!(),
            ItemEnum::ProcMacro(_) => todo!(),
            ItemEnum::Primitive(_) => todo!(),
            ItemEnum::AssocConst { .. } => todo!(),
            ItemEnum::AssocType { .. } => todo!(),
            ItemEnum::Module(_) => todo!(),
            ItemEnum::ExternCrate { .. } => todo!(),
            ItemEnum::Import(_) => todo!(),
            ItemEnum::Union(_) => todo!(),
        }
    }

    pub fn gen(&mut self) -> anyhow::Result<Vec<Reflection>> {
        let items_to_reflect = Self::items_to_reflect(&self.krate, &self.reflect_id);
        let mut reflections = vec![];

        for (reflection_path, mut reflection_items) in items_to_reflect {
            let mut items = vec![];
            let mut processed = vec![];
            let prelude = self.prelude(&reflection_items)?;

            while let Some(item) = reflection_items.pop() {
                if processed.contains(&item.id) {
                    continue;
                }
                processed.push(item.id.clone());

                items.push(self.reflect_item(&mut reflection_items, &item)?);
            }

            reflections.push(Reflection {
                path: reflection_path,
                reflection: quote! {
                    #prelude
                    #(#items)*
                },
            });
        }

        Ok(reflections)
    }
}

// pub fn run() -> anyhow::Result<Mirror> {
//     let mirror = Mirror::build()?;

//     let stream = mirror.gen()?;
//     println!("{}", stream);

//     Ok(())
// }

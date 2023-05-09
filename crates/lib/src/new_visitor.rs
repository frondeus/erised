use crate::heap_types::*;
pub use crate::utils::CycleDetector;
pub fn visit_crate(vis: &mut (impl Visitor + ?Sized), krate: &Crate) {
    vis.visit_module(&krate.root);
    for all_items in &krate.all_items {
        vis.visit_item(all_items);
    }
}
pub fn visit_identifiable(vis: &mut (impl Visitor + ?Sized), identifiable: &Identifiable) {
    match &identifiable {
        Identifiable::Item(item) => {
            if let Some(item) = item.upgrade().as_ref() {
                vis.visit_item(item);
            }
        }
        Identifiable::Summary(item_summary) => {
            vis.visit_item_summary(item_summary);
        }
        _ => (),
    }
}
pub fn visit_item_summary(vis: &mut (impl Visitor + ?Sized), item_summary: &ItemSummary) {
    vis.visit_external_crate(&item_summary.krate);
    vis.visit_item_kind(&item_summary.kind);
}
pub fn visit_item_meta(vis: &mut (impl Visitor + ?Sized), item_meta: &ItemMeta) {
    vis.visit_external_crate(&item_meta.krate);
    if let Some(summary) = &item_meta.summary.as_ref() {
        vis.visit_item_summary(summary);
    }
    if let Some(span) = &item_meta.span.as_ref() {
        vis.visit_span(span);
    }
    vis.visit_visibility(&item_meta.visibility);
    if let Some(deprecation) = &item_meta.deprecation.as_ref() {
        vis.visit_deprecation(deprecation);
    }
}
pub fn visit_visibility(vis: &mut (impl Visitor + ?Sized), visibility: &Visibility) {
    match &visibility {
        Visibility::Restricted { parent, .. } => {
            vis.visit_identifiable(parent);
        }
        _ => (),
    }
}
pub fn visit_dyn_trait(vis: &mut (impl Visitor + ?Sized), dyn_trait: &DynTrait) {
    for traits in &dyn_trait.traits {
        vis.visit_poly_trait(traits);
    }
}
pub fn visit_poly_trait(vis: &mut (impl Visitor + ?Sized), poly_trait: &PolyTrait) {
    vis.visit_path(&poly_trait.trait_);
    for generic_params in &poly_trait.generic_params {
        vis.visit_generic_param_def(generic_params);
    }
}
pub fn visit_generic_args(vis: &mut (impl Visitor + ?Sized), generic_args: &GenericArgs) {
    match &generic_args {
        GenericArgs::AngleBracketed { args, bindings, .. } => {
            for args in args {
                vis.visit_generic_arg(args);
            }
            for bindings in bindings {
                vis.visit_type_binding(bindings);
            }
        }
        GenericArgs::Parenthesized { inputs, output, .. } => {
            for inputs in inputs {
                vis.visit_type(inputs);
            }
            if let Some(output) = output.as_ref() {
                vis.visit_type(output);
            }
        }
        _ => (),
    }
}
pub fn visit_generic_arg(vis: &mut (impl Visitor + ?Sized), generic_arg: &GenericArg) {
    match &generic_arg {
        GenericArg::Type(ty) => {
            vis.visit_type(ty);
        }
        GenericArg::Const(constant) => {
            vis.visit_constant(constant);
        }
        _ => (),
    }
}
pub fn visit_constant_item(vis: &mut (impl Visitor + ?Sized), constant_item: &ConstantItem) {
    vis.visit_item_meta(&constant_item.meta);
    vis.visit_constant(&constant_item.constant);
}
pub fn visit_constant(vis: &mut (impl Visitor + ?Sized), constant: &Constant) {
    vis.visit_type(&constant.type_);
}
pub fn visit_type_binding(vis: &mut (impl Visitor + ?Sized), type_binding: &TypeBinding) {
    vis.visit_generic_args(&type_binding.args);
    vis.visit_type_binding_kind(&type_binding.binding);
}
pub fn visit_type_binding_kind(
    vis: &mut (impl Visitor + ?Sized),
    type_binding_kind: &TypeBindingKind,
) {
    match &type_binding_kind {
        TypeBindingKind::Equality(term) => {
            vis.visit_term(term);
        }
        TypeBindingKind::Constraint(generic_bound) => {
            for generic_bound in generic_bound {
                vis.visit_generic_bound(generic_bound);
            }
        }
        _ => (),
    }
}
pub fn visit_item(vis: &mut (impl Visitor + ?Sized), item: &Item) {
    match &item {
        Item::Module(module) => {
            vis.visit_module(module);
        }
        Item::ExternCrate { meta, .. } => {
            vis.visit_item_meta(meta);
        }
        Item::Import(import) => {
            vis.visit_import(import);
        }
        Item::Union(union) => {
            vis.visit_union(union);
        }
        Item::Struct(struct_) => {
            vis.visit_struct(struct_);
        }
        Item::Enum(enum_) => {
            vis.visit_enum(enum_);
        }
        Item::Function(function) => {
            vis.visit_function(function);
        }
        Item::Trait(trait_) => {
            vis.visit_trait(trait_);
        }
        Item::TraitAlias(trait_alias) => {
            vis.visit_trait_alias(trait_alias);
        }
        Item::Impl(imp) => {
            vis.visit_impl(imp);
        }
        Item::Typedef(typedef) => {
            vis.visit_typedef(typedef);
        }
        Item::OpaqueTy(opaque_ty) => {
            vis.visit_opaque_ty(opaque_ty);
        }
        Item::Constant(constant_item) => {
            vis.visit_constant_item(constant_item);
        }
        Item::Static(statik) => {
            vis.visit_static(statik);
        }
        Item::Macro { meta, .. } => {
            vis.visit_item_meta(meta);
        }
        Item::ProcMacro(proc_macro) => {
            vis.visit_proc_macro(proc_macro);
        }
        Item::Primitive(primitive) => {
            vis.visit_primitive(primitive);
        }
        Item::AssocConst { meta, type_, .. } => {
            vis.visit_item_meta(meta);
            vis.visit_type(type_);
        }
        Item::AssocType {
            meta,
            generics,
            bounds,
            default,
            ..
        } => {
            vis.visit_item_meta(meta);
            vis.visit_generics(generics);
            for bounds in bounds {
                vis.visit_generic_bound(bounds);
            }
            if let Some(default) = default.as_ref() {
                vis.visit_type(default);
            }
        }
        _ => (),
    }
}
pub fn visit_module(vis: &mut (impl Visitor + ?Sized), module: &Module) {
    vis.visit_item_meta(&module.meta);
    for items in &module.items {
        vis.visit_identifiable(items);
    }
}
pub fn visit_union(vis: &mut (impl Visitor + ?Sized), union: &Union) {
    vis.visit_generics(&union.generics);
    for fields in &union.fields {
        vis.visit_identifiable(fields);
    }
    for impls in &union.impls {
        vis.visit_identifiable(impls);
    }
}
pub fn visit_struct(vis: &mut (impl Visitor + ?Sized), struct_: &Struct) {
    vis.visit_item_meta(&struct_.meta);
    vis.visit_struct_kind(&struct_.kind);
    vis.visit_generics(&struct_.generics);
    for impls in &struct_.impls {
        vis.visit_identifiable(impls);
    }
}
pub fn visit_struct_kind(vis: &mut (impl Visitor + ?Sized), struct_kind: &StructKind) {
    match &struct_kind {
        StructKind::Tuple(struct_field) => {
            for struct_field in struct_field {
                if let Some(struct_field) = struct_field.as_ref() {
                    vis.visit_struct_field(struct_field);
                }
            }
        }
        StructKind::Plain { fields, .. } => {
            for fields in fields {
                vis.visit_struct_field(fields);
            }
        }
        _ => (),
    }
}
pub fn visit_struct_field(vis: &mut (impl Visitor + ?Sized), struct_field: &StructField) {
    vis.visit_item_meta(&struct_field.meta);
    vis.visit_type(&struct_field.ty);
}
pub fn visit_enum(vis: &mut (impl Visitor + ?Sized), enum_: &Enum) {
    vis.visit_item_meta(&enum_.meta);
    vis.visit_generics(&enum_.generics);
    for variants in &enum_.variants {
        vis.visit_variant(variants);
    }
    for impls in &enum_.impls {
        vis.visit_identifiable(impls);
    }
}
pub fn visit_variant(vis: &mut (impl Visitor + ?Sized), variant: &Variant) {
    vis.visit_item_meta(&variant.meta);
    vis.visit_variant_kind(&variant.kind);
    if let Some(discriminant) = &variant.discriminant.as_ref() {
        vis.visit_discriminant(discriminant);
    }
}
pub fn visit_variant_kind(vis: &mut (impl Visitor + ?Sized), variant_kind: &VariantKind) {
    match &variant_kind {
        VariantKind::Tuple(struct_field) => {
            for struct_field in struct_field {
                if let Some(struct_field) = struct_field.as_ref() {
                    vis.visit_struct_field(struct_field);
                }
            }
        }
        VariantKind::Struct { fields, .. } => {
            for fields in fields {
                vis.visit_struct_field(fields);
            }
        }
        _ => (),
    }
}
pub fn visit_header(vis: &mut (impl Visitor + ?Sized), header: &Header) {
    vis.visit_abi(&header.abi);
}
pub fn visit_function(vis: &mut (impl Visitor + ?Sized), function: &Function) {
    vis.visit_item_meta(&function.meta);
    vis.visit_fn_decl(&function.decl);
    vis.visit_generics(&function.generics);
    vis.visit_header(&function.header);
}
pub fn visit_generics(vis: &mut (impl Visitor + ?Sized), generics: &Generics) {
    for params in &generics.params {
        vis.visit_generic_param_def(params);
    }
    for where_predicates in &generics.where_predicates {
        vis.visit_where_predicate(where_predicates);
    }
}
pub fn visit_generic_param_def(
    vis: &mut (impl Visitor + ?Sized),
    generic_param_def: &GenericParamDef,
) {
    vis.visit_generic_param_def_kind(&generic_param_def.kind);
}
pub fn visit_generic_param_def_kind(
    vis: &mut (impl Visitor + ?Sized),
    generic_param_def_kind: &GenericParamDefKind,
) {
    match &generic_param_def_kind {
        GenericParamDefKind::Type {
            bounds, default, ..
        } => {
            for bounds in bounds {
                vis.visit_generic_bound(bounds);
            }
            if let Some(default) = default.as_ref() {
                vis.visit_type(default);
            }
        }
        GenericParamDefKind::Const { type_, .. } => {
            vis.visit_type(type_);
        }
        _ => (),
    }
}
pub fn visit_where_predicate(vis: &mut (impl Visitor + ?Sized), where_predicate: &WherePredicate) {
    match &where_predicate {
        WherePredicate::BoundPredicate {
            type_,
            bounds,
            generic_params,
            ..
        } => {
            vis.visit_type(type_);
            for bounds in bounds {
                vis.visit_generic_bound(bounds);
            }
            for generic_params in generic_params {
                vis.visit_generic_param_def(generic_params);
            }
        }
        WherePredicate::RegionPredicate { bounds, .. } => {
            for bounds in bounds {
                vis.visit_generic_bound(bounds);
            }
        }
        WherePredicate::EqPredicate { lhs, rhs, .. } => {
            vis.visit_type(lhs);
            vis.visit_term(rhs);
        }
        _ => (),
    }
}
pub fn visit_generic_bound(vis: &mut (impl Visitor + ?Sized), generic_bound: &GenericBound) {
    match &generic_bound {
        GenericBound::TraitBound {
            trait_,
            generic_params,
            modifier,
            ..
        } => {
            vis.visit_path(trait_);
            for generic_params in generic_params {
                vis.visit_generic_param_def(generic_params);
            }
            vis.visit_trait_bound_modifier(modifier);
        }
        _ => (),
    }
}
pub fn visit_term(vis: &mut (impl Visitor + ?Sized), term: &Term) {
    match &term {
        Term::Type(ty) => {
            vis.visit_type(ty);
        }
        Term::Constant(constant) => {
            vis.visit_constant(constant);
        }
        _ => (),
    }
}
pub fn visit_type(vis: &mut (impl Visitor + ?Sized), ty: &Type) {
    match &ty {
        Type::ResolvedPath(path) => {
            vis.visit_path(path);
        }
        Type::DynTrait(dyn_trait) => {
            vis.visit_dyn_trait(dyn_trait);
        }
        Type::FunctionPointer(function_pointer) => {
            vis.visit_function_pointer(function_pointer);
        }
        Type::Tuple(ty) => {
            for ty in ty {
                vis.visit_type(ty);
            }
        }
        Type::Slice(ty) => {
            vis.visit_type(ty);
        }
        Type::Array { type_, .. } => {
            vis.visit_type(type_);
        }
        Type::ImplTrait(generic_bound) => {
            for generic_bound in generic_bound {
                vis.visit_generic_bound(generic_bound);
            }
        }
        Type::RawPointer { type_, .. } => {
            vis.visit_type(type_);
        }
        Type::BorrowedRef { type_, .. } => {
            vis.visit_type(type_);
        }
        Type::QualifiedPath {
            args,
            self_type,
            trait_,
            ..
        } => {
            vis.visit_generic_args(args);
            vis.visit_type(self_type);
            vis.visit_path(trait_);
        }
        _ => (),
    }
}
pub fn visit_path(vis: &mut (impl Visitor + ?Sized), path: &Path) {
    vis.visit_identifiable(&path.target);
    if let Some(args) = &path.args.as_ref() {
        vis.visit_generic_args(args);
    }
}
pub fn visit_function_pointer(
    vis: &mut (impl Visitor + ?Sized),
    function_pointer: &FunctionPointer,
) {
    vis.visit_fn_decl(&function_pointer.decl);
    for generic_params in &function_pointer.generic_params {
        vis.visit_generic_param_def(generic_params);
    }
    vis.visit_header(&function_pointer.header);
}
pub fn visit_fn_decl(vis: &mut (impl Visitor + ?Sized), fn_decl: &FnDecl) {
    for inputs in &fn_decl.inputs {
        vis.visit_fn_input(inputs);
    }
    if let Some(output) = &fn_decl.output.as_ref() {
        vis.visit_type(output);
    }
}
pub fn visit_fn_input(vis: &mut (impl Visitor + ?Sized), fn_input: &FnInput) {
    vis.visit_type(&fn_input.ty);
}
pub fn visit_trait(vis: &mut (impl Visitor + ?Sized), trait_: &Trait) {
    vis.visit_item_meta(&trait_.meta);
    for items in &trait_.items {
        vis.visit_identifiable(items);
    }
    vis.visit_generics(&trait_.generics);
    for bounds in &trait_.bounds {
        vis.visit_generic_bound(bounds);
    }
    for implementations in &trait_.implementations {
        vis.visit_identifiable(implementations);
    }
}
pub fn visit_trait_alias(vis: &mut (impl Visitor + ?Sized), trait_alias: &TraitAlias) {
    vis.visit_generics(&trait_alias.generics);
    for params in &trait_alias.params {
        vis.visit_generic_bound(params);
    }
}
pub fn visit_impl(vis: &mut (impl Visitor + ?Sized), imp: &Impl) {
    vis.visit_item_meta(&imp.meta);
    vis.visit_generics(&imp.generics);
    if let Some(trait_) = &imp.trait_.as_ref() {
        vis.visit_path(trait_);
    }
    vis.visit_type(&imp.for_);
    for items in &imp.items {
        if let Some(items) = items.upgrade().as_ref() {
            vis.visit_item(items);
        }
    }
    if let Some(blanket_impl) = &imp.blanket_impl.as_ref() {
        vis.visit_type(blanket_impl);
    }
}
pub fn visit_import(vis: &mut (impl Visitor + ?Sized), import: &Import) {
    vis.visit_item_meta(&import.meta);
    if let Some(target) = &import.target.as_ref() {
        vis.visit_identifiable(target);
    }
}
pub fn visit_proc_macro(vis: &mut (impl Visitor + ?Sized), proc_macro: &ProcMacro) {
    vis.visit_macro_kind(&proc_macro.kind);
}
pub fn visit_typedef(vis: &mut (impl Visitor + ?Sized), typedef: &Typedef) {
    vis.visit_item_meta(&typedef.meta);
    vis.visit_type(&typedef.type_);
    vis.visit_generics(&typedef.generics);
}
pub fn visit_opaque_ty(vis: &mut (impl Visitor + ?Sized), opaque_ty: &OpaqueTy) {
    vis.visit_item_meta(&opaque_ty.meta);
    for bounds in &opaque_ty.bounds {
        vis.visit_generic_bound(bounds);
    }
    vis.visit_generics(&opaque_ty.generics);
}
pub fn visit_static(vis: &mut (impl Visitor + ?Sized), statik: &Static) {
    vis.visit_item_meta(&statik.meta);
    vis.visit_type(&statik.type_);
}
pub fn visit_primitive(vis: &mut (impl Visitor + ?Sized), primitive: &Primitive) {
    for impls in &primitive.impls {
        vis.visit_identifiable(impls);
    }
}
pub trait Visitor {
    fn visit_crate(&mut self, krate: &Crate) {
        visit_crate(self, krate)
    }
    fn visit_identifiable(&mut self, identifiable: &Identifiable) {
        visit_identifiable(self, identifiable)
    }
    fn visit_external_crate(&mut self, external_crate: &ExternalCrate) {}
    fn visit_item_summary(&mut self, item_summary: &ItemSummary) {
        visit_item_summary(self, item_summary)
    }
    fn visit_item_meta(&mut self, item_meta: &ItemMeta) {
        visit_item_meta(self, item_meta)
    }
    fn visit_span(&mut self, span: &Span) {}
    fn visit_deprecation(&mut self, deprecation: &Deprecation) {}
    fn visit_visibility(&mut self, visibility: &Visibility) {
        visit_visibility(self, visibility)
    }
    fn visit_dyn_trait(&mut self, dyn_trait: &DynTrait) {
        visit_dyn_trait(self, dyn_trait)
    }
    fn visit_poly_trait(&mut self, poly_trait: &PolyTrait) {
        visit_poly_trait(self, poly_trait)
    }
    fn visit_generic_args(&mut self, generic_args: &GenericArgs) {
        visit_generic_args(self, generic_args)
    }
    fn visit_generic_arg(&mut self, generic_arg: &GenericArg) {
        visit_generic_arg(self, generic_arg)
    }
    fn visit_constant_item(&mut self, constant_item: &ConstantItem) {
        visit_constant_item(self, constant_item)
    }
    fn visit_constant(&mut self, constant: &Constant) {
        visit_constant(self, constant)
    }
    fn visit_type_binding(&mut self, type_binding: &TypeBinding) {
        visit_type_binding(self, type_binding)
    }
    fn visit_type_binding_kind(&mut self, type_binding_kind: &TypeBindingKind) {
        visit_type_binding_kind(self, type_binding_kind)
    }
    fn visit_item_kind(&mut self, item_kind: &ItemKind) {}
    fn visit_item(&mut self, item: &Item) {
        visit_item(self, item)
    }
    fn visit_module(&mut self, module: &Module) {
        visit_module(self, module)
    }
    fn visit_union(&mut self, union: &Union) {
        visit_union(self, union)
    }
    fn visit_struct(&mut self, struct_: &Struct) {
        visit_struct(self, struct_)
    }
    fn visit_struct_kind(&mut self, struct_kind: &StructKind) {
        visit_struct_kind(self, struct_kind)
    }
    fn visit_struct_field(&mut self, struct_field: &StructField) {
        visit_struct_field(self, struct_field)
    }
    fn visit_enum(&mut self, enum_: &Enum) {
        visit_enum(self, enum_)
    }
    fn visit_variant(&mut self, variant: &Variant) {
        visit_variant(self, variant)
    }
    fn visit_variant_kind(&mut self, variant_kind: &VariantKind) {
        visit_variant_kind(self, variant_kind)
    }
    fn visit_discriminant(&mut self, discriminant: &Discriminant) {}
    fn visit_header(&mut self, header: &Header) {
        visit_header(self, header)
    }
    fn visit_abi(&mut self, abi: &Abi) {}
    fn visit_function(&mut self, function: &Function) {
        visit_function(self, function)
    }
    fn visit_generics(&mut self, generics: &Generics) {
        visit_generics(self, generics)
    }
    fn visit_generic_param_def(&mut self, generic_param_def: &GenericParamDef) {
        visit_generic_param_def(self, generic_param_def)
    }
    fn visit_generic_param_def_kind(&mut self, generic_param_def_kind: &GenericParamDefKind) {
        visit_generic_param_def_kind(self, generic_param_def_kind)
    }
    fn visit_where_predicate(&mut self, where_predicate: &WherePredicate) {
        visit_where_predicate(self, where_predicate)
    }
    fn visit_generic_bound(&mut self, generic_bound: &GenericBound) {
        visit_generic_bound(self, generic_bound)
    }
    fn visit_trait_bound_modifier(&mut self, trait_bound_modifier: &TraitBoundModifier) {}
    fn visit_term(&mut self, term: &Term) {
        visit_term(self, term)
    }
    fn visit_type(&mut self, ty: &Type) {
        visit_type(self, ty)
    }
    fn visit_path(&mut self, path: &Path) {
        visit_path(self, path)
    }
    fn visit_function_pointer(&mut self, function_pointer: &FunctionPointer) {
        visit_function_pointer(self, function_pointer)
    }
    fn visit_fn_decl(&mut self, fn_decl: &FnDecl) {
        visit_fn_decl(self, fn_decl)
    }
    fn visit_fn_input(&mut self, fn_input: &FnInput) {
        visit_fn_input(self, fn_input)
    }
    fn visit_trait(&mut self, trait_: &Trait) {
        visit_trait(self, trait_)
    }
    fn visit_trait_alias(&mut self, trait_alias: &TraitAlias) {
        visit_trait_alias(self, trait_alias)
    }
    fn visit_impl(&mut self, imp: &Impl) {
        visit_impl(self, imp)
    }
    fn visit_import(&mut self, import: &Import) {
        visit_import(self, import)
    }
    fn visit_proc_macro(&mut self, proc_macro: &ProcMacro) {
        visit_proc_macro(self, proc_macro)
    }
    fn visit_macro_kind(&mut self, macro_kind: &MacroKind) {}
    fn visit_typedef(&mut self, typedef: &Typedef) {
        visit_typedef(self, typedef)
    }
    fn visit_opaque_ty(&mut self, opaque_ty: &OpaqueTy) {
        visit_opaque_ty(self, opaque_ty)
    }
    fn visit_static(&mut self, statik: &Static) {
        visit_static(self, statik)
    }
    fn visit_primitive(&mut self, primitive: &Primitive) {
        visit_primitive(self, primitive)
    }
}

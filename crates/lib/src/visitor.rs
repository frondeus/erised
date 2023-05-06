use crate::heap_types::*;

pub trait Visitor {
    fn visit_crate(&mut self, krate: &Crate) {
        visit_crate(self, krate);
    }
    fn visit_module(&mut self, module: &Module) {
        visit_module(self, module);
    }
    fn visit_identifiable(&mut self, ident: &Identifiable) {
        visit_identifiable(self, ident);
    }
    fn visit_item_meta(&mut self, meta: &ItemMeta) {
        visit_item_meta(self, meta);
    }
    fn visit_summary(&mut self, summary: &ItemSummary) {
        visit_summary(self, summary);
    }
    fn visit_external_crate(&mut self, ext: &ExternalCrate) {
        visit_external_crate(self, ext);
    }
    fn visit_item(&mut self, item: &Item) {
        visit_item(self, item);
    }
    fn visit_extern_crate(&mut self, meta: &ItemMeta, name: &str, rename: Option<&str>) {
        visit_extern_crate(self, meta, name, rename);
    }
    fn visit_import(&mut self, imp: &Import) {
        visit_import(self, imp);
    }
    fn visit_union(&mut self, u: &Union) {
        visit_union(self, u);
    }
    fn visit_struct(&mut self, s: &Struct) {
        visit_struct(self, s);
    }
    fn visit_enum(&mut self, e: &Enum) {
        visit_enum(self, e)
    }
    fn visit_function(&mut self, f: &Function) {
        visit_function(self, f)
    }
    fn visit_trait(&mut self, t: &Trait) {
        visit_trait(self, t)
    }
    fn visit_trait_alias(&mut self, t: &TraitAlias) {
        visit_trait_alias(self, t)
    }
    fn visit_impl(&mut self, i: &Impl) {
        visit_impl(self, i)
    }
    fn visit_typedef(&mut self, t: &Typedef) {
        visit_typedef(self, t)
    }
    fn visit_opaque_type(&mut self, o: &OpaqueTy) {
        visit_opaque_type(self, o)
    }
    fn visit_constant_item(&mut self, c: &ConstantItem) {
        visit_constant_item(self, c)
    }
    fn visit_static(&mut self, s: &Static) {
        visit_static(self, s)
    }
    fn visit_foreign_type(&mut self) {
        visit_foreign_type(self)
    }
    fn visit_macro(&mut self, name: &str, meta: &ItemMeta, expr: &str) {
        visit_macro(self, name, meta, expr)
    }
    fn visit_proc_macro(&mut self, p: &ProcMacro) {
        visit_proc_macro(self, p)
    }
    fn visit_primitive_item(&mut self, p: &Primitive) {
        visit_primitive_item(self, p)
    }
    fn visit_assoc_const(&mut self, meta: &ItemMeta, type_: &Type, default: Option<&str>) {
        visit_assoc_const(self, meta, type_, default)
    }
    fn visit_assoc_type(
        &mut self,
        meta: &ItemMeta,
        generics: &Generics,
        bounds: &[GenericBound],
        default: Option<&Type>,
    ) {
        visit_assoc_type(self, meta, generics, bounds, default)
    }
    fn visit_generics(&mut self, generics: &Generics) {
        visit_generics(self, generics);
    }
    fn visit_generic_param_def(&mut self, param: &GenericParamDef) {
        visit_generic_param_def(self, param);
    }
    fn visit_where_predicate(&mut self, predicate: &WherePredicate) {
        visit_where_predicate(self, predicate);
    }
    fn visit_struct_field(&mut self, field: &StructField) {
        visit_struct_field(self, field);
    }
    fn visit_type(&mut self, ty: &Type) {
        visit_type(self, ty);
    }
    fn visit_resolved_path(&mut self, r: &Path) {
        visit_resolved_path(self, r);
    }
    fn visit_dyn_trait(&mut self, d: &DynTrait) {
        visit_dyn_trait(self, d);
    }
    fn visit_function_pointer(&mut self, f: &FunctionPointer) {
        visit_function_pointer(self, f);
    }
    fn visit_tuple(&mut self, t: &[Type]) {
        visit_tuple(self, t)
    }
    fn visit_array(&mut self, type_: &Type, len: &str) {
        visit_array(self, type_, len)
    }
    fn visit_impl_trait(&mut self, i: &[GenericBound]) {
        visit_impl_trait(self, i)
    }
    fn visit_raw_pointer(&mut self, mutable: bool, type_: &Type) {
        visit_raw_pointer(self, mutable, type_)
    }
    fn visit_borrowed_ref(&mut self, lifetime: Option<&str>, mutable: bool, type_: &Type) {
        visit_borrowed_ref(self, lifetime, mutable, type_)
    }
    fn visit_qualified_path(
        &mut self,

        name: &str,
        args: &GenericArgs,
        self_type: &Type,
        trait_: &Path,
    ) {
        visit_qualified_path(self, name, args, self_type, trait_)
    }
    fn visit_function_decl(&mut self, decl: &FnDecl) {
        visit_function_decl(self, decl);
    }
    fn visit_function_header(&mut self, header: &Header) {
        visit_function_header(self, header);
    }
    fn visit_function_input(&mut self, input: &FnInput) {
        visit_function_input(self, input);
    }
    fn visit_generic_args(&mut self, args: &GenericArgs) {
        visit_generic_args(self, args);
    }
    fn visit_generic_arg(&mut self, arg: &GenericArg) {
        visit_generic_arg(self, arg);
    }
    fn visit_type_binding(&mut self, binding: &TypeBinding) {
        visit_type_binding(self, binding);
    }
    fn visit_constant(&mut self, con: &Constant) {
        visit_constant(self, con);
    }
    fn visit_variant(&mut self, variant: &Variant) {
        visit_variant(self, variant);
    }
    fn visit_generic_bound(&mut self, bound: &GenericBound) {
        visit_generic_bound(self, bound);
    }
}

pub fn visit_crate(vis: &mut (impl Visitor + ?Sized), krate: &Crate) {
    vis.visit_module(&krate.root);
}
pub fn visit_module(vis: &mut (impl Visitor + ?Sized), module: &Module) {
    vis.visit_item_meta(&module.meta);
    for ident in &module.items {
        vis.visit_identifiable(ident);
    }
}
pub fn visit_identifiable(vis: &mut (impl Visitor + ?Sized), ident: &Identifiable) {
    match ident {
        Identifiable::Item(item) => {
            if let Some(item) = item.upgrade() {
                vis.visit_item(&item);
            }
        }
        Identifiable::Summary(summary) => vis.visit_summary(summary),
    }
}
pub fn visit_item_meta(vis: &mut (impl Visitor + ?Sized), meta: &ItemMeta) {
    vis.visit_external_crate(&meta.krate);
    if let Some(summary) = meta.summary.as_ref() {
        vis.visit_summary(summary);
    }
}
pub fn visit_summary(vis: &mut (impl Visitor + ?Sized), summary: &ItemSummary) {
    vis.visit_external_crate(&summary.krate);
}
pub fn visit_external_crate(_vis: &mut (impl Visitor + ?Sized), _external_crate: &ExternalCrate) {
    // Nothing to do here
}
pub fn visit_item(vis: &mut (impl Visitor + ?Sized), item: &Item) {
    match item {
        Item::Module(m) => vis.visit_module(m),
        Item::ExternCrate { meta, name, rename } => {
            vis.visit_extern_crate(meta, name, rename.as_deref())
        }
        Item::Import(i) => vis.visit_import(i),
        Item::Union(u) => vis.visit_union(u),
        Item::Struct(s) => vis.visit_struct(s),
        Item::Enum(e) => vis.visit_enum(e),
        Item::Function(f) => vis.visit_function(f),
        Item::Trait(t) => vis.visit_trait(t),
        Item::TraitAlias(t) => vis.visit_trait_alias(t),
        Item::Impl(i) => vis.visit_impl(i),
        Item::Typedef(t) => vis.visit_typedef(t),
        Item::OpaqueTy(o) => vis.visit_opaque_type(o),
        Item::Constant(c) => vis.visit_constant_item(c),
        Item::Static(s) => vis.visit_static(s),
        Item::ForeignType => vis.visit_foreign_type(),
        Item::Macro { name, meta, expr } => vis.visit_macro(name, meta, expr),
        Item::ProcMacro(p) => vis.visit_proc_macro(p),
        Item::Primitive(p) => vis.visit_primitive_item(p),
        Item::AssocConst {
            meta,
            type_,
            default,
        } => vis.visit_assoc_const(meta, type_, default.as_deref()),
        Item::AssocType {
            meta,
            generics,
            bounds,
            default,
        } => vis.visit_assoc_type(meta, generics, bounds, default.as_ref()),
    }
}
pub fn visit_extern_crate(
    vis: &mut (impl Visitor + ?Sized),
    meta: &ItemMeta,
    _name: &str,
    _rename: Option<&str>,
) {
    vis.visit_item_meta(meta);
}
pub fn visit_import(vis: &mut (impl Visitor + ?Sized), imp: &Import) {
    vis.visit_item_meta(&imp.meta);
    if let Some(target) = imp.target.as_ref() {
        vis.visit_identifiable(target);
    }
}
pub fn visit_union(_vis: &mut (impl Visitor + ?Sized), _u: &Union) {
    todo!()
}
pub fn visit_struct(vis: &mut (impl Visitor + ?Sized), s: &Struct) {
    vis.visit_item_meta(&s.meta);
    vis.visit_generics(&s.generics);
    match &s.kind {
        StructKind::Unit => (),
        StructKind::Tuple(fields) => {
            for field in fields {
                if let Some(field) = field.as_ref() {
                    vis.visit_struct_field(field);
                }
            }
        }
        StructKind::Plain {
            fields,
            fields_stripped: _,
        } => {
            for field in fields {
                vis.visit_struct_field(field);
            }
        }
    }
}

pub fn visit_struct_field(vis: &mut (impl Visitor + ?Sized), f: &StructField) {
    vis.visit_item_meta(&f.meta);
    vis.visit_type(&f.ty);
}

pub fn visit_enum(vis: &mut (impl Visitor + ?Sized), e: &Enum) {
    vis.visit_item_meta(&e.meta);
    vis.visit_generics(&e.generics);
    for variant in &e.variants {
        vis.visit_variant(variant);
    }
    for imp in &e.impls {
        vis.visit_identifiable(imp);
    }
}
pub fn visit_variant(vis: &mut (impl Visitor + ?Sized), variant: &Variant) {
    vis.visit_item_meta(&variant.meta);
    match &variant.kind {
        VariantKind::Plain => (),
        VariantKind::Tuple(fields) => {
            for field in fields {
                if let Some(field) = field.as_ref() {
                    vis.visit_struct_field(field);
                }
            }
        }
        VariantKind::Struct {
            fields,
            fields_stripped: _,
        } => {
            for field in fields {
                vis.visit_struct_field(field);
            }
        }
    }
}
pub fn visit_function(vis: &mut (impl Visitor + ?Sized), f: &Function) {
    vis.visit_item_meta(&f.meta);
    vis.visit_function_decl(&f.decl);
    vis.visit_generics(&f.generics);
    vis.visit_function_header(&f.header);
}
pub fn visit_trait(vis: &mut (impl Visitor + ?Sized), t: &Trait) {
    vis.visit_item_meta(&t.meta);
    for item in &t.items {
        vis.visit_identifiable(item);
    }
    vis.visit_generics(&t.generics);
    for bound in &t.bounds {
        vis.visit_generic_bound(bound);
    }
    for imp in &t.implementations {
        vis.visit_identifiable(imp);
    }
}
pub fn visit_trait_alias(_vis: &mut (impl Visitor + ?Sized), _t: &TraitAlias) {
    todo!()
}
pub fn visit_impl(vis: &mut (impl Visitor + ?Sized), i: &Impl) {
    vis.visit_item_meta(&i.meta);
    vis.visit_generics(&i.generics);
    vis.visit_type(&i.for_);
    if let Some(path) = i.trait_.as_ref() {
        vis.visit_resolved_path(path);
    }
    for item in &i.items {
        if let Some(item) = item.upgrade().as_ref() {
            vis.visit_item(item);
        }
    }
    if let Some(blanket_impl) = i.blanket_impl.as_ref() {
        vis.visit_type(blanket_impl);
    }
}
pub fn visit_typedef(vis: &mut (impl Visitor + ?Sized), t: &Typedef) {
    vis.visit_item_meta(&t.meta);
    vis.visit_type(&t.type_);
    vis.visit_generics(&t.generics);
}
pub fn visit_opaque_type(_vis: &mut (impl Visitor + ?Sized), _o: &OpaqueTy) {
    todo!()
}
pub fn visit_constant_item(_vis: &mut (impl Visitor + ?Sized), _c: &ConstantItem) {
    todo!()
}
pub fn visit_static(_vis: &mut (impl Visitor + ?Sized), _s: &Static) {
    todo!()
}
pub fn visit_foreign_type(_vis: &mut (impl Visitor + ?Sized)) {
    todo!()
}
pub fn visit_macro(vis: &mut (impl Visitor + ?Sized), _name: &str, meta: &ItemMeta, _expr: &str) {
    vis.visit_item_meta(meta);
}
pub fn visit_proc_macro(_vis: &mut (impl Visitor + ?Sized), _p: &ProcMacro) {
    todo!()
}
pub fn visit_primitive_item(_vis: &mut (impl Visitor + ?Sized), _p: &Primitive) {
    todo!()
}
pub fn visit_assoc_const(
    _vis: &mut (impl Visitor + ?Sized),
    _meta: &ItemMeta,
    _type_: &Type,
    _default: Option<&str>,
) {
    todo!()
}
pub fn visit_assoc_type(
    vis: &mut (impl Visitor + ?Sized),
    meta: &ItemMeta,
    generics: &Generics,
    bounds: &[GenericBound],
    default: Option<&Type>,
) {
    vis.visit_item_meta(meta);
    vis.visit_generics(generics);
    for bound in bounds {
        vis.visit_generic_bound(bound);
    }
    if let Some(default) = default {
        vis.visit_type(default);
    }
}

pub fn visit_generics(vis: &mut (impl Visitor + ?Sized), generics: &Generics) {
    for param in &generics.params {
        vis.visit_generic_param_def(param);
    }
    for predicate in &generics.where_predicates {
        vis.visit_where_predicate(predicate);
    }
}
pub fn visit_generic_param_def(
    vis: &mut (impl Visitor + ?Sized),
    generic_param_def: &GenericParamDef,
) {
    match &generic_param_def.kind {
        GenericParamDefKind::Lifetime { .. } => (),
        GenericParamDefKind::Type {
            bounds, default, ..
        } => {
            for bound in bounds {
                vis.visit_generic_bound(bound);
            }
            if let Some(default) = default.as_ref() {
                vis.visit_type(default);
            }
        }
        GenericParamDefKind::Const { type_, .. } => {
            vis.visit_type(type_);
        }
    }
}
pub fn visit_where_predicate(vis: &mut (impl Visitor + ?Sized), where_predicate: &WherePredicate) {
    match where_predicate {
        WherePredicate::BoundPredicate {
            type_,
            bounds,
            generic_params,
        } => {
            vis.visit_type(type_);
            for bound in bounds {
                vis.visit_generic_bound(bound);
            }
            for param in generic_params {
                vis.visit_generic_param_def(param);
            }
        }
        WherePredicate::RegionPredicate { bounds, .. } => {
            for bound in bounds {
                vis.visit_generic_bound(bound);
            }
        }
        WherePredicate::EqPredicate { lhs, rhs } => {
            vis.visit_type(lhs);
            match rhs {
                Term::Type(ty) => vis.visit_type(ty),
                Term::Constant(co) => vis.visit_constant(co),
            }
        }
    }
}
pub fn visit_type(vis: &mut (impl Visitor + ?Sized), ty: &Type) {
    match ty {
        Type::ResolvedPath(rp) => vis.visit_resolved_path(rp),
        Type::DynTrait(d) => vis.visit_dyn_trait(d),
        Type::Generic(_) => (),
        Type::Primitive(_) => (),
        Type::FunctionPointer(f) => vis.visit_function_pointer(f),
        Type::Tuple(t) => vis.visit_tuple(t),
        Type::Slice(ty) => vis.visit_type(ty),
        Type::Array { type_, len } => vis.visit_array(type_, len),
        Type::ImplTrait(i) => vis.visit_impl_trait(i),
        Type::Infer => (),
        Type::RawPointer { mutable, type_ } => vis.visit_raw_pointer(*mutable, type_),
        Type::BorrowedRef {
            lifetime,
            mutable,
            type_,
        } => vis.visit_borrowed_ref(lifetime.as_deref(), *mutable, type_),
        Type::QualifiedPath {
            name,
            args,
            self_type,
            trait_,
        } => vis.visit_qualified_path(name, args, self_type, trait_),
    }
}

fn visit_resolved_path(vis: &mut (impl Visitor + ?Sized), r: &Path) {
    vis.visit_identifiable(&r.target);
    if let Some(args) = r.args.as_ref() {
        vis.visit_generic_args(args);
    }
}
fn visit_dyn_trait(vis: &mut (impl Visitor + ?Sized), d: &DynTrait) {
    for trait_ in &d.traits {
        vis.visit_resolved_path(&trait_.trait_);
        for param in &trait_.generic_params {
            vis.visit_generic_param_def(param);
        }
    }
}
fn visit_function_pointer(vis: &mut (impl Visitor + ?Sized), f: &FunctionPointer) {
    vis.visit_function_decl(&f.decl);
    for param in &f.generic_params {
        vis.visit_generic_param_def(param);
    }
    vis.visit_function_header(&f.header);
}
fn visit_function_decl(vis: &mut (impl Visitor + ?Sized), decl: &FnDecl) {
    for input in &decl.inputs {
        vis.visit_function_input(input);
    }
    if let Some(opt) = decl.output.as_ref() {
        vis.visit_type(opt);
    }
}
fn visit_function_input(vis: &mut (impl Visitor + ?Sized), input: &FnInput) {
    vis.visit_type(&input.ty);
}
fn visit_function_header(_vis: &mut (impl Visitor + ?Sized), _header: &Header) {}
fn visit_tuple(vis: &mut (impl Visitor + ?Sized), t: &[Type]) {
    for t in t {
        vis.visit_type(t);
    }
}
fn visit_array(_vis: &mut (impl Visitor + ?Sized), _type_: &Type, _len: &str) {
    todo!()
}
fn visit_impl_trait(vis: &mut (impl Visitor + ?Sized), bounds: &[GenericBound]) {
    for bound in bounds {
        vis.visit_generic_bound(bound);
    }
}

fn visit_raw_pointer(_vis: &mut (impl Visitor + ?Sized), _mutable: bool, _type_: &Type) {
    todo!()
}
fn visit_borrowed_ref(
    vis: &mut (impl Visitor + ?Sized),

    _lifetime: Option<&str>,
    _mutable: bool,
    type_: &Type,
) {
    vis.visit_type(type_);
}
fn visit_qualified_path(
    vis: &mut (impl Visitor + ?Sized),

    _name: &str,
    args: &GenericArgs,
    self_type: &Type,
    trait_: &Path,
) {
    vis.visit_generic_args(args);
    vis.visit_type(self_type);
    vis.visit_resolved_path(trait_);
}

fn visit_generic_args(vis: &mut (impl Visitor + ?Sized), args: &GenericArgs) {
    match args {
        GenericArgs::AngleBracketed { args, bindings } => {
            for arg in args {
                vis.visit_generic_arg(arg);
            }
            for binding in bindings {
                vis.visit_type_binding(binding);
            }
        }
        GenericArgs::Parenthesized { inputs, output } => {
            for input in inputs {
                vis.visit_type(input);
            }
            if let Some(output) = output.as_ref() {
                vis.visit_type(output);
            }
        }
    }
}
fn visit_generic_arg(vis: &mut (impl Visitor + ?Sized), arg: &GenericArg) {
    match arg {
        GenericArg::Lifetime(_) => (),
        GenericArg::Type(ty) => vis.visit_type(ty),
        GenericArg::Const(con) => vis.visit_constant(con),
        GenericArg::Infer => (),
    }
}
fn visit_type_binding(_vis: &mut (impl Visitor + ?Sized), _binding: &TypeBinding) {
    todo!()
}
fn visit_constant(vis: &mut (impl Visitor + ?Sized), con: &Constant) {
    vis.visit_type(&con.type_);
}
fn visit_generic_bound(vis: &mut (impl Visitor + ?Sized), bound: &GenericBound) {
    match bound {
        GenericBound::TraitBound {
            trait_,
            generic_params,
            ..
        } => {
            vis.visit_resolved_path(trait_);
            for param in generic_params {
                vis.visit_generic_param_def(param);
            }
        }
        GenericBound::Outlives(_) => (),
    }
}

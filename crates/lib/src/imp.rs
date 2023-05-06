use crate as erised;
use crate::heap_types::*;
use std::sync::{Arc, Weak};
impl erised::destruct::ToTokens for Crate {
    fn to_tokens(&self, paths: &mut erised::destruct::ItemsPaths) -> proc_macro2::TokenStream {
        let &Self {
            root,
            crate_version,
            all_items,
            ..
        } = &self;
        let root = erised::destruct::ToTokens::to_tokens(root, paths);
        let crate_version = erised::destruct::ToTokens::to_tokens(crate_version, paths);
        let all_items = erised::destruct::ToTokens::to_tokens(all_items, paths);
        quote :: quote ! (erised :: types :: Crate { root : #root , crate_version : #crate_version , all_items : #all_items , })
    }
}
impl quote::ToTokens for Crate {
    fn to_tokens(&self, tokens: &mut proc_macro2::TokenStream) {
        use quote::TokenStreamExt;
        let mut paths = erised::destruct::ItemsPaths::default();
        tokens.append_all(erised::destruct::ToTokens::to_tokens(self, &mut paths))
    }
}
impl erised::destruct::ToTokens for Identifiable {
    fn to_tokens(&self, paths: &mut erised::destruct::ItemsPaths) -> proc_macro2::TokenStream {
        match &self {
            Self::Item(_0) => {
                let _0 = erised::destruct::ToTokens::to_tokens(_0, paths);
                quote :: quote ! (erised :: types :: Identifiable :: Item (#_0 ,))
            }
            Self::Summary(_0) => {
                let _0 = erised::destruct::ToTokens::to_tokens(_0, paths);
                quote :: quote ! (erised :: types :: Identifiable :: Summary (#_0 ,))
            }
        }
    }
}
impl quote::ToTokens for Identifiable {
    fn to_tokens(&self, tokens: &mut proc_macro2::TokenStream) {
        use quote::TokenStreamExt;
        let mut paths = erised::destruct::ItemsPaths::default();
        tokens.append_all(erised::destruct::ToTokens::to_tokens(self, &mut paths))
    }
}
impl erised::destruct::ToTokens for ExternalCrate {
    fn to_tokens(&self, paths: &mut erised::destruct::ItemsPaths) -> proc_macro2::TokenStream {
        let &Self {
            name,
            html_root_url,
            ..
        } = &self;
        let name = erised::destruct::ToTokens::to_tokens(name, paths);
        let html_root_url = erised::destruct::ToTokens::to_tokens(html_root_url, paths);
        quote :: quote ! (erised :: types :: ExternalCrate { name : #name , html_root_url : #html_root_url , })
    }
}
impl quote::ToTokens for ExternalCrate {
    fn to_tokens(&self, tokens: &mut proc_macro2::TokenStream) {
        use quote::TokenStreamExt;
        let mut paths = erised::destruct::ItemsPaths::default();
        tokens.append_all(erised::destruct::ToTokens::to_tokens(self, &mut paths))
    }
}
impl erised::destruct::ToTokens for ItemSummary {
    fn to_tokens(&self, paths: &mut erised::destruct::ItemsPaths) -> proc_macro2::TokenStream {
        let &Self {
            krate, path, kind, ..
        } = &self;
        let krate = erised::destruct::ToTokens::to_tokens(krate, paths);
        let path = erised::destruct::ToTokens::to_tokens(path, paths);
        let kind = erised::destruct::ToTokens::to_tokens(kind, paths);
        quote :: quote ! (erised :: types :: ItemSummary { krate : #krate , path : #path , kind : #kind , })
    }
}
impl quote::ToTokens for ItemSummary {
    fn to_tokens(&self, tokens: &mut proc_macro2::TokenStream) {
        use quote::TokenStreamExt;
        let mut paths = erised::destruct::ItemsPaths::default();
        tokens.append_all(erised::destruct::ToTokens::to_tokens(self, &mut paths))
    }
}
impl erised::destruct::ToTokens for ItemMeta {
    fn to_tokens(&self, paths: &mut erised::destruct::ItemsPaths) -> proc_macro2::TokenStream {
        let &Self {
            krate,
            summary,
            span,
            visibility,
            docs,
            attrs,
            deprecation,
            ..
        } = &self;
        let krate = erised::destruct::ToTokens::to_tokens(krate, paths);
        let summary = erised::destruct::ToTokens::to_tokens(summary, paths);
        let span = erised::destruct::ToTokens::to_tokens(span, paths);
        let visibility = erised::destruct::ToTokens::to_tokens(visibility, paths);
        let docs = erised::destruct::ToTokens::to_tokens(docs, paths);
        let attrs = erised::destruct::ToTokens::to_tokens(attrs, paths);
        let deprecation = erised::destruct::ToTokens::to_tokens(deprecation, paths);
        quote :: quote ! (erised :: types :: ItemMeta { krate : #krate , summary : #summary , span : #span , visibility : #visibility , docs : #docs , attrs : #attrs , deprecation : #deprecation , })
    }
}
impl quote::ToTokens for ItemMeta {
    fn to_tokens(&self, tokens: &mut proc_macro2::TokenStream) {
        use quote::TokenStreamExt;
        let mut paths = erised::destruct::ItemsPaths::default();
        tokens.append_all(erised::destruct::ToTokens::to_tokens(self, &mut paths))
    }
}
impl erised::destruct::ToTokens for Span {
    fn to_tokens(&self, paths: &mut erised::destruct::ItemsPaths) -> proc_macro2::TokenStream {
        let &Self {
            filename,
            begin,
            end,
            ..
        } = &self;
        let filename = erised::destruct::ToTokens::to_tokens(filename, paths);
        let begin = erised::destruct::ToTokens::to_tokens(begin, paths);
        let end = erised::destruct::ToTokens::to_tokens(end, paths);
        quote :: quote ! (erised :: types :: Span { filename : #filename , begin : #begin , end : #end , })
    }
}
impl quote::ToTokens for Span {
    fn to_tokens(&self, tokens: &mut proc_macro2::TokenStream) {
        use quote::TokenStreamExt;
        let mut paths = erised::destruct::ItemsPaths::default();
        tokens.append_all(erised::destruct::ToTokens::to_tokens(self, &mut paths))
    }
}
impl erised::destruct::ToTokens for Deprecation {
    fn to_tokens(&self, paths: &mut erised::destruct::ItemsPaths) -> proc_macro2::TokenStream {
        let &Self { since, note, .. } = &self;
        let since = erised::destruct::ToTokens::to_tokens(since, paths);
        let note = erised::destruct::ToTokens::to_tokens(note, paths);
        quote :: quote ! (erised :: types :: Deprecation { since : #since , note : #note , })
    }
}
impl quote::ToTokens for Deprecation {
    fn to_tokens(&self, tokens: &mut proc_macro2::TokenStream) {
        use quote::TokenStreamExt;
        let mut paths = erised::destruct::ItemsPaths::default();
        tokens.append_all(erised::destruct::ToTokens::to_tokens(self, &mut paths))
    }
}
impl erised::destruct::ToTokens for Visibility {
    fn to_tokens(&self, paths: &mut erised::destruct::ItemsPaths) -> proc_macro2::TokenStream {
        match &self {
            Self::Public => quote::quote!(erised::types::Visibility::Public),
            Self::Default => quote::quote!(erised::types::Visibility::Default),
            Self::Crate => quote::quote!(erised::types::Visibility::Crate),
            Self::Restricted { parent, path } => {
                let parent = erised::destruct::ToTokens::to_tokens(parent, paths);
                let path = erised::destruct::ToTokens::to_tokens(path, paths);
                quote :: quote ! (erised :: types :: Visibility :: Restricted { parent : #parent , path : #path , })
            }
        }
    }
}
impl quote::ToTokens for Visibility {
    fn to_tokens(&self, tokens: &mut proc_macro2::TokenStream) {
        use quote::TokenStreamExt;
        let mut paths = erised::destruct::ItemsPaths::default();
        tokens.append_all(erised::destruct::ToTokens::to_tokens(self, &mut paths))
    }
}
impl erised::destruct::ToTokens for DynTrait {
    fn to_tokens(&self, paths: &mut erised::destruct::ItemsPaths) -> proc_macro2::TokenStream {
        let &Self {
            traits, lifetime, ..
        } = &self;
        let traits = erised::destruct::ToTokens::to_tokens(traits, paths);
        let lifetime = erised::destruct::ToTokens::to_tokens(lifetime, paths);
        quote :: quote ! (erised :: types :: DynTrait { traits : #traits , lifetime : #lifetime , })
    }
}
impl quote::ToTokens for DynTrait {
    fn to_tokens(&self, tokens: &mut proc_macro2::TokenStream) {
        use quote::TokenStreamExt;
        let mut paths = erised::destruct::ItemsPaths::default();
        tokens.append_all(erised::destruct::ToTokens::to_tokens(self, &mut paths))
    }
}
impl erised::destruct::ToTokens for PolyTrait {
    fn to_tokens(&self, paths: &mut erised::destruct::ItemsPaths) -> proc_macro2::TokenStream {
        let &Self {
            trait_,
            generic_params,
            ..
        } = &self;
        let trait_ = erised::destruct::ToTokens::to_tokens(trait_, paths);
        let generic_params = erised::destruct::ToTokens::to_tokens(generic_params, paths);
        quote :: quote ! (erised :: types :: PolyTrait { trait_ : #trait_ , generic_params : #generic_params , })
    }
}
impl quote::ToTokens for PolyTrait {
    fn to_tokens(&self, tokens: &mut proc_macro2::TokenStream) {
        use quote::TokenStreamExt;
        let mut paths = erised::destruct::ItemsPaths::default();
        tokens.append_all(erised::destruct::ToTokens::to_tokens(self, &mut paths))
    }
}
impl erised::destruct::ToTokens for GenericArgs {
    fn to_tokens(&self, paths: &mut erised::destruct::ItemsPaths) -> proc_macro2::TokenStream {
        match &self {
            Self::AngleBracketed { args, bindings } => {
                let args = erised::destruct::ToTokens::to_tokens(args, paths);
                let bindings = erised::destruct::ToTokens::to_tokens(bindings, paths);
                quote :: quote ! (erised :: types :: GenericArgs :: AngleBracketed { args : #args , bindings : #bindings , })
            }
            Self::Parenthesized { inputs, output } => {
                let inputs = erised::destruct::ToTokens::to_tokens(inputs, paths);
                let output = erised::destruct::ToTokens::to_tokens(output, paths);
                quote :: quote ! (erised :: types :: GenericArgs :: Parenthesized { inputs : #inputs , output : #output , })
            }
        }
    }
}
impl quote::ToTokens for GenericArgs {
    fn to_tokens(&self, tokens: &mut proc_macro2::TokenStream) {
        use quote::TokenStreamExt;
        let mut paths = erised::destruct::ItemsPaths::default();
        tokens.append_all(erised::destruct::ToTokens::to_tokens(self, &mut paths))
    }
}
impl erised::destruct::ToTokens for GenericArg {
    fn to_tokens(&self, paths: &mut erised::destruct::ItemsPaths) -> proc_macro2::TokenStream {
        match &self {
            Self::Lifetime(_0) => {
                let _0 = erised::destruct::ToTokens::to_tokens(_0, paths);
                quote :: quote ! (erised :: types :: GenericArg :: Lifetime (#_0 ,))
            }
            Self::Type(_0) => {
                let _0 = erised::destruct::ToTokens::to_tokens(_0, paths);
                quote :: quote ! (erised :: types :: GenericArg :: Type (#_0 ,))
            }
            Self::Const(_0) => {
                let _0 = erised::destruct::ToTokens::to_tokens(_0, paths);
                quote :: quote ! (erised :: types :: GenericArg :: Const (#_0 ,))
            }
            Self::Infer => quote::quote!(erised::types::GenericArg::Infer),
        }
    }
}
impl quote::ToTokens for GenericArg {
    fn to_tokens(&self, tokens: &mut proc_macro2::TokenStream) {
        use quote::TokenStreamExt;
        let mut paths = erised::destruct::ItemsPaths::default();
        tokens.append_all(erised::destruct::ToTokens::to_tokens(self, &mut paths))
    }
}
impl erised::destruct::ToTokens for ConstantItem {
    fn to_tokens(&self, paths: &mut erised::destruct::ItemsPaths) -> proc_macro2::TokenStream {
        let &Self {
            name,
            meta,
            constant,
            ..
        } = &self;
        let name = erised::destruct::ToTokens::to_tokens(name, paths);
        let meta = erised::destruct::ToTokens::to_tokens(meta, paths);
        let constant = erised::destruct::ToTokens::to_tokens(constant, paths);
        quote :: quote ! (erised :: types :: ConstantItem { name : #name , meta : #meta , constant : #constant , })
    }
}
impl quote::ToTokens for ConstantItem {
    fn to_tokens(&self, tokens: &mut proc_macro2::TokenStream) {
        use quote::TokenStreamExt;
        let mut paths = erised::destruct::ItemsPaths::default();
        tokens.append_all(erised::destruct::ToTokens::to_tokens(self, &mut paths))
    }
}
impl erised::destruct::ToTokens for Constant {
    fn to_tokens(&self, paths: &mut erised::destruct::ItemsPaths) -> proc_macro2::TokenStream {
        let &Self {
            type_,
            expr,
            value,
            is_literal,
            ..
        } = &self;
        let type_ = erised::destruct::ToTokens::to_tokens(type_, paths);
        let expr = erised::destruct::ToTokens::to_tokens(expr, paths);
        let value = erised::destruct::ToTokens::to_tokens(value, paths);
        let is_literal = erised::destruct::ToTokens::to_tokens(is_literal, paths);
        quote :: quote ! (erised :: types :: Constant { type_ : #type_ , expr : #expr , value : #value , is_literal : #is_literal , })
    }
}
impl quote::ToTokens for Constant {
    fn to_tokens(&self, tokens: &mut proc_macro2::TokenStream) {
        use quote::TokenStreamExt;
        let mut paths = erised::destruct::ItemsPaths::default();
        tokens.append_all(erised::destruct::ToTokens::to_tokens(self, &mut paths))
    }
}
impl erised::destruct::ToTokens for TypeBinding {
    fn to_tokens(&self, paths: &mut erised::destruct::ItemsPaths) -> proc_macro2::TokenStream {
        let &Self {
            name,
            args,
            binding,
            ..
        } = &self;
        let name = erised::destruct::ToTokens::to_tokens(name, paths);
        let args = erised::destruct::ToTokens::to_tokens(args, paths);
        let binding = erised::destruct::ToTokens::to_tokens(binding, paths);
        quote :: quote ! (erised :: types :: TypeBinding { name : #name , args : #args , binding : #binding , })
    }
}
impl quote::ToTokens for TypeBinding {
    fn to_tokens(&self, tokens: &mut proc_macro2::TokenStream) {
        use quote::TokenStreamExt;
        let mut paths = erised::destruct::ItemsPaths::default();
        tokens.append_all(erised::destruct::ToTokens::to_tokens(self, &mut paths))
    }
}
impl erised::destruct::ToTokens for TypeBindingKind {
    fn to_tokens(&self, paths: &mut erised::destruct::ItemsPaths) -> proc_macro2::TokenStream {
        match &self {
            Self::Equality(_0) => {
                let _0 = erised::destruct::ToTokens::to_tokens(_0, paths);
                quote :: quote ! (erised :: types :: TypeBindingKind :: Equality (#_0 ,))
            }
            Self::Constraint(_0) => {
                let _0 = erised::destruct::ToTokens::to_tokens(_0, paths);
                quote :: quote ! (erised :: types :: TypeBindingKind :: Constraint (#_0 ,))
            }
        }
    }
}
impl quote::ToTokens for TypeBindingKind {
    fn to_tokens(&self, tokens: &mut proc_macro2::TokenStream) {
        use quote::TokenStreamExt;
        let mut paths = erised::destruct::ItemsPaths::default();
        tokens.append_all(erised::destruct::ToTokens::to_tokens(self, &mut paths))
    }
}
impl erised::destruct::ToTokens for ItemKind {
    fn to_tokens(&self, paths: &mut erised::destruct::ItemsPaths) -> proc_macro2::TokenStream {
        match &self {
            Self::Module => quote::quote!(erised::types::ItemKind::Module),
            Self::ExternCrate => quote::quote!(erised::types::ItemKind::ExternCrate),
            Self::Import => quote::quote!(erised::types::ItemKind::Import),
            Self::Struct => quote::quote!(erised::types::ItemKind::Struct),
            Self::StructField => quote::quote!(erised::types::ItemKind::StructField),
            Self::Union => quote::quote!(erised::types::ItemKind::Union),
            Self::Enum => quote::quote!(erised::types::ItemKind::Enum),
            Self::Variant => quote::quote!(erised::types::ItemKind::Variant),
            Self::Function => quote::quote!(erised::types::ItemKind::Function),
            Self::Typedef => quote::quote!(erised::types::ItemKind::Typedef),
            Self::OpaqueTy => quote::quote!(erised::types::ItemKind::OpaqueTy),
            Self::Constant => quote::quote!(erised::types::ItemKind::Constant),
            Self::Trait => quote::quote!(erised::types::ItemKind::Trait),
            Self::TraitAlias => quote::quote!(erised::types::ItemKind::TraitAlias),
            Self::Impl => quote::quote!(erised::types::ItemKind::Impl),
            Self::Static => quote::quote!(erised::types::ItemKind::Static),
            Self::ForeignType => quote::quote!(erised::types::ItemKind::ForeignType),
            Self::Macro => quote::quote!(erised::types::ItemKind::Macro),
            Self::ProcAttribute => quote::quote!(erised::types::ItemKind::ProcAttribute),
            Self::ProcDerive => quote::quote!(erised::types::ItemKind::ProcDerive),
            Self::AssocConst => quote::quote!(erised::types::ItemKind::AssocConst),
            Self::AssocType => quote::quote!(erised::types::ItemKind::AssocType),
            Self::Primitive => quote::quote!(erised::types::ItemKind::Primitive),
            Self::Keyword => quote::quote!(erised::types::ItemKind::Keyword),
        }
    }
}
impl quote::ToTokens for ItemKind {
    fn to_tokens(&self, tokens: &mut proc_macro2::TokenStream) {
        use quote::TokenStreamExt;
        let mut paths = erised::destruct::ItemsPaths::default();
        tokens.append_all(erised::destruct::ToTokens::to_tokens(self, &mut paths))
    }
}
impl erised::destruct::ToTokens for Item {
    fn to_tokens(&self, paths: &mut erised::destruct::ItemsPaths) -> proc_macro2::TokenStream {
        match &self {
            Self::Module(_0) => {
                let _0 = erised::destruct::ToTokens::to_tokens(_0, paths);
                quote :: quote ! (erised :: types :: Item :: Module (#_0 ,))
            }
            Self::ExternCrate { meta, name, rename } => {
                let meta = erised::destruct::ToTokens::to_tokens(meta, paths);
                let name = erised::destruct::ToTokens::to_tokens(name, paths);
                let rename = erised::destruct::ToTokens::to_tokens(rename, paths);
                quote :: quote ! (erised :: types :: Item :: ExternCrate { meta : #meta , name : #name , rename : #rename , })
            }
            Self::Import(_0) => {
                let _0 = erised::destruct::ToTokens::to_tokens(_0, paths);
                quote :: quote ! (erised :: types :: Item :: Import (#_0 ,))
            }
            Self::Union(_0) => {
                let _0 = erised::destruct::ToTokens::to_tokens(_0, paths);
                quote :: quote ! (erised :: types :: Item :: Union (#_0 ,))
            }
            Self::Struct(_0) => {
                let _0 = erised::destruct::ToTokens::to_tokens(_0, paths);
                quote :: quote ! (erised :: types :: Item :: Struct (#_0 ,))
            }
            Self::Enum(_0) => {
                let _0 = erised::destruct::ToTokens::to_tokens(_0, paths);
                quote :: quote ! (erised :: types :: Item :: Enum (#_0 ,))
            }
            Self::Function(_0) => {
                let _0 = erised::destruct::ToTokens::to_tokens(_0, paths);
                quote :: quote ! (erised :: types :: Item :: Function (#_0 ,))
            }
            Self::Trait(_0) => {
                let _0 = erised::destruct::ToTokens::to_tokens(_0, paths);
                quote :: quote ! (erised :: types :: Item :: Trait (#_0 ,))
            }
            Self::TraitAlias(_0) => {
                let _0 = erised::destruct::ToTokens::to_tokens(_0, paths);
                quote :: quote ! (erised :: types :: Item :: TraitAlias (#_0 ,))
            }
            Self::Impl(_0) => {
                let _0 = erised::destruct::ToTokens::to_tokens(_0, paths);
                quote :: quote ! (erised :: types :: Item :: Impl (#_0 ,))
            }
            Self::Typedef(_0) => {
                let _0 = erised::destruct::ToTokens::to_tokens(_0, paths);
                quote :: quote ! (erised :: types :: Item :: Typedef (#_0 ,))
            }
            Self::OpaqueTy(_0) => {
                let _0 = erised::destruct::ToTokens::to_tokens(_0, paths);
                quote :: quote ! (erised :: types :: Item :: OpaqueTy (#_0 ,))
            }
            Self::Constant(_0) => {
                let _0 = erised::destruct::ToTokens::to_tokens(_0, paths);
                quote :: quote ! (erised :: types :: Item :: Constant (#_0 ,))
            }
            Self::Static(_0) => {
                let _0 = erised::destruct::ToTokens::to_tokens(_0, paths);
                quote :: quote ! (erised :: types :: Item :: Static (#_0 ,))
            }
            Self::ForeignType => quote::quote!(erised::types::Item::ForeignType),
            Self::Macro { name, meta, expr } => {
                let name = erised::destruct::ToTokens::to_tokens(name, paths);
                let meta = erised::destruct::ToTokens::to_tokens(meta, paths);
                let expr = erised::destruct::ToTokens::to_tokens(expr, paths);
                quote :: quote ! (erised :: types :: Item :: Macro { name : #name , meta : #meta , expr : #expr , })
            }
            Self::ProcMacro(_0) => {
                let _0 = erised::destruct::ToTokens::to_tokens(_0, paths);
                quote :: quote ! (erised :: types :: Item :: ProcMacro (#_0 ,))
            }
            Self::Primitive(_0) => {
                let _0 = erised::destruct::ToTokens::to_tokens(_0, paths);
                quote :: quote ! (erised :: types :: Item :: Primitive (#_0 ,))
            }
            Self::AssocConst {
                meta,
                type_,
                default,
            } => {
                let meta = erised::destruct::ToTokens::to_tokens(meta, paths);
                let type_ = erised::destruct::ToTokens::to_tokens(type_, paths);
                let default = erised::destruct::ToTokens::to_tokens(default, paths);
                quote :: quote ! (erised :: types :: Item :: AssocConst { meta : #meta , type_ : #type_ , default : #default , })
            }
            Self::AssocType {
                meta,
                generics,
                bounds,
                default,
            } => {
                let meta = erised::destruct::ToTokens::to_tokens(meta, paths);
                let generics = erised::destruct::ToTokens::to_tokens(generics, paths);
                let bounds = erised::destruct::ToTokens::to_tokens(bounds, paths);
                let default = erised::destruct::ToTokens::to_tokens(default, paths);
                quote :: quote ! (erised :: types :: Item :: AssocType { meta : #meta , generics : #generics , bounds : #bounds , default : #default , })
            }
        }
    }
}
impl quote::ToTokens for Item {
    fn to_tokens(&self, tokens: &mut proc_macro2::TokenStream) {
        use quote::TokenStreamExt;
        let mut paths = erised::destruct::ItemsPaths::default();
        tokens.append_all(erised::destruct::ToTokens::to_tokens(self, &mut paths))
    }
}
impl erised::destruct::ToTokens for Module {
    fn to_tokens(&self, paths: &mut erised::destruct::ItemsPaths) -> proc_macro2::TokenStream {
        let &Self {
            name,
            meta,
            is_crate,
            items,
            is_stripped,
            ..
        } = &self;
        let name = erised::destruct::ToTokens::to_tokens(name, paths);
        let meta = erised::destruct::ToTokens::to_tokens(meta, paths);
        let is_crate = erised::destruct::ToTokens::to_tokens(is_crate, paths);
        let items = erised::destruct::ToTokens::to_tokens(items, paths);
        let is_stripped = erised::destruct::ToTokens::to_tokens(is_stripped, paths);
        quote :: quote ! (erised :: types :: Module { name : #name , meta : #meta , is_crate : #is_crate , items : #items , is_stripped : #is_stripped , })
    }
}
impl quote::ToTokens for Module {
    fn to_tokens(&self, tokens: &mut proc_macro2::TokenStream) {
        use quote::TokenStreamExt;
        let mut paths = erised::destruct::ItemsPaths::default();
        tokens.append_all(erised::destruct::ToTokens::to_tokens(self, &mut paths))
    }
}
impl erised::destruct::ToTokens for Union {
    fn to_tokens(&self, paths: &mut erised::destruct::ItemsPaths) -> proc_macro2::TokenStream {
        let &Self {
            generics,
            fields_stripped,
            fields,
            impls,
            ..
        } = &self;
        let generics = erised::destruct::ToTokens::to_tokens(generics, paths);
        let fields_stripped = erised::destruct::ToTokens::to_tokens(fields_stripped, paths);
        let fields = erised::destruct::ToTokens::to_tokens(fields, paths);
        let impls = erised::destruct::ToTokens::to_tokens(impls, paths);
        quote :: quote ! (erised :: types :: Union { generics : #generics , fields_stripped : #fields_stripped , fields : #fields , impls : #impls , })
    }
}
impl quote::ToTokens for Union {
    fn to_tokens(&self, tokens: &mut proc_macro2::TokenStream) {
        use quote::TokenStreamExt;
        let mut paths = erised::destruct::ItemsPaths::default();
        tokens.append_all(erised::destruct::ToTokens::to_tokens(self, &mut paths))
    }
}
impl erised::destruct::ToTokens for Struct {
    fn to_tokens(&self, paths: &mut erised::destruct::ItemsPaths) -> proc_macro2::TokenStream {
        let &Self {
            name,
            meta,
            kind,
            generics,
            impls,
            ..
        } = &self;
        let name = erised::destruct::ToTokens::to_tokens(name, paths);
        let meta = erised::destruct::ToTokens::to_tokens(meta, paths);
        let kind = erised::destruct::ToTokens::to_tokens(kind, paths);
        let generics = erised::destruct::ToTokens::to_tokens(generics, paths);
        let impls = erised::destruct::ToTokens::to_tokens(impls, paths);
        quote :: quote ! (erised :: types :: Struct { name : #name , meta : #meta , kind : #kind , generics : #generics , impls : #impls , })
    }
}
impl quote::ToTokens for Struct {
    fn to_tokens(&self, tokens: &mut proc_macro2::TokenStream) {
        use quote::TokenStreamExt;
        let mut paths = erised::destruct::ItemsPaths::default();
        tokens.append_all(erised::destruct::ToTokens::to_tokens(self, &mut paths))
    }
}
impl erised::destruct::ToTokens for StructKind {
    fn to_tokens(&self, paths: &mut erised::destruct::ItemsPaths) -> proc_macro2::TokenStream {
        match &self {
            Self::Unit => quote::quote!(erised::types::StructKind::Unit),
            Self::Tuple(_0) => {
                let _0 = erised::destruct::ToTokens::to_tokens(_0, paths);
                quote :: quote ! (erised :: types :: StructKind :: Tuple (#_0 ,))
            }
            Self::Plain {
                fields,
                fields_stripped,
            } => {
                let fields = erised::destruct::ToTokens::to_tokens(fields, paths);
                let fields_stripped = erised::destruct::ToTokens::to_tokens(fields_stripped, paths);
                quote :: quote ! (erised :: types :: StructKind :: Plain { fields : #fields , fields_stripped : #fields_stripped , })
            }
        }
    }
}
impl quote::ToTokens for StructKind {
    fn to_tokens(&self, tokens: &mut proc_macro2::TokenStream) {
        use quote::TokenStreamExt;
        let mut paths = erised::destruct::ItemsPaths::default();
        tokens.append_all(erised::destruct::ToTokens::to_tokens(self, &mut paths))
    }
}
impl erised::destruct::ToTokens for StructField {
    fn to_tokens(&self, paths: &mut erised::destruct::ItemsPaths) -> proc_macro2::TokenStream {
        let &Self {
            name,
            is_part_of_tuple,
            meta,
            ty,
            ..
        } = &self;
        let name = erised::destruct::ToTokens::to_tokens(name, paths);
        let is_part_of_tuple = erised::destruct::ToTokens::to_tokens(is_part_of_tuple, paths);
        let meta = erised::destruct::ToTokens::to_tokens(meta, paths);
        let ty = erised::destruct::ToTokens::to_tokens(ty, paths);
        quote :: quote ! (erised :: types :: StructField { name : #name , is_part_of_tuple : #is_part_of_tuple , meta : #meta , ty : #ty , })
    }
}
impl quote::ToTokens for StructField {
    fn to_tokens(&self, tokens: &mut proc_macro2::TokenStream) {
        use quote::TokenStreamExt;
        let mut paths = erised::destruct::ItemsPaths::default();
        tokens.append_all(erised::destruct::ToTokens::to_tokens(self, &mut paths))
    }
}
impl erised::destruct::ToTokens for Enum {
    fn to_tokens(&self, paths: &mut erised::destruct::ItemsPaths) -> proc_macro2::TokenStream {
        let &Self {
            name,
            meta,
            generics,
            variants_stripped,
            variants,
            impls,
            ..
        } = &self;
        let name = erised::destruct::ToTokens::to_tokens(name, paths);
        let meta = erised::destruct::ToTokens::to_tokens(meta, paths);
        let generics = erised::destruct::ToTokens::to_tokens(generics, paths);
        let variants_stripped = erised::destruct::ToTokens::to_tokens(variants_stripped, paths);
        let variants = erised::destruct::ToTokens::to_tokens(variants, paths);
        let impls = erised::destruct::ToTokens::to_tokens(impls, paths);
        quote :: quote ! (erised :: types :: Enum { name : #name , meta : #meta , generics : #generics , variants_stripped : #variants_stripped , variants : #variants , impls : #impls , })
    }
}
impl quote::ToTokens for Enum {
    fn to_tokens(&self, tokens: &mut proc_macro2::TokenStream) {
        use quote::TokenStreamExt;
        let mut paths = erised::destruct::ItemsPaths::default();
        tokens.append_all(erised::destruct::ToTokens::to_tokens(self, &mut paths))
    }
}
impl erised::destruct::ToTokens for Variant {
    fn to_tokens(&self, paths: &mut erised::destruct::ItemsPaths) -> proc_macro2::TokenStream {
        let &Self {
            name,
            meta,
            kind,
            discriminant,
            ..
        } = &self;
        let name = erised::destruct::ToTokens::to_tokens(name, paths);
        let meta = erised::destruct::ToTokens::to_tokens(meta, paths);
        let kind = erised::destruct::ToTokens::to_tokens(kind, paths);
        let discriminant = erised::destruct::ToTokens::to_tokens(discriminant, paths);
        quote :: quote ! (erised :: types :: Variant { name : #name , meta : #meta , kind : #kind , discriminant : #discriminant , })
    }
}
impl quote::ToTokens for Variant {
    fn to_tokens(&self, tokens: &mut proc_macro2::TokenStream) {
        use quote::TokenStreamExt;
        let mut paths = erised::destruct::ItemsPaths::default();
        tokens.append_all(erised::destruct::ToTokens::to_tokens(self, &mut paths))
    }
}
impl erised::destruct::ToTokens for VariantKind {
    fn to_tokens(&self, paths: &mut erised::destruct::ItemsPaths) -> proc_macro2::TokenStream {
        match &self {
            Self::Plain => quote::quote!(erised::types::VariantKind::Plain),
            Self::Tuple(_0) => {
                let _0 = erised::destruct::ToTokens::to_tokens(_0, paths);
                quote :: quote ! (erised :: types :: VariantKind :: Tuple (#_0 ,))
            }
            Self::Struct {
                fields,
                fields_stripped,
            } => {
                let fields = erised::destruct::ToTokens::to_tokens(fields, paths);
                let fields_stripped = erised::destruct::ToTokens::to_tokens(fields_stripped, paths);
                quote :: quote ! (erised :: types :: VariantKind :: Struct { fields : #fields , fields_stripped : #fields_stripped , })
            }
        }
    }
}
impl quote::ToTokens for VariantKind {
    fn to_tokens(&self, tokens: &mut proc_macro2::TokenStream) {
        use quote::TokenStreamExt;
        let mut paths = erised::destruct::ItemsPaths::default();
        tokens.append_all(erised::destruct::ToTokens::to_tokens(self, &mut paths))
    }
}
impl erised::destruct::ToTokens for Discriminant {
    fn to_tokens(&self, paths: &mut erised::destruct::ItemsPaths) -> proc_macro2::TokenStream {
        let &Self { expr, value, .. } = &self;
        let expr = erised::destruct::ToTokens::to_tokens(expr, paths);
        let value = erised::destruct::ToTokens::to_tokens(value, paths);
        quote :: quote ! (erised :: types :: Discriminant { expr : #expr , value : #value , })
    }
}
impl quote::ToTokens for Discriminant {
    fn to_tokens(&self, tokens: &mut proc_macro2::TokenStream) {
        use quote::TokenStreamExt;
        let mut paths = erised::destruct::ItemsPaths::default();
        tokens.append_all(erised::destruct::ToTokens::to_tokens(self, &mut paths))
    }
}
impl erised::destruct::ToTokens for Header {
    fn to_tokens(&self, paths: &mut erised::destruct::ItemsPaths) -> proc_macro2::TokenStream {
        let &Self {
            const_,
            unsafe_,
            async_,
            abi,
            ..
        } = &self;
        let const_ = erised::destruct::ToTokens::to_tokens(const_, paths);
        let unsafe_ = erised::destruct::ToTokens::to_tokens(unsafe_, paths);
        let async_ = erised::destruct::ToTokens::to_tokens(async_, paths);
        let abi = erised::destruct::ToTokens::to_tokens(abi, paths);
        quote :: quote ! (erised :: types :: Header { const_ : #const_ , unsafe_ : #unsafe_ , async_ : #async_ , abi : #abi , })
    }
}
impl quote::ToTokens for Header {
    fn to_tokens(&self, tokens: &mut proc_macro2::TokenStream) {
        use quote::TokenStreamExt;
        let mut paths = erised::destruct::ItemsPaths::default();
        tokens.append_all(erised::destruct::ToTokens::to_tokens(self, &mut paths))
    }
}
impl erised::destruct::ToTokens for Abi {
    fn to_tokens(&self, paths: &mut erised::destruct::ItemsPaths) -> proc_macro2::TokenStream {
        match &self {
            Self::Rust => quote::quote!(erised::types::Abi::Rust),
            Self::C { unwind } => {
                let unwind = erised::destruct::ToTokens::to_tokens(unwind, paths);
                quote :: quote ! (erised :: types :: Abi :: C { unwind : #unwind , })
            }
            Self::Cdecl { unwind } => {
                let unwind = erised::destruct::ToTokens::to_tokens(unwind, paths);
                quote :: quote ! (erised :: types :: Abi :: Cdecl { unwind : #unwind , })
            }
            Self::Stdcall { unwind } => {
                let unwind = erised::destruct::ToTokens::to_tokens(unwind, paths);
                quote :: quote ! (erised :: types :: Abi :: Stdcall { unwind : #unwind , })
            }
            Self::Fastcall { unwind } => {
                let unwind = erised::destruct::ToTokens::to_tokens(unwind, paths);
                quote :: quote ! (erised :: types :: Abi :: Fastcall { unwind : #unwind , })
            }
            Self::Aapcs { unwind } => {
                let unwind = erised::destruct::ToTokens::to_tokens(unwind, paths);
                quote :: quote ! (erised :: types :: Abi :: Aapcs { unwind : #unwind , })
            }
            Self::Win64 { unwind } => {
                let unwind = erised::destruct::ToTokens::to_tokens(unwind, paths);
                quote :: quote ! (erised :: types :: Abi :: Win64 { unwind : #unwind , })
            }
            Self::SysV64 { unwind } => {
                let unwind = erised::destruct::ToTokens::to_tokens(unwind, paths);
                quote :: quote ! (erised :: types :: Abi :: SysV64 { unwind : #unwind , })
            }
            Self::System { unwind } => {
                let unwind = erised::destruct::ToTokens::to_tokens(unwind, paths);
                quote :: quote ! (erised :: types :: Abi :: System { unwind : #unwind , })
            }
            Self::Other(_0) => {
                let _0 = erised::destruct::ToTokens::to_tokens(_0, paths);
                quote :: quote ! (erised :: types :: Abi :: Other (#_0 ,))
            }
        }
    }
}
impl quote::ToTokens for Abi {
    fn to_tokens(&self, tokens: &mut proc_macro2::TokenStream) {
        use quote::TokenStreamExt;
        let mut paths = erised::destruct::ItemsPaths::default();
        tokens.append_all(erised::destruct::ToTokens::to_tokens(self, &mut paths))
    }
}
impl erised::destruct::ToTokens for Function {
    fn to_tokens(&self, paths: &mut erised::destruct::ItemsPaths) -> proc_macro2::TokenStream {
        let &Self {
            name,
            meta,
            decl,
            generics,
            header,
            has_body,
            ..
        } = &self;
        let name = erised::destruct::ToTokens::to_tokens(name, paths);
        let meta = erised::destruct::ToTokens::to_tokens(meta, paths);
        let decl = erised::destruct::ToTokens::to_tokens(decl, paths);
        let generics = erised::destruct::ToTokens::to_tokens(generics, paths);
        let header = erised::destruct::ToTokens::to_tokens(header, paths);
        let has_body = erised::destruct::ToTokens::to_tokens(has_body, paths);
        quote :: quote ! (erised :: types :: Function { name : #name , meta : #meta , decl : #decl , generics : #generics , header : #header , has_body : #has_body , })
    }
}
impl quote::ToTokens for Function {
    fn to_tokens(&self, tokens: &mut proc_macro2::TokenStream) {
        use quote::TokenStreamExt;
        let mut paths = erised::destruct::ItemsPaths::default();
        tokens.append_all(erised::destruct::ToTokens::to_tokens(self, &mut paths))
    }
}
impl erised::destruct::ToTokens for Generics {
    fn to_tokens(&self, paths: &mut erised::destruct::ItemsPaths) -> proc_macro2::TokenStream {
        let &Self {
            params,
            where_predicates,
            ..
        } = &self;
        let params = erised::destruct::ToTokens::to_tokens(params, paths);
        let where_predicates = erised::destruct::ToTokens::to_tokens(where_predicates, paths);
        quote :: quote ! (erised :: types :: Generics { params : #params , where_predicates : #where_predicates , })
    }
}
impl quote::ToTokens for Generics {
    fn to_tokens(&self, tokens: &mut proc_macro2::TokenStream) {
        use quote::TokenStreamExt;
        let mut paths = erised::destruct::ItemsPaths::default();
        tokens.append_all(erised::destruct::ToTokens::to_tokens(self, &mut paths))
    }
}
impl erised::destruct::ToTokens for GenericParamDef {
    fn to_tokens(&self, paths: &mut erised::destruct::ItemsPaths) -> proc_macro2::TokenStream {
        let &Self { name, kind, .. } = &self;
        let name = erised::destruct::ToTokens::to_tokens(name, paths);
        let kind = erised::destruct::ToTokens::to_tokens(kind, paths);
        quote :: quote ! (erised :: types :: GenericParamDef { name : #name , kind : #kind , })
    }
}
impl quote::ToTokens for GenericParamDef {
    fn to_tokens(&self, tokens: &mut proc_macro2::TokenStream) {
        use quote::TokenStreamExt;
        let mut paths = erised::destruct::ItemsPaths::default();
        tokens.append_all(erised::destruct::ToTokens::to_tokens(self, &mut paths))
    }
}
impl erised::destruct::ToTokens for GenericParamDefKind {
    fn to_tokens(&self, paths: &mut erised::destruct::ItemsPaths) -> proc_macro2::TokenStream {
        match &self {
            Self::Lifetime { outlives } => {
                let outlives = erised::destruct::ToTokens::to_tokens(outlives, paths);
                quote :: quote ! (erised :: types :: GenericParamDefKind :: Lifetime { outlives : #outlives , })
            }
            Self::Type {
                bounds,
                default,
                synthetic,
            } => {
                let bounds = erised::destruct::ToTokens::to_tokens(bounds, paths);
                let default = erised::destruct::ToTokens::to_tokens(default, paths);
                let synthetic = erised::destruct::ToTokens::to_tokens(synthetic, paths);
                quote :: quote ! (erised :: types :: GenericParamDefKind :: Type { bounds : #bounds , default : #default , synthetic : #synthetic , })
            }
            Self::Const { type_, default } => {
                let type_ = erised::destruct::ToTokens::to_tokens(type_, paths);
                let default = erised::destruct::ToTokens::to_tokens(default, paths);
                quote :: quote ! (erised :: types :: GenericParamDefKind :: Const { type_ : #type_ , default : #default , })
            }
        }
    }
}
impl quote::ToTokens for GenericParamDefKind {
    fn to_tokens(&self, tokens: &mut proc_macro2::TokenStream) {
        use quote::TokenStreamExt;
        let mut paths = erised::destruct::ItemsPaths::default();
        tokens.append_all(erised::destruct::ToTokens::to_tokens(self, &mut paths))
    }
}
impl erised::destruct::ToTokens for WherePredicate {
    fn to_tokens(&self, paths: &mut erised::destruct::ItemsPaths) -> proc_macro2::TokenStream {
        match &self {
            Self::BoundPredicate {
                type_,
                bounds,
                generic_params,
            } => {
                let type_ = erised::destruct::ToTokens::to_tokens(type_, paths);
                let bounds = erised::destruct::ToTokens::to_tokens(bounds, paths);
                let generic_params = erised::destruct::ToTokens::to_tokens(generic_params, paths);
                quote :: quote ! (erised :: types :: WherePredicate :: BoundPredicate { type_ : #type_ , bounds : #bounds , generic_params : #generic_params , })
            }
            Self::RegionPredicate { lifetime, bounds } => {
                let lifetime = erised::destruct::ToTokens::to_tokens(lifetime, paths);
                let bounds = erised::destruct::ToTokens::to_tokens(bounds, paths);
                quote :: quote ! (erised :: types :: WherePredicate :: RegionPredicate { lifetime : #lifetime , bounds : #bounds , })
            }
            Self::EqPredicate { lhs, rhs } => {
                let lhs = erised::destruct::ToTokens::to_tokens(lhs, paths);
                let rhs = erised::destruct::ToTokens::to_tokens(rhs, paths);
                quote :: quote ! (erised :: types :: WherePredicate :: EqPredicate { lhs : #lhs , rhs : #rhs , })
            }
        }
    }
}
impl quote::ToTokens for WherePredicate {
    fn to_tokens(&self, tokens: &mut proc_macro2::TokenStream) {
        use quote::TokenStreamExt;
        let mut paths = erised::destruct::ItemsPaths::default();
        tokens.append_all(erised::destruct::ToTokens::to_tokens(self, &mut paths))
    }
}
impl erised::destruct::ToTokens for GenericBound {
    fn to_tokens(&self, paths: &mut erised::destruct::ItemsPaths) -> proc_macro2::TokenStream {
        match &self {
            Self::TraitBound {
                trait_,
                generic_params,
                modifier,
            } => {
                let trait_ = erised::destruct::ToTokens::to_tokens(trait_, paths);
                let generic_params = erised::destruct::ToTokens::to_tokens(generic_params, paths);
                let modifier = erised::destruct::ToTokens::to_tokens(modifier, paths);
                quote :: quote ! (erised :: types :: GenericBound :: TraitBound { trait_ : #trait_ , generic_params : #generic_params , modifier : #modifier , })
            }
            Self::Outlives(_0) => {
                let _0 = erised::destruct::ToTokens::to_tokens(_0, paths);
                quote :: quote ! (erised :: types :: GenericBound :: Outlives (#_0 ,))
            }
        }
    }
}
impl quote::ToTokens for GenericBound {
    fn to_tokens(&self, tokens: &mut proc_macro2::TokenStream) {
        use quote::TokenStreamExt;
        let mut paths = erised::destruct::ItemsPaths::default();
        tokens.append_all(erised::destruct::ToTokens::to_tokens(self, &mut paths))
    }
}
impl erised::destruct::ToTokens for TraitBoundModifier {
    fn to_tokens(&self, paths: &mut erised::destruct::ItemsPaths) -> proc_macro2::TokenStream {
        match &self {
            Self::None => quote::quote!(erised::types::TraitBoundModifier::None),
            Self::Maybe => quote::quote!(erised::types::TraitBoundModifier::Maybe),
            Self::MaybeConst => quote::quote!(erised::types::TraitBoundModifier::MaybeConst),
        }
    }
}
impl quote::ToTokens for TraitBoundModifier {
    fn to_tokens(&self, tokens: &mut proc_macro2::TokenStream) {
        use quote::TokenStreamExt;
        let mut paths = erised::destruct::ItemsPaths::default();
        tokens.append_all(erised::destruct::ToTokens::to_tokens(self, &mut paths))
    }
}
impl erised::destruct::ToTokens for Term {
    fn to_tokens(&self, paths: &mut erised::destruct::ItemsPaths) -> proc_macro2::TokenStream {
        match &self {
            Self::Type(_0) => {
                let _0 = erised::destruct::ToTokens::to_tokens(_0, paths);
                quote :: quote ! (erised :: types :: Term :: Type (#_0 ,))
            }
            Self::Constant(_0) => {
                let _0 = erised::destruct::ToTokens::to_tokens(_0, paths);
                quote :: quote ! (erised :: types :: Term :: Constant (#_0 ,))
            }
        }
    }
}
impl quote::ToTokens for Term {
    fn to_tokens(&self, tokens: &mut proc_macro2::TokenStream) {
        use quote::TokenStreamExt;
        let mut paths = erised::destruct::ItemsPaths::default();
        tokens.append_all(erised::destruct::ToTokens::to_tokens(self, &mut paths))
    }
}
impl erised::destruct::ToTokens for Type {
    fn to_tokens(&self, paths: &mut erised::destruct::ItemsPaths) -> proc_macro2::TokenStream {
        match &self {
            Self::ResolvedPath(_0) => {
                let _0 = erised::destruct::ToTokens::to_tokens(_0, paths);
                quote :: quote ! (erised :: types :: Type :: ResolvedPath (#_0 ,))
            }
            Self::DynTrait(_0) => {
                let _0 = erised::destruct::ToTokens::to_tokens(_0, paths);
                quote :: quote ! (erised :: types :: Type :: DynTrait (#_0 ,))
            }
            Self::Generic(_0) => {
                let _0 = erised::destruct::ToTokens::to_tokens(_0, paths);
                quote :: quote ! (erised :: types :: Type :: Generic (#_0 ,))
            }
            Self::Primitive(_0) => {
                let _0 = erised::destruct::ToTokens::to_tokens(_0, paths);
                quote :: quote ! (erised :: types :: Type :: Primitive (#_0 ,))
            }
            Self::FunctionPointer(_0) => {
                let _0 = erised::destruct::ToTokens::to_tokens(_0, paths);
                quote :: quote ! (erised :: types :: Type :: FunctionPointer (#_0 ,))
            }
            Self::Tuple(_0) => {
                let _0 = erised::destruct::ToTokens::to_tokens(_0, paths);
                quote :: quote ! (erised :: types :: Type :: Tuple (#_0 ,))
            }
            Self::Slice(_0) => {
                let _0 = erised::destruct::ToTokens::to_tokens(_0, paths);
                quote :: quote ! (erised :: types :: Type :: Slice (#_0 ,))
            }
            Self::Array { type_, len } => {
                let type_ = erised::destruct::ToTokens::to_tokens(type_, paths);
                let len = erised::destruct::ToTokens::to_tokens(len, paths);
                quote :: quote ! (erised :: types :: Type :: Array { type_ : #type_ , len : #len , })
            }
            Self::ImplTrait(_0) => {
                let _0 = erised::destruct::ToTokens::to_tokens(_0, paths);
                quote :: quote ! (erised :: types :: Type :: ImplTrait (#_0 ,))
            }
            Self::Infer => quote::quote!(erised::types::Type::Infer),
            Self::RawPointer { mutable, type_ } => {
                let mutable = erised::destruct::ToTokens::to_tokens(mutable, paths);
                let type_ = erised::destruct::ToTokens::to_tokens(type_, paths);
                quote :: quote ! (erised :: types :: Type :: RawPointer { mutable : #mutable , type_ : #type_ , })
            }
            Self::BorrowedRef {
                lifetime,
                mutable,
                type_,
            } => {
                let lifetime = erised::destruct::ToTokens::to_tokens(lifetime, paths);
                let mutable = erised::destruct::ToTokens::to_tokens(mutable, paths);
                let type_ = erised::destruct::ToTokens::to_tokens(type_, paths);
                quote :: quote ! (erised :: types :: Type :: BorrowedRef { lifetime : #lifetime , mutable : #mutable , type_ : #type_ , })
            }
            Self::QualifiedPath {
                name,
                args,
                self_type,
                trait_,
            } => {
                let name = erised::destruct::ToTokens::to_tokens(name, paths);
                let args = erised::destruct::ToTokens::to_tokens(args, paths);
                let self_type = erised::destruct::ToTokens::to_tokens(self_type, paths);
                let trait_ = erised::destruct::ToTokens::to_tokens(trait_, paths);
                quote :: quote ! (erised :: types :: Type :: QualifiedPath { name : #name , args : #args , self_type : #self_type , trait_ : #trait_ , })
            }
        }
    }
}
impl quote::ToTokens for Type {
    fn to_tokens(&self, tokens: &mut proc_macro2::TokenStream) {
        use quote::TokenStreamExt;
        let mut paths = erised::destruct::ItemsPaths::default();
        tokens.append_all(erised::destruct::ToTokens::to_tokens(self, &mut paths))
    }
}
impl erised::destruct::ToTokens for Path {
    fn to_tokens(&self, paths: &mut erised::destruct::ItemsPaths) -> proc_macro2::TokenStream {
        let &Self {
            name,
            prefix,
            target,
            args,
            ..
        } = &self;
        let name = erised::destruct::ToTokens::to_tokens(name, paths);
        let prefix = erised::destruct::ToTokens::to_tokens(prefix, paths);
        let target = erised::destruct::ToTokens::to_tokens(target, paths);
        let args = erised::destruct::ToTokens::to_tokens(args, paths);
        quote :: quote ! (erised :: types :: Path { name : #name , prefix : #prefix , target : #target , args : #args , })
    }
}
impl quote::ToTokens for Path {
    fn to_tokens(&self, tokens: &mut proc_macro2::TokenStream) {
        use quote::TokenStreamExt;
        let mut paths = erised::destruct::ItemsPaths::default();
        tokens.append_all(erised::destruct::ToTokens::to_tokens(self, &mut paths))
    }
}
impl erised::destruct::ToTokens for FunctionPointer {
    fn to_tokens(&self, paths: &mut erised::destruct::ItemsPaths) -> proc_macro2::TokenStream {
        let &Self {
            decl,
            generic_params,
            header,
            ..
        } = &self;
        let decl = erised::destruct::ToTokens::to_tokens(decl, paths);
        let generic_params = erised::destruct::ToTokens::to_tokens(generic_params, paths);
        let header = erised::destruct::ToTokens::to_tokens(header, paths);
        quote :: quote ! (erised :: types :: FunctionPointer { decl : #decl , generic_params : #generic_params , header : #header , })
    }
}
impl quote::ToTokens for FunctionPointer {
    fn to_tokens(&self, tokens: &mut proc_macro2::TokenStream) {
        use quote::TokenStreamExt;
        let mut paths = erised::destruct::ItemsPaths::default();
        tokens.append_all(erised::destruct::ToTokens::to_tokens(self, &mut paths))
    }
}
impl erised::destruct::ToTokens for FnDecl {
    fn to_tokens(&self, paths: &mut erised::destruct::ItemsPaths) -> proc_macro2::TokenStream {
        let &Self {
            inputs,
            output,
            c_variadic,
            ..
        } = &self;
        let inputs = erised::destruct::ToTokens::to_tokens(inputs, paths);
        let output = erised::destruct::ToTokens::to_tokens(output, paths);
        let c_variadic = erised::destruct::ToTokens::to_tokens(c_variadic, paths);
        quote :: quote ! (erised :: types :: FnDecl { inputs : #inputs , output : #output , c_variadic : #c_variadic , })
    }
}
impl quote::ToTokens for FnDecl {
    fn to_tokens(&self, tokens: &mut proc_macro2::TokenStream) {
        use quote::TokenStreamExt;
        let mut paths = erised::destruct::ItemsPaths::default();
        tokens.append_all(erised::destruct::ToTokens::to_tokens(self, &mut paths))
    }
}
impl erised::destruct::ToTokens for FnInput {
    fn to_tokens(&self, paths: &mut erised::destruct::ItemsPaths) -> proc_macro2::TokenStream {
        let &Self { pat, ty, .. } = &self;
        let pat = erised::destruct::ToTokens::to_tokens(pat, paths);
        let ty = erised::destruct::ToTokens::to_tokens(ty, paths);
        quote :: quote ! (erised :: types :: FnInput { pat : #pat , ty : #ty , })
    }
}
impl quote::ToTokens for FnInput {
    fn to_tokens(&self, tokens: &mut proc_macro2::TokenStream) {
        use quote::TokenStreamExt;
        let mut paths = erised::destruct::ItemsPaths::default();
        tokens.append_all(erised::destruct::ToTokens::to_tokens(self, &mut paths))
    }
}
impl erised::destruct::ToTokens for Trait {
    fn to_tokens(&self, paths: &mut erised::destruct::ItemsPaths) -> proc_macro2::TokenStream {
        let &Self {
            name,
            meta,
            is_auto,
            is_unsafe,
            items,
            generics,
            bounds,
            implementations,
            ..
        } = &self;
        let name = erised::destruct::ToTokens::to_tokens(name, paths);
        let meta = erised::destruct::ToTokens::to_tokens(meta, paths);
        let is_auto = erised::destruct::ToTokens::to_tokens(is_auto, paths);
        let is_unsafe = erised::destruct::ToTokens::to_tokens(is_unsafe, paths);
        let items = erised::destruct::ToTokens::to_tokens(items, paths);
        let generics = erised::destruct::ToTokens::to_tokens(generics, paths);
        let bounds = erised::destruct::ToTokens::to_tokens(bounds, paths);
        let implementations = erised::destruct::ToTokens::to_tokens(implementations, paths);
        quote :: quote ! (erised :: types :: Trait { name : #name , meta : #meta , is_auto : #is_auto , is_unsafe : #is_unsafe , items : #items , generics : #generics , bounds : #bounds , implementations : #implementations , })
    }
}
impl quote::ToTokens for Trait {
    fn to_tokens(&self, tokens: &mut proc_macro2::TokenStream) {
        use quote::TokenStreamExt;
        let mut paths = erised::destruct::ItemsPaths::default();
        tokens.append_all(erised::destruct::ToTokens::to_tokens(self, &mut paths))
    }
}
impl erised::destruct::ToTokens for TraitAlias {
    fn to_tokens(&self, paths: &mut erised::destruct::ItemsPaths) -> proc_macro2::TokenStream {
        let &Self {
            generics, params, ..
        } = &self;
        let generics = erised::destruct::ToTokens::to_tokens(generics, paths);
        let params = erised::destruct::ToTokens::to_tokens(params, paths);
        quote :: quote ! (erised :: types :: TraitAlias { generics : #generics , params : #params , })
    }
}
impl quote::ToTokens for TraitAlias {
    fn to_tokens(&self, tokens: &mut proc_macro2::TokenStream) {
        use quote::TokenStreamExt;
        let mut paths = erised::destruct::ItemsPaths::default();
        tokens.append_all(erised::destruct::ToTokens::to_tokens(self, &mut paths))
    }
}
impl erised::destruct::ToTokens for Impl {
    fn to_tokens(&self, paths: &mut erised::destruct::ItemsPaths) -> proc_macro2::TokenStream {
        let &Self {
            meta,
            is_unsafe,
            generics,
            provided_trait_methods,
            trait_,
            for_,
            items,
            negative,
            synthetic,
            blanket_impl,
            ..
        } = &self;
        let meta = erised::destruct::ToTokens::to_tokens(meta, paths);
        let is_unsafe = erised::destruct::ToTokens::to_tokens(is_unsafe, paths);
        let generics = erised::destruct::ToTokens::to_tokens(generics, paths);
        let provided_trait_methods =
            erised::destruct::ToTokens::to_tokens(provided_trait_methods, paths);
        let trait_ = erised::destruct::ToTokens::to_tokens(trait_, paths);
        let for_ = erised::destruct::ToTokens::to_tokens(for_, paths);
        let items = erised::destruct::ToTokens::to_tokens(items, paths);
        let negative = erised::destruct::ToTokens::to_tokens(negative, paths);
        let synthetic = erised::destruct::ToTokens::to_tokens(synthetic, paths);
        let blanket_impl = erised::destruct::ToTokens::to_tokens(blanket_impl, paths);
        quote :: quote ! (erised :: types :: Impl { meta : #meta , is_unsafe : #is_unsafe , generics : #generics , provided_trait_methods : #provided_trait_methods , trait_ : #trait_ , for_ : #for_ , items : #items , negative : #negative , synthetic : #synthetic , blanket_impl : #blanket_impl , })
    }
}
impl quote::ToTokens for Impl {
    fn to_tokens(&self, tokens: &mut proc_macro2::TokenStream) {
        use quote::TokenStreamExt;
        let mut paths = erised::destruct::ItemsPaths::default();
        tokens.append_all(erised::destruct::ToTokens::to_tokens(self, &mut paths))
    }
}
impl erised::destruct::ToTokens for Import {
    fn to_tokens(&self, paths: &mut erised::destruct::ItemsPaths) -> proc_macro2::TokenStream {
        let &Self {
            meta,
            source,
            name,
            target,
            glob,
            ..
        } = &self;
        let meta = erised::destruct::ToTokens::to_tokens(meta, paths);
        let source = erised::destruct::ToTokens::to_tokens(source, paths);
        let name = erised::destruct::ToTokens::to_tokens(name, paths);
        let target = erised::destruct::ToTokens::to_tokens(target, paths);
        let glob = erised::destruct::ToTokens::to_tokens(glob, paths);
        quote :: quote ! (erised :: types :: Import { meta : #meta , source : #source , name : #name , target : #target , glob : #glob , })
    }
}
impl quote::ToTokens for Import {
    fn to_tokens(&self, tokens: &mut proc_macro2::TokenStream) {
        use quote::TokenStreamExt;
        let mut paths = erised::destruct::ItemsPaths::default();
        tokens.append_all(erised::destruct::ToTokens::to_tokens(self, &mut paths))
    }
}
impl erised::destruct::ToTokens for ProcMacro {
    fn to_tokens(&self, paths: &mut erised::destruct::ItemsPaths) -> proc_macro2::TokenStream {
        let &Self { kind, helpers, .. } = &self;
        let kind = erised::destruct::ToTokens::to_tokens(kind, paths);
        let helpers = erised::destruct::ToTokens::to_tokens(helpers, paths);
        quote :: quote ! (erised :: types :: ProcMacro { kind : #kind , helpers : #helpers , })
    }
}
impl quote::ToTokens for ProcMacro {
    fn to_tokens(&self, tokens: &mut proc_macro2::TokenStream) {
        use quote::TokenStreamExt;
        let mut paths = erised::destruct::ItemsPaths::default();
        tokens.append_all(erised::destruct::ToTokens::to_tokens(self, &mut paths))
    }
}
impl erised::destruct::ToTokens for MacroKind {
    fn to_tokens(&self, paths: &mut erised::destruct::ItemsPaths) -> proc_macro2::TokenStream {
        match &self {
            Self::Bang => quote::quote!(erised::types::MacroKind::Bang),
            Self::Attr => quote::quote!(erised::types::MacroKind::Attr),
            Self::Derive => quote::quote!(erised::types::MacroKind::Derive),
        }
    }
}
impl quote::ToTokens for MacroKind {
    fn to_tokens(&self, tokens: &mut proc_macro2::TokenStream) {
        use quote::TokenStreamExt;
        let mut paths = erised::destruct::ItemsPaths::default();
        tokens.append_all(erised::destruct::ToTokens::to_tokens(self, &mut paths))
    }
}
impl erised::destruct::ToTokens for Typedef {
    fn to_tokens(&self, paths: &mut erised::destruct::ItemsPaths) -> proc_macro2::TokenStream {
        let &Self {
            name,
            meta,
            type_,
            generics,
            ..
        } = &self;
        let name = erised::destruct::ToTokens::to_tokens(name, paths);
        let meta = erised::destruct::ToTokens::to_tokens(meta, paths);
        let type_ = erised::destruct::ToTokens::to_tokens(type_, paths);
        let generics = erised::destruct::ToTokens::to_tokens(generics, paths);
        quote :: quote ! (erised :: types :: Typedef { name : #name , meta : #meta , type_ : #type_ , generics : #generics , })
    }
}
impl quote::ToTokens for Typedef {
    fn to_tokens(&self, tokens: &mut proc_macro2::TokenStream) {
        use quote::TokenStreamExt;
        let mut paths = erised::destruct::ItemsPaths::default();
        tokens.append_all(erised::destruct::ToTokens::to_tokens(self, &mut paths))
    }
}
impl erised::destruct::ToTokens for OpaqueTy {
    fn to_tokens(&self, paths: &mut erised::destruct::ItemsPaths) -> proc_macro2::TokenStream {
        let &Self {
            name,
            meta,
            bounds,
            generics,
            ..
        } = &self;
        let name = erised::destruct::ToTokens::to_tokens(name, paths);
        let meta = erised::destruct::ToTokens::to_tokens(meta, paths);
        let bounds = erised::destruct::ToTokens::to_tokens(bounds, paths);
        let generics = erised::destruct::ToTokens::to_tokens(generics, paths);
        quote :: quote ! (erised :: types :: OpaqueTy { name : #name , meta : #meta , bounds : #bounds , generics : #generics , })
    }
}
impl quote::ToTokens for OpaqueTy {
    fn to_tokens(&self, tokens: &mut proc_macro2::TokenStream) {
        use quote::TokenStreamExt;
        let mut paths = erised::destruct::ItemsPaths::default();
        tokens.append_all(erised::destruct::ToTokens::to_tokens(self, &mut paths))
    }
}
impl erised::destruct::ToTokens for Static {
    fn to_tokens(&self, paths: &mut erised::destruct::ItemsPaths) -> proc_macro2::TokenStream {
        let &Self {
            name,
            meta,
            type_,
            mutable,
            expr,
            ..
        } = &self;
        let name = erised::destruct::ToTokens::to_tokens(name, paths);
        let meta = erised::destruct::ToTokens::to_tokens(meta, paths);
        let type_ = erised::destruct::ToTokens::to_tokens(type_, paths);
        let mutable = erised::destruct::ToTokens::to_tokens(mutable, paths);
        let expr = erised::destruct::ToTokens::to_tokens(expr, paths);
        quote :: quote ! (erised :: types :: Static { name : #name , meta : #meta , type_ : #type_ , mutable : #mutable , expr : #expr , })
    }
}
impl quote::ToTokens for Static {
    fn to_tokens(&self, tokens: &mut proc_macro2::TokenStream) {
        use quote::TokenStreamExt;
        let mut paths = erised::destruct::ItemsPaths::default();
        tokens.append_all(erised::destruct::ToTokens::to_tokens(self, &mut paths))
    }
}
impl erised::destruct::ToTokens for Primitive {
    fn to_tokens(&self, paths: &mut erised::destruct::ItemsPaths) -> proc_macro2::TokenStream {
        let &Self { name, impls, .. } = &self;
        let name = erised::destruct::ToTokens::to_tokens(name, paths);
        let impls = erised::destruct::ToTokens::to_tokens(impls, paths);
        quote :: quote ! (erised :: types :: Primitive { name : #name , impls : #impls , })
    }
}
impl quote::ToTokens for Primitive {
    fn to_tokens(&self, tokens: &mut proc_macro2::TokenStream) {
        use quote::TokenStreamExt;
        let mut paths = erised::destruct::ItemsPaths::default();
        tokens.append_all(erised::destruct::ToTokens::to_tokens(self, &mut paths))
    }
}
impl ItemKind {
    pub fn as_module(self) -> Option<()> {
        match self {
            Self::Module => Some(()),
            _ => None,
        }
    }
    pub fn as_extern_crate(self) -> Option<()> {
        match self {
            Self::ExternCrate => Some(()),
            _ => None,
        }
    }
    pub fn as_import(self) -> Option<()> {
        match self {
            Self::Import => Some(()),
            _ => None,
        }
    }
    pub fn as_struct(self) -> Option<()> {
        match self {
            Self::Struct => Some(()),
            _ => None,
        }
    }
    pub fn as_struct_field(self) -> Option<()> {
        match self {
            Self::StructField => Some(()),
            _ => None,
        }
    }
    pub fn as_union(self) -> Option<()> {
        match self {
            Self::Union => Some(()),
            _ => None,
        }
    }
    pub fn as_enum(self) -> Option<()> {
        match self {
            Self::Enum => Some(()),
            _ => None,
        }
    }
    pub fn as_variant(self) -> Option<()> {
        match self {
            Self::Variant => Some(()),
            _ => None,
        }
    }
    pub fn as_function(self) -> Option<()> {
        match self {
            Self::Function => Some(()),
            _ => None,
        }
    }
    pub fn as_typedef(self) -> Option<()> {
        match self {
            Self::Typedef => Some(()),
            _ => None,
        }
    }
    pub fn as_opaque_ty(self) -> Option<()> {
        match self {
            Self::OpaqueTy => Some(()),
            _ => None,
        }
    }
    pub fn as_constant(self) -> Option<()> {
        match self {
            Self::Constant => Some(()),
            _ => None,
        }
    }
    pub fn as_trait(self) -> Option<()> {
        match self {
            Self::Trait => Some(()),
            _ => None,
        }
    }
    pub fn as_trait_alias(self) -> Option<()> {
        match self {
            Self::TraitAlias => Some(()),
            _ => None,
        }
    }
    pub fn as_impl(self) -> Option<()> {
        match self {
            Self::Impl => Some(()),
            _ => None,
        }
    }
    pub fn as_static(self) -> Option<()> {
        match self {
            Self::Static => Some(()),
            _ => None,
        }
    }
    pub fn as_foreign_type(self) -> Option<()> {
        match self {
            Self::ForeignType => Some(()),
            _ => None,
        }
    }
    pub fn as_macro(self) -> Option<()> {
        match self {
            Self::Macro => Some(()),
            _ => None,
        }
    }
    pub fn as_proc_attribute(self) -> Option<()> {
        match self {
            Self::ProcAttribute => Some(()),
            _ => None,
        }
    }
    pub fn as_proc_derive(self) -> Option<()> {
        match self {
            Self::ProcDerive => Some(()),
            _ => None,
        }
    }
    pub fn as_assoc_const(self) -> Option<()> {
        match self {
            Self::AssocConst => Some(()),
            _ => None,
        }
    }
    pub fn as_assoc_type(self) -> Option<()> {
        match self {
            Self::AssocType => Some(()),
            _ => None,
        }
    }
    pub fn as_primitive(self) -> Option<()> {
        match self {
            Self::Primitive => Some(()),
            _ => None,
        }
    }
    pub fn as_keyword(self) -> Option<()> {
        match self {
            Self::Keyword => Some(()),
            _ => None,
        }
    }
}
impl Visibility {
    pub fn as_public(self) -> Option<()> {
        match self {
            Self::Public => Some(()),
            _ => None,
        }
    }
    pub fn as_default(self) -> Option<()> {
        match self {
            Self::Default => Some(()),
            _ => None,
        }
    }
    pub fn as_crate(self) -> Option<()> {
        match self {
            Self::Crate => Some(()),
            _ => None,
        }
    }
    pub fn as_restricted(self) -> Option<(Identifiable, String)> {
        match self {
            Self::Restricted { parent, path } => Some((parent, path)),
            _ => None,
        }
    }
}
impl Identifiable {
    pub fn as_item(self) -> Option<Weak<Item>> {
        match self {
            Self::Item(_0) => Some(_0),
            _ => None,
        }
    }
    pub fn as_summary(self) -> Option<Arc<ItemSummary>> {
        match self {
            Self::Summary(_0) => Some(_0),
            _ => None,
        }
    }
}
impl Item {
    pub fn as_module(self) -> Option<Module> {
        match self {
            Self::Module(_0) => Some(_0),
            _ => None,
        }
    }
    pub fn as_extern_crate(self) -> Option<(ItemMeta, String, Option<String>)> {
        match self {
            Self::ExternCrate { meta, name, rename } => Some((meta, name, rename)),
            _ => None,
        }
    }
    pub fn as_import(self) -> Option<Import> {
        match self {
            Self::Import(_0) => Some(_0),
            _ => None,
        }
    }
    pub fn as_union(self) -> Option<Union> {
        match self {
            Self::Union(_0) => Some(_0),
            _ => None,
        }
    }
    pub fn as_struct(self) -> Option<Struct> {
        match self {
            Self::Struct(_0) => Some(_0),
            _ => None,
        }
    }
    pub fn as_enum(self) -> Option<Enum> {
        match self {
            Self::Enum(_0) => Some(_0),
            _ => None,
        }
    }
    pub fn as_function(self) -> Option<Function> {
        match self {
            Self::Function(_0) => Some(_0),
            _ => None,
        }
    }
    pub fn as_trait(self) -> Option<Trait> {
        match self {
            Self::Trait(_0) => Some(_0),
            _ => None,
        }
    }
    pub fn as_trait_alias(self) -> Option<TraitAlias> {
        match self {
            Self::TraitAlias(_0) => Some(_0),
            _ => None,
        }
    }
    pub fn as_impl(self) -> Option<Impl> {
        match self {
            Self::Impl(_0) => Some(_0),
            _ => None,
        }
    }
    pub fn as_typedef(self) -> Option<Typedef> {
        match self {
            Self::Typedef(_0) => Some(_0),
            _ => None,
        }
    }
    pub fn as_opaque_ty(self) -> Option<OpaqueTy> {
        match self {
            Self::OpaqueTy(_0) => Some(_0),
            _ => None,
        }
    }
    pub fn as_constant(self) -> Option<ConstantItem> {
        match self {
            Self::Constant(_0) => Some(_0),
            _ => None,
        }
    }
    pub fn as_static(self) -> Option<Static> {
        match self {
            Self::Static(_0) => Some(_0),
            _ => None,
        }
    }
    pub fn as_foreign_type(self) -> Option<()> {
        match self {
            Self::ForeignType => Some(()),
            _ => None,
        }
    }
    pub fn as_macro(self) -> Option<(String, ItemMeta, String)> {
        match self {
            Self::Macro { name, meta, expr } => Some((name, meta, expr)),
            _ => None,
        }
    }
    pub fn as_proc_macro(self) -> Option<ProcMacro> {
        match self {
            Self::ProcMacro(_0) => Some(_0),
            _ => None,
        }
    }
    pub fn as_primitive(self) -> Option<Primitive> {
        match self {
            Self::Primitive(_0) => Some(_0),
            _ => None,
        }
    }
    pub fn as_assoc_const(self) -> Option<(ItemMeta, Type, Option<String>)> {
        match self {
            Self::AssocConst {
                meta,
                type_,
                default,
            } => Some((meta, type_, default)),
            _ => None,
        }
    }
    pub fn as_assoc_type(self) -> Option<(ItemMeta, Generics, Vec<GenericBound>, Option<Type>)> {
        match self {
            Self::AssocType {
                meta,
                generics,
                bounds,
                default,
            } => Some((meta, generics, bounds, default)),
            _ => None,
        }
    }
}
impl GenericArgs {
    pub fn as_angle_bracketed(self) -> Option<(Vec<GenericArg>, Vec<TypeBinding>)> {
        match self {
            Self::AngleBracketed { args, bindings } => Some((args, bindings)),
            _ => None,
        }
    }
    pub fn as_parenthesized(self) -> Option<(Vec<Type>, Option<Type>)> {
        match self {
            Self::Parenthesized { inputs, output } => Some((inputs, output)),
            _ => None,
        }
    }
}
impl GenericParamDefKind {
    pub fn as_lifetime(self) -> Option<Vec<String>> {
        match self {
            Self::Lifetime { outlives } => Some(outlives),
            _ => None,
        }
    }
    pub fn as_type(self) -> Option<(Vec<GenericBound>, Option<Type>, bool)> {
        match self {
            Self::Type {
                bounds,
                default,
                synthetic,
            } => Some((bounds, default, synthetic)),
            _ => None,
        }
    }
    pub fn as_const(self) -> Option<(Type, Option<String>)> {
        match self {
            Self::Const { type_, default } => Some((type_, default)),
            _ => None,
        }
    }
}
impl GenericArg {
    pub fn as_lifetime(self) -> Option<String> {
        match self {
            Self::Lifetime(_0) => Some(_0),
            _ => None,
        }
    }
    pub fn as_type(self) -> Option<Type> {
        match self {
            Self::Type(_0) => Some(_0),
            _ => None,
        }
    }
    pub fn as_const(self) -> Option<Constant> {
        match self {
            Self::Const(_0) => Some(_0),
            _ => None,
        }
    }
    pub fn as_infer(self) -> Option<()> {
        match self {
            Self::Infer => Some(()),
            _ => None,
        }
    }
}
impl Type {
    pub fn as_resolved_path(self) -> Option<Path> {
        match self {
            Self::ResolvedPath(_0) => Some(_0),
            _ => None,
        }
    }
    pub fn as_dyn_trait(self) -> Option<DynTrait> {
        match self {
            Self::DynTrait(_0) => Some(_0),
            _ => None,
        }
    }
    pub fn as_generic(self) -> Option<String> {
        match self {
            Self::Generic(_0) => Some(_0),
            _ => None,
        }
    }
    pub fn as_primitive(self) -> Option<String> {
        match self {
            Self::Primitive(_0) => Some(_0),
            _ => None,
        }
    }
    pub fn as_function_pointer(self) -> Option<Box<FunctionPointer>> {
        match self {
            Self::FunctionPointer(_0) => Some(_0),
            _ => None,
        }
    }
    pub fn as_tuple(self) -> Option<Vec<Type>> {
        match self {
            Self::Tuple(_0) => Some(_0),
            _ => None,
        }
    }
    pub fn as_slice(self) -> Option<Box<Type>> {
        match self {
            Self::Slice(_0) => Some(_0),
            _ => None,
        }
    }
    pub fn as_array(self) -> Option<(Box<Type>, String)> {
        match self {
            Self::Array { type_, len } => Some((type_, len)),
            _ => None,
        }
    }
    pub fn as_impl_trait(self) -> Option<Vec<GenericBound>> {
        match self {
            Self::ImplTrait(_0) => Some(_0),
            _ => None,
        }
    }
    pub fn as_infer(self) -> Option<()> {
        match self {
            Self::Infer => Some(()),
            _ => None,
        }
    }
    pub fn as_raw_pointer(self) -> Option<(bool, Box<Type>)> {
        match self {
            Self::RawPointer { mutable, type_ } => Some((mutable, type_)),
            _ => None,
        }
    }
    pub fn as_borrowed_ref(self) -> Option<(Option<String>, bool, Box<Type>)> {
        match self {
            Self::BorrowedRef {
                lifetime,
                mutable,
                type_,
            } => Some((lifetime, mutable, type_)),
            _ => None,
        }
    }
    pub fn as_qualified_path(self) -> Option<(String, Box<GenericArgs>, Box<Type>, Path)> {
        match self {
            Self::QualifiedPath {
                name,
                args,
                self_type,
                trait_,
            } => Some((name, args, self_type, trait_)),
            _ => None,
        }
    }
}
impl TypeBindingKind {
    pub fn as_equality(self) -> Option<Term> {
        match self {
            Self::Equality(_0) => Some(_0),
            _ => None,
        }
    }
    pub fn as_constraint(self) -> Option<Vec<GenericBound>> {
        match self {
            Self::Constraint(_0) => Some(_0),
            _ => None,
        }
    }
}
impl WherePredicate {
    pub fn as_bound_predicate(self) -> Option<(Type, Vec<GenericBound>, Vec<GenericParamDef>)> {
        match self {
            Self::BoundPredicate {
                type_,
                bounds,
                generic_params,
            } => Some((type_, bounds, generic_params)),
            _ => None,
        }
    }
    pub fn as_region_predicate(self) -> Option<(String, Vec<GenericBound>)> {
        match self {
            Self::RegionPredicate { lifetime, bounds } => Some((lifetime, bounds)),
            _ => None,
        }
    }
    pub fn as_eq_predicate(self) -> Option<(Type, Term)> {
        match self {
            Self::EqPredicate { lhs, rhs } => Some((lhs, rhs)),
            _ => None,
        }
    }
}
impl StructKind {
    pub fn as_unit(self) -> Option<()> {
        match self {
            Self::Unit => Some(()),
            _ => None,
        }
    }
    pub fn as_tuple(self) -> Option<Vec<Option<StructField>>> {
        match self {
            Self::Tuple(_0) => Some(_0),
            _ => None,
        }
    }
    pub fn as_plain(self) -> Option<(Vec<StructField>, bool)> {
        match self {
            Self::Plain {
                fields,
                fields_stripped,
            } => Some((fields, fields_stripped)),
            _ => None,
        }
    }
}
impl VariantKind {
    pub fn as_plain(self) -> Option<()> {
        match self {
            Self::Plain => Some(()),
            _ => None,
        }
    }
    pub fn as_tuple(self) -> Option<Vec<Option<StructField>>> {
        match self {
            Self::Tuple(_0) => Some(_0),
            _ => None,
        }
    }
    pub fn as_struct(self) -> Option<(Vec<StructField>, bool)> {
        match self {
            Self::Struct {
                fields,
                fields_stripped,
            } => Some((fields, fields_stripped)),
            _ => None,
        }
    }
}
impl Abi {
    pub fn as_rust(self) -> Option<()> {
        match self {
            Self::Rust => Some(()),
            _ => None,
        }
    }
    pub fn as_c(self) -> Option<bool> {
        match self {
            Self::C { unwind } => Some(unwind),
            _ => None,
        }
    }
    pub fn as_cdecl(self) -> Option<bool> {
        match self {
            Self::Cdecl { unwind } => Some(unwind),
            _ => None,
        }
    }
    pub fn as_stdcall(self) -> Option<bool> {
        match self {
            Self::Stdcall { unwind } => Some(unwind),
            _ => None,
        }
    }
    pub fn as_fastcall(self) -> Option<bool> {
        match self {
            Self::Fastcall { unwind } => Some(unwind),
            _ => None,
        }
    }
    pub fn as_aapcs(self) -> Option<bool> {
        match self {
            Self::Aapcs { unwind } => Some(unwind),
            _ => None,
        }
    }
    pub fn as_win_64(self) -> Option<bool> {
        match self {
            Self::Win64 { unwind } => Some(unwind),
            _ => None,
        }
    }
    pub fn as_sys_v64(self) -> Option<bool> {
        match self {
            Self::SysV64 { unwind } => Some(unwind),
            _ => None,
        }
    }
    pub fn as_system(self) -> Option<bool> {
        match self {
            Self::System { unwind } => Some(unwind),
            _ => None,
        }
    }
    pub fn as_other(self) -> Option<String> {
        match self {
            Self::Other(_0) => Some(_0),
            _ => None,
        }
    }
}
impl GenericBound {
    pub fn as_trait_bound(self) -> Option<(Path, Vec<GenericParamDef>, TraitBoundModifier)> {
        match self {
            Self::TraitBound {
                trait_,
                generic_params,
                modifier,
            } => Some((trait_, generic_params, modifier)),
            _ => None,
        }
    }
    pub fn as_outlives(self) -> Option<String> {
        match self {
            Self::Outlives(_0) => Some(_0),
            _ => None,
        }
    }
}
impl TraitBoundModifier {
    pub fn as_none(self) -> Option<()> {
        match self {
            Self::None => Some(()),
            _ => None,
        }
    }
    pub fn as_maybe(self) -> Option<()> {
        match self {
            Self::Maybe => Some(()),
            _ => None,
        }
    }
    pub fn as_maybe_const(self) -> Option<()> {
        match self {
            Self::MaybeConst => Some(()),
            _ => None,
        }
    }
}
impl Term {
    pub fn as_type(self) -> Option<Type> {
        match self {
            Self::Type(_0) => Some(_0),
            _ => None,
        }
    }
    pub fn as_constant(self) -> Option<Constant> {
        match self {
            Self::Constant(_0) => Some(_0),
            _ => None,
        }
    }
}
impl MacroKind {
    pub fn as_bang(self) -> Option<()> {
        match self {
            Self::Bang => Some(()),
            _ => None,
        }
    }
    pub fn as_attr(self) -> Option<()> {
        match self {
            Self::Attr => Some(()),
            _ => None,
        }
    }
    pub fn as_derive(self) -> Option<()> {
        match self {
            Self::Derive => Some(()),
            _ => None,
        }
    }
}

use crate::heap_types::*;

use super::{Builder, Cache, Result};

impl Builder {
    pub(crate) fn build_function(
        &self,
        cache: &mut Cache,
        name: Option<String>,
        meta: ItemMeta,
        function: &rustdoc_types::Function,
    ) -> Result<Function> {
        Ok(Function {
            name: name.unwrap_or_default(),
            meta,
            decl: self.build_function_decl(cache, &function.decl)?,
            generics: self.build_generics(cache, &function.generics)?,
            header: self.build_header(cache, &function.header)?,
            has_body: function.has_body,
        })
    }

    pub(crate) fn build_function_decl(
        &self,
        cache: &mut Cache,
        decl: &rustdoc_types::FnDecl,
    ) -> Result<FnDecl> {
        let mut inputs = vec![];
        for input in &decl.inputs {
            inputs.push(self.build_function_input(cache, input)?);
        }
        Ok(FnDecl {
            inputs,
            output: match decl.output.as_ref() {
                Some(output) => Some(self.build_type(cache, output)?),
                None => None,
            },
            c_variadic: decl.c_variadic,
        })
    }

    pub(crate) fn build_function_input(
        &self,
        cache: &mut Cache,
        (pat, ty): &(String, rustdoc_types::Type),
    ) -> Result<FnInput> {
        Ok(FnInput {
            pat: pat.clone(),
            ty: self.build_type(cache, ty)?,
        })
    }

    pub(crate) fn build_header(
        &self,
        _cache: &mut Cache,
        header: &rustdoc_types::Header,
    ) -> Result<Header> {
        Ok(Header {
            const_: header.const_,
            unsafe_: header.unsafe_,
            async_: header.async_,
            abi: match header.abi.clone() {
                rustdoc_types::Abi::Rust => Abi::Rust,
                rustdoc_types::Abi::C { unwind } => Abi::C { unwind },
                rustdoc_types::Abi::Cdecl { unwind } => Abi::Cdecl { unwind },
                rustdoc_types::Abi::Stdcall { unwind } => Abi::Stdcall { unwind },
                rustdoc_types::Abi::Fastcall { unwind } => Abi::Fastcall { unwind },
                rustdoc_types::Abi::Aapcs { unwind } => Abi::Aapcs { unwind },
                rustdoc_types::Abi::Win64 { unwind } => Abi::Win64 { unwind },
                rustdoc_types::Abi::SysV64 { unwind } => Abi::SysV64 { unwind },
                rustdoc_types::Abi::System { unwind } => Abi::System { unwind },
                rustdoc_types::Abi::Other(o) => Abi::Other(o),
            },
        })
    }
}

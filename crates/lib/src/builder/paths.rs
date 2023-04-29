

use crate::heap_types::*;


use super::{Builder, Cache, Result};

impl Builder {
    pub(crate) fn build_path(&self, cache: &mut Cache, ty: &rustdoc_types::Path) -> Result<Path> {
        let split = ty.name.rsplit_once("::");
        let (prefix, name) = match split {
            Some((prefix, name)) => (prefix.to_owned(), name.to_owned()),
            None => ("".to_owned(), ty.name.clone()),
        };

        Ok(Path {
            name,
            prefix,
            target: self.get_identifiable(cache, &ty.id)?,
            args: match ty.args.as_ref() {
                None => None,
                Some(args) => Some(Box::new(self.build_generic_args(cache, args)?)),
            },
        })
    }

    pub(crate) fn build_generic_args(
        &self,
        cache: &mut Cache,
        args: &rustdoc_types::GenericArgs,
    ) -> Result<GenericArgs> {
        match args {
            rustdoc_types::GenericArgs::AngleBracketed {
                args: source_args,
                bindings: source_bindings,
            } => {
                let mut args = vec![];
                for arg in source_args {
                    args.push(self.build_generic_arg(cache, arg)?);
                }
                let mut bindings = vec![];
                for binding in source_bindings {
                    bindings.push(self.build_type_binding(cache, binding)?);
                }
                Ok(GenericArgs::AngleBracketed { args, bindings })
            }
            rustdoc_types::GenericArgs::Parenthesized {
                inputs: source_inputs,
                output,
            } => {
                let mut inputs = vec![];
                for input in source_inputs {
                    inputs.push(self.build_type(cache, input)?);
                }
                Ok(GenericArgs::Parenthesized {
                    inputs,
                    output: match output.as_ref() {
                        None => None,
                        Some(o) => Some(self.build_type(cache, o)?),
                    },
                })
            }
        }
    }

    pub(crate) fn build_generic_arg(
        &self,
        cache: &mut Cache,
        arg: &rustdoc_types::GenericArg,
    ) -> Result<GenericArg> {
        Ok(match arg {
            rustdoc_types::GenericArg::Lifetime(l) => GenericArg::Lifetime(l.clone()),
            rustdoc_types::GenericArg::Type(ty) => GenericArg::Type(self.build_type(cache, ty)?),
            rustdoc_types::GenericArg::Const(_) => todo!(),
            rustdoc_types::GenericArg::Infer => GenericArg::Infer,
        })
    }

    pub(crate) fn build_type_binding(
        &self,
        cache: &mut Cache,
        source: &rustdoc_types::TypeBinding,
    ) -> Result<TypeBinding> {
        Ok(TypeBinding {
            name: source.name.clone(),
            args: self.build_generic_args(cache, &source.args)?,
            binding: self.build_type_binding_kind(cache, &source.binding)?,
        })
    }

    pub(crate) fn build_type_binding_kind(
        &self,
        cache: &mut Cache,
        source: &rustdoc_types::TypeBindingKind,
    ) -> Result<TypeBindingKind> {
        Ok(match source {
            rustdoc_types::TypeBindingKind::Equality(term) => {
                TypeBindingKind::Equality(self.build_term(cache, term)?)
            }
            rustdoc_types::TypeBindingKind::Constraint(generic_bounds) => {
                let mut bounds = vec![];
                for bound in generic_bounds {
                    bounds.push(self.build_generic_bound(cache, bound)?);
                }
                TypeBindingKind::Constraint(bounds)
            }
        })
    }

    pub(crate) fn build_term(
        &self,
        cache: &mut Cache,
        source: &rustdoc_types::Term,
    ) -> Result<Term> {
        Ok(match source {
            rustdoc_types::Term::Type(ty) => Term::Type(self.build_type(cache, ty)?),
            rustdoc_types::Term::Constant(_c) => todo!(),
        })
    }
}

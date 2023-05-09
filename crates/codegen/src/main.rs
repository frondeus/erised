use std::{
    io::Write,
    path::{Path, PathBuf},
    process::{Command, Output, Stdio},
};

use anyhow::Context;
use erised::{builder::BuilderOpts, visitor::Visitor};
use module_finder::ModuleFinder;
use proc_macro2::TokenStream;

mod extra_gen;
mod module_finder;
mod static_items_gen;
mod to_tokens_gen;
mod visitor;

pub trait Generator: Visitor + Sized {
    fn branch(&mut self, f: impl FnOnce(&mut Self)) -> TokenStream {
        let saved = std::mem::take(self.output());
        f(self);
        std::mem::replace(self.output(), saved)
    }

    fn output(&mut self) -> &mut TokenStream;
}

fn main() -> anyhow::Result<()> {
    let manifest_dir = PathBuf::from(std::env::var("CARGO_MANIFEST_DIR").unwrap_or_default());

    let krate = BuilderOpts::default()
        .manifest_dir(manifest_dir)
        .package("erised")
        .document_private_items(true)
        .load()?
        .build()?;

    let mut finder = ModuleFinder::default();

    finder.visit_crate(&krate);

    let full_visitor = pretty_print(&finder.visitor.output)?;

    let static_items = &finder.static_items.output;
    let static_items = pretty_print(quote::quote!(
        #![allow(clippy::type_complexity, clippy::just_underscores_and_digits)]

        #static_items
    ))?;

    let to_tokens = &finder.to_tokens.output;
    let extra = &finder.extra.output;

    let imp = pretty_print(quote::quote!(
        #![allow(
            clippy::type_complexity,
            unused_variables,
            clippy::just_underscores_and_digits
        )]

        use crate::heap_types::*;
        use std::sync::{Arc, Weak};
        use crate as erised;

        #to_tokens
        #extra
    ))?;

    write_file("crates/lib/src/types.rs", &static_items)?;
    write_file("crates/lib/src/imp.rs", &imp)?;
    write_file("crates/lib/src/visitor.rs", &full_visitor)?;

    Ok(())
}

pub fn write_file(path: impl AsRef<Path>, input: &str) -> Result<(), anyhow::Error> {
    let path = path.as_ref();
    let mut file = std::fs::OpenOptions::new()
        .read(true)
        .write(true)
        .create(true)
        .truncate(true)
        .open(path)
        .with_context(|| format!("Could not open {path:?}"))?;

    write!(&mut file, "{input}").with_context(|| format!("Could not write to {path:?}"))?;

    file.flush()
        .with_context(|| format!("Could not flush {path:?}"))?;
    Ok(())
}

pub fn pretty_print(tokens: impl quote::ToTokens) -> Result<String, anyhow::Error> {
    let tokens = tokens.into_token_stream().to_string();

    let mut child = Command::new("rustfmt")
        .stdin(Stdio::piped())
        .stdout(Stdio::piped())
        .stderr(Stdio::piped())
        .spawn()
        .context("Unable to start `rustfmt`. Is it installed?")?;

    let mut stdin = child.stdin.take().unwrap();
    write!(stdin, "{tokens}")?;
    stdin.flush()?;
    drop(stdin);

    let Output {
        status,
        stdout,
        stderr,
    } = child.wait_with_output()?;
    let stdout = String::from_utf8_lossy(&stdout);
    let stderr = String::from_utf8_lossy(&stderr);

    if !status.success() {
        eprintln!("---- Stdout ----");
        eprintln!("{stdout}");
        eprintln!("---- Stderr ----");
        eprintln!("{stderr}");
        let code = status.code();
        match code {
            Some(code) => anyhow::bail!("The `rustfmt` command failed with return code {code}"),
            None => anyhow::bail!("The `rustfmt` command failed"),
        }
    }

    Ok(stdout.into())
}

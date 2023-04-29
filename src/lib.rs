use anyhow::{Context, Error};
use quote::ToTokens;
use std::{
    io::Write,
    process::{Command, Output, Stdio},
};

pub use erised::heap_types::*;

pub fn pretty_print_item(tokens: impl ToTokens) -> Result<String, Error> {
    pretty_print(quote::quote!(
        fn test() {
            #tokens
        }
    ))
}

/// Use `rustfmt` to pretty-print the tokens.
pub fn pretty_print(tokens: impl ToTokens) -> Result<String, Error> {
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

pub struct MyStruct {
    name: String,
    en: MyEnum,
}

pub enum MyEnum {
    Unit,
}

mod reflection;

#[test]
fn struct_info() {
    use reflection::Reflect;
    let typeinfo = MyStruct::TYPE_INFO.as_struct().expect("Struct");
}

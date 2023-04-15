use anyhow::Context;
use std::{io::Write, path::Path, process::Command};

use crate::Mirror;

pub fn build_reflection(name: &str, path: impl AsRef<Path>, extra_doc_args: &[&'static str]) {
    build_reflection_inner(name, path, extra_doc_args).expect("Reflection");
}

fn build_reflection_inner(
    name: &str,
    path: impl AsRef<Path>,
    extra_doc_args: &[&'static str],
) -> anyhow::Result<()> {
    let mut mirror = Mirror::build(name, extra_doc_args).context("Mirror build")?;

    let stream = mirror.gen()?;

    let path = path.as_ref();
    let mut file = std::fs::OpenOptions::new()
        .read(true)
        .write(true)
        .create(true)
        .truncate(true)
        .open(path)
        .with_context(|| format!("Could not open {}", path.display()))?;

    write!(&mut file, "{}", stream)?;

    file.flush()?;

    Command::new("cargo").arg("fmt").output()?;
    Ok(())
}

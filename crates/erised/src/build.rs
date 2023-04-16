use anyhow::Context;
use std::{io::Write, process::Command};

use crate::Mirror;

pub fn build_reflection(name: &str, extra_doc_args: &[&'static str]) {
    build_reflection_inner(name, extra_doc_args).expect("Reflection");
}

fn build_reflection_inner(name: &str, extra_doc_args: &[&'static str]) -> anyhow::Result<()> {
    let mut mirror = Mirror::build(name, extra_doc_args).context("Mirror build")?;

    let stream = mirror.gen()?;

    for reflection in stream {
        let path = &reflection.path;
        let mut file = std::fs::OpenOptions::new()
            .read(true)
            .write(true)
            .create(true)
            .truncate(true)
            .open(path)
            .with_context(|| format!("Could not open {}", path.display()))?;

        write!(&mut file, "{}", reflection.reflection)?;

        file.flush()?;
    }

    Command::new("cargo").arg("fmt").output()?;
    Ok(())
}

use std::{io::Write, path::Path, process::Command};

use crate::Mirror;

pub fn build_reflection(path: impl AsRef<Path>) {
    build_reflection_inner(path).expect("Reflection");
}

fn build_reflection_inner(path: impl AsRef<Path>) -> anyhow::Result<()> {
    let mut mirror = Mirror::build()?;

    let stream = mirror.gen()?;

    let mut file = std::fs::OpenOptions::new()
        .read(true)
        .write(true)
        .create(true)
        .truncate(true)
        .open(path)?;

    write!(&mut file, "{}", stream)?;

    file.flush()?;

    Command::new("cargo").arg("fmt").output()?;
    Ok(())
}

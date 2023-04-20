use anyhow::Context;
use std::io::Write;

use crate::Mirror;

pub use rustdoc_json::PackageTarget;

pub fn build_reflection() -> anyhow::Result<()> {
    let mirror = Mirror::build().context("Mirror build")?;
    let mirror = match mirror {
        Some(m) => m,
        None => return Ok(()),
    };
    build_reflection_inner(mirror)?;
    Ok(())
}

pub fn build_reflection_opts(
    opts: impl Fn(rustdoc_json::Builder) -> rustdoc_json::Builder,
) -> anyhow::Result<()> {
    let mirror = Mirror::build_opts(opts).context("Mirror build")?;
    let mirror = match mirror {
        Some(m) => m,
        None => return Ok(()),
    };
    build_reflection_inner(mirror)?;
    Ok(())
}

fn build_reflection_inner(mut mirror: Mirror) -> anyhow::Result<()> {
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

    std::process::Command::new("cargo").arg("fmt").output()?;
    Ok(())
}

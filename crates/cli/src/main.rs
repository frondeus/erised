use error_stack::{IntoReport, Result, ResultExt};
use std::path::PathBuf;
use thiserror::Error;

use clap::{Args, Parser};

#[derive(Parser)]
pub struct BuildArgs {
    /// Rust toolchain used to generate JSON documentation
    #[arg(long, default_value = "nightly")]
    toolchain: String,
    /// Instead using `nightly` erised will use whatever is set with Rustup
    #[arg(long)]
    use_default_toolchain: bool,
    #[arg(long, default_value = "Cargo.toml")]
    manifest_path: PathBuf,
    /// By default it is `${manifest_path}/erised_target` directory
    /// Set what `--target-dir` to pass to `cargo`.
    #[arg(long)]
    target_dir: Option<PathBuf>,
    /// Whether or not to pass `--target` to `cargo rustdoc`.
    #[arg(long)]
    target: Option<String>,
    /// Whether or not to pass `--quiet` to `cargo rustdoc`.
    #[arg(long)]
    quiet: bool,
    /// Whether or not to redirect stdout and stderr to /dev/null.
    #[arg(long)]
    silent: bool,
    /// Whether to pass `--no-default-features` to `cargo rustdoc`.
    #[arg(long)]
    no_default_features: bool,
    /// Whether to pass `--all-features` to `cargo rustdoc`.
    #[arg(long)]
    all_features: bool,
    /// Features to pass to `cargo rustdoc` via `--features`.
    #[arg(long)]
    features: Vec<String>,
    /// Package to use for `cargo rustdoc` via `-p`.
    #[arg(long)]
    package: Option<String>,
    /// Whether to pass `--document-private-items` to `cargo rustdoc`.
    #[arg(long)]
    reflect_private_items: bool,
    /// What to pass as `--cap-lints` to rustdoc JSON build command
    #[arg(long)]
    cap_lints: Option<String>,
    #[command(flatten)]
    package_target: PackageTarget,
    #[command(flatten)]
    crate_replacement: CrateReplacement,

    output: PathBuf,
}

#[derive(Args)]
pub struct PackageTarget {
    /// Document the given binary, i.e. pass `--bin <name>`
    #[arg(long)]
    bin: Option<String>,
    /// Document the given binary, i.e. pass `--example <name>`
    #[arg(long)]
    example: Option<String>,
    /// Document the given binary, i.e. pass `--test <name>`
    #[arg(long)]
    test: Option<String>,
    /// Document the given binary, i.e. pass `--bench <name>`
    #[arg(long)]
    bench: Option<String>,
}
#[derive(Args)]
pub struct CrateReplacement {
    /// While generating paths to the types of this crate, should it replace `full_name_of_crate::my_path` with `crate::my_path`
    #[arg(long)]
    replace_root_crate: bool,
    /// While generating paths to the types of other crate provided with the argument, should it replace `full_name_of_crate::my_path` with `crate::my_path`
    #[arg(long)]
    replace_other_crate: Option<String>,
}

fn main() -> Result<(), StaticReflectionError> {
    let mut args = BuildArgs::parse();
    let output = std::mem::take(&mut args.output);
    let builder = erised::builder::BuilderOpts::from(args)
        .load()
        .into_report()
        .change_context(StaticReflectionError)?;

    builder
        .build_static_reflection(output)
        .into_report()
        .change_context(StaticReflectionError)?;

    Ok(())
}

#[derive(Error, Debug)]
#[error("Could not build static reflection")]
struct StaticReflectionError;

impl From<BuildArgs> for erised::builder::BuilderOpts {
    fn from(value: BuildArgs) -> Self {
        let BuildArgs {
            toolchain,
            use_default_toolchain,
            manifest_path,
            target_dir,
            target,
            quiet,
            silent,
            no_default_features,
            all_features,
            features,
            package,
            reflect_private_items,
            cap_lints,
            package_target,
            crate_replacement,
            output: _,
        } = value;
        let manifest_dir = manifest_path
            .parent()
            .expect("Path to have parent")
            .to_owned();
        Self {
            toolchain: if use_default_toolchain {
                None
            } else {
                Some(toolchain)
            },
            manifest_path,
            target_dir,
            target,
            quiet,
            silent,
            no_default_features,
            all_features,
            features,
            package,
            package_target: {
                match (
                    package_target.bin,
                    package_target.example,
                    package_target.test,
                    package_target.bench,
                ) {
                    (Some(b), _, _, _) => erised::builder::PackageTarget::Bin(b),
                    (_, Some(e), _, _) => erised::builder::PackageTarget::Example(e),
                    (_, _, Some(t), _) => erised::builder::PackageTarget::Test(t),
                    (_, _, _, Some(b)) => erised::builder::PackageTarget::Bench(b),
                    (None, None, None, None) => erised::builder::PackageTarget::Lib,
                }
            },
            document_private_items: reflect_private_items,
            cap_lints,
            crate_replacement: {
                match (
                    crate_replacement.replace_root_crate,
                    crate_replacement.replace_other_crate,
                ) {
                    (true, _) => Some(erised::builder::CrateReplacement::Root),
                    (_, Some(c)) => Some(erised::builder::CrateReplacement::OtherByName(c)),
                    (_, _) => None,
                }
            },
        }
        .manifest_dir(manifest_dir)
    }
}

//! Stripping of Cargo generated binaries.

use std::process::Command;

use anyhow::Context;
use camino::{Utf8Path, Utf8PathBuf};

#[derive(Debug, Default, Clone, Copy, PartialEq, clap::ValueEnum)]
pub enum StripAction {
    /// Don't do anything. Default.
    #[default]
    Skip,
    /// Keep only line-tables debug information. This is the minimum requirement for stacktraces.
    KeepLineTables,
    /// Strip off all debug information. Information is lost if not preserved otherwise.
    Strip,
    /// Debug information is saved in a separate .debug file.
    Separate,
}

pub fn strip_if_required<P>(
    binary_path: P,
    target_triple: &str,
    action: StripAction,
) -> anyhow::Result<()>
where
    P: AsRef<Utf8Path>,
{
    match action {
        StripAction::Skip => Ok(()),
        StripAction::KeepLineTables => todo!(),
        StripAction::Strip => {
            let status = objcopy(target_triple)?
                .arg("--strip-debug")
                .arg("--strip-unneeded")
                .arg(binary_path.as_ref().as_str())
                .status()
                .context("calling llvm-objcopy")?;
            if !status.success() {
                return Err(anyhow::anyhow!("llvm-objcopy exited with non-zero"));
            }

            Ok(())
        }

        StripAction::Separate => {
            let debug_sym_path = binary_path.as_ref().with_extension("debug");

            let status = objcopy(target_triple)?
                .arg("--only-keep-debug")
                .arg("--compress-debug-sections=zlib")
                .arg(binary_path.as_ref().as_str())
                .arg(debug_sym_path.as_str())
                .status()
                .context("calling llvm-objcopy")?;
            if !status.success() {
                return Err(anyhow::anyhow!("llvm-objcopy exited with non-zero"));
            }

            let symbol_name = debug_sym_path.file_name().context("missing filename")?;
            let status = objcopy(target_triple)?
                .arg("--strip-debug")
                .arg("--strip-unneeded")
                .arg("--remove-section=\".gnu_debuglink\"")
                .arg(format!("--add-gnu-debuglink=\"{symbol_name}\""))
                .arg(binary_path.as_ref().as_str())
                .status()
                .context("calling llvm-objcopy")?;
            if !status.success() {
                return Err(anyhow::anyhow!("llvm-objcopy exited with non-zero"));
            }

            Ok(())
        }
    }
}

fn objcopy(target_triple: &str) -> anyhow::Result<Command> {
    let out = Command::new("rustc")
        .args(["--print", "sysroot"])
        .output()
        .context("finding rustc sysroot")?;
    if !out.status.success() {
        return Err(anyhow::anyhow!("failed to find rustc sysroot"));
    }

    let outstr = String::from_utf8(out.stdout).context("invalid utf8")?;
    let sysroot = Utf8PathBuf::from(outstr.trim());
    let objcopy_path = sysroot
        .join("lib/rustlib")
        .join(target_triple)
        .join("bin/llvm-objcopy");

    Ok(Command::new(objcopy_path))
}

use std::process::Command;

use anyhow::Context;
use cargo_nfpm::cargo::{self, ProjectBuilder};
use cargo_nfpm::generator::{get_config_from_package, OutputFormat};
use cargo_nfpm::nfpm::download_nfpm;
use cargo_nfpm::nfpm_schema::{ContentElement, FileInfo};
use cargo_nfpm::strip::{strip_if_required, StripAction};
use clap::Parser;

#[derive(clap::Parser, Debug)]
#[command(author, version, about, long_about = None)]
#[command(next_line_help = true)]
#[command(propagate_version = true)]
pub struct CliArgs {
    #[command(subcommand)]
    command: Commands,
}

#[derive(clap::Subcommand, Debug)]
pub enum Commands {
    /// Call `cargo-nfpm`.
    Nfpm(NfpmArgs),
}

#[derive(clap::Args, Debug)]
pub struct NfpmArgs {
    #[command(subcommand)]
    command: NfpmCommands,
}

#[derive(clap::Subcommand, Debug)]
pub enum NfpmCommands {
    /// Creates a package based on the `Cargo.toml` config
    #[clap(aliases = ["pkg", "p"])]
    Package(PackageArgs),
}

#[derive(clap::Args, Debug)]
pub struct PackageArgs {
    /// Package to build (see `cargo help pkgid`)
    #[arg(long, short, value_name = "SPEC")]
    package: Option<String>,

    /// Build for the target triple
    #[arg(long, value_name = "TRIPLE")]
    target: Option<String>,

    /// Build artifacts with the specified profile
    #[arg(long, group = "profilegroup", value_name = "PROFILE-NAME")]
    profile: Option<String>,

    /// List of features to activate
    #[arg(long, short = 'F', value_name = "FEATURES")]
    features: Option<Vec<String>>,

    /// Where to save the generated package.
    #[arg(long, short, value_name = "PATH")]
    output: Option<String>,

    /// Whether to skip the build.
    #[arg(long, action = clap::ArgAction::SetTrue)]
    no_build: bool,

    /// Whether to skip downloading nFPM.
    #[arg(long, action = clap::ArgAction::SetTrue)]
    no_vendor: bool,

    /// Package format.
    #[arg(long, short)]
    format: OutputFormat,

    /// Strip action.
    #[arg(long, short, default_value = "skip")]
    strip: StripAction,

    /// Build options passed to `cargo build`
    build_options: Option<Vec<String>>,
}

fn main() -> anyhow::Result<()> {
    let cli: CliArgs = CliArgs::parse();
    let args = match cli.command {
        Commands::Nfpm(args) => match args.command {
            NfpmCommands::Package(package_args) => package_args,
        },
    };

    let metadata = cargo::Metadata::get()?;
    let triple = if let Some(target) = &args.target {
        target.clone()
    } else {
        cargo::get_host_triple()?
    };
    let tmpdir = metadata.target_directory().join("tmp");
    std::fs::create_dir_all(&tmpdir).context("creating temporary directory")?;

    let nfpm_bin = if args.no_vendor {
        if let Ok(bin_path) = std::env::var("CARGO_NFPM_BIN") {
            bin_path
        } else {
            "nfpm".to_owned()
        }
    } else {
        download_nfpm(&tmpdir)?;
        tmpdir.join("nfpm").to_string()
    };

    let package = metadata
        .root_package(args.package.as_deref())
        .context("couldn't find root package")?;
    let bin_target = package
        .targets
        .iter()
        .find(|el| el.is_bin())
        .context("no binary target found")?;
    let profile = {
        if let Some(profile) = &args.profile {
            profile.clone()
        } else {
            "release".to_owned()
        }
    };

    let target_path = if let Some(target) = &args.target {
        metadata.target_directory().join(target).join(&profile)
    } else {
        metadata.target_directory().join(&profile)
    };

    let binary_path = target_path.join(&bin_target.name);

    if !args.no_build {
        let mut builder = ProjectBuilder::default().with_profile(&profile);
        if let Some(package) = &args.package {
            builder = builder.with_package(package);
        }
        if let Some(target) = &args.target {
            builder = builder.with_target(target);
        }
        if let Some(features) = &args.features {
            for feature in features {
                builder = builder.with_feature(feature);
            }
        }

        if let Some(extra_args) = &args.build_options {
            builder = builder.with_extra_args(extra_args.clone());
        }

        builder.build()?;
    }

    strip_if_required(&binary_path, &triple, args.strip).context("stripping binary")?;

    let mut config = get_config_from_package(package, &triple, args.format)
        .context("create config from Cargo manifest")?;

    let auto_add_binary = if let Some(contents) = &config.contents {
        !contents.iter().any(|el| {
            if let Some(src) = &el.src {
                src == binary_path.as_str()
            } else {
                false
            }
        })
    } else {
        true
    };

    if auto_add_binary {
        let bin_content = ContentElement {
            dst: format!("/usr/bin/{}", bin_target.name),
            src: Some(binary_path.to_string()),
            expand: Some(false),
            file_info: Some(FileInfo {
                group: None,
                mode: Some(0o755),
                mtime: None,
                owner: None,
            }),
            packager: None,
            list_connections_result_type: None,
        };

        if let Some(contents) = &mut config.contents {
            contents.push(bin_content);
        } else {
            config.contents = Some(vec![bin_content]);
        }
    }

    let output = serde_yaml::to_string(&config).context("serializing config as YAML")?;
    let config_path = tmpdir.join("nfpm.yml");
    std::fs::write(&config_path, &output).context("writing nfpm.yml to disk")?;

    let packager = match args.format {
        OutputFormat::Apk => "apk",
        OutputFormat::Archlinux => "archlinux",
        OutputFormat::Deb => "deb",
        OutputFormat::Ipk => "ipk",
        OutputFormat::Rpm => "rpm",
    };
    let target = if let Some(output) = &args.output {
        output.clone()
    } else {
        target_path.to_string()
    };
    let mut cmd = Command::new(nfpm_bin);
    cmd.arg("package")
        .arg("--config")
        .arg(config_path)
        .arg("--packager")
        .arg(packager)
        .arg("--target")
        .arg(target);

    let res = cmd.status().context("running nfpm")?;
    if res.success() {
        return Ok(());
    }

    Err(anyhow::anyhow!(
        "failed to build package. check stdout/stderr"
    ))
}

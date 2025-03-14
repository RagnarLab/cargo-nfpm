use anyhow::Context;
use cargo_nfpm::cargo::{self, ProjectBuilder};
use cargo_nfpm::generator::get_config_from_metadata;
use cargo_nfpm::nfpm_schema::{ContentElement, FileInfo};
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
    Nfpm(NfpmArgs),
}

#[derive(clap::Args, Debug)]
pub struct NfpmArgs {
    #[command(subcommand)]
    command: NfpmCommands,
}

#[derive(clap::Subcommand, Debug)]
pub enum NfpmCommands {
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
    #[arg(long, short, value_name = "FEATURES")]
    features: Option<Vec<String>>,

    /// Whether to skip the build.
    #[arg(long, action = clap::ArgAction::SetTrue)]
    no_build: bool,

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

    let metadata = cargo::find_cargo_metadata()?;
    let triple = if let Some(target) = &args.target {
        target.clone()
    } else {
        cargo::get_host_triple()?
    };

    let package = metadata
        .root_package()
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
        metadata.target_directory.join(target).join(&profile)
    } else {
        metadata.target_directory.join(&profile)
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

        builder.build()?;
    }

    let mut config = get_config_from_metadata(&metadata, package, &triple)
        .context("create config from Cargo manifest")?;

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
        confi_type: None,
    };

    config.contents = Some(vec![bin_content]);

    let output = serde_yaml::to_string(&config).context("serializing config as YAML")?;
    println!("{output}");

    Ok(())
}

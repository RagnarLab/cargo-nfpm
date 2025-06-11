//! Loading and converting the `Cargo.toml` to a `nfpm.yaml`.

use anyhow::Context;

use crate::cargo_schema::CargoManifest;
use crate::nfpm_schema;
use crate::triple::LlvmTriple;

#[derive(Debug, Clone, Copy, clap::ValueEnum)]
pub enum OutputFormat {
    Apk,
    Archlinux,
    Deb,
    Ipk,
    Rpm,
}

pub fn get_config_from_package(
    package: &cargo_metadata::Package,
    triple: &str,
    format: OutputFormat,
) -> anyhow::Result<nfpm_schema::Config> {
    let manifest_str =
        std::fs::read_to_string(&package.manifest_path).context("reading Cargo.toml manifest")?;
    let manifest: CargoManifest = toml::from_str(&manifest_str).context("parsing Cargo.toml")?;

    let mut base_config = manifest
        .package
        .metadata
        .as_ref()
        .and_then(|metadata| metadata.nfpm.clone())
        .unwrap_or_default();

    merge_base_config_with_package(package, &mut base_config)
        .context("merging config with manifest values")?;

    let parsed = LlvmTriple::new(triple)
        .with_context(|| format!("parsing cargo target triple: {triple}"))?;
    base_config.arch = match format {
        OutputFormat::Apk => Some(parsed.to_apk_arch()?.to_owned()),
        OutputFormat::Archlinux => Some(parsed.to_archlinux_arch()?.to_owned()),
        OutputFormat::Deb => Some(parsed.to_deb_arch()?.to_owned()),
        OutputFormat::Ipk => Some(parsed.to_ipk_arch()?.to_owned()),
        OutputFormat::Rpm => Some(parsed.to_rpm_arch()?.to_owned()),
    };

    // Set some sane defaults.
    if base_config.epoch.is_none() {
        base_config.epoch = Some("1".to_owned());
    }
    if base_config.release.is_none() {
        base_config.release = Some("1".to_owned());
    }
    if base_config.platform.is_none() {
        base_config.platform = Some("linux".to_owned());
    }
    if base_config.section.is_none() {
        base_config.section = Some("default".to_owned());
    }
    if base_config.priority.is_none() {
        base_config.priority = Some("extra".to_owned());
    }

    assert!(base_config.name.is_some());
    assert!(base_config.arch.is_some());
    assert!(base_config.version.is_some());

    Ok(base_config)
}

fn merge_base_config_with_package(
    package: &cargo_metadata::Package,
    config: &mut nfpm_schema::Config,
) -> anyhow::Result<()> {
    // Only apply defaults if override isn't set.
    if config.name.is_none() {
        config.name = Some(package.name.as_ref().to_owned());
    }
    if config.version.is_none() {
        config.version = Some(package.version.to_string());
    }
    if config.maintainer.is_none() {
        config.maintainer = Some(package.authors.join(", "));
    }
    if config.description.is_none() {
        if let Some(description) = &package.description {
            config.description = Some(description.clone());
        }
    }
    // The properties `documentation`, `readme`, and `repository` have no corresponding
    // config value.
    if config.homepage.is_none() {
        if let Some(homepage) = &package.homepage {
            config.homepage = Some(homepage.clone());
        }
    }
    if config.license.is_none() {
        if let Some(license) = &package.license {
            config.license = Some(license.clone());
        }
    }

    Ok(())
}

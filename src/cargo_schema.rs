//! The schema for deserializing `Cargo.toml` with `nFPM` metadata.

use serde::{Deserialize, Serialize};

use crate::nfpm_schema;

/// The `Cargo.toml` file for each package is called its manifest.
#[derive(Debug, Default, Clone, Serialize, Deserialize)]
pub struct CargoManifest {
    /// Defines a package.
    pub package: CargoPackage,
}

/// Defines a package.
#[derive(Debug, Default, Clone, Serialize, Deserialize)]
pub struct CargoPackage {
    // /// The name of the package.
    // pub name: Option<String>,
    // /// The version of the package.
    // pub version: Option<String>,
    // /// The authors of the package.
    // pub authors: Option<Vec<String>>,
    // /// A description of the package.
    // pub description: Option<String>,
    // /// URL of the package documentation.
    // pub documentation: Option<String>,
    // /// Path to the package’s README file.
    // pub readme: Option<String>,
    // /// URL of the package homepage.
    // pub homepage: Option<String>,
    // /// URL of the package source repository.
    // pub repository: Option<String>,
    // /// The package license.
    // pub license: Option<String>,
    // /// Path to the text of the license.
    // #[serde(rename = "license-file")]
    // pub license_file: Option<String>,
    /// Extra settings for external tools.
    pub metadata: Option<CargoMetadata>,
}

/// Metadata for nFPM.
#[derive(Debug, Default, Clone, Serialize, Deserialize)]
pub struct CargoMetadata {
    /// `Config` is derived from the JSON schema from `nFPM`.
    pub nfpm: Option<nfpm_schema::Config>,
}

//! This file is autogenerated from the JSON schema of nFPM v2.41.3 using [QuickType].
//!
//! [QuickType]: https://app.quicktype.io/
use std::collections::HashMap;

use serde::{Deserialize, Serialize};

/// nFPM configuration definition file
#[derive(Debug, Default, Clone, PartialEq, Serialize, Deserialize)]
pub struct Config {
    // Non optional.
    pub name: Option<String>,
    pub arch: Option<String>,
    pub version: Option<String>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub apk: Option<ApkSpecificSettings>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub archlinux: Option<ArchlinuxSpecificSettings>,
    /// see https://github.com/goreleaser/chglog for more details
    #[serde(skip_serializing_if = "Option::is_none")]
    pub changelog: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub conflicts: Option<Vec<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub contents: Option<Vec<ContentElement>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub deb: Option<DebSpecificSettings>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub depends: Option<Vec<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub disable_globbing: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub epoch: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub homepage: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub ipk: Option<IpkSpecificSettings>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub license: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub maintainer: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub mtime: Option<String>,
    /// override some fields when packaging with a specific packager
    #[serde(skip_serializing_if = "Option::is_none")]
    pub overrides: Option<HashMap<String, OverrideValue>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub platform: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub prerelease: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub priority: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub provides: Option<Vec<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub recommends: Option<Vec<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub release: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub replaces: Option<Vec<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub rpm: Option<RpmSpecificSettings>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub scripts: Option<ScriptsToExecute>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub section: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub suggests: Option<Vec<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub umask: Option<i64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub vendor: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub version_metadata: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub version_schema: Option<VersionSchema>,
}

#[derive(Debug, Default, Clone, PartialEq, Serialize, Deserialize)]
pub struct ApkSpecificSettings {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub arch: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub scripts: Option<ApkScripts>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub signature: Option<ApkSignature>,
}

#[derive(Debug, Default, Clone, PartialEq, Serialize, Deserialize)]
pub struct ApkScripts {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub postupgrade: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub preupgrade: Option<String>,
}

#[derive(Debug, Default, Clone, PartialEq, Serialize, Deserialize)]
pub struct ApkSignature {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub key_file: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub key_id: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub key_name: Option<String>,
}

#[derive(Debug, Default, Clone, PartialEq, Serialize, Deserialize)]
pub struct ArchlinuxSpecificSettings {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub arch: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub packager: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub pkgbase: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub scripts: Option<ArchlinuxSpecificScripts>,
}

#[derive(Debug, Default, Clone, PartialEq, Serialize, Deserialize)]
pub struct ArchlinuxSpecificScripts {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub postupgrade: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub preupgrade: Option<String>,
}

#[derive(Debug, Default, Clone, PartialEq, Serialize, Deserialize)]
pub struct ContentElement {
    pub dst: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub expand: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub file_info: Option<FileInfo>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub packager: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub src: Option<String>,
    #[serde(rename = "type")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub confi_type: Option<Type>,
}

#[derive(Debug, Default, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum Type {
    Config,
    #[serde(rename = "config|noreplace")]
    ConfigNoreplace,
    Dir,
    #[default]
    #[serde(rename = "")]
    Empty,
    Ghost,
    Symlink,
    Tree,
}

#[derive(Debug, Default, Clone, PartialEq, Serialize, Deserialize)]
pub struct FileInfo {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub group: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub mode: Option<i64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub mtime: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub owner: Option<String>,
}

#[derive(Debug, Default, Clone, PartialEq, Serialize, Deserialize)]
pub struct DebSpecificSettings {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub arch: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub breaks: Option<Vec<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub compression: Option<DebCompression>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub fields: Option<HashMap<String, String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub predepends: Option<Vec<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub scripts: Option<Scripts>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub signature: Option<Signature>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub triggers: Option<Triggers>,
}

#[derive(Debug, Default, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum DebCompression {
    #[default]
    Gzip,
    None,
    Xz,
}

#[derive(Debug, Default, Clone, PartialEq, Serialize, Deserialize)]
pub struct Scripts {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub config: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub rules: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub templates: Option<String>,
}

#[derive(Debug, Default, Clone, PartialEq, Serialize, Deserialize)]
pub struct Signature {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub key_file: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub key_id: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub method: Option<MethodRole>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub signer: Option<String>,
    #[serde(rename = "type")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub signature_type: Option<SignerRole>,
}

#[derive(Debug, Default, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum MethodRole {
    #[default]
    Debsign,
}

#[derive(Debug, Default, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum SignerRole {
    Archive,
    Maint,
    #[default]
    Origin,
}

#[derive(Debug, Default, Clone, PartialEq, Serialize, Deserialize)]
pub struct Triggers {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub activate: Option<Vec<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub activate_await: Option<Vec<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub activate_noawait: Option<Vec<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub interest: Option<Vec<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub interest_await: Option<Vec<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub interest_noawait: Option<Vec<String>>,
}

#[derive(Debug, Default, Clone, PartialEq, Serialize, Deserialize)]
pub struct IpkSpecificSettings {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub abi_version: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub alternatives: Option<Vec<Alternative>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub arch: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub auto_installed: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub essential: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub fields: Option<HashMap<String, String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub predepends: Option<Vec<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tags: Option<Vec<String>>,
}

#[derive(Debug, Default, Clone, PartialEq, Serialize, Deserialize)]
pub struct Alternative {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub link_name: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub priority: Option<i64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub target: Option<String>,
}

#[derive(Debug, Default, Clone, PartialEq, Serialize, Deserialize)]
pub struct OverrideValue {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub apk: Option<ApkSpecificSettings>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub archlinux: Option<ArchlinuxSpecificSettings>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub conflicts: Option<Vec<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub contents: Option<Vec<ContentElement>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub deb: Option<DebSpecificSettings>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub depends: Option<Vec<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub ipk: Option<IpkSpecificSettings>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub provides: Option<Vec<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub recommends: Option<Vec<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub replaces: Option<Vec<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub rpm: Option<RpmSpecificSettings>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub scripts: Option<ScriptsToExecute>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub suggests: Option<Vec<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub umask: Option<i64>,
}

#[derive(Debug, Default, Clone, PartialEq, Serialize, Deserialize)]
pub struct RpmSpecificSettings {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub arch: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub compression: Option<RpmCompression>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub group: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub packager: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub prefixes: Option<Vec<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub scripts: Option<RpmSpecificScripts>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub signature: Option<RpmSignature>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub summary: Option<String>,
}

#[derive(Debug, Default, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum RpmCompression {
    #[default]
    Gzip,
    Lzma,
    Xz,
}

#[derive(Debug, Default, Clone, PartialEq, Serialize, Deserialize)]
pub struct RpmSpecificScripts {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub posttrans: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub pretrans: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub verify: Option<String>,
}

#[derive(Debug, Default, Clone, PartialEq, Serialize, Deserialize)]
pub struct RpmSignature {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub key_file: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub key_id: Option<String>,
}

#[derive(Debug, Default, Clone, PartialEq, Serialize, Deserialize)]
pub struct ScriptsToExecute {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub postinstall: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub postremove: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub preinstall: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub preremove: Option<String>,
}

#[derive(Debug, Default, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum VersionSchema {
    None,
    #[default]
    Semver,
}

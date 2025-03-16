//! Cargo utilities.

use std::process::{Command, Stdio};

use anyhow::Context;
use camino::Utf8PathBuf;
use cargo_metadata::{Artifact, Message};

#[derive(Debug, Clone)]
pub struct Metadata {
    inner: cargo_metadata::Metadata,
}

impl Metadata {
    /// Retrieves the metadata of the current Cargo workspace / project.
    pub fn get() -> anyhow::Result<Self> {
        let mut cmd = cargo_metadata::MetadataCommand::new();
        cmd.no_deps();
        let inner = cmd.exec().context("failed to run `cargo metadata`")?;
        Ok(Self { inner })
    }

    /// Retrieves Cargo `target` directory.
    pub fn target_directory(&self) -> Utf8PathBuf {
        if let Ok(target_dir) = std::env::var("CARGO_TARGET_DIR") {
            return Utf8PathBuf::from(target_dir);
        }

        self.inner.target_directory.clone()
    }

    /// Get the workspace's root package of this metadata instance.
    pub fn root_package(&self) -> Option<&cargo_metadata::Package> {
        self.inner.root_package()
    }
}

/// Find the host triple by invoking `cargo -vV`.
pub fn get_host_triple() -> anyhow::Result<String> {
    let out = cargo_cmd()
        .arg("-vV")
        .output()
        .context("calling `cargo -vV`")?;
    let outstr = String::from_utf8(out.stdout).context("output malformed")?;
    for line in outstr.lines() {
        if line.starts_with("host:") {
            let (_, triple) = line.split_once(":").expect("malformed format");
            return Ok(triple.trim().to_owned());
        }
    }

    Err(anyhow::anyhow!("couldn't find host triple"))
}

#[derive(Debug, Default)]
pub struct ProjectBuilder {
    package: Option<String>,
    target: Option<String>,
    profile: Option<String>,
    features: Option<Vec<String>>,
    extra_args: Option<Vec<String>>,
}

impl ProjectBuilder {
    pub fn with_package<S>(mut self, package: S) -> Self
    where
        S: Into<String>,
    {
        self.package = Some(package.into());
        self
    }

    pub fn with_target<S>(mut self, target: S) -> Self
    where
        S: Into<String>,
    {
        self.target = Some(target.into());
        self
    }

    pub fn with_profile<S>(mut self, profile: S) -> Self
    where
        S: Into<String>,
    {
        self.profile = Some(profile.into());
        self
    }

    pub fn with_feature<S>(mut self, feature: S) -> Self
    where
        S: Into<String>,
    {
        if let Some(features) = &mut self.features {
            features.push(feature.into());
        } else {
            self.features = Some(vec![feature.into()]);
        }
        self
    }

    pub fn with_extra_args(mut self, extra_args: Vec<String>) -> Self {
        self.extra_args = Some(extra_args);
        self
    }

    pub fn build(self) -> anyhow::Result<Vec<Artifact>> {
        let mut command = cargo_cmd();

        command
            .arg("build")
            .arg("--message-format=json-render-diagnostics");

        if let Some(package) = &self.package {
            command.arg("-p");
            command.arg(package);
        }
        if let Some(target) = &self.target {
            command.arg("--target");
            command.arg(target);
        }
        if let Some(profile) = &self.profile {
            command.arg("--profile");
            command.arg(profile);
        }
        if let Some(features) = &self.features {
            for feature in features {
                command.arg("-F");
                command.arg(feature);
            }
        }

        if let Some(extra_args) = &self.extra_args {
            command.args(extra_args);
        }

        let mut child = command
            .stderr(Stdio::inherit())
            .stdout(Stdio::piped())
            .spawn()
            .context("running `cargo build`")?;

        let reader = std::io::BufReader::new(child.stdout.take().unwrap());
        let mut artifacts: Vec<Artifact> = Vec::with_capacity(8);
        for message in cargo_metadata::Message::parse_stream(reader) {
            match message.unwrap() {
                Message::CompilerMessage(_msg) => {
                    // println!("{:?}", msg);
                }
                Message::CompilerArtifact(artifact) => {
                    if artifact.target.is_bin() {
                        artifacts.push(artifact);
                    }
                }
                Message::BuildScriptExecuted(_script) => {
                    // println!("{:?}", script);
                }
                Message::BuildFinished(_finished) => {
                    // println!("{:?}", finished);
                }
                _ => (), // Unknown message
            }
        }

        let status = child.wait().expect("Couldn't get cargo's exit status");

        if status.success() {
            return Ok(artifacts);
        }

        Err(anyhow::anyhow!(
            "build failed: {}",
            status.code().unwrap_or(-1)
        ))
    }
}

pub fn cargo_cmd() -> Command {
    let cargo_bin = std::env::var("CARGO").unwrap_or("cargo".to_owned());
    let mut cmd = Command::new(cargo_bin);
    if let Ok(term_color) = std::env::var("CARGO_TERM_COLOR") {
        cmd.env("CARGO_TERM_COLOR", term_color);
    }
    if let Ok(target_dir) = std::env::var("CARGO_TARGET_DIR") {
        cmd.env("CARGO_TARGET_DIR", target_dir);
    }

    cmd
}

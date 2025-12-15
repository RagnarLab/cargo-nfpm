//! Downloading and invoking nFPM

use std::fs::File;
use std::io::{Read as _, Write as _};
use std::path::PathBuf;

use anyhow::Context;
use camino::Utf8Path;
use flate2::read::GzDecoder;
use sha2::Digest;
use tar::Archive;

const NFPM_VERSION: &str = "2.44.0";
const CHECKSUMS: &str = include_str!(concat!(
    std::env!("CARGO_MANIFEST_DIR"),
    "/nfpm_checksums.txt"
));

pub fn download_nfpm<P>(outdir: P) -> anyhow::Result<()>
where
    P: AsRef<Utf8Path>,
{
    let binary_path = outdir.as_ref().join("nfpm");
    if binary_path.exists() {
        return Ok(());
    }
    println!("downloading nfpm...");

    let (os, filetype) = if cfg!(target_os = "linux") {
        ("Linux", "tar.gz")
    } else if cfg!(target_os = "macos") {
        ("Darwin", "tar.gz")
    } else if cfg!(target_os = "windows") {
        ("Windows", "zip")
    } else {
        return Err(anyhow::anyhow!("unsupported operating system"));
    };

    let arch = if cfg!(target_arch = "x86_64") {
        "x86_64"
    } else if cfg!(target_arch = "aarch64") {
        "arm64"
    } else {
        return Err(anyhow::anyhow!("unsupported system architecture"));
    };

    let archive_name = format!("nfpm_{NFPM_VERSION}_{os}_{arch}.{filetype}");
    let url = format!(
        "https://github.com/goreleaser/nfpm/releases/download/v{NFPM_VERSION}/{archive_name}"
    );

    let mut res = ureq::get(&url)
        .call()
        .with_context(|| format!("downloading nfpm from {url}"))?;

    let body: &mut ureq::Body = res.body_mut();
    let archive_path = outdir.as_ref().join(format!("nfpm.{filetype}"));
    let mut archivefp = std::fs::File::create(&archive_path)
        .with_context(|| format!("creating file: nfpm.{filetype}"))?;
    let mut buf = [0_u8; 16 * 1024];
    let mut reader = body.as_reader();
    let mut hasher = sha2::Sha256::new();
    loop {
        match reader.read(&mut buf) {
            Ok(0) => {
                break;
            }
            Ok(n) => {
                hasher.update(&buf[0..n]);
                archivefp.write_all(&buf[0..n]).context("writing to file")?;
                continue;
            }
            Err(err) => return Err(err.into()),
        }
    }

    let real_sha256sum = hasher.finalize().to_vec();
    let expected_sha256sum = find_hash_for(&archive_name)?;
    if real_sha256sum != expected_sha256sum {
        return Err(anyhow::anyhow!(
            "checksum mismatch. expected: {}, got: {}",
            hex::encode(&expected_sha256sum),
            hex::encode(&real_sha256sum)
        ));
    }

    archivefp
        .flush()
        .context("flushing temporary archive to disk")?;
    let archivefp = File::open(&archive_path).context("opening nfpm.tar.gz for reading")?;

    if filetype == "tar.gz" {
        let tar = GzDecoder::new(archivefp);
        let mut archive = Archive::new(tar);
        let entries = archive
            .entries()
            .context("reading tar.gz archive entries")?;
        for entry in entries {
            let mut entry = entry.context("reading archive entry")?;
            let path = entry.path().context("retrieving entry path")?;
            if let Some(file_name) = path.file_name() {
                if file_name.to_str() == Some("nfpm") {
                    entry.unpack(&binary_path).context("unpacking nfpm")?;
                    return Ok(());
                }
            }
        }
    } else if filetype == "zip" {
        let mut archive = zip::ZipArchive::new(archivefp).context("reading zip archive")?;
        let n = archive.len();
        for i in 0..n {
            let mut entry = archive.by_index(i).context("reading zip entry")?;
            let path = PathBuf::from(entry.name());
            if let Some(file_name) = path.file_name() {
                if file_name.to_str() == Some("nfpm") || file_name.to_str() == Some("nfpm.exe") {
                    let mut output_file =
                        File::create_new(&binary_path).context("opening output file")?;
                    std::io::copy(&mut entry, &mut output_file).context("writing nfpm to disk")?;
                    return Ok(());
                }
            }
        }
    } else {
        return Err(anyhow::anyhow!("unsupported filetype: {filetype}"));
    }

    Err(anyhow::anyhow!("nfpm binary not found"))
}

fn find_hash_for<S>(needle: S) -> anyhow::Result<Vec<u8>>
where
    S: AsRef<str> + std::fmt::Display,
{
    for line in CHECKSUMS.lines() {
        let Some((checksum, filename)) = line.split_once("  ") else {
            continue;
        };

        if filename == needle.as_ref() {
            return hex::decode(checksum).context("decoding hex hashsum to bytes");
        }
    }

    Err(anyhow::anyhow!("checksum not found for: {needle}"))
}

// SPDX-License-Identifier: BSD-3-Clause
// Copyright (c) 2026 Bo Lin

//! Runs generation through a pinned prebuilt `dbt-antlr-codegen` release
//! binary instead of the in-process generator.
//!
//! This module is an optimization for build environments where compiling
//! the generator (the `generator` feature) might be too expensive: the binary for
//! the build host is downloaded from the GitHub release matching a pinned
//! version, verified against a SHA-256 checksum, and invoked with the
//! configured grammar arguments.
//!
//! NOTE: `curl` on `PATH` is required.

use std::path::{Path, PathBuf};
use std::process::Command;

use sha2::Digest;

use crate::error::EmitError;

/// Default location of the project's GitHub release assets.
const DEFAULT_RELEASE_BASE_URL: &str = "https://github.com/sdf-labs/dbt-antlr/releases/download";

/// Host triples the release workflow ships prebuilt binaries for. A build
/// host outside this list must use the in-process generator instead.
const SUPPORTED_TRIPLES: &[&str] = &[
    "aarch64-apple-darwin",
    "aarch64-unknown-linux-gnu",
    "x86_64-apple-darwin",
    "x86_64-pc-windows-msvc",
    "x86_64-unknown-linux-gnu",
];

/// A pinned release to download the generator binary from.
#[derive(Clone, Debug)]
pub(crate) struct PinnedRelease {
    pub version: String,
    pub expected_sha256: Option<String>,
    pub base_url: Option<String>,
}

const fn exe_name() -> &'static str {
    if cfg!(windows) {
        "dbt-antlr-codegen.exe"
    } else {
        "dbt-antlr-codegen"
    }
}

fn archive_name(triple: &str) -> String {
    let extension = if triple.ends_with("windows-msvc") {
        "zip"
    } else {
        "tar.gz"
    };
    format!("dbt-antlr-codegen-{triple}.{extension}")
}

fn archive_url(base_url: &str, version: &str, triple: &str) -> String {
    format!(
        "{base_url}/dbt-antlr-codegen-v{version}/{name}",
        name = archive_name(triple)
    )
}

fn host_triple() -> Result<String, EmitError> {
    let triple = std::env::var("HOST").map_err(|_| {
        EmitError::Download(
            "HOST is not set; pinned-release mode must run from a cargo build script".to_owned(),
        )
    })?;
    if SUPPORTED_TRIPLES.contains(&triple.as_str()) {
        Ok(triple)
    } else {
        Err(EmitError::Download(format!(
            "no prebuilt dbt-antlr-codegen binary for host triple {triple}; \
             supported: {}. Use the in-process generator instead",
            SUPPORTED_TRIPLES.join(", ")
        )))
    }
}

fn sha256_hex(path: &Path) -> Result<String, EmitError> {
    let bytes = std::fs::read(path).map_err(|source| EmitError::Write {
        path: path.to_path_buf(),
        source,
    })?;
    Ok(format!("{:x}", sha2::Sha256::digest(&bytes)))
}

fn curl_download(url: &str, destination: &Path) -> Result<(), EmitError> {
    let status = Command::new("curl")
        .args(["--fail", "--location", "--retry", "3"])
        .arg("--output")
        .arg(destination)
        .arg(url)
        .status()
        .map_err(|error| {
            EmitError::Download(format!("failed to launch curl (is it on PATH?): {error}"))
        })?;
    if status.success() {
        Ok(())
    } else {
        Err(EmitError::Download(format!(
            "curl failed with {status} downloading {url}"
        )))
    }
}

/// Returns the expected checksum: the configured pin when set, otherwise the
/// checksum file published next to the archive.
fn expected_sha256(
    release: &PinnedRelease,
    url: &str,
    scratch: &Path,
) -> Result<String, EmitError> {
    if let Some(expected) = &release.expected_sha256 {
        return Ok(expected.to_ascii_lowercase());
    }
    let checksum_file = scratch.join("archive.sha256");
    curl_download(&format!("{url}.sha256"), &checksum_file)?;
    let content = std::fs::read_to_string(&checksum_file).map_err(|source| EmitError::Write {
        path: checksum_file.clone(),
        source,
    })?;
    content
        .split_whitespace()
        .next()
        .filter(|hex| hex.len() == 64 && hex.bytes().all(|b| b.is_ascii_hexdigit()))
        .map(str::to_ascii_lowercase)
        .ok_or_else(|| EmitError::Download(format!("malformed checksum file from {url}.sha256")))
}

#[cfg(not(windows))]
fn extract_binary(archive: &Path, destination: &Path) -> Result<(), EmitError> {
    let file = std::fs::File::open(archive).map_err(|source| EmitError::Write {
        path: archive.to_path_buf(),
        source,
    })?;
    let mut tar_archive = tar::Archive::new(flate2::read::GzDecoder::new(file));
    let entries = tar_archive
        .entries()
        .map_err(|error| EmitError::Download(format!("cannot read archive: {error}")))?;
    for entry in entries {
        let mut entry =
            entry.map_err(|error| EmitError::Download(format!("cannot read archive: {error}")))?;
        let path = entry
            .path()
            .map_err(|error| EmitError::Download(format!("cannot read archive: {error}")))?
            .into_owned();
        if path.file_name().is_some_and(|name| name == exe_name()) {
            entry
                .unpack(destination)
                .map_err(|error| EmitError::Download(format!("cannot extract binary: {error}")))?;
            use std::os::unix::fs::PermissionsExt;
            std::fs::set_permissions(destination, std::fs::Permissions::from_mode(0o755)).map_err(
                |source| EmitError::Write {
                    path: destination.to_path_buf(),
                    source,
                },
            )?;
            return Ok(());
        }
    }
    Err(EmitError::Download(format!(
        "archive {} contains no {}",
        archive.display(),
        exe_name()
    )))
}

#[cfg(windows)]
fn extract_binary(archive: &Path, destination: &Path) -> Result<(), EmitError> {
    let file = std::fs::File::open(archive).map_err(|source| EmitError::Write {
        path: archive.to_path_buf(),
        source,
    })?;
    let mut zip_archive = zip::ZipArchive::new(file)
        .map_err(|error| EmitError::Download(format!("cannot read archive: {error}")))?;
    for index in 0..zip_archive.len() {
        let mut entry = zip_archive
            .by_index(index)
            .map_err(|error| EmitError::Download(format!("cannot read archive: {error}")))?;
        if entry.name().ends_with(exe_name()) {
            let mut output =
                std::fs::File::create(destination).map_err(|source| EmitError::Write {
                    path: destination.to_path_buf(),
                    source,
                })?;
            std::io::copy(&mut entry, &mut output).map_err(|source| EmitError::Write {
                path: destination.to_path_buf(),
                source,
            })?;
            return Ok(());
        }
    }
    Err(EmitError::Download(format!(
        "archive {} contains no {}",
        archive.display(),
        exe_name()
    )))
}

/// Downloads (or reuses the cached copy of) the pinned binary and returns
/// its path.
fn ensure_binary(release: &PinnedRelease, out_dir: &Path) -> Result<PathBuf, EmitError> {
    let triple = host_triple()?;
    let stage = out_dir
        .join(".dbt-antlr-codegen-bin")
        .join(&release.version)
        .join(&triple);
    let binary = stage.join(exe_name());
    if binary.exists() {
        // We just trust local to never corrupt the binary. A potential pitfall
        // is if you manually changed the cached binary to something else (e.g.
        // for testing), then the script would keep invoking the modified binary
        // until either the cache is cleared or the pinned version is upgraded
        return Ok(binary);
    }
    std::fs::create_dir_all(&stage).map_err(|source| EmitError::Write {
        path: stage.clone(),
        source,
    })?;
    let base_url = release
        .base_url
        .as_deref()
        .unwrap_or(DEFAULT_RELEASE_BASE_URL);
    let url = archive_url(base_url, &release.version, &triple);
    let archive = stage.join(archive_name(&triple));
    curl_download(&url, &archive)?;
    let expected = expected_sha256(release, &url, &stage)?;
    let actual = sha256_hex(&archive)?;
    if actual != expected {
        return Err(EmitError::Download(format!(
            "checksum mismatch for {url}: expected {expected}, got {actual}"
        )));
    }
    extract_binary(&archive, &binary)?;
    std::fs::remove_file(&archive).ok();
    Ok(binary)
}

/// Runs the pinned binary over the configured grammars and returns the
/// generated file paths in `out_dir`.
pub(crate) fn generate_with_pinned_binary(
    release: &PinnedRelease,
    grammars: &[PathBuf],
    lib_dirs: &[PathBuf],
    gen_listener: bool,
    gen_visitor: bool,
    out_dir: &Path,
) -> Result<Vec<PathBuf>, EmitError> {
    let binary = ensure_binary(release, out_dir)?;
    let staging = out_dir.join(".dbt-antlr-codegen-out");
    std::fs::remove_dir_all(&staging).ok();
    std::fs::create_dir_all(&staging).map_err(|source| EmitError::Write {
        path: staging.clone(),
        source,
    })?;

    let mut command = Command::new(&binary);
    command.arg("-o").arg(&staging);
    for dir in lib_dirs {
        command.arg("-lib").arg(dir);
    }
    if gen_visitor {
        command.arg("-visitor");
    }
    if !gen_listener {
        command.arg("-no-listener");
    }
    command.args(grammars);
    let status = command.status().map_err(|error| {
        EmitError::Download(format!("failed to launch {}: {error}", binary.display()))
    })?;
    if !status.success() {
        return Err(EmitError::Download(format!(
            "{} exited with {status}",
            binary.display()
        )));
    }

    let mut written = Vec::new();
    let entries = std::fs::read_dir(&staging).map_err(|source| EmitError::Write {
        path: staging.clone(),
        source,
    })?;
    for entry in entries {
        let entry = entry.map_err(|source| EmitError::Write {
            path: staging.clone(),
            source,
        })?;
        let destination = out_dir.join(entry.file_name());
        std::fs::rename(entry.path(), &destination).map_err(|source| EmitError::Write {
            path: destination.clone(),
            source,
        })?;
        written.push(destination);
    }
    std::fs::remove_dir_all(&staging).ok();
    written.sort();
    Ok(written)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn archive_naming_matches_cargo_dist_layout() {
        assert_eq!(
            archive_name("x86_64-unknown-linux-gnu"),
            "dbt-antlr-codegen-x86_64-unknown-linux-gnu.tar.gz"
        );
        assert_eq!(
            archive_name("x86_64-pc-windows-msvc"),
            "dbt-antlr-codegen-x86_64-pc-windows-msvc.zip"
        );
        assert_eq!(
            archive_url(DEFAULT_RELEASE_BASE_URL, "0.1.0", "aarch64-apple-darwin"),
            "https://github.com/sdf-labs/dbt-antlr/releases/download/\
             dbt-antlr-codegen-v0.1.0/dbt-antlr-codegen-aarch64-apple-darwin.tar.gz"
        );
    }

    #[test]
    fn pinned_checksum_wins_over_download() {
        let release = PinnedRelease {
            version: "0.1.0".to_owned(),
            expected_sha256: Some("ABCDEF".repeat(10) + "abcd"),
            base_url: None,
        };
        let expected = expected_sha256(&release, "http://unused", Path::new("/tmp")).unwrap();
        assert_eq!(expected, "abcdef".repeat(10) + "abcd");
    }
}

//! Fetches and verifies the pinned upstream `dbt-antlr4` 2.0.0 release
//! artifacts (tool jar and runtime-testsuite resources) from GitHub.
//!
//! Both artifacts are cached under `<workspace>/target/dbt-antlr-runtime-testsuite-cache`
//! and verified against
//! the recorded SHA-256 checksums before use. Explicit `--antlr-jar` /
//! `--descriptors` flags or the `ANTLR4_JAR` / `ANTLR4_RUNTIME_TESTSUITE`
//! environment variables bypass this module entirely.

use std::fs;
use std::io;
use std::path::{Path, PathBuf};
use std::process::Command;

use sha2::{Digest, Sha256};

const TOOL_JAR_URL: &str = "https://github.com/sdf-labs/antlr4/releases/download/dbt-antlr4-2.0.0/dbt-antlr4-2.0.0-complete.jar";
const TOOL_JAR_SHA256: &str = "a83eac18b0d654c1fdac263ff0cd1c954a3a013ab9da30eee8c6c00a02ef65d2";
const SOURCE_ZIP_URL: &str =
    "https://github.com/sdf-labs/antlr4/archive/refs/tags/dbt-antlr4-2.0.0.zip";
const SOURCE_ZIP_SHA256: &str = "62583dc78e0d7f698bfe16331efd7f1e33f86df59d5bef73dfd31625756f322f";

/// Subtree of the source zip carrying the test descriptors and the target
/// action templates (`Rust.test.stg`).
const SOURCE_ZIP_PREFIX: &str = "antlr4-dbt-antlr4-2.0.0/runtime-testsuite/";

/// Returns the cached, checksum-verified ANTLR tool jar, downloading it from
/// the pinned GitHub release on first use.
pub(crate) fn ensure_tool_jar(workspace_root: &Path) -> Result<PathBuf, String> {
    let jar = cache_dir(workspace_root)?.join("dbt-antlr4-2.0.0-complete.jar");
    fetch_verified(TOOL_JAR_URL, TOOL_JAR_SHA256, &jar)?;
    Ok(jar)
}

/// Returns the cached, checksum-verified runtime-testsuite directory (holding
/// `resources/`) extracted from the pinned source zip.
pub(crate) fn ensure_runtime_testsuite(workspace_root: &Path) -> Result<PathBuf, String> {
    let cache = cache_dir(workspace_root)?;
    let root = cache.join("src");
    let marker = root.join(".source-zip-sha256");
    let testsuite = root.join("runtime-testsuite");
    let extracted = fs::read_to_string(&marker)
        .is_ok_and(|recorded| recorded.trim() == SOURCE_ZIP_SHA256)
        && testsuite.is_dir();
    if !extracted {
        let zip = cache.join("dbt-antlr4-2.0.0.zip");
        fetch_verified(SOURCE_ZIP_URL, SOURCE_ZIP_SHA256, &zip)?;
        if root.exists() {
            fs::remove_dir_all(&root)
                .map_err(|error| format!("clearing {}: {error}", root.display()))?;
        }
        extract_subtree(&zip, SOURCE_ZIP_PREFIX, &root)?;
        fs::write(&marker, SOURCE_ZIP_SHA256)
            .map_err(|error| format!("writing {}: {error}", marker.display()))?;
    }
    Ok(testsuite)
}

fn cache_dir(workspace_root: &Path) -> Result<PathBuf, String> {
    let cache = workspace_root.join("target/dbt-antlr-runtime-testsuite-cache");
    fs::create_dir_all(&cache).map_err(|error| format!("creating {}: {error}", cache.display()))?;
    Ok(cache)
}

/// Ensures `dest` holds the artifact at `url` with the recorded SHA-256,
/// (re)downloading when the cached copy is missing or does not verify.
fn fetch_verified(url: &str, sha256: &str, dest: &Path) -> Result<(), String> {
    if dest.is_file() && sha256_hex(dest).is_ok_and(|actual| actual == sha256) {
        return Ok(());
    }
    let tmp = dest.with_extension("download");
    let status = Command::new("curl")
        .arg("--fail")
        .arg("--location")
        .arg("--silent")
        .arg("--show-error")
        .arg("--output")
        .arg(&tmp)
        .arg(url)
        .status()
        .map_err(|error| format!("launching curl to download {url}: {error}"))?;
    if !status.success() {
        return Err(format!(
            "downloading {url} failed ({status}); pass an explicit path instead"
        ));
    }
    let actual = sha256_hex(&tmp)?;
    if actual != sha256 {
        return Err(format!(
            "checksum mismatch for {url}: expected {sha256}, got {actual}; refusing to use it"
        ));
    }
    fs::rename(&tmp, dest).map_err(|error| format!("storing {}: {error}", dest.display()))
}

fn sha256_hex(path: &Path) -> Result<String, String> {
    let mut file =
        fs::File::open(path).map_err(|error| format!("reading {}: {error}", path.display()))?;
    let mut hasher = Sha256::new();
    io::copy(&mut file, &mut hasher)
        .map_err(|error| format!("hashing {}: {error}", path.display()))?;
    Ok(format!("{:x}", hasher.finalize()))
}

/// Extracts the `prefix` subtree of `zip` into `dest`, stripping the leading
/// `antlr4-dbt-antlr4-2.0.0/` path component so `dest` holds
/// `runtime-testsuite/...` directly.
fn extract_subtree(zip: &Path, prefix: &str, dest: &Path) -> Result<(), String> {
    let file =
        fs::File::open(zip).map_err(|error| format!("reading {}: {error}", zip.display()))?;
    let mut archive = zip::ZipArchive::new(io::BufReader::new(file))
        .map_err(|error| format!("opening {}: {error}", zip.display()))?;
    for index in 0..archive.len() {
        let mut entry = archive
            .by_index(index)
            .map_err(|error| format!("reading {} entry {index}: {error}", zip.display()))?;
        let Some(name) = entry.enclosed_name() else {
            continue;
        };
        let (root, subtree) = prefix.split_once('/').unwrap_or((prefix, ""));
        let Some(relative) = name
            .strip_prefix(root)
            .ok()
            .filter(|relative| relative.starts_with(subtree.trim_end_matches('/')))
        else {
            continue;
        };
        if entry.is_dir() {
            continue;
        }
        let target = dest.join(relative);
        if let Some(parent) = target.parent() {
            fs::create_dir_all(parent)
                .map_err(|error| format!("creating {}: {error}", parent.display()))?;
        }
        let mut out = fs::File::create(&target)
            .map_err(|error| format!("creating {}: {error}", target.display()))?;
        io::copy(&mut entry, &mut out)
            .map_err(|error| format!("extracting {}: {error}", target.display()))?;
    }
    Ok(())
}

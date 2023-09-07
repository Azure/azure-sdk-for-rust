use crate::{ErrorKind, Result, ResultExt};
use camino::{Utf8Path, Utf8PathBuf};
use serde::Deserialize;
use std::io::BufRead;
use std::{
    fs::{self, File},
    io::BufReader,
    str::FromStr,
};

/// Get all directories below the given directory.
fn list_dirs_in(dir: impl AsRef<Utf8Path>) -> Result<Vec<Utf8PathBuf>> {
    let mut dirs = Vec::new();
    let paths = fs::read_dir(dir.as_ref())?;
    for path in paths.flatten() {
        if let Some(path) = Utf8Path::from_path(&path.path()) {
            if path.is_dir() && path.join("Cargo.toml").exists() {
                dirs.push(path.to_path_buf());
            }
        }
    }
    Ok(dirs)
}

pub fn list_dirs() -> Result<Vec<Utf8PathBuf>> {
    let mut names: Vec<_> = list_dirs_in("../mgmt")?.into_iter().collect();
    names.extend(list_dirs_in("../svc")?);
    names.sort();
    Ok(names)
}

pub fn list_crate_names() -> Result<Vec<String>> {
    let mut names: Vec<_> = list_dirs_in("../mgmt")?
        .into_iter()
        .filter_map(|d| d.file_name().map(|d| format!("azure_mgmt_{d}")))
        .collect();
    names.extend(
        list_dirs_in("../svc")?
            .into_iter()
            .filter_map(|d| d.file_name().map(|d| format!("azure_svc_{d}"))),
    );
    names.sort();
    Ok(names)
}

pub fn has_version(name: &str, version: &str) -> Result<bool> {
    Ok(get_versions(name)?.iter().any(|v| v.vers.as_str() == version))
}

/// Gets all the versions for a given crate
/// Expects https://github.com/rust-lang/crates.io-index to be cloned as a sibling directory.
fn get_versions(crate_name: &str) -> Result<Vec<CrateVersion>> {
    // all of these crates begin with "azure".
    let path = format!("../../../crates.io-index/az/ur/{crate_name}");
    let path = Utf8PathBuf::from_str(&path).map_kind(ErrorKind::Parse)?;
    let mut versions = Vec::new();
    if path.exists() {
        let file = File::open(path)?;
        let reader = BufReader::new(file);
        for line in reader.lines() {
            let version: CrateVersion = serde_json::from_str(&line?)?;
            versions.push(version);
        }
    }
    Ok(versions)
}

#[derive(Debug, Deserialize)]
pub struct CrateVersion {
    pub name: String,
    pub vers: String,
}

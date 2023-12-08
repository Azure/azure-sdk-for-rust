use crate::{ErrorKind, Result, ResultExt};
use camino::{Utf8Path, Utf8PathBuf};
use cargo_toml::Manifest;
use serde::Deserialize;
use std::{
    collections::BTreeSet,
    fs::{self, File},
    io::{BufRead, BufReader},
    path::Path,
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

pub fn list_crates(services_dir: &Path) -> Result<BTreeSet<String>> {
    let mut package_names = BTreeSet::new();
    let base_path = services_dir.join("Cargo.toml");
    let manifest = Manifest::from_path(base_path)?;
    if let Some(workspaces) = manifest.workspace {
        for member in workspaces.members {
            let member_path = services_dir.join(member).join("Cargo.toml");
            let Ok(manifest) = Manifest::from_path(member_path) else { continue };
            let Some(package) = manifest.package else {
                continue;
            };
            package_names.insert(package.name);
        }
    }
    Ok(package_names)
}

pub fn list_dirs() -> Result<Vec<Utf8PathBuf>> {
    let mut names: Vec<_> = list_dirs_in("../mgmt")?.into_iter().collect();
    names.extend(list_dirs_in("../svc")?);
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

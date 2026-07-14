#!/usr/bin/env -S cargo +nightly-2026-04-14 -Zscript
---
[package]
edition = "2021"
description = "In sample Cargo.toml files in the repo, update dependency versions from crate@version arguments."

[dependencies]
regex = "1.5"
toml_edit = "0.22"
---

use regex::Regex;
use std::collections::BTreeMap;
use std::io::{Error as IoError, ErrorKind, Write};
use std::{env, error::Error, fs, path::Path, path::PathBuf};
use toml_edit::{value, DocumentMut, Item, Table};

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let released_versions = parse_released_versions(env::args().skip(1))?;
    let script_root = PathBuf::from(env::var("CARGO_MANIFEST_DIR")?);
    let repo_root = script_root.join("../..").canonicalize()?;
    let sample_root = repo_root.join("samples");
    let toml_files = load_sample_toml_files(&sample_root)?;

    for path in toml_files {
        let content = fs::read_to_string(&path)?;
        let mut document = content.parse::<DocumentMut>()?;

        if update_dependency_versions(document.as_table_mut(), &released_versions) {
            let mut file = fs::File::create(path)?;
            fs::File::write_all(&mut file, document.to_string().as_bytes())?;
        }
    }

    Ok(())
}

fn parse_released_versions(
    args: impl Iterator<Item = String>,
) -> Result<BTreeMap<String, String>, Box<dyn Error>> {
    let mut released_versions = BTreeMap::new();

    for arg in args {
        let (crate_name, version) = arg.split_once('@').ok_or_else(|| {
            IoError::new(
                ErrorKind::InvalidInput,
                format!("invalid released version argument '{arg}'; expected crate@version"),
            )
        })?;

        if crate_name.is_empty() || version.is_empty() {
            return Err(IoError::new(
                ErrorKind::InvalidInput,
                format!("invalid released version argument '{arg}'; expected crate@version"),
            )
            .into());
        }

        released_versions.insert(crate_name.to_string(), version.to_string());
    }

    if released_versions.is_empty() {
        return Err(IoError::new(
            ErrorKind::InvalidInput,
            "requires at least one crate@version argument",
        )
        .into());
    }

    Ok(released_versions)
}

fn load_sample_toml_files(sample_root: &Path) -> Result<Vec<PathBuf>, Box<dyn Error>> {
    let mut toml_paths = Vec::new();
    find_cargo_toml_files(sample_root, &mut toml_paths)?;
    toml_paths.sort();
    Ok(toml_paths)
}

fn find_cargo_toml_files(dir: &Path, toml_paths: &mut Vec<PathBuf>) -> Result<(), Box<dyn Error>> {
    for entry in fs::read_dir(dir)? {
        let entry = entry?;
        let path = entry.path();

        if path.is_dir() {
            find_cargo_toml_files(&path, toml_paths)?;
        } else if path.is_file() && path.file_name() == Some("Cargo.toml".as_ref()) {
            toml_paths.push(path);
        }
    }

    Ok(())
}

fn update_dependency_versions(
    toml: &mut Table,
    released_versions: &BTreeMap<String, String>,
) -> bool {
    let dependency_tables = get_dependency_tables(toml);
    let mut updated = false;

    for (_, table) in dependency_tables {
        for (crate_name, version) in released_versions {
            if let Some(dependency) = table.get_mut(crate_name) {
                updated |= update_dependency_version(dependency, version);
            }
        }
    }

    updated
}

fn update_dependency_version(dependency: &mut Item, version: &str) -> bool {
    if dependency.get("version").is_some() {
        if dependency.get("version").and_then(Item::as_str) == Some(version) {
            return false;
        }

        dependency["version"] = value(version);
        return true;
    }

    if dependency.as_str() == Some(version) {
        return false;
    }

    if dependency.is_str() {
        *dependency = value(version);
        return true;
    }

    false
}

fn get_dependency_tables(toml: &mut Table) -> Vec<(String, &mut Table)> {
    let re = Regex::new(r"[.-]?dependencies$").unwrap();
    let mut tables = Vec::new();

    for (key, value) in toml.iter_mut() {
        if let Some(table) = value.as_table_mut() {
            if re.is_match(&key) {
                tables.push((key.to_string(), table));
            }
        }
    }

    tables
}

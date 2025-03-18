#!/usr/bin/env -S cargo +nightly -Zscript
---
[package]
edition = "2021"
description = "In all Cargo.toml files in the repo, for all dependencies that have both path and version properties, update the version property to the version in the package."

[dependencies]
regex = "1.5"
toml_edit = "^0.22"
---


use regex::Regex;
use std::io::Write;
use std::{env, error::Error, fs, path::PathBuf};
use toml_edit::{value, DocumentMut, Item, Table};

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let script_root = PathBuf::from(env::var("CARGO_MANIFEST_DIR")?);
    let repo_root = script_root.join("../../..").canonicalize()?;

    // find all Cargo.toml files in the repo_root directory
    let exclude_dirs = vec![
        repo_root.join("eng"),
        repo_root.join("target")
    ];

    let toml_files = load_cargo_toml_files(&repo_root, &exclude_dirs)?;

    let package_versions = get_package_versions(&toml_files);

    for (path, mut document) in toml_files {
        update_package_versions(document.as_table_mut(), &package_versions);
        // if the toml file has a workspace table, update the workspace table
        if let Some(workspace) = document.get_mut("workspace") {
            if let Some(table) = workspace.as_table_mut() {
                update_package_versions(table, &package_versions);
            }
        }

        // write the updated document back to the file
        let mut file = fs::File::create(&path)?;
        fs::File::write_all(&mut file, document.to_string().as_bytes())?;
    }

    Ok(())
}

fn load_cargo_toml_files(
    repo_root: &PathBuf,
    exclude_dirs: &Vec<PathBuf>,
) -> Result<Vec<(PathBuf, DocumentMut)>, Box<dyn Error>> {
    let mut toml_paths = Vec::new();
    find_cargo_toml_files(repo_root, exclude_dirs, &mut toml_paths)?;

    let mut toml_files = Vec::new();
    for path in toml_paths {
        let content = fs::read_to_string(&path)?;
        let doc = content.parse::<DocumentMut>()?;
        toml_files.push((path, doc));
    }

    Ok(toml_files)
}

fn find_cargo_toml_files(
    dir: &PathBuf,
    exclude_dirs: &Vec<PathBuf>,
    toml_paths: &mut Vec<PathBuf>,
) -> Result<(), Box<dyn Error>> {
    for entry in fs::read_dir(dir)? {
        let entry = entry?;
        let path = entry.path();
        if path.is_dir() && !exclude_dirs.contains(&path) {
            find_cargo_toml_files(&path, &exclude_dirs, toml_paths)?;
        } else if path.is_file() && path.file_name() == Some("Cargo.toml".as_ref()) {
            toml_paths.push(path);
        }
    }
    Ok(())
}

fn get_package_versions(toml_files: &Vec<(PathBuf, DocumentMut)>) -> Vec<(String, String)> {
    let mut package_versions = Vec::new();

    for (_, doc) in toml_files {
        if let Some(table) = doc.get("package").and_then(Item::as_table) {
            if let Some(name) = table.get("name").and_then(Item::as_str) {
                if let Some(version) = table.get("version").and_then(Item::as_str) {
                    package_versions.push((name.to_string(), version.to_string()));
                }
            }
        }
    }

    package_versions
}

fn update_package_versions(toml: &mut Table, package_versions: &Vec<(String, String)>) {
    let dependency_tables = get_dependency_tables(toml);
    // for each dependency table, for each package in package_versions
    // if the package is in the dependency table and the dependency has both path and version properties
    // update the version property to the version in package_versions
    for table in dependency_tables {
        for (package, version) in package_versions {
            if let Some(dependency) = table.get_mut(package) {
                if dependency.get("path").is_some() && dependency.get("version").is_some() {
                    dependency["version"] = value(version);
                }
            }
        }
    }
}

fn get_dependency_tables(toml: &mut Table) -> Vec<&mut Table> {
    let re = Regex::new(r"[.-]?dependencies$").unwrap();
    let mut tables = Vec::new();

    for (key, value) in toml.iter_mut() {
        if let Some(table) = value.as_table_mut() {
            if re.is_match(&key) {
                tables.push(table);
            }
        }
    }

    tables
}

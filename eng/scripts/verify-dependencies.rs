#!/usr/bin/env -S cargo +nightly -Zscript
---
[package]
edition = "2021"

[dependencies]
cargo-util-schemas = "0.1.0"
serde = { version = "1.0.197", features = ["derive"] }
serde_json = "1.0.114"
toml = "0.8.10"
---

use cargo_util_schemas::manifest::{InheritableDependency, TomlDependency, TomlManifest};
use serde::Deserialize;
use std::{
    io::Read,
    path::PathBuf,
    process::{Command, Stdio},
};

fn main() {
    let manifest_path = std::env::args()
        .nth(1)
        .or_else(|| {
            find_file(
                std::env::current_dir().expect("current directory"),
                "Cargo.toml",
            )
        })
        .expect("manifest path");

    let package_manifest_path = package_manifest_path(&manifest_path);
    let workspace_manifest_path = workspace_manifest_path(&manifest_path);

    eprintln!("Checking {}", package_manifest_path.display());
    let package_manifest_content =
        std::fs::read_to_string(package_manifest_path).expect("read package manifest");
    let package_manifest: TomlManifest =
        toml::from_str(&package_manifest_content).expect("deserialize package manifest");
    let Some(dependencies) = package_manifest.dependencies else {
        eprintln!("No package dependencies");
        return;
    };

    let mut dependencies: Vec<String> = dependencies
        .into_iter()
        .filter_map(|v| match v.1 {
            InheritableDependency::Value(dep) => match dep {
                TomlDependency::Simple(_) => Some(v.0.to_string()),
                TomlDependency::Detailed(details) if details.path.is_none() => {
                    Some(v.0.to_string())
                }
                _ => None,
            },
            InheritableDependency::Inherit(_) => None,
        })
        .collect();

    if !dependencies.is_empty() {
        dependencies.sort();
        println!(
            "The following dependencies do not inherit from workspace {}:\n",
            workspace_manifest_path.display()
        );
        println!("* {}\n", dependencies.join("\n* "));
        println!("Add dependencies to workspace and change the package dependency to `{{ workspace = true }}`.");
        println!("See https://doc.rust-lang.org/cargo/reference/specifying-dependencies.html#inheriting-a-dependency-from-a-workspace for more information.");

        std::process::exit(1);
    }
}

fn package_manifest_path(manifest_path: &str) -> PathBuf {
    let mut cmd = Command::new("cargo")
        .args(&["read-manifest", "--manifest-path", manifest_path])
        .stdout(Stdio::piped())
        .spawn()
        .expect("executing cargo read-manifest");

    let path: PathBuf = read_manifest(&mut cmd)
        .manifest_path
        .expect("manifest_path")
        .into();

    if !path.exists() {
        panic!("package manifest not found");
    }

    path
}

fn workspace_manifest_path(manifest_path: &str) -> PathBuf {
    let mut cmd = Command::new("cargo")
        .args(&[
            "metadata",
            "--format-version",
            "1",
            "--all-features",
            "--manifest-path",
            manifest_path,
        ])
        .stdout(Stdio::piped())
        .spawn()
        .expect("executing cargo metadata");

    let path: PathBuf = read_manifest(&mut cmd)
        .workspace_root
        .expect("workspace_root")
        .into();
    let path = path.join("Cargo.toml");

    if !path.exists() {
        panic!("workspace manifest not found");
    }

    path
}

fn find_file(dir: impl AsRef<std::path::Path>, name: &str) -> Option<String> {
    for dir in dir.as_ref().ancestors() {
        let path = dir.join(name);
        if path.exists() {
            return Some(path.to_str().unwrap().into());
        }
    }
    None
}

fn read_manifest(cmd: &mut std::process::Child) -> CargoManifest {
    let mut reader = std::io::BufReader::new(cmd.stdout.take().expect("buffering stdout"));

    let mut content: String = Default::default();
    reader.read_to_string(&mut content).expect("reading stdout");

    serde_json::from_str::<CargoManifest>(&content).expect("deserializing manifest")
}

#[derive(Deserialize)]
struct CargoManifest {
    pub manifest_path: Option<String>,
    pub workspace_root: Option<String>,
}


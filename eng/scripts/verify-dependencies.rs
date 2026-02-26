#!/usr/bin/env -S cargo +nightly -Zscript
---
[package]
edition = "2021"

[dependencies]
cargo-util-schemas = "0.11.0"
serde = { version = "1.0.197", features = ["derive"] }
serde_json = "1.0.114"
toml = "1.0.1"
---

use cargo_util_schemas::manifest::{InheritableDependency, TomlDependency, TomlManifest};
use serde::Deserialize;
use std::{
    path::{Path, PathBuf},
    process::Command,
};

static EXEMPTIONS: &[(&str, &str)] = &[
    ("azure_core", "http"),
    ("azure_core", "json-patch"),
    ("azure_core", "ureq"),
    ("azure_core_test", "dotenvy"),
    ("azure_canary", "serde"),
    ("azure_data_cosmos_native", "cbindgen"),
    // Temporary: Allow azure_data_cosmos to release depending on the latest release of azure_core and azure_identity
    ("azure_data_cosmos_native", "azure_core"),
    ("azure_data_cosmos", "azure_core"),
    ("azure_data_cosmos", "azure_identity"),
];

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
    let workspace_manifest_path = workspace_manifest_path();
    let packages = if package_manifest_path == workspace_manifest_path {
        workspace_packages(&workspace_manifest_path)
    } else {
        vec![package_manifest_path]
    };

    let mut found = false;
    for ref package_manifest_path in packages {
        eprintln!("Checking {}", package_manifest_path.display());
        let package_manifest_content =
            std::fs::read_to_string(package_manifest_path).expect("read package manifest");
        let package_manifest: TomlManifest =
            toml::from_str(&package_manifest_content).expect("deserialize package manifest");

        // Collect all package dependencies including in platform targets.
        let mut all_dependencies = vec![
            (
                "dependencies".to_string(),
                package_manifest.dependencies.as_ref(),
            ),
            (
                "dev-dependencies".to_string(),
                package_manifest.dev_dependencies(),
            ),
            (
                "build-dependencies".to_string(),
                package_manifest.build_dependencies(),
            ),
        ];
        if let Some(targets) = package_manifest.target.as_ref() {
            for (target, platform) in targets {
                all_dependencies.push((
                    format!("target.'{}'.dependencies", target),
                    platform.dependencies.as_ref(),
                ));
                all_dependencies.push((
                    format!("target.'{}'.dev-dependencies", target),
                    platform.dev_dependencies(),
                ));
                all_dependencies.push((
                    format!("target.'{}'.build-dependencies", target),
                    platform.build_dependencies(),
                ));
            }
        }

        let mut dependencies: Vec<Package> = all_dependencies
            .into_iter()
            .filter_map(|v| {
                if let Some(dependencies) = v.1 {
                    return Some((v.0, dependencies));
                }
                None
            })
            .flat_map(|v| std::iter::repeat(v.0).zip(v.1.iter()))
            .filter_map(|v| match v.1 .1 {
                InheritableDependency::Value(dep) => match dep {
                    TomlDependency::Simple(_) => Some(Package {
                        section: v.0,
                        name: v.1 .0.to_string(),
                    }),
                    TomlDependency::Detailed(details) if details.path.is_none() => Some(Package {
                        section: v.0,
                        name: v.1 .0.to_string(),
                    }),
                    _ => None,
                },
                InheritableDependency::Inherit(_) => None,
            })
            .filter(|v| {
                package_manifest.package.as_ref().is_some_and(|package| {
                    !EXEMPTIONS
                        .contains(&(package.name.as_ref().expect("REASON").as_str(), &v.name))
                })
            })
            .collect();

        if !dependencies.is_empty() {
            dependencies.sort();
            println!(
                "The following `{}` dependencies do not inherit from workspace `{}`:\n",
                package_manifest_path.display(),
                workspace_manifest_path.display(),
            );
            println!(
                "* {}\n",
                dependencies
                    .into_iter()
                    .map(|v| v.to_string())
                    .collect::<Vec<String>>()
                    .join("\n* ")
            );
            println!("Add dependencies to workspace and change the package dependency to `{{ workspace = true }}`.");
            println!("See <https://doc.rust-lang.org/cargo/reference/specifying-dependencies.html#inheriting-a-dependency-from-a-workspace> for more information.");
            println!();

            found = true;
        }
    }

    if found {
        std::process::exit(1);
    }
}

fn package_manifest_path(manifest_path: &str) -> PathBuf {
    let output = Command::new("cargo")
        .args([
            "locate-project",
            "--message-format",
            "plain",
            "--manifest-path",
            manifest_path,
        ])
        .output()
        .expect("executing cargo locate-project");

    let path: PathBuf = String::from_utf8(output.stdout)
        .expect("valid path")
        .trim_end()
        .into();

    if !path.exists() {
        panic!("package manifest not found");
    }

    path
}

fn workspace_manifest_path() -> PathBuf {
    let output = Command::new("cargo")
        .args(["locate-project", "--message-format", "plain", "--workspace"])
        .output()
        .expect("executing cargo locate-project");

    let path: PathBuf = String::from_utf8(output.stdout)
        .expect("valid path")
        .trim_end()
        .into();

    if !path.exists() {
        panic!("workspace manifest not found");
    }

    path
}

fn workspace_packages(manifest_path: &Path) -> Vec<PathBuf> {
    let output = Command::new("cargo")
        .args([
            "metadata",
            "--format-version",
            "1",
            "--no-deps",
            "--manifest-path",
            &manifest_path.to_string_lossy(),
        ])
        .output()
        .expect("executing cargo metadata");
    let manifest: Manifest =
        serde_json::from_slice(&output.stdout).expect("bad workspace metadata");

    let mut paths = Vec::with_capacity(manifest.packages.len());
    for package in manifest.packages {
        paths.push(package.manifest_path.into());
    }

    paths
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

#[derive(Deserialize)]
struct Manifest {
    packages: Vec<ManifestPackage>,
}

#[derive(Deserialize)]
struct ManifestPackage {
    manifest_path: String,
}

#[derive(PartialEq, Eq, PartialOrd, Ord)]
struct Package {
    name: String,
    section: String,
}

impl std::fmt::Display for Package {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{} ({})", self.name, self.section)
    }
}

#!/usr/bin/env -S cargo +nightly-2026-04-14 -Zscript
---
[package]
edition = "2021"

[dependencies]
cargo-util-schemas = "0.11.0"
serde = { version = "1.0.197", features = ["derive"] }
serde_json = "1.0.114"
toml = "1.0.1"
---

//! This cargo script helps maintain that most crates should take a workspace dependency on core.
//!
//! This is congruent with most other Azure SDK languages that use centralized dependency management.
//! Whenever a service crates ships, it should depend on the latest azure_core.
//! That is much easier to manage centrally. Service crates can take a path dependency on azure_core
//! when changes are needed but must switch back to a workspace dependency once that new version is released.
//!
//! Service crates can also take version dependencies on other crates in their service directory e.g., sdk/cosmos.
//! This is useful in cases where a crate is not expected to change often and you always want to dependency on the released version.
//! It is up to service owners to update those references when they ship a new version of that dependency.

use cargo_util_schemas::manifest::{InheritableDependency, TomlDependency, TomlManifest};
use serde::Deserialize;
use std::{
    collections::HashMap,
    path::{Path, PathBuf},
    process::Command,
};

static EXEMPTIONS: &[(&str, &str)] = &[
    ("azure_core", "http"),
    ("azure_core", "json-patch"),
    ("azure_core", "ureq"),
    ("azure_core_test", "dotenvy"),
    ("azure_canary", "serde"),
    ("azure_data_cosmos_driver_native", "cbindgen"),
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

    // Always collect workspace packages to build the crate-name → service-directory map.
    let all_workspace_packages = workspace_packages(&workspace_manifest_path);

    let packages: Vec<PathBuf> = if package_manifest_path == workspace_manifest_path {
        all_workspace_packages
            .values()
            .map(|p| PathBuf::from(&p.manifest_path))
            .collect()
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
                    format!("target.'{target}'.dependencies"),
                    platform.dependencies.as_ref(),
                ));
                all_dependencies.push((
                    format!("target.'{target}'.dev-dependencies"),
                    platform.dev_dependencies(),
                ));
                all_dependencies.push((
                    format!("target.'{target}'.build-dependencies"),
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
                    let package_name = package.name.as_ref().unwrap().as_str();
                    if EXEMPTIONS.contains(&(package_name, &v.name)) {
                        return false;
                    }
                    // Allow version deps on other crates within the same service directory.
                    let pkg_service_dir = all_workspace_packages
                        .get(package_name)
                        .and_then(|p| p.service_dir.as_ref());
                    let dep_service_dir = all_workspace_packages
                        .get(v.name.as_str())
                        .and_then(|p| p.service_dir.as_ref());
                    !matches!((pkg_service_dir, dep_service_dir), (Some(a), Some(b)) if a == b)
                })
            })
            .collect();

        if !dependencies.is_empty() {
            dependencies.sort();
            println!(
                "The following `{}` dependencies do not inherit from workspace `{}` or reference via `path`:\n",
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
            println!("Add dependencies to the workspace and change the package dependency to `{{ workspace = true }}`, or use a `path` dependency if changes to the dependency are required.");
            println!("See <https://github.com/Azure/azure-sdk-for-rust/blob/main/CONTRIBUTING.md#versions> for more information.");
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

fn workspace_packages(manifest_path: &Path) -> HashMap<String, ManifestPackage> {
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

    let workspace_root = PathBuf::from(&manifest.workspace_root);
    manifest
        .packages
        .into_iter()
        .map(|mut p| {
            let manifest_path = PathBuf::from(&p.manifest_path);
            let relative = manifest_path
                .strip_prefix(&workspace_root)
                .unwrap_or(&manifest_path);
            p.service_dir = service_dir(relative);
            (p.name.clone(), p)
        })
        .collect()
}

fn service_dir(manifest_path: &Path) -> Option<PathBuf> {
    // manifest_path must be workspace-relative. If the first component is "sdk",
    // return "sdk/<service>" as the service directory; otherwise return None to
    // indicate the crate is outside the sdk/ tree and should be ignored.
    let mut components = manifest_path.components();
    if components.next()?.as_os_str() != "sdk" {
        return None;
    }
    components.next().map(|service| {
        let mut path = PathBuf::from("sdk");
        path.push(service);
        path
    })
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
    workspace_root: String,
    packages: Vec<ManifestPackage>,
}

#[derive(Deserialize)]
struct ManifestPackage {
    name: String,
    manifest_path: String,
    #[serde(skip)]
    service_dir: Option<PathBuf>,
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

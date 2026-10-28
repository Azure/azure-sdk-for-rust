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
    collections::{HashMap, HashSet},
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
    (
        "azure_messaging_eventhubs_checkpointstore_blob",
        "azure_storage_blob",
    ),
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

    let workspace_manifest: TomlManifest = toml::from_str(
        &std::fs::read_to_string(&workspace_manifest_path).expect("read workspace manifest"),
    )
    .expect("deserialize workspace manifest");
    let mut found = !workspace_core_defaults_disabled(&workspace_manifest);
    if found {
        eprintln!("Workspace azure_core must set `default-features = false`; HTTP libraries enable `azure_core/default` through their own default feature.");
    }

    let packages: Vec<PathBuf> = if package_manifest_path == workspace_manifest_path {
        all_workspace_packages
            .values()
            .map(|p| PathBuf::from(&p.manifest_path))
            .collect()
    } else {
        vec![package_manifest_path]
    };

    for ref package_manifest_path in packages {
        eprintln!("Checking {}", package_manifest_path.display());
        let package = all_workspace_packages
            .values()
            .find(|package| Path::new(&package.manifest_path) == package_manifest_path)
            .expect("package in workspace metadata");
        for error in core_feature_errors(package, &all_workspace_packages) {
            eprintln!("{}: {error}", package_manifest_path.display());
            found = true;
        }
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

fn workspace_core_defaults_disabled(manifest: &TomlManifest) -> bool {
    let core = manifest
        .workspace
        .as_ref()
        .and_then(|workspace| workspace.dependencies.as_ref())
        .and_then(|dependencies| dependencies.get("azure_core"));
    matches!(
        core,
        Some(TomlDependency::Detailed(details))
            if details.default_features() == Some(false)
                && !details.features.as_ref().is_some_and(|features| features.iter().any(|feature| feature == "default"))
    )
}

fn core_feature_errors(
    package: &ManifestPackage,
    workspace_packages: &HashMap<String, ManifestPackage>,
) -> Vec<String> {
    if package.publish.as_ref().is_some_and(Vec::is_empty) {
        return Vec::new();
    }

    let mut errors = Vec::new();
    for dependency in package
        .dependencies
        .iter()
        .filter(|dependency| dependency.kind.is_none())
    {
        let is_core = dependency.name == "azure_core";
        let is_http_library = workspace_packages
            .get(&dependency.name)
            .is_some_and(|package| forwards_core_defaults(package, workspace_packages));
        if (is_core || is_http_library)
            && (dependency.uses_default_features
                || dependency
                    .features
                    .iter()
                    .any(|feature| feature == "default"))
        {
            errors.push(format!(
                "{} must disable dependency defaults with `default-features = false` and not enable `default` unconditionally",
                dependency.name,
            ));
        }
        if is_core
            && !uses_custom_transport(&package.name)
            && !forwards_dependency_defaults(package, dependency)
        {
            errors.push(format!(
                "HTTP libraries must include `{}/default` in `[features].default`",
                dependency.rename.as_ref().unwrap_or(&dependency.name),
            ));
        }
    }
    errors
}

fn forwards_core_defaults(
    package: &ManifestPackage,
    workspace_packages: &HashMap<String, ManifestPackage>,
) -> bool {
    forwards_core_defaults_inner(package, workspace_packages, &mut HashSet::new())
}

fn forwards_core_defaults_inner(
    package: &ManifestPackage,
    workspace_packages: &HashMap<String, ManifestPackage>,
    visited: &mut HashSet<String>,
) -> bool {
    if !visited.insert(package.name.clone()) {
        return false;
    }

    package
        .dependencies
        .iter()
        .filter(|dependency| dependency.kind.is_none())
        .any(|dependency| {
            if !forwards_dependency_defaults(package, dependency) {
                return false;
            }

            dependency.name == "azure_core"
                || workspace_packages
                    .get(&dependency.name)
                    .is_some_and(|dependency_package| {
                        forwards_core_defaults_inner(
                            dependency_package,
                            workspace_packages,
                            visited,
                        )
                    })
        })
}

fn forwards_dependency_defaults(
    package: &ManifestPackage,
    dependency: &ManifestDependency,
) -> bool {
    let feature = format!(
        "{}/default",
        dependency.rename.as_ref().unwrap_or(&dependency.name)
    );
    package
        .features
        .get("default")
        .is_some_and(|defaults| defaults.contains(&feature))
}

fn uses_custom_transport(package_name: &str) -> bool {
    // Cosmos owns its HTTP/TLS selection; AMQP libraries do not need Core's HTTP transport.
    matches!(
        package_name,
        "azure_data_cosmos"
            | "azure_data_cosmos_driver"
            | "azure_core_amqp"
            | "azure_messaging_eventhubs"
            | "azure_messaging_servicebus"
    )
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
    publish: Option<Vec<String>>,
    dependencies: Vec<ManifestDependency>,
    features: HashMap<String, Vec<String>>,
    #[serde(skip)]
    service_dir: Option<PathBuf>,
}

#[derive(Deserialize)]
struct ManifestDependency {
    name: String,
    rename: Option<String>,
    kind: Option<String>,
    uses_default_features: bool,
    features: Vec<String>,
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

#[cfg(test)]
mod tests {
    use super::{
        core_feature_errors, forwards_core_defaults, workspace_core_defaults_disabled,
        ManifestPackage, TomlManifest,
    };
    use std::collections::HashMap;

    fn package(name: &str, dependency_name: &str, defaults: bool) -> ManifestPackage {
        serde_json::from_value(serde_json::json!({
            "name": name,
            "manifest_path": format!("/sdk/{name}/Cargo.toml"),
            "publish": null,
            "dependencies": [{
                "name": dependency_name,
                "rename": null,
                "kind": null,
                "uses_default_features": defaults,
                "features": [],
            }],
            "features": {"default": [format!("{dependency_name}/default")]},
        }))
        .expect("package metadata")
    }

    #[test]
    fn workspace_disables_core_defaults() {
        for (dependency, valid) in [
            (r#"{ version = "1.1.0", default-features = false }"#, true),
            (r#"{ version = "1.1.0" }"#, false),
            (r#"{ version = "1.1.0", default-features = true }"#, false),
            (r#""1.1.0""#, false),
            (
                r#"{ version = "1.1.0", default-features = false, features = ["default"] }"#,
                false,
            ),
        ] {
            let manifest: TomlManifest = toml::from_str(&format!(
                "[workspace.dependencies]\nazure_core = {dependency}\n"
            ))
            .expect("workspace manifest");
            assert_eq!(workspace_core_defaults_disabled(&manifest), valid);
        }
    }

    #[test]
    fn http_defaults_are_opt_in() {
        let mut client = package("client", "azure_core", false);
        assert!(core_feature_errors(&client, &HashMap::new()).is_empty());
        client.dependencies[0].uses_default_features = true;
        assert_eq!(core_feature_errors(&client, &HashMap::new()).len(), 1);
        client.features.clear();
        assert_eq!(core_feature_errors(&client, &HashMap::new()).len(), 2);
    }

    #[test]
    fn explicitly_enabling_dependency_default_is_rejected() {
        let mut client = package("client", "azure_core", false);
        client.dependencies[0].features.push("default".to_string());
        assert_eq!(core_feature_errors(&client, &HashMap::new()).len(), 1);
    }

    #[test]
    fn transitive_http_defaults_are_rejected() {
        let mut client = package("client", "wrapper", true);
        let wrapper = package("wrapper", "http_library", false);
        let library = package("http_library", "azure_core", false);
        let packages = HashMap::from([
            (wrapper.name.clone(), wrapper),
            (library.name.clone(), library),
        ]);
        assert_eq!(core_feature_errors(&client, &packages).len(), 1);
        client.dependencies[0].uses_default_features = false;
        assert!(core_feature_errors(&client, &packages).is_empty());
    }

    #[test]
    fn cyclic_default_features_do_not_forward_core_defaults() {
        let client = package("client", "wrapper", false);
        let wrapper = package("wrapper", "http_library", false);
        let library = package("http_library", "wrapper", false);
        let packages = HashMap::from([
            (wrapper.name.clone(), wrapper),
            (library.name.clone(), library),
        ]);
        assert!(!forwards_core_defaults(
            packages.get("wrapper").expect("wrapper"),
            &packages
        ));
        assert!(core_feature_errors(&client, &packages).is_empty());
    }

    #[test]
    fn custom_transports_still_disable_core_defaults() {
        for name in [
            "azure_data_cosmos",
            "azure_data_cosmos_driver",
            "azure_core_amqp",
            "azure_messaging_eventhubs",
            "azure_messaging_servicebus",
        ] {
            let mut client = package(name, "azure_core", false);
            client.features.clear();
            assert!(core_feature_errors(&client, &HashMap::new()).is_empty());
            client.dependencies[0].uses_default_features = true;
            assert_eq!(core_feature_errors(&client, &HashMap::new()).len(), 1);
        }
    }

    #[test]
    fn unpublished_helpers_and_dev_dependencies_are_not_constrained() {
        let mut client = package("test_helper", "azure_core", true);
        client.publish = Some(Vec::new());
        client.features.clear();
        assert!(core_feature_errors(&client, &HashMap::new()).is_empty());
        client.publish = None;
        client.dependencies[0].kind = Some("dev".to_string());
        assert!(core_feature_errors(&client, &HashMap::new()).is_empty());
    }

    #[test]
    fn renamed_and_target_dependencies_are_checked() {
        let mut client = package("client", "azure_core", false);
        client.dependencies[0].rename = Some("core".to_string());
        assert!(!forwards_core_defaults(&client, &HashMap::new()));
        client
            .features
            .insert("default".to_string(), vec!["core/default".to_string()]);
        assert!(core_feature_errors(&client, &HashMap::new()).is_empty());

        // Cargo metadata includes target dependencies in the same dependency list.
        let mut target_dependency = package("client", "azure_core", true);
        client
            .dependencies
            .append(&mut target_dependency.dependencies);
        assert_eq!(core_feature_errors(&client, &HashMap::new()).len(), 2);
        client
            .features
            .get_mut("default")
            .expect("defaults")
            .push("azure_core/default".to_string());
        assert_eq!(core_feature_errors(&client, &HashMap::new()).len(), 1);
    }
}

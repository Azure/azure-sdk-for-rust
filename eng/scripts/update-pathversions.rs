#!/usr/bin/env -S cargo +nightly-2026-04-14 -Zscript
---
[package]
edition = "2021"
description = "In all Cargo.toml files in the repo, for all dependencies that have both path and version properties, update the version property to the version in the package."

[dependencies]
regex = "1.5"
semver = "1"
serde_json = "1"
toml_edit = { version = "0.22", features = ["serde"] }
---

// Copyright (c) Microsoft Corporation. All rights reserved.
// Licensed under the MIT License.

// cspell:ignore Zscript
use regex::Regex;
use semver::{Version, VersionReq};
use serde_json::Value as JsonValue;
use std::{
    collections::{BTreeMap, BTreeSet},
    env,
    error::Error,
    fs,
    io::Write,
    path::{Path, PathBuf},
};
use toml_edit::{value, DocumentMut, Item, Table, TableLike, Value};

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let mode = env::args().nth(1);
    if matches!(
        mode.as_deref(),
        Some("release-inspect" | "release-dependencies")
    ) {
        let input = fs::read_to_string(env::args().nth(2).ok_or("requires input JSON path")?)?;
        let input: JsonValue = serde_json::from_str(&input)?;
        let output = if mode.as_deref() == Some("release-inspect") {
            release_inspect(&input)?
        } else {
            release_dependencies(&input)?
        };
        println!("{}", serde_json::to_string(&output)?);
        return Ok(());
    }
    let add_mode = env::args()
        .nth(1)
        .map(|arg| match arg.as_str() {
            "add" => true,
            "update" => false,
            _ => panic!("Invalid mode. Use 'add' or 'update'."),
        })
        .expect("requires 'add' or 'update' mode argument");

    let script_root = PathBuf::from(env::var("CARGO_MANIFEST_DIR")?);
    let repo_root = script_root.join("../..").canonicalize()?;

    // find all Cargo.toml files in the repo_root directory
    let exclude_dirs = vec![
        repo_root.join("eng"),
        // Sample manifests should stay on the latest stable published versions unless they
        // intentionally use a path dependency to highlight unpublished functionality.
        repo_root.join("samples"),
        repo_root.join("target"),
    ];

    let toml_files = load_cargo_toml_files(&repo_root, &exclude_dirs)?;

    let package_versions = get_package_versions(&toml_files);

    for mut toml_file in toml_files {
        let should_add = add_mode && !toml_file.is_publish_disabled;

        update_package_versions(
            toml_file.document.as_table_mut(),
            &package_versions,
            should_add,
        );

        // if the toml file has a workspace table, update the workspace table
        if let Some(workspace) = toml_file.document.get_mut("workspace") {
            if let Some(table) = workspace.as_table_mut() {
                update_package_versions(table, &package_versions, should_add);
            }
        }

        // write the updated document back to the file
        let mut file = fs::File::create(toml_file.path)?;
        fs::File::write_all(&mut file, toml_file.document.to_string().as_bytes())?;
    }

    Ok(())
}

fn release_inspect(input: &JsonValue) -> Result<JsonValue, Box<dyn Error>> {
    let mut output = serde_json::Map::new();
    for (id, source) in input
        .as_object()
        .ok_or("expected an object of TOML sources")?
    {
        let mut document = source
            .as_str()
            .ok_or("expected a TOML source string")?
            .parse::<DocumentMut>()?;
        if let Some(package) = document
            .get_mut("package")
            .and_then(Item::as_table_like_mut)
        {
            package.remove("version");
        }
        let parsed: JsonValue = toml_edit::de::from_str(&document.to_string())?;
        output.insert(id.clone(), parsed);
    }
    Ok(JsonValue::Object(output))
}

fn dependency_tables(table: &dyn TableLike) -> Vec<(bool, &dyn TableLike)> {
    let mut tables = Vec::new();
    for section in ["dependencies", "build-dependencies", "dev-dependencies"] {
        if let Some(dependencies) = table.get(section).and_then(Item::as_table_like) {
            tables.push((section == "dev-dependencies", dependencies));
        }
    }
    if let Some(targets) = table.get("target").and_then(Item::as_table_like) {
        for (_, target) in targets.iter() {
            if let Some(target) = target.as_table_like() {
                tables.extend(dependency_tables(target));
            }
        }
    }
    tables
}

fn dependency_package<'a>(alias: &'a str, dependency: &'a Item) -> &'a str {
    dependency
        .get("package")
        .and_then(Item::as_str)
        .unwrap_or(alias)
}

fn inherited(dependency: &Item) -> bool {
    dependency.get("workspace").and_then(Item::as_bool) == Some(true)
}

fn released_requirement(
    requirement: Option<&str>,
    target: &Version,
) -> Result<String, Box<dyn Error>> {
    let Some(requirement) = requirement else {
        return Ok(target.to_string());
    };
    let parsed = VersionReq::parse(requirement)?;
    // A concrete baseline may be lowered, but must never be raised.
    let baseline = requirement
        .trim()
        .trim_start_matches(['=', '^', '~'])
        .trim();
    if let Ok(baseline_version) = Version::parse(baseline) {
        if baseline_version > *target {
            return Ok(requirement.replacen(baseline, &target.to_string(), 1));
        }
    }
    if parsed.matches(target)
        && (!target.pre.is_empty()
            || parsed
                .comparators
                .iter()
                .all(|comparator| comparator.pre.is_empty()))
    {
        return Ok(requirement.to_string());
    }
    Err(format!("requirement {requirement:?} cannot use released version {target} without increasing or changing its compatibility").into())
}

fn rewrite_dependency(dependency: &mut Item, target: &Version) -> Result<(), Box<dyn Error>> {
    if ["git", "registry", "registry-index", "branch", "tag", "rev"]
        .iter()
        .any(|key| dependency.get(key).is_some())
    {
        return Err("unsupported dependency source in release plan".into());
    }
    let old = dependency
        .as_str()
        .or_else(|| dependency.get("version").and_then(Item::as_str));
    if old.is_none() && dependency.get("path").is_none() {
        return Err("dependency has neither a version nor a path".into());
    }
    let version = released_requirement(old, target)?;
    if let Some(string) = dependency.as_value_mut().filter(|v| v.is_str()) {
        if string.as_str() != Some(version.as_str()) {
            let decor = string.decor().clone();
            *string = Value::from(version);
            *string.decor_mut() = decor;
        }
    } else {
        let table = dependency
            .as_table_like_mut()
            .ok_or("expected a dependency table")?;
        let mut comments = String::new();
        if let Some(key) = table.key("path") {
            if let Some(prefix) = key.leaf_decor().prefix().and_then(|s| s.as_str()) {
                if prefix.contains('#') {
                    comments.push_str(prefix);
                }
            }
        }
        if let Some(suffix) = table
            .get("path")
            .and_then(Item::as_value)
            .and_then(|v| v.decor().suffix())
            .and_then(|s| s.as_str())
        {
            if suffix.contains('#') {
                comments.push_str(suffix.trim_start());
                comments.push('\n');
            }
        }
        table.remove("path");
        if table.get("version").and_then(Item::as_str) != Some(version.as_str()) {
            let decor = table
                .get("version")
                .and_then(Item::as_value)
                .map(|v| v.decor().clone());
            let mut replacement = Value::from(version);
            if let Some(decor) = decor {
                *replacement.decor_mut() = decor;
            }
            table.insert("version", Item::Value(replacement));
        }
        // Keep comments attached to the removed path on the remaining version entry.
        if !comments.is_empty() {
            if let Some(mut key) = table.key_mut("version") {
                let prefix = key
                    .leaf_decor()
                    .prefix()
                    .and_then(|s| s.as_str())
                    .unwrap_or("");
                let prefix = format!("{comments}{prefix}");
                key.leaf_decor_mut().set_prefix(prefix);
            }
        }
    }
    Ok(())
}

fn rewrite_dependencies(
    table: &mut dyn TableLike,
    versions: &BTreeMap<String, Version>,
    canonical_packages: &BTreeSet<String>,
) -> Result<(), Box<dyn Error>> {
    for section in ["dependencies", "build-dependencies", "dev-dependencies"] {
        if let Some(dependencies) = table.get_mut(section).and_then(Item::as_table_like_mut) {
            for (alias, dependency) in dependencies.iter_mut() {
                let package = dependency_package(&alias, dependency).to_string();
                if inherited(dependency) || !canonical_packages.contains(&package) {
                    continue;
                }
                if let Some(target) = versions.get(&package) {
                    rewrite_dependency(dependency, target)?;
                }
            }
        }
    }
    if let Some(targets) = table.get_mut("target").and_then(Item::as_table_like_mut) {
        for (_, target) in targets.iter_mut() {
            if let Some(target) = target.as_table_like_mut() {
                rewrite_dependencies(target, versions, canonical_packages)?;
            }
        }
    }
    Ok(())
}

fn release_dependencies(input: &JsonValue) -> Result<JsonValue, Box<dyn Error>> {
    let root = PathBuf::from(
        input
            .get("root")
            .and_then(JsonValue::as_str)
            .ok_or("requires root")?,
    );
    if !root.is_absolute() {
        return Err("root must be absolute".into());
    }
    let apply = input
        .get("apply")
        .and_then(JsonValue::as_bool)
        .ok_or("requires apply boolean")?;
    let versions = input
        .get("versions")
        .and_then(JsonValue::as_object)
        .ok_or("requires versions object")?
        .iter()
        .map(|(name, version)| {
            Ok((
                name.clone(),
                Version::parse(version.as_str().ok_or("requires exact semver string")?)?,
            ))
        })
        .collect::<Result<BTreeMap<_, _>, Box<dyn Error>>>()?;
    let mut paths = vec![root.join("Cargo.toml")];
    for service in fs::read_dir(root.join("sdk"))? {
        let service = service?.path();
        if !service.is_dir() {
            continue;
        }
        for package in fs::read_dir(service)? {
            let path = package?.path().join("Cargo.toml");
            if path.is_file() {
                paths.push(path);
            }
        }
    }
    paths[1..].sort();
    let mut documents = paths
        .iter()
        .map(|path| {
            let source = fs::read_to_string(path)?;
            let document = source.parse::<DocumentMut>()?;
            Ok((source, document))
        })
        .collect::<Result<Vec<_>, Box<dyn Error>>>()?;
    let report = plan_release_dependencies(&paths, &mut documents, &root, &versions)?;
    if apply {
        for (path, (source, document)) in paths.iter().zip(&documents) {
            let output = document.to_string();
            if output != *source {
                fs::write(path, output)?;
            }
        }
    }
    Ok(report)
}

fn plan_release_dependencies(
    paths: &[PathBuf],
    documents: &mut [(String, DocumentMut)],
    root: &Path,
    versions: &BTreeMap<String, Version>,
) -> Result<JsonValue, Box<dyn Error>> {
    let workspace = documents[0]
        .1
        .get("workspace")
        .and_then(|w| w.get("dependencies"))
        .and_then(Item::as_table_like)
        .map(|table| {
            table
                .iter()
                .map(|(key, item)| (key.to_string(), item.clone()))
                .collect::<BTreeMap<_, _>>()
        })
        .unwrap_or_default();
    let mut workspace_used = BTreeSet::new();
    let mut canonical_sets = Vec::new();
    for (_, document) in documents.iter() {
        let mut normal = BTreeSet::new();
        for (dev, dependencies) in dependency_tables(document.as_table()) {
            if !dev {
                for (alias, dependency) in dependencies.iter() {
                    let resolved = if inherited(dependency) {
                        workspace
                            .get(alias)
                            .ok_or_else(|| format!("missing workspace dependency {alias}"))?
                    } else {
                        dependency
                    };
                    normal.insert(dependency_package(alias, resolved).to_string());
                }
            }
        }
        let mut canonical = BTreeSet::new();
        for (_, dependencies) in dependency_tables(document.as_table()) {
            for (alias, dependency) in dependencies.iter() {
                let resolved = if inherited(dependency) {
                    workspace
                        .get(alias)
                        .ok_or_else(|| format!("missing workspace dependency {alias}"))?
                } else {
                    dependency
                };
                let package = dependency_package(alias, resolved);
                if normal.contains(package)
                    && versions.contains_key(package)
                    && resolved.get("path").is_some()
                {
                    canonical.insert(package.to_string());
                }
            }
        }
        for (_, dependencies) in dependency_tables(document.as_table()) {
            for (alias, dependency) in dependencies.iter() {
                if inherited(dependency) {
                    let resolved = workspace
                        .get(alias)
                        .ok_or_else(|| format!("missing workspace dependency {alias}"))?;
                    let package = dependency_package(alias, resolved);
                    if canonical.contains(package) {
                        workspace_used.insert(alias.to_string());
                    }
                }
            }
        }
        canonical_sets.push(canonical);
    }
    let mut workspace_changed = BTreeSet::new();
    let mut workspace_processed = BTreeSet::new();
    while !workspace_used.is_empty() {
        for alias in std::mem::take(&mut workspace_used) {
            let original = &workspace[&alias];
            let package = dependency_package(&alias, original);
            let dependency = &mut documents[0].1["workspace"]["dependencies"][&alias];
            rewrite_dependency(dependency, &versions[package])?;
            if dependency.to_string() != original.to_string() {
                workspace_changed.insert(alias.clone());
            }
            workspace_processed.insert(alias);
        }
        // A shared source change also constrains dev-only aliases in each inheritor.
        for ((_, document), canonical) in documents.iter().zip(&mut canonical_sets) {
            for (_, dependencies) in dependency_tables(document.as_table()) {
                for (alias, dependency) in dependencies.iter() {
                    if inherited(dependency) && workspace_changed.contains(alias) {
                        canonical.insert(dependency_package(alias, &workspace[alias]).to_string());
                    }
                }
            }
            for (_, dependencies) in dependency_tables(document.as_table()) {
                for (alias, dependency) in dependencies.iter() {
                    if inherited(dependency)
                        && !workspace_processed.contains(alias)
                        && canonical.contains(dependency_package(alias, &workspace[alias]))
                        && versions.contains_key(dependency_package(alias, &workspace[alias]))
                    {
                        workspace_used.insert(alias.to_string());
                    }
                }
            }
        }
    }
    let mut changed = Vec::new();
    let mut affected = BTreeSet::new();
    for ((path, (source, document)), canonical) in
        paths.iter().zip(documents.iter_mut()).zip(canonical_sets)
    {
        rewrite_dependencies(document.as_table_mut(), versions, &canonical)?;
        let output = document.to_string();
        let before: JsonValue = toml_edit::de::from_str(source)?;
        let after: JsonValue = toml_edit::de::from_str(&output)?;
        if before.get("package").and_then(|p| p.get("version"))
            != after.get("package").and_then(|p| p.get("version"))
        {
            return Err("release dependency rewrite changed package.version".into());
        }
        let inherits_change =
            dependency_tables(document.as_table())
                .iter()
                .any(|(_, dependencies)| {
                    dependencies.iter().any(|(alias, dependency)| {
                        inherited(dependency) && workspace_changed.contains(alias)
                    })
                });
        if output != *source {
            changed.push(
                path.strip_prefix(root)?
                    .to_string_lossy()
                    .replace('\\', "/"),
            );
        }
        if output != *source || inherits_change {
            if let Some(name) = document
                .get("package")
                .and_then(|p| p.get("name"))
                .and_then(Item::as_str)
            {
                affected.insert(name.to_string());
            }
        }
    }
    Ok(serde_json::json!({"changed_manifests": changed, "affected_packages": affected}))
}

fn load_cargo_toml_files(
    repo_root: &PathBuf,
    exclude_dirs: &Vec<PathBuf>,
) -> Result<Vec<TomlInfo>, Box<dyn Error>> {
    let mut toml_paths = Vec::new();
    find_cargo_toml_files(repo_root, exclude_dirs, &mut toml_paths)?;

    let mut toml_files = Vec::new();
    for path in toml_paths {
        let content = fs::read_to_string(&path)?;
        let doc = content.parse::<DocumentMut>()?;
        let package_table = doc.get("package").and_then(Item::as_table);
        let publish_property = package_table
            .and_then(|table| table.get("publish"))
            .and_then(Item::as_bool);
        let package_name = package_table
            .and_then(|table| table.get("name"))
            .and_then(Item::as_str);
        let package_version = package_table
            .and_then(|table| table.get("version"))
            .and_then(Item::as_str);

        toml_files.push(TomlInfo {
            path,
            package_name: package_name.map(|s| s.to_string()),
            package_version: package_version.map(|s| s.to_string()),
            is_publish_disabled: publish_property == Some(false),
            document: doc,
        });
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
            find_cargo_toml_files(&path, exclude_dirs, toml_paths)?;
        } else if path.is_file() && path.file_name() == Some("Cargo.toml".as_ref()) {
            toml_paths.push(path);
        }
    }

    Ok(())
}

fn get_package_versions(toml_files: &Vec<TomlInfo>) -> Vec<(String, String, bool)> {
    let mut package_versions = Vec::new();

    for toml_file in toml_files {
        if toml_file.package_name.is_none() || toml_file.package_version.is_none() {
            continue;
        }

        package_versions.push((
            toml_file.package_name.clone().unwrap(),
            toml_file.package_version.clone().unwrap(),
            toml_file.is_publish_disabled,
        ));
    }

    package_versions
}

fn update_package_versions(
    toml: &mut Table,
    package_versions: &Vec<(String, String, bool)>,
    add: bool,
) {
    // for each dependency table, for each package in package_versions
    // if the package is in the dependency table
    //   if the dependency has both path and version properties, update the version property
    //   if the dependency has has path, but not version, add the version property only if
    //     1. the table name is not "dev-dependencies"
    //     2. the package is not publish disabled
    //     3. the add flag is true

    let dependency_tables = get_dependency_tables(toml);

    for (table_name, table) in dependency_tables {
        for (package, version, is_publish_disabled) in package_versions {
            if let Some(dependency) = table.get_mut(package) {
                // azure_idenentity will only be a transitive dev-dependency
                let should_add = add
                    && table_name != "dev-dependencies"
                    && !is_publish_disabled
                    && package != "azure_identity";

                let has_path_property = dependency.get("path").is_some();
                let has_version_property = dependency.get("version").is_some();

                if has_path_property && (has_version_property || should_add) {
                    dependency["version"] = value(version);
                }
            }
        }
    }
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

struct TomlInfo {
    path: PathBuf,
    package_name: Option<String>,
    package_version: Option<String>,
    is_publish_disabled: bool,
    document: DocumentMut,
}

#[cfg(test)]
mod tests {
    use super::{
        plan_release_dependencies, release_inspect, released_requirement, update_package_versions,
    };
    use semver::{Version, VersionReq};
    use serde_json::{json, Value as JsonValue};
    use std::{collections::BTreeMap, path::PathBuf};
    use toml_edit::DocumentMut;

    fn release_plan(
        sources: &[&str],
    ) -> Result<(Vec<String>, JsonValue), Box<dyn std::error::Error>> {
        release_plan_for_version(sources, "1.2.0")
    }

    fn release_plan_for_version(
        sources: &[&str],
        target: &str,
    ) -> Result<(Vec<String>, JsonValue), Box<dyn std::error::Error>> {
        let root = PathBuf::from("/repo");
        let paths = sources
            .iter()
            .enumerate()
            .map(|(index, _)| {
                if index == 0 {
                    root.join("Cargo.toml")
                } else {
                    root.join(format!("sdk/test/consumer{index}/Cargo.toml"))
                }
            })
            .collect::<Vec<_>>();
        let mut documents = sources
            .iter()
            .map(|source| Ok((source.to_string(), source.parse::<DocumentMut>()?)))
            .collect::<Result<Vec<_>, toml_edit::TomlError>>()?;
        let versions = BTreeMap::from([("released".to_string(), Version::parse(target)?)]);
        let report = plan_release_dependencies(&paths, &mut documents, &root, &versions)?;
        Ok((
            documents
                .iter()
                .map(|(_, document)| document.to_string())
                .collect(),
            report,
        ))
    }

    #[test]
    fn inspect_removes_only_document_package_version() {
        let input = json!({
            "arbitrary/id": "[package]\nname = 'consumer'\nversion = '2.0.0'\n[dependencies]\nreleased = { version = '1', features = ['xml'] }\n[workspace.package]\nversion = '3.0.0'\n",
            "root": "[workspace.dependencies]\nreleased = { path = 'sdk/core/released', version = '1.0.0' }\n"
        });
        let output = release_inspect(&input).expect("inspect");
        assert_eq!(
            output["arbitrary/id"]["package"],
            json!({"name": "consumer"})
        );
        assert_eq!(
            output["arbitrary/id"]["dependencies"]["released"]["features"],
            json!(["xml"])
        );
        assert_eq!(
            output["arbitrary/id"]["workspace"]["package"]["version"],
            "3.0.0"
        );
        assert_eq!(
            output["root"]["workspace"]["dependencies"]["released"]["version"],
            "1.0.0"
        );
        assert!(release_inspect(&json!({"bad": "[broken"})).is_err());
    }

    #[test]
    fn requirements_never_increase() {
        let target = Version::parse("1.2.0").expect("version");
        for requirement in ["1", "^1.0.0", "1.0.0", "1.2.0", ">=1.0, <2"] {
            assert_eq!(
                released_requirement(Some(requirement), &target).expect("compatible"),
                requirement
            );
        }
        assert_eq!(
            released_requirement(None, &target).expect("path only"),
            "1.2.0"
        );
        assert_eq!(
            released_requirement(Some("2.0.0"), &target).expect("lowered"),
            "1.2.0"
        );
        for requirement in ["0.9.0", "=1.0.0", "~1.0.0", "invalid"] {
            assert!(
                released_requirement(Some(requirement), &target).is_err(),
                "{requirement}"
            );
        }
    }

    #[test]
    fn lowering_requirements_preserves_operators() {
        let target = Version::parse("1.2.0").expect("version");
        for (requirement, expected) in [
            ("=2.0.0", "=1.2.0"),
            ("~2.0.0", "~1.2.0"),
            ("^2.0.0", "^1.2.0"),
            (" ~ 2.0.0 ", " ~ 1.2.0 "),
        ] {
            assert_eq!(
                released_requirement(Some(requirement), &target).expect("lowered"),
                expected
            );
        }
    }

    #[test]
    fn stable_targets_reject_retained_preview_requirements() {
        let target = Version::parse("1.2.0").expect("version");
        for requirement in ["^1.2.0-beta.1", "~1.2.0-beta.1", ">=1.0.0-beta.1, <2"] {
            assert!(
                VersionReq::parse(requirement)
                    .expect("requirement")
                    .matches(&target),
                "{requirement}"
            );
            assert!(
                released_requirement(Some(requirement), &target).is_err(),
                "{requirement}"
            );
        }
        assert_eq!(
            released_requirement(Some("=1.3.0-beta.1"), &target).expect("lowered"),
            "=1.2.0"
        );
    }

    #[test]
    fn rewrites_alias_target_dotted_and_dev_canonical_sources() {
        let source = r#"[package]
name = "consumer"
version = "9.0.0"
publish = false
[dependencies]
alias = { package = "released", path = "../released", version = "2.0.0", features = ["xml"], default-features = false } # retain
[dev-dependencies]
released.path = "../released"
released.version = "2.0.0" # dev version
other = { path = "../other" }
[target.'cfg(unix)'.build-dependencies.released]
path = "../released"
version = "2.0.0" # table version
optional = true
"#;
        let (output, report) = release_plan(&["[workspace]\n", source]).expect("plan");
        let document = output[1].parse::<DocumentMut>().expect("manifest");
        assert_eq!(document["package"]["version"].as_str(), Some("9.0.0"));
        assert!(document["dependencies"]["alias"].get("path").is_none());
        assert_eq!(
            document["dependencies"]["alias"]["version"].as_str(),
            Some("1.2.0")
        );
        assert_eq!(
            document["dependencies"]["alias"]["default-features"].as_bool(),
            Some(false)
        );
        assert!(output[1].contains("features = [\"xml\"]"));
        assert!(output[1].contains("# retain"));
        assert!(output[1].contains("# dev version"));
        assert!(output[1].contains("# table version"));
        assert!(document["dev-dependencies"]["released"]
            .get("path")
            .is_none());
        assert_eq!(
            document["dev-dependencies"]["other"]["path"].as_str(),
            Some("../other")
        );
        assert!(
            document["target"]["cfg(unix)"]["build-dependencies"]["released"]
                .get("path")
                .is_none()
        );
        assert_eq!(report["affected_packages"], json!(["consumer"]));
        assert_eq!(
            report["changed_manifests"],
            json!(["sdk/test/consumer1/Cargo.toml"])
        );
    }

    #[test]
    fn workspace_effects_include_unchanged_inheritors() {
        let root = r#"[workspace.dependencies]
alias = { package = "released", path = "sdk/test/released", version = "2.0.0", features = ["xml"] } # shared
dev_only = { package = "released", path = "sdk/test/released" }
"#;
        let consumer = r#"[package]
name = "consumer"
version = "1.0.0"
[dependencies]
alias.workspace = true
[dev-dependencies]
released = { path = "../released" }
"#;
        let dev_consumer = r#"[package]
name = "dev_consumer"
version = "1.0.0"
[dev-dependencies]
alias = { workspace = true }
"#;
        let (output, report) = release_plan(&[root, consumer, dev_consumer]).expect("plan");
        let document = output[0].parse::<DocumentMut>().expect("root");
        assert!(document["workspace"]["dependencies"]["alias"]
            .get("path")
            .is_none());
        assert_eq!(
            document["workspace"]["dependencies"]["dev_only"]["path"].as_str(),
            Some("sdk/test/released")
        );
        assert!(output[0].contains("# shared"));
        assert_eq!(output[2], dev_consumer);
        assert_eq!(
            report["affected_packages"],
            json!(["consumer", "dev_consumer"])
        );
        assert_eq!(
            report["changed_manifests"],
            json!(["Cargo.toml", "sdk/test/consumer1/Cargo.toml"])
        );
        let sources = output.iter().map(String::as_str).collect::<Vec<_>>();
        let (_, second) = release_plan(&sources).expect("repeat");
        assert_eq!(
            second,
            json!({"changed_manifests": [], "affected_packages": []})
        );
    }

    #[test]
    fn inherited_dev_source_follows_direct_normal_dependency() {
        let root = "[workspace.dependencies]\nreleased = { path = 'sdk/test/released' }\n";
        let consumer = "[package]\nname = 'consumer'\nversion = '1.0.0'\n[dependencies]\nreleased = '1'\n[dev-dependencies]\nreleased.workspace = true\n";
        let (output, report) = release_plan(&[root, consumer]).expect("plan");
        assert!(!output[0].contains("path"));
        assert_eq!(output[1], consumer);
        assert_eq!(report["affected_packages"], json!(["consumer"]));
    }

    #[test]
    fn changed_workspace_sources_align_dev_only_aliases_across_targets() {
        let root = r#"[workspace.dependencies]
shared = { package = "released", path = "sdk/test/released" }
other_shared = { package = "released", path = "sdk/test/released" }
untouched = { package = "released", path = "sdk/test/released" }
"#;
        let consumer = "[package]\nname = 'consumer'\nversion = '1.0.0'\n[dependencies]\nshared.workspace = true\n";
        let dev_consumer = r#"[package]
name = "dev_consumer"
version = "1.0.0"
publish = false
[dev-dependencies]
shared.workspace = true
direct = { package = "released", path = "../released", features = ["xml"] }
[target.'cfg(unix)'.dev-dependencies]
other_shared.workspace = true
[target.'cfg(windows)'.dev-dependencies.local]
package = "released"
path = "../released"
version = "1.0.0"
"#;
        let pure_dev = "[package]\nname = 'pure_dev'\nversion = '1.0.0'\n[dev-dependencies]\nreleased = { path = '../released' }\nuntouched.workspace = true\n";
        let (output, report) =
            release_plan(&[root, consumer, dev_consumer, pure_dev]).expect("plan");
        let workspace = output[0].parse::<DocumentMut>().expect("workspace");
        assert!(workspace["workspace"]["dependencies"]["shared"]
            .get("path")
            .is_none());
        assert!(workspace["workspace"]["dependencies"]["other_shared"]
            .get("path")
            .is_none());
        assert_eq!(
            workspace["workspace"]["dependencies"]["untouched"]["path"].as_str(),
            Some("sdk/test/released")
        );
        let dev = output[2].parse::<DocumentMut>().expect("dev manifest");
        assert!(dev["dev-dependencies"]["direct"].get("path").is_none());
        assert_eq!(
            dev["dev-dependencies"]["direct"]["version"].as_str(),
            Some("1.2.0")
        );
        assert_eq!(
            dev["dev-dependencies"]["direct"]["features"]
                .as_array()
                .expect("features")
                .get(0)
                .and_then(toml_edit::Value::as_str),
            Some("xml")
        );
        assert!(dev["target"]["cfg(windows)"]["dev-dependencies"]["local"]
            .get("path")
            .is_none());
        assert_eq!(
            dev["target"]["cfg(windows)"]["dev-dependencies"]["local"]["version"].as_str(),
            Some("1.0.0")
        );
        assert_eq!(output[1], consumer);
        assert_eq!(output[3], pure_dev);
        assert_eq!(
            report["affected_packages"],
            json!(["consumer", "dev_consumer"])
        );
        assert_eq!(
            report["changed_manifests"],
            json!(["Cargo.toml", "sdk/test/consumer2/Cargo.toml"])
        );
        let sources = output.iter().map(String::as_str).collect::<Vec<_>>();
        let (_, second) = release_plan(&sources).expect("repeat");
        assert_eq!(
            second,
            json!({"changed_manifests": [], "affected_packages": []})
        );
    }

    #[test]
    fn registry_and_dev_only_dependencies_are_noops() {
        let source = r#"[package]
name = "consumer"
version = "1.0.0"
[dependencies]
released = "1" # compatible registry
[build-dependencies]
renamed = { package = "released", version = "=2.0.0" }
[dev-dependencies]
other = { path = "../other" }
"#;
        let dev_only =
            "[dev-dependencies]\nreleased = { path = '../released', version = '2.0.0' }\n";
        let (output, report) = release_plan(&["[workspace]\n", source, dev_only]).expect("plan");
        assert_eq!(output[1], source);
        assert_eq!(output[2], dev_only);
        assert_eq!(
            report,
            json!({"changed_manifests": [], "affected_packages": []})
        );
    }

    #[test]
    fn beta_fallback_leaves_unrelated_registry_consumers_unchanged() {
        let root = r#"[workspace.dependencies]
released = { version = "1.1.0", default-features = false }
"#;
        let registry = r#"[package]
name = "registry"
version = "1.0.0"
[dependencies]
released = "1.1.0"
[build-dependencies]
renamed = { package = "released", version = "=1.1.0", features = ["xml"] }
[target.'cfg(unix)'.dependencies]
released = { version = "~1.1.0", default-features = false }
"#;
        let inherited = r#"[package]
name = "inherited"
version = "1.0.0"
[dependencies]
released.workspace = true
"#;
        let local = r#"[package]
name = "local"
version = "1.0.0"
[dependencies]
released = { path = "../released", version = "1.2.0-beta.2" }
[dev-dependencies]
released = { path = "../released", version = "1.2.0-beta.2" }
"#;
        let (output, report) =
            release_plan_for_version(&[root, registry, inherited, local], "1.2.0-beta.1")
                .expect("beta fallback");
        assert_eq!(output[0], root);
        assert_eq!(output[1], registry);
        assert_eq!(output[2], inherited);
        let local = output[3].parse::<DocumentMut>().expect("local manifest");
        for section in ["dependencies", "dev-dependencies"] {
            assert!(local[section]["released"].get("path").is_none());
            assert_eq!(
                local[section]["released"]["version"].as_str(),
                Some("1.2.0-beta.1")
            );
        }
        assert_eq!(
            report,
            json!({
                "changed_manifests": ["sdk/test/consumer3/Cargo.toml"],
                "affected_packages": ["local"]
            })
        );
        let sources = output.iter().map(String::as_str).collect::<Vec<_>>();
        let (_, second) = release_plan_for_version(&sources, "1.2.0-beta.1").expect("repeat");
        assert_eq!(
            second,
            json!({"changed_manifests": [], "affected_packages": []})
        );
    }

    #[test]
    fn incompatible_local_or_alignment_sources_fail() {
        for dependency in [
            "{ path = '../released', version = '0.9.0' }",
            "'0.9.0'",
            "{ version = '1', git = 'https://example.invalid/released' }",
            "{ version = '1', registry = 'custom' }",
        ] {
            let source = format!(
                "[dependencies]\nreleased = {dependency}\n[dev-dependencies]\nreleased = {{ path = '../released' }}\n"
            );
            assert!(
                release_plan(&["[workspace]\n", &source]).is_err(),
                "{dependency}"
            );
        }
    }

    #[test]
    fn path_removal_preserves_comments_and_compatible_lower_requirements() {
        let source = r#"[dependencies.released]
# local source
path = "../released" # path explanation
version = "1.0.0" # lower compatible requirement
features = ["xml"]
"#;
        let (output, _) = release_plan(&["[workspace]\n", source]).expect("plan");
        let document = output[1].parse::<DocumentMut>().expect("manifest");
        assert!(document["dependencies"]["released"].get("path").is_none());
        assert_eq!(
            document["dependencies"]["released"]["version"].as_str(),
            Some("1.0.0")
        );
        for comment in [
            "# local source",
            "# path explanation",
            "# lower compatible requirement",
        ] {
            assert!(output[1].contains(comment), "{comment}");
        }
    }

    #[test]
    fn existing_modes_keep_path_only_exceptions() {
        let source = r#"[dependencies]
released = { path = "../released" }
private = { path = "../private" }
azure_identity = { path = "../identity" }
[dev-dependencies]
released = { path = "../released" }
"#;
        let versions = vec![
            ("released".to_string(), "1.2.0".to_string(), false),
            ("private".to_string(), "1.2.0".to_string(), true),
            ("azure_identity".to_string(), "1.2.0".to_string(), false),
        ];
        let mut document = source.parse::<DocumentMut>().expect("manifest");
        update_package_versions(document.as_table_mut(), &versions, false);
        assert_eq!(document.to_string(), source);
        update_package_versions(document.as_table_mut(), &versions, true);
        assert_eq!(
            document["dependencies"]["released"]["version"].as_str(),
            Some("1.2.0")
        );
        assert_eq!(
            document["dependencies"]["released"]["path"].as_str(),
            Some("../released")
        );
        assert!(document["dependencies"]["private"].get("version").is_none());
        assert!(document["dependencies"]["azure_identity"]
            .get("version")
            .is_none());
        assert!(document["dev-dependencies"]["released"]
            .get("version")
            .is_none());
    }

    #[test]
    fn updating_versions_preserves_feature_settings() {
        for add in [false, true] {
            let mut document = r#"
[features]
default = ["azure_core/default"]

[dependencies]
azure_core = { path = "../../core/azure_core", version = "1.0.0", default-features = false, features = ["xml"] }

[dev-dependencies.azure_core]
path = "../../core/azure_core"
version = "1.0.0"
default-features = false
features = ["test"]
"#
            .parse::<DocumentMut>()
            .expect("manifest");
            let before = document.to_string();
            update_package_versions(
                document.as_table_mut(),
                &vec![("azure_core".to_string(), "1.2.0-beta.2".to_string(), false)],
                add,
            );
            assert_eq!(
                document.to_string(),
                before.replace("version = \"1.0.0\"", "version = \"1.2.0-beta.2\""),
            );
        }
    }

    #[test]
    fn adding_versions_preserves_dependency_feature_opt_out() {
        let mut document = r#"
[dependencies]
azure_core = { path = "../../core/azure_core", default-features = false, features = ["xml"] }
"#
        .parse::<DocumentMut>()
        .expect("manifest");
        update_package_versions(
            document.as_table_mut(),
            &vec![("azure_core".to_string(), "1.2.0-beta.2".to_string(), false)],
            true,
        );
        let dependency = &document["dependencies"]["azure_core"];
        assert_eq!(dependency["version"].as_str(), Some("1.2.0-beta.2"));
        assert_eq!(dependency["default-features"].as_bool(), Some(false));
        assert_eq!(
            dependency["features"]
                .as_array()
                .expect("features")
                .get(0)
                .and_then(toml_edit::Value::as_str),
            Some("xml"),
        );
    }

    #[test]
    fn workspace_feature_settings_are_preserved() {
        let mut document = r#"
[workspace.dependencies.azure_core]
path = "sdk/core/azure_core"
version = "1.0.0"
default-features = false
"#
        .parse::<DocumentMut>()
        .expect("workspace manifest");
        let before = document.to_string();
        update_package_versions(
            document["workspace"].as_table_mut().expect("workspace"),
            &vec![("azure_core".to_string(), "1.2.0-beta.2".to_string(), false)],
            false,
        );
        assert_eq!(
            document.to_string(),
            before.replace("version = \"1.0.0\"", "version = \"1.2.0-beta.2\""),
        );
    }
}

// Copyright (c) Microsoft Corporation. All rights reserved.
// Licensed under the MIT License.

use rustdoc_types::{Crate, Id, ItemEnum, ItemKind, ItemSummary, Visibility};
use std::collections::{BTreeMap, BTreeSet, HashMap};
use std::error::Error;
use std::ffi::OsStr;
use std::fs::File;
use std::io::Read;
use std::path::Path;
use std::process::Command;

const CRATES: &[&str] = &["typespec", "typespec_client_core", "azure_core"];

fn main() -> Result<(), Box<dyn Error>> {
    if !Path::new("eng/tools/check_api_superset").exists() {
        return Err(
            "This tool must be run from the root of the azure-sdk-for-rust repository.".into(),
        );
    }

    let channel = env!("TOOLCHAIN_CHANNEL");
    let exemptions = load_exemptions("eng/tools/check_api_superset/exemptions.txt");

    // Generate rustdoc JSON for all crates first so we can cross-reference.
    let mut crate_data: Vec<(&str, Crate)> = Vec::new();
    for crate_name in CRATES {
        let mut command = Command::new("cargo");
        command
            .arg(format!("+{channel}"))
            .arg("rustdoc")
            .arg("-Z")
            .arg("unstable-options")
            .arg("--output-format")
            .arg("json")
            .arg("--package")
            .arg(crate_name)
            .arg("--all-features");
        eprintln!(
            "Running: {} {}",
            command.get_program().to_string_lossy(),
            command
                .get_args()
                .collect::<Vec<&OsStr>>()
                .join(OsStr::new(" "))
                .to_string_lossy(),
        );
        let output = command.output()?;
        if !output.status.success() {
            return Err(format!(
                "Failed to generate rustdoc JSON for {crate_name}: {}",
                String::from_utf8_lossy(&output.stderr)
            )
            .into());
        }

        let json_path = format!("./target/doc/{crate_name}.json");
        let mut file = File::open(&json_path)?;
        let mut contents = String::new();
        file.read_to_string(&mut contents)?;

        let krate: Crate = serde_json::from_str(&contents)?;
        crate_data.push((crate_name, krate));
    }

    // Build a lookup from crate name to its parsed Crate data, so we can
    // resolve glob re-exports and external module re-exports across crates.
    let dep_crates: HashMap<&str, &Crate> = crate_data
        .iter()
        .map(|(name, krate)| (*name, krate))
        .collect();

    // Collect public types for each crate.
    let mut all_types: Vec<(&str, BTreeMap<String, &'static str>)> = Vec::new();
    for (crate_name, krate) in &crate_data {
        let types = collect_public_types(krate, &dep_crates);
        eprintln!("Found {} public items in {crate_name}", types.len());
        all_types.push((crate_name, types));
    }

    // Compare each crate with its expected superset:
    // typespec ⊆ typespec_client_core ⊆ azure_core
    let checks: &[(&str, usize, usize)] = &[
        ("azure_core", 2, 1),           // azure_core ⊇ typespec_client_core
        ("typespec_client_core", 1, 0), // typespec_client_core ⊇ typespec
    ];

    let mut has_differences = false;
    for &(superset_name, superset_idx, subset_idx) in checks {
        let superset = &all_types[superset_idx].1;
        let subset_name = all_types[subset_idx].0;
        let subset = &all_types[subset_idx].1;

        let missing = find_missing(superset_name, superset, subset, &exemptions);

        if !missing.is_empty() {
            has_differences = true;
            println!("- {superset_name}");
            for (path, kind) in &missing {
                println!("  - Missing `{path}` ({kind}) from {subset_name}");
            }
        }
    }

    if !has_differences {
        println!("All public types are properly exported through the crate hierarchy.");
    }

    if has_differences {
        std::process::exit(1);
    }

    Ok(())
}

/// Return items in `subset` that are not in `superset` and not exempted.
/// Exemptions use the superset crate's fully-qualified path (e.g.,
/// `azure_core::http::Foo` when that item is missing from `azure_core`).
fn find_missing<'a>(
    superset_name: &str,
    superset: &BTreeMap<String, &'static str>,
    subset: &'a BTreeMap<String, &'static str>,
    exemptions: &BTreeSet<String>,
) -> Vec<(&'a String, &'a &'static str)> {
    subset
        .iter()
        .filter(|(path, _)| {
            let qualified = format!("{superset_name}::{path}");
            !superset.contains_key(*path) && !exemptions.contains(&qualified)
        })
        .collect()
}

/// Load exempted paths from a file (one fully-qualified path per line, e.g.,
/// `azure_core::http::REDACTED_PATTERN`). Lines that are empty or start with `#` are ignored.
/// Returns an empty set if the file does not exist.
fn load_exemptions(path: &str) -> BTreeSet<String> {
    let Ok(contents) = std::fs::read_to_string(path) else {
        return BTreeSet::new();
    };
    parse_exemptions(&contents)
}

/// Parse exemption entries from text content.
fn parse_exemptions(contents: &str) -> BTreeSet<String> {
    contents
        .lines()
        .map(str::trim)
        .filter(|line| !line.is_empty() && !line.starts_with('#'))
        .map(String::from)
        .collect()
}

/// Walk the public module tree and collect all public type paths relative to the crate root.
fn collect_public_types(
    krate: &Crate,
    dep_crates: &HashMap<&str, &Crate>,
) -> BTreeMap<String, &'static str> {
    let mut types = BTreeMap::new();
    let root = &krate.index[&krate.root];
    if let ItemEnum::Module(module) = &root.inner {
        walk_module(krate, dep_crates, &module.items, "", &mut types);
    }
    types
}

fn walk_module(
    krate: &Crate,
    dep_crates: &HashMap<&str, &Crate>,
    items: &[Id],
    prefix: &str,
    types: &mut BTreeMap<String, &'static str>,
) {
    for id in items {
        let Some(item) = krate.index.get(id) else {
            continue;
        };
        if !matches!(item.visibility, Visibility::Public) {
            continue;
        }
        match &item.inner {
            ItemEnum::Module(module) => {
                let name = item.name.as_deref().unwrap_or("");
                let path = join_path(prefix, name);
                walk_module(krate, dep_crates, &module.items, &path, types);
            }
            ItemEnum::Use(use_item) => {
                if use_item.is_glob {
                    // Resolve glob re-exports by looking up the source module
                    // in the dependency crate's rustdoc JSON.
                    resolve_glob(krate, dep_crates, use_item, prefix, types);
                    continue;
                }
                let path = join_path(prefix, &use_item.name);
                if let Some(target_id) = &use_item.id {
                    if let Some(target) = krate.index.get(target_id) {
                        collect_item(krate, dep_crates, target, &path, types);
                    } else if let Some(summary) = krate.paths.get(target_id) {
                        collect_external(dep_crates, summary, &path, types);
                    }
                }
            }
            ItemEnum::Impl(_) => {}
            _ => {
                let name = item.name.as_deref().unwrap_or("");
                let path = join_path(prefix, name);
                if let Some(kind) = item_kind_str(&item.inner) {
                    types.insert(path, kind);
                }
            }
        }
    }
}

/// Collect an item found in this crate's index (may be a module to recurse into).
fn collect_item(
    krate: &Crate,
    dep_crates: &HashMap<&str, &Crate>,
    item: &rustdoc_types::Item,
    path: &str,
    types: &mut BTreeMap<String, &'static str>,
) {
    match &item.inner {
        ItemEnum::Module(module) => {
            walk_module(krate, dep_crates, &module.items, path, types);
        }
        _ => {
            if let Some(kind) = item_kind_str(&item.inner) {
                types.insert(path.to_string(), kind);
            }
        }
    }
}

/// Handle an external item referenced only via `paths` (not in `index`).
/// For modules, resolve them by finding the module in the source crate's JSON.
fn collect_external(
    dep_crates: &HashMap<&str, &Crate>,
    summary: &ItemSummary,
    path: &str,
    types: &mut BTreeMap<String, &'static str>,
) {
    if summary.kind == ItemKind::Module {
        // Find the source crate and walk its module contents.
        let source_crate_name = &summary.path[0];
        if let Some(dep) = dep_crates.get(source_crate_name.as_str()) {
            let module_path = &summary.path[1..];
            if let Some(module_items) = find_module_items(dep, module_path) {
                // Walk the external module's items using the dep crate's index.
                let mut sub_types = BTreeMap::new();
                walk_module(dep, dep_crates, &module_items, "", &mut sub_types);
                for (sub_path, kind) in sub_types {
                    types.insert(join_path(path, &sub_path), kind);
                }
            }
        }
    } else if let Some(kind) = item_kind_display(summary.kind) {
        types.insert(path.to_string(), kind);
    }
}

/// Resolve a `pub use source::*` glob re-export by finding the source module
/// in a dependency crate and walking its public items.
fn resolve_glob(
    krate: &Crate,
    dep_crates: &HashMap<&str, &Crate>,
    use_item: &rustdoc_types::Use,
    prefix: &str,
    types: &mut BTreeMap<String, &'static str>,
) {
    let source = &use_item.source;
    let parts: Vec<&str> = source.split("::").collect();
    if parts.is_empty() {
        return;
    }

    let source_crate = parts[0];

    // Check if this is a glob from an external dependency crate.
    if let Some(dep) = dep_crates.get(source_crate) {
        let module_path = &parts[1..];
        if let Some(module_items) = find_module_items(dep, module_path) {
            let mut sub_types = BTreeMap::new();
            walk_module(dep, dep_crates, &module_items, "", &mut sub_types);
            for (sub_path, kind) in sub_types {
                types.insert(join_path(prefix, &sub_path), kind);
            }
        }
        return;
    }

    // Internal glob re-export (e.g., `pub use submodule::*`).
    // Resolve by finding the target module in this crate.
    if let Some(target_id) = &use_item.id {
        if let Some(target) = krate.index.get(target_id) {
            if let ItemEnum::Module(module) = &target.inner {
                walk_module(krate, dep_crates, &module.items, prefix, types);
            }
        } else if let Some(summary) = krate.paths.get(target_id) {
            collect_external(dep_crates, summary, prefix, types);
        }
    }
}

/// Navigate down a crate's module tree by path segments to find a module's items.
fn find_module_items<S: AsRef<str>>(krate: &Crate, module_path: &[S]) -> Option<Vec<Id>> {
    let root = krate.index.get(&krate.root)?;
    let ItemEnum::Module(root_module) = &root.inner else {
        return None;
    };

    if module_path.is_empty() {
        return Some(root_module.items.clone());
    }

    let mut current_items = &root_module.items;
    for (i, segment) in module_path.iter().enumerate() {
        let mut found = false;
        for id in current_items {
            let Some(item) = krate.index.get(id) else {
                continue;
            };
            if item.name.as_deref() == Some(segment.as_ref()) {
                if let ItemEnum::Module(module) = &item.inner {
                    if i == module_path.len() - 1 {
                        return Some(module.items.clone());
                    }
                    current_items = &module.items;
                    found = true;
                    break;
                }
            }
        }
        if !found {
            return None;
        }
    }
    None
}

fn join_path(prefix: &str, name: &str) -> String {
    if prefix.is_empty() {
        name.to_string()
    } else {
        format!("{prefix}::{name}")
    }
}

fn item_kind_str(inner: &ItemEnum) -> Option<&'static str> {
    match inner {
        ItemEnum::Struct(_) => Some("struct"),
        ItemEnum::Enum(_) => Some("enum"),
        ItemEnum::Function(_) => Some("fn"),
        ItemEnum::TypeAlias(_) => Some("type alias"),
        ItemEnum::Constant { .. } => Some("const"),
        ItemEnum::Trait(_) => Some("trait"),
        ItemEnum::Static(_) => Some("static"),
        ItemEnum::Macro(_) => Some("macro"),
        ItemEnum::ProcMacro(_) => Some("proc macro"),
        ItemEnum::Union(_) => Some("union"),
        _ => None,
    }
}

fn item_kind_display(kind: ItemKind) -> Option<&'static str> {
    match kind {
        ItemKind::Struct => Some("struct"),
        ItemKind::Enum => Some("enum"),
        ItemKind::Function => Some("fn"),
        ItemKind::TypeAlias => Some("type alias"),
        ItemKind::Constant => Some("const"),
        ItemKind::Trait => Some("trait"),
        ItemKind::Static => Some("static"),
        ItemKind::Macro => Some("macro"),
        ItemKind::ProcAttribute | ItemKind::ProcDerive => Some("proc macro"),
        ItemKind::Union => Some("union"),
        _ => None,
    }
}

#[cfg(test)]
mod tests;

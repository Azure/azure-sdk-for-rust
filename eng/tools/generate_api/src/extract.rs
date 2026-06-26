// Copyright (c) Microsoft Corporation. All rights reserved.
// Licensed under the MIT License.

use crate::{
    driver::PackageMetadata,
    model::{ApiAttribute, ApiItem, ApiItemKind, ApiMember, ApiModel, ApiModule},
};
use rustdoc_types::{
    Constant, Crate, Function, FunctionHeader, GenericArg, GenericArgs, GenericBound,
    GenericParamDef, GenericParamDefKind, Id, Impl, Item, ItemEnum, MacroKind, Path, Static,
    StructKind, Term, Trait, TraitAlias, Type, TypeAlias, Union, Variant, VariantKind, Visibility,
    WherePredicate,
};
use std::collections::{BTreeSet, HashSet};
use std::sync::Arc;

pub(crate) trait WorkspaceResolver {
    fn is_workspace_crate(&self, crate_name: &str) -> bool;
    fn load_workspace_model(&mut self, crate_name: &str) -> Result<Option<Arc<ApiModel>>, String>;
    fn load_workspace_crate(&mut self, crate_name: &str) -> Result<Option<Arc<Crate>>, String>;
}

pub(crate) fn extract_model(
    package: &PackageMetadata,
    krate: &Crate,
    resolver: &mut impl WorkspaceResolver,
) -> Result<ApiModel, String> {
    let root = krate
        .index
        .get(&krate.root)
        .ok_or_else(|| "rustdoc JSON root module was missing from the index".to_string())?;
    let ItemEnum::Module(_) = &root.inner else {
        return Err("rustdoc JSON root item was not a module".to_string());
    };

    let mut model = ApiModel::new(package.name.clone(), package.version.clone());
    model.root_module = extract_module(krate, root, package.name.clone(), resolver)?;
    Ok(model)
}

fn extract_module(
    krate: &Crate,
    item: &Item,
    path: String,
    resolver: &mut impl WorkspaceResolver,
) -> Result<ApiModule, String> {
    let ItemEnum::Module(module) = &item.inner else {
        unreachable!("extract_module only accepts module items");
    };

    let mut result = ApiModule {
        path,
        doc_comments: extract_doc_comments(item),
        attributes: extract_attributes(item),
        items: Vec::new(),
        modules: Vec::new(),
    };
    let mut seen_declarations = BTreeSet::new();
    let mut seen_modules = BTreeSet::new();

    for child_id in &module.items {
        let Some(child) = krate.index.get(child_id) else {
            continue;
        };

        match &child.inner {
            ItemEnum::Module(inner) if !inner.is_stripped && is_visible(child) => {
                let child_path = format!(
                    "{}::{}",
                    result.path,
                    child.name.as_deref().unwrap_or("unknown_module")
                );
                let module = extract_module(krate, child, child_path, resolver)?;
                insert_module(&mut result.modules, &mut seen_modules, module);
            }
            ItemEnum::Impl(impl_block) if include_trait_impl_block(impl_block) => {
                if let Some(extracted) = extract_trait_impl(krate, child, impl_block) {
                    if seen_declarations.insert(extracted.declaration.clone()) {
                        result.items.push(extracted);
                    }
                }
            }
            ItemEnum::Impl(_) => {}
            ItemEnum::Use(use_item) if should_include_item(child) => {
                if let Some(expanded) =
                    expand_reexport(krate, child, use_item, &result.path, resolver)?
                {
                    insert_expanded(
                        &mut result,
                        &mut seen_declarations,
                        &mut seen_modules,
                        expanded,
                    );
                } else {
                    let extracted = extract_item(krate, child);
                    if seen_declarations.insert(extracted.declaration.clone()) {
                        result.items.push(extracted);
                    }
                }
            }
            _ if should_include_item(child) => {
                let extracted = extract_item(krate, child);
                if seen_declarations.insert(extracted.declaration.clone()) {
                    result.items.push(extracted);
                }
            }
            _ => {}
        }
    }

    Ok(result)
}

#[derive(Default)]
struct ExpandedUse {
    items: Vec<ApiItem>,
    modules: Vec<ApiModule>,
}

fn expand_reexport(
    krate: &Crate,
    use_item: &Item,
    import: &rustdoc_types::Use,
    current_module_path: &str,
    resolver: &mut impl WorkspaceResolver,
) -> Result<Option<ExpandedUse>, String> {
    let import_attributes = extract_attributes(use_item);
    if !import.is_glob && import.name != last_path_segment(&import.source) {
        return Ok(None);
    }

    let Some(target_id) = &import.id else {
        return Ok(None);
    };

    if let Some(target) = krate.index.get(target_id) {
        if should_expand_local_reexport(krate, import, current_module_path) {
            return expand_local_reexport(
                krate,
                target,
                import,
                current_module_path,
                &import_attributes,
                resolver,
            );
        }

        return Ok(None);
    }

    let Some(summary) = krate.paths.get(target_id) else {
        return Ok(None);
    };
    let Some(crate_name) = summary.path.first() else {
        return Ok(None);
    };
    if !resolver.is_workspace_crate(crate_name) {
        return Ok(None);
    }

    if let Some(krate) = resolver.load_workspace_crate(crate_name)? {
        if let Some(target) = find_raw_item_by_path(&krate, &summary.path[1..]) {
            return expand_local_reexport(
                &krate,
                target,
                import,
                current_module_path,
                &import_attributes,
                resolver,
            );
        }
    }

    let Some(model) = resolver.load_workspace_model(crate_name)? else {
        return Ok(None);
    };
    let target_segments = summary
        .path
        .iter()
        .skip(1)
        .map(String::as_str)
        .collect::<Vec<&str>>();
    Ok(expand_model_reexport(
        &model,
        import,
        current_module_path,
        &import_attributes,
        &target_segments,
    ))
}

fn expand_local_reexport(
    krate: &Crate,
    target: &Item,
    import: &rustdoc_types::Use,
    current_module_path: &str,
    import_attributes: &[ApiAttribute],
    resolver: &mut impl WorkspaceResolver,
) -> Result<Option<ExpandedUse>, String> {
    if let ItemEnum::Use(other_import) = &target.inner {
        let mut expanded = ExpandedUse::default();
        if let Some(nested) =
            expand_reexport(krate, target, other_import, current_module_path, resolver)?
        {
            merge_expanded(&mut expanded, nested);
        }
        if let Some(target_id) = &other_import.id {
            if let Some(raw_target) = krate.index.get(target_id) {
                merge_expanded(
                    &mut expanded,
                    expand_item_with_impls(krate, raw_target, current_module_path),
                );
            }
        }
        if expanded.items.is_empty() && expanded.modules.is_empty() {
            return Ok(None);
        }
        apply_import_attributes(&mut expanded, import_attributes);
        return Ok(Some(expanded));
    }

    let mut expanded = match &target.inner {
        ItemEnum::Module(_) => {
            let rebased_path = format!("{}::{}", current_module_path, import.name);
            let module = extract_module(krate, target, rebased_path, resolver)?;
            if import.is_glob {
                ExpandedUse {
                    items: module.items,
                    modules: module.modules,
                }
            } else {
                ExpandedUse {
                    items: Vec::new(),
                    modules: vec![module],
                }
            }
        }
        _ if !import.is_glob => expand_item_with_impls(krate, target, current_module_path),
        _ => return Ok(None),
    };
    apply_import_attributes(&mut expanded, import_attributes);
    Ok(Some(expanded))
}

fn expand_model_reexport(
    model: &ApiModel,
    import: &rustdoc_types::Use,
    current_module_path: &str,
    import_attributes: &[ApiAttribute],
    target_segments: &[&str],
) -> Option<ExpandedUse> {
    let mut expanded = if import.is_glob {
        let module = if target_segments.is_empty() {
            model.root_module.clone()
        } else {
            find_module(&model.root_module, target_segments)?.clone()
        };
        let mut module = rebase_module(module, current_module_path.to_string());
        ExpandedUse {
            items: module.items.drain(..).collect(),
            modules: module.modules.drain(..).collect(),
        }
    } else if let Some(module) = find_module(&model.root_module, target_segments) {
        ExpandedUse {
            items: Vec::new(),
            modules: vec![rebase_module(
                module.clone(),
                format!("{}::{}", current_module_path, import.name),
            )],
        }
    } else {
        expand_model_item_reexport(&model.root_module, target_segments, current_module_path)?
    };
    apply_import_attributes(&mut expanded, import_attributes);
    Some(expanded)
}

fn expand_item_with_impls(krate: &Crate, target: &Item, current_module_path: &str) -> ExpandedUse {
    let mut expanded = ExpandedUse {
        items: vec![extract_item(krate, target)],
        modules: Vec::new(),
    };

    for sibling in trait_impls_for_item(krate, target, current_module_path) {
        expanded.items.push(sibling);
    }

    expanded
}

fn trait_impls_for_item(krate: &Crate, target: &Item, current_module_path: &str) -> Vec<ApiItem> {
    let impl_ids = match &target.inner {
        ItemEnum::Struct(struct_item) => &struct_item.impls,
        ItemEnum::Enum(enum_item) => &enum_item.impls,
        ItemEnum::Union(union_item) => &union_item.impls,
        _ => return Vec::new(),
    };

    impl_ids
        .iter()
        .filter_map(|impl_id| krate.index.get(impl_id))
        .filter_map(|impl_item| match &impl_item.inner {
            ItemEnum::Impl(impl_block) if include_trait_impl_block(impl_block) => {
                extract_trait_impl(krate, impl_item, impl_block)
            }
            _ => None,
        })
        .map(|item| rebase_trait_impl_item(item, current_module_path))
        .collect()
}

fn rebase_trait_impl_item(mut item: ApiItem, current_module_path: &str) -> ApiItem {
    if let Some((trait_name, _)) = item.declaration.split_once(" for ") {
        let trait_name = trait_name
            .trim_start_matches("unsafe ")
            .trim_start_matches("impl ");
        item.name = format!("{}_{}", last_path_segment(trait_name), item.name);
    }
    if current_module_path.is_empty() {
        return item;
    }
    item
}

fn expand_model_item_reexport(
    module: &ApiModule,
    target_segments: &[&str],
    current_module_path: &str,
) -> Option<ExpandedUse> {
    let target = find_item(module, target_segments)?.clone();
    let local_name = target_segments.last().copied()?;
    let items = module
        .items
        .iter()
        .filter(|candidate| {
            candidate.name == local_name
                || (candidate.kind == ApiItemKind::TraitImpl
                    && candidate
                        .declaration
                        .contains(&format!(" for {local_name}")))
        })
        .cloned()
        .map(|item| rebase_trait_impl_item(item, current_module_path))
        .collect::<Vec<_>>();

    if items.is_empty() {
        Some(ExpandedUse {
            items: vec![target],
            modules: Vec::new(),
        })
    } else {
        Some(ExpandedUse {
            items,
            modules: Vec::new(),
        })
    }
}

fn merge_expanded(target: &mut ExpandedUse, source: ExpandedUse) {
    target.items.extend(source.items);
    target.modules.extend(source.modules);
}

fn should_expand_local_reexport(
    krate: &Crate,
    import: &rustdoc_types::Use,
    current_module_path: &str,
) -> bool {
    !is_local_source_publicly_reachable(krate, &import.source, import.is_glob, current_module_path)
}

fn is_local_source_publicly_reachable(
    krate: &Crate,
    source: &str,
    is_glob: bool,
    current_module_path: &str,
) -> bool {
    let absolute_segments = resolve_local_source_segments(source, current_module_path);
    if absolute_segments.is_empty() {
        return false;
    }

    let mut module = match krate.index.get(&krate.root) {
        Some(module) => module,
        None => return false,
    };

    let module_segments = if is_glob {
        absolute_segments.as_slice()
    } else {
        &absolute_segments[..absolute_segments.len().saturating_sub(1)]
    };

    for segment in module_segments {
        let ItemEnum::Module(module_data) = &module.inner else {
            return false;
        };

        let Some(child_module) = module_data
            .items
            .iter()
            .filter_map(|child_id| krate.index.get(child_id))
            .find(|child| {
                item_lookup_name(child) == Some(segment.as_str())
                    && matches!(child.inner, ItemEnum::Module(_))
            })
        else {
            return false;
        };

        if !is_visible(child_module) {
            return false;
        }

        let ItemEnum::Module(child_data) = &child_module.inner else {
            return false;
        };
        if child_data.is_stripped {
            return false;
        }

        module = child_module;
    }

    true
}

fn resolve_local_source_segments(source: &str, current_module_path: &str) -> Vec<String> {
    let mut base_segments = current_module_path
        .split("::")
        .skip(1)
        .map(str::to_string)
        .collect::<Vec<String>>();
    let mut remaining = source;

    if let Some(rest) = remaining.strip_prefix("crate::") {
        return rest.split("::").map(str::to_string).collect();
    }
    if remaining == "crate" {
        return Vec::new();
    }

    while let Some(rest) = remaining.strip_prefix("self::") {
        remaining = rest;
    }

    while let Some(rest) = remaining.strip_prefix("super::") {
        base_segments.pop();
        remaining = rest;
    }

    if remaining.is_empty() {
        base_segments
    } else {
        base_segments.extend(remaining.split("::").map(str::to_string));
        base_segments
    }
}

fn find_raw_item_by_path<'a>(krate: &'a Crate, path: &[String]) -> Option<&'a Item> {
    let mut module = krate.index.get(&krate.root)?;
    if path.is_empty() {
        return Some(module);
    }

    for (index, segment) in path.iter().enumerate() {
        let ItemEnum::Module(module_data) = &module.inner else {
            return None;
        };
        let child = module_data
            .items
            .iter()
            .filter_map(|child_id| krate.index.get(child_id))
            .find(|child| item_lookup_name(child) == Some(segment.as_str()))?;
        if index + 1 == path.len() {
            return Some(child);
        }
        module = child;
    }

    None
}

fn item_lookup_name(item: &Item) -> Option<&str> {
    item.name.as_deref().or(match &item.inner {
        ItemEnum::Use(use_item) => Some(use_item.name.as_str()),
        _ => None,
    })
}

fn find_module<'a>(module: &'a ApiModule, segments: &[&str]) -> Option<&'a ApiModule> {
    if segments.is_empty() {
        return Some(module);
    }

    let (head, tail) = segments.split_first()?;
    if let Some(child) = module
        .modules
        .iter()
        .find(|candidate| candidate.local_name() == *head)
    {
        if tail.is_empty() {
            return Some(child);
        }
        if let Some(found) = find_module(child, tail) {
            return Some(found);
        }
    }

    if tail.is_empty() {
        None
    } else {
        find_module(module, tail)
    }
}

fn find_item<'a>(module: &'a ApiModule, segments: &[&str]) -> Option<&'a ApiItem> {
    let (head, tail) = segments.split_first()?;
    if tail.is_empty() {
        module
            .items
            .iter()
            .find(|candidate| candidate.name == *head)
    } else {
        if let Some(child) = module
            .modules
            .iter()
            .find(|candidate| candidate.local_name() == *head)
        {
            if let Some(found) = find_item(child, tail) {
                return Some(found);
            }
        }

        find_item(module, tail)
    }
}

fn rebase_module(mut module: ApiModule, new_path: String) -> ApiModule {
    let parent_path = new_path.clone();
    module.path = new_path;
    module.modules = module
        .modules
        .into_iter()
        .map(|child| {
            let child_name = child.local_name().to_string();
            rebase_module(child, format!("{parent_path}::{child_name}"))
        })
        .collect();
    module
}

fn apply_import_attributes(expanded: &mut ExpandedUse, import_attributes: &[ApiAttribute]) {
    if import_attributes.is_empty() {
        return;
    }

    for item in &mut expanded.items {
        prepend_attributes(&mut item.attributes, import_attributes);
    }
    for module in &mut expanded.modules {
        prepend_attributes(&mut module.attributes, import_attributes);
    }
}

fn prepend_attributes(attributes: &mut Vec<ApiAttribute>, prefix: &[ApiAttribute]) {
    if prefix.is_empty() {
        return;
    }

    let mut combined = prefix.to_vec();
    for attribute in attributes.drain(..) {
        if !combined
            .iter()
            .any(|candidate| candidate.text == attribute.text)
        {
            combined.push(attribute);
        }
    }
    *attributes = combined;
}

fn insert_expanded(
    module: &mut ApiModule,
    seen_declarations: &mut BTreeSet<String>,
    seen_modules: &mut BTreeSet<String>,
    expanded: ExpandedUse,
) {
    for item in expanded.items {
        if seen_declarations.insert(item.declaration.clone()) {
            module.items.push(item);
        }
    }

    for child_module in expanded.modules {
        insert_module(&mut module.modules, seen_modules, child_module);
    }
}

fn insert_module(
    modules: &mut Vec<ApiModule>,
    seen_modules: &mut BTreeSet<String>,
    module: ApiModule,
) {
    if seen_modules.insert(module.path.clone()) {
        modules.push(module);
    }
}

fn extract_item(krate: &Crate, item: &Item) -> ApiItem {
    let mut attributes = extract_attributes(item);
    if let Some(attribute) = synthesize_derive_attribute(krate, item) {
        prepend_attributes(&mut attributes, &[attribute]);
    }
    if matches!(item.inner, ItemEnum::Trait(_)) && trait_uses_async_trait(krate, item) {
        prepend_attributes(
            &mut attributes,
            &[ApiAttribute {
                text: "#[async_trait]".to_string(),
            }],
        );
    }

    ApiItem {
        name: item
            .name
            .clone()
            .unwrap_or_else(|| fallback_item_name(item).to_string()),
        kind: item_kind(item),
        doc_comments: extract_doc_comments(item),
        attributes,
        declaration: render_item_declaration(krate, item),
        members: extract_members(krate, item),
    }
}

fn extract_members(krate: &Crate, item: &Item) -> Vec<ApiMember> {
    match &item.inner {
        ItemEnum::Struct(struct_item) => extract_inherent_impl_members(krate, &struct_item.impls),
        ItemEnum::Enum(enum_item) => extract_inherent_impl_members(krate, &enum_item.impls),
        ItemEnum::Union(union_item) => extract_inherent_impl_members(krate, &union_item.impls),
        ItemEnum::Trait(trait_item) => extract_trait_members(krate, trait_item),
        _ => Vec::new(),
    }
}

fn synthesize_derive_attribute(krate: &Crate, item: &Item) -> Option<ApiAttribute> {
    let impl_ids = match &item.inner {
        ItemEnum::Struct(struct_item) => &struct_item.impls,
        ItemEnum::Enum(enum_item) => &enum_item.impls,
        ItemEnum::Union(union_item) => &union_item.impls,
        _ => return None,
    };

    let mut derived = BTreeSet::new();
    for impl_id in impl_ids {
        let Some(impl_item) = krate.index.get(impl_id) else {
            continue;
        };
        let ItemEnum::Impl(impl_block) = &impl_item.inner else {
            continue;
        };
        if impl_block.is_synthetic || impl_block.blanket_impl.is_some() {
            continue;
        }
        if !has_automatically_derived(impl_item) {
            continue;
        }
        let Some(trait_path) = &impl_block.trait_ else {
            continue;
        };
        if let Some(derive_name) = known_derive_trait_name(trait_path) {
            derived.insert(derive_name);
        }
    }

    if derived.is_empty() {
        None
    } else {
        Some(ApiAttribute {
            text: format!(
                "#[derive({})]",
                derived.into_iter().collect::<Vec<_>>().join(", ")
            ),
        })
    }
}

fn has_automatically_derived(item: &Item) -> bool {
    item.attrs
        .iter()
        .any(|attr| attr.contains("automatically_derived"))
}

fn known_derive_trait_name(path: &Path) -> Option<&'static str> {
    match path.path.as_str() {
        "Clone" | "core::clone::Clone" | "std::clone::Clone" => Some("Clone"),
        "Copy" | "core::marker::Copy" | "std::marker::Copy" => Some("Copy"),
        "Debug" | "fmt::Debug" | "core::fmt::Debug" | "std::fmt::Debug" => Some("Debug"),
        "Default" | "core::default::Default" | "std::default::Default" => Some("Default"),
        "Eq" | "core::cmp::Eq" | "std::cmp::Eq" => Some("Eq"),
        "Hash" | "core::hash::Hash" | "std::hash::Hash" => Some("Hash"),
        "Ord" | "core::cmp::Ord" | "std::cmp::Ord" => Some("Ord"),
        "PartialEq" | "core::cmp::PartialEq" | "std::cmp::PartialEq" => Some("PartialEq"),
        "PartialOrd" | "core::cmp::PartialOrd" | "std::cmp::PartialOrd" => Some("PartialOrd"),
        "Serialize" => Some("serde::Serialize"),
        _ if path.path == "serde::Serialize" || path.path.ends_with("::Serialize") => {
            Some("serde::Serialize")
        }
        "Deserialize" => Some("serde::Deserialize"),
        _ if path.path == "serde::Deserialize" || path.path.ends_with("::Deserialize") => {
            Some("serde::Deserialize")
        }
        _ => None,
    }
}

fn extract_trait_impl(krate: &Crate, item: &Item, impl_block: &Impl) -> Option<ApiItem> {
    if has_automatically_derived(item) {
        return None;
    }

    let trait_path = impl_block.trait_.as_ref()?;
    let self_type = render_type(&impl_block.for_);
    let declaration = render_trait_impl_declaration(impl_block, trait_path, &self_type);

    Some(ApiItem {
        name: self_type,
        kind: ApiItemKind::TraitImpl,
        doc_comments: extract_doc_comments(item),
        attributes: extract_attributes(item),
        declaration,
        members: extract_impl_items(krate, &impl_block.items),
    })
}

fn extract_inherent_impl_members(krate: &Crate, impl_ids: &[Id]) -> Vec<ApiMember> {
    impl_ids
        .iter()
        .filter_map(|impl_id| krate.index.get(impl_id))
        .filter_map(|impl_item| match &impl_item.inner {
            ItemEnum::Impl(impl_block) if include_inherent_impl_block(impl_block) => {
                Some(&impl_block.items)
            }
            _ => None,
        })
        .flat_map(|items| extract_impl_items(krate, items))
        .collect()
}

fn extract_impl_items(krate: &Crate, item_ids: &[Id]) -> Vec<ApiMember> {
    item_ids
        .iter()
        .filter_map(|item_id| krate.index.get(item_id))
        .filter(|item| is_visible(item))
        .filter_map(extract_associated_member)
        .collect()
}

fn extract_trait_members(krate: &Crate, trait_item: &Trait) -> Vec<ApiMember> {
    trait_item
        .items
        .iter()
        .filter_map(|item_id| krate.index.get(item_id))
        .filter_map(extract_associated_member)
        .collect()
}

fn extract_associated_member(item: &Item) -> Option<ApiMember> {
    match &item.inner {
        ItemEnum::Function(function) => Some(ApiMember {
            name: item.name.clone().unwrap_or_default(),
            doc_comments: extract_doc_comments(item),
            attributes: extract_attributes(item),
            declaration: render_function_declaration(
                item.name.as_deref().unwrap_or("unknown_fn"),
                function,
                false,
            ),
        }),
        ItemEnum::AssocConst { type_, value } => Some(ApiMember {
            name: item.name.clone().unwrap_or_default(),
            doc_comments: extract_doc_comments(item),
            attributes: extract_attributes(item),
            declaration: render_assoc_const(
                item.name.as_deref().unwrap_or("UNKNOWN_CONST"),
                type_,
                value.as_deref(),
            ),
        }),
        ItemEnum::AssocType {
            generics,
            bounds,
            type_,
        } => Some(ApiMember {
            name: item.name.clone().unwrap_or_default(),
            doc_comments: extract_doc_comments(item),
            attributes: extract_attributes(item),
            declaration: render_assoc_type(
                item.name.as_deref().unwrap_or("UnknownType"),
                generics,
                bounds,
                type_.as_ref(),
            ),
        }),
        _ => None,
    }
}

fn extract_attributes(item: &Item) -> Vec<ApiAttribute> {
    item.attrs
        .iter()
        .map(|text| ApiAttribute {
            text: normalize_attribute(text),
        })
        .collect()
}

fn extract_doc_comments(item: &Item) -> Vec<String> {
    item.docs
        .as_deref()
        .map(|docs| {
            docs.lines()
                .map(|line| {
                    if line.is_empty() {
                        "///".to_string()
                    } else {
                        format!("/// {line}")
                    }
                })
                .collect()
        })
        .unwrap_or_default()
}

fn normalize_attribute(text: &str) -> String {
    let mut normalized = text
        .replace("#[<cfg>(", "#[cfg(")
        .replace("#![<cfg>(", "#![cfg(")
        .replace("#[<cfg_attr>(", "#[cfg_attr(")
        .replace("#![<cfg_attr>(", "#![cfg_attr(")
        .replace("clippy :: ", "clippy::")
        .replace("clippy ::", "clippy::");

    normalized = normalize_pin_attribute(&normalized);
    normalized = collapse_clippy_lint_whitespace(&normalized);
    normalized
}

fn normalize_pin_attribute(attribute: &str) -> String {
    normalize_pin_attribute_with_prefix(attribute, "#[")
        .or_else(|| normalize_pin_attribute_with_prefix(attribute, "#!["))
        .unwrap_or_else(|| attribute.to_string())
}

fn normalize_pin_attribute_with_prefix(attribute: &str, prefix: &str) -> Option<String> {
    let body = attribute.strip_prefix(prefix)?;
    let body = body.strip_suffix(']')?;
    let inner = body.strip_prefix("pin(__private(")?;
    let inner = inner.strip_suffix("))")?;
    if inner.is_empty() {
        Some(format!("{prefix}pin_project]"))
    } else {
        Some(format!("{prefix}pin_project({inner})]"))
    }
}

fn collapse_clippy_lint_whitespace(attribute: &str) -> String {
    let mut remaining = attribute;
    let mut normalized = String::new();

    while let Some(index) = remaining.find("clippy::") {
        normalized.push_str(&remaining[..index + "clippy::".len()]);
        remaining = &remaining[index + "clippy::".len()..];
        remaining = remaining.trim_start_matches(char::is_whitespace);
    }

    normalized.push_str(remaining);
    normalized
}

fn should_include_item(item: &Item) -> bool {
    is_visible(item)
        && !matches!(
            item.inner,
            ItemEnum::Variant(_)
                | ItemEnum::StructField(_)
                | ItemEnum::AssocConst { .. }
                | ItemEnum::AssocType { .. }
                | ItemEnum::ExternCrate { .. }
                | ItemEnum::Primitive(_)
        )
}

fn is_visible(item: &Item) -> bool {
    matches!(item.visibility, Visibility::Public | Visibility::Default)
}

fn include_inherent_impl_block(impl_block: &Impl) -> bool {
    !impl_block.is_synthetic && impl_block.blanket_impl.is_none() && impl_block.trait_.is_none()
}

fn include_trait_impl_block(impl_block: &Impl) -> bool {
    !impl_block.is_synthetic && impl_block.blanket_impl.is_none() && impl_block.trait_.is_some()
}

fn item_kind(item: &Item) -> ApiItemKind {
    match &item.inner {
        ItemEnum::Use(_) => ApiItemKind::Use,
        ItemEnum::Macro(_) => ApiItemKind::Macro,
        ItemEnum::ProcMacro(_) => ApiItemKind::ProcMacro,
        ItemEnum::Function(_) => ApiItemKind::Function,
        ItemEnum::Struct(_) => ApiItemKind::Struct,
        ItemEnum::Enum(_) => ApiItemKind::Enum,
        ItemEnum::Trait(_) => ApiItemKind::Trait,
        ItemEnum::TraitAlias(_) => ApiItemKind::TraitAlias,
        ItemEnum::Union(_) => ApiItemKind::Union,
        ItemEnum::TypeAlias(_) => ApiItemKind::TypeAlias,
        ItemEnum::Constant { .. } => ApiItemKind::Const,
        ItemEnum::Static(_) => ApiItemKind::Static,
        _ => ApiItemKind::TypeAlias,
    }
}

fn fallback_item_name(item: &Item) -> &'static str {
    match &item.inner {
        ItemEnum::Use(_) => "use",
        ItemEnum::Macro(_) => "macro",
        ItemEnum::ProcMacro(_) => "proc_macro",
        ItemEnum::Function(_) => "function",
        ItemEnum::Struct(_) => "struct",
        ItemEnum::Enum(_) => "enum",
        ItemEnum::Trait(_) => "trait",
        ItemEnum::TraitAlias(_) => "trait_alias",
        ItemEnum::Union(_) => "union",
        ItemEnum::TypeAlias(_) => "type_alias",
        ItemEnum::Constant { .. } => "const",
        ItemEnum::Static(_) => "static",
        _ => "item",
    }
}

fn render_item_declaration(krate: &Crate, item: &Item) -> String {
    match &item.inner {
        ItemEnum::Use(use_item) => render_use_declaration(krate, use_item),
        ItemEnum::Macro(source) => source.clone(),
        ItemEnum::ProcMacro(proc_macro) => render_proc_macro_declaration(
            item.name.as_deref().unwrap_or("unknown_macro"),
            proc_macro,
        ),
        ItemEnum::Function(function) => render_function_declaration(
            item.name.as_deref().unwrap_or("unknown_fn"),
            function,
            true,
        ),
        ItemEnum::Struct(struct_item) => render_struct_declaration(krate, item, struct_item),
        ItemEnum::Enum(enum_item) => render_enum_declaration(krate, item, enum_item),
        ItemEnum::Trait(trait_item) => render_trait_declaration(krate, item, trait_item),
        ItemEnum::TraitAlias(trait_alias) => render_trait_alias_declaration(item, trait_alias),
        ItemEnum::Union(union_item) => render_union_declaration(krate, item, union_item),
        ItemEnum::TypeAlias(type_alias) => render_type_alias_declaration(item, type_alias),
        ItemEnum::Constant { type_, const_ } => render_const_declaration(item, type_, const_),
        ItemEnum::Static(static_item) => render_static_declaration(item, static_item),
        _ => format!("// Unsupported item: {}", fallback_item_name(item)),
    }
}

fn render_use_declaration(krate: &Crate, use_item: &rustdoc_types::Use) -> String {
    let source = use_item
        .id
        .as_ref()
        .and_then(|id| krate.paths.get(id))
        .map(|summary| normalize_use_path(&summary.path))
        .unwrap_or_else(|| use_item.source.clone());

    if use_item.is_glob {
        format!("pub use {source}::*;")
    } else if use_item.name == last_path_segment(&source) {
        format!("pub use {source};")
    } else {
        format!("pub use {} as {};", source, use_item.name)
    }
}

fn normalize_use_path(path: &[String]) -> String {
    if path.len() >= 2 && path[0] == path[1] {
        path[1..].join("::")
    } else {
        path.join("::")
    }
}

fn render_proc_macro_declaration(name: &str, proc_macro: &rustdoc_types::ProcMacro) -> String {
    match proc_macro.kind {
        MacroKind::Bang => format!("{name}!() {{ /* proc-macro */ }}"),
        MacroKind::Attr => format!("#[{name}]"),
        MacroKind::Derive => {
            if proc_macro.helpers.is_empty() {
                format!("#[derive({name})]")
            } else {
                let mut declaration = format!("#[derive({name})]\n{{\n");
                declaration.push_str("    // Attributes available to this derive:\n");
                for helper in &proc_macro.helpers {
                    declaration.push_str("    #[");
                    declaration.push_str(helper);
                    declaration.push_str("]\n");
                }
                declaration.push('}');
                declaration
            }
        }
    }
}

fn render_function_declaration(name: &str, function: &Function, is_public: bool) -> String {
    let synthetic_lifetimes = synthetic_async_trait_lifetimes(function);
    let mut declaration = String::new();
    if is_public {
        declaration.push_str("pub ");
    }

    declaration.push_str(&render_function_header(&function.header));
    declaration.push_str("fn ");
    declaration.push_str(name);
    declaration.push_str(&render_generics_declaration_with_elision(
        &function.generics,
        &synthetic_lifetimes,
    ));
    declaration.push('(');
    declaration.push_str(
        &function
            .sig
            .inputs
            .iter()
            .map(|(argument_name, argument_type)| {
                if argument_name.is_empty() {
                    render_type_with_elision(argument_type, &synthetic_lifetimes)
                } else {
                    format!(
                        "{argument_name}: {}",
                        render_type_with_elision(argument_type, &synthetic_lifetimes)
                    )
                }
            })
            .collect::<Vec<String>>()
            .join(", "),
    );
    if function.sig.is_c_variadic {
        if !function.sig.inputs.is_empty() {
            declaration.push_str(", ...");
        } else {
            declaration.push_str("...");
        }
    }
    declaration.push(')');

    if let Some(output) = &function.sig.output {
        declaration.push_str(" -> ");
        declaration.push_str(&render_type_with_elision(output, &synthetic_lifetimes));
    }

    declaration.push_str(&render_where_clause_with_elision(
        &function.generics.where_predicates,
        &synthetic_lifetimes,
    ));
    declaration.push(';');
    declaration
}

fn render_function_header(header: &FunctionHeader) -> String {
    let mut parts = Vec::new();
    if header.is_const {
        parts.push("const".to_string());
    }
    if header.is_async {
        parts.push("async".to_string());
    }
    if header.is_unsafe {
        parts.push("unsafe".to_string());
    }

    match &header.abi {
        rustdoc_types::Abi::Rust => {}
        rustdoc_types::Abi::C { unwind } => parts.push(render_abi("C", *unwind)),
        rustdoc_types::Abi::Cdecl { unwind } => parts.push(render_abi("cdecl", *unwind)),
        rustdoc_types::Abi::Stdcall { unwind } => parts.push(render_abi("stdcall", *unwind)),
        rustdoc_types::Abi::Fastcall { unwind } => parts.push(render_abi("fastcall", *unwind)),
        rustdoc_types::Abi::Aapcs { unwind } => parts.push(render_abi("aapcs", *unwind)),
        rustdoc_types::Abi::Win64 { unwind } => parts.push(render_abi("win64", *unwind)),
        rustdoc_types::Abi::SysV64 { unwind } => parts.push(render_abi("sysv64", *unwind)),
        rustdoc_types::Abi::System { unwind } => parts.push(render_abi("system", *unwind)),
        rustdoc_types::Abi::Other(abi) => parts.push(format!("extern {abi:?}")),
    }

    if parts.is_empty() {
        String::new()
    } else {
        format!("{} ", parts.join(" "))
    }
}

fn render_abi(abi: &str, unwind: bool) -> String {
    if unwind {
        format!("extern \"{abi}-unwind\"")
    } else {
        format!("extern \"{abi}\"")
    }
}

fn render_struct_declaration(
    krate: &Crate,
    item: &Item,
    struct_item: &rustdoc_types::Struct,
) -> String {
    let mut declaration = format!(
        "pub struct {}{}",
        item.name.as_deref().unwrap_or("UnknownStruct"),
        render_generics_declaration(&struct_item.generics)
    );
    declaration.push_str(&render_where_clause(&struct_item.generics.where_predicates));

    match &struct_item.kind {
        StructKind::Unit => declaration.push(';'),
        StructKind::Tuple(fields) => {
            let rendered_fields = fields
                .iter()
                .filter_map(|field_id| field_id.as_ref())
                .filter_map(|field_id| krate.index.get(field_id))
                .filter_map(render_tuple_field)
                .collect::<Vec<String>>()
                .join(", ");
            declaration.push('(');
            declaration.push_str(&rendered_fields);
            declaration.push_str(");");
        }
        StructKind::Plain { fields, .. } => {
            declaration.push_str(" {\n");
            for field in fields
                .iter()
                .filter_map(|field_id| krate.index.get(field_id))
            {
                if let Some(field_line) = render_named_field(field) {
                    declaration.push_str("    ");
                    declaration.push_str(&field_line);
                    declaration.push('\n');
                }
            }
            declaration.push('}');
        }
    }

    declaration
}

fn render_union_declaration(krate: &Crate, item: &Item, union_item: &Union) -> String {
    let mut declaration = format!(
        "pub union {}{}",
        item.name.as_deref().unwrap_or("UnknownUnion"),
        render_generics_declaration(&union_item.generics)
    );
    declaration.push_str(&render_where_clause(&union_item.generics.where_predicates));
    declaration.push_str(" {\n");
    for field in union_item
        .fields
        .iter()
        .filter_map(|field_id| krate.index.get(field_id))
    {
        if let Some(field_line) = render_named_field(field) {
            declaration.push_str("    ");
            declaration.push_str(&field_line);
            declaration.push('\n');
        }
    }
    declaration.push('}');
    declaration
}

fn render_enum_declaration(krate: &Crate, item: &Item, enum_item: &rustdoc_types::Enum) -> String {
    let mut declaration = format!(
        "pub enum {}{}",
        item.name.as_deref().unwrap_or("UnknownEnum"),
        render_generics_declaration(&enum_item.generics)
    );
    declaration.push_str(&render_where_clause(&enum_item.generics.where_predicates));
    declaration.push_str(" {\n");
    for variant_id in &enum_item.variants {
        if let Some(variant_item) = krate.index.get(variant_id) {
            declaration.push_str("    ");
            declaration.push_str(&render_variant(krate, variant_item));
            declaration.push('\n');
        }
    }
    declaration.push('}');
    declaration
}

fn render_variant(krate: &Crate, variant_item: &Item) -> String {
    let name = variant_item.name.as_deref().unwrap_or("UnknownVariant");
    let ItemEnum::Variant(Variant { kind, discriminant }) = &variant_item.inner else {
        return name.to_string();
    };

    let mut declaration = String::from(name);
    match kind {
        VariantKind::Plain => {}
        VariantKind::Tuple(fields) => {
            declaration.push('(');
            declaration.push_str(
                &fields
                    .iter()
                    .filter_map(|field_id| field_id.as_ref())
                    .filter_map(|field_id| krate.index.get(field_id))
                    .filter_map(render_tuple_field)
                    .collect::<Vec<String>>()
                    .join(", "),
            );
            declaration.push(')');
        }
        VariantKind::Struct { fields, .. } => {
            declaration.push_str(" { ");
            declaration.push_str(
                &fields
                    .iter()
                    .filter_map(|field_id| krate.index.get(field_id))
                    .filter_map(render_named_field)
                    .collect::<Vec<String>>()
                    .join(", "),
            );
            declaration.push_str(" }");
        }
    }

    if let Some(discriminant) = discriminant {
        declaration.push_str(" = ");
        declaration.push_str(&discriminant.expr);
    }

    declaration.push(',');
    declaration
}

fn render_trait_declaration(_krate: &Crate, item: &Item, trait_item: &Trait) -> String {
    let mut declaration = String::from("pub ");
    if trait_item.is_unsafe {
        declaration.push_str("unsafe ");
    }
    if trait_item.is_auto {
        declaration.push_str("auto ");
    }
    declaration.push_str("trait ");
    declaration.push_str(item.name.as_deref().unwrap_or("UnknownTrait"));
    declaration.push_str(&render_generics_declaration(&trait_item.generics));

    if !trait_item.bounds.is_empty() {
        declaration.push_str(": ");
        declaration.push_str(
            &trait_item
                .bounds
                .iter()
                .map(render_generic_bound)
                .collect::<Vec<String>>()
                .join(" + "),
        );
    }

    declaration.push_str(&render_where_clause(&trait_item.generics.where_predicates));
    declaration.push_str(" {");
    declaration
}

fn render_trait_alias_declaration(item: &Item, trait_alias: &TraitAlias) -> String {
    let mut declaration = format!(
        "pub trait {}{} = ",
        item.name.as_deref().unwrap_or("UnknownTraitAlias"),
        render_generics_declaration(&trait_alias.generics)
    );
    declaration.push_str(
        &trait_alias
            .params
            .iter()
            .map(render_generic_bound)
            .collect::<Vec<String>>()
            .join(" + "),
    );
    declaration.push_str(&render_where_clause(&trait_alias.generics.where_predicates));
    declaration.push(';');
    declaration
}

fn render_trait_impl_declaration(impl_block: &Impl, trait_path: &Path, self_type: &str) -> String {
    let mut declaration = String::new();
    if impl_block.is_unsafe {
        declaration.push_str("unsafe ");
    }
    declaration.push_str("impl");
    declaration.push_str(&render_generics_declaration(&impl_block.generics));
    declaration.push(' ');
    if impl_block.is_negative {
        declaration.push('!');
    }
    declaration.push_str(&render_path(trait_path));
    declaration.push_str(" for ");
    declaration.push_str(self_type);
    declaration.push_str(&render_where_clause(&impl_block.generics.where_predicates));
    declaration.push_str(" {");
    declaration
}

fn render_type_alias_declaration(item: &Item, type_alias: &TypeAlias) -> String {
    let mut declaration = format!(
        "pub type {}{} = {}",
        item.name.as_deref().unwrap_or("UnknownTypeAlias"),
        render_generics_declaration(&type_alias.generics),
        render_type(&type_alias.type_)
    );
    declaration.push_str(&render_where_clause(&type_alias.generics.where_predicates));
    declaration.push(';');
    declaration
}

fn render_const_declaration(item: &Item, type_: &Type, const_: &Constant) -> String {
    format!(
        "pub const {}: {} = {};",
        item.name.as_deref().unwrap_or("UNKNOWN_CONST"),
        render_type(type_),
        const_.expr
    )
}

fn render_static_declaration(item: &Item, static_item: &Static) -> String {
    format!(
        "pub {}static {}{}: {} = {};",
        if static_item.is_unsafe { "unsafe " } else { "" },
        if static_item.is_mutable { "mut " } else { "" },
        item.name.as_deref().unwrap_or("UNKNOWN_STATIC"),
        render_type(&static_item.type_),
        static_item.expr
    )
}

fn render_assoc_const(name: &str, type_: &Type, value: Option<&str>) -> String {
    match value {
        Some(value) => format!("const {name}: {} = {value};", render_type(type_)),
        None => format!("const {name}: {};", render_type(type_)),
    }
}

fn render_assoc_type(
    name: &str,
    generics: &rustdoc_types::Generics,
    bounds: &[GenericBound],
    type_: Option<&Type>,
) -> String {
    let mut declaration = format!("type {name}{}", render_generics_declaration(generics));
    if !bounds.is_empty() {
        declaration.push_str(": ");
        declaration.push_str(
            &bounds
                .iter()
                .map(render_generic_bound)
                .collect::<Vec<String>>()
                .join(" + "),
        );
    }
    if let Some(type_) = type_ {
        declaration.push_str(" = ");
        declaration.push_str(&render_type(type_));
    }
    declaration.push_str(&render_where_clause(&generics.where_predicates));
    declaration.push(';');
    declaration
}

fn render_tuple_field(field_item: &Item) -> Option<String> {
    let ItemEnum::StructField(type_) = &field_item.inner else {
        return None;
    };

    let mut field = String::new();
    if matches!(field_item.visibility, Visibility::Public) {
        field.push_str("pub ");
    }
    field.push_str(&render_type(type_));
    Some(field)
}

fn render_named_field(field_item: &Item) -> Option<String> {
    let ItemEnum::StructField(type_) = &field_item.inner else {
        return None;
    };

    let mut field = String::new();
    if matches!(field_item.visibility, Visibility::Public) {
        field.push_str("pub ");
    }
    field.push_str(field_item.name.as_deref().unwrap_or("unknown_field"));
    field.push_str(": ");
    field.push_str(&render_type(type_));
    field.push(',');
    Some(field)
}

fn render_generics_declaration(generics: &rustdoc_types::Generics) -> String {
    render_generics_declaration_with_elision(generics, &HashSet::new())
}

fn render_generics_declaration_with_elision(
    generics: &rustdoc_types::Generics,
    synthetic_lifetimes: &HashSet<String>,
) -> String {
    let rendered_params = generics
        .params
        .iter()
        .filter(|param| !synthetic_lifetimes.contains(&param.name))
        .map(|param| render_generic_param_with_elision(param, synthetic_lifetimes))
        .filter(|param| !param.is_empty())
        .collect::<Vec<String>>();
    if rendered_params.is_empty() {
        String::new()
    } else {
        format!("<{}>", rendered_params.join(", "))
    }
}

fn render_generic_param_with_elision(
    param: &GenericParamDef,
    synthetic_lifetimes: &HashSet<String>,
) -> String {
    if synthetic_lifetimes.contains(&param.name) {
        return String::new();
    }

    match &param.kind {
        GenericParamDefKind::Lifetime { outlives } => {
            let filtered = outlives
                .iter()
                .filter(|lifetime| !synthetic_lifetimes.contains(*lifetime))
                .cloned()
                .collect::<Vec<String>>();
            if filtered.is_empty() {
                param.name.clone()
            } else {
                format!("{}: {}", param.name, filtered.join(" + "))
            }
        }
        GenericParamDefKind::Type {
            bounds,
            default,
            is_synthetic: _,
        } => {
            let mut rendered = param.name.clone();
            let rendered_bounds = bounds
                .iter()
                .map(|bound| render_generic_bound_with_elision(bound, synthetic_lifetimes))
                .filter(|bound| !bound.is_empty())
                .collect::<Vec<String>>();
            if !rendered_bounds.is_empty() {
                rendered.push_str(": ");
                rendered.push_str(&rendered_bounds.join(" + "));
            }
            if let Some(default) = default {
                rendered.push_str(" = ");
                rendered.push_str(&render_type_with_elision(default, synthetic_lifetimes));
            }
            rendered
        }
        GenericParamDefKind::Const { type_, default } => {
            let mut rendered = format!(
                "const {}: {}",
                param.name,
                render_type_with_elision(type_, synthetic_lifetimes)
            );
            if let Some(default) = default {
                rendered.push_str(" = ");
                rendered.push_str(default);
            }
            rendered
        }
    }
}

fn render_where_clause(predicates: &[WherePredicate]) -> String {
    render_where_clause_with_elision(predicates, &HashSet::new())
}

fn render_where_clause_with_elision(
    predicates: &[WherePredicate],
    synthetic_lifetimes: &HashSet<String>,
) -> String {
    let rendered_predicates = predicates
        .iter()
        .filter_map(|predicate| render_where_predicate_with_elision(predicate, synthetic_lifetimes))
        .collect::<Vec<String>>();
    if rendered_predicates.is_empty() {
        String::new()
    } else {
        format!(" where {}", rendered_predicates.join(", "))
    }
}

fn render_where_predicate_with_elision(
    predicate: &WherePredicate,
    synthetic_lifetimes: &HashSet<String>,
) -> Option<String> {
    match predicate {
        WherePredicate::BoundPredicate {
            type_,
            bounds,
            generic_params,
        } => {
            let rendered_generic_params = generic_params
                .iter()
                .filter(|param| !synthetic_lifetimes.contains(&param.name))
                .map(|param| render_generic_param_with_elision(param, synthetic_lifetimes))
                .filter(|param| !param.is_empty())
                .collect::<Vec<String>>();
            let prefix = if rendered_generic_params.is_empty() {
                String::new()
            } else {
                format!("for<{}> ", rendered_generic_params.join(", "))
            };
            let rendered_bounds = bounds
                .iter()
                .map(|bound| render_generic_bound_with_elision(bound, synthetic_lifetimes))
                .filter(|bound| !bound.is_empty())
                .collect::<Vec<String>>();
            if rendered_bounds.is_empty() {
                None
            } else {
                Some(format!(
                    "{prefix}{}: {}",
                    render_type_with_elision(type_, synthetic_lifetimes),
                    rendered_bounds.join(" + ")
                ))
            }
        }
        WherePredicate::LifetimePredicate { lifetime, outlives } => {
            if synthetic_lifetimes.contains(lifetime) {
                return None;
            }
            let filtered = outlives
                .iter()
                .filter(|outlives| !synthetic_lifetimes.contains(*outlives))
                .cloned()
                .collect::<Vec<String>>();
            if filtered.is_empty() {
                None
            } else {
                Some(format!("{lifetime}: {}", filtered.join(" + ")))
            }
        }
        WherePredicate::EqPredicate { lhs, rhs } => Some(format!(
            "{} = {}",
            render_type_with_elision(lhs, synthetic_lifetimes),
            render_term_with_elision(rhs, synthetic_lifetimes)
        )),
    }
}

fn render_generic_bound(bound: &GenericBound) -> String {
    render_generic_bound_with_elision(bound, &HashSet::new())
}

fn render_generic_bound_with_elision(
    bound: &GenericBound,
    synthetic_lifetimes: &HashSet<String>,
) -> String {
    match bound {
        GenericBound::TraitBound {
            trait_,
            generic_params,
            modifier,
        } => {
            let rendered_generic_params = generic_params
                .iter()
                .filter(|param| !synthetic_lifetimes.contains(&param.name))
                .map(|param| render_generic_param_with_elision(param, synthetic_lifetimes))
                .filter(|param| !param.is_empty())
                .collect::<Vec<String>>();
            let prefix = if rendered_generic_params.is_empty() {
                String::new()
            } else {
                format!("for<{}> ", rendered_generic_params.join(", "))
            };
            let modifier = match modifier {
                rustdoc_types::TraitBoundModifier::None => "",
                rustdoc_types::TraitBoundModifier::Maybe => "?",
                rustdoc_types::TraitBoundModifier::MaybeConst => "const ",
            };
            format!(
                "{prefix}{modifier}{}",
                render_path_with_elision(trait_, synthetic_lifetimes)
            )
        }
        GenericBound::Outlives(lifetime) => {
            if synthetic_lifetimes.contains(lifetime) {
                String::new()
            } else {
                lifetime.clone()
            }
        }
        GenericBound::Use(args) => {
            let rendered = args
                .iter()
                .map(|arg| match arg {
                    rustdoc_types::PreciseCapturingArg::Lifetime(lifetime) => {
                        if synthetic_lifetimes.contains(lifetime) {
                            "'_".to_string()
                        } else {
                            lifetime.clone()
                        }
                    }
                    rustdoc_types::PreciseCapturingArg::Param(param) => param.clone(),
                })
                .collect::<Vec<String>>()
                .join(", ");
            format!("use<{rendered}>")
        }
    }
}

fn render_term_with_elision(term: &Term, synthetic_lifetimes: &HashSet<String>) -> String {
    match term {
        Term::Type(type_) => render_type_with_elision(type_, synthetic_lifetimes),
        Term::Constant(constant) => constant.expr.clone(),
    }
}

fn render_type(type_: &Type) -> String {
    render_type_with_elision(type_, &HashSet::new())
}

fn render_path(path: &Path) -> String {
    render_path_with_elision(path, &HashSet::new())
}

fn render_type_with_elision(type_: &Type, synthetic_lifetimes: &HashSet<String>) -> String {
    match type_ {
        Type::ResolvedPath(path) => render_path_with_elision(path, synthetic_lifetimes),
        Type::DynTrait(dyn_trait) => {
            let mut rendered = String::from("dyn ");
            rendered.push_str(
                &dyn_trait
                    .traits
                    .iter()
                    .map(|trait_| {
                        let rendered_generic_params = trait_
                            .generic_params
                            .iter()
                            .filter(|param| !synthetic_lifetimes.contains(&param.name))
                            .map(|param| {
                                render_generic_param_with_elision(param, synthetic_lifetimes)
                            })
                            .filter(|param| !param.is_empty())
                            .collect::<Vec<String>>();
                        let prefix = if rendered_generic_params.is_empty() {
                            String::new()
                        } else {
                            format!("for<{}> ", rendered_generic_params.join(", "))
                        };
                        format!(
                            "{prefix}{}",
                            render_path_with_elision(&trait_.trait_, synthetic_lifetimes)
                        )
                    })
                    .collect::<Vec<String>>()
                    .join(" + "),
            );
            if let Some(lifetime) = &dyn_trait.lifetime {
                if !synthetic_lifetimes.contains(lifetime) {
                    rendered.push_str(" + ");
                    rendered.push_str(lifetime);
                }
            }
            rendered
        }
        Type::Generic(name) => name.clone(),
        Type::Primitive(name) => name.clone(),
        Type::FunctionPointer(pointer) => {
            let mut rendered = render_function_header(&pointer.header);
            rendered.push_str("fn");
            let rendered_generic_params = pointer
                .generic_params
                .iter()
                .filter(|param| !synthetic_lifetimes.contains(&param.name))
                .map(|param| render_generic_param_with_elision(param, synthetic_lifetimes))
                .filter(|param| !param.is_empty())
                .collect::<Vec<String>>();
            if !rendered_generic_params.is_empty() {
                rendered.push_str(&format!("<{}>", rendered_generic_params.join(", ")));
            }
            rendered.push('(');
            rendered.push_str(
                &pointer
                    .sig
                    .inputs
                    .iter()
                    .map(|(_, type_)| render_type_with_elision(type_, synthetic_lifetimes))
                    .collect::<Vec<String>>()
                    .join(", "),
            );
            if pointer.sig.is_c_variadic {
                if !pointer.sig.inputs.is_empty() {
                    rendered.push_str(", ...");
                } else {
                    rendered.push_str("...");
                }
            }
            rendered.push(')');
            if let Some(output) = &pointer.sig.output {
                rendered.push_str(" -> ");
                rendered.push_str(&render_type_with_elision(output, synthetic_lifetimes));
            }
            rendered
        }
        Type::Tuple(types) => format!(
            "({})",
            types
                .iter()
                .map(|type_| render_type_with_elision(type_, synthetic_lifetimes))
                .collect::<Vec<String>>()
                .join(", ")
        ),
        Type::Slice(type_) => format!("[{}]", render_type_with_elision(type_, synthetic_lifetimes)),
        Type::Array { type_, len } => {
            format!(
                "[{}; {len}]",
                render_type_with_elision(type_, synthetic_lifetimes)
            )
        }
        Type::Pat { type_, .. } => render_type_with_elision(type_, synthetic_lifetimes),
        Type::ImplTrait(bounds) => format!(
            "impl {}",
            bounds
                .iter()
                .map(|bound| render_generic_bound_with_elision(bound, synthetic_lifetimes))
                .filter(|bound| !bound.is_empty())
                .collect::<Vec<String>>()
                .join(" + ")
        ),
        Type::Infer => "_".to_string(),
        Type::RawPointer { is_mutable, type_ } => {
            format!(
                "*{} {}",
                if *is_mutable { "mut" } else { "const" },
                render_type_with_elision(type_, synthetic_lifetimes)
            )
        }
        Type::BorrowedRef {
            lifetime,
            is_mutable,
            type_,
        } => {
            let mut rendered = String::from("&");
            if let Some(lifetime) = lifetime {
                if !synthetic_lifetimes.contains(lifetime) {
                    rendered.push_str(lifetime);
                    rendered.push(' ');
                }
            }
            if *is_mutable {
                rendered.push_str("mut ");
            }
            rendered.push_str(&render_type_with_elision(type_, synthetic_lifetimes));
            rendered
        }
        Type::QualifiedPath {
            name,
            args,
            self_type,
            trait_,
        } => {
            let mut rendered = format!(
                "<{}",
                render_type_with_elision(self_type, synthetic_lifetimes)
            );
            if let Some(trait_) = trait_ {
                rendered.push_str(" as ");
                rendered.push_str(&render_path_with_elision(trait_, synthetic_lifetimes));
            }
            rendered.push_str(">::");
            rendered.push_str(name);
            rendered.push_str(&render_generic_args_with_elision(args, synthetic_lifetimes));
            rendered
        }
    }
}

fn render_path_with_elision(path: &Path, synthetic_lifetimes: &HashSet<String>) -> String {
    let mut rendered = path.path.clone();
    if let Some(args) = &path.args {
        rendered.push_str(&render_generic_args_with_elision(args, synthetic_lifetimes));
    }
    rendered
}

fn render_generic_args_with_elision(
    args: &GenericArgs,
    synthetic_lifetimes: &HashSet<String>,
) -> String {
    match args {
        GenericArgs::AngleBracketed { args, constraints } => {
            let mut rendered_args = args
                .iter()
                .map(|arg| render_generic_arg_with_elision(arg, synthetic_lifetimes))
                .collect::<Vec<String>>();
            rendered_args.extend(constraints.iter().map(|constraint| {
                render_assoc_constraint_with_elision(constraint, synthetic_lifetimes)
            }));
            if rendered_args.is_empty() {
                String::new()
            } else {
                format!("<{}>", rendered_args.join(", "))
            }
        }
        GenericArgs::Parenthesized { inputs, output } => {
            let mut rendered = format!(
                "({})",
                inputs
                    .iter()
                    .map(|type_| render_type_with_elision(type_, synthetic_lifetimes))
                    .collect::<Vec<String>>()
                    .join(", ")
            );
            if let Some(output) = output {
                rendered.push_str(" -> ");
                rendered.push_str(&render_type_with_elision(output, synthetic_lifetimes));
            }
            rendered
        }
        GenericArgs::ReturnTypeNotation => "(..)".to_string(),
    }
}

fn render_generic_arg_with_elision(
    arg: &GenericArg,
    synthetic_lifetimes: &HashSet<String>,
) -> String {
    match arg {
        GenericArg::Lifetime(lifetime) => {
            if synthetic_lifetimes.contains(lifetime) {
                "'_".to_string()
            } else {
                lifetime.clone()
            }
        }
        GenericArg::Type(type_) => render_type_with_elision(type_, synthetic_lifetimes),
        GenericArg::Const(constant) => constant.expr.clone(),
        GenericArg::Infer => "_".to_string(),
    }
}

fn render_assoc_constraint_with_elision(
    constraint: &rustdoc_types::AssocItemConstraint,
    synthetic_lifetimes: &HashSet<String>,
) -> String {
    let mut rendered = constraint.name.clone();
    rendered.push_str(&render_generic_args_with_elision(
        &constraint.args,
        synthetic_lifetimes,
    ));
    match &constraint.binding {
        rustdoc_types::AssocItemConstraintKind::Equality(term) => {
            rendered.push_str(" = ");
            rendered.push_str(&render_term_with_elision(term, synthetic_lifetimes));
        }
        rustdoc_types::AssocItemConstraintKind::Constraint(bounds) => {
            rendered.push_str(": ");
            rendered.push_str(
                &bounds
                    .iter()
                    .map(|bound| render_generic_bound_with_elision(bound, synthetic_lifetimes))
                    .filter(|bound| !bound.is_empty())
                    .collect::<Vec<String>>()
                    .join(" + "),
            );
        }
    }
    rendered
}

fn trait_uses_async_trait(krate: &Crate, item: &Item) -> bool {
    let ItemEnum::Trait(trait_item) = &item.inner else {
        return false;
    };

    trait_item
        .items
        .iter()
        .filter_map(|item_id| krate.index.get(item_id))
        .filter_map(|item| match &item.inner {
            ItemEnum::Function(function) => Some(function),
            _ => None,
        })
        .any(function_uses_async_trait)
}

fn synthetic_async_trait_lifetimes(function: &Function) -> HashSet<String> {
    function
        .generics
        .params
        .iter()
        .filter_map(|param| {
            if is_synthetic_async_trait_lifetime(&param.name) {
                Some(param.name.clone())
            } else {
                None
            }
        })
        .collect()
}

fn function_uses_async_trait(function: &Function) -> bool {
    !synthetic_async_trait_lifetimes(function).is_empty()
}

fn is_synthetic_async_trait_lifetime(name: &str) -> bool {
    name == "'async_trait"
        || name.strip_prefix("'life").is_some_and(|suffix| {
            !suffix.is_empty() && suffix.chars().all(|ch| ch.is_ascii_digit())
        })
}

fn last_path_segment(path: &str) -> &str {
    path.rsplit("::").next().unwrap_or(path)
}

#[cfg(test)]
mod tests {
    use super::*;
    use rustdoc_types::{
        Abi, FunctionSignature, Generics, ItemSummary, Module, Struct, Target, Type,
    };
    use std::collections::HashMap;

    #[test]
    fn recognizes_common_derive_trait_paths() {
        assert_eq!(known_derive_trait_name(&path("Clone", 1)), Some("Clone"));
        assert_eq!(
            known_derive_trait_name(&path("fmt::Debug", 1)),
            Some("Debug")
        );
        assert_eq!(
            known_derive_trait_name(&path("std::fmt::Debug", 1)),
            Some("Debug")
        );
        assert_eq!(
            known_derive_trait_name(&path("Serialize", 1)),
            Some("serde::Serialize")
        );
        assert_eq!(
            known_derive_trait_name(&path("serde::de::Deserialize", 1)),
            Some("serde::Deserialize")
        );
        assert_eq!(known_derive_trait_name(&path("SafeDebug", 1)), None);
    }

    #[test]
    fn synthesizes_known_derives_and_ignores_workspace_defined_traits() {
        let struct_id = Id(1);
        let clone_impl_id = Id(2);
        let debug_impl_id = Id(3);
        let serialize_impl_id = Id(4);
        let safe_debug_impl_id = Id(5);
        let explicit_default_impl_id = Id(6);

        let krate = crate_with_items(vec![
            item(
                struct_id,
                Some("Model"),
                ItemEnum::Struct(Struct {
                    kind: StructKind::Unit,
                    generics: empty_generics(),
                    impls: vec![
                        clone_impl_id,
                        debug_impl_id,
                        serialize_impl_id,
                        safe_debug_impl_id,
                        explicit_default_impl_id,
                    ],
                }),
            ),
            impl_item(
                clone_impl_id,
                Some(path("Clone", 10)),
                "Model",
                struct_id,
                true,
            ),
            impl_item(
                debug_impl_id,
                Some(path("fmt::Debug", 11)),
                "Model",
                struct_id,
                true,
            ),
            impl_item(
                serialize_impl_id,
                Some(path("Serialize", 12)),
                "Model",
                struct_id,
                true,
            ),
            impl_item(
                safe_debug_impl_id,
                Some(path("SafeDebug", 13)),
                "Model",
                struct_id,
                true,
            ),
            impl_item(
                explicit_default_impl_id,
                Some(path("Default", 14)),
                "Model",
                struct_id,
                false,
            ),
        ]);

        let item = krate.index.get(&struct_id).expect("struct item present");
        let attribute = synthesize_derive_attribute(&krate, item)
            .expect("recognized derive attribute should be synthesized");

        assert_eq!(attribute.text, "#[derive(Clone, Debug, serde::Serialize)]");
    }

    #[test]
    fn extracts_explicit_trait_impl_blocks_with_members() {
        let struct_id = Id(1);
        let impl_id = Id(2);
        let fmt_id = Id(3);

        let model = extract_model(
            &package_metadata("demo"),
            &crate_with_items(vec![
                item(
                    struct_id,
                    Some("MyType"),
                    ItemEnum::Struct(Struct {
                        kind: StructKind::Unit,
                        generics: empty_generics(),
                        impls: vec![impl_id],
                    }),
                ),
                impl_item_with_items(
                    impl_id,
                    Some(path("fmt::Debug", 10)),
                    "MyType",
                    struct_id,
                    false,
                    vec![fmt_id],
                ),
                item(
                    fmt_id,
                    Some("fmt"),
                    ItemEnum::Function(Function {
                        sig: FunctionSignature {
                            inputs: vec![
                                (
                                    "self".to_string(),
                                    Type::BorrowedRef {
                                        lifetime: None,
                                        is_mutable: false,
                                        type_: Box::new(Type::Generic("Self".to_string())),
                                    },
                                ),
                                (
                                    "f".to_string(),
                                    Type::BorrowedRef {
                                        lifetime: None,
                                        is_mutable: true,
                                        type_: Box::new(Type::ResolvedPath(path(
                                            "fmt::Formatter",
                                            11,
                                        ))),
                                    },
                                ),
                            ],
                            output: Some(Type::ResolvedPath(path("fmt::Result", 12))),
                            is_c_variadic: false,
                        },
                        generics: empty_generics(),
                        header: FunctionHeader {
                            is_const: false,
                            is_unsafe: false,
                            is_async: false,
                            abi: Abi::Rust,
                        },
                        has_body: true,
                    }),
                ),
            ]),
            &mut NoopResolver,
        )
        .expect("model extraction should succeed");

        let trait_impl = model
            .root_module
            .items
            .iter()
            .find(|item| item.kind == ApiItemKind::TraitImpl)
            .expect("explicit trait impl should be extracted");

        assert_eq!(trait_impl.declaration, "impl fmt::Debug for MyType {");
        assert_eq!(trait_impl.members.len(), 1);
        assert_eq!(
            trait_impl.members[0].declaration,
            "fn fmt(self: &Self, f: &mut fmt::Formatter) -> fmt::Result;"
        );
    }

    #[test]
    fn extract_item_synthesizes_async_trait_and_elides_synthetic_lifetimes() {
        let function_id = Id(2);
        let trait_id = Id(1);

        let krate = crate_with_items(vec![
            item(
                trait_id,
                Some("Polling"),
                ItemEnum::Trait(Trait {
                    is_auto: false,
                    is_unsafe: false,
                    is_dyn_compatible: true,
                    items: vec![function_id],
                    generics: empty_generics(),
                    bounds: Vec::new(),
                    implementations: Vec::new(),
                }),
            ),
            item(
                function_id,
                Some("poll"),
                ItemEnum::Function(Function {
                    sig: FunctionSignature {
                        inputs: vec![(
                            "self".to_string(),
                            Type::BorrowedRef {
                                lifetime: Some("'life0".to_string()),
                                is_mutable: false,
                                type_: Box::new(Type::Generic("Self".to_string())),
                            },
                        )],
                        output: None,
                        is_c_variadic: false,
                    },
                    generics: Generics {
                        params: vec![lifetime_param("'life0"), lifetime_param("'async_trait")],
                        where_predicates: Vec::new(),
                    },
                    header: FunctionHeader {
                        is_const: false,
                        is_unsafe: false,
                        is_async: false,
                        abi: Abi::Rust,
                    },
                    has_body: false,
                }),
            ),
        ]);

        let item = krate.index.get(&trait_id).expect("trait item present");
        let extracted = extract_item(&krate, item);

        assert!(
            extracted
                .attributes
                .iter()
                .any(|attribute| attribute.text == "#[async_trait]"),
            "trait should synthesize #[async_trait]"
        );
        assert_eq!(extracted.members.len(), 1);
        assert_eq!(extracted.members[0].declaration, "fn poll(self: &Self);");
    }

    #[test]
    fn local_reexport_carries_explicit_trait_impls_for_reexported_items() {
        let hidden_module_id = Id(1);
        let struct_id = Id(2);
        let impl_id = Id(3);
        let fmt_id = Id(4);
        let reexport_id = Id(5);

        let model = extract_model(
            &package_metadata("demo"),
            &crate_with_root_items(
                vec![hidden_module_id, reexport_id],
                vec![
                    module_item(hidden_module_id, "hidden", vec![struct_id, impl_id], true),
                    item(
                        struct_id,
                        Some("Error"),
                        ItemEnum::Struct(Struct {
                            kind: StructKind::Unit,
                            generics: empty_generics(),
                            impls: vec![impl_id],
                        }),
                    ),
                    impl_item_with_items(
                        impl_id,
                        Some(path("fmt::Debug", 10)),
                        "Error",
                        struct_id,
                        false,
                        vec![fmt_id],
                    ),
                    item(
                        fmt_id,
                        Some("fmt"),
                        ItemEnum::Function(Function {
                            sig: FunctionSignature {
                                inputs: vec![
                                    (
                                        "self".to_string(),
                                        Type::BorrowedRef {
                                            lifetime: None,
                                            is_mutable: false,
                                            type_: Box::new(Type::Generic("Self".to_string())),
                                        },
                                    ),
                                    (
                                        "f".to_string(),
                                        Type::BorrowedRef {
                                            lifetime: None,
                                            is_mutable: true,
                                            type_: Box::new(Type::ResolvedPath(path(
                                                "fmt::Formatter",
                                                11,
                                            ))),
                                        },
                                    ),
                                ],
                                output: Some(Type::ResolvedPath(path("fmt::Result", 12))),
                                is_c_variadic: false,
                            },
                            generics: empty_generics(),
                            header: FunctionHeader {
                                is_const: false,
                                is_unsafe: false,
                                is_async: false,
                                abi: Abi::Rust,
                            },
                            has_body: true,
                        }),
                    ),
                    item(
                        reexport_id,
                        Some("Error"),
                        ItemEnum::Use(rustdoc_types::Use {
                            source: "crate::hidden::Error".to_string(),
                            name: "Error".to_string(),
                            id: Some(struct_id),
                            is_glob: false,
                        }),
                    ),
                ],
            ),
            &mut NoopResolver,
        )
        .expect("model extraction should succeed");

        assert!(model.root_module.modules.is_empty());
        assert!(model
            .root_module
            .items
            .iter()
            .any(|item| item.declaration == "pub struct Error;"));
        assert!(model.root_module.items.iter().any(|item| {
            item.kind == ApiItemKind::TraitImpl
                && item.declaration == "impl fmt::Debug for Error {"
                && item.members.iter().any(|member| member.name == "fmt")
        }));
    }

    #[test]
    fn local_reexport_preserves_synthesized_derives_for_reexported_items() {
        let hidden_module_id = Id(1);
        let struct_id = Id(2);
        let clone_impl_id = Id(3);
        let debug_impl_id = Id(4);
        let reexport_id = Id(5);

        let model = extract_model(
            &package_metadata("demo"),
            &crate_with_root_items(
                vec![hidden_module_id, reexport_id],
                vec![
                    module_item(
                        hidden_module_id,
                        "hidden",
                        vec![struct_id, clone_impl_id, debug_impl_id],
                        true,
                    ),
                    item(
                        struct_id,
                        Some("ErrorKind"),
                        ItemEnum::Struct(Struct {
                            kind: StructKind::Unit,
                            generics: empty_generics(),
                            impls: vec![clone_impl_id, debug_impl_id],
                        }),
                    ),
                    impl_item(
                        clone_impl_id,
                        Some(path("Clone", 20)),
                        "ErrorKind",
                        struct_id,
                        true,
                    ),
                    impl_item(
                        debug_impl_id,
                        Some(path("fmt::Debug", 21)),
                        "ErrorKind",
                        struct_id,
                        true,
                    ),
                    item(
                        reexport_id,
                        Some("ErrorKind"),
                        ItemEnum::Use(rustdoc_types::Use {
                            source: "crate::hidden::ErrorKind".to_string(),
                            name: "ErrorKind".to_string(),
                            id: Some(struct_id),
                            is_glob: false,
                        }),
                    ),
                ],
            ),
            &mut NoopResolver,
        )
        .expect("model extraction should succeed");

        let item = model
            .root_module
            .items
            .iter()
            .find(|item| item.declaration == "pub struct ErrorKind;")
            .expect("re-exported struct should be lifted");

        assert!(model.root_module.modules.is_empty());
        assert_eq!(
            item.attributes
                .iter()
                .map(|attribute| attribute.text.as_str())
                .collect::<Vec<_>>(),
            vec!["#[derive(Clone, Debug)]"]
        );
    }

    fn crate_with_items(items: Vec<Item>) -> Crate {
        let module_items = items.iter().map(|item| item.id).collect::<Vec<_>>();
        crate_with_root_items(module_items, items)
    }

    fn crate_with_root_items(root_items: Vec<Id>, items: Vec<Item>) -> Crate {
        let root = Id(0);
        let mut index = HashMap::new();
        index.insert(
            root,
            item(
                root,
                Some("crate"),
                ItemEnum::Module(Module {
                    is_crate: true,
                    items: root_items,
                    is_stripped: false,
                }),
            ),
        );
        index.extend(items.into_iter().map(|item| (item.id, item)));

        Crate {
            root,
            crate_version: None,
            includes_private: false,
            index,
            paths: HashMap::<Id, ItemSummary>::new(),
            external_crates: HashMap::new(),
            target: Target {
                triple: "x86_64-unknown-linux-gnu".to_string(),
                target_features: Vec::new(),
            },
            format_version: 0,
        }
    }

    fn item(id: Id, name: Option<&str>, inner: ItemEnum) -> Item {
        Item {
            id,
            crate_id: 0,
            name: name.map(str::to_string),
            span: None,
            visibility: Visibility::Public,
            docs: None,
            links: HashMap::new(),
            attrs: Vec::new(),
            deprecation: None,
            inner,
        }
    }

    fn impl_item(
        id: Id,
        trait_path: Option<Path>,
        self_type_name: &str,
        struct_id: Id,
        automatically_derived: bool,
    ) -> Item {
        impl_item_with_items(
            id,
            trait_path,
            self_type_name,
            struct_id,
            automatically_derived,
            Vec::new(),
        )
    }

    fn impl_item_with_items(
        id: Id,
        trait_path: Option<Path>,
        self_type_name: &str,
        struct_id: Id,
        automatically_derived: bool,
        items: Vec<Id>,
    ) -> Item {
        item(
            id,
            None,
            ItemEnum::Impl(Impl {
                is_unsafe: false,
                generics: empty_generics(),
                provided_trait_methods: Vec::new(),
                trait_: trait_path,
                for_: Type::ResolvedPath(path(self_type_name, struct_id.0)),
                items,
                is_negative: false,
                is_synthetic: false,
                blanket_impl: None,
            }),
        )
        .with_attrs(if automatically_derived {
            vec!["#[automatically_derived]".to_string()]
        } else {
            Vec::new()
        })
    }

    fn module_item(id: Id, name: &str, items: Vec<Id>, is_stripped: bool) -> Item {
        item(
            id,
            Some(name),
            ItemEnum::Module(Module {
                is_crate: false,
                items,
                is_stripped,
            }),
        )
    }

    fn path(path: &str, id: u32) -> Path {
        Path {
            path: path.to_string(),
            id: Id(id),
            args: None,
        }
    }

    fn lifetime_param(name: &str) -> GenericParamDef {
        GenericParamDef {
            name: name.to_string(),
            kind: GenericParamDefKind::Lifetime {
                outlives: Vec::new(),
            },
        }
    }

    fn empty_generics() -> Generics {
        Generics {
            params: Vec::new(),
            where_predicates: Vec::new(),
        }
    }

    fn package_metadata(name: &str) -> PackageMetadata {
        PackageMetadata {
            name: name.to_string(),
            version: "1.0.0".to_string(),
            manifest_path: std::path::PathBuf::from("Cargo.toml"),
        }
    }

    struct NoopResolver;

    impl WorkspaceResolver for NoopResolver {
        fn is_workspace_crate(&self, _crate_name: &str) -> bool {
            false
        }

        fn load_workspace_model(
            &mut self,
            _crate_name: &str,
        ) -> Result<Option<Arc<ApiModel>>, String> {
            Ok(None)
        }

        fn load_workspace_crate(
            &mut self,
            _crate_name: &str,
        ) -> Result<Option<Arc<Crate>>, String> {
            Ok(None)
        }
    }

    trait ItemTestExt {
        fn with_attrs(self, attrs: Vec<String>) -> Self;
    }

    impl ItemTestExt for Item {
        fn with_attrs(mut self, attrs: Vec<String>) -> Self {
            self.attrs = attrs;
            self
        }
    }
}

// Copyright (c) Microsoft Corporation. All rights reserved.
// Licensed under the MIT License.

use crate::{
    driver::PackageMetadata,
    model::{
        ApiAttribute, ApiItem, ApiItemKind, ApiMember, ApiMemberKind, ApiModel, ApiModule,
        ApiNavigationPath, ApiPathReference, InherentImplSortKey, SourceLocation,
    },
    rustdoc_compat,
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

fn source_location_at_offset(item: &Item, source: &str, offset: usize) -> Option<SourceLocation> {
    let mut location = source_location(item)?;
    let prefix = source.get(..offset)?;
    let line_count = prefix.bytes().filter(|byte| *byte == b'\n').count();
    location.line += line_count;
    location.column = if line_count == 0 {
        location.column + prefix.chars().count()
    } else {
        prefix
            .rsplit_once('\n')
            .map_or(0, |(_, suffix)| suffix.chars().count())
    };
    Some(location)
}

fn proc_macro_helper_location(item: &Item, helper: &str) -> Option<SourceLocation> {
    let span = item.span.as_ref()?;
    let source = crate::source_cache::get(&span.filename)?;
    let lines = source.lines().collect::<Vec<_>>();
    let item_line = span.begin.0.saturating_sub(1).min(lines.len());
    let attribute_start = (0..item_line)
        .rev()
        .find(|index| lines[*index].contains("#[proc_macro_derive"))?;

    let mut in_helpers = false;
    for (line_index, line) in lines
        .iter()
        .enumerate()
        .take(item_line)
        .skip(attribute_start)
    {
        let search_from = if in_helpers {
            0
        } else {
            let Some(attributes) = line.find("attributes") else {
                continue;
            };
            in_helpers = true;
            attributes + "attributes".len()
        };
        if let Some(column) = find_identifier(&line[search_from..], helper) {
            return Some(SourceLocation {
                path: repo_relative_source_path(&span.filename)?,
                line: line_index,
                column: search_from + column,
            });
        }
    }
    None
}

fn find_identifier(text: &str, identifier: &str) -> Option<usize> {
    text.match_indices(identifier)
        .map(|(index, _)| index)
        .find(|index| {
            let before = text[..*index].chars().next_back();
            let after = text[*index + identifier.len()..].chars().next();
            !before.is_some_and(|character| character == '_' || character.is_alphanumeric())
                && !after.is_some_and(|character| character == '_' || character.is_alphanumeric())
        })
}

fn find_item_entry<'a>(
    module: &'a ApiModule,
    segments: &[&str],
) -> Option<(&'a ApiModule, &'a ApiItem)> {
    let segments = strip_duplicate_leading_module_segments(module, segments);
    let (head, tail) = segments.split_first()?;
    if tail.is_empty() {
        return module
            .items
            .iter()
            .find(|candidate| candidate.name == *head)
            .map(|item| (module, item));
    }

    if let Some(child) = module
        .modules
        .iter()
        .find(|candidate| candidate.local_name() == *head)
    {
        if let Some(found) = find_item_entry(child, tail) {
            return Some(found);
        }
    }

    if tail.is_empty() {
        None
    } else {
        find_item_entry(module, tail)
    }
}

fn strip_duplicate_leading_module_segments<'a>(
    module: &ApiModule,
    mut segments: &'a [&'a str],
) -> &'a [&'a str] {
    while segments.len() > 1 && segments[0] == module.local_name() {
        segments = &segments[1..];
    }
    segments
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

    let mut model = ApiModel::new(
        package.name.clone(),
        package.version.clone(),
        package.api.clone(),
    );
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
        declaration_location: source_location(item),
        doc_comments: extract_doc_comments(item),
        attributes: extract_module_attributes(item, module.is_crate),
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
                let mut module = extract_module(krate, child, child_path, resolver)?;
                module.declaration_location = module_declaration_location(item, child);
                insert_module(&mut result.modules, &mut seen_modules, module);
            }
            ItemEnum::Impl(impl_block) => {
                if let Some(extracted) = extract_unassociated_trait_impl(krate, child, impl_block) {
                    insert_item(&mut result, &mut seen_declarations, extracted);
                }
            }
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
                    insert_item(&mut result, &mut seen_declarations, extracted);
                }
            }
            _ if should_include_item(child) => {
                let extracted = extract_item(krate, child);
                insert_item(&mut result, &mut seen_declarations, extracted);
                for inherent_impl in inherent_impls_for_item(krate, child) {
                    insert_item(&mut result, &mut seen_declarations, inherent_impl);
                }
                for trait_impl in trait_impls_for_item(krate, child) {
                    insert_item(&mut result, &mut seen_declarations, trait_impl);
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

fn apply_import_navigation_path(expanded: &mut ExpandedUse, path: &str) {
    for item in expanded
        .items
        .iter_mut()
        .filter(|item| item.owner_name.is_none())
    {
        let import_path = if last_path_segment(path) == item.name {
            path.to_string()
        } else {
            format!("{path}::{}", item.name)
        };

        for path in reexport_navigation_paths(&import_path) {
            if !item
                .navigation_paths
                .iter()
                .any(|candidate| candidate.path == path)
            {
                item.navigation_paths.push(ApiNavigationPath {
                    path,
                    source_id: None,
                });
            }
        }
    }
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
    let normalized_target_path = normalize_use_path(&summary.path);
    let target_segments = normalized_target_path
        .split("::")
        .skip(1)
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
                merge_expanded(&mut expanded, expand_item_with_impls(krate, raw_target));
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
        _ if !import.is_glob => expand_item_with_impls(krate, target),
        _ => return Ok(None),
    };
    apply_import_attributes(&mut expanded, import_attributes);
    apply_import_navigation_path(&mut expanded, &import.source);
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
        expand_model_item_reexport(&model.root_module, target_segments)?
    };
    apply_import_attributes(&mut expanded, import_attributes);
    apply_import_navigation_path(&mut expanded, &import.source);
    Some(expanded)
}

fn expand_item_with_impls(krate: &Crate, target: &Item) -> ExpandedUse {
    let mut expanded = ExpandedUse {
        items: vec![extract_item(krate, target)],
        modules: Vec::new(),
    };

    for sibling in inherent_impls_for_item(krate, target) {
        expanded.items.push(sibling);
    }

    for sibling in trait_impls_for_item(krate, target) {
        expanded.items.push(sibling);
    }

    expanded
}

fn trait_impls_for_item(krate: &Crate, target: &Item) -> Vec<ApiItem> {
    let Some(impl_ids) = item_impl_ids(target) else {
        return Vec::new();
    };
    let owner_kind = item_kind(target);

    impl_ids
        .iter()
        .filter_map(|impl_id| krate.index.get(impl_id))
        .filter_map(|impl_item| match &impl_item.inner {
            ItemEnum::Impl(impl_block) if include_trait_impl_block(impl_block) => {
                extract_trait_impl(krate, impl_item, impl_block, Some(target), Some(owner_kind))
            }
            _ => None,
        })
        .map(rebase_trait_impl_item)
        .collect()
}

fn extract_unassociated_trait_impl(
    krate: &Crate,
    item: &Item,
    impl_block: &Impl,
) -> Option<ApiItem> {
    if !include_trait_impl_block(impl_block) {
        return None;
    }

    let owner = local_impl_owner(krate, &impl_block.for_);
    let owner_kind = owner.map(item_kind);
    extract_trait_impl(krate, item, impl_block, owner, owner_kind).map(rebase_trait_impl_item)
}

fn local_impl_owner<'a>(krate: &'a Crate, type_: &Type) -> Option<&'a Item> {
    match type_ {
        Type::ResolvedPath(path) => krate.index.get(&path.id),
        _ => None,
    }
}

fn rebase_trait_impl_item(mut item: ApiItem) -> ApiItem {
    if let Some((trait_name, _)) = item.declaration.split_once(" for ") {
        let trait_name = trait_name
            .trim_start_matches("unsafe ")
            .trim_start_matches("impl ");
        item.name = format!("{}_{}", last_path_segment(trait_name), item.name);
    }
    item
}

fn expand_model_item_reexport(module: &ApiModule, target_segments: &[&str]) -> Option<ExpandedUse> {
    let (containing_module, target) = find_item_entry(module, target_segments)?;
    let target = target.clone();
    let local_name = target_segments.last().copied()?;
    let target_source_id = target.source_id.clone();
    let items =
        containing_module
            .items
            .iter()
            .filter(|candidate| {
                target_source_id.as_ref().is_some_and(|source_id| {
                    candidate.source_id.as_deref() == Some(source_id.as_str())
                }) || target_source_id.as_ref().is_some_and(|source_id| {
                    candidate.owner_source_id.as_deref() == Some(source_id.as_str())
                }) || (candidate.owner_name.is_none()
                    && candidate.kind == target.kind
                    && candidate.name == local_name)
                    || candidate.owner_name.as_deref() == Some(local_name)
            })
            .cloned()
            .map(rebase_trait_impl_item)
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
        insert_item(module, seen_declarations, item);
    }

    for child_module in expanded.modules {
        insert_module(&mut module.modules, seen_modules, child_module);
    }
}

fn insert_item(module: &mut ApiModule, seen_declarations: &mut BTreeSet<String>, item: ApiItem) {
    if seen_declarations.insert(item_dedup_key(&item)) {
        module.items.push(item);
    }
}

fn item_dedup_key(item: &ApiItem) -> String {
    match item.kind {
        ApiItemKind::InherentImpl => {
            let source_id = item.source_id.as_deref().unwrap_or("no-source-id");
            format!("{source_id}\u{1f}{}", inherent_impl_fingerprint(item))
        }
        _ => item.declaration.clone(),
    }
}

fn inherent_impl_fingerprint(item: &ApiItem) -> String {
    let doc_comments = item.doc_comments.join("\u{1f}");
    let attributes = item
        .attributes
        .iter()
        .map(|attribute| attribute.text.as_str())
        .collect::<Vec<_>>()
        .join("\u{1f}");
    let members = item
        .members
        .iter()
        .map(|member| {
            format!(
                "{}\u{1f}{}\u{1f}{}",
                member.name,
                member.doc_comments.join("\u{1f}"),
                member
                    .attributes
                    .iter()
                    .map(|attribute| attribute.text.as_str())
                    .collect::<Vec<_>>()
                    .join("\u{1f}")
            ) + &format!("\u{1f}{}", member.declaration)
        })
        .collect::<Vec<_>>()
        .join("\u{1e}");

    format!(
        "{}\u{1d}{}\u{1d}{}\u{1d}{}",
        item.declaration, doc_comments, attributes, members
    )
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
    if let Some(attribute) = synthesize_pin_project_attribute(krate, item, &attributes) {
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
        name: item_name(krate, item),
        kind: item_kind(item),
        declaration_location: source_location(item),
        source_id: (!matches!(item.inner, ItemEnum::Use(_)))
            .then(|| qualified_source_id(krate, item.id)),
        navigation_paths: item_navigation_paths(krate, item),
        owner_name: None,
        owner_kind: None,
        owner_source_id: None,
        inherent_impl_sort_key: None,
        doc_comments: extract_doc_comments(item),
        attributes,
        declaration: render_item_declaration(krate, item),
        declaration_path_references: collect_item_declaration_path_references(krate, item),
        members: extract_members(krate, item),
    }
}

fn item_name(krate: &Crate, item: &Item) -> String {
    match &item.inner {
        ItemEnum::Use(use_item) => {
            if !use_item.name.is_empty() {
                return use_item.name.clone();
            }

            let source = use_item
                .id
                .as_ref()
                .and_then(|id| krate.paths.get(id))
                .map(|summary| normalize_use_path(&summary.path))
                .unwrap_or_else(|| use_item.source.clone());
            last_path_segment(&source).to_string()
        }
        _ => item
            .name
            .clone()
            .unwrap_or_else(|| fallback_item_name(item).to_string()),
    }
}

fn extract_members(krate: &Crate, item: &Item) -> Vec<ApiMember> {
    match &item.inner {
        ItemEnum::Macro(source) => extract_macro_members(item, source),
        ItemEnum::ProcMacro(proc_macro) => extract_proc_macro_members(item, proc_macro),
        ItemEnum::Struct(struct_item) => extract_struct_members(krate, struct_item),
        ItemEnum::Enum(enum_item) => extract_enum_members(krate, enum_item),
        ItemEnum::Trait(trait_item) => extract_trait_members(krate, trait_item),
        ItemEnum::Union(union_item) => extract_union_members(krate, union_item),
        _ => Vec::new(),
    }
}

fn item_navigation_paths(krate: &Crate, item: &Item) -> Vec<ApiNavigationPath> {
    match &item.inner {
        ItemEnum::Use(use_item) => {
            let source = use_item
                .id
                .as_ref()
                .and_then(|id| krate.paths.get(id))
                .map(|summary| normalize_use_path(&summary.path))
                .unwrap_or_else(|| use_item.source.clone());
            reexport_navigation_paths(&source)
                .into_iter()
                .map(|path| ApiNavigationPath {
                    path,
                    source_id: use_item
                        .id
                        .as_ref()
                        .map(|id| qualified_source_id(krate, *id)),
                })
                .collect()
        }
        _ => Vec::new(),
    }
}

fn qualified_source_id(krate: &Crate, id: Id) -> String {
    krate
        .paths
        .get(&id)
        .and_then(|summary| summary.path.first())
        .map(|crate_name| format!("{crate_name}::{}", id.0))
        .unwrap_or_else(|| id.0.to_string())
}

fn source_location(item: &Item) -> Option<SourceLocation> {
    let span = item.span.as_ref()?;
    Some(SourceLocation {
        path: repo_relative_source_path(&span.filename)?,
        line: span.begin.0.saturating_sub(1),
        column: span.begin.1.saturating_sub(1),
    })
}

fn module_declaration_location(parent: &Item, module: &Item) -> Option<SourceLocation> {
    let parent_span = parent.span.as_ref()?;
    let module_span = module.span.as_ref()?;
    if parent_span.filename == module_span.filename {
        return source_location(module);
    }

    let name = module.name.as_deref()?;
    let source = crate::source_cache::get(&parent_span.filename)?;
    let (line, column) = find_module_declaration(
        &source,
        name,
        parent_span.begin.0.saturating_sub(1),
        parent_span.end.0,
        Some(&module_span.filename),
        matches!(module.visibility, Visibility::Public),
    )?;
    Some(SourceLocation {
        path: repo_relative_source_path(&parent_span.filename)?,
        line,
        column,
    })
}

fn find_module_declaration(
    source: &str,
    name: &str,
    start_line: usize,
    end_line: usize,
    module_file: Option<&std::path::Path>,
    is_public: bool,
) -> Option<(usize, usize)> {
    let mut candidates = Vec::new();
    for (line_index, line) in source
        .lines()
        .enumerate()
        .skip(start_line)
        .take(end_line.saturating_sub(start_line))
    {
        for (mod_index, _) in line.match_indices("mod") {
            let before = &line[..mod_index];
            let previous = before.chars().next_back();
            if previous.is_some_and(|character| character == '_' || character.is_alphanumeric()) {
                continue;
            }

            let prefix = before.trim();
            if !is_module_declaration_prefix(prefix) {
                continue;
            }

            let after_mod = &line[mod_index + 3..];
            let after_whitespace = after_mod.trim_start();
            let Some(after_name) = after_whitespace.strip_prefix(name) else {
                continue;
            };
            if after_name
                .chars()
                .next()
                .is_some_and(|character| character == '_' || character.is_alphanumeric())
            {
                continue;
            }
            if matches!(after_name.trim_start().chars().next(), Some(';' | '{')) {
                candidates.push((
                    line_index,
                    line.len() - line.trim_start().len(),
                    path_attribute_before(source, line_index),
                    is_public_module_declaration_prefix(prefix),
                ));
            }
        }
    }
    if candidates.len() == 1 {
        return candidates.pop().map(|(line, column, _, _)| (line, column));
    }

    let candidates = candidates
        .into_iter()
        .filter(|(_, _, _, candidate_is_public)| *candidate_is_public == is_public)
        .collect::<Vec<_>>();
    if candidates.len() == 1 {
        return candidates
            .into_iter()
            .next()
            .map(|(line, column, _, _)| (line, column));
    }

    let module_file = module_file?.to_string_lossy().replace('\\', "/");
    candidates
        .into_iter()
        .find(|(_, _, path, _)| {
            path.as_deref()
                .is_some_and(|path| module_file.ends_with(&path.replace('\\', "/")))
        })
        .map(|(line, column, _, _)| (line, column))
}

fn is_module_declaration_prefix(prefix: &str) -> bool {
    let visibility = prefix
        .strip_suffix("unsafe")
        .map(str::trim_end)
        .unwrap_or(prefix);
    visibility.is_empty()
        || visibility == "pub"
        || (visibility.starts_with("pub(") && visibility.ends_with(')'))
}

fn is_public_module_declaration_prefix(prefix: &str) -> bool {
    prefix
        .strip_suffix("unsafe")
        .map(str::trim_end)
        .unwrap_or(prefix)
        == "pub"
}

fn path_attribute_before(source: &str, line_index: usize) -> Option<String> {
    for line in source
        .lines()
        .collect::<Vec<_>>()
        .into_iter()
        .take(line_index)
        .rev()
    {
        let line = line.trim();
        if line.is_empty() || line.starts_with("///") || line.starts_with("//!") {
            continue;
        }
        if !line.starts_with("#[") {
            break;
        }
        let Some(path) = line.strip_prefix("#[path") else {
            continue;
        };
        let start = path.find('"')? + 1;
        let end = path[start..].find('"')? + start;
        return Some(path[start..end].to_string());
    }
    None
}

fn repo_relative_source_path(path: &std::path::Path) -> Option<String> {
    let path = if path.is_absolute() {
        path.strip_prefix(std::env::current_dir().ok()?).ok()?
    } else {
        path
    };
    if path
        .components()
        .any(|component| matches!(component, std::path::Component::ParentDir))
    {
        return None;
    }

    Some(path.to_string_lossy().replace('\\', "/"))
}

fn reexport_navigation_paths(path: &str) -> Vec<String> {
    let mut paths = vec![path.to_string()];
    if let Some(stripped) = path.strip_prefix("crate::") {
        paths.push(stripped.to_string());
    }
    paths
}

fn collect_item_declaration_path_references(krate: &Crate, item: &Item) -> Vec<ApiPathReference> {
    let mut references = Vec::new();
    match &item.inner {
        ItemEnum::Function(function) => {
            collect_function_declaration_path_references(krate, function, &mut references);
        }
        ItemEnum::Struct(struct_item) => {
            collect_generic_params_path_references(krate, &struct_item.generics, &mut references);
            collect_where_predicates_path_references(
                krate,
                &struct_item.generics.where_predicates,
                &mut references,
            );
            if let StructKind::Tuple(fields) = &struct_item.kind {
                for field_id in fields.iter().flatten() {
                    let Some(field_item) = krate.index.get(field_id) else {
                        continue;
                    };
                    let ItemEnum::StructField(type_) = &field_item.inner else {
                        continue;
                    };
                    collect_type_path_references(krate, type_, &mut references);
                }
            }
        }
        ItemEnum::Enum(enum_item) => {
            collect_generic_params_path_references(krate, &enum_item.generics, &mut references);
            collect_where_predicates_path_references(
                krate,
                &enum_item.generics.where_predicates,
                &mut references,
            );
        }
        ItemEnum::Trait(trait_item) => {
            collect_generic_params_path_references(krate, &trait_item.generics, &mut references);
            collect_generic_bounds_path_references(krate, &trait_item.bounds, &mut references);
            collect_where_predicates_path_references(
                krate,
                &trait_item.generics.where_predicates,
                &mut references,
            );
        }
        ItemEnum::TraitAlias(trait_alias) => {
            collect_generic_params_path_references(krate, &trait_alias.generics, &mut references);
            collect_generic_bounds_path_references(krate, &trait_alias.params, &mut references);
            collect_where_predicates_path_references(
                krate,
                &trait_alias.generics.where_predicates,
                &mut references,
            );
        }
        ItemEnum::Union(union_item) => {
            collect_generic_params_path_references(krate, &union_item.generics, &mut references);
            collect_where_predicates_path_references(
                krate,
                &union_item.generics.where_predicates,
                &mut references,
            );
        }
        ItemEnum::TypeAlias(type_alias) => {
            collect_generic_params_path_references(krate, &type_alias.generics, &mut references);
            collect_type_path_references(krate, &type_alias.type_, &mut references);
            collect_where_predicates_path_references(
                krate,
                &type_alias.generics.where_predicates,
                &mut references,
            );
        }
        ItemEnum::Constant { type_, .. } => {
            collect_type_path_references(krate, type_, &mut references);
        }
        ItemEnum::Static(static_item) => {
            collect_type_path_references(krate, &static_item.type_, &mut references);
        }
        _ => {}
    }
    references
}

fn collect_function_declaration_path_references(
    krate: &Crate,
    function: &Function,
    references: &mut Vec<ApiPathReference>,
) {
    let synthetic_lifetimes = synthetic_async_trait_lifetimes(function);
    collect_generic_param_path_references_with_elision(
        krate,
        &function.generics,
        &synthetic_lifetimes,
        references,
    );
    for (_, argument_type) in &function.sig.inputs {
        collect_type_path_references_with_elision(
            krate,
            argument_type,
            &synthetic_lifetimes,
            references,
        );
    }
    if let Some(output) = &function.sig.output {
        collect_type_path_references_with_elision(krate, output, &synthetic_lifetimes, references);
    }
    collect_where_predicates_path_references_with_elision(
        krate,
        &function.generics.where_predicates,
        &synthetic_lifetimes,
        references,
    );
}

fn collect_generic_params_path_references(
    krate: &Crate,
    generics: &rustdoc_types::Generics,
    references: &mut Vec<ApiPathReference>,
) {
    collect_generic_param_path_references_with_elision(
        krate,
        generics,
        &HashSet::new(),
        references,
    );
}

fn collect_generic_param_path_references_with_elision(
    krate: &Crate,
    generics: &rustdoc_types::Generics,
    synthetic_lifetimes: &HashSet<String>,
    references: &mut Vec<ApiPathReference>,
) {
    for parameter in &generics.params {
        match &parameter.kind {
            GenericParamDefKind::Type {
                bounds, default, ..
            } => {
                collect_generic_bounds_path_references_with_elision(
                    krate,
                    bounds,
                    synthetic_lifetimes,
                    references,
                );
                if let Some(default) = default {
                    collect_type_path_references_with_elision(
                        krate,
                        default,
                        synthetic_lifetimes,
                        references,
                    );
                }
            }
            GenericParamDefKind::Const { type_, .. } => {
                collect_type_path_references_with_elision(
                    krate,
                    type_,
                    synthetic_lifetimes,
                    references,
                );
            }
            GenericParamDefKind::Lifetime { .. } => {}
        }
    }
}

fn collect_where_predicates_path_references(
    krate: &Crate,
    predicates: &[WherePredicate],
    references: &mut Vec<ApiPathReference>,
) {
    collect_where_predicates_path_references_with_elision(
        krate,
        predicates,
        &HashSet::new(),
        references,
    );
}

fn collect_generic_bounds_path_references(
    krate: &Crate,
    bounds: &[GenericBound],
    references: &mut Vec<ApiPathReference>,
) {
    collect_generic_bounds_path_references_with_elision(krate, bounds, &HashSet::new(), references);
}

fn collect_generic_bounds_path_references_with_elision(
    krate: &Crate,
    bounds: &[GenericBound],
    synthetic_lifetimes: &HashSet<String>,
    references: &mut Vec<ApiPathReference>,
) {
    for bound in bounds {
        match bound {
            GenericBound::TraitBound {
                trait_,
                generic_params,
                ..
            } => {
                for parameter in generic_params {
                    if synthetic_lifetimes.contains(&parameter.name) {
                        continue;
                    }
                    match &parameter.kind {
                        GenericParamDefKind::Type {
                            bounds, default, ..
                        } => {
                            collect_generic_bounds_path_references_with_elision(
                                krate,
                                bounds,
                                synthetic_lifetimes,
                                references,
                            );
                            if let Some(default) = default {
                                collect_type_path_references_with_elision(
                                    krate,
                                    default,
                                    synthetic_lifetimes,
                                    references,
                                );
                            }
                        }
                        GenericParamDefKind::Const { type_, .. } => {
                            collect_type_path_references_with_elision(
                                krate,
                                type_,
                                synthetic_lifetimes,
                                references,
                            );
                        }
                        GenericParamDefKind::Lifetime { .. } => {}
                    }
                }
                collect_path_reference(krate, trait_, references);
            }
            GenericBound::Use(_) => {}
            GenericBound::Outlives(_) => {}
        }
    }
}

fn collect_where_predicates_path_references_with_elision(
    krate: &Crate,
    predicates: &[WherePredicate],
    synthetic_lifetimes: &HashSet<String>,
    references: &mut Vec<ApiPathReference>,
) {
    for predicate in predicates {
        match predicate {
            WherePredicate::BoundPredicate { type_, bounds, .. } => {
                collect_type_path_references_with_elision(
                    krate,
                    type_,
                    synthetic_lifetimes,
                    references,
                );
                collect_generic_bounds_path_references_with_elision(
                    krate,
                    bounds,
                    synthetic_lifetimes,
                    references,
                );
            }
            WherePredicate::LifetimePredicate { .. } => {}
            WherePredicate::EqPredicate { lhs, rhs } => {
                collect_type_path_references_with_elision(
                    krate,
                    lhs,
                    synthetic_lifetimes,
                    references,
                );
                collect_term_path_references_with_elision(
                    krate,
                    rhs,
                    synthetic_lifetimes,
                    references,
                );
            }
        }
    }
}

fn collect_term_path_references_with_elision(
    krate: &Crate,
    term: &Term,
    synthetic_lifetimes: &HashSet<String>,
    references: &mut Vec<ApiPathReference>,
) {
    if let Term::Type(type_) = term {
        collect_type_path_references_with_elision(krate, type_, synthetic_lifetimes, references);
    }
}

fn collect_type_path_references(
    krate: &Crate,
    type_: &Type,
    references: &mut Vec<ApiPathReference>,
) {
    collect_type_path_references_with_elision(krate, type_, &HashSet::new(), references);
}

fn collect_type_path_references_with_elision(
    krate: &Crate,
    type_: &Type,
    synthetic_lifetimes: &HashSet<String>,
    references: &mut Vec<ApiPathReference>,
) {
    match type_ {
        Type::ResolvedPath(path) => {
            collect_path_reference(krate, path, references);
            if let Some(args) = &path.args {
                collect_generic_args_path_references_with_elision(
                    krate,
                    args,
                    synthetic_lifetimes,
                    references,
                );
            }
        }
        Type::DynTrait(dyn_trait) => {
            for trait_ in &dyn_trait.traits {
                for parameter in &trait_.generic_params {
                    if synthetic_lifetimes.contains(&parameter.name) {
                        continue;
                    }
                    match &parameter.kind {
                        GenericParamDefKind::Type {
                            bounds, default, ..
                        } => {
                            collect_generic_bounds_path_references_with_elision(
                                krate,
                                bounds,
                                synthetic_lifetimes,
                                references,
                            );
                            if let Some(default) = default {
                                collect_type_path_references_with_elision(
                                    krate,
                                    default,
                                    synthetic_lifetimes,
                                    references,
                                );
                            }
                        }
                        GenericParamDefKind::Const { type_, .. } => {
                            collect_type_path_references_with_elision(
                                krate,
                                type_,
                                synthetic_lifetimes,
                                references,
                            );
                        }
                        GenericParamDefKind::Lifetime { .. } => {}
                    }
                }
                collect_path_reference(krate, &trait_.trait_, references);
                if let Some(args) = &trait_.trait_.args {
                    collect_generic_args_path_references_with_elision(
                        krate,
                        args,
                        synthetic_lifetimes,
                        references,
                    );
                }
            }
        }
        Type::FunctionPointer(pointer) => {
            for parameter in &pointer.generic_params {
                if synthetic_lifetimes.contains(&parameter.name) {
                    continue;
                }
                match &parameter.kind {
                    GenericParamDefKind::Type {
                        bounds, default, ..
                    } => {
                        collect_generic_bounds_path_references_with_elision(
                            krate,
                            bounds,
                            synthetic_lifetimes,
                            references,
                        );
                        if let Some(default) = default {
                            collect_type_path_references_with_elision(
                                krate,
                                default,
                                synthetic_lifetimes,
                                references,
                            );
                        }
                    }
                    GenericParamDefKind::Const { type_, .. } => {
                        collect_type_path_references_with_elision(
                            krate,
                            type_,
                            synthetic_lifetimes,
                            references,
                        );
                    }
                    GenericParamDefKind::Lifetime { .. } => {}
                }
            }
            for (_, input) in &pointer.sig.inputs {
                collect_type_path_references_with_elision(
                    krate,
                    input,
                    synthetic_lifetimes,
                    references,
                );
            }
            if let Some(output) = &pointer.sig.output {
                collect_type_path_references_with_elision(
                    krate,
                    output,
                    synthetic_lifetimes,
                    references,
                );
            }
        }
        Type::Tuple(types) => {
            for type_ in types {
                collect_type_path_references_with_elision(
                    krate,
                    type_,
                    synthetic_lifetimes,
                    references,
                );
            }
        }
        Type::Slice(type_)
        | Type::Pat { type_, .. }
        | Type::RawPointer { type_, .. }
        | Type::BorrowedRef { type_, .. } => {
            collect_type_path_references_with_elision(
                krate,
                type_,
                synthetic_lifetimes,
                references,
            );
        }
        Type::Array { type_, .. } => {
            collect_type_path_references_with_elision(
                krate,
                type_,
                synthetic_lifetimes,
                references,
            );
        }
        Type::ImplTrait(bounds) => {
            collect_generic_bounds_path_references_with_elision(
                krate,
                bounds,
                synthetic_lifetimes,
                references,
            );
        }
        Type::QualifiedPath {
            args,
            self_type,
            trait_,
            ..
        } => {
            collect_type_path_references_with_elision(
                krate,
                self_type,
                synthetic_lifetimes,
                references,
            );
            if let Some(trait_) = trait_ {
                collect_path_reference(krate, trait_, references);
                if let Some(args) = &trait_.args {
                    collect_generic_args_path_references_with_elision(
                        krate,
                        args,
                        synthetic_lifetimes,
                        references,
                    );
                }
            }
            if let Some(args) = args {
                collect_generic_args_path_references_with_elision(
                    krate,
                    args,
                    synthetic_lifetimes,
                    references,
                );
            }
        }
        Type::Generic(_) | Type::Primitive(_) | Type::Infer => {}
    }
}

fn collect_generic_args_path_references_with_elision(
    krate: &Crate,
    args: &GenericArgs,
    synthetic_lifetimes: &HashSet<String>,
    references: &mut Vec<ApiPathReference>,
) {
    match args {
        GenericArgs::AngleBracketed { args, constraints } => {
            for argument in args {
                match argument {
                    GenericArg::Type(type_) => {
                        collect_type_path_references_with_elision(
                            krate,
                            type_,
                            synthetic_lifetimes,
                            references,
                        );
                    }
                    GenericArg::Const(_) | GenericArg::Lifetime(_) | GenericArg::Infer => {}
                }
            }
            for constraint in constraints {
                if let Some(args) = &constraint.args {
                    collect_generic_args_path_references_with_elision(
                        krate,
                        args,
                        synthetic_lifetimes,
                        references,
                    );
                }
                match &constraint.binding {
                    rustdoc_types::AssocItemConstraintKind::Equality(term) => {
                        collect_term_path_references_with_elision(
                            krate,
                            term,
                            synthetic_lifetimes,
                            references,
                        );
                    }
                    rustdoc_types::AssocItemConstraintKind::Constraint(bounds) => {
                        collect_generic_bounds_path_references_with_elision(
                            krate,
                            bounds,
                            synthetic_lifetimes,
                            references,
                        );
                    }
                }
            }
        }
        GenericArgs::Parenthesized { inputs, output } => {
            for input in inputs {
                collect_type_path_references_with_elision(
                    krate,
                    input,
                    synthetic_lifetimes,
                    references,
                );
            }
            if let Some(output) = output {
                collect_type_path_references_with_elision(
                    krate,
                    output,
                    synthetic_lifetimes,
                    references,
                );
            }
        }
        GenericArgs::ReturnTypeNotation => {}
    }
}

fn collect_path_reference(krate: &Crate, path: &Path, references: &mut Vec<ApiPathReference>) {
    references.push(ApiPathReference {
        path: path.path.clone(),
        canonical_path: krate
            .paths
            .get(&path.id)
            .map(|summary| normalize_use_path(&summary.path)),
        target_source_id: Some(qualified_source_id(krate, path.id)),
    });
}

fn collect_trait_impl_declaration_path_references(
    krate: &Crate,
    impl_block: &Impl,
) -> Vec<ApiPathReference> {
    let mut references = Vec::new();
    collect_generic_params_path_references(krate, &impl_block.generics, &mut references);
    if let Some(trait_path) = &impl_block.trait_ {
        collect_path_reference(krate, trait_path, &mut references);
        if let Some(args) = &trait_path.args {
            collect_generic_args_path_references_with_elision(
                krate,
                args,
                &HashSet::new(),
                &mut references,
            );
        }
    }
    collect_type_path_references(krate, &impl_block.for_, &mut references);
    collect_where_predicates_path_references(
        krate,
        &impl_block.generics.where_predicates,
        &mut references,
    );
    references
}

fn collect_inherent_impl_declaration_path_references(
    krate: &Crate,
    impl_block: &Impl,
) -> Vec<ApiPathReference> {
    let mut references = Vec::new();
    collect_generic_params_path_references(krate, &impl_block.generics, &mut references);
    collect_type_path_references(krate, &impl_block.for_, &mut references);
    collect_where_predicates_path_references(
        krate,
        &impl_block.generics.where_predicates,
        &mut references,
    );
    references
}

fn collect_variant_declaration_path_references(
    krate: &Crate,
    variant_item: &Item,
) -> Vec<ApiPathReference> {
    let mut references = Vec::new();
    let ItemEnum::Variant(Variant { kind, .. }) = &variant_item.inner else {
        return references;
    };
    match kind {
        VariantKind::Plain => {}
        VariantKind::Tuple(fields) => {
            for field_id in fields.iter().flatten() {
                let Some(field_item) = krate.index.get(field_id) else {
                    continue;
                };
                let ItemEnum::StructField(type_) = &field_item.inner else {
                    continue;
                };
                collect_type_path_references(krate, type_, &mut references);
            }
        }
        VariantKind::Struct { fields, .. } => {
            for field_id in fields {
                let Some(field_item) = krate.index.get(field_id) else {
                    continue;
                };
                let ItemEnum::StructField(type_) = &field_item.inner else {
                    continue;
                };
                collect_type_path_references(krate, type_, &mut references);
            }
        }
    }
    references
}

fn collect_field_declaration_path_references(
    krate: &Crate,
    field_item: &Item,
) -> Vec<ApiPathReference> {
    let mut references = Vec::new();
    let ItemEnum::StructField(type_) = &field_item.inner else {
        return references;
    };
    collect_type_path_references(krate, type_, &mut references);
    references
}

fn collect_associated_member_declaration_path_references(
    krate: &Crate,
    item: &Item,
) -> Vec<ApiPathReference> {
    let mut references = Vec::new();
    match &item.inner {
        ItemEnum::Function(function) => {
            collect_function_declaration_path_references(krate, function, &mut references);
        }
        ItemEnum::AssocConst { type_, .. } => {
            collect_type_path_references(krate, type_, &mut references);
        }
        ItemEnum::AssocType {
            generics,
            bounds,
            type_,
            ..
        } => {
            collect_generic_params_path_references(krate, generics, &mut references);
            collect_generic_bounds_path_references(krate, bounds, &mut references);
            if let Some(type_) = type_ {
                collect_type_path_references(krate, type_, &mut references);
            }
            collect_where_predicates_path_references(
                krate,
                &generics.where_predicates,
                &mut references,
            );
        }
        _ => {}
    }
    references
}

fn synthesize_derive_attribute(krate: &Crate, item: &Item) -> Option<ApiAttribute> {
    let impl_ids = item_impl_ids(item)?;

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

fn synthesize_pin_project_attribute(
    krate: &Crate,
    item: &Item,
    existing_attributes: &[ApiAttribute],
) -> Option<ApiAttribute> {
    if existing_attributes
        .iter()
        .any(|attribute| attribute.text.starts_with("#[pin_project"))
    {
        return None;
    }

    match &item.inner {
        ItemEnum::Struct(struct_item) if struct_has_pin_fields(krate, struct_item) => {
            Some(ApiAttribute {
                text: "#[pin_project]".to_string(),
            })
        }
        ItemEnum::Enum(enum_item) if enum_has_pin_fields(krate, enum_item) => Some(ApiAttribute {
            text: "#[pin_project]".to_string(),
        }),
        _ => None,
    }
}

fn struct_has_pin_fields(krate: &Crate, struct_item: &rustdoc_types::Struct) -> bool {
    match &struct_item.kind {
        StructKind::Unit => false,
        StructKind::Tuple(fields) => fields
            .iter()
            .filter_map(|field_id| field_id.as_ref())
            .filter_map(|field_id| krate.index.get(field_id))
            .any(field_has_pin_attribute),
        StructKind::Plain { fields, .. } => fields
            .iter()
            .filter_map(|field_id| krate.index.get(field_id))
            .any(field_has_pin_attribute),
    }
}

fn enum_has_pin_fields(krate: &Crate, enum_item: &rustdoc_types::Enum) -> bool {
    enum_item
        .variants
        .iter()
        .filter_map(|variant_id| krate.index.get(variant_id))
        .any(|variant_item| variant_has_pin_fields(krate, variant_item))
}

fn variant_has_pin_fields(krate: &Crate, variant_item: &Item) -> bool {
    let ItemEnum::Variant(Variant { kind, .. }) = &variant_item.inner else {
        return false;
    };

    match kind {
        VariantKind::Plain => false,
        VariantKind::Tuple(fields) => fields
            .iter()
            .filter_map(|field_id| field_id.as_ref())
            .filter_map(|field_id| krate.index.get(field_id))
            .any(field_has_pin_attribute),
        VariantKind::Struct { fields, .. } => fields
            .iter()
            .filter_map(|field_id| krate.index.get(field_id))
            .any(field_has_pin_attribute),
    }
}

fn field_has_pin_attribute(field_item: &Item) -> bool {
    extract_attributes(field_item)
        .iter()
        .any(|attribute| attribute.text == "#[pin]")
}

fn has_automatically_derived(item: &Item) -> bool {
    rustdoc_compat::is_automatically_derived(item)
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

fn extract_trait_impl(
    krate: &Crate,
    item: &Item,
    impl_block: &Impl,
    owner: Option<&Item>,
    owner_kind: Option<ApiItemKind>,
) -> Option<ApiItem> {
    if has_automatically_derived(item) {
        return None;
    }

    let trait_path = impl_block.trait_.as_ref()?;
    let self_type = render_type(&impl_block.for_);
    let declaration = render_trait_impl_declaration(impl_block, trait_path, &self_type);

    Some(ApiItem {
        name: self_type,
        kind: ApiItemKind::TraitImpl,
        declaration_location: source_location(item),
        source_id: Some(qualified_source_id(krate, item.id)),
        navigation_paths: Vec::new(),
        owner_name: owner.map(|owner| {
            owner
                .name
                .clone()
                .unwrap_or_else(|| fallback_item_name(owner).to_string())
        }),
        owner_kind,
        owner_source_id: owner.map(|owner| qualified_source_id(krate, owner.id)),
        inherent_impl_sort_key: None,
        doc_comments: extract_doc_comments(item),
        attributes: extract_attributes(item),
        declaration,
        declaration_path_references: collect_trait_impl_declaration_path_references(
            krate, impl_block,
        ),
        members: extract_impl_items(krate, &impl_block.items),
    })
}

fn inherent_impls_for_item(krate: &Crate, target: &Item) -> Vec<ApiItem> {
    let Some(impl_ids) = item_impl_ids(target) else {
        return Vec::new();
    };
    let owner_kind = item_kind(target);

    impl_ids
        .iter()
        .filter_map(|impl_id| krate.index.get(impl_id))
        .filter_map(|impl_item| match &impl_item.inner {
            ItemEnum::Impl(impl_block) if include_inherent_impl_block(impl_block) => {
                extract_inherent_impl(krate, target, owner_kind, impl_item, impl_block)
            }
            _ => None,
        })
        .collect()
}

fn extract_inherent_impl(
    krate: &Crate,
    target: &Item,
    owner_kind: ApiItemKind,
    item: &Item,
    impl_block: &Impl,
) -> Option<ApiItem> {
    let self_type = render_type(&impl_block.for_);
    let members = extract_impl_items(krate, &impl_block.items);
    if members.is_empty() {
        return None;
    }

    Some(ApiItem {
        name: target
            .name
            .clone()
            .unwrap_or_else(|| fallback_item_name(target).to_string()),
        kind: ApiItemKind::InherentImpl,
        declaration_location: source_location(item),
        source_id: Some(qualified_source_id(krate, item.id)),
        navigation_paths: Vec::new(),
        owner_name: Some(
            target
                .name
                .clone()
                .unwrap_or_else(|| fallback_item_name(target).to_string()),
        ),
        owner_kind: Some(owner_kind),
        owner_source_id: Some(qualified_source_id(krate, target.id)),
        inherent_impl_sort_key: Some(inherent_impl_sort_key(&impl_block.for_)),
        doc_comments: extract_doc_comments(item),
        attributes: extract_attributes(item),
        declaration: render_inherent_impl_declaration(impl_block, &self_type),
        declaration_path_references: collect_inherent_impl_declaration_path_references(
            krate, impl_block,
        ),
        members,
    })
}

fn inherent_impl_sort_key(type_: &Type) -> InherentImplSortKey {
    InherentImplSortKey {
        type_arg_classes: inherent_impl_type_arg_classes(type_),
        rendered_self_type: render_type(type_),
    }
}

fn inherent_impl_type_arg_classes(type_: &Type) -> Vec<u8> {
    match type_ {
        Type::ResolvedPath(path) => match &path.args {
            Some(args) => match args.as_ref() {
                GenericArgs::AngleBracketed { args, .. } => args
                    .iter()
                    .filter_map(|arg| match arg {
                        GenericArg::Type(Type::Generic(_)) => Some(0),
                        GenericArg::Infer => Some(1),
                        GenericArg::Type(Type::Infer) => Some(1),
                        GenericArg::Type(_) => Some(2),
                        _ => None,
                    })
                    .collect(),
                _ => Vec::new(),
            },
            None => Vec::new(),
        },
        _ => Vec::new(),
    }
}

fn extract_impl_items(krate: &Crate, item_ids: &[Id]) -> Vec<ApiMember> {
    item_ids
        .iter()
        .filter_map(|item_id| krate.index.get(item_id))
        .filter(|item| is_visible(item))
        .filter_map(|item| extract_associated_member(krate, item))
        .collect()
}

fn extract_trait_members(krate: &Crate, trait_item: &Trait) -> Vec<ApiMember> {
    trait_item
        .items
        .iter()
        .filter_map(|item_id| krate.index.get(item_id))
        .filter_map(|item| extract_associated_member(krate, item))
        .collect()
}

fn extract_macro_members(item: &Item, source: &str) -> Vec<ApiMember> {
    let source = source_text_for_span(item).unwrap_or_else(|| source.to_string());
    parse_macro_definition(&source)
        .map(|parsed| {
            parsed
                .members
                .into_iter()
                .map(|(offset, mut member)| {
                    member.declaration_location = source_location_at_offset(item, &source, offset);
                    member
                })
                .collect()
        })
        .unwrap_or_default()
}

fn source_text_for_span(item: &Item) -> Option<String> {
    let span = item.span.as_ref()?;
    let source = crate::source_cache::get(&span.filename)?;
    let lines = source.lines().collect::<Vec<_>>();
    let start_line = span.begin.0.checked_sub(1)?;
    let end_line = span.end.0.min(lines.len());
    let mut selected = lines.get(start_line..end_line)?.to_vec();
    let start_column = span.begin.1.saturating_sub(1);
    let end_column = span.end.1.saturating_sub(1);
    if selected.len() == 1 {
        let line = *selected.first()?;
        let start_byte = byte_index_at_character(line, start_column);
        let end_byte = byte_index_at_character(line, end_column);
        *selected.first_mut()? = line.get(start_byte..end_byte)?;
    } else {
        let first = *selected.first()?;
        let start_byte = byte_index_at_character(first, start_column);
        *selected.first_mut()? = first.get(start_byte..)?;

        let last = *selected.last()?;
        let end_byte = byte_index_at_character(last, end_column);
        *selected.last_mut()? = last.get(..end_byte)?;
    }
    Some(selected.join("\n"))
}

fn byte_index_at_character(text: &str, character: usize) -> usize {
    text.char_indices()
        .nth(character)
        .map_or(text.len(), |(index, _)| index)
}

fn extract_proc_macro_members(
    item: &Item,
    proc_macro: &rustdoc_types::ProcMacro,
) -> Vec<ApiMember> {
    if !matches!(proc_macro.kind, MacroKind::Derive) || proc_macro.helpers.is_empty() {
        return Vec::new();
    }

    let mut members = vec![text_member(
        "available_attributes",
        "// Attributes available to this derive:",
    )];
    members.extend(proc_macro.helpers.iter().map(|helper| ApiMember {
        name: helper.clone(),
        kind: ApiMemberKind::MacroInput,
        declaration_location: proc_macro_helper_location(item, helper),
        doc_comments: Vec::new(),
        attributes: Vec::new(),
        declaration: format!("#[{helper}]"),
        declaration_path_references: Vec::new(),
    }));
    members
}

fn extract_struct_members(krate: &Crate, struct_item: &rustdoc_types::Struct) -> Vec<ApiMember> {
    let StructKind::Plain { fields, .. } = &struct_item.kind else {
        return Vec::new();
    };

    fields
        .iter()
        .filter_map(|field_id| krate.index.get(field_id))
        .filter_map(|field_item| extract_field_member(krate, field_item))
        .collect()
}

fn extract_union_members(krate: &Crate, union_item: &Union) -> Vec<ApiMember> {
    union_item
        .fields
        .iter()
        .filter_map(|field_id| krate.index.get(field_id))
        .filter_map(|field_item| extract_field_member(krate, field_item))
        .collect()
}

fn extract_enum_members(krate: &Crate, enum_item: &rustdoc_types::Enum) -> Vec<ApiMember> {
    enum_item
        .variants
        .iter()
        .filter_map(|variant_id| krate.index.get(variant_id))
        .map(|variant_item| ApiMember {
            name: variant_item.name.clone().unwrap_or_default(),
            kind: ApiMemberKind::Variant,
            declaration_location: source_location(variant_item),
            doc_comments: extract_doc_comments(variant_item),
            attributes: extract_attributes(variant_item),
            declaration: render_variant(krate, variant_item),
            declaration_path_references: collect_variant_declaration_path_references(
                krate,
                variant_item,
            ),
        })
        .collect()
}

fn extract_field_member(krate: &Crate, field_item: &Item) -> Option<ApiMember> {
    render_named_field(field_item).map(|declaration| ApiMember {
        name: field_item.name.clone().unwrap_or_default(),
        kind: ApiMemberKind::Field,
        declaration_location: source_location(field_item),
        doc_comments: extract_doc_comments(field_item),
        attributes: extract_attributes(field_item),
        declaration,
        declaration_path_references: collect_field_declaration_path_references(krate, field_item),
    })
}

fn api_member(krate: &Crate, item: &Item, kind: ApiMemberKind, declaration: String) -> ApiMember {
    ApiMember {
        name: item.name.clone().unwrap_or_default(),
        kind,
        declaration_location: source_location(item),
        doc_comments: extract_doc_comments(item),
        attributes: extract_attributes(item),
        declaration,
        declaration_path_references: collect_associated_member_declaration_path_references(
            krate, item,
        ),
    }
}

fn text_member(name: impl Into<String>, declaration: impl Into<String>) -> ApiMember {
    ApiMember {
        name: name.into(),
        kind: ApiMemberKind::Text,
        declaration_location: None,
        doc_comments: Vec::new(),
        attributes: Vec::new(),
        declaration: declaration.into(),
        declaration_path_references: Vec::new(),
    }
}

fn extract_associated_member(krate: &Crate, item: &Item) -> Option<ApiMember> {
    match &item.inner {
        ItemEnum::Function(function) => Some(api_member(
            krate,
            item,
            ApiMemberKind::Associated,
            render_function_declaration(
                item.name.as_deref().unwrap_or("unknown_fn"),
                function,
                false,
            ),
        )),
        ItemEnum::AssocConst { type_, value } => Some(api_member(
            krate,
            item,
            ApiMemberKind::Associated,
            render_assoc_const(
                item.name.as_deref().unwrap_or("UNKNOWN_CONST"),
                type_,
                value.as_deref(),
            ),
        )),
        ItemEnum::AssocType {
            generics,
            bounds,
            type_,
        } => Some(api_member(
            krate,
            item,
            ApiMemberKind::Associated,
            render_assoc_type(
                item.name.as_deref().unwrap_or("UnknownType"),
                generics,
                bounds,
                type_.as_ref(),
            ),
        )),
        _ => None,
    }
}

fn extract_attributes(item: &Item) -> Vec<ApiAttribute> {
    rustdoc_compat::attribute_texts(item)
        .into_iter()
        .map(|text| ApiAttribute {
            text: normalize_attribute(&text),
        })
        .collect()
}

fn extract_module_attributes(item: &Item, is_crate_root: bool) -> Vec<ApiAttribute> {
    rustdoc_compat::attribute_texts(item)
        .into_iter()
        .map(|text| ApiAttribute {
            text: normalize_module_attribute(&text, is_crate_root),
        })
        .collect()
}

fn item_impl_ids(item: &Item) -> Option<&[Id]> {
    match &item.inner {
        ItemEnum::Struct(struct_item) => Some(&struct_item.impls),
        ItemEnum::Enum(enum_item) => Some(&enum_item.impls),
        ItemEnum::Union(union_item) => Some(&union_item.impls),
        _ => None,
    }
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
        .replace("#![<cfg_attr>(", "#![cfg_attr(");

    normalized = normalize_pin_attribute(&normalized);
    normalized = collapse_attribute_whitespace(&normalized);
    normalized = collapse_path_separator_whitespace(&normalized);
    normalized = collapse_clippy_lint_whitespace(&normalized);
    normalized
}

fn normalize_module_attribute(text: &str, is_crate_root: bool) -> String {
    let normalized = normalize_attribute(text);
    if is_crate_root {
        if normalized.starts_with("#![") || normalized.starts_with("#[") {
            rewrite_attribute_prefix(&normalized, "#![")
        } else {
            normalized
        }
    } else if normalized.starts_with("#![") {
        rewrite_attribute_prefix(&normalized, "#[")
    } else {
        normalized
    }
}

fn rewrite_attribute_prefix(attribute: &str, prefix: &str) -> String {
    let Some(body) = attribute
        .strip_prefix("#![")
        .or_else(|| attribute.strip_prefix("#["))
        .and_then(|body| body.strip_suffix(']'))
    else {
        return attribute.to_string();
    };

    format!("{prefix}{body}]")
}

fn normalize_pin_attribute(attribute: &str) -> String {
    normalize_pin_attribute_with_prefix(attribute, "#[")
        .or_else(|| normalize_pin_attribute_with_prefix(attribute, "#!["))
        .unwrap_or_else(|| attribute.to_string())
}

fn normalize_pin_attribute_with_prefix(attribute: &str, prefix: &str) -> Option<String> {
    let body = attribute.strip_prefix(prefix)?;
    let body = body.strip_suffix(']')?;
    if body == "pin_project::pin_project" {
        return Some(format!("{prefix}pin_project]"));
    }
    if let Some(inner) = body.strip_prefix("pin_project::pin_project(") {
        let inner = inner.strip_suffix(')')?;
        return Some(format!("{prefix}pin_project({inner})]"));
    }
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

fn collapse_attribute_whitespace(attribute: &str) -> String {
    let mut normalized = String::new();
    let mut chars = attribute.chars().peekable();
    let mut in_string = false;
    let mut previous_was_escape = false;
    let mut pending_space = false;

    while let Some(ch) = chars.next() {
        if in_string {
            normalized.push(ch);
            if previous_was_escape {
                previous_was_escape = false;
            } else if ch == '\\' {
                previous_was_escape = true;
            } else if ch == '"' {
                in_string = false;
            }
            continue;
        }

        if ch == '"' {
            if pending_space && !normalized.is_empty() && !normalized.ends_with(['(', '[', '{']) {
                normalized.push(' ');
            }
            pending_space = false;
            in_string = true;
            normalized.push(ch);
            continue;
        }

        if ch.is_whitespace() {
            pending_space = !normalized.is_empty();
            while chars.next_if(|next| next.is_whitespace()).is_some() {}
            continue;
        }

        if pending_space
            && !normalized.is_empty()
            && !normalized.ends_with(['(', '[', '{'])
            && !matches!(ch, ')' | ']' | '}' | ',')
        {
            normalized.push(' ');
        }
        pending_space = false;
        normalized.push(ch);
    }

    normalized
}

fn collapse_path_separator_whitespace(attribute: &str) -> String {
    let mut normalized = String::new();
    let mut chars = attribute.chars().peekable();

    while let Some(ch) = chars.next() {
        if ch == ':' {
            // Strip any trailing space we just wrote before the first colon.
            while normalized.ends_with(' ') {
                normalized.pop();
            }
            normalized.push(':');

            // Skip whitespace between the two colons.
            while chars.peek().is_some_and(|c| c.is_whitespace()) {
                chars.next();
            }

            // Consume the second colon if present.
            if chars.peek() == Some(&':') {
                chars.next();
                normalized.push(':');
            }
            continue;
        }

        normalized.push(ch);
    }

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
    !impl_block.is_synthetic
        && impl_block.blanket_impl.is_none()
        && impl_block.trait_.is_some()
        && !is_pin_project_generated_unpin_impl(impl_block)
}

fn is_pin_project_generated_unpin_impl(impl_block: &Impl) -> bool {
    impl_block.trait_.as_ref().is_some_and(is_unpin_trait_path)
        && impl_block
            .generics
            .where_predicates
            .iter()
            .any(is_pin_project_generated_unpin_predicate)
}

fn is_unpin_trait_path(path: &Path) -> bool {
    matches!(
        path.path.as_str(),
        "Unpin" | "core::marker::Unpin" | "std::marker::Unpin"
    )
}

fn is_pin_project_generated_unpin_predicate(predicate: &WherePredicate) -> bool {
    match predicate {
        WherePredicate::BoundPredicate { type_, bounds, .. } => {
            is_pin_project_pinned_fields_type(type_)
                && bounds.iter().any(is_pin_project_private_unpin_bound)
        }
        _ => false,
    }
}

fn is_pin_project_pinned_fields_type(type_: &Type) -> bool {
    match type_ {
        Type::ResolvedPath(path) => matches!(
            path.path.as_str(),
            "_pin_project::__private::PinnedFieldsOf" | "pin_project::__private::PinnedFieldsOf"
        ),
        _ => false,
    }
}

fn is_pin_project_private_unpin_bound(bound: &GenericBound) -> bool {
    match bound {
        GenericBound::TraitBound { trait_, .. } => matches!(
            trait_.path.as_str(),
            "_pin_project::__private::Unpin" | "pin_project::__private::Unpin"
        ),
        _ => false,
    }
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
        ItemEnum::Macro(source) => render_macro_declaration(source),
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

fn render_macro_declaration(source: &str) -> String {
    parse_macro_definition(source)
        .map(|parsed| parsed.declaration)
        .unwrap_or_else(|| source.to_string())
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
                format!("#[derive({name})] {{")
            }
        }
    }
}

struct ParsedMacro {
    declaration: String,
    members: Vec<(usize, ApiMember)>,
}

fn parse_macro_definition(source: &str) -> Option<ParsedMacro> {
    let leading_whitespace = source.len() - source.trim_start().len();
    let source = source.trim();
    let open_index = source.find('{')?;
    let close_index = find_matching_delimiter(source, open_index, '{', '}')?;
    if !source[close_index + 1..].trim().is_empty() {
        return None;
    }

    let declaration = source[..=open_index].trim().to_string();
    let body = &source[open_index + 1..close_index];
    let members = split_macro_arms(body)
        .into_iter()
        .enumerate()
        .map(|(index, (offset, arm))| {
            (
                leading_whitespace + open_index + 1 + offset,
                ApiMember {
                    name: format!("arm_{index}"),
                    kind: ApiMemberKind::MacroInput,
                    declaration_location: None,
                    doc_comments: Vec::new(),
                    attributes: Vec::new(),
                    declaration: summarize_macro_arm(&arm),
                    declaration_path_references: Vec::new(),
                },
            )
        })
        .collect::<Vec<_>>();

    (!members.is_empty()).then_some(ParsedMacro {
        declaration,
        members,
    })
}

fn split_macro_arms(body: &str) -> Vec<(usize, String)> {
    let mut arms = Vec::new();
    let mut start = None;
    let mut paren_depth = 0usize;
    let mut bracket_depth = 0usize;
    let mut brace_depth = 0usize;

    for (index, character) in body.char_indices() {
        match character {
            '(' => {
                paren_depth += 1;
                start.get_or_insert(index);
            }
            ')' => paren_depth = paren_depth.saturating_sub(1),
            '[' => {
                bracket_depth += 1;
                start.get_or_insert(index);
            }
            ']' => bracket_depth = bracket_depth.saturating_sub(1),
            '{' => {
                brace_depth += 1;
                start.get_or_insert(index);
            }
            '}' => brace_depth = brace_depth.saturating_sub(1),
            ';' if paren_depth == 0 && bracket_depth == 0 && brace_depth == 0 => {
                if let Some(start_index) = start {
                    let arm = body[start_index..index].trim();
                    if !arm.is_empty() {
                        arms.push((start_index, arm.to_string()));
                    }
                }
                start = None;
            }
            character if !character.is_whitespace() => {
                start.get_or_insert(index);
            }
            _ => {}
        }
    }

    if let Some(start_index) = start {
        let arm = body[start_index..].trim();
        if !arm.is_empty() {
            arms.push((start_index, arm.to_string()));
        }
    }

    arms
}

fn summarize_macro_arm(arm: &str) -> String {
    if let Some(fat_arrow_index) = find_top_level_fat_arrow(arm) {
        let matcher = arm[..fat_arrow_index].trim();
        let expansion = arm[fat_arrow_index + 2..].trim();
        format!("{matcher} => {};", summarize_macro_expansion(expansion))
    } else {
        format!("{};", arm.trim())
    }
}

fn summarize_macro_expansion(expansion: &str) -> String {
    let expansion = expansion.trim();
    match expansion.chars().next() {
        Some('{') if expansion.ends_with('}') => "{ ... }".to_string(),
        Some('(') if expansion.ends_with(')') => "( ... )".to_string(),
        Some('[') if expansion.ends_with(']') => "[ ... ]".to_string(),
        _ => expansion.to_string(),
    }
}

fn find_top_level_fat_arrow(value: &str) -> Option<usize> {
    let mut paren_depth = 0usize;
    let mut bracket_depth = 0usize;
    let mut brace_depth = 0usize;
    let mut chars = value.char_indices().peekable();

    while let Some((index, character)) = chars.next() {
        match character {
            '(' => paren_depth += 1,
            ')' => paren_depth = paren_depth.saturating_sub(1),
            '[' => bracket_depth += 1,
            ']' => bracket_depth = bracket_depth.saturating_sub(1),
            '{' => brace_depth += 1,
            '}' => brace_depth = brace_depth.saturating_sub(1),
            '=' if paren_depth == 0 && bracket_depth == 0 && brace_depth == 0 => {
                if let Some((_, '>')) = chars.peek() {
                    return Some(index);
                }
            }
            _ => {}
        }
    }

    None
}

fn find_matching_delimiter(
    value: &str,
    open_index: usize,
    open_delimiter: char,
    close_delimiter: char,
) -> Option<usize> {
    let mut depth = 0usize;
    for (index, character) in value
        .char_indices()
        .skip_while(|(index, _)| *index < open_index)
    {
        if character == open_delimiter {
            depth += 1;
        } else if character == close_delimiter {
            depth = depth.checked_sub(1)?;
            if depth == 0 {
                return Some(index);
            }
        }
    }

    None
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
                render_function_parameter(argument_name, argument_type, &synthetic_lifetimes)
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

fn render_function_parameter(
    argument_name: &str,
    argument_type: &Type,
    synthetic_lifetimes: &HashSet<String>,
) -> String {
    if let Some(receiver) = render_self_parameter(argument_name, argument_type) {
        return receiver;
    }

    if argument_name.is_empty() {
        render_type_with_elision(argument_type, synthetic_lifetimes)
    } else {
        format!(
            "{argument_name}: {}",
            render_type_with_elision(argument_type, synthetic_lifetimes)
        )
    }
}

fn render_self_parameter(argument_name: &str, argument_type: &Type) -> Option<String> {
    if argument_name != "self" {
        return None;
    }

    match argument_type {
        Type::Generic(name) if name == "Self" => Some("self".to_string()),
        Type::BorrowedRef {
            is_mutable, type_, ..
        } if matches!(type_.as_ref(), Type::Generic(name) if name == "Self") => {
            Some(if *is_mutable { "&mut self" } else { "&self" }.to_string())
        }
        _ => None,
    }
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
            let tuple_fields = fields
                .iter()
                .map(|field_id| match field_id {
                    Some(field_id) => krate.index.get(field_id).and_then(render_tuple_field),
                    None => None,
                })
                .collect::<Vec<_>>();
            declaration.push('(');
            declaration.push_str(&join_rendered_fields(tuple_fields));
            declaration.push_str(");");
        }
        StructKind::Plain { fields: _, .. } => {
            declaration.push_str(" {");
        }
    }

    declaration
}

fn render_union_declaration(_krate: &Crate, item: &Item, union_item: &Union) -> String {
    let mut declaration = format!(
        "pub union {}{}",
        item.name.as_deref().unwrap_or("UnknownUnion"),
        render_generics_declaration(&union_item.generics)
    );
    declaration.push_str(&render_where_clause(&union_item.generics.where_predicates));
    declaration.push_str(" {");
    declaration
}

fn render_enum_declaration(_krate: &Crate, item: &Item, enum_item: &rustdoc_types::Enum) -> String {
    let mut declaration = format!(
        "pub enum {}{}",
        item.name.as_deref().unwrap_or("UnknownEnum"),
        render_generics_declaration(&enum_item.generics)
    );
    declaration.push_str(&render_where_clause(&enum_item.generics.where_predicates));
    declaration.push_str(" {");
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
                    .filter_map(render_variant_tuple_field)
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
                    .filter_map(render_variant_named_field)
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

fn render_inherent_impl_declaration(impl_block: &Impl, self_type: &str) -> String {
    let mut declaration = String::from("impl");
    declaration.push_str(&render_generics_declaration(&impl_block.generics));
    declaration.push(' ');
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
    for attribute in extract_attributes(field_item) {
        field.push_str(&attribute.text);
        field.push(' ');
    }
    if matches!(field_item.visibility, Visibility::Public) {
        field.push_str("pub ");
    }
    field.push_str(&render_type(type_));
    Some(field)
}

fn join_rendered_fields(fields: Vec<Option<String>>) -> String {
    fields
        .into_iter()
        .map(|field| field.unwrap_or_else(|| "/* private fields */".to_string()))
        .collect::<Vec<_>>()
        .join(", ")
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

fn render_variant_tuple_field(field_item: &Item) -> Option<String> {
    let ItemEnum::StructField(type_) = &field_item.inner else {
        return None;
    };

    Some(render_type(type_))
}

fn render_variant_named_field(field_item: &Item) -> Option<String> {
    let ItemEnum::StructField(type_) = &field_item.inner else {
        return None;
    };

    Some(format!(
        "{}: {}",
        field_item.name.as_deref().unwrap_or("unknown_field"),
        render_type(type_)
    ))
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
            if let Some(args) = args {
                rendered.push_str(&render_generic_args_with_elision(args, synthetic_lifetimes));
            }
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
    if let Some(args) = &constraint.args {
        rendered.push_str(&render_generic_args_with_elision(args, synthetic_lifetimes));
    }
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
mod tests;

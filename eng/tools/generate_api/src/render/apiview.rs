// Copyright (c) Microsoft Corporation. All rights reserved.
// Licensed under the MIT License.

use crate::model::{
    ApiItem, ApiItemKind, ApiMember, ApiMemberKind, ApiModel, ApiModule, ApiPathReference,
};
use serde::Serialize;
use std::collections::{BTreeMap, BTreeSet};

#[derive(Debug, Clone, Copy)]
pub(crate) struct RenderOptions {
    pub(crate) include_docs: bool,
}

fn prefers_navigation_target(existing: &NavigationTarget, candidate: &NavigationTarget) -> bool {
    navigation_target_rank(candidate) > navigation_target_rank(existing)
}

fn navigation_target_rank(target: &NavigationTarget) -> (usize, usize) {
    let declaration_rank = usize::from(!matches!(target.kind, Some(ApiItemKind::Use)));
    (declaration_rank, target.path_depth)
}

fn path_depth(path: &str) -> usize {
    path.split("::").count()
}

impl Default for RenderOptions {
    fn default() -> Self {
        Self { include_docs: true }
    }
}

impl RenderOptions {
    pub(crate) fn new(include_docs: bool) -> Self {
        Self { include_docs }
    }
}

#[derive(Debug, Serialize)]
struct CodeFile<'a> {
    #[serde(rename = "PackageName")]
    package_name: &'a str,
    #[serde(rename = "PackageVersion")]
    package_version: &'a str,
    #[serde(rename = "ParserVersion")]
    parser_version: &'a str,
    #[serde(rename = "Language")]
    language: &'static str,
    #[serde(rename = "ReviewLines")]
    review_lines: Vec<ReviewLine>,
}

#[derive(Clone, Debug, Serialize)]
struct ReviewLine {
    #[serde(rename = "LineId", skip_serializing_if = "Option::is_none")]
    line_id: Option<String>,
    #[serde(rename = "Tokens")]
    tokens: Vec<ReviewToken>,
    #[serde(rename = "Children", skip_serializing_if = "Vec::is_empty")]
    children: Vec<ReviewLine>,
    #[serde(rename = "IsContextEndLine", skip_serializing_if = "Option::is_none")]
    is_context_end_line: Option<bool>,
    #[serde(rename = "RelatedToLine", skip_serializing_if = "Option::is_none")]
    related_to_line: Option<String>,
}

#[derive(Clone, Debug, Serialize)]
struct ReviewToken {
    #[serde(rename = "Kind")]
    kind: u8,
    #[serde(rename = "Value")]
    value: String,
    #[serde(rename = "HasPrefixSpace")]
    has_prefix_space: bool,
    #[serde(rename = "HasSuffixSpace")]
    has_suffix_space: bool,
    #[serde(rename = "IsDocumentation", skip_serializing_if = "std::ops::Not::not")]
    is_documentation: bool,
    #[serde(
        rename = "NavigationDisplayName",
        skip_serializing_if = "Option::is_none"
    )]
    navigation_display_name: Option<String>,
    #[serde(rename = "NavigateToId", skip_serializing_if = "Option::is_none")]
    navigate_to_id: Option<String>,
    #[serde(rename = "RenderClasses", skip_serializing_if = "Option::is_none")]
    render_classes: Option<Vec<String>>,
}

pub(crate) fn render(model: &ApiModel, options: &RenderOptions) -> Result<String, String> {
    let navigation_lookup = NavigationLookup::new(model);
    let document = CodeFile {
        package_name: &model.package_name,
        package_version: &model.package_version,
        parser_version: &model.parser_version,
        language: "Rust",
        review_lines: render_all_review_lines(model, options, &navigation_lookup),
    };
    validate_code_file(&document)?;

    serde_json::to_string_pretty(&document)
        .map_err(|error| format!("Failed to serialize APIView JSON: {error}"))
}

fn validate_code_file(document: &CodeFile<'_>) -> Result<(), String> {
    if document.package_name.is_empty() {
        return Err("APIView output is missing PackageName".to_string());
    }
    if document.package_version.is_empty() {
        return Err("APIView output is missing PackageVersion".to_string());
    }
    if document.parser_version.is_empty() {
        return Err("APIView output is missing ParserVersion".to_string());
    }

    let mut line_ids = BTreeSet::new();
    validate_review_lines(&document.review_lines, &mut line_ids)
}

fn validate_review_lines(
    lines: &[ReviewLine],
    line_ids: &mut BTreeSet<String>,
) -> Result<(), String> {
    for line in lines {
        if line.tokens.is_empty() {
            return Err("APIView output contained a review line with no tokens".to_string());
        }
        if let Some(line_id) = &line.line_id {
            if !line_ids.insert(line_id.clone()) {
                return Err(format!(
                    "APIView output contained a duplicate LineId: {line_id}"
                ));
            }
        }

        validate_review_lines(&line.children, line_ids)?;
    }

    Ok(())
}

fn render_review_lines(
    model: &ApiModel,
    options: &RenderOptions,
    navigation_lookup: &NavigationLookup,
) -> Vec<ReviewLine> {
    render_root_module(&model.root_module, options, navigation_lookup)
}

fn render_all_review_lines(
    model: &ApiModel,
    options: &RenderOptions,
    navigation_lookup: &NavigationLookup,
) -> Vec<ReviewLine> {
    let mut lines = render_package_metadata(model);
    lines.extend(render_review_lines(model, options, navigation_lookup));
    lines
}

fn render_package_metadata(model: &ApiModel) -> Vec<ReviewLine> {
    let mut lines = Vec::new();
    let metadata = &model.package_metadata;
    if let Some(description) = metadata.description_lines() {
        if description.len() == 1 {
            lines.push(metadata_line(&format!("Description: {}", description[0])));
        } else {
            lines.push(metadata_line("Description:"));
            for line in description {
                lines.push(metadata_line(line));
            }
        }
    }
    if let Some(edition) = &metadata.edition {
        lines.push(metadata_line(&format!("Edition: {edition}")));
    }
    if let Some(rust_version) = &metadata.rust_version {
        lines.push(metadata_line(&format!("Rust version: {rust_version}")));
    }

    lines.push(metadata_line("Features:"));
    for feature in metadata.feature_names() {
        lines.push(metadata_line(&format!("- {feature}")));
        if feature == "default" {
            for child in metadata.default_feature_children() {
                lines.push(metadata_line(&format!("  - {child}")));
            }
        }
    }
    lines.push(metadata_line(""));
    lines
}

fn metadata_line(value: &str) -> ReviewLine {
    ReviewLine {
        line_id: None,
        tokens: vec![ReviewToken {
            kind: token_kind::TEXT,
            value: value.to_string(),
            has_prefix_space: false,
            has_suffix_space: false,
            is_documentation: false,
            navigation_display_name: None,
            navigate_to_id: None,
            render_classes: None,
        }],
        children: Vec::new(),
        is_context_end_line: None,
        related_to_line: None,
    }
}

fn render_module_contents(
    module: &ApiModule,
    options: &RenderOptions,
    navigation_lookup: &NavigationLookup,
) -> Vec<ReviewLine> {
    let mut lines = Vec::new();
    let mut item_line_id_counts = BTreeMap::new();
    for entry in apiview_tree_entries(module) {
        match entry {
            ModuleTreeEntry::Item(item) => {
                let line_id = allocate_unique_line_id(
                    item_line_id_base(module, item),
                    &mut item_line_id_counts,
                );
                lines.extend(render_item(
                    module,
                    item,
                    line_id,
                    item_in_tree(item.kind),
                    options,
                    navigation_lookup,
                ));
            }
            ModuleTreeEntry::Module(child) => {
                lines.extend(render_module(child, options, navigation_lookup));
            }
        }
    }
    lines
}

fn render_root_module(
    module: &ApiModule,
    options: &RenderOptions,
    navigation_lookup: &NavigationLookup,
) -> Vec<ReviewLine> {
    let mut lines = Vec::new();
    if options.include_docs {
        lines.extend(render_doc_comment_lines(&module.doc_comments, None));
    }
    for attribute in &module.attributes {
        lines.push(ReviewLine {
            line_id: None,
            tokens: tokenize_line(&attribute.text, "", token_kind::TYPE_NAME),
            children: Vec::new(),
            is_context_end_line: None,
            related_to_line: None,
        });
    }
    lines.extend(render_module_contents(module, options, navigation_lookup));
    lines
}

fn render_module(
    module: &ApiModule,
    options: &RenderOptions,
    navigation_lookup: &NavigationLookup,
) -> Vec<ReviewLine> {
    let line_id = module_line_id(&module.path);
    let mut lines = Vec::new();
    if options.include_docs {
        lines.extend(render_doc_comment_lines(
            &module.doc_comments,
            Some(line_id.clone()),
        ));
    }

    for attribute in &module.attributes {
        lines.push(ReviewLine {
            line_id: None,
            tokens: tokenize_line(&attribute.text, "", token_kind::TYPE_NAME),
            children: Vec::new(),
            is_context_end_line: None,
            related_to_line: Some(line_id.clone()),
        });
    }
    let mut tokens = tokenize_line(
        &format!("pub mod {} {{", module.local_name()),
        module.local_name(),
        token_kind::TYPE_NAME,
    );
    annotate_navigation_token(
        &mut tokens,
        module.local_name(),
        token_kind::TYPE_NAME,
        module.local_name(),
        &line_id,
        "namespace",
        NavigationMatch::First,
    );

    lines.push(ReviewLine {
        line_id: Some(line_id.clone()),
        tokens,
        children: render_module_contents(module, options, navigation_lookup),
        is_context_end_line: None,
        related_to_line: None,
    });
    lines.push(ReviewLine {
        line_id: None,
        tokens: tokenize_line("}", "", token_kind::TYPE_NAME),
        children: Vec::new(),
        is_context_end_line: Some(true),
        related_to_line: Some(line_id),
    });
    lines
}

enum ModuleTreeEntry<'a> {
    Item(&'a ApiItem),
    Module(&'a ApiModule),
}

fn render_item(
    module: &ApiModule,
    item: &ApiItem,
    line_id: String,
    should_render_tree_node: bool,
    options: &RenderOptions,
    navigation_lookup: &NavigationLookup,
) -> Vec<ReviewLine> {
    let name_token_kind = item_name_token_kind(item.kind);
    let mut lines = Vec::new();

    if options.include_docs {
        lines.extend(render_doc_comment_lines(
            &item.doc_comments,
            Some(line_id.clone()),
        ));
    }

    for attribute in &item.attributes {
        lines.push(ReviewLine {
            line_id: None,
            tokens: tokenize_line(&attribute.text, "", token_kind::TYPE_NAME),
            children: Vec::new(),
            is_context_end_line: None,
            related_to_line: Some(line_id.clone()),
        });
    }

    let members = sorted_members(&item.members);
    let declaration_owns_members = item.declaration.trim_end().ends_with('{');
    let member_related_line = if declaration_owns_members {
        line_id.clone()
    } else {
        format!("{line_id}.impl")
    };
    let member_lines =
        render_member_lines(&members, &member_related_line, options, navigation_lookup);
    let tree_render_class = item_render_class(item.kind);
    let mut tree_navigation_emitted = false;
    let navigation_match = item_navigation_match(item.kind);

    for (declaration_index, declaration_line) in item.declaration.lines().enumerate() {
        if declaration_line.trim().is_empty() {
            continue;
        }

        let mut tokens = tokenize_line(declaration_line, &item.name, name_token_kind);
        if should_render_tree_node && !tree_navigation_emitted {
            let navigate_target =
                navigation_target_for_item(module, item, &line_id, navigation_lookup);
            tree_navigation_emitted = annotate_navigation_token(
                &mut tokens,
                &item.name,
                name_token_kind,
                &item.name,
                navigate_target.map_or(line_id.as_str(), |target| target.line_id.as_str()),
                tree_render_class,
                navigation_match,
            );
        }
        annotate_path_references(
            &mut tokens,
            &item.declaration_path_references,
            navigation_lookup,
        );
        annotate_trait_impl_owner_reference(&mut tokens, module, item, navigation_lookup);
        annotate_reference_tokens(
            &mut tokens,
            navigation_lookup,
            item.declaration_path_references.is_empty(),
        );

        lines.push(ReviewLine {
            line_id: if declaration_index == 0 {
                Some(line_id.clone())
            } else {
                None
            },
            tokens,
            children: if declaration_index == 0 && declaration_owns_members {
                member_lines.clone()
            } else {
                Vec::new()
            },
            is_context_end_line: None,
            related_to_line: if declaration_index == 0 {
                None
            } else {
                Some(line_id.clone())
            },
        });
    }

    if declaration_owns_members {
        lines.push(ReviewLine {
            line_id: None,
            tokens: tokenize_line("}", "", token_kind::TYPE_NAME),
            children: Vec::new(),
            is_context_end_line: Some(true),
            related_to_line: Some(line_id),
        });
    } else if !member_lines.is_empty() {
        lines.push(ReviewLine {
            line_id: Some(member_related_line.clone()),
            tokens: tokenize_line(
                &format!("impl {} {{", item.name),
                &item.name,
                token_kind::TYPE_NAME,
            ),
            children: member_lines,
            is_context_end_line: None,
            related_to_line: Some(line_id.clone()),
        });
        lines.push(ReviewLine {
            line_id: None,
            tokens: tokenize_line("}", "", token_kind::TYPE_NAME),
            children: Vec::new(),
            is_context_end_line: Some(true),
            related_to_line: Some(member_related_line),
        });
    }

    lines
}

fn render_member_lines(
    members: &[&ApiMember],
    related_to_line: &str,
    options: &RenderOptions,
    navigation_lookup: &NavigationLookup,
) -> Vec<ReviewLine> {
    let mut lines = Vec::new();
    let mut member_line_id_counts = BTreeMap::new();
    for member in members.iter() {
        if options.include_docs {
            lines.extend(render_doc_comment_lines(
                &member.doc_comments,
                Some(related_to_line.to_string()),
            ));
        }

        for attribute in &member.attributes {
            lines.push(ReviewLine {
                line_id: None,
                tokens: tokenize_line(&attribute.text, "", token_kind::TYPE_NAME),
                children: Vec::new(),
                is_context_end_line: None,
                related_to_line: Some(related_to_line.to_string()),
            });
        }

        let mut tokens = tokenize_line(
            &member.declaration,
            &member.name,
            member_name_token_kind(member.kind),
        );
        annotate_path_references(
            &mut tokens,
            &member.declaration_path_references,
            navigation_lookup,
        );
        annotate_reference_tokens(
            &mut tokens,
            navigation_lookup,
            member.declaration_path_references.is_empty(),
        );
        let line_id = allocate_unique_line_id(
            member_line_id_base(related_to_line, member),
            &mut member_line_id_counts,
        );
        lines.push(ReviewLine {
            line_id: Some(line_id),
            tokens,
            children: Vec::new(),
            is_context_end_line: None,
            related_to_line: Some(related_to_line.to_string()),
        });
    }
    lines
}

fn render_doc_comment_lines(
    doc_comments: &[String],
    related_to_line: Option<String>,
) -> Vec<ReviewLine> {
    doc_comments
        .iter()
        .map(|comment| ReviewLine {
            line_id: None,
            tokens: vec![doc_token(comment)],
            children: Vec::new(),
            is_context_end_line: None,
            related_to_line: related_to_line.clone(),
        })
        .collect()
}

fn module_line_id(path: &str) -> String {
    format!("module.{}", sanitize(path))
}

fn item_line_id_base(module: &ApiModule, item: &ApiItem) -> String {
    let module_line_id = module_line_id(&module.path);
    match item.kind {
        ApiItemKind::Use => {
            format!("{module_line_id}.reexport.{}", sanitize_segment(&item.name))
        }
        ApiItemKind::InherentImpl | ApiItemKind::TraitImpl => format!(
            "{module_line_id}.{}.{}",
            item_kind_slug(item.kind),
            sanitize_segment(&normalized_declaration_identity(&item.declaration)),
        ),
        _ => format!(
            "{module_line_id}.{}.{}",
            item_kind_slug(item.kind),
            sanitize_segment(&item.name),
        ),
    }
}

fn member_line_id_base(related_to_line: &str, member: &ApiMember) -> String {
    let identity = match member.kind {
        ApiMemberKind::Field | ApiMemberKind::Variant | ApiMemberKind::Associated => {
            sanitize_segment(&member.name)
        }
        ApiMemberKind::MacroInput | ApiMemberKind::Text => {
            sanitize_segment(&normalized_declaration_identity(&member.declaration))
        }
    };
    format!(
        "{related_to_line}.{}.{}",
        member_kind_slug(member.kind),
        identity
    )
}

fn allocate_unique_line_id(base: String, counts: &mut BTreeMap<String, usize>) -> String {
    let count = counts.entry(base.clone()).or_default();
    let line_id = if *count == 0 {
        base
    } else {
        format!("{base}.alt{count}")
    };
    *count += 1;
    line_id
}

fn item_kind_slug(kind: ApiItemKind) -> &'static str {
    match kind {
        ApiItemKind::Use => "reexport",
        ApiItemKind::Macro => "macro",
        ApiItemKind::ProcMacro => "proc_macro",
        ApiItemKind::Function => "function",
        ApiItemKind::Struct => "struct",
        ApiItemKind::Enum => "enum",
        ApiItemKind::Trait => "trait",
        ApiItemKind::TraitAlias => "trait_alias",
        ApiItemKind::InherentImpl => "inherent_impl",
        ApiItemKind::TraitImpl => "trait_impl",
        ApiItemKind::Union => "union",
        ApiItemKind::TypeAlias => "type_alias",
        ApiItemKind::Const => "const",
        ApiItemKind::Static => "static",
    }
}

fn member_kind_slug(kind: ApiMemberKind) -> &'static str {
    match kind {
        ApiMemberKind::Associated => "associated",
        ApiMemberKind::Field => "field",
        ApiMemberKind::Variant => "variant",
        ApiMemberKind::MacroInput => "macro_input",
        ApiMemberKind::Text => "text",
    }
}

fn normalized_declaration_identity(declaration: &str) -> String {
    let declaration = declaration
        .lines()
        .map(str::trim)
        .filter(|line| !line.is_empty())
        .collect::<Vec<_>>()
        .join(" ");
    declaration
        .trim_end_matches('{')
        .trim_end_matches(';')
        .trim()
        .to_string()
}

fn sanitize_segment(value: &str) -> String {
    let mut segment = String::new();
    let mut previous_was_separator = false;
    for character in value.chars() {
        if character.is_ascii_alphanumeric() {
            segment.push(character);
            previous_was_separator = false;
        } else if !previous_was_separator && !segment.is_empty() {
            segment.push('_');
            previous_was_separator = true;
        }
    }

    segment.trim_matches('_').to_string()
}

fn sanitize(value: &str) -> String {
    value
        .chars()
        .map(|character| match character {
            'a'..='z' | 'A'..='Z' | '0'..='9' => character,
            _ => '_',
        })
        .collect()
}

mod token_kind {
    pub const TEXT: u8 = 0;
    pub const PUNCTUATION: u8 = 1;
    pub const KEYWORD: u8 = 2;
    pub const TYPE_NAME: u8 = 3;
    pub const MEMBER_NAME: u8 = 4;
    pub const COMMENT: u8 = 7;
}

const RUST_KEYWORDS: &[&str] = &[
    "as", "async", "auto", "const", "crate", "derive", "dyn", "enum", "extern", "false", "fn",
    "for", "impl", "in", "mod", "move", "mut", "pub", "ref", "self", "Self", "static", "struct",
    "super", "trait", "true", "type", "union", "unsafe", "use", "where",
];

fn is_rust_keyword(s: &str) -> bool {
    RUST_KEYWORDS.contains(&s)
}

fn item_name_token_kind(kind: ApiItemKind) -> u8 {
    match kind {
        ApiItemKind::Function => token_kind::MEMBER_NAME,
        _ => token_kind::TYPE_NAME,
    }
}

fn member_name_token_kind(kind: ApiMemberKind) -> u8 {
    match kind {
        ApiMemberKind::Associated | ApiMemberKind::Field => token_kind::MEMBER_NAME,
        ApiMemberKind::Variant | ApiMemberKind::MacroInput | ApiMemberKind::Text => {
            token_kind::TYPE_NAME
        }
    }
}

fn sorted_members(members: &[ApiMember]) -> Vec<&ApiMember> {
    let mut indexed = members.iter().enumerate().collect::<Vec<_>>();
    indexed.sort_by(
        |(left_index, left), (right_index, right)| match (left.kind, right.kind) {
            (ApiMemberKind::Associated, ApiMemberKind::Associated) => left
                .name
                .cmp(&right.name)
                .then_with(|| left.declaration.cmp(&right.declaration)),
            _ => left_index.cmp(right_index),
        },
    );
    indexed.into_iter().map(|(_, member)| member).collect()
}

fn tokenize_line(line: &str, item_name: &str, name_token_kind: u8) -> Vec<ReviewToken> {
    let mut tokens: Vec<ReviewToken> = Vec::new();
    let mut s = line.trim_start();
    let mut name_emitted = false;

    while !s.is_empty() {
        let trimmed = s.trim_start();
        let has_prefix_space = trimmed.len() < s.len() && !tokens.is_empty();
        s = trimmed;

        if s.is_empty() {
            break;
        }

        let (kind, len) = next_token_kind(s, item_name, name_token_kind, &mut name_emitted);
        tokens.push(ReviewToken {
            kind,
            value: s[..len].to_string(),
            has_prefix_space,
            has_suffix_space: false,
            is_documentation: false,
            navigation_display_name: None,
            navigate_to_id: None,
            render_classes: None,
        });
        s = &s[len..];
    }

    tokens
}

fn next_token_kind(
    s: &str,
    item_name: &str,
    name_token_kind: u8,
    name_emitted: &mut bool,
) -> (u8, usize) {
    if s.starts_with("/*") {
        let len = s.find("*/").map_or(s.len(), |index| index + 2);
        return (token_kind::COMMENT, len);
    }

    // Multi-character punctuation sequences
    if s.starts_with("::") {
        return (token_kind::PUNCTUATION, 2);
    }
    if s.starts_with("->") {
        return (token_kind::PUNCTUATION, 2);
    }
    if s.starts_with("=>") {
        return (token_kind::PUNCTUATION, 2);
    }
    if s.starts_with("..=") {
        return (token_kind::PUNCTUATION, 3);
    }
    if s.starts_with("..") {
        return (token_kind::PUNCTUATION, 2);
    }

    let ch = s.chars().next().expect("non-empty string");

    // Identifier or keyword
    if ch.is_alphabetic() || ch == '_' {
        let end = s
            .find(|c: char| !c.is_alphanumeric() && c != '_')
            .unwrap_or(s.len());
        let word = &s[..end];
        let kind = if is_rust_keyword(word) {
            token_kind::KEYWORD
        } else if word == item_name && !*name_emitted {
            *name_emitted = true;
            name_token_kind
        } else {
            token_kind::TYPE_NAME
        };
        return (kind, end);
    }

    // Lifetime: 'a, 'static, 'async_trait, '_
    if ch == '\'' {
        let rest = &s[1..];
        if rest.starts_with(|c: char| c.is_alphabetic() || c == '_') {
            let inner_end = rest
                .find(|c: char| !c.is_alphanumeric() && c != '_')
                .unwrap_or(rest.len());
            return (token_kind::TYPE_NAME, 1 + inner_end);
        }
        return (token_kind::PUNCTUATION, 1);
    }

    // String literal (ABI strings, default values)
    if ch == '"' {
        let mut end = 1;
        let mut chars = s[1..].char_indices();
        while let Some((i, c)) = chars.next() {
            if c == '\\' {
                chars.next(); // skip escaped char
            } else if c == '"' {
                end = 1 + i + c.len_utf8();
                break;
            }
        }
        return (token_kind::TEXT, end);
    }

    // Numeric literals (in discriminants, array lengths, const values)
    if ch.is_ascii_digit() {
        let end = s
            .find(|c: char| !c.is_alphanumeric() && c != '_' && c != '.')
            .unwrap_or(s.len());
        return (token_kind::TEXT, end.max(1));
    }

    // Punctuation characters
    if "{}()<>[],;=+*&|!?@#:./-\\^%~".contains(ch) {
        return (token_kind::PUNCTUATION, ch.len_utf8());
    }

    // Anything else: single character as text
    (token_kind::TEXT, ch.len_utf8())
}

fn doc_token(value: &str) -> ReviewToken {
    ReviewToken {
        kind: token_kind::COMMENT,
        value: value.to_string(),
        has_prefix_space: false,
        has_suffix_space: false,
        is_documentation: true,
        navigation_display_name: None,
        navigate_to_id: None,
        render_classes: None,
    }
}

fn apiview_sorted_items(module: &ApiModule) -> Vec<&ApiItem> {
    let sorted_items = module.sorted_items();
    let mut consts_and_statics = Vec::new();
    let mut aliases_and_reexports = Vec::new();
    let mut macros = Vec::new();
    let mut functions = Vec::new();
    let mut type_like_items = Vec::new();
    let mut impls_by_owner = std::collections::BTreeMap::<&str, Vec<&ApiItem>>::new();
    let mut trailing_impls = Vec::new();

    for item in &sorted_items {
        match item.kind {
            ApiItemKind::Const | ApiItemKind::Static => consts_and_statics.push(*item),
            ApiItemKind::TypeAlias | ApiItemKind::Use => aliases_and_reexports.push(*item),
            ApiItemKind::Macro | ApiItemKind::ProcMacro => macros.push(*item),
            ApiItemKind::Function => functions.push(*item),
            ApiItemKind::InherentImpl | ApiItemKind::TraitImpl => {
                impls_by_owner
                    .entry(item.owner_name.as_deref().unwrap_or(item.name.as_str()))
                    .or_default()
                    .push(*item);
            }
            _ => type_like_items.push(*item),
        }
    }

    sort_tree_bucket(&mut consts_and_statics);
    sort_tree_bucket(&mut aliases_and_reexports);
    sort_tree_bucket(&mut macros);
    sort_tree_bucket(&mut functions);
    sort_tree_bucket(&mut type_like_items);

    let mut items = Vec::new();
    items.extend(consts_and_statics);
    items.extend(aliases_and_reexports);
    items.extend(macros);
    items.extend(functions);

    for item in type_like_items {
        items.push(item);
        if let Some(impls) = impls_by_owner.remove(item.name.as_str()) {
            items.extend(impls);
        }
    }

    for item in sorted_items {
        let owner_name = item.owner_name.as_deref().unwrap_or(item.name.as_str());
        if matches!(
            item.kind,
            ApiItemKind::InherentImpl | ApiItemKind::TraitImpl
        ) && impls_by_owner.contains_key(owner_name)
        {
            trailing_impls.push(item);
            impls_by_owner.remove(owner_name);
        }
    }
    items.extend(trailing_impls);

    items
}

fn apiview_tree_entries(module: &ApiModule) -> Vec<ModuleTreeEntry<'_>> {
    let mut entries = apiview_sorted_items(module)
        .into_iter()
        .map(ModuleTreeEntry::Item)
        .collect::<Vec<_>>();
    let mut child_modules = module.modules.iter().collect::<Vec<_>>();
    child_modules.sort_by(|left, right| left.path.cmp(&right.path));
    entries.extend(child_modules.into_iter().map(ModuleTreeEntry::Module));
    entries
}

fn sort_tree_bucket(items: &mut Vec<&ApiItem>) {
    items.sort_by(|left, right| {
        left.name
            .cmp(&right.name)
            .then_with(|| {
                left.kind
                    .sort_rank(left.owner_kind)
                    .cmp(&right.kind.sort_rank(right.owner_kind))
            })
            .then_with(|| left.declaration.cmp(&right.declaration))
    });
}

fn item_in_tree(kind: ApiItemKind) -> bool {
    !matches!(kind, ApiItemKind::InherentImpl | ApiItemKind::TraitImpl)
}

fn item_render_class(kind: ApiItemKind) -> &'static str {
    match kind {
        ApiItemKind::Function => "method",
        ApiItemKind::Struct | ApiItemKind::Union => "class",
        ApiItemKind::Enum => "enum",
        ApiItemKind::Trait | ApiItemKind::TraitAlias => "interface",
        ApiItemKind::Const
        | ApiItemKind::Static
        | ApiItemKind::TypeAlias
        | ApiItemKind::Use
        | ApiItemKind::Macro
        | ApiItemKind::ProcMacro => "type",
        ApiItemKind::InherentImpl | ApiItemKind::TraitImpl => "interface",
    }
}

#[derive(Clone)]
struct NavigationTarget {
    line_id: String,
    display_name: String,
    render_class: &'static str,
    kind: Option<ApiItemKind>,
    path_depth: usize,
}

struct NavigationLookup {
    root_path: String,
    paths: BTreeMap<String, NavigationTarget>,
    visible_source_ids: BTreeMap<String, NavigationTarget>,
    simple_names: BTreeMap<String, Option<NavigationTarget>>,
}

impl NavigationLookup {
    fn new(model: &ApiModel) -> Self {
        let mut lookup = Self {
            root_path: model.root_module.path.clone(),
            paths: BTreeMap::new(),
            visible_source_ids: BTreeMap::new(),
            simple_names: BTreeMap::new(),
        };
        collect_navigation_targets(&mut lookup, &model.root_module);
        lookup
    }

    fn insert_path(&mut self, path: impl Into<String>, target: &NavigationTarget) {
        let path = path.into();
        match self.paths.entry(path) {
            std::collections::btree_map::Entry::Vacant(entry) => {
                entry.insert(target.clone());
            }
            std::collections::btree_map::Entry::Occupied(mut entry) => {
                if prefers_navigation_target(entry.get(), target) {
                    entry.insert(target.clone());
                }
            }
        }
    }

    fn insert_simple_name(&mut self, name: &str, target: &NavigationTarget) {
        use std::collections::btree_map::Entry;

        match self.simple_names.entry(name.to_string()) {
            Entry::Vacant(entry) => {
                entry.insert(Some(target.clone()));
            }
            Entry::Occupied(mut entry) => {
                if entry
                    .get()
                    .as_ref()
                    .is_some_and(|existing| existing.line_id == target.line_id)
                {
                    return;
                }
                entry.insert(None);
            }
        }
    }

    fn insert_visible_source_id(&mut self, source_id: &str, target: &NavigationTarget) {
        match self.visible_source_ids.entry(source_id.to_string()) {
            std::collections::btree_map::Entry::Vacant(entry) => {
                entry.insert(target.clone());
            }
            std::collections::btree_map::Entry::Occupied(mut entry) => {
                if prefers_navigation_target(entry.get(), target) {
                    entry.insert(target.clone());
                }
            }
        }
    }

    fn resolve_visible_source_id(&self, source_id: &str) -> Option<&NavigationTarget> {
        self.visible_source_ids.get(source_id)
    }

    fn resolve_path(&self, path: &str, allow_simple_names: bool) -> Option<&NavigationTarget> {
        let path = self.normalize_path(path);
        self.paths.get(path.as_ref()).or_else(|| {
            if path.contains("::") || !allow_simple_names {
                None
            } else {
                self.simple_names
                    .get(path.as_ref())
                    .and_then(Option::as_ref)
            }
        })
    }

    fn resolve_reference_target(&self, reference: &ApiPathReference) -> Option<&NavigationTarget> {
        if let Some(source_id) = reference.target_source_id.as_deref() {
            if let Some(target) = self.resolve_visible_source_id(source_id) {
                return Some(target);
            }
        }

        reference
            .canonical_path
            .as_deref()
            .and_then(|reference_path| self.resolve_path(reference_path, false))
    }

    fn normalize_path<'a>(&'a self, path: &'a str) -> std::borrow::Cow<'a, str> {
        match path {
            "crate" => std::borrow::Cow::Borrowed(self.root_path.as_str()),
            _ => path
                .strip_prefix("crate::")
                .map(|suffix| std::borrow::Cow::Owned(format!("{}::{suffix}", self.root_path)))
                .unwrap_or_else(|| std::borrow::Cow::Borrowed(path)),
        }
    }
}

fn collect_navigation_targets(lookup: &mut NavigationLookup, module: &ApiModule) {
    let module_target = NavigationTarget {
        line_id: module_line_id(&module.path),
        display_name: module.local_name().to_string(),
        render_class: "namespace",
        kind: None,
        path_depth: path_depth(&module.path),
    };
    lookup.insert_path(module.path.clone(), &module_target);
    lookup.insert_simple_name(module.local_name(), &module_target);

    let mut item_line_id_counts = BTreeMap::new();
    for item in apiview_sorted_items(module) {
        let line_id =
            allocate_unique_line_id(item_line_id_base(module, item), &mut item_line_id_counts);
        if !item_in_tree(item.kind) {
            continue;
        }

        let target = NavigationTarget {
            line_id,
            display_name: item.name.clone(),
            render_class: item_render_class(item.kind),
            kind: Some(item.kind),
            path_depth: path_depth(&format!("{}::{}", module.path, item.name)),
        };
        lookup.insert_path(format!("{}::{}", module.path, item.name), &target);
        if let Some(source_id) = item.source_id.as_deref() {
            lookup.insert_visible_source_id(source_id, &target);
        }
        lookup.insert_simple_name(&item.name, &target);
        for path in &item.navigation_paths {
            lookup.insert_path(path.path.clone(), &target);
            if let Some(source_id) = path.source_id.as_deref() {
                lookup.insert_visible_source_id(source_id, &target);
            }
        }
    }

    for entry in apiview_tree_entries(module) {
        if let ModuleTreeEntry::Module(child) = entry {
            collect_navigation_targets(lookup, child);
        }
    }
}

fn annotate_navigation_token(
    tokens: &mut [ReviewToken],
    item_name: &str,
    name_token_kind: u8,
    display_name: &str,
    navigate_to_id: &str,
    render_class: &'static str,
    match_preference: NavigationMatch,
) -> bool {
    let token_index = match_preference
        .find(tokens, item_name, Some(name_token_kind))
        .or_else(|| match_preference.find(tokens, item_name, None));

    if let Some(token) = token_index.and_then(|index| tokens.get_mut(index)) {
        annotate_token_with_target(
            token,
            &NavigationTarget {
                line_id: navigate_to_id.to_string(),
                display_name: display_name.to_string(),
                render_class,
                kind: None,
                path_depth: 0,
            },
            NavigationAnnotation::TreeNode,
        );
        true
    } else {
        false
    }
}

fn navigation_target_for_item<'a>(
    module: &ApiModule,
    item: &ApiItem,
    line_id: &'a str,
    navigation_lookup: &'a NavigationLookup,
) -> Option<&'a NavigationTarget> {
    if item.kind != ApiItemKind::Use {
        return None;
    }

    item.source_id
        .as_deref()
        .and_then(|source_id| navigation_lookup.resolve_visible_source_id(source_id))
        .filter(|target| target.line_id != line_id)
        .or_else(|| {
            item.navigation_paths
                .iter()
                .filter_map(|path| {
                    path.source_id.as_deref().and_then(|source_id| {
                        navigation_lookup.resolve_visible_source_id(source_id)
                    })
                })
                .find(|target| target.line_id != line_id)
        })
        .or_else(|| {
            item.navigation_paths
                .iter()
                .find_map(|path| navigation_lookup.resolve_path(&path.path, false))
                .filter(|target| target.line_id != line_id)
        })
        .or_else(|| {
            let item_path = format!("{}::{}", module.path, item.name);
            navigation_lookup
                .resolve_path(&item_path, false)
                .filter(|target| target.line_id != line_id)
        })
}

fn annotate_path_references(
    tokens: &mut [ReviewToken],
    references: &[ApiPathReference],
    navigation_lookup: &NavigationLookup,
) {
    let mut search_from = 0;
    for reference in references {
        let Some(target) = navigation_lookup.resolve_reference_target(reference) else {
            continue;
        };
        let path_tokens = tokenize_line(&reference.path, "", token_kind::TYPE_NAME);
        if path_tokens.is_empty() {
            continue;
        }
        let Some(start) = find_token_sequence(tokens, &path_tokens, search_from) else {
            continue;
        };
        let leaf_index = start + path_tokens.len().saturating_sub(1);
        if let Some(token) = tokens.get_mut(leaf_index) {
            if token.navigate_to_id.is_none() {
                annotate_token_with_target(token, target, NavigationAnnotation::CodeOnly);
            }
        }
        search_from = leaf_index.saturating_add(1);
    }
}

fn find_token_sequence(
    tokens: &[ReviewToken],
    pattern: &[ReviewToken],
    search_from: usize,
) -> Option<usize> {
    if pattern.is_empty() || pattern.len() > tokens.len() || search_from >= tokens.len() {
        return None;
    }

    (search_from..=tokens.len().saturating_sub(pattern.len())).find(|start| {
        pattern.iter().enumerate().all(|(offset, expected)| {
            let actual = &tokens[start + offset];
            actual.kind == expected.kind && actual.value == expected.value
        })
    })
}

fn annotate_reference_tokens(
    tokens: &mut [ReviewToken],
    navigation_lookup: &NavigationLookup,
    allow_simple_names: bool,
) {
    let mut index = 0;
    while index < tokens.len() {
        let Some(token) = tokens.get(index) else {
            break;
        };
        if !can_start_reference(token)
            || token.value.starts_with('\'')
            || index > 0 && tokens[index - 1].value == "::"
        {
            index += 1;
            continue;
        }

        let start = index;
        let mut end = index;
        let mut segments = vec![token.value.clone()];
        while end + 2 < tokens.len()
            && tokens[end + 1].value == "::"
            && tokens[end + 2].kind == token_kind::TYPE_NAME
        {
            end += 2;
            segments.push(tokens[end].value.clone());
        }

        if is_reference_candidate(tokens, start, end) {
            let path = segments.join("::");
            if let Some(target) = navigation_lookup.resolve_path(&path, allow_simple_names) {
                let token = &mut tokens[end];
                if token.navigate_to_id.is_none() {
                    annotate_token_with_target(token, target, NavigationAnnotation::CodeOnly);
                }
            }
        }

        index = end + 1;
    }
}

fn annotate_trait_impl_owner_reference(
    tokens: &mut [ReviewToken],
    module: &ApiModule,
    item: &ApiItem,
    navigation_lookup: &NavigationLookup,
) {
    if item.kind != ApiItemKind::TraitImpl {
        return;
    }

    let Some(owner_name) = item.owner_name.as_deref() else {
        return;
    };
    let owner_path = format!("{}::{owner_name}", module.path);
    let Some(target) = item
        .owner_source_id
        .as_deref()
        .and_then(|owner_source_id| {
            item.declaration_path_references
                .iter()
                .rev()
                .find(|reference| reference.target_source_id.as_deref() == Some(owner_source_id))
                .and_then(|reference| navigation_lookup.resolve_reference_target(reference))
                .or_else(|| navigation_lookup.resolve_visible_source_id(owner_source_id))
        })
        .or_else(|| navigation_lookup.resolve_path(&owner_path, false))
    else {
        return;
    };
    let Some(for_index) = tokens.iter().position(|token| token.value == "for") else {
        return;
    };

    let mut index = for_index + 1;
    while index < tokens.len() && !can_start_reference(&tokens[index]) {
        index += 1;
    }
    if index >= tokens.len() || tokens[index].value.starts_with('\'') {
        return;
    }

    let mut leaf_index = index;
    while leaf_index + 2 < tokens.len()
        && tokens[leaf_index + 1].value == "::"
        && tokens[leaf_index + 2].kind == token_kind::TYPE_NAME
    {
        leaf_index += 2;
    }

    let token = &mut tokens[leaf_index];
    annotate_token_with_target(token, target, NavigationAnnotation::CodeOnly);
}

fn can_start_reference(token: &ReviewToken) -> bool {
    token.kind == token_kind::TYPE_NAME
        || token.kind == token_kind::KEYWORD && token.value == "crate"
}

fn is_reference_candidate(tokens: &[ReviewToken], start: usize, end: usize) -> bool {
    if start == end {
        if tokens.get(end + 1).is_some_and(|token| token.value == ":") {
            return false;
        }
        if tokens.get(end + 1).is_some_and(|token| token.value == "!") {
            return false;
        }
    }

    let previous = start.checked_sub(1).and_then(|index| tokens.get(index));
    !previous.is_some_and(|token| {
        matches!(
            token.value.as_str(),
            "fn" | "struct"
                | "enum"
                | "trait"
                | "type"
                | "mod"
                | "impl"
                | "for"
                | "use"
                | "as"
                | "const"
                | "static"
                | "derive"
                | "macro"
        )
    })
}

#[derive(Clone, Copy)]
enum NavigationAnnotation {
    TreeNode,
    CodeOnly,
}

fn annotate_token_with_target(
    token: &mut ReviewToken,
    target: &NavigationTarget,
    annotation: NavigationAnnotation,
) {
    token.navigate_to_id = Some(target.line_id.clone());
    match annotation {
        NavigationAnnotation::TreeNode => {
            token.navigation_display_name = Some(target.display_name.clone());
            token.render_classes = Some(vec![target.render_class.to_string()]);
        }
        NavigationAnnotation::CodeOnly => {
            token.navigation_display_name = None;
            token.render_classes = None;
        }
    }
}

#[derive(Clone, Copy)]
enum NavigationMatch {
    First,
    Last,
}

impl NavigationMatch {
    fn find(self, tokens: &[ReviewToken], item_name: &str, kind: Option<u8>) -> Option<usize> {
        let matches = |token: &ReviewToken| {
            token.value == item_name && kind.is_none_or(|kind| token.kind == kind)
        };
        match self {
            Self::First => tokens.iter().position(matches),
            Self::Last => tokens.iter().rposition(matches),
        }
    }
}

fn item_navigation_match(kind: ApiItemKind) -> NavigationMatch {
    match kind {
        ApiItemKind::Use => NavigationMatch::Last,
        _ => NavigationMatch::First,
    }
}

#[cfg(test)]
mod tests;

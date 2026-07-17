// Copyright (c) Microsoft Corporation. All rights reserved.
// Licensed under the MIT License.

use crate::model::{ApiItem, ApiItemKind, ApiModel, ApiModule};
use serde::Serialize;
use std::collections::BTreeSet;

#[derive(Debug, Clone, Copy)]
pub(crate) struct RenderOptions {
    pub(crate) include_docs: bool,
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
    #[serde(rename = "RenderClasses", skip_serializing_if = "Option::is_none")]
    render_classes: Option<Vec<String>>,
}

pub(crate) fn render(model: &ApiModel, options: &RenderOptions) -> Result<String, String> {
    let document = CodeFile {
        package_name: &model.package_name,
        package_version: &model.package_version,
        parser_version: &model.parser_version,
        language: "Rust",
        review_lines: render_module_contents(&model.root_module, options),
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

fn render_module_contents(module: &ApiModule, options: &RenderOptions) -> Vec<ReviewLine> {
    let mut lines = render_items(module, options);
    let mut child_modules = module.modules.clone();
    child_modules.sort_by(|left, right| left.path.cmp(&right.path));
    for child in &child_modules {
        lines.extend(render_module(child, options));
    }
    lines
}

fn render_module(module: &ApiModule, options: &RenderOptions) -> Vec<ReviewLine> {
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

    lines.push(ReviewLine {
        line_id: Some(line_id.clone()),
        tokens: tokenize_line(
            &format!("pub mod {} {{", module.local_name()),
            module.local_name(),
            token_kind::TYPE_NAME,
        ),
        children: render_module_contents(module, options),
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

fn render_items(module: &ApiModule, options: &RenderOptions) -> Vec<ReviewLine> {
    let items = module.sorted_items();
    let mut lines = Vec::new();
    for (index, item) in items.into_iter().enumerate() {
        lines.extend(render_item(module, item, index, options));
    }
    lines
}

fn render_item(
    module: &ApiModule,
    item: &ApiItem,
    index: usize,
    options: &RenderOptions,
) -> Vec<ReviewLine> {
    let line_id = format!("{}.{}_{index}", module_line_id(&module.path), item.name);
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

    let mut members = item.members.clone();
    members.sort_by(|left, right| left.name.cmp(&right.name));
    let declaration_owns_members = item.declaration.trim_end().ends_with('{');
    let member_related_line = if declaration_owns_members {
        line_id.clone()
    } else {
        format!("{line_id}.impl")
    };
    let member_lines = render_member_lines(&members, &member_related_line, options);

    for (declaration_index, declaration_line) in item.declaration.lines().enumerate() {
        if declaration_line.trim().is_empty() {
            continue;
        }

        lines.push(ReviewLine {
            line_id: if declaration_index == 0 {
                Some(line_id.clone())
            } else {
                None
            },
            tokens: tokenize_line(declaration_line, &item.name, name_token_kind),
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
    members: &[crate::model::ApiMember],
    related_to_line: &str,
    options: &RenderOptions,
) -> Vec<ReviewLine> {
    let mut lines = Vec::new();
    for (member_index, member) in members.iter().enumerate() {
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

        lines.push(ReviewLine {
            line_id: Some(format!("{related_to_line}.{}_{member_index}", member.name)),
            tokens: tokenize_line(&member.declaration, &member.name, token_kind::MEMBER_NAME),
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
        render_classes: None,
    }
}

#[cfg(test)]
mod tests;

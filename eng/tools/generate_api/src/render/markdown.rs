// Copyright (c) Microsoft Corporation. All rights reserved.
// Licensed under the MIT License.

use crate::{
    model::{ApiItem, ApiMember, ApiMemberKind, ApiModel, ApiModule, SourceLocation},
    source_map::GeneratedMapping,
};

/// A single rendered Markdown line and whether it is a documentation comment.
#[derive(Debug, Clone, Eq, PartialEq)]
pub(crate) struct RenderedLine {
    pub(crate) text: String,
    pub(crate) is_doc_comment: bool,
    pub(crate) declaration_location: Option<SourceLocation>,
}

/// Renders the API surface without documentation comments.
pub(crate) fn render_from_lines(lines: &[RenderedLine]) -> String {
    let mut output = String::new();
    for line in lines.iter().filter(|line| !line.is_doc_comment) {
        output.push_str(&line.text);
        output.push('\n');
    }
    output
}

pub(crate) fn source_mappings_from_lines(lines: &[RenderedLine]) -> Vec<GeneratedMapping> {
    lines
        .iter()
        .filter(|line| !line.is_doc_comment)
        .enumerate()
        .filter_map(|(generated_line, line)| {
            line.declaration_location
                .clone()
                .map(|original| GeneratedMapping {
                    generated_line,
                    generated_column: line.text.len() - line.text.trim_start_matches(' ').len(),
                    original,
                })
        })
        .collect()
}

/// Renders every Markdown line including documentation comments.
///
/// Documentation comments are marked so callers can render the API surface
/// without them and still generate a patch that adds them back.
pub(crate) fn render_lines(model: &ApiModel) -> Vec<RenderedLine> {
    let mut output = Vec::new();
    push_code(&mut output, 0, &format!("# {}", model.package_name));
    push_code(&mut output, 0, "");
    render_package_metadata(&mut output, model);
    push_code(&mut output, 0, "## Features");
    push_code(&mut output, 0, "");
    for feature in model.package_metadata.feature_names() {
        push_code(&mut output, 0, &format!("- `{feature}`"));
        if feature == "default" {
            for child in model.package_metadata.default_feature_children() {
                push_code(&mut output, 0, &format!("  - `{child}`"));
            }
        }
    }
    if !model.package_metadata.features.is_empty() {
        push_code(&mut output, 0, "");
    }
    push_code(&mut output, 0, "```rust");
    render_module(&mut output, &model.root_module, true, 0);
    push_code(&mut output, 0, "```");
    output
}

fn render_package_metadata(output: &mut Vec<RenderedLine>, model: &ApiModel) {
    let metadata = &model.package_metadata;
    if let Some(description) = metadata.description_lines() {
        if description.len() == 1 {
            push_code(output, 0, &format!("- **Description**: {}", description[0]));
        } else {
            push_code(output, 0, "- **Description:**");
            push_code(output, 0, "");
            for line in description {
                if line.is_empty() {
                    push_code(output, 0, "");
                } else {
                    push_code(output, 0, &format!("  {line}"));
                }
            }
        }
    }
    if let Some(edition) = &metadata.edition {
        push_code(output, 0, &format!("- **Edition**: {edition}"));
    }
    if let Some(rust_version) = &metadata.rust_version {
        push_code(output, 0, &format!("- **Rust version**: {rust_version}"));
    }
    if metadata.description.is_some()
        || metadata.edition.is_some()
        || metadata.rust_version.is_some()
    {
        push_code(output, 0, "");
    }
}

fn render_module(output: &mut Vec<RenderedLine>, module: &ApiModule, is_root: bool, indent: usize) {
    let items = module.sorted_items();

    let mut modules = module.modules.clone();
    modules.sort_by(|left, right| left.path.cmp(&right.path));

    let body_indent = if is_root { indent } else { indent + 1 };
    push_module_doc_comments(output, indent, &module.doc_comments, is_root);
    for attribute in &module.attributes {
        push_code(output, indent, &attribute.text);
    }
    if !is_root {
        push_declaration(
            output,
            indent,
            &format!("pub mod {} {{", module.local_name()),
            module.declaration_location.as_ref(),
        );
    }

    for item in items {
        render_item(output, item, body_indent);
    }

    for child in &modules {
        render_module(output, child, false, body_indent);
    }

    if !is_root {
        push_code(output, indent, "}");
    }
}

fn render_item(output: &mut Vec<RenderedLine>, item: &ApiItem, indent: usize) {
    push_doc_comments(output, indent, &item.doc_comments);
    for attribute in &item.attributes {
        push_code(output, indent, &attribute.text);
    }

    push_declaration_multiline(
        output,
        indent,
        &item.declaration,
        item.declaration_location.as_ref(),
    );

    let members = sorted_members(&item.members);

    if item.declaration.trim_end().ends_with('{') {
        for member in members {
            render_member(output, member, indent + 1);
        }
        push_code(output, indent, "}");
    } else if !members.is_empty() {
        push_code(output, indent, &format!("impl {} {{", item.name));
        for member in members {
            render_member(output, member, indent + 1);
        }
        push_code(output, indent, "}");
    }
}

fn render_member(output: &mut Vec<RenderedLine>, function: &ApiMember, indent: usize) {
    push_doc_comments(output, indent, &function.doc_comments);
    for attribute in &function.attributes {
        push_code(output, indent, &attribute.text);
    }
    push_declaration_multiline(
        output,
        indent,
        &function.declaration,
        function.declaration_location.as_ref(),
    );
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

fn push_declaration_multiline(
    output: &mut Vec<RenderedLine>,
    indent: usize,
    text: &str,
    location: Option<&SourceLocation>,
) {
    for (index, line) in text.lines().enumerate() {
        push_line(
            output,
            indent,
            line,
            false,
            (index == 0).then_some(location).flatten(),
        );
    }
}

fn push_doc_comments(output: &mut Vec<RenderedLine>, indent: usize, doc_comments: &[String]) {
    for comment in doc_comments {
        push_line(output, indent, comment, true, None);
    }
}

fn push_module_doc_comments(
    output: &mut Vec<RenderedLine>,
    indent: usize,
    doc_comments: &[String],
    is_root: bool,
) {
    for comment in doc_comments {
        let comment = if is_root {
            comment
                .strip_prefix("///")
                .map(|suffix| format!("//!{suffix}"))
                .unwrap_or_else(|| comment.clone())
        } else {
            comment.clone()
        };
        push_line(output, indent, &comment, true, None);
    }
}

fn push_code(output: &mut Vec<RenderedLine>, indent: usize, text: &str) {
    push_line(output, indent, text, false, None);
}

fn push_declaration(
    output: &mut Vec<RenderedLine>,
    indent: usize,
    text: &str,
    location: Option<&SourceLocation>,
) {
    push_line(output, indent, text, false, location);
}

fn push_line(
    output: &mut Vec<RenderedLine>,
    indent: usize,
    text: &str,
    is_doc_comment: bool,
    declaration_location: Option<&SourceLocation>,
) {
    let mut line = "    ".repeat(indent);
    line.push_str(text);
    output.push(RenderedLine {
        text: line,
        is_doc_comment,
        declaration_location: declaration_location.cloned(),
    });
}

#[cfg(test)]
mod tests;

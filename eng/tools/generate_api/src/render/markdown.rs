// Copyright (c) Microsoft Corporation. All rights reserved.
// Licensed under the MIT License.

use crate::model::{ApiItem, ApiMember, ApiMemberKind, ApiModel, ApiModule};

/// A single rendered Markdown line and whether it is a documentation comment.
#[derive(Debug, Clone, Eq, PartialEq)]
pub(crate) struct RenderedLine {
    pub(crate) text: String,
    pub(crate) is_doc_comment: bool,
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

/// Renders every Markdown line including documentation comments.
///
/// Documentation comments are marked so callers can render the API surface
/// without them and still generate a patch that adds them back.
pub(crate) fn render_lines(model: &ApiModel) -> Vec<RenderedLine> {
    let mut output = Vec::new();
    push_code(&mut output, 0, "```rust");
    render_module(&mut output, &model.root_module, true, 0);
    push_code(&mut output, 0, "```");
    output
}

fn render_module(output: &mut Vec<RenderedLine>, module: &ApiModule, is_root: bool, indent: usize) {
    let items = module.sorted_items();

    let mut modules = module.modules.clone();
    modules.sort_by(|left, right| left.path.cmp(&right.path));

    let body_indent = if is_root { indent } else { indent + 1 };
    push_doc_comments(output, indent, &module.doc_comments);
    for attribute in &module.attributes {
        push_code(output, indent, &attribute.text);
    }
    if !is_root {
        push_code(
            output,
            indent,
            &format!("pub mod {} {{", module.local_name()),
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

    push_multiline(output, indent, &item.declaration);

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
    push_multiline(output, indent, &function.declaration);
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

fn push_multiline(output: &mut Vec<RenderedLine>, indent: usize, text: &str) {
    for line in text.lines() {
        push_code(output, indent, line);
    }
}

fn push_doc_comments(output: &mut Vec<RenderedLine>, indent: usize, doc_comments: &[String]) {
    for comment in doc_comments {
        push_line(output, indent, comment, true);
    }
}

fn push_code(output: &mut Vec<RenderedLine>, indent: usize, text: &str) {
    push_line(output, indent, text, false);
}

fn push_line(output: &mut Vec<RenderedLine>, indent: usize, text: &str, is_doc_comment: bool) {
    let mut line = "    ".repeat(indent);
    line.push_str(text);
    output.push(RenderedLine {
        text: line,
        is_doc_comment,
    });
}

#[cfg(test)]
mod tests;

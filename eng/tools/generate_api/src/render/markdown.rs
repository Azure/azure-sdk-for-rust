// Copyright (c) Microsoft Corporation. All rights reserved.
// Licensed under the MIT License.

use crate::model::{ApiItem, ApiMember, ApiMemberKind, ApiModel, ApiModule};

pub(crate) fn render(model: &ApiModel) -> String {
    let mut output = String::from("```rust\n");
    render_module(&mut output, &model.root_module, true, 0);
    output.push_str("```\n");
    output
}

fn render_module(output: &mut String, module: &ApiModule, is_root: bool, indent: usize) {
    let items = module.sorted_items();

    let mut modules = module.modules.clone();
    modules.sort_by(|left, right| left.path.cmp(&right.path));

    let body_indent = if is_root { indent } else { indent + 1 };
    for attribute in &module.attributes {
        push_line(output, indent, &attribute.text);
    }
    if !is_root {
        push_line(
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
        push_line(output, indent, "}");
    }
}

fn render_item(output: &mut String, item: &ApiItem, indent: usize) {
    for attribute in &item.attributes {
        push_line(output, indent, &attribute.text);
    }

    push_multiline(output, indent, &item.declaration);

    let members = sorted_members(&item.members);

    if item.declaration.trim_end().ends_with('{') {
        for member in members {
            render_member(output, member, indent + 1);
        }
        push_line(output, indent, "}");
    } else if !members.is_empty() {
        push_line(output, indent, &format!("impl {} {{", item.name));
        for member in members {
            render_member(output, member, indent + 1);
        }
        push_line(output, indent, "}");
    }
}

fn render_member(output: &mut String, function: &ApiMember, indent: usize) {
    for attribute in &function.attributes {
        push_line(output, indent, &attribute.text);
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

fn push_multiline(output: &mut String, indent: usize, text: &str) {
    for line in text.lines() {
        push_line(output, indent, line);
    }
}

fn push_line(output: &mut String, indent: usize, text: &str) {
    output.push_str(&"    ".repeat(indent));
    output.push_str(text);
    output.push('\n');
}

#[cfg(test)]
mod tests;

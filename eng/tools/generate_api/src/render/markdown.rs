// Copyright (c) Microsoft Corporation. All rights reserved.
// Licensed under the MIT License.

use crate::model::{ApiItem, ApiMember, ApiModel, ApiModule};

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
    if !is_root {
        for attribute in &module.attributes {
            push_line(output, indent, &attribute.text);
        }
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

    let mut members = item.members.clone();
    members.sort_by(|left, right| left.name.cmp(&right.name));

    if item.declaration.trim_end().ends_with('{') {
        for member in &members {
            render_member(output, member, indent + 1);
        }
        push_line(output, indent, "}");
    } else if !members.is_empty() {
        push_line(output, indent, &format!("impl {} {{", item.name));
        for member in &members {
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

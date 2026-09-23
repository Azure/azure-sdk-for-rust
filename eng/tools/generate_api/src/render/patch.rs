// Copyright (c) Microsoft Corporation. All rights reserved.
// Licensed under the MIT License.

use crate::render::markdown::RenderedLine;

/// Renders a unified diff that adds documentation comments back to `file_name`.
///
/// The rendered Markdown omits documentation comments, so the patch only ever
/// contains insertions. Applying it to `file_name` yields the same API surface
/// with documentation comments included.
pub(crate) fn render(lines: &[RenderedLine], file_name: &str) -> String {
    let hunks = collect_hunks(lines);
    if hunks.is_empty() {
        return String::new();
    }

    // Map each rendered line to its line number in the file without documentation comments.
    let mut source_numbers = Vec::with_capacity(lines.len());
    let mut source_line = 0usize;
    for line in lines {
        if line.is_doc_comment {
            source_numbers.push(None);
        } else {
            source_line += 1;
            source_numbers.push(Some(source_line));
        }
    }

    let mut output = format!("--- a/{file_name}\n+++ b/{file_name}\n");
    for (start, end) in hunks {
        let source_count = source_numbers[start..=end]
            .iter()
            .filter(|number| number.is_some())
            .count();
        let source_start = source_numbers[start..=end]
            .iter()
            .find_map(|number| *number)
            .unwrap_or_else(|| {
                source_numbers[..start]
                    .iter()
                    .rev()
                    .find_map(|number| *number)
                    .unwrap_or(0)
            });

        output.push_str(&format!(
            "@@ -{source_start},{source_count} +{},{} @@\n",
            start + 1,
            end - start + 1
        ));
        for line in &lines[start..=end] {
            output.push(if line.is_doc_comment { '+' } else { ' ' });
            output.push_str(&line.text);
            output.push('\n');
        }
    }

    output
}

/// Collects one hunk per contiguous doc-comment block plus its declaration context.
fn collect_hunks(lines: &[RenderedLine]) -> Vec<(usize, usize)> {
    let mut hunks = Vec::new();
    let mut index = 0;
    while index < lines.len() {
        if !lines[index].is_doc_comment {
            index += 1;
            continue;
        }

        let start = index;
        while index + 1 < lines.len() && lines[index + 1].is_doc_comment {
            index += 1;
        }

        let mut end = index;
        let has_crate_root_anchor = end + 1 < lines.len() && lines[end + 1].is_crate_root_anchor;
        if has_crate_root_anchor {
            while end + 1 < lines.len() && lines[end + 1].is_crate_root_anchor {
                end += 1;
            }
        } else {
            while end + 1 < lines.len() && lines[end + 1].is_attribute {
                end += 1;
            }
        }
        if !has_crate_root_anchor && end + 1 < lines.len() && !lines[end + 1].is_doc_comment {
            end += 1;
        }
        hunks.push((start, end));
        index = end + 1;
    }
    hunks
}

#[cfg(test)]
mod tests;

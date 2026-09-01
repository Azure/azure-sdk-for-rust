// Copyright (c) Microsoft Corporation. All rights reserved.
// Licensed under the MIT License.

use crate::render::markdown::RenderedLine;

/// Number of unchanged lines rendered around each change in a hunk.
const CONTEXT_LINES: usize = 3;

/// Renders a unified diff that adds documentation comments back to `file_name`.
///
/// The rendered Markdown omits documentation comments, so the patch only ever
/// contains insertions. Applying it to `file_name` yields the same API surface
/// with documentation comments included.
pub(crate) fn render(lines: &[RenderedLine], file_name: &str) -> String {
    let changes = lines
        .iter()
        .enumerate()
        .filter_map(|(index, line)| line.is_doc_comment.then_some(index))
        .collect::<Vec<_>>();
    if changes.is_empty() {
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
    for (first_change, last_change) in group_changes(&changes) {
        let start = first_change.saturating_sub(CONTEXT_LINES);
        let end = (last_change + CONTEXT_LINES).min(lines.len() - 1);

        let source_count = source_numbers[start..=end]
            .iter()
            .filter(|number| number.is_some())
            .count();
        let source_start = source_numbers[start..=end]
            .iter()
            .find_map(|number| *number)
            // A hunk without context starts after the preceding source line.
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

/// Groups changed line indexes into hunks, merging groups with overlapping context.
fn group_changes(changes: &[usize]) -> Vec<(usize, usize)> {
    let mut groups: Vec<(usize, usize)> = Vec::new();
    for &change in changes {
        match groups.last_mut() {
            Some((_, last)) if change <= *last + (CONTEXT_LINES * 2) + 1 => *last = change,
            _ => groups.push((change, change)),
        }
    }
    groups
}

#[cfg(test)]
mod tests;

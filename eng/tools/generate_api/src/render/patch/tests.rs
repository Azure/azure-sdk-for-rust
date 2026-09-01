// Copyright (c) Microsoft Corporation. All rights reserved.
// Licensed under the MIT License.

use super::*;

fn code(text: &str) -> RenderedLine {
    RenderedLine {
        text: text.to_string(),
        is_doc_comment: false,
    }
}

fn doc(text: &str) -> RenderedLine {
    RenderedLine {
        text: text.to_string(),
        is_doc_comment: true,
    }
}

#[test]
fn renders_empty_patch_without_doc_comments() {
    let lines = vec![code("```rust"), code("pub fn foo();"), code("```")];

    assert!(render(&lines, "API.md").is_empty());
}

#[test]
fn renders_insertions_with_context() {
    let lines = vec![
        code("```rust"),
        doc("/// Does foo."),
        doc("///"),
        doc("/// What else did you expect?"),
        code("pub fn foo();"),
        code("```"),
    ];

    let patch = render(&lines, "API.md");

    assert_eq!(
        patch,
        "--- a/API.md\n\
         +++ b/API.md\n\
         @@ -1,3 +1,6 @@\n\
         \x20```rust\n\
         +/// Does foo.\n\
         +///\n\
         +/// What else did you expect?\n\
         \x20pub fn foo();\n\
         \x20```\n"
    );
}

#[test]
fn merges_nearby_changes_into_one_hunk() {
    let mut lines = vec![code("```rust"), doc("/// First.")];
    lines.extend((0..4).map(|index| code(&format!("pub fn item{index}();"))));
    lines.push(doc("/// Second."));
    lines.push(code("pub fn last();"));
    lines.push(code("```"));

    let patch = render(&lines, "API.md");

    assert_eq!(patch.matches("@@ -").count(), 1);
    assert!(patch.contains("@@ -1,7 +1,9 @@"));
}

#[test]
fn splits_distant_changes_into_separate_hunks() {
    let mut lines = vec![code("```rust"), doc("/// First.")];
    lines.extend((0..20).map(|index| code(&format!("pub fn item{index}();"))));
    lines.push(doc("/// Second."));
    lines.push(code("pub fn last();"));
    lines.push(code("```"));

    let patch = render(&lines, "API.md");

    assert_eq!(patch.matches("@@ -").count(), 2);
    assert!(patch.contains("@@ -1,4 +1,5 @@"));
}

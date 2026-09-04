// Copyright (c) Microsoft Corporation. All rights reserved.
// Licensed under the MIT License.

use super::*;

fn code(text: &str) -> RenderedLine {
    RenderedLine {
        declaration_location: None,
        text: text.to_string(),
        is_doc_comment: false,
    }
}

fn doc(text: &str) -> RenderedLine {
    RenderedLine {
        declaration_location: None,
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
fn renders_one_hunk_per_doc_block() {
    let lines = vec![
        code("```rust"),
        doc("/// Does foo."),
        doc("///"),
        doc("/// What else did you expect?"),
        code("pub fn foo();"),
        doc("/// Does bar."),
        code("pub fn bar();"),
        code("```"),
    ];

    let patch = render(&lines, "API.md");

    assert_eq!(
        patch,
        "--- a/API.md\n\
         +++ b/API.md\n\
         @@ -2,1 +2,4 @@\n\
         +/// Does foo.\n\
         +///\n\
         +/// What else did you expect?\n\
         \x20pub fn foo();\n\
         @@ -3,1 +6,2 @@\n\
         +/// Does bar.\n\
         \x20pub fn bar();\n"
    );
}

#[test]
fn keeps_following_context_to_one_line() {
    let lines = vec![
        code("```rust"),
        doc("/// Foo."),
        code("pub struct Foo {"),
        code("    pub field: bool,"),
        code("}"),
        doc("/// Builds foo."),
        code("pub fn build(&self) -> Foo;"),
        code("```"),
    ];

    let patch = render(&lines, "API.md");

    assert_eq!(patch.matches("@@ -").count(), 2);
    assert!(patch.contains("@@ -2,1 +2,2 @@\n+/// Foo.\n pub struct Foo {\n"));
    assert!(!patch.contains("pub struct Foo {\n     pub field: bool,"));
    assert!(patch.contains("@@ -5,1 +6,2 @@\n+/// Builds foo.\n pub fn build(&self) -> Foo;\n"));
}

#[test]
fn anchors_doc_comments_before_attributes() {
    let lines = vec![
        code("```rust"),
        doc("/// Foo."),
        code("#[cfg(feature = \"preview\")]"),
        code("pub struct Foo;"),
        code("```"),
    ];

    let patch = render(&lines, "API.md");

    assert_eq!(
        patch,
        "--- a/API.md\n\
         +++ b/API.md\n\
         @@ -2,1 +2,2 @@\n\
         +/// Foo.\n\
         \x20#[cfg(feature = \"preview\")]\n"
    );
}

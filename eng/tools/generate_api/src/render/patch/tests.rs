// Copyright (c) Microsoft Corporation. All rights reserved.
// Licensed under the MIT License.

use super::*;

fn code(text: &str) -> RenderedLine {
    RenderedLine {
        declaration_location: None,
        text: text.to_string(),
        is_doc_comment: false,
        is_attribute: false,
        is_crate_root_anchor: false,
    }
}

fn attribute(text: &str) -> RenderedLine {
    RenderedLine {
        declaration_location: None,
        text: text.to_string(),
        is_doc_comment: false,
        is_attribute: true,
        is_crate_root_anchor: false,
    }
}

fn crate_root_anchor(text: &str) -> RenderedLine {
    RenderedLine {
        declaration_location: None,
        text: text.to_string(),
        is_doc_comment: false,
        is_attribute: true,
        is_crate_root_anchor: true,
    }
}

fn doc(text: &str) -> RenderedLine {
    RenderedLine {
        declaration_location: None,
        text: text.to_string(),
        is_doc_comment: true,
        is_attribute: false,
        is_crate_root_anchor: false,
    }
}

#[test]
fn renders_empty_patch_without_doc_comments() {
    let lines = vec![code("```rust"), code("pub fn foo();"), code("```")];

    assert!(render(&lines, "api.md").is_empty());
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

    let patch = render(&lines, "api.md");

    assert_eq!(
        patch,
        "--- a/api.md\n\
         +++ b/api.md\n\
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

    let patch = render(&lines, "api.md");

    assert_eq!(patch.matches("@@ -").count(), 2);
    assert!(patch.contains("@@ -2,1 +2,2 @@\n+/// Foo.\n pub struct Foo {\n"));
    assert!(!patch.contains("pub struct Foo {\n     pub field: bool,"));
    assert!(patch.contains("@@ -5,1 +6,2 @@\n+/// Builds foo.\n pub fn build(&self) -> Foo;\n"));
}

#[test]
fn anchors_doc_comments_through_attributes_and_declaration() {
    let lines = vec![
        code("```rust"),
        doc("/// Foo."),
        attribute("#[cfg(feature = \"preview\")]"),
        attribute("#[derive(Clone, Debug)]"),
        code("pub struct Foo;"),
        code("```"),
    ];

    let patch = render(&lines, "api.md");

    assert_eq!(
        patch,
        "--- a/api.md\n\
         +++ b/api.md\n\
         @@ -2,3 +2,4 @@\n\
         +/// Foo.\n\
         \x20#[cfg(feature = \"preview\")]\n\
         \x20#[derive(Clone, Debug)]\n\
         \x20pub struct Foo;\n"
    );
}

#[test]
fn keeps_documented_members_in_separate_hunks_with_attributes() {
    let lines = vec![
        code("pub struct Foo {"),
        doc("    /// The first field."),
        attribute("    #[cfg(feature = \"preview\")]"),
        code("    pub first: bool,"),
        doc("    /// The second field."),
        attribute("    #[cfg(feature = \"preview\")]"),
        attribute("    #[deprecated]"),
        code("    pub second: bool,"),
        code("}"),
    ];

    let patch = render(&lines, "api.md");

    assert_eq!(patch.matches("@@ -").count(), 2);
    assert!(patch.contains(
        "@@ -2,2 +2,3 @@\n\
         +    /// The first field.\n\
         \x20    #[cfg(feature = \"preview\")]\n\
         \x20    pub first: bool,\n"
    ));
    assert!(patch.contains(
        "@@ -4,3 +5,4 @@\n\
         +    /// The second field.\n\
         \x20    #[cfg(feature = \"preview\")]\n\
         \x20    #[deprecated]\n\
         \x20    pub second: bool,\n"
    ));
}

#[test]
fn anchors_root_doc_comments_only_to_synthetic_root_attributes() {
    let lines = vec![
        code("```rust"),
        doc("//! Demo crate."),
        crate_root_anchor("#![crate_name = \"demo\"]"),
        crate_root_anchor("#![crate_type = \"lib\"]"),
        attribute("#![cfg_attr(docsrs, feature(doc_cfg))]"),
        code("pub struct Foo;"),
        code("```"),
    ];

    let patch = render(&lines, "api.md");

    assert_eq!(
        patch,
        "--- a/api.md\n\
         +++ b/api.md\n\
         @@ -2,2 +2,3 @@\n\
         +//! Demo crate.\n\
         \x20#![crate_name = \"demo\"]\n\
         \x20#![crate_type = \"lib\"]\n"
    );
}

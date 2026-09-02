// Copyright (c) Microsoft Corporation. All rights reserved.
// Licensed under the MIT License.

use super::*;
use crate::model::{
    ApiAttribute, ApiItem, ApiItemKind, ApiMember, ApiMemberKind, ApiModel, ApiModule,
};

fn render(model: &ApiModel) -> String {
    render_from_lines(&render_lines(model))
}

fn item(name: &str, kind: ApiItemKind, declaration: &str) -> ApiItem {
    ApiItem {
        name: name.to_string(),
        kind,
        source_id: None,
        navigation_paths: Vec::new(),
        owner_name: None,
        owner_kind: None,
        owner_source_id: None,
        inherent_impl_sort_key: None,
        doc_comments: Vec::new(),
        attributes: Vec::new(),
        declaration: declaration.to_string(),
        declaration_path_references: Vec::new(),
        members: Vec::new(),
    }
}

fn member(name: &str, kind: ApiMemberKind, declaration: &str) -> ApiMember {
    ApiMember {
        name: name.to_string(),
        kind,
        doc_comments: Vec::new(),
        attributes: Vec::new(),
        declaration: declaration.to_string(),
        declaration_path_references: Vec::new(),
    }
}

#[test]
fn renders_explicit_trait_impl_blocks() {
    let mut item = item(
        "MyType",
        ApiItemKind::TraitImpl,
        "impl fmt::Debug for MyType {",
    );
    item.attributes.push(ApiAttribute {
        text: "#[cfg(feature = \"std\")]".to_string(),
    });
    item.members.push(member(
        "fmt",
        ApiMemberKind::Associated,
        "fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result;",
    ));

    let model = ApiModel {
        package_name: "demo".to_string(),
        package_version: "1.0.0".to_string(),
        parser_version: "0.0.0".to_string(),
        root_module: ApiModule {
            path: "demo".to_string(),
            doc_comments: Vec::new(),
            attributes: Vec::new(),
            items: vec![item],
            modules: Vec::new(),
        },
    };

    let rendered = render(&model);

    assert!(rendered.contains("#[cfg(feature = \"std\")]"));
    assert!(rendered.contains("impl fmt::Debug for MyType {"));
    assert!(rendered.contains("    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result;"));
    assert!(rendered.contains("}\n```\n"));
}

#[test]
fn renders_inherent_members_inside_impl_blocks() {
    let mut inherent_impl = item("Foo", ApiItemKind::InherentImpl, "impl Foo {");
    inherent_impl.owner_name = Some("Foo".to_string());
    inherent_impl.owner_kind = Some(ApiItemKind::Struct);
    inherent_impl.members.push(member(
        "method",
        ApiMemberKind::Associated,
        "pub fn method(&self);",
    ));

    let model = ApiModel {
        package_name: "demo".to_string(),
        package_version: "1.0.0".to_string(),
        parser_version: "0.0.0".to_string(),
        root_module: ApiModule {
            path: "demo".to_string(),
            doc_comments: Vec::new(),
            attributes: Vec::new(),
            items: vec![
                item("Foo", ApiItemKind::Struct, "pub struct Foo;"),
                inherent_impl,
            ],
            modules: Vec::new(),
        },
    };

    let rendered = render(&model);

    assert!(rendered.contains("pub struct Foo;"));
    assert!(rendered.contains("impl Foo {\n    pub fn method(&self);\n}"));
    assert!(!rendered.contains("pub struct Foo;\n    pub fn method(&self);"));
}

#[test]
fn renders_root_inner_attrs_and_child_module_outer_attrs() {
    let model = ApiModel {
        package_name: "demo".to_string(),
        package_version: "1.0.0".to_string(),
        parser_version: "0.0.0".to_string(),
        root_module: ApiModule {
            path: "demo".to_string(),
            doc_comments: Vec::new(),
            attributes: vec![ApiAttribute {
                text: "#![warn(missing_docs)]".to_string(),
            }],
            items: Vec::new(),
            modules: vec![ApiModule {
                path: "demo::inner".to_string(),
                doc_comments: Vec::new(),
                attributes: vec![ApiAttribute {
                    text: "#[deny(unsafe_code)]".to_string(),
                }],
                items: vec![item("Nested", ApiItemKind::Struct, "pub struct Nested;")],
                modules: Vec::new(),
            }],
        },
    };

    let rendered = render(&model);

    assert!(rendered.contains("#![warn(missing_docs)]"));
    assert!(rendered.contains("#[deny(unsafe_code)]\npub mod inner {"));
    assert!(!rendered.contains("#![deny(unsafe_code)]\npub mod inner {"));
}

#[test]
fn marks_doc_comments_and_omits_them_from_markdown() {
    let mut documented = item("Foo", ApiItemKind::Struct, "pub struct Foo;");
    documented.doc_comments = vec!["/// Does foo.".to_string()];

    let model = ApiModel {
        package_name: "demo".to_string(),
        package_version: "1.0.0".to_string(),
        parser_version: "0.0.0".to_string(),
        root_module: ApiModule {
            path: "demo".to_string(),
            doc_comments: vec![
                "/// Demo crate.".to_string(),
                "///".to_string(),
                "/// More details.".to_string(),
            ],
            attributes: Vec::new(),
            items: vec![documented],
            modules: Vec::new(),
        },
    };

    let lines = render_lines(&model);

    assert_eq!(
        lines
            .iter()
            .filter(|line| line.is_doc_comment)
            .map(|line| line.text.as_str())
            .collect::<Vec<_>>(),
        vec![
            "//! Demo crate.",
            "//!",
            "//! More details.",
            "/// Does foo."
        ]
    );
    assert_eq!(render_from_lines(&lines), "```rust\npub struct Foo;\n```\n");
}

// Copyright (c) Microsoft Corporation. All rights reserved.
// Licensed under the MIT License.

use super::*;
use crate::model::{
    ApiAttribute, ApiItem, ApiItemKind, ApiMember, ApiMemberKind, ApiModel, ApiModule,
    PackageMetadata, SourceLocation,
};
use std::collections::BTreeMap;

fn render(model: &ApiModel) -> String {
    render_from_lines(&render_lines(model))
}

fn item(name: &str, kind: ApiItemKind, declaration: &str) -> ApiItem {
    ApiItem {
        declaration_location: None,
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
        declaration_location: None,
        name: name.to_string(),
        kind,
        doc_comments: Vec::new(),
        attributes: Vec::new(),
        declaration: declaration.to_string(),
        declaration_path_references: Vec::new(),
    }
}

fn location(path: &str, line: usize, column: usize) -> SourceLocation {
    SourceLocation {
        path: path.to_string(),
        line,
        column,
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
        package_metadata: Default::default(),
        root_module: ApiModule {
            declaration_location: None,
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
        package_metadata: Default::default(),
        root_module: ApiModule {
            declaration_location: None,
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
fn maps_only_declaration_lines_after_removing_docs() {
    let mut foo = item("Foo", ApiItemKind::Struct, "pub struct Foo {");
    foo.declaration_location = Some(location("src/lib.rs", 10, 0));
    let mut field = member("value", ApiMemberKind::Field, "pub value: u32,");
    field.doc_comments = vec!["/// The value.".to_string()];
    field.declaration_location = Some(location("src/lib.rs", 12, 4));
    foo.members.push(field);

    let model = ApiModel {
        package_name: "demo".to_string(),
        package_version: "1.0.0".to_string(),
        parser_version: "0.0.0".to_string(),
        package_metadata: Default::default(),
        root_module: ApiModule {
            declaration_location: None,
            path: "demo".to_string(),
            doc_comments: Vec::new(),
            attributes: Vec::new(),
            items: vec![foo],
            modules: Vec::new(),
        },
    };

    let mappings = source_mappings_from_lines(&render_lines(&model));

    assert_eq!(
        mappings,
        vec![
            GeneratedMapping {
                generated_line: 7,
                generated_column: 0,
                original: location("src/lib.rs", 10, 0),
            },
            GeneratedMapping {
                generated_line: 8,
                generated_column: 4,
                original: location("src/lib.rs", 12, 4),
            },
        ]
    );
}

#[test]
fn renders_root_inner_attrs_and_child_module_outer_attrs() {
    let model = ApiModel {
        package_name: "demo".to_string(),
        package_version: "1.0.0".to_string(),
        parser_version: "0.0.0".to_string(),
        package_metadata: Default::default(),
        root_module: ApiModule {
            declaration_location: None,
            path: "demo".to_string(),
            doc_comments: Vec::new(),
            attributes: vec![ApiAttribute {
                text: "#![warn(missing_docs)]".to_string(),
            }],
            items: Vec::new(),
            modules: vec![ApiModule {
                declaration_location: None,
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
        package_metadata: Default::default(),
        root_module: ApiModule {
            declaration_location: None,
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
    assert_eq!(
        render_from_lines(&lines),
        "# demo\n\n## Features\n\n- `default`\n\n```rust\npub struct Foo;\n```\n"
    );
}

#[test]
fn renders_package_metadata_and_features_before_api() {
    let model = ApiModel {
        package_name: "azure_security_keyvault_secrets".to_string(),
        package_version: "1.0.0".to_string(),
        parser_version: "0.0.0".to_string(),
        package_metadata: PackageMetadata {
            description: Some("Manage secrets.\n".to_string()),
            edition: Some("2021".to_string()),
            rust_version: Some("1.88".to_string()),
            features: BTreeMap::from([
                ("alpha".to_string(), vec!["dep:alpha".to_string()]),
                (
                    "default".to_string(),
                    vec!["hmac".to_string(), "azure_core/default".to_string()],
                ),
                ("test".to_string(), vec!["default".to_string()]),
            ]),
        },
        root_module: ApiModule {
            declaration_location: None,
            path: "azure_security_keyvault_secrets".to_string(),
            doc_comments: Vec::new(),
            attributes: Vec::new(),
            items: vec![item(
                "SecretClient",
                ApiItemKind::Struct,
                "pub struct SecretClient;",
            )],
            modules: Vec::new(),
        },
    };

    assert_eq!(
        render(&model),
        "# azure_security_keyvault_secrets\n\
         \n\
         - **Description**: Manage secrets.\n\
         - **Edition**: 2021\n\
         - **Rust version**: 1.88\n\
         \n\
         ## Features\n\
         \n\
         - `default`\n\
         \x20\x20- `azure_core/default`\n\
         \x20\x20- `hmac`\n\
         - `alpha`\n\
         - `test`\n\
         \n\
         ```rust\n\
         pub struct SecretClient;\n\
         ```\n"
    );
}

#[test]
fn preserves_multiline_description_paragraphs() {
    let model = ApiModel {
        package_name: "demo".to_string(),
        package_version: "1.0.0".to_string(),
        parser_version: "0.0.0".to_string(),
        package_metadata: PackageMetadata {
            description: Some("Line one\n\nLine two\nAlso line two\n\nLine three\n".to_string()),
            ..Default::default()
        },
        root_module: ApiModule {
            path: "demo".to_string(),
            declaration_location: None,
            doc_comments: Vec::new(),
            attributes: Vec::new(),
            items: Vec::new(),
            modules: Vec::new(),
        },
    };

    assert_eq!(
        render(&model),
        "# demo\n\
         \n\
         - **Description:**\n\
         \n\
         \x20\x20Line one\n\
         \n\
         \x20\x20Line two\n\
         \x20\x20Also line two\n\
         \n\
         \x20\x20Line three\n\
         \n\
         ## Features\n\
         \n\
         - `default`\n\
         \n\
         ```rust\n\
         ```\n"
    );
}

#[test]
fn omits_missing_package_metadata() {
    let model = ApiModel {
        package_name: "demo".to_string(),
        package_version: "1.0.0".to_string(),
        parser_version: "0.0.0".to_string(),
        package_metadata: Default::default(),
        root_module: ApiModule {
            declaration_location: None,
            path: "demo".to_string(),
            doc_comments: Vec::new(),
            attributes: Vec::new(),
            items: Vec::new(),
            modules: Vec::new(),
        },
    };

    assert_eq!(
        render(&model),
        "# demo\n\n## Features\n\n- `default`\n\n```rust\n```\n"
    );
}

#[test]
fn comments_patch_accounts_for_package_metadata_lines() {
    let mut documented = item("Foo", ApiItemKind::Struct, "pub struct Foo;");
    documented.doc_comments = vec!["/// Does foo.".to_string()];
    let model = ApiModel {
        package_name: "demo".to_string(),
        package_version: "1.0.0".to_string(),
        parser_version: "0.0.0".to_string(),
        package_metadata: PackageMetadata {
            description: Some("Multi-line\ncomment\n".to_string()),
            edition: Some("2021".to_string()),
            rust_version: Some("1.88".to_string()),
            features: BTreeMap::from([("default".to_string(), vec!["dep:foo".to_string()])]),
        },
        root_module: ApiModule {
            declaration_location: None,
            path: "demo".to_string(),
            doc_comments: vec!["//! Demo docs.".to_string()],
            attributes: Vec::new(),
            items: vec![documented],
            modules: Vec::new(),
        },
    };

    let lines = render_lines(&model);
    let api_without_docs = render_from_lines(&lines);
    let api_with_docs = lines
        .iter()
        .map(|line| format!("{}\n", line.text))
        .collect::<String>();
    let patch = crate::render::patch::render(&lines, "API.md");
    let parsed_patch = diffy::Patch::from_str(&patch).expect("patch should parse");

    assert!(api_without_docs.contains(concat!(
        "- **Description:**\n",
        "\n",
        "  Multi-line\n",
        "  comment\n",
    )));
    assert!(patch.contains(concat!(
        "@@ -16,1 +16,3 @@\n",
        "+//! Demo docs.\n",
        "+/// Does foo.\n",
        " pub struct Foo;\n",
    )));
    assert_eq!(
        diffy::apply(&api_without_docs, &parsed_patch).expect("patch should apply"),
        api_with_docs
    );
}

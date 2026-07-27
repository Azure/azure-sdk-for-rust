// Copyright (c) Microsoft Corporation. All rights reserved.
// Licensed under the MIT License.

use super::*;
use crate::model::{ApiAttribute, ApiItemKind};

#[test]
fn renders_explicit_trait_impl_blocks() {
    let model = ApiModel {
        package_name: "demo".to_string(),
        package_version: "1.0.0".to_string(),
        parser_version: "0.0.0".to_string(),
        root_module: ApiModule {
            path: "demo".to_string(),
            doc_comments: Vec::new(),
            attributes: Vec::new(),
            items: vec![ApiItem {
                name: "MyType".to_string(),
                kind: ApiItemKind::TraitImpl,
                source_id: None,
                owner_kind: None,
                inherent_impl_sort_key: None,
                doc_comments: Vec::new(),
                attributes: vec![ApiAttribute {
                    text: "#[cfg(feature = \"std\")]".to_string(),
                }],
                declaration: "impl fmt::Debug for MyType {".to_string(),
                members: vec![ApiMember {
                    name: "fmt".to_string(),
                    doc_comments: Vec::new(),
                    attributes: Vec::new(),
                    declaration: "fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result;"
                        .to_string(),
                }],
            }],
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
    let model = ApiModel {
        package_name: "demo".to_string(),
        package_version: "1.0.0".to_string(),
        parser_version: "0.0.0".to_string(),
        root_module: ApiModule {
            path: "demo".to_string(),
            doc_comments: Vec::new(),
            attributes: Vec::new(),
            items: vec![
                ApiItem {
                    name: "Foo".to_string(),
                    kind: ApiItemKind::Struct,
                    source_id: None,
                    owner_kind: None,
                    inherent_impl_sort_key: None,
                    doc_comments: Vec::new(),
                    attributes: Vec::new(),
                    declaration: "pub struct Foo;".to_string(),
                    members: Vec::new(),
                },
                ApiItem {
                    name: "Foo".to_string(),
                    kind: ApiItemKind::InherentImpl,
                    source_id: None,
                    owner_kind: Some(ApiItemKind::Struct),
                    inherent_impl_sort_key: None,
                    doc_comments: Vec::new(),
                    attributes: Vec::new(),
                    declaration: "impl Foo {".to_string(),
                    members: vec![ApiMember {
                        name: "method".to_string(),
                        doc_comments: Vec::new(),
                        attributes: Vec::new(),
                        declaration: "pub fn method(&self);".to_string(),
                    }],
                },
            ],
            modules: Vec::new(),
        },
    };

    let rendered = render(&model);

    assert!(rendered.contains("pub struct Foo;"));
    assert!(rendered.contains("impl Foo {\n    pub fn method(&self);\n}"));
    assert!(!rendered.contains("pub struct Foo;\n    pub fn method(&self);"));
}

// Copyright (c) Microsoft Corporation. All rights reserved.
// Licensed under the MIT License.

use super::*;
use crate::model::{ApiAttribute, ApiMember};

#[test]
fn renders_trait_impl_tokens_with_typed_members() {
    let module = ApiModule {
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
                declaration: "fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result;".to_string(),
            }],
        }],
        modules: Vec::new(),
    };

    let lines = render_module_contents(&module, &RenderOptions::default());

    assert_eq!(lines.len(), 3);
    assert_eq!(
        lines[1]
            .tokens
            .iter()
            .map(|token| (token.kind, token.value.as_str()))
            .collect::<Vec<_>>(),
        vec![
            (token_kind::KEYWORD, "impl"),
            (token_kind::TYPE_NAME, "fmt"),
            (token_kind::PUNCTUATION, "::"),
            (token_kind::TYPE_NAME, "Debug"),
            (token_kind::KEYWORD, "for"),
            (token_kind::TYPE_NAME, "MyType"),
            (token_kind::PUNCTUATION, "{"),
        ]
    );
    assert_eq!(lines[1].children.len(), 1);
    assert_eq!(
        lines[1].children[0]
            .tokens
            .iter()
            .map(|token| (token.kind, token.value.as_str()))
            .collect::<Vec<_>>(),
        vec![
            (token_kind::KEYWORD, "fn"),
            (token_kind::MEMBER_NAME, "fmt"),
            (token_kind::PUNCTUATION, "("),
            (token_kind::PUNCTUATION, "&"),
            (token_kind::KEYWORD, "self"),
            (token_kind::PUNCTUATION, ","),
            (token_kind::TYPE_NAME, "f"),
            (token_kind::PUNCTUATION, ":"),
            (token_kind::PUNCTUATION, "&"),
            (token_kind::KEYWORD, "mut"),
            (token_kind::TYPE_NAME, "fmt"),
            (token_kind::PUNCTUATION, "::"),
            (token_kind::TYPE_NAME, "Formatter"),
            (token_kind::PUNCTUATION, ")"),
            (token_kind::PUNCTUATION, "->"),
            (token_kind::TYPE_NAME, "fmt"),
            (token_kind::PUNCTUATION, "::"),
            (token_kind::TYPE_NAME, "Result"),
            (token_kind::PUNCTUATION, ";"),
        ]
    );
}

#[test]
fn renders_inherent_members_inside_impl_blocks() {
    let module = ApiModule {
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
    };

    let lines = render_module_contents(&module, &RenderOptions::default());

    assert_eq!(lines.len(), 3);
    assert_eq!(
        lines[0]
            .tokens
            .iter()
            .map(|token| token.value.as_str())
            .collect::<Vec<_>>(),
        vec!["pub", "struct", "Foo", ";"]
    );
    assert_eq!(
        lines[1]
            .tokens
            .iter()
            .map(|token| token.value.as_str())
            .collect::<Vec<_>>(),
        vec!["impl", "Foo", "{"]
    );
    assert_eq!(lines[1].children.len(), 1);
    assert_eq!(
        lines[1].children[0]
            .tokens
            .iter()
            .map(|token| token.value.as_str())
            .collect::<Vec<_>>(),
        vec!["pub", "fn", "method", "(", "&", "self", ")", ";"]
    );
    assert_eq!(
        lines[2].related_to_line.as_deref(),
        Some("module.demo.Foo_1")
    );
}

#[test]
fn keeps_duplicate_member_names_in_separate_inherent_impl_blocks() {
    let module = ApiModule {
        path: "demo".to_string(),
        doc_comments: Vec::new(),
        attributes: Vec::new(),
        items: vec![
            ApiItem {
                name: "Builder".to_string(),
                kind: ApiItemKind::Struct,
                source_id: None,
                owner_kind: None,
                inherent_impl_sort_key: None,
                doc_comments: Vec::new(),
                attributes: Vec::new(),
                declaration: "pub struct Builder<S>(S);".to_string(),
                members: Vec::new(),
            },
            ApiItem {
                name: "Builder".to_string(),
                kind: ApiItemKind::InherentImpl,
                source_id: None,
                owner_kind: Some(ApiItemKind::Struct),
                inherent_impl_sort_key: None,
                doc_comments: Vec::new(),
                attributes: Vec::new(),
                declaration: "impl Builder<BlobState> {".to_string(),
                members: vec![ApiMember {
                    name: "read".to_string(),
                    doc_comments: Vec::new(),
                    attributes: Vec::new(),
                    declaration: "pub fn read(self) -> Self;".to_string(),
                }],
            },
            ApiItem {
                name: "Builder".to_string(),
                kind: ApiItemKind::InherentImpl,
                source_id: None,
                owner_kind: Some(ApiItemKind::Struct),
                inherent_impl_sort_key: None,
                doc_comments: Vec::new(),
                attributes: Vec::new(),
                declaration: "impl<S: QueueState> Builder<S> {".to_string(),
                members: vec![ApiMember {
                    name: "read".to_string(),
                    doc_comments: Vec::new(),
                    attributes: Vec::new(),
                    declaration: "pub fn read(self) -> Self;".to_string(),
                }],
            },
        ],
        modules: Vec::new(),
    };

    let lines = render_module_contents(&module, &RenderOptions::default());
    let read_line_ids = lines
        .iter()
        .flat_map(|line| line.children.iter())
        .filter_map(|line| line.line_id.as_deref())
        .filter(|line_id| line_id.ends_with(".read_0"))
        .collect::<Vec<_>>();

    assert_eq!(read_line_ids.len(), 2);
    assert_ne!(read_line_ids[0], read_line_ids[1]);
}

#[test]
fn omits_doc_comment_lines_when_docs_are_disabled() {
    let module = ApiModule {
        path: "demo".to_string(),
        doc_comments: vec!["/// module docs".to_string()],
        attributes: Vec::new(),
        items: vec![ApiItem {
            name: "Foo".to_string(),
            kind: ApiItemKind::Struct,
            source_id: None,
            owner_kind: None,
            inherent_impl_sort_key: None,
            doc_comments: vec!["/// item docs".to_string()],
            attributes: Vec::new(),
            declaration: "pub struct Foo;".to_string(),
            members: vec![ApiMember {
                name: "method".to_string(),
                doc_comments: vec!["/// member docs".to_string()],
                attributes: Vec::new(),
                declaration: "pub fn method(&self);".to_string(),
            }],
        }],
        modules: Vec::new(),
    };

    let with_docs = render_module(&module, &RenderOptions::default());
    let without_docs = render_module(&module, &RenderOptions::new(false));

    assert!(with_docs.iter().any(|line| {
        line.tokens
            .iter()
            .any(|token| token.is_documentation && token.value == "/// module docs")
    }));
    assert!(!without_docs
        .iter()
        .any(|line| { line.tokens.iter().any(|token| token.is_documentation) }));
}

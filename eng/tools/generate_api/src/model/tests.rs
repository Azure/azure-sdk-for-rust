// Copyright (c) Microsoft Corporation. All rights reserved.
// Licensed under the MIT License.

use super::*;

#[test]
fn sorts_inherent_impls_by_type_parameter_then_infer_then_explicit_type() {
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
                inherent_impl_sort_key: Some(InherentImplSortKey {
                    type_arg_classes: vec![2],
                    rendered_self_type: "Builder<BlobState>".to_string(),
                }),
                doc_comments: Vec::new(),
                attributes: Vec::new(),
                declaration: "impl Builder<BlobState> {".to_string(),
                members: Vec::new(),
            },
            ApiItem {
                name: "Builder".to_string(),
                kind: ApiItemKind::InherentImpl,
                source_id: None,
                owner_kind: Some(ApiItemKind::Struct),
                inherent_impl_sort_key: Some(InherentImplSortKey {
                    type_arg_classes: vec![0],
                    rendered_self_type: "Builder<S>".to_string(),
                }),
                doc_comments: Vec::new(),
                attributes: Vec::new(),
                declaration: "impl<S> Builder<S> {".to_string(),
                members: Vec::new(),
            },
            ApiItem {
                name: "Builder".to_string(),
                kind: ApiItemKind::InherentImpl,
                source_id: None,
                owner_kind: Some(ApiItemKind::Struct),
                inherent_impl_sort_key: Some(InherentImplSortKey {
                    type_arg_classes: vec![1],
                    rendered_self_type: "Builder<_>".to_string(),
                }),
                doc_comments: Vec::new(),
                attributes: Vec::new(),
                declaration: "impl Builder<_> {".to_string(),
                members: Vec::new(),
            },
        ],
        modules: Vec::new(),
    };

    let declarations = module
        .sorted_items()
        .into_iter()
        .map(|item| item.declaration.as_str())
        .collect::<Vec<_>>();

    assert_eq!(
        declarations,
        vec![
            "pub struct Builder<S>(S);",
            "impl<S> Builder<S> {",
            "impl Builder<_> {",
            "impl Builder<BlobState> {",
        ]
    );
}

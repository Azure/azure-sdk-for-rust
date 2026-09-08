// Copyright (c) Microsoft Corporation. All rights reserved.
// Licensed under the MIT License.

use super::*;

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

#[test]
fn orders_feature_names_and_default_children() {
    let metadata = PackageMetadata {
        description: None,
        edition: None,
        rust_version: None,
        features: BTreeMap::from([
            ("alpha".to_string(), vec!["ignored".to_string()]),
            (
                "default".to_string(),
                vec!["zeta".to_string(), "alpha".to_string()],
            ),
            ("zeta".to_string(), Vec::new()),
        ]),
    };

    assert_eq!(
        metadata
            .feature_names()
            .map(String::as_str)
            .collect::<Vec<_>>(),
        vec!["default", "alpha", "zeta"]
    );
    assert_eq!(
        metadata
            .default_feature_children()
            .into_iter()
            .map(String::as_str)
            .collect::<Vec<_>>(),
        vec!["alpha", "zeta"]
    );
}

#[test]
fn splits_description_lines_after_ignoring_one_final_newline() {
    let single_line = PackageMetadata {
        description: Some("Single-line comment\n".to_string()),
        ..Default::default()
    };
    assert_eq!(
        single_line.description_lines(),
        Some(vec!["Single-line comment"])
    );

    let multi_line = PackageMetadata {
        description: Some("Multi-line\ncomment\n".to_string()),
        ..Default::default()
    };
    assert_eq!(
        multi_line.description_lines(),
        Some(vec!["Multi-line", "comment"])
    );
}

#[test]
fn sorts_inherent_impls_by_type_parameter_then_infer_then_explicit_type() {
    let mut explicit = item(
        "Builder",
        ApiItemKind::InherentImpl,
        "impl Builder<BlobState> {",
    );
    explicit.owner_name = Some("Builder".to_string());
    explicit.owner_kind = Some(ApiItemKind::Struct);
    explicit.inherent_impl_sort_key = Some(InherentImplSortKey {
        type_arg_classes: vec![2],
        rendered_self_type: "Builder<BlobState>".to_string(),
    });

    let mut generic = item("Builder", ApiItemKind::InherentImpl, "impl<S> Builder<S> {");
    generic.owner_name = Some("Builder".to_string());
    generic.owner_kind = Some(ApiItemKind::Struct);
    generic.inherent_impl_sort_key = Some(InherentImplSortKey {
        type_arg_classes: vec![0],
        rendered_self_type: "Builder<S>".to_string(),
    });

    let mut inferred = item("Builder", ApiItemKind::InherentImpl, "impl Builder<_> {");
    inferred.owner_name = Some("Builder".to_string());
    inferred.owner_kind = Some(ApiItemKind::Struct);
    inferred.inherent_impl_sort_key = Some(InherentImplSortKey {
        type_arg_classes: vec![1],
        rendered_self_type: "Builder<_>".to_string(),
    });

    let module = ApiModule {
        declaration_location: None,
        path: "demo".to_string(),
        doc_comments: Vec::new(),
        attributes: Vec::new(),
        items: vec![
            item("Builder", ApiItemKind::Struct, "pub struct Builder<S>(S);"),
            explicit,
            generic,
            inferred,
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

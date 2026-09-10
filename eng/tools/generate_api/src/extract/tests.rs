// Copyright (c) Microsoft Corporation. All rights reserved.
// Licensed under the MIT License.

use super::*;
use crate::model::{ApiMemberKind, ApiPathReference};
use rustdoc_types::{
    Abi, Enum as RustdocEnum, FunctionSignature, Generics, ItemSummary, Module, Struct, Target,
    Type,
};
use std::collections::HashMap;

#[test]
fn recognizes_common_derive_trait_paths() {
    assert_eq!(known_derive_trait_name(&path("Clone", 1)), Some("Clone"));
    assert_eq!(
        known_derive_trait_name(&path("fmt::Debug", 1)),
        Some("Debug")
    );
    assert_eq!(
        known_derive_trait_name(&path("std::fmt::Debug", 1)),
        Some("Debug")
    );
    assert_eq!(
        known_derive_trait_name(&path("Serialize", 1)),
        Some("serde::Serialize")
    );
    assert_eq!(
        known_derive_trait_name(&path("serde::de::Deserialize", 1)),
        Some("serde::Deserialize")
    );
    assert_eq!(known_derive_trait_name(&path("SafeDebug", 1)), None);
}

#[test]
fn synthesizes_known_derives_and_ignores_workspace_defined_traits() {
    let struct_id = Id(1);
    let clone_impl_id = Id(2);
    let debug_impl_id = Id(3);
    let serialize_impl_id = Id(4);
    let safe_debug_impl_id = Id(5);
    let explicit_default_impl_id = Id(6);

    let krate = crate_with_items(vec![
        item(
            struct_id,
            Some("Model"),
            ItemEnum::Struct(Struct {
                kind: StructKind::Unit,
                generics: empty_generics(),
                impls: vec![
                    clone_impl_id,
                    debug_impl_id,
                    serialize_impl_id,
                    safe_debug_impl_id,
                    explicit_default_impl_id,
                ],
            }),
        ),
        impl_item(
            clone_impl_id,
            Some(path("Clone", 10)),
            "Model",
            struct_id,
            true,
        ),
        impl_item(
            debug_impl_id,
            Some(path("fmt::Debug", 11)),
            "Model",
            struct_id,
            true,
        ),
        impl_item(
            serialize_impl_id,
            Some(path("Serialize", 12)),
            "Model",
            struct_id,
            true,
        ),
        impl_item(
            safe_debug_impl_id,
            Some(path("SafeDebug", 13)),
            "Model",
            struct_id,
            true,
        ),
        impl_item(
            explicit_default_impl_id,
            Some(path("Default", 14)),
            "Model",
            struct_id,
            false,
        ),
    ]);

    let item = krate.index.get(&struct_id).expect("struct item present");
    let attribute = synthesize_derive_attribute(&krate, item)
        .expect("recognized derive attribute should be synthesized");

    assert_eq!(attribute.text, "#[derive(Clone, Debug, serde::Serialize)]");
}

#[test]
fn extracts_explicit_trait_impl_blocks_with_members() {
    let struct_id = Id(1);
    let impl_id = Id(2);
    let fmt_id = Id(3);

    let model = extract_model(
        &package_metadata("demo"),
        &crate_with_items(vec![
            item(
                struct_id,
                Some("MyType"),
                ItemEnum::Struct(Struct {
                    kind: StructKind::Unit,
                    generics: empty_generics(),
                    impls: vec![impl_id],
                }),
            ),
            impl_item_with_items(
                impl_id,
                Some(path("fmt::Debug", 10)),
                "MyType",
                struct_id,
                false,
                vec![fmt_id],
            ),
            item(
                fmt_id,
                Some("fmt"),
                ItemEnum::Function(Function {
                    sig: FunctionSignature {
                        inputs: vec![
                            (
                                "self".to_string(),
                                Type::BorrowedRef {
                                    lifetime: None,
                                    is_mutable: false,
                                    type_: Box::new(Type::Generic("Self".to_string())),
                                },
                            ),
                            (
                                "f".to_string(),
                                Type::BorrowedRef {
                                    lifetime: None,
                                    is_mutable: true,
                                    type_: Box::new(Type::ResolvedPath(path("fmt::Formatter", 11))),
                                },
                            ),
                        ],
                        output: Some(Type::ResolvedPath(path("fmt::Result", 12))),
                        is_c_variadic: false,
                    },
                    generics: empty_generics(),
                    header: FunctionHeader {
                        is_const: false,
                        is_unsafe: false,
                        is_async: false,
                        abi: Abi::Rust,
                    },
                    has_body: true,
                }),
            ),
        ]),
        &mut NoopResolver,
    )
    .expect("model extraction should succeed");

    let trait_impl = model
        .root_module
        .items
        .iter()
        .find(|item| item.kind == ApiItemKind::TraitImpl)
        .expect("explicit trait impl should be extracted");

    assert_eq!(trait_impl.declaration, "impl fmt::Debug for MyType {");
    assert_eq!(trait_impl.members.len(), 1);
    assert_eq!(
        trait_impl.members[0].declaration,
        "fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result;"
    );
}

#[test]
fn extracts_declaration_path_references_for_resolved_result_paths() {
    let result_id = Id(1);
    let function_id = Id(2);

    let model = extract_model(
        &package_metadata("demo"),
        &crate_with_items(vec![
            item(
                result_id,
                Some("Result"),
                ItemEnum::Struct(Struct {
                    kind: StructKind::Unit,
                    generics: empty_generics(),
                    impls: Vec::new(),
                }),
            ),
            item(
                function_id,
                Some("parse"),
                ItemEnum::Function(Function {
                    sig: FunctionSignature {
                        inputs: vec![
                            (
                                "std_result".to_string(),
                                Type::ResolvedPath(path("std::result::Result", 10)),
                            ),
                            (
                                "std_fmt".to_string(),
                                Type::ResolvedPath(path("std::fmt::Result", 11)),
                            ),
                            (
                                "fmt_result".to_string(),
                                Type::ResolvedPath(path("fmt::Result", 12)),
                            ),
                        ],
                        output: Some(Type::ResolvedPath(path("Result", result_id.0))),
                        is_c_variadic: false,
                    },
                    generics: empty_generics(),
                    header: FunctionHeader {
                        is_const: false,
                        is_unsafe: false,
                        is_async: false,
                        abi: Abi::Rust,
                    },
                    has_body: true,
                }),
            ),
        ]),
        &mut NoopResolver,
    )
    .expect("model extraction should succeed");

    let parse = model
        .root_module
        .items
        .iter()
        .find(|item| item.name == "parse")
        .expect("function should be extracted");

    assert_eq!(
        parse
            .declaration_path_references
            .iter()
            .map(|reference| ApiPathReference {
                path: reference.path.clone(),
                canonical_path: reference.canonical_path.clone(),
                target_source_id: reference.target_source_id.clone(),
            })
            .collect::<Vec<_>>(),
        vec![
            ApiPathReference {
                path: "std::result::Result".to_string(),
                canonical_path: None,
                target_source_id: Some("10".to_string()),
            },
            ApiPathReference {
                path: "std::fmt::Result".to_string(),
                canonical_path: None,
                target_source_id: Some("11".to_string()),
            },
            ApiPathReference {
                path: "fmt::Result".to_string(),
                canonical_path: None,
                target_source_id: Some("12".to_string()),
            },
            ApiPathReference {
                path: "Result".to_string(),
                canonical_path: None,
                target_source_id: Some(result_id.0.to_string()),
            },
        ]
    );
}

#[test]
fn extract_item_synthesizes_async_trait_and_elides_synthetic_lifetimes() {
    let function_id = Id(2);
    let trait_id = Id(1);

    let krate = crate_with_items(vec![
        item(
            trait_id,
            Some("Polling"),
            ItemEnum::Trait(Trait {
                is_auto: false,
                is_unsafe: false,
                is_dyn_compatible: true,
                items: vec![function_id],
                generics: empty_generics(),
                bounds: Vec::new(),
                implementations: Vec::new(),
            }),
        ),
        item(
            function_id,
            Some("poll"),
            ItemEnum::Function(Function {
                sig: FunctionSignature {
                    inputs: vec![(
                        "self".to_string(),
                        Type::BorrowedRef {
                            lifetime: Some("'life0".to_string()),
                            is_mutable: false,
                            type_: Box::new(Type::Generic("Self".to_string())),
                        },
                    )],
                    output: None,
                    is_c_variadic: false,
                },
                generics: Generics {
                    params: vec![lifetime_param("'life0"), lifetime_param("'async_trait")],
                    where_predicates: Vec::new(),
                },
                header: FunctionHeader {
                    is_const: false,
                    is_unsafe: false,
                    is_async: false,
                    abi: Abi::Rust,
                },
                has_body: false,
            }),
        ),
    ]);

    let item = krate.index.get(&trait_id).expect("trait item present");
    let extracted = extract_item(&krate, item);

    assert!(
        extracted
            .attributes
            .iter()
            .any(|attribute| attribute.text == "#[async_trait]"),
        "trait should synthesize #[async_trait]"
    );
    assert_eq!(extracted.members.len(), 1);
    assert_eq!(extracted.members[0].declaration, "fn poll(&self);");
}

#[test]
fn suppresses_pin_project_generated_unpin_impls_for_bare_attributes() {
    let struct_id = Id(1);
    let impl_id = Id(2);

    let model = extract_model(
        &package_metadata("demo"),
        &crate_with_items(vec![
            item(
                struct_id,
                Some("AsyncResponseBody"),
                ItemEnum::Struct(Struct {
                    kind: StructKind::Tuple(vec![None]),
                    generics: empty_generics(),
                    impls: vec![impl_id],
                }),
            )
            .with_attrs(vec!["#[pin(__private())]".to_string()]),
            trait_impl_item_for_type(
                impl_id,
                path("Unpin", 10),
                Type::ResolvedPath(path("AsyncResponseBody", struct_id.0)),
                Generics {
                    params: vec![lifetime_param("'pin")],
                    where_predicates: vec![pin_project_generated_unpin_predicate(
                        Type::ResolvedPath(path("__AsyncResponseBody", 11).with_args(
                            GenericArgs::AngleBracketed {
                                args: vec![GenericArg::Lifetime("'pin".to_string())],
                                constraints: Vec::new(),
                            },
                        )),
                    )],
                },
            ),
        ]),
        &mut NoopResolver,
    )
    .expect("model extraction should succeed");

    assert_eq!(model.root_module.items.len(), 1);
    assert_eq!(model.root_module.items[0].kind, ApiItemKind::Struct);
    assert_eq!(
        model.root_module.items[0]
            .attributes
            .iter()
            .map(|attribute| attribute.text.as_str())
            .collect::<Vec<_>>(),
        vec!["#[pin_project]"]
    );
    assert!(
        model
            .root_module
            .items
            .iter()
            .all(|item| item.kind != ApiItemKind::TraitImpl),
        "pin-project generated Unpin impl should be suppressed"
    );
}

#[test]
fn suppresses_pin_project_generated_unpin_impls_for_argument_bearing_attributes() {
    let struct_id = Id(1);
    let field_id = Id(2);
    let impl_id = Id(3);
    let mut field = item(
        field_id,
        Some("iter"),
        ItemEnum::StructField(Type::ResolvedPath(path("PageIterator<P>", 20))),
    )
    .with_attrs(vec!["#[pin]".to_string()]);
    field.visibility = Visibility::Default;

    let model = extract_model(
        &package_metadata("demo"),
        &crate_with_items(vec![
            item(
                struct_id,
                Some("ItemIterator"),
                ItemEnum::Struct(Struct {
                    kind: StructKind::Plain {
                        fields: vec![field_id],
                        has_stripped_fields: false,
                    },
                    generics: Generics {
                        params: vec![type_param("P")],
                        where_predicates: Vec::new(),
                    },
                    impls: vec![impl_id],
                }),
            )
            .with_attrs(vec![
                "#[pin_project::pin_project(project = ItemIteratorProjection, project_replace = ItemIteratorProjectionOwned)]"
                    .to_string(),
            ]),
            field,
            trait_impl_item_for_type(
                impl_id,
                path("Unpin", 21),
                Type::ResolvedPath(path("ItemIterator", struct_id.0).with_args(
                    GenericArgs::AngleBracketed {
                        args: vec![GenericArg::Type(Type::Generic("P".to_string()))],
                        constraints: Vec::new(),
                    },
                )),
                Generics {
                    params: vec![lifetime_param("'pin"), type_param("P")],
                    where_predicates: vec![
                        bound_predicate(
                            Type::Generic("P".to_string()),
                            vec![trait_bound("Page", 22), trait_bound("Send", 23)],
                        ),
                        pin_project_generated_unpin_predicate(Type::ResolvedPath(
                            path("__ItemIterator", 24).with_args(GenericArgs::AngleBracketed {
                                args: vec![
                                    GenericArg::Lifetime("'pin".to_string()),
                                    GenericArg::Type(Type::Generic("P".to_string())),
                                ],
                                constraints: Vec::new(),
                            }),
                        )),
                    ],
                },
            ),
        ]),
        &mut NoopResolver,
    )
    .expect("model extraction should succeed");

    assert_eq!(model.root_module.items.len(), 1);
    assert_eq!(model.root_module.items[0].kind, ApiItemKind::Struct);
    assert_eq!(
        model.root_module.items[0]
            .attributes
            .iter()
            .map(|attribute| attribute.text.as_str())
            .collect::<Vec<_>>(),
        vec!["#[pin_project(project = ItemIteratorProjection, project_replace = ItemIteratorProjectionOwned)]"]
    );
    assert!(
        model
            .root_module
            .items
            .iter()
            .all(|item| item.kind != ApiItemKind::TraitImpl),
        "pin-project generated Unpin impl should be suppressed"
    );
}

#[test]
fn preserves_manual_unpin_impls() {
    let struct_id = Id(1);
    let impl_id = Id(2);

    let model = extract_model(
        &package_metadata("demo"),
        &crate_with_items(vec![
            item(
                struct_id,
                Some("ItemIterator"),
                ItemEnum::Struct(Struct {
                    kind: StructKind::Unit,
                    generics: Generics {
                        params: vec![type_param("P")],
                        where_predicates: Vec::new(),
                    },
                    impls: vec![impl_id],
                }),
            ),
            trait_impl_item_for_type(
                impl_id,
                path("Unpin", 30),
                Type::ResolvedPath(path("ItemIterator", struct_id.0).with_args(
                    GenericArgs::AngleBracketed {
                        args: vec![GenericArg::Type(Type::Generic("P".to_string()))],
                        constraints: Vec::new(),
                    },
                )),
                Generics {
                    params: vec![type_param("P")],
                    where_predicates: vec![bound_predicate(
                        Type::Generic("P".to_string()),
                        vec![trait_bound("Send", 31)],
                    )],
                },
            ),
        ]),
        &mut NoopResolver,
    )
    .expect("model extraction should succeed");

    let trait_impls = model
        .root_module
        .items
        .iter()
        .filter(|item| item.kind == ApiItemKind::TraitImpl)
        .collect::<Vec<_>>();

    assert_eq!(trait_impls.len(), 1);
    assert_eq!(
        trait_impls[0].declaration,
        "impl<P> Unpin for ItemIterator<P> where P: Send {"
    );
}

#[test]
fn extracts_root_module_attributes_as_inner_attrs() {
    let mut krate = crate_with_items(vec![item(
        Id(1),
        Some("Foo"),
        ItemEnum::Struct(Struct {
            kind: StructKind::Unit,
            generics: empty_generics(),
            impls: Vec::new(),
        }),
    )]);
    krate
        .index
        .get_mut(&Id(0))
        .expect("crate root present")
        .attrs = vec![
        rustdoc_types::Attribute::Other("#[warn(missing_docs)]".to_string()),
        rustdoc_types::Attribute::Other("#[doc = include_str!(\"../README.md\")]".to_string()),
    ];

    let model = extract_model(&package_metadata("demo"), &krate, &mut NoopResolver)
        .expect("model extraction should succeed");

    assert_eq!(
        model
            .root_module
            .attributes
            .iter()
            .map(|attribute| attribute.text.as_str())
            .collect::<Vec<_>>(),
        vec![
            "#![warn(missing_docs)]",
            "#![doc = include_str!(\"../README.md\")]",
        ]
    );
}

#[test]
fn extracts_module_scope_lint_attrs_without_rewriting_item_attrs() {
    let module_id = Id(1);
    let module_item_id = Id(2);
    let root_item_id = Id(3);
    let model = extract_model(
        &package_metadata("demo"),
        &crate_with_root_items(
            vec![module_id, root_item_id],
            vec![
                module_item(module_id, "inner", vec![module_item_id], false)
                    .with_attrs(vec!["#[deny(unsafe_code)]".to_string()]),
                item(
                    module_item_id,
                    Some("Nested"),
                    ItemEnum::Struct(Struct {
                        kind: StructKind::Unit,
                        generics: empty_generics(),
                        impls: Vec::new(),
                    }),
                ),
                item(
                    root_item_id,
                    Some("Root"),
                    ItemEnum::Struct(Struct {
                        kind: StructKind::Unit,
                        generics: empty_generics(),
                        impls: Vec::new(),
                    }),
                )
                .with_attrs(vec!["#[deny(unsafe_code)]".to_string()]),
            ],
        ),
        &mut NoopResolver,
    )
    .expect("model extraction should succeed");

    assert_eq!(
        model.root_module.modules[0]
            .attributes
            .iter()
            .map(|attribute| attribute.text.as_str())
            .collect::<Vec<_>>(),
        vec!["#[deny(unsafe_code)]"]
    );
    assert_eq!(
        model.root_module.items[0]
            .attributes
            .iter()
            .map(|attribute| attribute.text.as_str())
            .collect::<Vec<_>>(),
        vec!["#[deny(unsafe_code)]"]
    );
}

#[test]
fn extracts_unassociated_trait_impl_blocks() {
    let trait_id = Id(1);
    let impl_id = Id(2);

    let model = extract_model(
        &package_metadata("demo"),
        &crate_with_items(vec![
            item(
                trait_id,
                Some("LocalTrait"),
                ItemEnum::Trait(Trait {
                    is_auto: false,
                    is_unsafe: false,
                    is_dyn_compatible: true,
                    items: Vec::new(),
                    generics: empty_generics(),
                    bounds: Vec::new(),
                    implementations: Vec::new(),
                }),
            ),
            trait_impl_item_for_type(
                impl_id,
                path("LocalTrait", trait_id.0),
                Type::ResolvedPath(path("other_crate::ExternalType", 99)),
                empty_generics(),
            ),
        ]),
        &mut NoopResolver,
    )
    .expect("model extraction should succeed");

    assert_eq!(
        model
            .root_module
            .items
            .iter()
            .filter(|item| item.kind == ApiItemKind::TraitImpl)
            .map(|item| item.declaration.as_str())
            .collect::<Vec<_>>(),
        vec!["impl LocalTrait for other_crate::ExternalType {"]
    );
}

#[test]
fn renders_self_receivers_in_source_like_forms() {
    let function = Function {
        sig: FunctionSignature {
            inputs: vec![
                ("self".to_string(), Type::Generic("Self".to_string())),
                (
                    "other".to_string(),
                    Type::ResolvedPath(path("Pin", 30).with_args(GenericArgs::AngleBracketed {
                        args: vec![GenericArg::Type(Type::Generic("Self".to_string()))],
                        constraints: Vec::new(),
                    })),
                ),
            ],
            output: Some(Type::Generic("Self".to_string())),
            is_c_variadic: false,
        },
        generics: empty_generics(),
        header: FunctionHeader {
            is_const: false,
            is_unsafe: false,
            is_async: false,
            abi: Abi::Rust,
        },
        has_body: false,
    };

    assert_eq!(
        render_function_declaration("into_self", &function, false),
        "fn into_self(self, other: Pin<Self>) -> Self;"
    );

    let mut_ref_function = Function {
        sig: FunctionSignature {
            inputs: vec![(
                "self".to_string(),
                Type::BorrowedRef {
                    lifetime: None,
                    is_mutable: true,
                    type_: Box::new(Type::Generic("Self".to_string())),
                },
            )],
            output: None,
            is_c_variadic: false,
        },
        generics: empty_generics(),
        header: FunctionHeader {
            is_const: false,
            is_unsafe: false,
            is_async: false,
            abi: Abi::Rust,
        },
        has_body: false,
    };

    assert_eq!(
        render_function_declaration("touch", &mut_ref_function, false),
        "fn touch(&mut self);"
    );
}

#[test]
fn extract_item_uses_reexport_leaf_name_when_rustdoc_item_name_is_missing() {
    let bytes_id = Id(1);
    let reexport_id = Id(2);
    let krate = crate_with_items(vec![
        item(
            bytes_id,
            Some("Bytes"),
            ItemEnum::Struct(Struct {
                kind: StructKind::Unit,
                generics: empty_generics(),
                impls: Vec::new(),
            }),
        ),
        item(
            reexport_id,
            None,
            ItemEnum::Use(rustdoc_types::Use {
                source: "bytes::Bytes".to_string(),
                name: "Bytes".to_string(),
                id: Some(bytes_id),
                is_glob: false,
            }),
        ),
    ]);

    let reexport = krate
        .index
        .get(&reexport_id)
        .expect("reexport item present");
    let extracted = extract_item(&krate, reexport);

    assert_eq!(extracted.name, "Bytes");
    assert_eq!(extracted.declaration, "pub use bytes::Bytes;");
    assert_eq!(
        extracted
            .navigation_paths
            .iter()
            .map(|path| path.path.as_str())
            .collect::<Vec<_>>(),
        vec!["bytes::Bytes"]
    );
}

#[test]
fn local_reexport_carries_explicit_trait_impls_for_reexported_items() {
    let hidden_module_id = Id(1);
    let struct_id = Id(2);
    let impl_id = Id(3);
    let fmt_id = Id(4);
    let reexport_id = Id(5);

    let model = extract_model(
        &package_metadata("demo"),
        &crate_with_root_items(
            vec![hidden_module_id, reexport_id],
            vec![
                module_item(hidden_module_id, "hidden", vec![struct_id, impl_id], true),
                item(
                    struct_id,
                    Some("Error"),
                    ItemEnum::Struct(Struct {
                        kind: StructKind::Unit,
                        generics: empty_generics(),
                        impls: vec![impl_id],
                    }),
                ),
                impl_item_with_items(
                    impl_id,
                    Some(path("fmt::Debug", 10)),
                    "Error",
                    struct_id,
                    false,
                    vec![fmt_id],
                ),
                item(
                    fmt_id,
                    Some("fmt"),
                    ItemEnum::Function(Function {
                        sig: FunctionSignature {
                            inputs: vec![
                                (
                                    "self".to_string(),
                                    Type::BorrowedRef {
                                        lifetime: None,
                                        is_mutable: false,
                                        type_: Box::new(Type::Generic("Self".to_string())),
                                    },
                                ),
                                (
                                    "f".to_string(),
                                    Type::BorrowedRef {
                                        lifetime: None,
                                        is_mutable: true,
                                        type_: Box::new(Type::ResolvedPath(path(
                                            "fmt::Formatter",
                                            11,
                                        ))),
                                    },
                                ),
                            ],
                            output: Some(Type::ResolvedPath(path("fmt::Result", 12))),
                            is_c_variadic: false,
                        },
                        generics: empty_generics(),
                        header: FunctionHeader {
                            is_const: false,
                            is_unsafe: false,
                            is_async: false,
                            abi: Abi::Rust,
                        },
                        has_body: true,
                    }),
                ),
                item(
                    reexport_id,
                    Some("Error"),
                    ItemEnum::Use(rustdoc_types::Use {
                        source: "crate::hidden::Error".to_string(),
                        name: "Error".to_string(),
                        id: Some(struct_id),
                        is_glob: false,
                    }),
                ),
            ],
        ),
        &mut NoopResolver,
    )
    .expect("model extraction should succeed");

    assert!(model.root_module.modules.is_empty());
    assert!(model
        .root_module
        .items
        .iter()
        .any(|item| item.declaration == "pub struct Error;"));
    assert!(model.root_module.items.iter().any(|item| {
        item.kind == ApiItemKind::TraitImpl
            && item.declaration == "impl fmt::Debug for Error {"
            && item.members.iter().any(|member| member.name == "fmt")
    }));
}

#[test]
fn local_types_are_followed_by_inherent_then_trait_impls() {
    let secret_id = Id(1);
    let debug_impl_id = Id(2);
    let inherent_impl_id = Id(3);
    let fmt_id = Id(4);
    let new_id = Id(5);
    let zebra_id = Id(6);

    let model = extract_model(
        &package_metadata("demo"),
        &crate_with_root_items(
            vec![debug_impl_id, zebra_id, secret_id, inherent_impl_id],
            vec![
                item(
                    secret_id,
                    Some("Secret"),
                    ItemEnum::Struct(Struct {
                        kind: StructKind::Unit,
                        generics: empty_generics(),
                        impls: vec![debug_impl_id, inherent_impl_id],
                    }),
                ),
                impl_item_with_items(
                    debug_impl_id,
                    Some(path("fmt::Debug", 30)),
                    "Secret",
                    secret_id,
                    false,
                    vec![fmt_id],
                ),
                item(
                    fmt_id,
                    Some("fmt"),
                    ItemEnum::Function(Function {
                        sig: FunctionSignature {
                            inputs: vec![(
                                "self".to_string(),
                                Type::BorrowedRef {
                                    lifetime: None,
                                    is_mutable: false,
                                    type_: Box::new(Type::Generic("Self".to_string())),
                                },
                            )],
                            output: Some(Type::ResolvedPath(path("fmt::Result", 31))),
                            is_c_variadic: false,
                        },
                        generics: empty_generics(),
                        header: FunctionHeader {
                            is_const: false,
                            is_unsafe: false,
                            is_async: false,
                            abi: Abi::Rust,
                        },
                        has_body: true,
                    }),
                ),
                impl_item_for_type_with_items(
                    inherent_impl_id,
                    Type::ResolvedPath(path("Secret", secret_id.0)),
                    empty_generics(),
                    vec![new_id],
                ),
                inherent_method(new_id, "new"),
                item(
                    zebra_id,
                    Some("Zebra"),
                    ItemEnum::Struct(Struct {
                        kind: StructKind::Unit,
                        generics: empty_generics(),
                        impls: Vec::new(),
                    }),
                ),
            ],
        ),
        &mut NoopResolver,
    )
    .expect("model extraction should succeed");

    let declarations = model
        .root_module
        .sorted_items()
        .into_iter()
        .map(|item| item.declaration.as_str())
        .collect::<Vec<_>>();

    assert_eq!(
        declarations,
        vec![
            "pub struct Secret;",
            "impl Secret {",
            "impl fmt::Debug for Secret {",
            "pub struct Zebra;",
        ]
    );
}

#[test]
fn local_reexport_preserves_synthesized_derives_for_reexported_items() {
    let hidden_module_id = Id(1);
    let struct_id = Id(2);
    let clone_impl_id = Id(3);
    let debug_impl_id = Id(4);
    let reexport_id = Id(5);

    let model = extract_model(
        &package_metadata("demo"),
        &crate_with_root_items(
            vec![hidden_module_id, reexport_id],
            vec![
                module_item(
                    hidden_module_id,
                    "hidden",
                    vec![struct_id, clone_impl_id, debug_impl_id],
                    true,
                ),
                item(
                    struct_id,
                    Some("ErrorKind"),
                    ItemEnum::Struct(Struct {
                        kind: StructKind::Unit,
                        generics: empty_generics(),
                        impls: vec![clone_impl_id, debug_impl_id],
                    }),
                ),
                impl_item(
                    clone_impl_id,
                    Some(path("Clone", 20)),
                    "ErrorKind",
                    struct_id,
                    true,
                ),
                impl_item(
                    debug_impl_id,
                    Some(path("fmt::Debug", 21)),
                    "ErrorKind",
                    struct_id,
                    true,
                ),
                item(
                    reexport_id,
                    Some("ErrorKind"),
                    ItemEnum::Use(rustdoc_types::Use {
                        source: "crate::hidden::ErrorKind".to_string(),
                        name: "ErrorKind".to_string(),
                        id: Some(struct_id),
                        is_glob: false,
                    }),
                ),
            ],
        ),
        &mut NoopResolver,
    )
    .expect("model extraction should succeed");

    let item = model
        .root_module
        .items
        .iter()
        .find(|item| item.declaration == "pub struct ErrorKind;")
        .expect("re-exported struct should be lifted");

    assert!(model.root_module.modules.is_empty());
    assert_eq!(
        item.attributes
            .iter()
            .map(|attribute| attribute.text.as_str())
            .collect::<Vec<_>>(),
        vec!["#[derive(Clone, Debug)]"]
    );
}

#[test]
fn model_reexport_collects_nested_impls_by_owner_identity() {
    let expanded = expand_model_item_reexport(
        &ApiModule {
            declaration_location: None,
            path: "azure_core".to_string(),
            doc_comments: Vec::new(),
            attributes: Vec::new(),
            items: Vec::new(),
            modules: vec![ApiModule {
                declaration_location: None,
                path: "azure_core::credentials".to_string(),
                doc_comments: Vec::new(),
                attributes: Vec::new(),
                items: vec![
                    ApiItem {
                        declaration_location: None,
                        name: "Secret".to_string(),
                        kind: ApiItemKind::Struct,
                        source_id: Some("secret".to_string()),
                        navigation_paths: Vec::new(),
                        owner_name: None,
                        owner_kind: None,
                        owner_source_id: None,
                        inherent_impl_sort_key: None,
                        doc_comments: Vec::new(),
                        attributes: Vec::new(),
                        declaration: "pub struct Secret<T>(T);".to_string(),
                        declaration_path_references: Vec::new(),
                        members: Vec::new(),
                    },
                    ApiItem {
                        declaration_location: None,
                        name: "Secret".to_string(),
                        kind: ApiItemKind::InherentImpl,
                        source_id: Some("secret-inherent".to_string()),
                        navigation_paths: Vec::new(),
                        owner_name: Some("Secret".to_string()),
                        owner_kind: Some(ApiItemKind::Struct),
                        owner_source_id: Some("secret".to_string()),
                        inherent_impl_sort_key: Some(InherentImplSortKey {
                            type_arg_classes: vec![0],
                            rendered_self_type: "Secret<T>".to_string(),
                        }),
                        doc_comments: Vec::new(),
                        attributes: Vec::new(),
                        declaration: "impl<T> Secret<T> {".to_string(),
                        declaration_path_references: Vec::new(),
                        members: Vec::new(),
                    },
                    ApiItem {
                        declaration_location: None,
                        name: "Secret<T>".to_string(),
                        kind: ApiItemKind::TraitImpl,
                        source_id: Some("secret-debug".to_string()),
                        navigation_paths: Vec::new(),
                        owner_name: Some("Secret".to_string()),
                        owner_kind: Some(ApiItemKind::Struct),
                        owner_source_id: Some("secret".to_string()),
                        inherent_impl_sort_key: None,
                        doc_comments: Vec::new(),
                        attributes: Vec::new(),
                        declaration: "impl fmt::Debug for Secret<T> {".to_string(),
                        declaration_path_references: Vec::new(),
                        members: Vec::new(),
                    },
                ],
                modules: Vec::new(),
            }],
        },
        &["credentials", "Secret"],
    )
    .expect("model re-export should resolve nested item");

    assert_eq!(
        expanded
            .items
            .iter()
            .map(|item| item.declaration.as_str())
            .collect::<Vec<_>>(),
        vec![
            "pub struct Secret<T>(T);",
            "impl<T> Secret<T> {",
            "impl fmt::Debug for Secret<T> {",
        ]
    );
}

#[test]
fn model_reexport_resolves_duplicated_leading_module_segments() {
    let expanded = expand_model_item_reexport(
        &ApiModule {
            declaration_location: None,
            path: "demo".to_string(),
            doc_comments: Vec::new(),
            attributes: Vec::new(),
            items: Vec::new(),
            modules: vec![ApiModule {
                declaration_location: None,
                path: "demo::credentials".to_string(),
                doc_comments: Vec::new(),
                attributes: Vec::new(),
                items: vec![ApiItem {
                    declaration_location: None,
                    name: "Secret".to_string(),
                    kind: ApiItemKind::Struct,
                    source_id: Some("secret".to_string()),
                    navigation_paths: Vec::new(),
                    owner_name: None,
                    owner_kind: None,
                    owner_source_id: None,
                    inherent_impl_sort_key: None,
                    doc_comments: Vec::new(),
                    attributes: Vec::new(),
                    declaration: "pub struct Secret;".to_string(),
                    declaration_path_references: Vec::new(),
                    members: Vec::new(),
                }],
                modules: Vec::new(),
            }],
        },
        &["demo", "credentials", "Secret"],
    )
    .expect("duplicated leading root segment should still resolve");

    assert_eq!(expanded.items.len(), 1);
    assert_eq!(expanded.items[0].declaration, "pub struct Secret;");
}

#[test]
fn normalize_attribute_flattens_multiline_reason_strings() {
    assert_eq!(
        normalize_attribute(
            "#[allow(unknown_lints, clippy::infallible_try_from, reason =\n\"maintain a consistent pattern of `try_into()`\")]"
        ),
        "#[allow(unknown_lints, clippy::infallible_try_from, reason = \"maintain a consistent pattern of `try_into()`\")]"
    );
}

#[test]
fn normalize_attribute_flattens_multiline_pin_project_arguments() {
    assert_eq!(
        normalize_attribute(
            "#[pin_project(project = ItemIteratorProjection, project_replace =\nItemIteratorProjectionOwned)]"
        ),
        "#[pin_project(project = ItemIteratorProjection, project_replace = ItemIteratorProjectionOwned)]"
    );
}

#[test]
fn normalize_attribute_rewrites_namespaced_pin_project_forms() {
    assert_eq!(
        normalize_attribute("#[pin_project::pin_project]"),
        "#[pin_project]"
    );
    assert_eq!(
        normalize_attribute("#[pin_project::pin_project(project = BodyProj)]"),
        "#[pin_project(project = BodyProj)]"
    );
}

#[test]
fn extracts_pinned_tuple_struct_source_shape() {
    let struct_id = Id(1);
    let field_id = Id(2);
    let mut field = item(
        field_id,
        None,
        ItemEnum::StructField(Type::ResolvedPath(path("Body", 3))),
    )
    .with_attrs(vec!["#[pin]".to_string()]);
    field.visibility = Visibility::Default;
    let krate = crate_with_items(vec![
        item(
            struct_id,
            Some("AsyncResponseBody"),
            ItemEnum::Struct(Struct {
                kind: StructKind::Tuple(vec![Some(field_id)]),
                generics: empty_generics(),
                impls: Vec::new(),
            }),
        ),
        field,
    ]);

    let struct_item = krate.index.get(&struct_id).expect("struct item present");
    let extracted = extract_item(&krate, struct_item);

    assert_eq!(
        extracted
            .attributes
            .iter()
            .map(|attribute| attribute.text.as_str())
            .collect::<Vec<_>>(),
        vec!["#[pin_project]"]
    );
    assert_eq!(
        extracted.declaration,
        "pub struct AsyncResponseBody(#[pin] Body);"
    );
}

#[test]
fn renders_private_pinned_tuple_structs_as_opaque_fields() {
    let struct_id = Id(1);
    let krate = crate_with_items(vec![item(
        struct_id,
        Some("AsyncResponseBody"),
        ItemEnum::Struct(Struct {
            kind: StructKind::Tuple(vec![None]),
            generics: empty_generics(),
            impls: Vec::new(),
        }),
    )
    .with_attrs(vec!["#[pin(__private())]".to_string()])]);

    let struct_item = krate.index.get(&struct_id).expect("struct item present");
    let extracted = extract_item(&krate, struct_item);

    assert_eq!(
        extracted
            .attributes
            .iter()
            .map(|attribute| attribute.text.as_str())
            .collect::<Vec<_>>(),
        vec!["#[pin_project]"]
    );
    assert_eq!(
        extracted.declaration,
        "pub struct AsyncResponseBody(/* private fields */);"
    );
}

#[test]
fn preserves_argument_bearing_pin_project_attributes() {
    let struct_id = Id(1);
    let field_id = Id(2);
    let mut field = item(
        field_id,
        Some("iter"),
        ItemEnum::StructField(Type::ResolvedPath(path("PageIterator<P>", 3))),
    )
    .with_attrs(vec!["#[pin]".to_string()]);
    field.visibility = Visibility::Default;
    let krate = crate_with_items(vec![
        item(
            struct_id,
            Some("ItemIterator"),
            ItemEnum::Struct(Struct {
                kind: StructKind::Plain {
                    fields: vec![field_id],
                    has_stripped_fields: false,
                },
                generics: Generics {
                    params: vec![type_param("P")],
                    where_predicates: Vec::new(),
                },
                impls: Vec::new(),
            }),
        )
        .with_attrs(vec![
            "#[pin_project::pin_project(project = ItemIteratorProjection, project_replace = ItemIteratorProjectionOwned)]"
                .to_string(),
        ]),
        field,
    ]);

    let struct_item = krate.index.get(&struct_id).expect("struct item present");
    let extracted = extract_item(&krate, struct_item);

    assert_eq!(
        extracted
            .attributes
            .iter()
            .map(|attribute| attribute.text.as_str())
            .collect::<Vec<_>>(),
        vec!["#[pin_project(project = ItemIteratorProjection, project_replace = ItemIteratorProjectionOwned)]"]
    );
    assert_eq!(extracted.declaration, "pub struct ItemIterator<P> {");
    assert_eq!(extracted.members.len(), 1);
    assert_eq!(extracted.members[0].declaration, "iter: PageIterator<P>,");
    assert_eq!(
        extracted.members[0]
            .attributes
            .iter()
            .map(|attribute| attribute.text.as_str())
            .collect::<Vec<_>>(),
        vec!["#[pin]"]
    );
}

#[test]
fn normalize_attribute_removes_path_separator_spacing() {
    assert_eq!(
        normalize_attribute(
            "#[allow(elided_named_lifetimes, clippy\n:: shadow_same, clippy :: type_complexity)]"
        ),
        "#[allow(elided_named_lifetimes, clippy::shadow_same, clippy::type_complexity)]"
    );
}

#[test]
fn extracts_inherent_impl_blocks_for_enum_methods() {
    let enum_id = Id(1);
    let impl_id = Id(2);
    let func_id = Id(3);

    let krate = crate_with_items(vec![
        item(
            enum_id,
            Some("Status"),
            ItemEnum::Enum(RustdocEnum {
                generics: empty_generics(),
                has_stripped_variants: false,
                variants: Vec::new(),
                impls: vec![impl_id],
            }),
        ),
        item(
            impl_id,
            None,
            ItemEnum::Impl(Impl {
                is_unsafe: false,
                generics: empty_generics(),
                provided_trait_methods: Vec::new(),
                trait_: None,
                for_: Type::ResolvedPath(path("Status", enum_id.0)),
                items: vec![func_id],
                is_negative: false,
                is_synthetic: false,
                blanket_impl: None,
            }),
        ),
        item(
            func_id,
            Some("is_ready"),
            ItemEnum::Function(Function {
                sig: FunctionSignature {
                    inputs: vec![(
                        "self".to_string(),
                        Type::BorrowedRef {
                            lifetime: None,
                            is_mutable: false,
                            type_: Box::new(Type::Generic("Self".to_string())),
                        },
                    )],
                    output: Some(Type::Primitive("bool".to_string())),
                    is_c_variadic: false,
                },
                generics: empty_generics(),
                header: FunctionHeader {
                    is_const: false,
                    is_unsafe: false,
                    is_async: false,
                    abi: Abi::Rust,
                },
                has_body: true,
            }),
        ),
    ]);

    let enum_item = krate.index.get(&enum_id).expect("enum item present");
    let extracted = inherent_impls_for_item(&krate, enum_item);

    assert_eq!(extracted.len(), 1);
    assert_eq!(extracted[0].kind, ApiItemKind::InherentImpl);
    assert_eq!(extracted[0].declaration, "impl Status {");
    assert_eq!(
        extracted[0].members[0].declaration,
        "fn is_ready(&self) -> bool;"
    );
}

#[test]
fn keeps_source_distinct_generic_inherent_impl_blocks_separate() {
    let struct_id = Id(1);
    let impl_one_id = Id(2);
    let impl_two_id = Id(3);
    let fn_one_id = Id(4);
    let fn_two_id = Id(5);

    let model = extract_model(
        &package_metadata("demo"),
        &crate_with_items(vec![
            item(
                struct_id,
                Some("Foo"),
                ItemEnum::Struct(Struct {
                    kind: StructKind::Unit,
                    generics: empty_generics(),
                    impls: vec![impl_one_id, impl_two_id],
                }),
            ),
            impl_item_for_type_with_items(
                impl_one_id,
                Type::ResolvedPath(path("Foo", struct_id.0).with_args(
                    GenericArgs::AngleBracketed {
                        args: vec![GenericArg::Type(Type::Generic("T".to_string()))],
                        constraints: Vec::new(),
                    },
                )),
                Generics {
                    params: vec![type_param("T")],
                    where_predicates: Vec::new(),
                },
                vec![fn_one_id],
            )
            .with_docs("first impl docs")
            .with_attrs(vec!["#[cfg(feature = \"one\")]".to_string()]),
            impl_item_for_type_with_items(
                impl_two_id,
                Type::ResolvedPath(path("Foo", struct_id.0).with_args(
                    GenericArgs::AngleBracketed {
                        args: vec![GenericArg::Type(Type::Generic("T".to_string()))],
                        constraints: Vec::new(),
                    },
                )),
                Generics {
                    params: vec![type_param("T")],
                    where_predicates: Vec::new(),
                },
                vec![fn_two_id],
            )
            .with_docs("second impl docs")
            .with_attrs(vec!["#[must_use]".to_string()]),
            inherent_method(fn_one_id, "one"),
            inherent_method(fn_two_id, "two"),
        ]),
        &mut NoopResolver,
    )
    .expect("model extraction should succeed");

    let impls = model
        .root_module
        .items
        .iter()
        .filter(|item| item.kind == ApiItemKind::InherentImpl)
        .collect::<Vec<_>>();

    assert_eq!(impls.len(), 2);
    assert_eq!(
        impls
            .iter()
            .map(|item| item.declaration.as_str())
            .collect::<Vec<_>>(),
        vec!["impl<T> Foo<T> {", "impl<T> Foo<T> {"]
    );
    assert_eq!(
        impls[0]
            .doc_comments
            .iter()
            .map(String::as_str)
            .collect::<Vec<_>>(),
        vec!["/// first impl docs"]
    );
    assert_eq!(
        impls[0]
            .attributes
            .iter()
            .map(|attribute| attribute.text.as_str())
            .collect::<Vec<_>>(),
        vec!["#[cfg(feature = \"one\")]"]
    );
    assert_eq!(
        impls[0]
            .members
            .iter()
            .map(|member| member.name.as_str())
            .collect::<Vec<_>>(),
        vec!["one"]
    );
    assert_eq!(
        impls[1]
            .doc_comments
            .iter()
            .map(String::as_str)
            .collect::<Vec<_>>(),
        vec!["/// second impl docs"]
    );
    assert_eq!(
        impls[1]
            .attributes
            .iter()
            .map(|attribute| attribute.text.as_str())
            .collect::<Vec<_>>(),
        vec!["#[must_use]"]
    );
    assert_eq!(
        impls[1]
            .members
            .iter()
            .map(|member| member.name.as_str())
            .collect::<Vec<_>>(),
        vec!["two"]
    );
}

#[test]
fn keeps_source_distinct_explicit_inherent_impl_blocks_separate() {
    let struct_id = Id(1);
    let impl_one_id = Id(2);
    let impl_two_id = Id(3);
    let fn_one_id = Id(4);
    let fn_two_id = Id(5);

    let explicit_type = Type::ResolvedPath(path("Foo", struct_id.0).with_args(
        GenericArgs::AngleBracketed {
            args: vec![GenericArg::Type(Type::ResolvedPath(path("BlobState", 50)))],
            constraints: Vec::new(),
        },
    ));

    let model = extract_model(
        &package_metadata("demo"),
        &crate_with_items(vec![
            item(
                struct_id,
                Some("Foo"),
                ItemEnum::Struct(Struct {
                    kind: StructKind::Unit,
                    generics: empty_generics(),
                    impls: vec![impl_one_id, impl_two_id],
                }),
            ),
            impl_item_for_type_with_items(
                impl_one_id,
                explicit_type.clone(),
                empty_generics(),
                vec![fn_one_id],
            ),
            impl_item_for_type_with_items(
                impl_two_id,
                explicit_type,
                empty_generics(),
                vec![fn_two_id],
            ),
            inherent_method(fn_one_id, "one"),
            inherent_method(fn_two_id, "two"),
        ]),
        &mut NoopResolver,
    )
    .expect("model extraction should succeed");

    let impls = model
        .root_module
        .items
        .iter()
        .filter(|item| item.kind == ApiItemKind::InherentImpl)
        .collect::<Vec<_>>();

    assert_eq!(impls.len(), 2);
    assert_eq!(
        impls
            .iter()
            .map(|item| item.declaration.as_str())
            .collect::<Vec<_>>(),
        vec!["impl Foo<BlobState> {", "impl Foo<BlobState> {"]
    );
    assert_eq!(
        impls[0]
            .members
            .iter()
            .map(|member| member.name.as_str())
            .collect::<Vec<_>>(),
        vec!["one"]
    );
    assert_eq!(
        impls[1]
            .members
            .iter()
            .map(|member| member.name.as_str())
            .collect::<Vec<_>>(),
        vec!["two"]
    );
}

#[test]
fn keeps_divergent_typestate_inherent_impl_blocks_separate() {
    let struct_id = Id(1);
    let generic_impl_id = Id(2);
    let explicit_impl_id = Id(3);
    let generic_read_id = Id(4);
    let explicit_read_id = Id(5);

    let model = extract_model(
        &package_metadata("demo"),
        &crate_with_items(vec![
            item(
                struct_id,
                Some("Foo"),
                ItemEnum::Struct(Struct {
                    kind: StructKind::Unit,
                    generics: empty_generics(),
                    impls: vec![generic_impl_id, explicit_impl_id],
                }),
            ),
            impl_item_for_type_with_items(
                generic_impl_id,
                Type::ResolvedPath(path("Foo", struct_id.0).with_args(
                    GenericArgs::AngleBracketed {
                        args: vec![GenericArg::Type(Type::Generic("T".to_string()))],
                        constraints: Vec::new(),
                    },
                )),
                Generics {
                    params: vec![type_param("T")],
                    where_predicates: Vec::new(),
                },
                vec![generic_read_id],
            ),
            impl_item_for_type_with_items(
                explicit_impl_id,
                Type::ResolvedPath(path("Foo", struct_id.0).with_args(
                    GenericArgs::AngleBracketed {
                        args: vec![GenericArg::Type(Type::ResolvedPath(path("BlobState", 50)))],
                        constraints: Vec::new(),
                    },
                )),
                empty_generics(),
                vec![explicit_read_id],
            ),
            inherent_method(generic_read_id, "read"),
            inherent_method(explicit_read_id, "read"),
        ]),
        &mut NoopResolver,
    )
    .expect("model extraction should succeed");

    let impls = model
        .root_module
        .items
        .iter()
        .filter(|item| item.kind == ApiItemKind::InherentImpl)
        .collect::<Vec<_>>();

    assert_eq!(impls.len(), 2);
    assert_eq!(
        impls
            .iter()
            .map(|item| item.declaration.as_str())
            .collect::<Vec<_>>(),
        vec!["impl<T> Foo<T> {", "impl Foo<BlobState> {"]
    );
    assert!(impls.iter().all(|item| {
        item.members
            .iter()
            .map(|member| member.name.as_str())
            .collect::<Vec<_>>()
            == vec!["read"]
    }));
}

#[test]
fn local_reexport_keeps_source_distinct_inherent_impl_blocks_separate() {
    let hidden_module_id = Id(1);
    let struct_id = Id(2);
    let impl_one_id = Id(3);
    let impl_two_id = Id(4);
    let fn_one_id = Id(5);
    let fn_two_id = Id(6);
    let reexport_id = Id(7);

    let model = extract_model(
        &package_metadata("demo"),
        &crate_with_root_items(
            vec![hidden_module_id, reexport_id],
            vec![
                module_item(
                    hidden_module_id,
                    "hidden",
                    vec![struct_id, impl_one_id, impl_two_id],
                    true,
                ),
                item(
                    struct_id,
                    Some("Foo"),
                    ItemEnum::Struct(Struct {
                        kind: StructKind::Unit,
                        generics: empty_generics(),
                        impls: vec![impl_one_id, impl_two_id],
                    }),
                ),
                impl_item_for_type_with_items(
                    impl_one_id,
                    Type::ResolvedPath(path("Foo", struct_id.0).with_args(
                        GenericArgs::AngleBracketed {
                            args: vec![GenericArg::Type(Type::Generic("T".to_string()))],
                            constraints: Vec::new(),
                        },
                    )),
                    Generics {
                        params: vec![type_param("T")],
                        where_predicates: Vec::new(),
                    },
                    vec![fn_one_id],
                ),
                impl_item_for_type_with_items(
                    impl_two_id,
                    Type::ResolvedPath(path("Foo", struct_id.0).with_args(
                        GenericArgs::AngleBracketed {
                            args: vec![GenericArg::Type(Type::Generic("T".to_string()))],
                            constraints: Vec::new(),
                        },
                    )),
                    Generics {
                        params: vec![type_param("T")],
                        where_predicates: Vec::new(),
                    },
                    vec![fn_two_id],
                ),
                inherent_method(fn_one_id, "one"),
                inherent_method(fn_two_id, "two"),
                item(
                    reexport_id,
                    Some("Foo"),
                    ItemEnum::Use(rustdoc_types::Use {
                        source: "crate::hidden::Foo".to_string(),
                        name: "Foo".to_string(),
                        id: Some(struct_id),
                        is_glob: false,
                    }),
                ),
            ],
        ),
        &mut NoopResolver,
    )
    .expect("model extraction should succeed");

    let impls = model
        .root_module
        .items
        .iter()
        .filter(|item| item.kind == ApiItemKind::InherentImpl)
        .collect::<Vec<_>>();

    assert_eq!(impls.len(), 2);
    assert_eq!(
        impls
            .iter()
            .map(|item| item.declaration.as_str())
            .collect::<Vec<_>>(),
        vec!["impl<T> Foo<T> {", "impl<T> Foo<T> {"]
    );
    assert_eq!(
        impls[0]
            .members
            .iter()
            .map(|member| member.name.as_str())
            .collect::<Vec<_>>(),
        vec!["one"]
    );
    assert_eq!(
        impls[1]
            .members
            .iter()
            .map(|member| member.name.as_str())
            .collect::<Vec<_>>(),
        vec!["two"]
    );
}

#[test]
fn sorts_inferred_type_arguments_after_generic_type_parameters() {
    let struct_id = Id(1);
    let generic_impl_id = Id(2);
    let inferred_impl_id = Id(3);
    let explicit_impl_id = Id(4);
    let generic_read_id = Id(5);
    let inferred_read_id = Id(6);
    let explicit_read_id = Id(7);

    let model = extract_model(
        &package_metadata("demo"),
        &crate_with_items(vec![
            item(
                struct_id,
                Some("Builder"),
                ItemEnum::Struct(Struct {
                    kind: StructKind::Unit,
                    generics: empty_generics(),
                    impls: vec![explicit_impl_id, inferred_impl_id, generic_impl_id],
                }),
            ),
            impl_item_for_type_with_items(
                generic_impl_id,
                Type::ResolvedPath(path("Builder", struct_id.0).with_args(
                    GenericArgs::AngleBracketed {
                        args: vec![GenericArg::Type(Type::Generic("S".to_string()))],
                        constraints: Vec::new(),
                    },
                )),
                Generics {
                    params: vec![type_param("S")],
                    where_predicates: Vec::new(),
                },
                vec![generic_read_id],
            ),
            impl_item_for_type_with_items(
                inferred_impl_id,
                Type::ResolvedPath(path("Builder", struct_id.0).with_args(
                    GenericArgs::AngleBracketed {
                        args: vec![GenericArg::Infer],
                        constraints: Vec::new(),
                    },
                )),
                empty_generics(),
                vec![inferred_read_id],
            ),
            impl_item_for_type_with_items(
                explicit_impl_id,
                Type::ResolvedPath(path("Builder", struct_id.0).with_args(
                    GenericArgs::AngleBracketed {
                        args: vec![GenericArg::Type(Type::ResolvedPath(path("BlobState", 50)))],
                        constraints: Vec::new(),
                    },
                )),
                empty_generics(),
                vec![explicit_read_id],
            ),
            inherent_method(generic_read_id, "generic"),
            inherent_method(inferred_read_id, "inferred"),
            inherent_method(explicit_read_id, "explicit"),
        ]),
        &mut NoopResolver,
    )
    .expect("model extraction should succeed");

    let impls = model
        .root_module
        .sorted_items()
        .into_iter()
        .filter(|item| item.kind == ApiItemKind::InherentImpl)
        .map(|item| item.declaration.as_str())
        .collect::<Vec<_>>();

    assert_eq!(
        impls,
        vec![
            "impl<S> Builder<S> {",
            "impl Builder<_> {",
            "impl Builder<BlobState> {",
        ]
    );
}

#[test]
fn synthesize_derive_attribute_for_enum() {
    let enum_id = Id(1);
    let clone_impl_id = Id(2);
    let debug_impl_id = Id(3);

    let krate = crate_with_items(vec![
        item(
            enum_id,
            Some("Kind"),
            ItemEnum::Enum(RustdocEnum {
                generics: empty_generics(),
                has_stripped_variants: false,
                variants: Vec::new(),
                impls: vec![clone_impl_id, debug_impl_id],
            }),
        ),
        impl_item(
            clone_impl_id,
            Some(path("Clone", 10)),
            "Kind",
            enum_id,
            true,
        ),
        impl_item(
            debug_impl_id,
            Some(path("Debug", 11)),
            "Kind",
            enum_id,
            true,
        ),
    ]);

    let enum_item = krate.index.get(&enum_id).expect("enum item present");
    let attribute = synthesize_derive_attribute(&krate, enum_item)
        .expect("derive attribute should be synthesized for enum");

    assert_eq!(attribute.text, "#[derive(Clone, Debug)]");
}

#[test]
fn extracts_assoc_const_member_from_trait() {
    let trait_id = Id(1);
    let const_id = Id(2);

    let krate = crate_with_items(vec![
        item(
            trait_id,
            Some("Configurable"),
            ItemEnum::Trait(Trait {
                is_auto: false,
                is_unsafe: false,
                is_dyn_compatible: true,
                items: vec![const_id],
                generics: empty_generics(),
                bounds: Vec::new(),
                implementations: Vec::new(),
            }),
        ),
        item(
            const_id,
            Some("MAX"),
            ItemEnum::AssocConst {
                type_: Type::Primitive("u32".to_string()),
                value: None,
            },
        ),
    ]);

    let trait_item = krate.index.get(&trait_id).expect("trait item present");
    let extracted = extract_item(&krate, trait_item);

    assert_eq!(extracted.members.len(), 1);
    assert_eq!(extracted.members[0].declaration, "const MAX: u32;");
}

#[test]
fn extracts_assoc_type_member_from_trait() {
    let trait_id = Id(1);
    let type_id = Id(2);

    let krate = crate_with_items(vec![
        item(
            trait_id,
            Some("IntoIter"),
            ItemEnum::Trait(Trait {
                is_auto: false,
                is_unsafe: false,
                is_dyn_compatible: true,
                items: vec![type_id],
                generics: empty_generics(),
                bounds: Vec::new(),
                implementations: Vec::new(),
            }),
        ),
        item(
            type_id,
            Some("Item"),
            ItemEnum::AssocType {
                generics: empty_generics(),
                bounds: Vec::new(),
                type_: None,
            },
        ),
    ]);

    let trait_item = krate.index.get(&trait_id).expect("trait item present");
    let extracted = extract_item(&krate, trait_item);

    assert_eq!(extracted.members.len(), 1);
    assert_eq!(extracted.members[0].declaration, "type Item;");
}

#[test]
fn extracts_plain_struct_fields_as_members() {
    let struct_id = Id(1);
    let field_id = Id(2);
    let krate = crate_with_items(vec![
        item(
            struct_id,
            Some("Blob"),
            ItemEnum::Struct(Struct {
                kind: StructKind::Plain {
                    fields: vec![field_id],
                    has_stripped_fields: false,
                },
                generics: empty_generics(),
                impls: Vec::new(),
            }),
        ),
        item(
            field_id,
            Some("body"),
            ItemEnum::StructField(Type::ResolvedPath(path("bytes::Bytes", 3))),
        ),
    ]);

    let struct_item = krate.index.get(&struct_id).expect("struct item present");
    let extracted = extract_item(&krate, struct_item);

    assert_eq!(extracted.declaration, "pub struct Blob {");
    assert_eq!(extracted.members.len(), 1);
    assert_eq!(extracted.members[0].kind, ApiMemberKind::Field);
    assert_eq!(extracted.members[0].declaration, "pub body: bytes::Bytes,");
}

#[test]
fn extracts_enum_variants_as_members() {
    let enum_id = Id(1);
    let variant_id = Id(2);
    let field_id = Id(3);
    let krate = crate_with_items(vec![
        item(
            enum_id,
            Some("Kind"),
            ItemEnum::Enum(RustdocEnum {
                variants: vec![variant_id],
                generics: empty_generics(),
                has_stripped_variants: false,
                impls: Vec::new(),
            }),
        ),
        item(
            variant_id,
            Some("Block"),
            ItemEnum::Variant(Variant {
                kind: VariantKind::Tuple(vec![Some(field_id)]),
                discriminant: None,
            }),
        ),
        item(
            field_id,
            None,
            ItemEnum::StructField(Type::ResolvedPath(path("bytes::Bytes", 4))),
        ),
    ]);

    let enum_item = krate.index.get(&enum_id).expect("enum item present");
    let extracted = extract_item(&krate, enum_item);

    assert_eq!(extracted.declaration, "pub enum Kind {");
    assert_eq!(extracted.members.len(), 1);
    assert_eq!(extracted.members[0].kind, ApiMemberKind::Variant);
    assert_eq!(extracted.members[0].declaration, "Block(bytes::Bytes),");
}

#[test]
fn extracts_derive_macro_helpers_as_members() {
    let macro_id = Id(1);
    let krate = crate_with_items(vec![item(
        macro_id,
        Some("BlobDerive"),
        ItemEnum::ProcMacro(rustdoc_types::ProcMacro {
            kind: rustdoc_types::MacroKind::Derive,
            helpers: vec!["blob".to_string()],
        }),
    )]);

    let macro_item = krate.index.get(&macro_id).expect("macro item present");
    let extracted = extract_item(&krate, macro_item);

    assert_eq!(extracted.declaration, "#[derive(BlobDerive)] {");
    assert_eq!(
        extracted
            .members
            .iter()
            .map(|member| (member.kind, member.declaration.as_str()))
            .collect::<Vec<_>>(),
        vec![
            (
                ApiMemberKind::Text,
                "// Attributes available to this derive:"
            ),
            (ApiMemberKind::MacroInput, "#[blob]"),
        ]
    );
}

#[test]
fn locates_derive_macro_helpers_in_long_attributes() {
    let path = std::path::PathBuf::from(format!(
        "generate_api_proc_macro_source_test_{}.rs",
        std::process::id()
    ));
    let source = format!(
        "#[proc_macro_derive(\n    BlobDerive,\n    attributes(\n        blob,\n{}\n    )\n)]\npub fn derive() {{}}\n",
        "\n".repeat(20)
    );
    std::fs::write(&path, source).unwrap();

    let mut macro_item = item(
        Id(1),
        Some("BlobDerive"),
        ItemEnum::ProcMacro(rustdoc_types::ProcMacro {
            kind: rustdoc_types::MacroKind::Derive,
            helpers: vec!["blob".to_string()],
        }),
    );
    macro_item.span = Some(rustdoc_types::Span {
        filename: path.clone(),
        begin: (28, 1),
        end: (28, 19),
    });

    let location = proc_macro_helper_location(&macro_item, "blob");

    std::fs::remove_file(path).unwrap();
    assert_eq!(
        location,
        Some(crate::model::SourceLocation {
            path: macro_item
                .span
                .as_ref()
                .unwrap()
                .filename
                .to_string_lossy()
                .to_string(),
            line: 3,
            column: 8,
        })
    );
}

#[test]
fn extracts_macro_matcher_arms_as_members() {
    let macro_id = Id(1);
    let krate = crate_with_items(vec![item(
        macro_id,
        Some("request_header"),
        ItemEnum::Macro(
            r#"macro_rules! request_header {
    ($(#[$outer:meta])* $name:ident, $header:ident) => {
        $crate::request_header!($name, $header,);
    };
    ($(#[$outer:meta])* $name:ident, $header:ident, $(($(#[$inner:meta])*$variant:ident, $value:expr)), *) => {
        $crate::request_option!($(#[$outer])* $name);
    };
}"#
            .to_string(),
        ),
    )]);

    let macro_item = krate.index.get(&macro_id).expect("macro item present");
    let extracted = extract_item(&krate, macro_item);

    assert_eq!(extracted.declaration, "macro_rules! request_header {");
    assert_eq!(
        extracted
            .members
            .iter()
            .map(|member| (member.kind, member.declaration.as_str()))
            .collect::<Vec<_>>(),
        vec![
            (
                ApiMemberKind::MacroInput,
                "($(#[$outer:meta])* $name:ident, $header:ident) => { ... };",
            ),
            (
                ApiMemberKind::MacroInput,
                "($(#[$outer:meta])* $name:ident, $header:ident, $(($(#[$inner:meta])*$variant:ident, $value:expr)), *) => { ... };",
            ),
        ]
    );
}

#[test]
fn preserves_macro_arm_literal_whitespace() {
    let macro_id = Id(1);
    let krate = crate_with_items(vec![item(
        macro_id,
        Some("literal_spaces"),
        ItemEnum::Macro(
            r#"macro_rules! literal_spaces {
    ("a  b") => "x  y";
}"#
            .to_string(),
        ),
    )]);

    let macro_item = krate.index.get(&macro_id).expect("macro item present");
    let extracted = extract_item(&krate, macro_item);

    assert_eq!(extracted.members[0].declaration, r#"("a  b") => "x  y";"#);
}

#[test]
fn collects_function_where_clause_references_after_signature_types() {
    let function_id = Id(1);

    let model = extract_model(
        &package_metadata("demo"),
        &crate_with_items(vec![item(
            function_id,
            Some("parse"),
            ItemEnum::Function(Function {
                sig: FunctionSignature {
                    inputs: vec![("value".to_string(), Type::ResolvedPath(path("Input", 10)))],
                    output: Some(Type::ResolvedPath(path("Output", 11))),
                    is_c_variadic: false,
                },
                generics: Generics {
                    params: vec![type_param("T")],
                    where_predicates: vec![bound_predicate(
                        Type::Generic("T".to_string()),
                        vec![trait_bound("Bound", 12)],
                    )],
                },
                header: FunctionHeader {
                    is_const: false,
                    is_unsafe: false,
                    is_async: false,
                    abi: Abi::Rust,
                },
                has_body: false,
            }),
        )]),
        &mut NoopResolver,
    )
    .expect("model extraction should succeed");

    let function = model
        .root_module
        .items
        .iter()
        .find(|item| item.name == "parse")
        .expect("function should be extracted");

    assert_eq!(
        function
            .declaration_path_references
            .iter()
            .map(|reference| reference.path.as_str())
            .collect::<Vec<_>>(),
        vec!["Input", "Output", "Bound"]
    );
}

#[test]
fn collects_trait_impl_where_clause_references_last() {
    let impl_id = Id(2);
    let struct_id = Id(3);

    let trait_impl = trait_impl_item_for_type(
        impl_id,
        path("Service", 20).with_args(GenericArgs::AngleBracketed {
            args: vec![GenericArg::Type(Type::ResolvedPath(path("Request", 21)))],
            constraints: Vec::new(),
        }),
        Type::ResolvedPath(path("Client", struct_id.0)),
        Generics {
            params: vec![type_param("T")],
            where_predicates: vec![bound_predicate(
                Type::Generic("T".to_string()),
                vec![trait_bound("Bound", 22)],
            )],
        },
    );

    let ItemEnum::Impl(impl_block) = &trait_impl.inner else {
        panic!("expected impl item");
    };

    assert_eq!(
        collect_trait_impl_declaration_path_references(
            &crate_with_items(vec![
                item(
                    struct_id,
                    Some("Client"),
                    ItemEnum::Struct(Struct {
                        kind: StructKind::Unit,
                        generics: empty_generics(),
                        impls: vec![impl_id],
                    }),
                ),
                trait_impl.clone()
            ]),
            impl_block,
        )
        .iter()
        .map(|reference| reference.path.as_str())
        .collect::<Vec<_>>(),
        vec!["Service", "Request", "Client", "Bound"]
    );
}

fn crate_with_items(items: Vec<Item>) -> Crate {
    let module_items = items.iter().map(|item| item.id).collect::<Vec<_>>();
    crate_with_root_items(module_items, items)
}

fn crate_with_root_items(root_items: Vec<Id>, items: Vec<Item>) -> Crate {
    let root = Id(0);
    let mut index = HashMap::new();
    index.insert(
        root,
        item(
            root,
            Some("crate"),
            ItemEnum::Module(Module {
                is_crate: true,
                items: root_items,
                is_stripped: false,
            }),
        ),
    );
    index.extend(items.into_iter().map(|item| (item.id, item)));

    Crate {
        root,
        crate_version: None,
        includes_private: false,
        index,
        paths: HashMap::<Id, ItemSummary>::new(),
        external_crates: HashMap::new(),
        target: Target {
            triple: "x86_64-unknown-linux-gnu".to_string(),
            target_features: Vec::new(),
        },
        format_version: 0,
    }
}

fn item(id: Id, name: Option<&str>, inner: ItemEnum) -> Item {
    Item {
        id,
        crate_id: 0,
        name: name.map(str::to_string),
        span: None,
        visibility: Visibility::Public,
        docs: None,
        links: HashMap::new(),
        attrs: Vec::new(),
        deprecation: None,
        inner,
    }
}

fn impl_item(
    id: Id,
    trait_path: Option<Path>,
    self_type_name: &str,
    struct_id: Id,
    automatically_derived: bool,
) -> Item {
    impl_item_with_items(
        id,
        trait_path,
        self_type_name,
        struct_id,
        automatically_derived,
        Vec::new(),
    )
}

fn impl_item_with_items(
    id: Id,
    trait_path: Option<Path>,
    self_type_name: &str,
    struct_id: Id,
    automatically_derived: bool,
    items: Vec<Id>,
) -> Item {
    item(
        id,
        None,
        ItemEnum::Impl(Impl {
            is_unsafe: false,
            generics: empty_generics(),
            provided_trait_methods: Vec::new(),
            trait_: trait_path,
            for_: Type::ResolvedPath(path(self_type_name, struct_id.0)),
            items,
            is_negative: false,
            is_synthetic: false,
            blanket_impl: None,
        }),
    )
    .with_attrs(if automatically_derived {
        vec!["#[automatically_derived]".to_string()]
    } else {
        Vec::new()
    })
}

fn impl_item_for_type_with_items(
    id: Id,
    for_type: Type,
    generics: Generics,
    items: Vec<Id>,
) -> Item {
    item(
        id,
        None,
        ItemEnum::Impl(Impl {
            is_unsafe: false,
            generics,
            provided_trait_methods: Vec::new(),
            trait_: None,
            for_: for_type,
            items,
            is_negative: false,
            is_synthetic: false,
            blanket_impl: None,
        }),
    )
}

fn trait_impl_item_for_type(id: Id, trait_path: Path, for_type: Type, generics: Generics) -> Item {
    item(
        id,
        None,
        ItemEnum::Impl(Impl {
            is_unsafe: false,
            generics,
            provided_trait_methods: Vec::new(),
            trait_: Some(trait_path),
            for_: for_type,
            items: Vec::new(),
            is_negative: false,
            is_synthetic: false,
            blanket_impl: None,
        }),
    )
}

fn inherent_method(id: Id, name: &str) -> Item {
    item(
        id,
        Some(name),
        ItemEnum::Function(Function {
            sig: FunctionSignature {
                inputs: Vec::new(),
                output: None,
                is_c_variadic: false,
            },
            generics: empty_generics(),
            header: FunctionHeader {
                is_const: false,
                is_unsafe: false,
                is_async: false,
                abi: Abi::Rust,
            },
            has_body: true,
        }),
    )
}

fn module_item(id: Id, name: &str, items: Vec<Id>, is_stripped: bool) -> Item {
    item(
        id,
        Some(name),
        ItemEnum::Module(Module {
            is_crate: false,
            items,
            is_stripped,
        }),
    )
}

fn path(path: &str, id: u32) -> Path {
    Path {
        path: path.to_string(),
        id: Id(id),
        args: None,
    }
}

trait PathTestExt {
    fn with_args(self, args: GenericArgs) -> Self;
}

impl PathTestExt for Path {
    fn with_args(mut self, args: GenericArgs) -> Self {
        self.args = Some(Box::new(args));
        self
    }
}

fn lifetime_param(name: &str) -> GenericParamDef {
    GenericParamDef {
        name: name.to_string(),
        kind: GenericParamDefKind::Lifetime {
            outlives: Vec::new(),
        },
    }
}

fn type_param(name: &str) -> GenericParamDef {
    GenericParamDef {
        name: name.to_string(),
        kind: GenericParamDefKind::Type {
            bounds: Vec::new(),
            default: None,
            is_synthetic: false,
        },
    }
}

fn bound_predicate(type_: Type, bounds: Vec<GenericBound>) -> WherePredicate {
    WherePredicate::BoundPredicate {
        type_,
        bounds,
        generic_params: Vec::new(),
    }
}

fn trait_bound(name: &str, id: u32) -> GenericBound {
    GenericBound::TraitBound {
        trait_: path(name, id),
        generic_params: Vec::new(),
        modifier: rustdoc_types::TraitBoundModifier::None,
    }
}

fn pin_project_generated_unpin_predicate(projected_type: Type) -> WherePredicate {
    bound_predicate(
        Type::ResolvedPath(
            path("_pin_project::__private::PinnedFieldsOf", 900).with_args(
                GenericArgs::AngleBracketed {
                    args: vec![GenericArg::Type(projected_type)],
                    constraints: Vec::new(),
                },
            ),
        ),
        vec![trait_bound("_pin_project::__private::Unpin", 901)],
    )
}

fn empty_generics() -> Generics {
    Generics {
        params: Vec::new(),
        where_predicates: Vec::new(),
    }
}

fn package_metadata(name: &str) -> PackageMetadata {
    PackageMetadata {
        name: name.to_string(),
        version: "1.0.0".to_string(),
        manifest_path: std::path::PathBuf::from("Cargo.toml"),
        has_library_target: true,
        api: Default::default(),
    }
}

struct NoopResolver;

impl WorkspaceResolver for NoopResolver {
    fn is_workspace_crate(&self, _crate_name: &str) -> bool {
        false
    }

    fn load_workspace_model(&mut self, _crate_name: &str) -> Result<Option<Arc<ApiModel>>, String> {
        Ok(None)
    }

    fn load_workspace_crate(&mut self, _crate_name: &str) -> Result<Option<Arc<Crate>>, String> {
        Ok(None)
    }
}

trait ItemTestExt {
    fn with_docs(self, docs: &str) -> Self;
    fn with_attrs(self, attrs: Vec<String>) -> Self;
}

impl ItemTestExt for Item {
    fn with_docs(mut self, docs: &str) -> Self {
        self.docs = Some(docs.to_string());
        self
    }

    fn with_attrs(mut self, attrs: Vec<String>) -> Self {
        self.attrs = attrs
            .into_iter()
            .map(|attribute| {
                if attribute == "#[automatically_derived]" {
                    rustdoc_types::Attribute::AutomaticallyDerived
                } else {
                    rustdoc_types::Attribute::Other(attribute)
                }
            })
            .collect();
        self
    }
}

#[test]
fn normalizes_repo_relative_source_paths() {
    assert_eq!(
        repo_relative_source_path(std::path::Path::new("sdk/core/typespec/src/lib.rs")).as_deref(),
        Some("sdk/core/typespec/src/lib.rs")
    );
    assert_eq!(
        repo_relative_source_path(
            &std::env::current_dir()
                .unwrap()
                .join("sdk/core/typespec/src/lib.rs")
        )
        .as_deref(),
        Some("sdk/core/typespec/src/lib.rs")
    );
    assert_eq!(
        repo_relative_source_path(std::path::Path::new("../outside.rs")),
        None
    );
}

#[test]
fn finds_out_of_line_module_declarations() {
    let source = r#"
#[path = "other.rs"]
pub(crate) mod other;
/// `mod ignored;` is documentation, not a declaration.
pub mod cloud;
"#;

    assert_eq!(
        find_module_declaration(source, "other", 0, 5, Some("other.rs".as_ref()), false),
        Some((2, 0))
    );
    assert_eq!(
        find_module_declaration(source, "cloud", 0, 5, Some("cloud.rs".as_ref()), true),
        Some((4, 0))
    );
    assert_eq!(
        find_module_declaration(source, "ignored", 0, 5, None, true),
        None
    );
}

#[test]
fn limits_module_declaration_search_to_the_parent_span() {
    let source =
        "pub mod first {\n    pub mod shared;\n}\npub mod second {\n    pub mod shared;\n}";

    assert_eq!(
        find_module_declaration(source, "shared", 0, 3, Some("shared.rs".as_ref()), true),
        Some((1, 4))
    );
    assert_eq!(
        find_module_declaration(source, "shared", 3, 6, Some("shared.rs".as_ref()), true),
        Some((4, 4))
    );
}

#[test]
fn uses_path_attributes_to_disambiguate_module_declarations() {
    let source =
        "#[path = \"unix/shared.rs\"]\nmod shared;\n#[path = \"windows/shared.rs\"]\nmod shared;";

    assert_eq!(
        find_module_declaration(
            source,
            "shared",
            0,
            4,
            Some("src/windows/shared.rs".as_ref()),
            false
        ),
        Some((3, 0))
    );
}

#[test]
fn uses_visibility_to_disambiguate_cfg_module_declarations() {
    let source = "#[cfg(feature = \"testing\")]\npub mod query;\n#[cfg(not(feature = \"testing\"))]\npub(crate) mod query;";

    assert_eq!(
        find_module_declaration(source, "query", 0, 4, Some("query.rs".as_ref()), true),
        Some((1, 0))
    );
}

#[test]
fn locates_macro_arms_relative_to_the_item_span() {
    let source = "macro_rules! demo {\n    ($value:expr) => { $value };\n}";
    let parsed = parse_macro_definition(source).unwrap();
    let offset = parsed.members[0].0;
    assert!(source[offset..].starts_with("($value:expr)"));

    let mut macro_item = item(Id(1), Some("demo"), ItemEnum::Macro(source.to_string()));
    macro_item.span = Some(rustdoc_types::Span {
        filename: "sdk/example/src/lib.rs".into(),
        begin: (10, 5),
        end: (12, 2),
    });

    assert_eq!(
        source_location_at_offset(&macro_item, source, offset),
        Some(crate::model::SourceLocation {
            path: "sdk/example/src/lib.rs".to_string(),
            line: 10,
            column: 4,
        })
    );
}

#[test]
fn uses_actual_source_layout_for_macro_arm_locations() {
    let actual = "macro_rules! demo {\n    (first) => {\n        one();\n        two();\n    };\n    (second) => { three() };\n} // outside the span";
    let path = std::path::PathBuf::from(format!(
        "generate_api_macro_source_test_{}.rs",
        std::process::id()
    ));
    std::fs::write(&path, actual).unwrap();
    let mut macro_item = item(
        Id(1),
        Some("demo"),
        ItemEnum::Macro(
            "macro_rules! demo {\n    (first) => { ... };\n    (second) => { ... };\n}".to_string(),
        ),
    );
    macro_item.span = Some(rustdoc_types::Span {
        filename: path.clone(),
        begin: (1, 1),
        end: (7, 2),
    });

    let members = extract_macro_members(
        &macro_item,
        "macro_rules! demo {\n    (first) => { ... };\n    (second) => { ... };\n}",
    );

    std::fs::remove_file(path).unwrap();
    assert_eq!(members[0].declaration_location.as_ref().unwrap().line, 1);
    assert_eq!(members[1].declaration_location.as_ref().unwrap().line, 5);
}

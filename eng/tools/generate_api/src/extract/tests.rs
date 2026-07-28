// Copyright (c) Microsoft Corporation. All rights reserved.
// Licensed under the MIT License.

use super::*;
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
        self.attrs = attrs;
        self
    }
}

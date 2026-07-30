// Copyright (c) Microsoft Corporation. All rights reserved.
// Licensed under the MIT License.

use super::*;
use crate::model::{
    ApiAttribute, ApiItem, ApiItemKind, ApiMember, ApiMemberKind, ApiModel, ApiModule,
    ApiNavigationPath, ApiPathReference, InherentImplSortKey,
};

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

fn path_reference(path: &str, target_source_id: &str) -> ApiPathReference {
    path_reference_with_canonical(path, None, target_source_id)
}

fn path_reference_with_canonical(
    path: &str,
    canonical_path: Option<&str>,
    target_source_id: &str,
) -> ApiPathReference {
    ApiPathReference {
        path: path.to_string(),
        canonical_path: canonical_path.map(str::to_string),
        target_source_id: Some(target_source_id.to_string()),
    }
}

fn navigation_path(path: &str, source_id: &str) -> ApiNavigationPath {
    ApiNavigationPath {
        path: path.to_string(),
        source_id: Some(source_id.to_string()),
    }
}

fn navigation_lookup(root_module: &ApiModule) -> NavigationLookup {
    NavigationLookup::new(&ApiModel {
        package_name: root_module.local_name().to_string(),
        package_version: "1.0.0".to_string(),
        parser_version: "0.0.0".to_string(),
        root_module: root_module.clone(),
    })
}

fn find_token<'a>(line: &'a ReviewLine, value: &str) -> &'a ReviewToken {
    line.tokens
        .iter()
        .find(|token| token.value == value)
        .unwrap_or_else(|| panic!("expected {value} token"))
}

fn find_child<'a>(line: &'a ReviewLine, line_id: &str) -> &'a ReviewLine {
    line.children
        .iter()
        .find(|child| child.line_id.as_deref() == Some(line_id))
        .unwrap_or_else(|| panic!("expected {line_id} child line"))
}

fn top_level_navigation_line_ids(lines: &[ReviewLine]) -> Vec<&str> {
    lines
        .iter()
        .filter(|line| line.related_to_line.is_none())
        .filter(|line| {
            line.tokens
                .iter()
                .any(|token| token.navigation_display_name.is_some())
        })
        .filter_map(|line| line.line_id.as_deref())
        .collect()
}

#[test]
fn renders_trait_impl_tokens_with_typed_members() {
    let mut impl_item = item(
        "MyType",
        ApiItemKind::TraitImpl,
        "impl fmt::Debug for MyType {",
    );
    impl_item.attributes.push(ApiAttribute {
        text: "#[cfg(feature = \"std\")]".to_string(),
    });
    impl_item.members.push(member(
        "fmt",
        ApiMemberKind::Associated,
        "fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result;",
    ));

    let module = ApiModule {
        path: "demo".to_string(),
        doc_comments: Vec::new(),
        attributes: Vec::new(),
        items: vec![impl_item],
        modules: Vec::new(),
    };

    let lookup = navigation_lookup(&module);
    let lines = render_module_contents(&module, &RenderOptions::default(), &lookup);

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
    let mut inherent_impl = item("Foo", ApiItemKind::InherentImpl, "impl Foo {");
    inherent_impl.owner_name = Some("Foo".to_string());
    inherent_impl.owner_kind = Some(ApiItemKind::Struct);
    inherent_impl.members.push(member(
        "method",
        ApiMemberKind::Associated,
        "pub fn method(&self);",
    ));

    let module = ApiModule {
        path: "demo".to_string(),
        doc_comments: Vec::new(),
        attributes: Vec::new(),
        items: vec![
            item("Foo", ApiItemKind::Struct, "pub struct Foo;"),
            inherent_impl,
        ],
        modules: Vec::new(),
    };

    let lookup = navigation_lookup(&module);
    let lines = render_module_contents(&module, &RenderOptions::default(), &lookup);

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
fn tokenizes_private_fields_block_comment_as_a_comment() {
    assert_eq!(
        tokenize_line(
            "pub struct AsyncResponseBody(/* private fields */);",
            "AsyncResponseBody",
            token_kind::TYPE_NAME,
        )
        .into_iter()
        .map(|token| (token.kind, token.value))
        .collect::<Vec<_>>(),
        vec![
            (token_kind::KEYWORD, "pub".to_string()),
            (token_kind::KEYWORD, "struct".to_string()),
            (token_kind::TYPE_NAME, "AsyncResponseBody".to_string()),
            (token_kind::PUNCTUATION, "(".to_string()),
            (token_kind::COMMENT, "/* private fields */".to_string()),
            (token_kind::PUNCTUATION, ")".to_string()),
            (token_kind::PUNCTUATION, ";".to_string()),
        ]
    );
}

#[test]
fn keeps_duplicate_member_names_in_separate_inherent_impl_blocks() {
    let mut blob_impl = item(
        "Builder",
        ApiItemKind::InherentImpl,
        "impl Builder<BlobState> {",
    );
    blob_impl.owner_name = Some("Builder".to_string());
    blob_impl.owner_kind = Some(ApiItemKind::Struct);
    blob_impl.inherent_impl_sort_key = Some(InherentImplSortKey {
        type_arg_classes: vec![2],
        rendered_self_type: "Builder<BlobState>".to_string(),
    });
    blob_impl.members.push(member(
        "read",
        ApiMemberKind::Associated,
        "pub fn read(self) -> Self;",
    ));

    let mut generic_impl = item(
        "Builder",
        ApiItemKind::InherentImpl,
        "impl<S: QueueState> Builder<S> {",
    );
    generic_impl.owner_name = Some("Builder".to_string());
    generic_impl.owner_kind = Some(ApiItemKind::Struct);
    generic_impl.inherent_impl_sort_key = Some(InherentImplSortKey {
        type_arg_classes: vec![0],
        rendered_self_type: "Builder<S>".to_string(),
    });
    generic_impl.members.push(member(
        "read",
        ApiMemberKind::Associated,
        "pub fn read(self) -> Self;",
    ));

    let module = ApiModule {
        path: "demo".to_string(),
        doc_comments: Vec::new(),
        attributes: Vec::new(),
        items: vec![
            item("Builder", ApiItemKind::Struct, "pub struct Builder<S>(S);"),
            blob_impl,
            generic_impl,
        ],
        modules: Vec::new(),
    };

    let lookup = navigation_lookup(&module);
    let lines = render_module_contents(&module, &RenderOptions::default(), &lookup);
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
    let mut struct_item = item("Foo", ApiItemKind::Struct, "pub struct Foo;");
    struct_item.doc_comments.push("/// item docs".to_string());
    struct_item.members.push(member(
        "method",
        ApiMemberKind::Associated,
        "pub fn method(&self);",
    ));
    struct_item.members[0]
        .doc_comments
        .push("/// member docs".to_string());

    let module = ApiModule {
        path: "demo".to_string(),
        doc_comments: vec!["/// module docs".to_string()],
        attributes: Vec::new(),
        items: vec![struct_item],
        modules: Vec::new(),
    };

    let lookup = navigation_lookup(&module);
    let with_docs = render_module(&module, &RenderOptions::default(), &lookup);
    let without_docs = render_module(&module, &RenderOptions::new(false), &lookup);

    assert!(with_docs.iter().any(|line| {
        line.tokens
            .iter()
            .any(|token| token.is_documentation && token.value == "/// module docs")
    }));
    assert!(!without_docs
        .iter()
        .any(|line| line.tokens.iter().any(|token| token.is_documentation)));
}

#[test]
fn links_trait_impl_owner_to_local_definition() {
    let mut secret_bytes = item(
        "SecretBytes",
        ApiItemKind::Struct,
        "pub struct SecretBytes;",
    );
    secret_bytes.source_id = Some("secret-bytes".to_string());

    let mut trait_impl = item(
        "SecretBytes",
        ApiItemKind::TraitImpl,
        "impl<T> From<T> for SecretBytes {",
    );
    trait_impl.owner_name = Some("SecretBytes".to_string());
    trait_impl.owner_kind = Some(ApiItemKind::Struct);
    trait_impl.owner_source_id = Some("secret-bytes".to_string());
    trait_impl.declaration_path_references = vec![
        path_reference("From", "from-trait"),
        path_reference("crate::SecretBytes", "secret-bytes"),
    ];

    let model = ApiModel {
        package_name: "demo".to_string(),
        package_version: "1.0.0".to_string(),
        parser_version: "0.0.0".to_string(),
        root_module: ApiModule {
            path: "demo".to_string(),
            doc_comments: Vec::new(),
            attributes: Vec::new(),
            items: vec![secret_bytes, trait_impl],
            modules: Vec::new(),
        },
    };

    let lookup = NavigationLookup::new(&model);
    let lines = render_review_lines(&model, &RenderOptions::default(), &lookup);
    let trait_impl = lines
        .iter()
        .find(|line| {
            line.line_id.as_deref() == Some("module.demo.SecretBytes_1")
                && line.tokens.iter().any(|token| token.value == "impl")
        })
        .expect("expected trait impl line");

    assert_eq!(
        find_token(trait_impl, "SecretBytes")
            .navigate_to_id
            .as_deref(),
        Some("module.demo.SecretBytes_0")
    );
}

#[test]
fn links_trait_impl_owner_to_public_reexport() {
    let mut secret_bytes = item(
        "SecretBytes",
        ApiItemKind::Use,
        "pub use hidden::SecretBytes;",
    );
    secret_bytes.navigation_paths.push(navigation_path(
        "hidden::SecretBytes",
        "hidden-secret-bytes",
    ));

    let mut trait_impl = item(
        "SecretBytes",
        ApiItemKind::TraitImpl,
        "impl<T> From<T> for SecretBytes {",
    );
    trait_impl.owner_name = Some("SecretBytes".to_string());
    trait_impl.owner_kind = Some(ApiItemKind::Struct);
    trait_impl.owner_source_id = Some("hidden-secret-bytes".to_string());
    trait_impl.declaration_path_references = vec![
        path_reference("From", "from-trait"),
        path_reference("hidden::SecretBytes", "hidden-secret-bytes"),
    ];

    let model = ApiModel {
        package_name: "demo".to_string(),
        package_version: "1.0.0".to_string(),
        parser_version: "0.0.0".to_string(),
        root_module: ApiModule {
            path: "demo".to_string(),
            doc_comments: Vec::new(),
            attributes: Vec::new(),
            items: vec![secret_bytes, trait_impl],
            modules: Vec::new(),
        },
    };

    let lookup = NavigationLookup::new(&model);
    let lines = render_review_lines(&model, &RenderOptions::default(), &lookup);
    let trait_impl = lines
        .iter()
        .find(|line| {
            line.line_id.as_deref() == Some("module.demo.SecretBytes_1")
                && line.tokens.iter().any(|token| token.value == "impl")
        })
        .expect("expected trait impl line");

    assert_eq!(
        find_token(trait_impl, "SecretBytes")
            .navigate_to_id
            .as_deref(),
        Some("module.demo.SecretBytes_0")
    );
}

#[test]
fn links_trait_impl_owner_to_declaration_over_visible_reexport() {
    let mut root_error = item("Error", ApiItemKind::Use, "pub use hidden::Error;");
    root_error
        .navigation_paths
        .push(navigation_path("hidden::Error", "hidden-error"));

    let mut hidden_error = item("Error", ApiItemKind::Struct, "pub struct Error;");
    hidden_error.source_id = Some("hidden-error".to_string());
    hidden_error.attributes.push(ApiAttribute {
        text: "#[derive(SafeDebug)]".to_string(),
    });

    let mut trait_impl = item(
        "Error",
        ApiItemKind::TraitImpl,
        "impl<T> From<T> for Error {",
    );
    trait_impl.owner_name = Some("Error".to_string());
    trait_impl.owner_kind = Some(ApiItemKind::Struct);
    trait_impl.owner_source_id = Some("hidden-error".to_string());
    trait_impl.declaration_path_references = vec![
        path_reference("From", "from-trait"),
        path_reference_with_canonical("hidden::Error", Some("hidden::Error"), "hidden-error"),
    ];

    let model = ApiModel {
        package_name: "demo".to_string(),
        package_version: "1.0.0".to_string(),
        parser_version: "0.0.0".to_string(),
        root_module: ApiModule {
            path: "demo".to_string(),
            doc_comments: Vec::new(),
            attributes: Vec::new(),
            items: vec![root_error, trait_impl],
            modules: vec![ApiModule {
                path: "demo::hidden".to_string(),
                doc_comments: Vec::new(),
                attributes: Vec::new(),
                items: vec![hidden_error],
                modules: Vec::new(),
            }],
        },
    };

    let lookup = NavigationLookup::new(&model);
    let lines = render_review_lines(&model, &RenderOptions::default(), &lookup);
    let trait_impl = lines
        .iter()
        .find(|line| {
            line.line_id.as_deref() == Some("module.demo.Error_1")
                && line.tokens.iter().any(|token| token.value == "impl")
        })
        .expect("expected trait impl line");

    assert_eq!(
        find_token(trait_impl, "Error").navigate_to_id.as_deref(),
        Some("module.demo__hidden.Error_0")
    );
}

#[test]
fn keeps_trait_impl_owner_unlinked_without_visible_target() {
    let mut trait_impl = item(
        "Error",
        ApiItemKind::TraitImpl,
        "impl<T> From<T> for Error {",
    );
    trait_impl.owner_name = Some("Error".to_string());
    trait_impl.owner_kind = Some(ApiItemKind::Struct);
    trait_impl.owner_source_id = Some("external-error".to_string());
    trait_impl
        .declaration_path_references
        .push(path_reference("From", "from-trait"));

    let model = ApiModel {
        package_name: "demo".to_string(),
        package_version: "1.0.0".to_string(),
        parser_version: "0.0.0".to_string(),
        root_module: ApiModule {
            path: "demo".to_string(),
            doc_comments: Vec::new(),
            attributes: Vec::new(),
            items: vec![trait_impl],
            modules: Vec::new(),
        },
    };

    let lookup = NavigationLookup::new(&model);
    let lines = render_review_lines(&model, &RenderOptions::default(), &lookup);
    let trait_impl = lines
        .iter()
        .find(|line| line.line_id.as_deref() == Some("module.demo.Error_0"))
        .expect("expected trait impl line");

    assert!(find_token(trait_impl, "Error").navigate_to_id.is_none());
}

#[test]
fn links_associated_error_type_to_visible_error_target() {
    let mut visible_error = item("Error", ApiItemKind::Use, "pub use hidden::Error;");
    visible_error
        .navigation_paths
        .push(navigation_path("hidden::Error", "hidden-error"));

    let response = item("Response", ApiItemKind::Struct, "pub struct Response;");

    let mut try_from_impl = item(
        "ErrorResponse",
        ApiItemKind::TraitImpl,
        "impl TryFrom<Error> for ErrorResponse {",
    );
    try_from_impl.owner_name = Some("ErrorResponse".to_string());
    try_from_impl.owner_kind = Some(ApiItemKind::Struct);
    try_from_impl.owner_source_id = Some("error-response".to_string());
    try_from_impl.members.push(ApiMember {
        name: "Error".to_string(),
        kind: ApiMemberKind::Associated,
        doc_comments: Vec::new(),
        attributes: Vec::new(),
        declaration: "type Error = Error;".to_string(),
        declaration_path_references: vec![path_reference_with_canonical(
            "Error",
            Some("hidden::Error"),
            "hidden-error",
        )],
    });

    let model = ApiModel {
        package_name: "demo".to_string(),
        package_version: "1.0.0".to_string(),
        parser_version: "0.0.0".to_string(),
        root_module: ApiModule {
            path: "demo".to_string(),
            doc_comments: Vec::new(),
            attributes: Vec::new(),
            items: vec![
                visible_error,
                response,
                item(
                    "ErrorResponse",
                    ApiItemKind::Struct,
                    "pub struct ErrorResponse;",
                ),
                try_from_impl,
            ],
            modules: Vec::new(),
        },
    };

    let lookup = NavigationLookup::new(&model);
    let lines = render_review_lines(&model, &RenderOptions::default(), &lookup);
    let trait_impl = lines
        .iter()
        .find(|line| {
            line.line_id
                .as_deref()
                .is_some_and(|line_id| line_id.starts_with("module.demo.ErrorResponse_"))
                && line.tokens.iter().any(|token| token.value == "impl")
        })
        .expect("expected trait impl line");
    let error_type = find_child(
        trait_impl,
        &format!(
            "{}.Error_0",
            trait_impl.line_id.as_deref().expect("trait impl line id")
        ),
    );
    let error_tokens = error_type
        .tokens
        .iter()
        .filter(|token| token.value == "Error")
        .collect::<Vec<_>>();

    assert_eq!(error_tokens.len(), 2);
    assert!(error_tokens[0].navigate_to_id.is_none());
    assert_eq!(
        error_tokens[1].navigate_to_id.as_deref(),
        Some("module.demo.Error_0")
    );
}

#[test]
fn links_bare_result_to_public_reexport_when_resolved_path_matches() {
    let mut result = item("Result", ApiItemKind::Use, "pub use hidden::Result;");
    result
        .navigation_paths
        .push(navigation_path("hidden::Result", "hidden-result"));

    let mut parse = item(
        "parse",
        ApiItemKind::Function,
        "pub fn parse() -> Result<u8>;",
    );
    parse
        .declaration_path_references
        .push(path_reference("Result", "hidden-result"));

    let model = ApiModel {
        package_name: "demo".to_string(),
        package_version: "1.0.0".to_string(),
        parser_version: "0.0.0".to_string(),
        root_module: ApiModule {
            path: "demo".to_string(),
            doc_comments: Vec::new(),
            attributes: Vec::new(),
            items: vec![result, parse],
            modules: Vec::new(),
        },
    };

    let lookup = NavigationLookup::new(&model);
    let lines = render_review_lines(&model, &RenderOptions::default(), &lookup);
    let parse = lines
        .iter()
        .find(|line| line.line_id.as_deref() == Some("module.demo.parse_1"))
        .expect("expected parse line");

    assert_eq!(
        find_token(parse, "Result").navigate_to_id.as_deref(),
        Some("module.demo.Result_0")
    );
}

#[test]
fn links_crate_result_with_resolved_path_metadata() {
    let mut result = item("Result", ApiItemKind::Use, "pub use hidden::Result;");
    result
        .navigation_paths
        .push(navigation_path("hidden::Result", "hidden-result"));

    let mut parse = item(
        "parse",
        ApiItemKind::Function,
        "pub fn parse() -> crate::Result<u8>;",
    );
    parse
        .declaration_path_references
        .push(path_reference("crate::Result", "hidden-result"));

    let model = ApiModel {
        package_name: "demo".to_string(),
        package_version: "1.0.0".to_string(),
        parser_version: "0.0.0".to_string(),
        root_module: ApiModule {
            path: "demo".to_string(),
            doc_comments: Vec::new(),
            attributes: Vec::new(),
            items: vec![result, parse],
            modules: Vec::new(),
        },
    };

    let lookup = NavigationLookup::new(&model);
    let lines = render_review_lines(&model, &RenderOptions::default(), &lookup);
    let parse = lines
        .iter()
        .find(|line| line.line_id.as_deref() == Some("module.demo.parse_1"))
        .expect("expected parse line");

    assert_eq!(
        find_token(parse, "Result").navigate_to_id.as_deref(),
        Some("module.demo.Result_0")
    );
}

#[test]
fn keeps_std_result_references_unlinked_without_current_crate_reexport() {
    let mut parse = item(
        "parse",
        ApiItemKind::Function,
        "pub fn parse(std_result: std::result::Result<u8, u8>, std_fmt: std::fmt::Result, fmt_result: fmt::Result) -> Result<u8>;",
    );
    parse.declaration_path_references = vec![
        path_reference("std::result::Result", "std-result"),
        path_reference("std::fmt::Result", "std-fmt-result"),
        path_reference("fmt::Result", "fmt-result"),
        path_reference("Result", "other-crate-result"),
    ];

    let model = ApiModel {
        package_name: "demo".to_string(),
        package_version: "1.0.0".to_string(),
        parser_version: "0.0.0".to_string(),
        root_module: ApiModule {
            path: "demo".to_string(),
            doc_comments: Vec::new(),
            attributes: Vec::new(),
            items: vec![parse],
            modules: Vec::new(),
        },
    };

    let lookup = NavigationLookup::new(&model);
    let lines = render_review_lines(&model, &RenderOptions::default(), &lookup);
    let parse = lines
        .iter()
        .find(|line| line.line_id.as_deref() == Some("module.demo.parse_0"))
        .expect("expected parse line");

    let result_tokens = parse
        .tokens
        .iter()
        .filter(|token| token.value == "Result")
        .collect::<Vec<_>>();
    assert_eq!(result_tokens.len(), 4);
    assert!(result_tokens
        .into_iter()
        .all(|token| token.navigate_to_id.is_none()));
}

#[test]
fn prefers_original_definition_over_public_reexport_for_bare_result_links() {
    let mut root_reexport = item("Result", ApiItemKind::Use, "pub use errors::Result;");
    root_reexport
        .navigation_paths
        .push(navigation_path("errors::Result", "errors-result"));

    let mut original = item("Result", ApiItemKind::Struct, "pub struct Result;");
    original.source_id = Some("errors-result".to_string());

    let mut parse = item("parse", ApiItemKind::Function, "pub fn parse() -> Result;");
    parse
        .declaration_path_references
        .push(path_reference("Result", "errors-result"));

    let model = ApiModel {
        package_name: "demo".to_string(),
        package_version: "1.0.0".to_string(),
        parser_version: "0.0.0".to_string(),
        root_module: ApiModule {
            path: "demo".to_string(),
            doc_comments: Vec::new(),
            attributes: Vec::new(),
            items: vec![root_reexport],
            modules: vec![
                ApiModule {
                    path: "demo::client".to_string(),
                    doc_comments: Vec::new(),
                    attributes: Vec::new(),
                    items: vec![parse],
                    modules: Vec::new(),
                },
                ApiModule {
                    path: "demo::errors".to_string(),
                    doc_comments: Vec::new(),
                    attributes: Vec::new(),
                    items: vec![original],
                    modules: Vec::new(),
                },
            ],
        },
    };

    let lookup = NavigationLookup::new(&model);
    let lines = render_review_lines(&model, &RenderOptions::default(), &lookup);
    let client = lines
        .iter()
        .find(|line| line.line_id.as_deref() == Some("module.demo__client"))
        .expect("expected client module");
    let parse = client
        .children
        .iter()
        .find(|line| line.line_id.as_deref() == Some("module.demo__client.parse_0"))
        .expect("expected parse line");

    assert_eq!(
        find_token(parse, "Result").navigate_to_id.as_deref(),
        Some("module.demo__errors.Result_0")
    );
}

#[test]
fn keeps_field_type_links_out_of_navigation_tree() {
    let mut access_token = item(
        "AccessToken",
        ApiItemKind::Struct,
        "pub struct AccessToken {",
    );
    access_token.members.push(member(
        "secret",
        ApiMemberKind::Field,
        "pub secret: Secret,",
    ));

    let module = ApiModule {
        path: "demo".to_string(),
        doc_comments: Vec::new(),
        attributes: Vec::new(),
        items: vec![
            access_token,
            item("Secret", ApiItemKind::Struct, "pub struct Secret;"),
        ],
        modules: Vec::new(),
    };

    let lookup = navigation_lookup(&module);
    let lines = render_module_contents(&module, &RenderOptions::default(), &lookup);
    let access_token = lines
        .iter()
        .find(|line| line.line_id.as_deref() == Some("module.demo.AccessToken_0"))
        .expect("expected AccessToken line");
    let field = find_child(access_token, "module.demo.AccessToken_0.secret_0");
    let secret = find_token(field, "Secret");

    assert_eq!(
        secret.navigate_to_id.as_deref(),
        Some("module.demo.Secret_1")
    );
    assert_eq!(secret.navigation_display_name, None);
    assert_eq!(secret.render_classes, None);

    let declaration = find_token(
        lines
            .iter()
            .find(|line| line.line_id.as_deref() == Some("module.demo.Secret_1"))
            .expect("expected Secret line"),
        "Secret",
    );
    assert_eq!(
        declaration.navigation_display_name.as_deref(),
        Some("Secret")
    );
    assert_eq!(declaration.render_classes, Some(vec!["class".to_string()]));
}

#[test]
fn keeps_member_type_references_out_of_navigation_tree() {
    let mut token_credential = item(
        "TokenCredential",
        ApiItemKind::Trait,
        "pub trait TokenCredential {",
    );
    token_credential.members.push(member(
        "get_token",
        ApiMemberKind::Associated,
        "fn get_token(&self, options: TokenRequestOptions) -> AccessToken;",
    ));

    let module = ApiModule {
        path: "demo".to_string(),
        doc_comments: Vec::new(),
        attributes: Vec::new(),
        items: vec![
            item(
                "AccessToken",
                ApiItemKind::Struct,
                "pub struct AccessToken;",
            ),
            item(
                "TokenRequestOptions",
                ApiItemKind::Struct,
                "pub struct TokenRequestOptions;",
            ),
            token_credential,
        ],
        modules: Vec::new(),
    };

    let lookup = navigation_lookup(&module);
    let lines = render_module_contents(&module, &RenderOptions::default(), &lookup);
    let token_credential = lines
        .iter()
        .find(|line| line.line_id.as_deref() == Some("module.demo.TokenCredential_1"))
        .expect("expected TokenCredential line");
    let get_token = find_child(
        token_credential,
        "module.demo.TokenCredential_1.get_token_0",
    );

    for (value, expected) in [
        ("TokenRequestOptions", "module.demo.TokenRequestOptions_2"),
        ("AccessToken", "module.demo.AccessToken_0"),
    ] {
        let token = find_token(get_token, value);
        assert_eq!(token.navigate_to_id.as_deref(), Some(expected));
        assert_eq!(token.navigation_display_name, None);
        assert_eq!(token.render_classes, None);
    }
}

#[test]
fn orders_modules_after_non_module_navigation_items() {
    let model = ApiModel {
        package_name: "demo".to_string(),
        package_version: "1.0.0".to_string(),
        parser_version: "0.0.0".to_string(),
        root_module: ApiModule {
            path: "demo".to_string(),
            doc_comments: Vec::new(),
            attributes: Vec::new(),
            items: vec![
                item(
                    "AccessToken",
                    ApiItemKind::Struct,
                    "pub struct AccessToken;",
                ),
                item("sleep", ApiItemKind::Function, "pub async fn sleep();"),
                item("ANSWER", ApiItemKind::Const, "pub const ANSWER: u32 = 42;"),
                item("Error", ApiItemKind::Use, "pub use hidden::Error;"),
            ],
            modules: vec![
                ApiModule {
                    path: "demo::zeta".to_string(),
                    doc_comments: Vec::new(),
                    attributes: Vec::new(),
                    items: Vec::new(),
                    modules: Vec::new(),
                },
                ApiModule {
                    path: "demo::alpha".to_string(),
                    doc_comments: Vec::new(),
                    attributes: Vec::new(),
                    items: Vec::new(),
                    modules: Vec::new(),
                },
            ],
        },
    };

    let lookup = NavigationLookup::new(&model);
    let lines = render_review_lines(&model, &RenderOptions::default(), &lookup);

    assert_eq!(
        top_level_navigation_line_ids(&lines),
        vec![
            "module.demo.ANSWER_0",
            "module.demo.Error_1",
            "module.demo.sleep_2",
            "module.demo.AccessToken_3",
            "module.demo__alpha",
            "module.demo__zeta",
        ]
    );
}

#[test]
fn shares_module_last_tree_order_with_navigation_walk() {
    let module = ApiModule {
        path: "demo".to_string(),
        doc_comments: Vec::new(),
        attributes: Vec::new(),
        items: vec![
            item("Error", ApiItemKind::Use, "pub use hidden::Error;"),
            item("sleep", ApiItemKind::Function, "pub async fn sleep();"),
        ],
        modules: vec![
            ApiModule {
                path: "demo::zeta".to_string(),
                doc_comments: Vec::new(),
                attributes: Vec::new(),
                items: Vec::new(),
                modules: Vec::new(),
            },
            ApiModule {
                path: "demo::alpha".to_string(),
                doc_comments: Vec::new(),
                attributes: Vec::new(),
                items: Vec::new(),
                modules: Vec::new(),
            },
        ],
    };

    let labels = apiview_tree_entries(&module)
        .into_iter()
        .map(|entry| match entry {
            ModuleTreeEntry::Item(item) => format!("item:{}", item.name),
            ModuleTreeEntry::Module(module) => format!("module:{}", module.local_name()),
        })
        .collect::<Vec<_>>();

    assert_eq!(
        labels,
        vec!["item:Error", "item:sleep", "module:alpha", "module:zeta",]
    );
}

#[test]
fn includes_same_crate_reexported_functions_in_tree() {
    let mut top_level_sleep = item("sleep", ApiItemKind::Use, "pub use sleep::sleep;");
    top_level_sleep.source_id = Some("sleep-reexport".to_string());
    top_level_sleep
        .navigation_paths
        .push(navigation_path("sleep::sleep", "sleep-fn"));

    let mut module_sleep = item("sleep", ApiItemKind::Function, "pub async fn sleep();");
    module_sleep.source_id = Some("sleep-fn".to_string());

    let module = ApiModule {
        path: "demo".to_string(),
        doc_comments: Vec::new(),
        attributes: Vec::new(),
        items: vec![
            item("Error", ApiItemKind::Struct, "pub struct Error;"),
            top_level_sleep,
        ],
        modules: vec![ApiModule {
            path: "demo::sleep".to_string(),
            doc_comments: Vec::new(),
            attributes: Vec::new(),
            items: vec![module_sleep],
            modules: Vec::new(),
        }],
    };

    let lines = render_module_contents(
        &module,
        &RenderOptions::default(),
        &navigation_lookup(&module),
    );

    assert_eq!(
        top_level_navigation_line_ids(&lines),
        vec![
            "module.demo.sleep_0",
            "module.demo.Error_1",
            "module.demo__sleep"
        ]
    );

    let top_level_sleep = lines
        .iter()
        .find(|line| line.line_id.as_deref() == Some("module.demo.sleep_0"))
        .expect("expected root sleep reexport line");
    let sleep_token = top_level_sleep
        .tokens
        .iter()
        .rev()
        .find(|token| token.value == "sleep")
        .expect("expected reexported sleep token");
    assert_eq!(
        sleep_token.navigate_to_id.as_deref(),
        Some("module.demo__sleep.sleep_0")
    );
}

#[test]
fn groups_same_crate_reexports_with_other_reexports() {
    let mut top_level_reexport = item(
        "ClientOptions",
        ApiItemKind::Use,
        "pub use options::ClientOptions;",
    );
    top_level_reexport
        .navigation_paths
        .push(navigation_path("options::ClientOptions", "client-options"));

    let mut module_item = item(
        "ClientOptions",
        ApiItemKind::Struct,
        "pub struct ClientOptions;",
    );
    module_item.source_id = Some("client-options".to_string());

    let module = ApiModule {
        path: "demo".to_string(),
        doc_comments: Vec::new(),
        attributes: Vec::new(),
        items: vec![top_level_reexport],
        modules: vec![ApiModule {
            path: "demo::options".to_string(),
            doc_comments: Vec::new(),
            attributes: Vec::new(),
            items: vec![module_item],
            modules: Vec::new(),
        }],
    };

    let lines = render_module_contents(
        &module,
        &RenderOptions::default(),
        &navigation_lookup(&module),
    );

    assert_eq!(
        top_level_navigation_line_ids(&lines),
        vec!["module.demo.ClientOptions_0", "module.demo__options"]
    );
}

#[test]
fn same_crate_reexport_links_to_nested_declaration() {
    let mut root_client = item(
        "CosmosClient",
        ApiItemKind::Use,
        "pub use clients::CosmosClient;",
    );
    root_client.source_id = Some("clients-cosmos-client".to_string());
    root_client.navigation_paths.push(navigation_path(
        "clients::CosmosClient",
        "clients-cosmos-client",
    ));

    let mut nested_client = item(
        "CosmosClient",
        ApiItemKind::Struct,
        "pub struct CosmosClient;",
    );
    nested_client.source_id = Some("clients-cosmos-client".to_string());

    let model = ApiModel {
        package_name: "demo".to_string(),
        package_version: "1.0.0".to_string(),
        parser_version: "0.0.0".to_string(),
        root_module: ApiModule {
            path: "demo".to_string(),
            doc_comments: Vec::new(),
            attributes: Vec::new(),
            items: vec![root_client],
            modules: vec![ApiModule {
                path: "demo::clients".to_string(),
                doc_comments: Vec::new(),
                attributes: Vec::new(),
                items: vec![nested_client],
                modules: Vec::new(),
            }],
        },
    };

    let lookup = NavigationLookup::new(&model);
    let lines = render_review_lines(&model, &RenderOptions::default(), &lookup);
    let reexport = lines
        .iter()
        .find(|line| line.line_id.as_deref() == Some("module.demo.CosmosClient_0"))
        .expect("expected root CosmosClient reexport line");

    assert_eq!(
        find_token(reexport, "CosmosClient")
            .navigate_to_id
            .as_deref(),
        Some("module.demo__clients.CosmosClient_0")
    );
}

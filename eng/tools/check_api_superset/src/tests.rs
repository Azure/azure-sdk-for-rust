// Copyright (c) Microsoft Corporation. All rights reserved.
// Licensed under the MIT License.

use super::*;
use rustdoc_types::{
    Abi, Enum, Function, FunctionHeader, FunctionSignature, Generics, Item, Module, Struct,
    StructKind, Target, Use,
};

fn empty_generics() -> Generics {
    Generics {
        params: vec![],
        where_predicates: vec![],
    }
}

fn make_item(id: u32, name: Option<&str>, vis: Visibility, inner: ItemEnum) -> Item {
    Item {
        id: Id(id),
        crate_id: 0,
        name: name.map(String::from),
        span: None,
        visibility: vis,
        docs: None,
        links: HashMap::new(),
        attrs: vec![],
        deprecation: None,
        inner,
    }
}

fn make_module(id: u32, name: &str, items: Vec<Id>) -> Item {
    make_item(
        id,
        Some(name),
        Visibility::Public,
        ItemEnum::Module(Module {
            is_crate: false,
            items,
            is_stripped: false,
        }),
    )
}

fn make_root_module(id: u32, items: Vec<Id>) -> Item {
    make_item(
        id,
        Some("my_crate"),
        Visibility::Public,
        ItemEnum::Module(Module {
            is_crate: true,
            items,
            is_stripped: false,
        }),
    )
}

fn make_struct(id: u32, name: &str) -> Item {
    make_item(
        id,
        Some(name),
        Visibility::Public,
        ItemEnum::Struct(Struct {
            kind: StructKind::Unit,
            generics: empty_generics(),
            impls: vec![],
        }),
    )
}

fn make_enum(id: u32, name: &str) -> Item {
    make_item(
        id,
        Some(name),
        Visibility::Public,
        ItemEnum::Enum(Enum {
            generics: empty_generics(),
            has_stripped_variants: false,
            variants: vec![],
            impls: vec![],
        }),
    )
}

fn make_fn(id: u32, name: &str) -> Item {
    make_item(
        id,
        Some(name),
        Visibility::Public,
        ItemEnum::Function(Function {
            sig: FunctionSignature {
                inputs: vec![],
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

fn make_use(id: u32, source: &str, name: &str, target: u32, is_glob: bool) -> Item {
    make_item(
        id,
        None,
        Visibility::Public,
        ItemEnum::Use(Use {
            source: source.to_string(),
            name: name.to_string(),
            id: Some(Id(target)),
            is_glob,
        }),
    )
}

fn make_crate(root_id: u32, items: Vec<Item>) -> Crate {
    let mut index = HashMap::new();
    for item in items {
        index.insert(item.id, item);
    }
    Crate {
        root: Id(root_id),
        crate_version: None,
        includes_private: false,
        index,
        paths: HashMap::new(),
        external_crates: HashMap::new(),
        target: Target {
            triple: String::new(),
            target_features: vec![],
        },
        format_version: 0,
    }
}

fn empty_deps() -> HashMap<&'static str, &'static Crate> {
    HashMap::new()
}

#[test]
fn basic_items_at_root() {
    let krate = make_crate(
        0,
        vec![
            make_root_module(0, vec![Id(1), Id(2), Id(3)]),
            make_struct(1, "MyStruct"),
            make_enum(2, "MyEnum"),
            make_fn(3, "my_fn"),
        ],
    );
    let types = collect_public_types(&krate, &empty_deps());
    assert_eq!(types.get("MyStruct"), Some(&"struct"));
    assert_eq!(types.get("MyEnum"), Some(&"enum"));
    assert_eq!(types.get("my_fn"), Some(&"fn"));
    assert_eq!(types.len(), 3);
}

#[test]
fn items_in_nested_module() {
    let krate = make_crate(
        0,
        vec![
            make_root_module(0, vec![Id(1)]),
            make_module(1, "error", vec![Id(2), Id(3)]),
            make_struct(2, "Error"),
            make_enum(3, "ErrorKind"),
        ],
    );
    let types = collect_public_types(&krate, &empty_deps());
    assert_eq!(types.get("error::Error"), Some(&"struct"));
    assert_eq!(types.get("error::ErrorKind"), Some(&"enum"));
    assert_eq!(types.len(), 2);
}

#[test]
fn private_items_skipped() {
    let private_struct = make_item(
        1,
        Some("Secret"),
        Visibility::Default,
        ItemEnum::Struct(Struct {
            kind: StructKind::Unit,
            generics: empty_generics(),
            impls: vec![],
        }),
    );
    let krate = make_crate(
        0,
        vec![
            make_root_module(0, vec![Id(1), Id(2)]),
            private_struct,
            make_struct(2, "Public"),
        ],
    );
    let types = collect_public_types(&krate, &empty_deps());
    assert_eq!(types.get("Public"), Some(&"struct"));
    assert!(!types.contains_key("Secret"));
    assert_eq!(types.len(), 1);
}

#[test]
fn non_glob_use_reexport() {
    // Crate re-exports a struct from within its own index via `pub use`.
    let krate = make_crate(
        0,
        vec![
            make_root_module(0, vec![Id(1)]),
            make_module(1, "models", vec![Id(2), Id(3)]),
            make_struct(3, "Foo"),
            make_use(2, "internal::Foo", "Foo", 3, false),
        ],
    );
    let types = collect_public_types(&krate, &empty_deps());
    assert_eq!(types.get("models::Foo"), Some(&"struct"));
}

#[test]
fn external_item_via_paths() {
    // A `pub use dep::SomeStruct` where the target is only in `paths`, not `index`.
    let mut krate = make_crate(
        0,
        vec![
            make_root_module(0, vec![Id(1)]),
            make_use(1, "dep::SomeStruct", "SomeStruct", 100, false),
        ],
    );
    krate.paths.insert(
        Id(100),
        ItemSummary {
            crate_id: 1,
            path: vec!["dep".into(), "SomeStruct".into()],
            kind: ItemKind::Struct,
        },
    );
    let types = collect_public_types(&krate, &empty_deps());
    assert_eq!(types.get("SomeStruct"), Some(&"struct"));
}

#[test]
fn glob_reexport_from_external_crate() {
    // dep_crate has error::Error and error::ErrorKind.
    let dep = make_crate(
        0,
        vec![
            make_root_module(0, vec![Id(1)]),
            make_module(1, "error", vec![Id(2), Id(3)]),
            make_struct(2, "Error"),
            make_enum(3, "ErrorKind"),
        ],
    );

    // main_crate has `pub use dep::error::*` inside its own error module.
    let mut main = make_crate(
        10,
        vec![
            make_root_module(10, vec![Id(11)]),
            make_module(11, "error", vec![Id(12)]),
            make_use(12, "dep::error", "error", 99, true),
        ],
    );
    main.paths.insert(
        Id(99),
        ItemSummary {
            crate_id: 1,
            path: vec!["dep".into(), "error".into()],
            kind: ItemKind::Module,
        },
    );

    let dep_static: &'static Crate = Box::leak(Box::new(dep));
    let deps: HashMap<&str, &Crate> = [("dep", dep_static as &Crate)].into();

    let types = collect_public_types(&main, &deps);
    assert_eq!(types.get("error::Error"), Some(&"struct"));
    assert_eq!(types.get("error::ErrorKind"), Some(&"enum"));
    assert_eq!(types.len(), 2);
}

#[test]
fn external_module_reexport() {
    // dep_crate has json::from_json and json::to_json.
    let dep = make_crate(
        0,
        vec![
            make_root_module(0, vec![Id(1)]),
            make_module(1, "json", vec![Id(2), Id(3)]),
            make_fn(2, "from_json"),
            make_fn(3, "to_json"),
        ],
    );

    // main_crate does `pub use dep::json` (re-exports the whole module).
    // The target is in paths as a Module kind.
    let mut main = make_crate(
        10,
        vec![
            make_root_module(10, vec![Id(11)]),
            make_use(11, "dep::json", "json", 99, false),
        ],
    );
    main.paths.insert(
        Id(99),
        ItemSummary {
            crate_id: 1,
            path: vec!["dep".into(), "json".into()],
            kind: ItemKind::Module,
        },
    );

    let dep_static: &'static Crate = Box::leak(Box::new(dep));
    let deps: HashMap<&str, &Crate> = [("dep", dep_static as &Crate)].into();

    let types = collect_public_types(&main, &deps);
    assert_eq!(types.get("json::from_json"), Some(&"fn"));
    assert_eq!(types.get("json::to_json"), Some(&"fn"));
    assert_eq!(types.len(), 2);
}

#[test]
fn internal_glob_reexport() {
    // Crate has a private submodule whose items are re-exported via glob
    // into a public module (e.g., `pub use retry::*` inside policies module).
    let retry_module = {
        let mut item = make_module(3, "retry", vec![Id(4)]);
        item.visibility = Visibility::Default; // private module
        item
    };
    let krate = make_crate(
        0,
        vec![
            make_root_module(0, vec![Id(1)]),
            make_module(1, "policies", vec![Id(2)]),
            make_use(2, "retry", "retry", 3, true),
            retry_module,
            make_struct(4, "RetryPolicy"),
        ],
    );
    let types = collect_public_types(&krate, &empty_deps());
    assert_eq!(types.get("policies::RetryPolicy"), Some(&"struct"));
    assert_eq!(types.len(), 1);
}

#[test]
fn find_module_items_nested() {
    let krate = make_crate(
        0,
        vec![
            make_root_module(0, vec![Id(1)]),
            make_module(1, "http", vec![Id(2)]),
            make_module(2, "headers", vec![Id(3)]),
            make_struct(3, "HeaderName"),
        ],
    );
    let items = find_module_items(&krate, &["http", "headers"]);
    assert!(items.is_some());
    assert_eq!(items.unwrap(), vec![Id(3)]);
}

#[test]
fn find_module_items_empty_path_returns_root() {
    let krate = make_crate(
        0,
        vec![
            make_root_module(0, vec![Id(1), Id(2)]),
            make_struct(1, "A"),
            make_struct(2, "B"),
        ],
    );
    let items = find_module_items::<&str>(&krate, &[]);
    assert!(items.is_some());
    assert_eq!(items.unwrap(), vec![Id(1), Id(2)]);
}

#[test]
fn find_module_items_missing_returns_none() {
    let krate = make_crate(
        0,
        vec![make_root_module(0, vec![Id(1)]), make_struct(1, "Foo")],
    );
    let items = find_module_items(&krate, &["nonexistent"]);
    assert!(items.is_none());
}

#[test]
fn impl_items_ignored() {
    let krate = make_crate(
        0,
        vec![
            make_root_module(0, vec![Id(1), Id(2)]),
            make_struct(1, "Foo"),
            make_item(
                2,
                None,
                Visibility::Public,
                ItemEnum::Impl(rustdoc_types::Impl {
                    is_unsafe: false,
                    generics: empty_generics(),
                    provided_trait_methods: vec![],
                    trait_: None,
                    for_: rustdoc_types::Type::Primitive("u32".into()),
                    items: vec![],
                    is_negative: false,
                    is_synthetic: false,
                    blanket_impl: None,
                }),
            ),
        ],
    );
    let types = collect_public_types(&krate, &empty_deps());
    assert_eq!(types.len(), 1);
    assert_eq!(types.get("Foo"), Some(&"struct"));
}

#[test]
fn transitive_glob_reexport() {
    // base_crate has error::Error.
    let base = make_crate(
        0,
        vec![
            make_root_module(0, vec![Id(1)]),
            make_module(1, "error", vec![Id(2)]),
            make_struct(2, "Error"),
        ],
    );

    // mid_crate re-exports base::error::* into its own error module.
    let mut mid = make_crate(
        10,
        vec![
            make_root_module(10, vec![Id(11)]),
            make_module(11, "error", vec![Id(12)]),
            make_use(12, "base::error", "error", 99, true),
        ],
    );
    mid.paths.insert(
        Id(99),
        ItemSummary {
            crate_id: 1,
            path: vec!["base".into(), "error".into()],
            kind: ItemKind::Module,
        },
    );

    // top_crate re-exports mid::error::* into its own error module.
    let mut top = make_crate(
        20,
        vec![
            make_root_module(20, vec![Id(21)]),
            make_module(21, "error", vec![Id(22)]),
            make_use(22, "mid::error", "error", 199, true),
        ],
    );
    top.paths.insert(
        Id(199),
        ItemSummary {
            crate_id: 2,
            path: vec!["mid".into(), "error".into()],
            kind: ItemKind::Module,
        },
    );

    let base_static: &'static Crate = Box::leak(Box::new(base));
    let mid_static: &'static Crate = Box::leak(Box::new(mid));
    let deps: HashMap<&str, &Crate> = [
        ("base", base_static as &Crate),
        ("mid", mid_static as &Crate),
    ]
    .into();

    let types = collect_public_types(&top, &deps);
    assert_eq!(types.get("error::Error"), Some(&"struct"));
    assert_eq!(types.len(), 1);
}

#[test]
fn parse_exemptions_filters_comments_and_blanks() {
    let input =
        "# comment\nazure_core::http::REDACTED_PATTERN\n\n  azure_core::http::QueryBuilder  \n";
    let exemptions = parse_exemptions(input);
    assert!(exemptions.contains("azure_core::http::REDACTED_PATTERN"));
    assert!(exemptions.contains("azure_core::http::QueryBuilder"));
    assert_eq!(exemptions.len(), 2);
}

#[test]
fn exemptions_exclude_missing_items() {
    let mut superset = BTreeMap::new();
    superset.insert("Foo".to_string(), "struct");

    let mut subset = BTreeMap::new();
    subset.insert("Foo".to_string(), "struct");
    subset.insert("Bar".to_string(), "fn");
    subset.insert("Baz".to_string(), "enum");

    // Exempt "Bar" using the superset crate's missing fully-qualified path.
    let exemptions = parse_exemptions("my_crate::Bar\n");

    let missing = find_missing("my_crate", &superset, &subset, &exemptions);
    assert_eq!(missing.len(), 1);
    assert_eq!(missing[0].0, "Baz");
}

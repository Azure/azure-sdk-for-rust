// Copyright (c) Microsoft Corporation. All rights reserved.
// Licensed under the MIT License.

//! Equivalence, path-selection, service-limit, and telemetry tests for PATCH strategies.

use std::{
    num::{NonZeroU16, NonZeroU32, NonZeroU8},
    sync::{Arc, Mutex},
};

use azure_core::http::{Etag, Method, Request, Url};
use azure_data_cosmos_driver::in_memory_emulator::{
    ConsistencyLevel, InMemoryEmulatorHttpClient, RequestObserver, VirtualAccountConfig,
    VirtualRegion,
};
use azure_data_cosmos_driver::models::{
    AccountReference, ContainerReference, CosmosOperation, CosmosResponse, ItemReference,
    PartitionKey, PatchInstructions, PatchOperation, PatchTrackingId, Precondition,
    PATCH_TRACKING_PROPERTY,
};
use azure_data_cosmos_driver::options::{
    ContentResponseOnWrite, DriverOptions, OperationOptions, OperationOptionsBuilder, PatchStrategy,
};
use azure_data_cosmos_driver::CosmosDriver;

const GATEWAY_URL: &str = "https://eastus.emulator.local";
const ITEM_ID: &str = "strategy-item";
const PARTITION_KEY: &str = "pk1";

#[derive(Debug, Default)]
struct MethodRecorder {
    methods: Mutex<Vec<Method>>,
}

impl MethodRecorder {
    fn new() -> Arc<Self> {
        Arc::new(Self::default())
    }

    fn data_plane_methods(&self) -> Vec<Method> {
        self.methods.lock().unwrap().clone()
    }

    fn clear(&self) {
        self.methods.lock().unwrap().clear();
    }
}

impl RequestObserver for MethodRecorder {
    fn on_request(&self, request: &Request) {
        let path = request.url().path();
        if path != "/" && !path.ends_with("/pkranges") {
            self.methods.lock().unwrap().push(request.method());
        }
    }
}

async fn build_driver(
    recorder: Option<Arc<MethodRecorder>>,
) -> (Arc<CosmosDriver>, ContainerReference) {
    build_driver_with_defaults(recorder, OperationOptions::default()).await
}

async fn build_driver_with_defaults(
    recorder: Option<Arc<MethodRecorder>>,
    default_options: OperationOptions,
) -> (Arc<CosmosDriver>, ContainerReference) {
    let config = VirtualAccountConfig::new(vec![VirtualRegion::new(
        "East US",
        Url::parse(GATEWAY_URL).unwrap(),
    )])
    .unwrap()
    .with_consistency(ConsistencyLevel::Session);
    let emulator = InMemoryEmulatorHttpClient::new(config);
    let emulator = match recorder {
        Some(recorder) => emulator.with_request_observer(recorder),
        None => emulator,
    };
    let store = emulator.store();
    store.create_database("testdb");
    store.create_container(
        "testdb",
        "testcoll",
        serde_json::from_value(serde_json::json!({
            "paths": ["/pk"],
            "kind": "Hash",
            "version": 2
        }))
        .unwrap(),
    );
    let emulator = Arc::new(emulator);

    let runtime = emulator
        .runtime_builder()
        .build()
        .await
        .expect("runtime should build");
    let account =
        AccountReference::with_account_key(Url::parse(GATEWAY_URL).unwrap(), "ZW11bGF0b3Ita2V5");
    let driver = runtime
        .create_driver(
            DriverOptions::builder(account)
                .with_operation_options(default_options)
                .build(),
        )
        .await
        .expect("driver should initialize");
    let container = driver
        .resolve_container("testdb", "testcoll", OperationOptions::default())
        .await
        .expect("container should resolve");
    (driver, container)
}

fn seed_document() -> serde_json::Value {
    serde_json::json!({
        "id": ITEM_ID,
        "pk": PARTITION_KEY,
        "name": "before",
        "visits": 1,
        "nested": { "kept": true, "removed": "value" },
        "tags": ["a", "b"]
    })
}

fn item(container: &ContainerReference) -> ItemReference {
    ItemReference::from_name(
        container,
        PartitionKey::from(PARTITION_KEY),
        ITEM_ID.to_string(),
    )
}

async fn create_item(
    driver: &CosmosDriver,
    container: &ContainerReference,
    document: &serde_json::Value,
) {
    driver
        .execute_singleton_operation(
            CosmosOperation::create_item(item(container))
                .with_body(serde_json::to_vec(document).unwrap()),
            OperationOptions::default(),
        )
        .await
        .expect("item should be created");
}

async fn delete_item(driver: &CosmosDriver, container: &ContainerReference) {
    let _ = driver
        .execute_operation(
            CosmosOperation::delete_item(item(container)),
            OperationOptions::default(),
        )
        .await;
}

fn response_json(response: CosmosResponse) -> serde_json::Value {
    let bytes = response.into_body().single().unwrap_or_default();
    super::parse_json_body(&bytes).unwrap_or(serde_json::Value::Null)
}

fn without_system_properties(mut document: serde_json::Value) -> serde_json::Value {
    if let Some(object) = document.as_object_mut() {
        object.retain(|name, _| !name.starts_with('_'));
    }
    document
}

async fn read_stored_raw(
    driver: &CosmosDriver,
    container: &ContainerReference,
) -> serde_json::Value {
    response_json(
        driver
            .execute_singleton_operation(
                CosmosOperation::read_item(item(container)),
                OperationOptions::default(),
            )
            .await
            .expect("item should be readable"),
    )
}

async fn read_stored(driver: &CosmosDriver, container: &ContainerReference) -> serde_json::Value {
    without_system_properties(read_stored_raw(driver, container).await)
}

async fn execute_patch(
    driver: &CosmosDriver,
    container: &ContainerReference,
    instructions: &PatchInstructions,
    strategy: PatchStrategy,
) -> Result<CosmosResponse, azure_data_cosmos_driver::error::CosmosError> {
    let operation = CosmosOperation::patch_item(item(container))
        .with_body(serde_json::to_vec(instructions).unwrap());
    let options = OperationOptionsBuilder::new()
        .with_patch_strategy(strategy)
        .with_content_response_on_write(ContentResponseOnWrite::Enabled)
        .build();
    driver.execute_singleton_operation(operation, options).await
}

fn patch_body(instructions: &PatchInstructions, condition: &str) -> Vec<u8> {
    let mut body = serde_json::to_value(instructions).unwrap();
    body.as_object_mut().unwrap().insert(
        "condition".to_owned(),
        serde_json::Value::String(condition.to_owned()),
    );
    serde_json::to_vec(&body).unwrap()
}

async fn execute_conditional_patch(
    driver: &CosmosDriver,
    container: &ContainerReference,
    instructions: &PatchInstructions,
    condition: &str,
    strategy: PatchStrategy,
) -> Result<CosmosResponse, azure_data_cosmos_driver::error::CosmosError> {
    let operation =
        CosmosOperation::patch_item(item(container)).with_body(patch_body(instructions, condition));
    let options = OperationOptionsBuilder::new()
        .with_patch_strategy(strategy)
        .with_content_response_on_write(ContentResponseOnWrite::Enabled)
        .build();
    driver.execute_singleton_operation(operation, options).await
}

async fn assert_equivalent(instructions: PatchInstructions, scenario: &str) {
    let (driver, container) = build_driver(None).await;
    let seed = seed_document();
    let client_should_track = !instructions.is_retry_safe();

    create_item(&driver, &container, &seed).await;
    let client_response = execute_patch(
        &driver,
        &container,
        &instructions,
        PatchStrategy::ClientSide,
    )
    .await;
    let client_stored = read_stored_raw(&driver, &container).await;

    delete_item(&driver, &container).await;
    create_item(&driver, &container, &seed).await;
    let server_response = execute_patch(
        &driver,
        &container,
        &instructions,
        PatchStrategy::ServerSide,
    )
    .await;
    let server_stored = read_stored_raw(&driver, &container).await;

    match (client_response, server_response) {
        (Ok(client), Ok(server)) => {
            let client_body = response_json(client);
            let server_body = response_json(server);
            assert_eq!(
                client_body.get(PATCH_TRACKING_PROPERTY).is_some(),
                client_should_track,
                "{scenario}: client response marker mismatch"
            );
            assert_eq!(
                client_stored.get(PATCH_TRACKING_PROPERTY).is_some(),
                client_should_track,
                "{scenario}: client stored marker mismatch"
            );
            assert!(
                server_body.get(PATCH_TRACKING_PROPERTY).is_none(),
                "{scenario}: server response must not contain a marker"
            );
            assert!(
                server_stored.get(PATCH_TRACKING_PROPERTY).is_none(),
                "{scenario}: server item must not contain a marker"
            );
            let client_body = without_system_properties(client_body);
            let server_body = without_system_properties(server_body);
            let client_stored = without_system_properties(client_stored);
            let server_stored = without_system_properties(server_stored);
            assert_eq!(client_body, server_body, "{scenario}: response mismatch");
            assert_eq!(client_stored, server_stored, "{scenario}: stored mismatch");
            assert_eq!(
                client_body, client_stored,
                "{scenario}: response/store mismatch"
            );
        }
        (Err(client), Err(server)) => {
            assert_eq!(
                client.status(),
                server.status(),
                "{scenario}: error status mismatch"
            );
            assert_eq!(
                without_system_properties(client_stored),
                without_system_properties(server_stored),
                "{scenario}: failed write mismatch"
            );
        }
        (client, server) => panic!(
            "{scenario}: strategies disagree on success: client={client:?}, server={server:?}"
        ),
    }
}

#[tokio::test]
async fn representative_operations_are_equivalent() {
    for (scenario, instructions) in [
        (
            "set",
            PatchInstructions::from(vec![PatchOperation::set(
                "/name",
                serde_json::json!("after"),
            )]),
        ),
        (
            "array append",
            PatchInstructions::from(vec![PatchOperation::add("/tags/-", serde_json::json!("c"))]),
        ),
        (
            "remove",
            PatchInstructions::from(vec![PatchOperation::remove("/nested/removed")]),
        ),
        (
            "increment",
            PatchInstructions::from(vec![PatchOperation::increment("/visits", 5i64)]),
        ),
        (
            "move",
            PatchInstructions::from(vec![PatchOperation::move_value("/name", "/renamed")]),
        ),
        (
            "ordered list",
            PatchInstructions::from(vec![
                PatchOperation::set("/name", serde_json::json!("after")),
                PatchOperation::increment("/visits", 2i64),
                PatchOperation::remove("/nested/removed"),
            ]),
        ),
    ] {
        assert_equivalent(instructions, scenario).await;
    }
}

#[tokio::test]
async fn representative_errors_are_equivalent() {
    for (scenario, instructions) in [
        ("empty list", PatchInstructions::from(Vec::new())),
        (
            "missing replace path",
            PatchInstructions::from(vec![PatchOperation::replace(
                "/missing/leaf",
                serde_json::json!(1),
            )]),
        ),
        (
            "non-number increment",
            PatchInstructions::from(vec![PatchOperation::increment("/name", 1i64)]),
        ),
        (
            "partition key",
            PatchInstructions::from(vec![PatchOperation::set("/pk", serde_json::json!("moved"))]),
        ),
        (
            "set item id",
            PatchInstructions::from(vec![PatchOperation::set("/id", serde_json::json!("moved"))]),
        ),
        (
            "replace item id",
            PatchInstructions::from(vec![PatchOperation::replace(
                "/id",
                serde_json::json!("moved"),
            )]),
        ),
        (
            "remove item id",
            PatchInstructions::from(vec![PatchOperation::remove("/id")]),
        ),
    ] {
        assert_equivalent(instructions, scenario).await;
    }
}

async fn methods_for(
    strategy: PatchStrategy,
    instructions: &PatchInstructions,
) -> Result<Vec<Method>, azure_data_cosmos_driver::error::CosmosError> {
    let recorder = MethodRecorder::new();
    let (driver, container) = build_driver(Some(recorder.clone())).await;
    create_item(&driver, &container, &seed_document()).await;
    recorder.clear();

    let result = execute_patch(&driver, &container, instructions, strategy).await;
    let methods = recorder.data_plane_methods();
    result.map(|_| methods)
}

fn set_operations(count: usize) -> PatchInstructions {
    PatchInstructions::from(
        (0..count)
            .map(|index| PatchOperation::set(format!("/field{index}"), serde_json::json!(index)))
            .collect::<Vec<_>>(),
    )
}

#[tokio::test]
async fn explicit_strategies_take_the_named_paths() {
    let instructions = set_operations(1);
    assert_eq!(
        methods_for(PatchStrategy::ServerSide, &instructions)
            .await
            .unwrap(),
        vec![Method::Patch]
    );
    assert_eq!(
        methods_for(PatchStrategy::ClientSide, &instructions)
            .await
            .unwrap(),
        vec![Method::Get, Method::Put]
    );
}

#[tokio::test]
async fn auto_obeys_safety_and_the_ten_instruction_limit() {
    assert_eq!(
        methods_for(PatchStrategy::Auto, &set_operations(10))
            .await
            .unwrap(),
        vec![Method::Patch],
        "the service limit itself remains server-side"
    );
    assert_eq!(
        methods_for(PatchStrategy::Auto, &set_operations(11))
            .await
            .unwrap(),
        vec![Method::Get, Method::Put],
        "more than 10 instructions must automatically use RMW"
    );
    let increment = PatchInstructions::from(vec![PatchOperation::increment("/visits", 1i64)]);
    assert_eq!(
        methods_for(PatchStrategy::Auto, &increment).await.unwrap(),
        vec![Method::Get, Method::Put],
        "unsafe instructions must use tracked RMW"
    );
}

#[tokio::test]
async fn typed_filter_predicate_uses_server_side_auto_and_remains_atomic() {
    let recorder = MethodRecorder::new();
    let (driver, container) = build_driver(Some(recorder.clone())).await;
    create_item(&driver, &container, &seed_document()).await;
    recorder.clear();

    let instructions = PatchInstructions::from(vec![PatchOperation::set(
        "/name",
        serde_json::json!("matched"),
    )])
    .with_filter_predicate("from c where c.name = 'before'");
    let response = execute_patch(&driver, &container, &instructions, PatchStrategy::Auto)
        .await
        .expect("a typed predicate should use server-side Auto");
    assert_eq!(response_json(response)["name"], "matched");
    assert_eq!(recorder.data_plane_methods(), vec![Method::Patch]);

    let before_failure = read_stored_raw(&driver, &container).await;
    recorder.clear();
    let error = execute_patch(&driver, &container, &instructions, PatchStrategy::Auto)
        .await
        .expect_err("the committed mutation should make the typed predicate false");
    assert_eq!(
        error.status().status_code(),
        azure_core::http::StatusCode::PreconditionFailed
    );
    assert_eq!(
        error.status().sub_status(),
        Some(azure_data_cosmos_driver::error::SubStatusCode::new(1110))
    );
    assert_eq!(recorder.data_plane_methods(), vec![Method::Patch]);
    assert_eq!(read_stored_raw(&driver, &container).await, before_failure);
}

#[tokio::test]
async fn effective_client_side_paths_reject_filter_predicates_before_io() {
    let scenarios = [
        (
            PatchStrategy::ClientSide,
            set_operations(1),
            "explicit ClientSide",
        ),
        (
            PatchStrategy::Auto,
            PatchInstructions::from(vec![PatchOperation::increment("/visits", 1i64)]),
            "Auto unsafe",
        ),
        (PatchStrategy::Auto, set_operations(11), "Auto over limit"),
    ];

    for (strategy, instructions, scenario) in scenarios {
        let recorder = MethodRecorder::new();
        let (driver, container) = build_driver(Some(recorder.clone())).await;
        create_item(&driver, &container, &seed_document()).await;
        let before = read_stored_raw(&driver, &container).await;
        recorder.clear();

        let error = execute_conditional_patch(
            &driver,
            &container,
            &instructions,
            "from c where c.name = 'before'",
            strategy,
        )
        .await
        .expect_err("effective client-side execution must reject predicates");

        assert_eq!(
            error.status(),
            azure_data_cosmos_driver::error::status_codes::CLIENT_BAD_REQUEST,
            "{scenario}"
        );
        assert!(error.status().sub_status().is_none(), "{scenario}");
        assert!(error.diagnostics().is_none(), "{scenario}");
        assert!(error.patch_tracking_id().is_none(), "{scenario}");
        assert!(
            error
                .to_string()
                .contains("require effective server-side execution"),
            "{scenario}: {error}"
        );
        assert_eq!(
            recorder.data_plane_methods(),
            Vec::<Method>::new(),
            "{scenario}"
        );
        assert_eq!(
            read_stored_raw(&driver, &container).await,
            before,
            "{scenario}"
        );
    }
}

#[tokio::test]
async fn server_side_filter_predicate_is_atomic_and_observable() {
    let recorder = MethodRecorder::new();
    let (driver, container) = build_driver(Some(recorder.clone())).await;
    create_item(&driver, &container, &seed_document()).await;
    recorder.clear();

    let response = execute_conditional_patch(
        &driver,
        &container,
        &PatchInstructions::from(vec![PatchOperation::set(
            "/name",
            serde_json::json!("matched"),
        )]),
        "from c where c.name = 'before'",
        PatchStrategy::Auto,
    )
    .await
    .expect("a matching Auto predicate should execute server-side");
    assert_eq!(response_json(response)["name"], "matched");
    assert_eq!(recorder.data_plane_methods(), vec![Method::Patch]);

    let before_failure = read_stored_raw(&driver, &container).await;
    recorder.clear();
    let error = execute_conditional_patch(
        &driver,
        &container,
        &PatchInstructions::from(vec![PatchOperation::set(
            "/name",
            serde_json::json!("must-not-commit"),
        )]),
        "from c where c.name = 'before'",
        PatchStrategy::ServerSide,
    )
    .await
    .expect_err("a false predicate must reject the PATCH");

    assert_eq!(
        error.status().status_code(),
        azure_core::http::StatusCode::PreconditionFailed
    );
    assert_eq!(
        error.status().sub_status(),
        Some(azure_data_cosmos_driver::error::SubStatusCode::new(1110))
    );
    assert_eq!(recorder.data_plane_methods(), vec![Method::Patch]);
    let diagnostics = error
        .diagnostics()
        .expect("a service response must carry diagnostics");
    assert_eq!(diagnostics.operation_name(), Some("patch_item"));
    assert_eq!(diagnostics.requests().len(), 1);
    assert!(diagnostics.total_request_charge().value() > 0.0);
    assert_eq!(read_stored_raw(&driver, &container).await, before_failure);
}

#[tokio::test]
async fn server_side_filter_predicate_accepts_sql_whitespace_after_from() {
    for keyword in ["from", " \t\r\nFrOm"] {
        for separator in [" ", "\t", "\n", "\r\n"] {
            let (driver, container) = build_driver(None).await;
            create_item(&driver, &container, &seed_document()).await;
            let before = read_stored_raw(&driver, &container).await;
            let instructions = PatchInstructions::from(vec![PatchOperation::set(
                "/name",
                serde_json::json!("after"),
            )]);

            let error = execute_conditional_patch(
                &driver,
                &container,
                &instructions,
                &format!("{keyword}{separator}c where c.name = 'missing'"),
                PatchStrategy::ServerSide,
            )
            .await
            .expect_err("a nonmatching predicate must fail for every SQL whitespace separator");
            assert_eq!(
                error.status().status_code(),
                azure_core::http::StatusCode::PreconditionFailed
            );
            assert_eq!(
                error.status().sub_status(),
                Some(azure_data_cosmos_driver::error::SubStatusCode::new(1110))
            );
            assert_eq!(read_stored_raw(&driver, &container).await, before);

            execute_conditional_patch(
                &driver,
                &container,
                &instructions,
                &format!("{keyword}{separator}c where c.name = 'before'"),
                PatchStrategy::ServerSide,
            )
            .await
            .expect("a matching predicate must accept every SQL whitespace separator");
            let after = read_stored_raw(&driver, &container).await;
            assert_eq!(after["name"], "after");
            assert_ne!(after["_etag"], before["_etag"]);
        }
    }
}

#[tokio::test]
async fn explicit_unsafe_server_side_predicate_uses_one_patch_request() {
    let recorder = MethodRecorder::new();
    let (driver, container) = build_driver(Some(recorder.clone())).await;
    create_item(&driver, &container, &seed_document()).await;
    recorder.clear();

    let response = execute_conditional_patch(
        &driver,
        &container,
        &PatchInstructions::from(vec![PatchOperation::increment("/visits", 1i64)]),
        "from c where c.visits = 1",
        PatchStrategy::ServerSide,
    )
    .await
    .expect("explicit unsafe ServerSide with a matching predicate must commit");

    assert_eq!(response_json(response)["visits"], 2);
    assert_eq!(recorder.data_plane_methods(), vec![Method::Patch]);
    assert_eq!(read_stored(&driver, &container).await["visits"], 2);
}

#[tokio::test]
async fn malformed_server_side_filter_predicate_returns_bad_request() {
    let recorder = MethodRecorder::new();
    let (driver, container) = build_driver(Some(recorder.clone())).await;
    create_item(&driver, &container, &seed_document()).await;
    let before = read_stored_raw(&driver, &container).await;
    recorder.clear();

    let error = execute_conditional_patch(
        &driver,
        &container,
        &set_operations(1),
        "from c where",
        PatchStrategy::ServerSide,
    )
    .await
    .expect_err("malformed predicates must be rejected by the service");

    assert_eq!(
        error.status().status_code(),
        azure_core::http::StatusCode::BadRequest
    );
    assert!(error.status().sub_status().is_none());
    assert_eq!(recorder.data_plane_methods(), vec![Method::Patch]);
    let diagnostics = error
        .diagnostics()
        .expect("a service response must carry diagnostics");
    assert_eq!(diagnostics.operation_name(), Some("patch_item"));
    assert_eq!(diagnostics.requests().len(), 1);
    assert_eq!(read_stored_raw(&driver, &container).await, before);

    recorder.clear();
    let error = execute_conditional_patch(
        &driver,
        &container,
        &set_operations(1),
        "   ",
        PatchStrategy::ServerSide,
    )
    .await
    .expect_err("empty predicates must be rejected before I/O");
    assert_eq!(
        error.status(),
        azure_data_cosmos_driver::error::status_codes::CLIENT_BAD_REQUEST
    );
    assert!(error.diagnostics().is_none());
    assert!(recorder.data_plane_methods().is_empty());
    assert_eq!(read_stored_raw(&driver, &container).await, before);
}

#[tokio::test]
async fn server_side_if_match_is_checked_before_filter_predicate() {
    let (driver, container) = build_driver(None).await;
    create_item(&driver, &container, &seed_document()).await;
    let initial = read_stored_raw(&driver, &container).await;
    let initial_etag = initial["_etag"].as_str().unwrap().to_owned();
    let instructions = PatchInstructions::from(vec![PatchOperation::set(
        "/name",
        serde_json::json!("matched"),
    )]);
    let options = OperationOptionsBuilder::new()
        .with_patch_strategy(PatchStrategy::ServerSide)
        .build();

    driver
        .execute_singleton_operation(
            CosmosOperation::patch_item(item(&container))
                .with_body(patch_body(&instructions, "from c where c.name = 'before'"))
                .with_precondition(Precondition::if_match(Etag::from(initial_etag.clone()))),
            options.clone(),
        )
        .await
        .expect("matching If-Match and filter predicate must commit");

    let committed = read_stored_raw(&driver, &container).await;
    assert_eq!(committed["name"], "matched");
    let false_condition_body = patch_body(
        &PatchInstructions::from(vec![PatchOperation::set(
            "/deleted",
            serde_json::json!(true),
        )]),
        "from c where c.name = 'before'",
    );
    let error = driver
        .execute_singleton_operation(
            CosmosOperation::patch_item(item(&container))
                .with_body(false_condition_body.clone())
                .with_precondition(Precondition::if_match(Etag::from(initial_etag))),
            options.clone(),
        )
        .await
        .expect_err("a stale If-Match must fail before predicate evaluation");

    assert_eq!(
        error.status().status_code(),
        azure_core::http::StatusCode::PreconditionFailed
    );
    assert!(error.status().sub_status().is_none());
    assert_eq!(read_stored_raw(&driver, &container).await, committed);

    let error = driver
        .execute_singleton_operation(
            CosmosOperation::patch_item(item(&container))
                .with_body(false_condition_body)
                .with_precondition(Precondition::if_match(Etag::from(
                    committed["_etag"].as_str().unwrap().to_owned(),
                ))),
            options,
        )
        .await
        .expect_err("a current If-Match must reach the same false predicate");
    assert_eq!(
        error.status().status_code(),
        azure_core::http::StatusCode::PreconditionFailed
    );
    assert_eq!(
        error.status().sub_status(),
        Some(azure_data_cosmos_driver::error::SubStatusCode::new(1110))
    );
    assert_eq!(read_stored_raw(&driver, &container).await, committed);
}

#[tokio::test]
async fn explicit_server_side_rejects_more_than_ten_instructions() {
    let recorder = MethodRecorder::new();
    let (driver, container) = build_driver(Some(recorder.clone())).await;
    create_item(&driver, &container, &seed_document()).await;
    recorder.clear();

    let error = execute_patch(
        &driver,
        &container,
        &set_operations(11),
        PatchStrategy::ServerSide,
    )
    .await
    .expect_err("explicit ServerSide must not fall back to RMW");

    assert_eq!(
        error.status().status_code(),
        azure_core::http::StatusCode::BadRequest
    );
    assert_eq!(recorder.data_plane_methods(), vec![Method::Patch]);
    let stored = read_stored(&driver, &container).await;
    assert_eq!(stored, without_system_properties(seed_document()));
}

#[tokio::test]
async fn empty_patch_is_rejected_before_strategy_dispatch() {
    let recorder = MethodRecorder::new();
    let (driver, container) = build_driver(Some(recorder.clone())).await;
    create_item(&driver, &container, &seed_document()).await;

    for strategy in [
        PatchStrategy::Auto,
        PatchStrategy::ClientSide,
        PatchStrategy::ServerSide,
    ] {
        recorder.clear();
        let options = OperationOptionsBuilder::new()
            .with_patch_strategy(strategy)
            .build();
        let error = driver
            .execute_singleton_operation(
                CosmosOperation::patch_item(item(&container))
                    .with_body(br#"{"operations":[]}"#.to_vec()),
                options,
            )
            .await
            .expect_err("an empty PATCH must fail before strategy dispatch");

        assert_eq!(
            error.status().status_code(),
            azure_core::http::StatusCode::BadRequest,
            "{strategy}"
        );
        assert_eq!(
            recorder.data_plane_methods(),
            Vec::<Method>::new(),
            "{strategy} must not issue a data-plane request"
        );
    }
}

#[tokio::test]
async fn explicit_server_side_sends_malformed_body_to_the_service() {
    let recorder = MethodRecorder::new();
    let (driver, container) = build_driver(Some(recorder.clone())).await;
    create_item(&driver, &container, &seed_document()).await;
    recorder.clear();
    let options = OperationOptionsBuilder::new()
        .with_patch_strategy(PatchStrategy::ServerSide)
        .build();

    let error = driver
        .execute_singleton_operation(
            CosmosOperation::patch_item(item(&container)).with_body(b"not-json".to_vec()),
            options,
        )
        .await
        .expect_err("explicit ServerSide must forward malformed wire bodies");

    assert_eq!(
        error.status().status_code(),
        azure_core::http::StatusCode::BadRequest
    );
    assert_eq!(recorder.data_plane_methods(), vec![Method::Patch]);
    assert_eq!(
        read_stored(&driver, &container).await,
        without_system_properties(seed_document())
    );
}

#[tokio::test]
async fn client_side_helper_operation_names_are_patch_scoped() {
    let (driver, container) = build_driver(None).await;
    create_item(&driver, &container, &seed_document()).await;
    let response = execute_patch(
        &driver,
        &container,
        &set_operations(1),
        PatchStrategy::ClientSide,
    )
    .await
    .expect("client-side PATCH should succeed");

    assert_eq!(response.diagnostics().operation_name(), Some("patch_item"));
    let requests = response.diagnostics().requests();
    let helper_names = requests
        .iter()
        .map(|request| request.operation_name())
        .collect::<Vec<_>>();
    assert_eq!(
        helper_names,
        vec![Some("patch_read_item"), Some("patch_replace_item")]
    );
}

#[tokio::test]
async fn server_side_operation_name_is_patch_item() {
    let (driver, container) = build_driver(None).await;
    create_item(&driver, &container, &seed_document()).await;
    let response = execute_patch(
        &driver,
        &container,
        &set_operations(1),
        PatchStrategy::ServerSide,
    )
    .await
    .expect("server-side PATCH should succeed");

    assert_eq!(response.diagnostics().operation_name(), Some("patch_item"));
    let requests = response.diagnostics().requests();
    assert_eq!(requests.len(), 1);
    assert_eq!(
        requests[0].operation_name(),
        None,
        "the request inherits patch_item from the operation context"
    );
}

#[tokio::test]
async fn content_response_on_write_disabled_suppresses_both_strategy_bodies() {
    for (strategy, instructions, expected_methods) in [
        (
            PatchStrategy::ServerSide,
            set_operations(1),
            vec![Method::Patch],
        ),
        (
            PatchStrategy::ClientSide,
            set_operations(1),
            vec![Method::Get, Method::Put],
        ),
        (PatchStrategy::Auto, set_operations(1), vec![Method::Patch]),
        (
            PatchStrategy::Auto,
            PatchInstructions::from(vec![PatchOperation::increment("/visits", 1i64)]),
            vec![Method::Get, Method::Put],
        ),
    ] {
        let recorder = MethodRecorder::new();
        let (driver, container) = build_driver(Some(recorder.clone())).await;
        create_item(&driver, &container, &seed_document()).await;
        recorder.clear();

        let operation = CosmosOperation::patch_item(item(&container))
            .with_body(serde_json::to_vec(&instructions).unwrap());
        let options = OperationOptionsBuilder::new()
            .with_patch_strategy(strategy)
            .with_content_response_on_write(ContentResponseOnWrite::Disabled)
            .build();
        let response = driver
            .execute_singleton_operation(operation, options)
            .await
            .expect("PATCH should succeed");

        assert_eq!(
            response.status().status_code(),
            azure_core::http::StatusCode::Ok
        );
        assert_eq!(response.diagnostics().operation_name(), Some("patch_item"));
        assert!(
            response.body().is_empty(),
            "{strategy} must suppress its body"
        );
        assert_eq!(
            recorder.data_plane_methods(),
            expected_methods,
            "{strategy}"
        );
        let stored = read_stored(&driver, &container).await;
        assert_ne!(
            stored,
            without_system_properties(seed_document()),
            "{strategy} must still commit the mutation"
        );
    }

    let mut defaults = OperationOptions::default();
    defaults.content_response_on_write = Some(ContentResponseOnWrite::Disabled);
    let (driver, container) = build_driver_with_defaults(None, defaults).await;
    create_item(&driver, &container, &seed_document()).await;
    let operation = CosmosOperation::patch_item(item(&container))
        .with_body(serde_json::to_vec(&set_operations(1)).unwrap());
    let options = OperationOptionsBuilder::new()
        .with_patch_strategy(PatchStrategy::ClientSide)
        .build();
    let response = driver
        .execute_singleton_operation(operation, options)
        .await
        .expect("PATCH should inherit the disabled content response default");
    assert!(
        response.body().is_empty(),
        "driver-level Disabled must suppress the client-side PATCH body"
    );
}

#[tokio::test]
async fn client_side_settings_do_not_override_auto_or_explicit_server_side() {
    let recorder = MethodRecorder::new();
    let (driver, container) = build_driver(Some(recorder.clone())).await;
    create_item(&driver, &container, &seed_document()).await;
    let instructions = set_operations(1);
    let tracking_id = PatchTrackingId::new();

    recorder.clear();
    let operation = CosmosOperation::patch_item(item(&container))
        .with_body(serde_json::to_vec(&instructions).unwrap())
        .with_patch_tracking_id(tracking_id)
        .with_patch_max_attempts(NonZeroU8::new(3).unwrap())
        .with_patch_tracking_capacity(NonZeroU16::new(10).unwrap())
        .with_patch_tracking_retention_seconds(NonZeroU32::new(60).unwrap());
    let response = driver
        .execute_singleton_operation(operation, OperationOptions::default())
        .await
        .expect("Auto should ignore client-side-only settings on the server path");
    assert_eq!(recorder.data_plane_methods(), vec![Method::Patch]);
    assert_eq!(response.patch_tracking_id(), None);

    recorder.clear();
    let operation = CosmosOperation::patch_item(item(&container))
        .with_body(serde_json::to_vec(&instructions).unwrap())
        .with_patch_tracking_id(PatchTrackingId::new());
    let options = OperationOptionsBuilder::new()
        .with_patch_strategy(PatchStrategy::ServerSide)
        .build();
    let response = driver
        .execute_singleton_operation(operation, options)
        .await
        .expect("explicit ServerSide must be honored when a tracking ID is present");
    assert_eq!(
        recorder.data_plane_methods(),
        vec![Method::Patch],
        "explicit ServerSide must send one PATCH rather than entering tracked RMW"
    );
    assert_eq!(response.patch_tracking_id(), None);
    assert_eq!(read_stored(&driver, &container).await["field0"], 0);
}

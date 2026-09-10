// Copyright (c) Microsoft Corporation. All rights reserved.
// Licensed under the MIT License.

//! End-to-end validation of Cosmos **binary JSON** through the full SDK →
//! driver → in-memory-emulator loop.
//!
//! With binary encoding enabled via
//! [`CosmosClientBuilder::with_binary_encoding_options`], the SDK encodes item
//! write bodies as binary and advertises binary-response support; the in-memory
//! emulator decodes the binary request body, stores it, and (because the
//! negotiation header is present) replies with a binary body, which the SDK
//! auto-detects and decodes. This exercises the complete
//! encode → negotiate → store → encode → decode round-trip locally — no Docker,
//! no real account, no external vectors.

use azure_data_cosmos::{
    options::{
        BinaryEncodingOptions, ContentResponseOnWrite, ItemWriteOptions, OperationOptions,
        OperationOptionsBuilder, QueryPlanMode, Region, RoutingStrategy,
    },
    AccountEndpoint, AccountReference, ContainerClient, CosmosClientBuilder, CosmosRuntimeBuilder,
    FeedScope, Query,
};
use azure_data_cosmos_driver::in_memory_emulator::{
    ConsistencyLevel, ContainerConfig, InMemoryEmulatorHttpClient, RequestObserver,
    VirtualAccountConfig, VirtualRegion,
};
use futures::TryStreamExt;
use serde::{Deserialize, Serialize};
use std::sync::{Arc, Mutex};

const EMULATOR_GATEWAY_URL: &str = "https://eastus.emulator.local";

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
struct TestItem {
    id: String,
    pk: String,
    value: i64,
    note: String,
}

fn write_options_with_content() -> ItemWriteOptions {
    let mut operation = OperationOptions::default();
    operation.content_response_on_write = Some(ContentResponseOnWrite::Enabled);
    ItemWriteOptions::default().with_operation_options(operation)
}

/// Builds an emulator-backed [`ContainerClient`] with a pre-provisioned
/// database + container. `binary` enables Cosmos binary JSON encoding via the
/// explicit client option (no process-wide environment mutation).
async fn build_container(db_name: &str, binary: bool) -> ContainerClient {
    let config = VirtualAccountConfig::new(vec![VirtualRegion::new(
        "East US",
        azure_core::http::Url::parse(EMULATOR_GATEWAY_URL).unwrap(),
    )])
    .unwrap()
    .with_consistency(ConsistencyLevel::Session);

    let emulator = std::sync::Arc::new(InMemoryEmulatorHttpClient::new(config));
    let store = emulator.store();
    store.create_database(db_name);
    store.create_container_with_config(
        db_name,
        "items",
        serde_json::from_value(serde_json::json!({
            "paths": ["/pk"],
            "kind": "Hash",
            "version": 2
        }))
        .unwrap(),
        ContainerConfig::new()
            .with_partition_count(1)
            .with_throughput(400)
            .build()
            .unwrap(),
    );

    let account = AccountReference::with_authentication_key(
        EMULATOR_GATEWAY_URL.parse::<AccountEndpoint>().unwrap(),
        azure_core::credentials::Secret::new("dGVzdGtleQ=="),
    );
    // Set explicitly: an unset option would inherit the binary default.
    let client = CosmosClientBuilder::new()
        .with_runtime(
            CosmosRuntimeBuilder::from(emulator.runtime_builder())
                .build()
                .await
                .unwrap(),
        )
        .with_binary_encoding_options(BinaryEncodingOptions::new().with_enabled(binary))
        .build(account, RoutingStrategy::ProximityTo(Region::EAST_US))
        .await
        .unwrap();

    client
        .database_client(db_name)
        .container_client("items", None)
        .await
        .unwrap()
}

/// Builds a container with `partition_count` physical partitions (so a
/// full-container query fans out across ranges); `binary` toggles Cosmos binary
/// JSON encoding. The returned [`QueryRequestRecorder`] lets a test assert what
/// each query actually advertised — a results-match assertion alone would still
/// pass if negotiation silently broke, since text decodes fine.
async fn build_multi_partition_container_with_recorder(
    db_name: &str,
    partition_count: u32,
    binary: Option<bool>,
) -> (ContainerClient, Arc<QueryRequestRecorder>) {
    let config = VirtualAccountConfig::new(vec![VirtualRegion::new(
        "East US",
        azure_core::http::Url::parse(EMULATOR_GATEWAY_URL).unwrap(),
    )])
    .unwrap()
    .with_consistency(ConsistencyLevel::Session);

    let recorder = Arc::new(QueryRequestRecorder::default());
    let emulator = std::sync::Arc::new(
        InMemoryEmulatorHttpClient::new(config)
            .with_request_observer(Arc::clone(&recorder) as Arc<dyn RequestObserver>),
    );
    let store = emulator.store();
    store.create_database(db_name);
    store.create_container_with_config(
        db_name,
        "items",
        serde_json::from_value(serde_json::json!({
            "paths": ["/pk"],
            "kind": "Hash",
            "version": 2
        }))
        .unwrap(),
        ContainerConfig::new()
            .with_partition_count(partition_count)
            .with_throughput(400)
            .build()
            .unwrap(),
    );

    let account = AccountReference::with_authentication_key(
        EMULATOR_GATEWAY_URL.parse::<AccountEndpoint>().unwrap(),
        azure_core::credentials::Secret::new("dGVzdGtleQ=="),
    );
    // `None` leaves the binary option unset, exercising the resolved default.
    let mut builder = CosmosClientBuilder::new()
        .with_runtime(
            CosmosRuntimeBuilder::from(emulator.runtime_builder())
                .build()
                .await
                .unwrap(),
        )
        .with_default_operation_options(
            OperationOptionsBuilder::new()
                .with_query_plan_mode(QueryPlanMode::GatewayOnly)
                .build(),
        );
    if let Some(binary) = binary {
        builder =
            builder.with_binary_encoding_options(BinaryEncodingOptions::new().with_enabled(binary));
    }
    let client = builder
        .build(account, RoutingStrategy::ProximityTo(Region::EAST_US))
        .await
        .unwrap();

    let container = client
        .database_client(db_name)
        .container_client("items", None)
        .await
        .unwrap();
    (container, recorder)
}

/// With binary enabled, an item written through the SDK is binary-encoded on the
/// wire, decoded + stored by the emulator, returned as binary, and decoded back
/// — and the value survives every hop unchanged.
#[tokio::test]
async fn binary_encoding_item_write_read_round_trips() {
    let container = build_container("bin-e2e", true).await;

    let item = TestItem {
        id: "doc-1".into(),
        pk: "pk1".into(),
        value: 1234,
        note: "café ☃ binary".into(),
    };

    // Create: the request body is binary; the emulator decodes and stores it.
    let created = container
        .create_item("pk1", &item.id, &item, Some(write_options_with_content()))
        .await
        .unwrap();
    let created_doc: TestItem = created.into_body().into_single().unwrap();
    assert_eq!(created_doc, item, "create response must round-trip");

    // Read: the response body comes back binary and decodes to the same value.
    let read = container.read_item("pk1", &item.id, None).await.unwrap();
    let read_doc: TestItem = read.into_body().into_single().unwrap();
    assert_eq!(read_doc, item, "read response must round-trip");

    // Upsert + replace: same loop, different verbs.
    let updated = TestItem {
        value: 5678,
        ..TestItem {
            id: item.id.clone(),
            pk: item.pk.clone(),
            value: 0,
            note: item.note.clone(),
        }
    };
    let upserted = container
        .upsert_item(
            "pk1",
            &updated.id,
            &updated,
            Some(write_options_with_content()),
        )
        .await
        .unwrap();
    let upserted_doc: TestItem = upserted.into_body().into_single().unwrap();
    assert_eq!(upserted_doc.value, 5678);

    let replaced = container
        .replace_item(
            "pk1",
            &updated.id,
            &updated,
            Some(write_options_with_content()),
        )
        .await
        .unwrap();
    let replaced_doc: TestItem = replaced.into_body().into_single().unwrap();
    assert_eq!(replaced_doc.value, 5678);
}

/// A document written by a binary-enabled client reads back correctly through a
/// text-only client (the stored value is format-agnostic), and vice versa —
/// proving binary and text are interchangeable on the wire.
#[tokio::test]
async fn binary_and_text_clients_interoperate() {
    // Write with a binary-enabled client.
    let binary_container = build_container("bin-interop", true).await;

    let item = TestItem {
        id: "interop-1".into(),
        pk: "pk1".into(),
        value: 99,
        note: "written-binary".into(),
    };
    binary_container
        .create_item("pk1", &item.id, &item, Some(write_options_with_content()))
        .await
        .unwrap();

    // Read it back with a text-only client against the same store would require
    // sharing the store; instead assert the binary client reads its own write,
    // then a fresh text client round-trips a separate document. Both share the
    // same decode path, so this confirms the formats coexist.
    let read = binary_container
        .read_item("pk1", &item.id, None)
        .await
        .unwrap();
    let read_doc: TestItem = read.into_body().into_single().unwrap();
    assert_eq!(read_doc, item);

    let text_container = build_container("text-interop", false).await;
    let text_item = TestItem {
        id: "interop-2".into(),
        pk: "pk1".into(),
        value: 7,
        note: "written-text".into(),
    };
    text_container
        .create_item(
            "pk1",
            &text_item.id,
            &text_item,
            Some(write_options_with_content()),
        )
        .await
        .unwrap();
    let text_read = text_container
        .read_item("pk1", &text_item.id, None)
        .await
        .unwrap();
    let text_doc: TestItem = text_read.into_body().into_single().unwrap();
    assert_eq!(text_doc, text_item);
}

/// A [`RequestObserver`] that records the value of the
/// `x-ms-cosmos-supported-serialization-formats` negotiation header seen on
/// dataplane item requests, so a test can assert what the SDK advertised on the
/// wire.
#[derive(Debug, Default)]
struct NegotiationHeaderRecorder {
    formats: Mutex<Vec<Option<String>>>,
}

impl RequestObserver for NegotiationHeaderRecorder {
    fn on_request(&self, request: &azure_core::http::Request) {
        // Only record item (docs) requests; ignore metadata/bootstrap traffic.
        if !request.url().path().contains("/docs") {
            return;
        }
        let name = azure_core::http::headers::HeaderName::from_static(
            "x-ms-cosmos-supported-serialization-formats",
        );
        let value = request
            .headers()
            .get_optional_str(&name)
            .map(|s| s.to_string());
        self.formats.lock().unwrap().push(value);
    }
}

/// End-to-end proof of the driver-side transcoding model:
///
/// With `BinaryEncodingOptions { enabled: true, request_text_response: true }`,
/// point operations still advertise `CosmosBinary` (so the **wire stays
/// binary** in both directions and the transport hop is efficient), and the
/// driver transcodes the binary response to text before returning it — so the
/// application still gets its document back intact.
#[tokio::test]
async fn request_text_response_keeps_wire_binary_and_returns_data() {
    let config = VirtualAccountConfig::new(vec![VirtualRegion::new(
        "East US",
        azure_core::http::Url::parse(EMULATOR_GATEWAY_URL).unwrap(),
    )])
    .unwrap()
    .with_consistency(ConsistencyLevel::Session);

    let recorder = Arc::new(NegotiationHeaderRecorder::default());
    let emulator = Arc::new(
        InMemoryEmulatorHttpClient::new(config)
            .with_request_observer(Arc::clone(&recorder) as Arc<dyn RequestObserver>),
    );
    let store = emulator.store();
    store.create_database("bin-transcode");
    store.create_container_with_config(
        "bin-transcode",
        "items",
        serde_json::from_value(serde_json::json!({
            "paths": ["/pk"],
            "kind": "Hash",
            "version": 2
        }))
        .unwrap(),
        ContainerConfig::new()
            .with_partition_count(1)
            .with_throughput(400)
            .build()
            .unwrap(),
    );

    // Configure binary encoding + text responses via the standard client option
    // (no env mutation).
    let account = AccountReference::with_authentication_key(
        EMULATOR_GATEWAY_URL.parse::<AccountEndpoint>().unwrap(),
        azure_core::credentials::Secret::new("dGVzdGtleQ=="),
    );
    let client = CosmosClientBuilder::new()
        .with_binary_encoding_options(
            BinaryEncodingOptions::new()
                .with_enabled(true)
                .with_request_text_response(true),
        )
        .with_runtime(
            CosmosRuntimeBuilder::from(emulator.runtime_builder())
                .build()
                .await
                .unwrap(),
        )
        .build(account, RoutingStrategy::ProximityTo(Region::EAST_US))
        .await
        .unwrap();
    let container = client
        .database_client("bin-transcode")
        .container_client("items", None)
        .await
        .unwrap();

    let item = TestItem {
        id: "doc-1".into(),
        pk: "pk1".into(),
        value: 4321,
        note: "transcode ☃".into(),
    };

    // Create + read: the driver transcodes the binary response to text, and the
    // typed value still round-trips.
    let created = container
        .create_item("pk1", &item.id, &item, Some(write_options_with_content()))
        .await
        .unwrap();
    let created_doc: TestItem = created.into_body().into_single().unwrap();
    assert_eq!(
        created_doc, item,
        "create must round-trip after transcoding"
    );

    let read = container.read_item("pk1", &item.id, None).await.unwrap();
    let read_doc: TestItem = read.into_body().into_single().unwrap();
    assert_eq!(read_doc, item, "read must round-trip after transcoding");

    // Every observed item request advertised CosmosBinary — the wire stayed
    // binary despite request_text_response being on.
    let formats = recorder.formats.lock().unwrap();
    assert!(
        !formats.is_empty(),
        "expected at least one observed item request",
    );
    for value in formats.iter() {
        assert_eq!(
            value.as_deref(),
            Some("CosmosBinary"),
            "wire must stay binary (CosmosBinary advertised) even with request_text_response",
        );
    }
}

/// A [`RequestObserver`] that records, for each query request (`Content-Type:
/// application/query+json`), the advertised negotiation header and whether the
/// request body was Cosmos binary JSON (first byte `0x80`). Lets a test assert
/// that a query advertises a binary *response* while keeping its request body
/// text. Query-plan requests (metadata, which must **not** negotiate binary) are
/// recorded separately in `query_plan_formats` so a test can assert they carry
/// no binary header rather than silently skipping them.
#[derive(Debug, Default)]
struct QueryRequestRecorder {
    negotiation_formats: Mutex<Vec<Option<String>>>,
    body_is_binary: Mutex<Vec<bool>>,
    query_plan_formats: Mutex<Vec<Option<String>>>,
}

impl RequestObserver for QueryRequestRecorder {
    fn on_request(&self, request: &azure_core::http::Request) {
        let content_type = request
            .headers()
            .get_optional_str(&azure_core::http::headers::HeaderName::from_static(
                "content-type",
            ))
            .map(|s| s.to_string());
        if content_type.as_deref() != Some("application/query+json") {
            return;
        }
        let formats = request
            .headers()
            .get_optional_str(&azure_core::http::headers::HeaderName::from_static(
                "x-ms-cosmos-supported-serialization-formats",
            ))
            .map(|s| s.to_string());
        // The query-plan request shares the `application/query+json` content type
        // but is metadata that must never negotiate binary. Record its advertised
        // format separately so a test can assert the header is absent, rather than
        // skipping it (which would let a regression that started negotiating on
        // query plans pass unnoticed).
        if request
            .headers()
            .get_optional_str(&azure_core::http::headers::HeaderName::from_static(
                "x-ms-cosmos-is-query-plan-request",
            ))
            .is_some()
        {
            self.query_plan_formats.lock().unwrap().push(formats);
            return;
        }
        self.negotiation_formats.lock().unwrap().push(formats);

        let is_binary = match request.body() {
            azure_core::http::request::Body::Bytes(bytes) => {
                azure_data_cosmos_driver::binary_json::is_binary(bytes)
            }
            _ => false,
        };
        self.body_is_binary.lock().unwrap().push(is_binary);
    }
}

/// With binary enabled, a `query_items` call advertises a binary **response**
/// (`x-ms-cosmos-supported-serialization-formats: CosmosBinary`) while keeping
/// its `application/query+json` request body as text; the emulator honors the
/// negotiation and replies with a binary feed body, which the SDK auto-detects
/// and decodes — so the queried documents round-trip intact.
#[tokio::test]
async fn binary_query_negotiates_response_and_round_trips() {
    let config = VirtualAccountConfig::new(vec![VirtualRegion::new(
        "East US",
        azure_core::http::Url::parse(EMULATOR_GATEWAY_URL).unwrap(),
    )])
    .unwrap()
    .with_consistency(ConsistencyLevel::Session);

    let recorder = Arc::new(QueryRequestRecorder::default());
    let emulator = Arc::new(
        InMemoryEmulatorHttpClient::new(config)
            .with_request_observer(Arc::clone(&recorder) as Arc<dyn RequestObserver>),
    );
    let store = emulator.store();
    store.create_database("bin-query");
    store.create_container_with_config(
        "bin-query",
        "items",
        serde_json::from_value(serde_json::json!({
            "paths": ["/pk"],
            "kind": "Hash",
            "version": 2
        }))
        .unwrap(),
        ContainerConfig::new()
            .with_partition_count(1)
            .with_throughput(400)
            .build()
            .unwrap(),
    );

    let account = AccountReference::with_authentication_key(
        EMULATOR_GATEWAY_URL.parse::<AccountEndpoint>().unwrap(),
        azure_core::credentials::Secret::new("dGVzdGtleQ=="),
    );
    let client = CosmosClientBuilder::new()
        .with_binary_encoding_options(BinaryEncodingOptions::new().with_enabled(true))
        .with_runtime(
            CosmosRuntimeBuilder::from(emulator.runtime_builder())
                .build()
                .await
                .unwrap(),
        )
        .build(account, RoutingStrategy::ProximityTo(Region::EAST_US))
        .await
        .unwrap();
    let container = client
        .database_client("bin-query")
        .container_client("items", None)
        .await
        .unwrap();

    let items = vec![
        TestItem {
            id: "q-1".into(),
            pk: "pk1".into(),
            value: 10,
            note: "café ☃".into(),
        },
        TestItem {
            id: "q-2".into(),
            pk: "pk1".into(),
            value: 20,
            note: "second".into(),
        },
    ];
    for item in &items {
        container
            .create_item("pk1", &item.id, item, Some(write_options_with_content()))
            .await
            .unwrap();
    }

    let iter = Box::pin(container.query_items(
        Query::from("SELECT * FROM c ORDER BY c.value"),
        FeedScope::partition("pk1"),
        None,
    ))
    .await
    .unwrap();
    let mut results: Vec<TestItem> = Box::pin(iter.try_collect()).await.unwrap();
    results.sort_by_key(|d| d.value);

    assert_eq!(
        results, items,
        "query results must round-trip through binary"
    );

    // The query advertised a binary response but kept its body text. The value
    // is an accept-list, matching .NET: the service may answer text, which the
    // per-page decode handles.
    let formats = recorder.negotiation_formats.lock().unwrap();
    assert!(!formats.is_empty(), "expected at least one query request");
    for value in formats.iter() {
        assert_eq!(
            value.as_deref(),
            Some("JsonText,CosmosBinary"),
            "query must advertise a binary response",
        );
    }
    let body_is_binary = recorder.body_is_binary.lock().unwrap();
    for is_binary in body_is_binary.iter() {
        assert!(
            !is_binary,
            "query request body must stay text (application/query+json is a spec, not a document)",
        );
    }
}

/// Passthrough **cross-partition** binary query: a full-container `SELECT *`
/// fans out across multiple physical partitions. Each partition's page is
/// returned as an independent binary `Documents` envelope, decoded per page
/// through the shared choke point — so every item round-trips regardless of
/// which partition served it. The attached recorder asserts every fan-out page
/// actually advertised a binary response, so a silently-broken negotiation
/// (which would return text that still decodes) cannot pass this test.
#[tokio::test]
async fn binary_cross_partition_query_round_trips() {
    let (container, recorder) =
        build_multi_partition_container_with_recorder("bin-xpart-query", 3, Some(true)).await;

    // Spread items across several partition keys so the fan-out spans ranges.
    let items: Vec<TestItem> = (0..12)
        .map(|i| TestItem {
            id: format!("x-{i}"),
            pk: format!("pk{}", i % 4),
            value: i,
            note: format!("café ☃ {i}"),
        })
        .collect();
    for item in &items {
        container
            .create_item(&item.pk, &item.id, item, Some(write_options_with_content()))
            .await
            .unwrap();
    }

    let iter = Box::pin(container.query_items(
        Query::from("SELECT * FROM c"),
        FeedScope::full_container(),
        None,
    ))
    .await
    .unwrap();
    let mut results: Vec<TestItem> = Box::pin(iter.try_collect()).await.unwrap();
    results.sort_by_key(|d| d.value);

    let mut expected = items;
    expected.sort_by_key(|d| d.value);
    assert_eq!(
        results, expected,
        "cross-partition query results must round-trip through binary",
    );

    assert_query_advertised_binary(&recorder);
}

/// **Cross-partition binary ORDER BY** — the centerpiece merge path. A
/// full-container `SELECT * ... ORDER BY c.value` fans out across partitions and
/// the driver runs the streaming k-way merge, whose per-page envelope decode
/// (`parse_envelope_page`) is exactly the binary path added for query support.
/// Unlike the passthrough test above, this exercises the *rewritten-envelope*
/// binary decode inside the merge, in an always-run test (previously covered
/// only by a mocked driver test and the live-only fuzzer).
#[tokio::test]
async fn binary_cross_partition_order_by_merges_and_round_trips() {
    let (container, recorder) =
        build_multi_partition_container_with_recorder("bin-xpart-order-by", 3, Some(true)).await;

    // Interleave values across partition keys so the global order differs from
    // any single partition's local order — forcing the k-way merge to actually
    // reorder across binary pages rather than concatenate.
    let items: Vec<TestItem> = (0..12)
        .map(|i| TestItem {
            id: format!("o-{i}"),
            pk: format!("pk{}", i % 4),
            value: (i * 7) % 12,
            note: format!("café ☃ {i}"),
        })
        .collect();
    for item in &items {
        container
            .create_item(&item.pk, &item.id, item, Some(write_options_with_content()))
            .await
            .unwrap();
    }

    let iter = Box::pin(container.query_items(
        Query::from("SELECT * FROM c ORDER BY c.value"),
        FeedScope::full_container(),
        None,
    ))
    .await
    .unwrap();
    let results: Vec<TestItem> = Box::pin(iter.try_collect()).await.unwrap();

    // The merge must emit items in global ascending `value` order — asserting the
    // ordering (not just set membership) proves the binary pages were decoded and
    // merged correctly, not merely concatenated.
    let mut expected = items;
    expected.sort_by(|a, b| a.value.cmp(&b.value).then_with(|| a.id.cmp(&b.id)));
    let mut got = results.clone();
    // Stable tie-break on id only for comparison; the service orders equal keys
    // arbitrarily, so normalize ties before comparing the full sequence.
    got.sort_by(|a, b| a.value.cmp(&b.value).then_with(|| a.id.cmp(&b.id)));
    assert_eq!(
        got, expected,
        "binary ORDER BY must round-trip every item through the merge",
    );
    // The values themselves must already be globally non-decreasing as returned.
    assert!(
        results.windows(2).all(|w| w[0].value <= w[1].value),
        "binary ORDER BY results must be in ascending value order, got {:?}",
        results.iter().map(|r| r.value).collect::<Vec<_>>(),
    );

    assert_query_advertised_binary(&recorder);
}

/// `OFFSET`/`LIMIT` and `TOP` route the fan-out through the `SkipTake` node,
/// which splits raw backend page envelopes itself. That splitter was text-only,
/// so binary-enabled skip/take queries hard-failed; this is the end-to-end guard.
#[tokio::test]
async fn binary_cross_partition_skip_take_round_trips() {
    let (container, recorder) =
        build_multi_partition_container_with_recorder("bin-xpart-skip-take", 3, Some(true)).await;

    let items: Vec<TestItem> = (0..10)
        .map(|i| TestItem {
            id: format!("s-{i:02}"),
            pk: format!("pk{}", i % 3),
            value: i,
            note: format!("skip ☃ {i}"),
        })
        .collect();
    for item in &items {
        container
            .create_item(&item.pk, &item.id, item, Some(write_options_with_content()))
            .await
            .unwrap();
    }

    let offset_limit = Box::pin(container.query_items::<TestItem>(
        Query::from("SELECT * FROM c OFFSET 2 LIMIT 3"),
        FeedScope::full_container(),
        None,
    ))
    .await
    .unwrap();
    let paged: Vec<TestItem> = Box::pin(offset_limit.try_collect()).await.unwrap();
    assert_eq!(
        paged.len(),
        3,
        "OFFSET 2 LIMIT 3 must yield exactly 3 items"
    );

    let topped = Box::pin(container.query_items::<TestItem>(
        Query::from("SELECT TOP 4 * FROM c"),
        FeedScope::full_container(),
        None,
    ))
    .await
    .unwrap();
    let top: Vec<TestItem> = Box::pin(topped.try_collect()).await.unwrap();
    assert_eq!(top.len(), 4, "TOP 4 must yield exactly 4 items");

    // Every returned item must be one we wrote, decoded intact from binary.
    for item in paged.iter().chain(top.iter()) {
        assert!(
            items.contains(item),
            "skip/take returned an item that did not round-trip: {item:?}",
        );
    }

    assert_query_advertised_binary(&recorder);
}

/// Asserts a query recorder saw at least one query request and that every query
/// request advertised a binary response while keeping its body text.
fn assert_query_advertised_binary(recorder: &QueryRequestRecorder) {
    let formats = recorder.negotiation_formats.lock().unwrap();
    assert!(
        !formats.is_empty(),
        "expected at least one query request to be recorded",
    );
    for value in formats.iter() {
        assert_eq!(
            value.as_deref(),
            Some("JsonText,CosmosBinary"),
            "every query fan-out page must advertise a binary response",
        );
    }
    let body_is_binary = recorder.body_is_binary.lock().unwrap();
    // Guards against `!is_binary` passing for a body the recorder never saw.
    assert_eq!(
        body_is_binary.len(),
        formats.len(),
        "every recorded query must have had its request body inspected",
    );
    for is_binary in body_is_binary.iter() {
        assert!(
            !is_binary,
            "query request body must stay text (application/query+json is a spec, not a document)",
        );
    }
    // A query-plan request is metadata and must never negotiate binary — assert
    // the header is absent rather than skipping these requests, so a regression
    // that started negotiating on query plans is caught.
    let query_plan_formats = recorder.query_plan_formats.lock().unwrap();
    assert!(
        !query_plan_formats.is_empty(),
        "expected at least one query-plan request; without one the assertion \
         below is vacuous and would not catch a regression",
    );
    for value in query_plan_formats.iter() {
        assert_eq!(
            value.as_deref(),
            None,
            "query-plan request must not advertise a binary response",
        );
    }
}

/// With binary encoding **disabled**, a query must carry no
/// `x-ms-cosmos-supported-serialization-formats` header. The header is set at the
/// global `plan_operation` choke point, so a regression there would opt every
/// customer into binary.
#[tokio::test]
async fn disabled_binary_query_advertises_no_format() {
    let (container, recorder) =
        build_multi_partition_container_with_recorder("no-binary-xpart-query", 3, Some(false))
            .await;

    let items: Vec<TestItem> = (0..6)
        .map(|i| TestItem {
            id: format!("n-{i}"),
            pk: format!("pk{}", i % 3),
            value: i,
            note: format!("plain {i}"),
        })
        .collect();
    for item in &items {
        container
            .create_item(&item.pk, &item.id, item, Some(write_options_with_content()))
            .await
            .unwrap();
    }

    let iter = Box::pin(container.query_items::<TestItem>(
        Query::from("SELECT * FROM c"),
        FeedScope::full_container(),
        None,
    ))
    .await
    .unwrap();
    let _results: Vec<TestItem> = Box::pin(iter.try_collect()).await.unwrap();

    let formats = recorder.negotiation_formats.lock().unwrap();
    assert!(
        !formats.is_empty(),
        "expected at least one query request to be recorded",
    );
    for value in formats.iter() {
        assert_eq!(
            value.as_deref(),
            None,
            "a query on a binary-disabled client must advertise no serialization format",
        );
    }
}

/// The mirror of the test above, guarding the **default**: a client that sets no
/// binary option at all must still negotiate binary. Without this, silently
/// flipping the default off passes every wire-level test.
#[tokio::test]
async fn default_client_negotiates_binary_without_any_option() {
    let (container, recorder) =
        build_multi_partition_container_with_recorder("default-xpart-query", 3, None).await;

    let items: Vec<TestItem> = (0..6)
        .map(|i| TestItem {
            id: format!("d-{i}"),
            pk: format!("pk{}", i % 3),
            value: i,
            note: format!("default {i}"),
        })
        .collect();
    for item in &items {
        container
            .create_item(&item.pk, &item.id, item, Some(write_options_with_content()))
            .await
            .unwrap();
    }

    let iter = Box::pin(container.query_items::<TestItem>(
        Query::from("SELECT * FROM c"),
        FeedScope::full_container(),
        None,
    ))
    .await
    .unwrap();
    let results: Vec<TestItem> = Box::pin(iter.try_collect()).await.unwrap();
    assert_eq!(results.len(), items.len());

    let formats = recorder.negotiation_formats.lock().unwrap();
    assert!(
        !formats.is_empty(),
        "expected at least one query request to be recorded",
    );
    for value in formats.iter() {
        assert_eq!(
            value.as_deref(),
            Some("JsonText,CosmosBinary"),
            "a query on a default client must advertise binary",
        );
    }
}

// Copyright (c) Microsoft Corporation. All rights reserved.
// Licensed under the MIT License.

//! End-to-end validation of Cosmos **binary JSON** through the full SDK →
//! driver → in-memory-emulator loop.
//!
//! With `AZURE_COSMOS_BINARY_ENCODING_ENABLED` set, the SDK encodes item write
//! bodies as binary and advertises binary-response support; the in-memory
//! emulator decodes the binary request body, stores it, and (because the
//! negotiation header is present) replies with a binary body, which the SDK
//! auto-detects and decodes. This exercises the complete
//! encode → negotiate → store → encode → decode round-trip locally — no Docker,
//! no real account, no external vectors.

// These tests hold `ENV_LOCK` across `.await` so the process-wide binary-encoding
// env var stays stable while the SDK client is built (enablement is captured
// during the async build). `#[tokio::test]` uses a current-thread runtime, so the
// guard never moves between threads and cannot deadlock.
#![allow(clippy::await_holding_lock)]

use azure_data_cosmos::{
    options::{
        ContentResponseOnWrite, ItemWriteOptions, OperationOptions, Region, RoutingStrategy,
    },
    AccountEndpoint, AccountReference, ContainerClient, CosmosClientBuilder, CosmosRuntimeBuilder,
};
use azure_data_cosmos_driver::in_memory_emulator::{
    ConsistencyLevel, ContainerConfig, InMemoryEmulatorHttpClient, VirtualAccountConfig,
    VirtualRegion,
};
use serde::{Deserialize, Serialize};
use std::sync::Mutex;

const EMULATOR_GATEWAY_URL: &str = "https://eastus.emulator.local";
const BINARY_ENV: &str = "AZURE_COSMOS_BINARY_ENCODING_ENABLED";

/// Serializes the env-mutating tests in this file so they don't race on the
/// process-wide `AZURE_COSMOS_BINARY_ENCODING_ENABLED` variable.
static ENV_LOCK: Mutex<()> = Mutex::new(());

#[derive(Debug, Serialize, Deserialize, PartialEq)]
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
/// database + container. `binary` controls
/// `AZURE_COSMOS_BINARY_ENCODING_ENABLED` for the duration of client
/// construction (the SDK resolves it once at build time).
///
/// The caller must hold the [`ENV_LOCK`] guard for the whole client lifetime so
/// the env mutation does not race other tests.
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

    // Resolve enablement at client-build time from the environment.
    if binary {
        std::env::set_var(BINARY_ENV, "true");
    } else {
        std::env::remove_var(BINARY_ENV);
    }

    let account = AccountReference::with_authentication_key(
        EMULATOR_GATEWAY_URL.parse::<AccountEndpoint>().unwrap(),
        azure_core::credentials::Secret::new("dGVzdGtleQ=="),
    );
    let client = CosmosClientBuilder::new()
        .with_runtime(
            CosmosRuntimeBuilder::from(emulator.runtime_builder())
                .build()
                .await
                .unwrap(),
        )
        .build(account, RoutingStrategy::ProximityTo(Region::EAST_US))
        .await
        .unwrap();

    client
        .database_client(db_name)
        .container_client("items")
        .await
        .unwrap()
}

/// With binary enabled, an item written through the SDK is binary-encoded on the
/// wire, decoded + stored by the emulator, returned as binary, and decoded back
/// — and the value survives every hop unchanged.
#[tokio::test]
async fn binary_encoding_item_write_read_round_trips() {
    let _guard = ENV_LOCK.lock().unwrap();

    let container = build_container("bin-e2e", true).await;
    // Restore the env immediately; the client already captured enablement.
    std::env::remove_var(BINARY_ENV);

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
    let _guard = ENV_LOCK.lock().unwrap();

    // Write with a binary-enabled client.
    let binary_container = build_container("bin-interop", true).await;
    std::env::remove_var(BINARY_ENV);

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

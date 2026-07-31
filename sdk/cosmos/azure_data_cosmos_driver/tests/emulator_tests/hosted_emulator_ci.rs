// Copyright (c) Microsoft Corporation. All rights reserved.
// Licensed under the MIT License.

//! CI contract for the out-of-process in-memory emulator host.

use serde::Deserialize;

use crate::framework::DriverTestClient;

#[derive(Debug, Deserialize)]
#[serde(rename_all = "camelCase")]
struct HostHealth {
    gateway20_enabled: bool,
    connectivity_probes: usize,
    gateway20_requests: usize,
}

#[tokio::test]
#[cfg_attr(
    not(test_category = "emulator_inmemory"),
    ignore = "requires test_category 'emulator_inmemory'"
)]
async fn configured_host_mode_is_exercised() -> Result<(), Box<dyn std::error::Error>> {
    DriverTestClient::run_with_unique_db(async |context, database| {
        let container_name = context.unique_container_name();
        let container = context
            .create_container(&database, &container_name, "/pk")
            .await?;
        let body = br#"{"id":"host-mode-item","pk":"pk1","value":42}"#;
        context
            .create_item(&container, "host-mode-item", "pk1", body)
            .await?;
        context
            .read_item(&container, "host-mode-item", "pk1")
            .await?;
        Ok(())
    })
    .await?;

    let management_endpoint = std::env::var("AZURE_COSMOS_INMEMORY_MANAGEMENT_ENDPOINT")?;
    let health_endpoint = url::Url::parse(&management_endpoint)?.join("health")?;
    let health = reqwest::Client::new()
        .get(health_endpoint)
        .send()
        .await?
        .error_for_status()?
        .bytes()
        .await?;
    let health: HostHealth = serde_json::from_slice(&health)?;
    match std::env::var("AZURE_COSMOS_EMULATOR_FLAVOR").as_deref() {
        Ok("inmemory-v1") => assert!(!health.gateway20_enabled),
        Ok("inmemory-v2") => {
            assert!(health.gateway20_enabled);
            assert!(health.connectivity_probes > 0);
            assert!(health.gateway20_requests > 0);
        }
        flavor => panic!("unexpected hosted emulator flavor: {flavor:?}"),
    }
    Ok(())
}

/// Proves the production driver — with no special knowledge that a real
/// partition split just happened — transparently recovers by refreshing its
/// stale PKRange cache and routing to the new child partitions.
///
/// This is the actual client-side recovery behavior the hosted in-memory
/// emulator exists to make testable (see ADR-001 in
/// `sdk/cosmos/azure_data_cosmos_emulator/docs/adr/`), as opposed to only
/// verifying the management API's own HTTP contract in isolation (which
/// `management.rs`'s own test module already covers).
#[tokio::test]
#[cfg_attr(
    not(test_category = "emulator_inmemory"),
    ignore = "requires test_category 'emulator_inmemory'"
)]
async fn driver_recovers_after_real_partition_split() -> Result<(), Box<dyn std::error::Error>> {
    let management_endpoint = std::env::var("AZURE_COSMOS_INMEMORY_MANAGEMENT_ENDPOINT")?;
    let client = reqwest::Client::new();

    DriverTestClient::run_with_unique_db(async |context, database| {
        let container_name = context.unique_container_name();
        let container = context
            .create_container(&database, &container_name, "/pk")
            .await?;
        let db_name = database
            .name()
            .ok_or("database reference must be name-based")?;

        let created = context
            .create_item(
                &container,
                "split-item-1",
                "pk-a",
                br#"{"id":"split-item-1","pk":"pk-a","value":1}"#,
            )
            .await?;
        context
            .create_item(
                &container,
                "split-item-2",
                "pk-z",
                br#"{"id":"split-item-2","pk":"pk-z","value":2}"#,
            )
            .await?;

        // The management API's partition IDs are opaque to normal Cosmos
        // clients and never used for routing by production code — recover
        // one here purely to target the split, from the diagnostic-only
        // `x-ms-documentdb-partitionkeyrangeid` response header.
        let partition_id: u32 = created
            .headers()
            .partition_key_range_id
            .as_deref()
            .and_then(|id| id.parse().ok())
            .ok_or("could not determine partition id from response headers")?;

        // Trigger a real split through the management API — the same real
        // topology-mutation infrastructure a test author would drive
        // interactively — and wait for it to complete. `midpoint` mode
        // (unlike `storage`) never depends on how many documents happen to
        // already be in the target partition, so it doesn't matter which of
        // the container's default partitions `partition_id` landed on.
        let split_url = format!(
            "{management_endpoint}databases/{db_name}/containers/{container_name}/partitions/{partition_id}/split"
        );
        let created_operation = client
            .post(&split_url)
            .header("content-type", "application/json")
            .body(serde_json::to_vec(&serde_json::json!({ "mode": "midpoint" }))?)
            .send()
            .await?
            .error_for_status()?
            .bytes()
            .await?;
        let created_operation: serde_json::Value = serde_json::from_slice(&created_operation)?;
        let operation_id = created_operation["operationId"]
            .as_str()
            .ok_or("split response missing operationId")?;

        let operation_url = format!("{management_endpoint}operations/{operation_id}");
        let mut terminal = None;
        for _ in 0..100 {
            let body = client
                .get(&operation_url)
                .send()
                .await?
                .error_for_status()?
                .bytes()
                .await?;
            let operation: serde_json::Value = serde_json::from_slice(&body)?;
            if operation["phase"] == "Succeeded" || operation["phase"] == "Failed" {
                terminal = Some(operation);
                break;
            }
            tokio::time::sleep(std::time::Duration::from_millis(50)).await;
        }
        let terminal = terminal.ok_or("split operation did not reach a terminal phase")?;
        assert_eq!(
            terminal["phase"], "Succeeded",
            "split must succeed: {terminal:?}"
        );

        // The production driver has no idea a split just happened. Reading
        // both items back must transparently refresh its stale PKRange
        // cache and route to the new child partitions.
        let read_back_1 = context.read_item(&container, "split-item-1", "pk-a").await?;
        assert!(read_back_1.status().is_success());
        let read_back_2 = context.read_item(&container, "split-item-2", "pk-z").await?;
        assert!(read_back_2.status().is_success());

        Ok(())
    })
    .await
}

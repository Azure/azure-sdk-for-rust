// Copyright (c) Microsoft Corporation. All rights reserved.
// Licensed under the MIT License.

//! End-to-end tests for the Gateway 2.0 transport, exercised through the
//! `azure_data_cosmos` SDK surface (not the underlying driver crate).
//!
//! These tests run against a pre-provisioned Gateway 2.0 ("thin client")
//! account. The endpoint and primary key are read from the
//! `AZURE_COSMOS_GW20_ENDPOINT` and `AZURE_COSMOS_GW20_KEY` environment
//! variables and gated by the `gateway20` test category. They are skipped by
//! default; the dedicated `ci-gateway20.yml` pipeline sets the matrix entry's
//! `testCategory` to `gateway20` (or `gateway20_multi_region`) so the tests
//! run in CI against the live account.
//!
//! ## What these tests assert today
//!
//! [`CosmosClientBuilder::with_gateway20_disabled`] now propagates the
//! Gateway 2.0 toggle into the underlying driver, so the tests exercise the
//! real SDK opt-in path against the live account.
//!
//! Each implemented test:
//!
//! * Builds a [`CosmosClient`] with `with_gateway20_disabled(false)` (or
//!   `true`, for the operator-override scenario), pointing at the
//!   `AZURE_COSMOS_GW20_ENDPOINT/_KEY` account.
//! * Provisions a fresh database + container and drives the operation
//!   appropriate to the test (CRUD, query, batch, point read).
//! * Asserts the operation succeeds and the standard
//!   [`CosmosDiagnostics`] fields (activity ID + server duration) are
//!   populated.
//!
//! ## Future work (`TODO`)
//!
//! The SDK-level [`CosmosDiagnostics`] type does not yet surface the driver's
//! `TransportKind` — that gap is documented on `CosmosDiagnostics` itself
//! ("will be expanded ... once the SDK pipeline is ported to the driver's
//! transport pipeline"). Once that exposure lands, each test should be
//! tightened to assert `TransportKind::Gateway20` (or `StandardGateway` for
//! the override case) on the diagnostics instance returned from the
//! operation.
//!
//! The change-feed test stays a placeholder until the SDK gains a public
//! change-feed API on `ContainerClient` (only the routing-layer change-feed
//! plumbing exists today; there is no `ContainerClient::change_feed` to
//! call from a public test).

#![cfg(feature = "key_auth")]

use azure_core::credentials::Secret;
use azure_data_cosmos::models::{ContainerProperties, PartitionKeyDefinition};
use azure_data_cosmos::{
    CosmosAccountEndpoint, CosmosAccountReference, CosmosClient, Query, Region, RoutingStrategy,
    TransactionalBatch,
};
use futures::StreamExt;
use serde::{Deserialize, Serialize};

fn read_env(name: &str) -> Option<String> {
    std::env::var(name).ok().filter(|v| !v.trim().is_empty())
}

/// Returns `Some((endpoint, key))` only when both env vars are set.
fn live_credentials() -> Option<(String, String)> {
    Some((
        read_env("AZURE_COSMOS_GW20_ENDPOINT")?,
        read_env("AZURE_COSMOS_GW20_KEY")?,
    ))
}

/// Build a [`CosmosClient`] against the live Gateway 2.0 account.
///
/// `gateway20_disabled = false` opts the client in to Gateway 2.0; passing
/// `true` exercises the operator-override path that pins the client to the
/// standard gateway even when the account advertises a thin-client endpoint.
async fn build_client(
    endpoint: &str,
    key: &str,
    gateway20_disabled: bool,
) -> Result<CosmosClient, Box<dyn std::error::Error>> {
    let endpoint: CosmosAccountEndpoint = endpoint.parse()?;
    let account_ref =
        CosmosAccountReference::with_master_key(endpoint, Secret::from(key.to_string()));
    let client = CosmosClient::builder()
        .with_gateway20_disabled(gateway20_disabled)
        .build(account_ref, RoutingStrategy::ProximityTo(Region::EAST_US))
        .await?;
    Ok(client)
}

/// Provisions a fresh database + container scoped to the test invocation and
/// returns the database name (so the caller can drop it) and a container
/// client to drive operations against.
async fn provision_database_and_container(
    client: &CosmosClient,
) -> Result<(String, azure_data_cosmos::clients::ContainerClient), Box<dyn std::error::Error>> {
    let unique = azure_core::Uuid::new_v4();
    let db_name = format!("gw20-test-db-{unique}");
    let container_name = format!("gw20-test-container-{unique}");

    client.create_database(&db_name, None).await?;
    let db_client = client.database_client(&db_name);

    let pk_def: PartitionKeyDefinition = "/pk".into();
    let properties = ContainerProperties::new(container_name.clone(), pk_def);
    db_client.create_container(properties, None).await?;
    let container_client = db_client.container_client(&container_name).await?;

    Ok((db_name, container_client))
}

async fn drop_database(client: &CosmosClient, db_name: &str) {
    let db_client = client.database_client(db_name);
    let _ = db_client.delete(None).await;
}

#[derive(Debug, Deserialize, Serialize, PartialEq, Eq, Clone)]
struct Gw20TestItem {
    id: String,
    pk: String,
    value: i64,
    label: String,
}

/// Drives a point CRUD round-trip (create → read → replace → delete) against
/// the live Gateway 2.0 account.
///
/// TODO: tighten the per-response diagnostics check to assert
/// `TransportKind::Gateway20` once `CosmosDiagnostics` surfaces the
/// transport kind from the driver.
#[tokio::test]
#[cfg_attr(
    not(test_category = "gateway20"),
    ignore = "requires test_category 'gateway20' and AZURE_COSMOS_GW20_ENDPOINT/_KEY"
)]
pub async fn gateway20_point_crud_round_trip() -> Result<(), Box<dyn std::error::Error>> {
    let Some((endpoint, key)) = live_credentials() else {
        return Ok(());
    };

    let client = build_client(&endpoint, &key, false).await?;
    let (db_name, container) = provision_database_and_container(&client).await?;

    let pk_value = format!("pk-{}", azure_core::Uuid::new_v4());
    let item_id = format!("item-{}", azure_core::Uuid::new_v4());
    let mut item = Gw20TestItem {
        id: item_id.clone(),
        pk: pk_value.clone(),
        value: 1,
        label: "initial".into(),
    };

    let create_resp = container.create_item(&pk_value, &item, None).await?;
    assert!(create_resp.diagnostics().activity_id().is_some());
    assert!(create_resp.diagnostics().server_duration_ms().is_some());

    let read_resp = container
        .read_item::<Gw20TestItem>(&pk_value, &item_id, None)
        .await?;
    assert!(read_resp.diagnostics().activity_id().is_some());
    let read_item: Gw20TestItem = read_resp.into_model()?;
    assert_eq!(read_item, item);

    item.value = 2;
    item.label = "updated".into();
    let replace_resp = container
        .replace_item(&pk_value, &item_id, &item, None)
        .await?;
    assert!(replace_resp.diagnostics().activity_id().is_some());

    let delete_resp = container.delete_item(&pk_value, &item_id, None).await?;
    assert!(delete_resp.diagnostics().activity_id().is_some());

    drop_database(&client, &db_name).await;
    Ok(())
}

/// Runs a SQL query through Gateway 2.0 and asserts the streamed pages all
/// route through the thin-client transport.
///
/// TODO: tighten the per-page diagnostics check to assert
/// `TransportKind::Gateway20` once the SDK exposes the driver transport
/// kind on the page diagnostics.
#[tokio::test]
#[cfg_attr(
    not(test_category = "gateway20"),
    ignore = "requires test_category 'gateway20' and AZURE_COSMOS_GW20_ENDPOINT/_KEY"
)]
pub async fn gateway20_query_streams_through_thin_client() -> Result<(), Box<dyn std::error::Error>>
{
    let Some((endpoint, key)) = live_credentials() else {
        return Ok(());
    };

    let client = build_client(&endpoint, &key, false).await?;
    let (db_name, container) = provision_database_and_container(&client).await?;

    let pk_value = format!("pk-{}", azure_core::Uuid::new_v4());
    for i in 0..5 {
        let item = Gw20TestItem {
            id: format!("query-item-{i}"),
            pk: pk_value.clone(),
            value: i64::from(i),
            label: format!("row-{i}"),
        };
        container.create_item(&pk_value, &item, None).await?;
    }

    let query = Query::from("SELECT * FROM c ORDER BY c.value");
    let mut pages = container
        .query_items::<Gw20TestItem>(query, pk_value.clone(), None)?
        .into_pages();

    let mut pages_seen = 0_usize;
    let mut items_seen = 0_usize;
    while let Some(page) = pages.next().await {
        let page = page?;
        pages_seen += 1;
        assert!(page.diagnostics().activity_id().is_some());
        items_seen += page.items().len();
    }
    assert!(pages_seen >= 1, "expected at least one query page");
    assert_eq!(items_seen, 5);

    drop_database(&client, &db_name).await;
    Ok(())
}

/// Runs a transactional batch through Gateway 2.0.
///
/// TODO: tighten the diagnostics check to assert `TransportKind::Gateway20`
/// once the SDK surfaces the driver transport kind on batch diagnostics.
#[tokio::test]
#[cfg_attr(
    not(test_category = "gateway20"),
    ignore = "requires test_category 'gateway20' and AZURE_COSMOS_GW20_ENDPOINT/_KEY"
)]
pub async fn gateway20_transactional_batch() -> Result<(), Box<dyn std::error::Error>> {
    let Some((endpoint, key)) = live_credentials() else {
        return Ok(());
    };

    let client = build_client(&endpoint, &key, false).await?;
    let (db_name, container) = provision_database_and_container(&client).await?;

    let pk_value = format!("pk-{}", azure_core::Uuid::new_v4());
    let item_a = Gw20TestItem {
        id: "batch-a".into(),
        pk: pk_value.clone(),
        value: 10,
        label: "a".into(),
    };
    let item_b = Gw20TestItem {
        id: "batch-b".into(),
        pk: pk_value.clone(),
        value: 20,
        label: "b".into(),
    };
    let upsert = Gw20TestItem {
        id: "batch-c".into(),
        pk: pk_value.clone(),
        value: 30,
        label: "c".into(),
    };

    let batch = TransactionalBatch::new(&pk_value)
        .create_item(&item_a)?
        .create_item(&item_b)?
        .upsert_item(&upsert, None)?;

    let response = container.execute_transactional_batch(batch, None).await?;
    let body = response.into_model()?;
    let codes: Vec<u16> = body.results().iter().map(|r| r.status_code()).collect();
    assert_eq!(codes, vec![201, 201, 201]);

    drop_database(&client, &db_name).await;
    Ok(())
}

/// Drives a `LatestVersion` change feed iterator through Gateway 2.0.
///
/// TODO: implement once the SDK exposes a public change-feed API on
/// `ContainerClient`. Only routing-layer change-feed plumbing exists today
/// (`execute_partition_key_range_read_change_feed`); there is no public
/// `ContainerClient::change_feed` entry point yet, so the test cannot
/// exercise the SDK surface end-to-end. Tracking item: SDK change-feed
/// public API.
#[tokio::test]
#[cfg_attr(
    not(test_category = "gateway20"),
    ignore = "requires test_category 'gateway20' and AZURE_COSMOS_GW20_ENDPOINT/_KEY"
)]
pub async fn gateway20_change_feed_latest_version() {
    let Some((_endpoint, _key)) = live_credentials() else {
        return;
    };
    // Intentionally empty — see the test docs above for why.
}

/// Verifies that diagnostics are populated for SDK-issued requests routed
/// through Gateway 2.0.
///
/// TODO: extend this test to assert `TransportKind::Gateway20` once
/// `CosmosDiagnostics` surfaces the driver transport kind. Today the SDK
/// `CosmosDiagnostics` only carries `activity_id` and `server_duration_ms`,
/// so the strongest behavioural assertion we can make is that those fields
/// are populated when the request was routed through the Gateway 2.0
/// pipeline.
#[tokio::test]
#[cfg_attr(
    not(test_category = "gateway20"),
    ignore = "requires test_category 'gateway20' and AZURE_COSMOS_GW20_ENDPOINT/_KEY"
)]
pub async fn gateway20_diagnostics_validation() -> Result<(), Box<dyn std::error::Error>> {
    let Some((endpoint, key)) = live_credentials() else {
        return Ok(());
    };

    let client = build_client(&endpoint, &key, false).await?;
    let (db_name, container) = provision_database_and_container(&client).await?;

    let pk_value = format!("pk-{}", azure_core::Uuid::new_v4());
    let item = Gw20TestItem {
        id: "diag-item".into(),
        pk: pk_value.clone(),
        value: 99,
        label: "diag".into(),
    };
    container.create_item(&pk_value, &item, None).await?;

    let read_resp = container
        .read_item::<Gw20TestItem>(&pk_value, "diag-item", None)
        .await?;
    let diagnostics = read_resp.diagnostics();
    assert!(
        diagnostics.activity_id().is_some(),
        "expected activity_id to be populated for a Gateway 2.0 request"
    );
    assert!(
        diagnostics.server_duration_ms().is_some(),
        "expected server_duration_ms to be populated for a Gateway 2.0 request"
    );

    drop_database(&client, &db_name).await;
    Ok(())
}

/// Verifies the operator override at the SDK boundary: when the operator
/// disables Gateway 2.0 via [`CosmosClientBuilder::with_gateway20_disabled`],
/// every request must route through the standard gateway even though the
/// account advertises a thin-client endpoint.
///
/// TODO: tighten the assertion to inspect `TransportKind::StandardGateway`
/// in the diagnostics once the SDK exposes the driver transport kind.
///
/// [`CosmosClientBuilder::with_gateway20_disabled`]: azure_data_cosmos::CosmosClientBuilder::with_gateway20_disabled
#[tokio::test]
#[cfg_attr(
    not(test_category = "gateway20"),
    ignore = "requires test_category 'gateway20' and AZURE_COSMOS_GW20_ENDPOINT/_KEY"
)]
pub async fn gateway20_operator_override_at_sdk_boundary() -> Result<(), Box<dyn std::error::Error>>
{
    let Some((endpoint, key)) = live_credentials() else {
        return Ok(());
    };

    let client = build_client(&endpoint, &key, true).await?;
    let (db_name, container) = provision_database_and_container(&client).await?;

    let pk_value = format!("pk-{}", azure_core::Uuid::new_v4());
    let item = Gw20TestItem {
        id: "override-item".into(),
        pk: pk_value.clone(),
        value: 7,
        label: "override".into(),
    };
    container.create_item(&pk_value, &item, None).await?;

    let read_resp = container
        .read_item::<Gw20TestItem>(&pk_value, "override-item", None)
        .await?;
    let diagnostics = read_resp.diagnostics();
    assert!(diagnostics.activity_id().is_some());

    drop_database(&client, &db_name).await;
    Ok(())
}

/// Provisions a fresh database + 3-component HPK container and returns the
/// db name (for cleanup) and a container client. Mirrors
/// [`provision_database_and_container`] but uses
/// `(/tenantId, /userId, /sessionId)` as the partition key paths so the
/// container exercises hierarchical partitioning end-to-end.
async fn provision_database_and_hpk_container(
    client: &CosmosClient,
) -> Result<(String, azure_data_cosmos::clients::ContainerClient), Box<dyn std::error::Error>> {
    let unique = azure_core::Uuid::new_v4();
    let db_name = format!("gw20-test-db-{unique}");
    let container_name = format!("gw20-test-hpk-container-{unique}");

    client.create_database(&db_name, None).await?;
    let db_client = client.database_client(&db_name);

    let pk_def = PartitionKeyDefinition::from(("/tenantId", "/userId", "/sessionId"));
    let properties = ContainerProperties::new(container_name.clone(), pk_def);
    db_client.create_container(properties, None).await?;
    let container_client = db_client.container_client(&container_name).await?;

    Ok((db_name, container_client))
}

#[derive(Debug, Deserialize, Serialize, PartialEq, Eq, Clone)]
struct Gw20HpkItem {
    id: String,
    #[serde(rename = "tenantId")]
    tenant_id: String,
    #[serde(rename = "userId")]
    user_id: String,
    #[serde(rename = "sessionId")]
    session_id: String,
    value: i64,
}

/// Round-trip exercises Gateway 2.0 against a 3-component hierarchical
/// partition key container, asserting both the **full PK** point-op path
/// and the **partial PK** range-dispatch path (`x-ms-thinclient-range-min`
/// / `-max`) discussed in the Gateway 2.0 spec test matrix
/// ("HPK + Gateway 2.0: full vs partial PK").
///
/// 1. Inserts items spread across two tenants × two users.
/// 2. Reads each item back via its full 3-component PK (point op → EPK token).
/// 3. Queries with a **1-component prefix** (`tenantId` only) and asserts
///    the items for that tenant come back across however many pages the
///    proxy fans out into.
///
/// The point-vs-range header emission is asserted at unit level in
/// `gateway20_dispatch::tests`; this E2E test guards the SDK-public surface
/// against regressions where partial-PK queries silently degrade to
/// single-partition or fail.
///
/// TODO: tighten the diagnostics check to assert `TransportKind::Gateway20`
/// once the SDK surfaces the driver transport kind.
#[tokio::test]
#[cfg_attr(
    not(test_category = "gateway20"),
    ignore = "requires test_category 'gateway20' and AZURE_COSMOS_GW20_ENDPOINT/_KEY"
)]
pub async fn gateway20_hpk_full_and_partial_partition_key_round_trip(
) -> Result<(), Box<dyn std::error::Error>> {
    use azure_data_cosmos::{PartitionKey, PartitionKeyValue};

    let Some((endpoint, key)) = live_credentials() else {
        return Ok(());
    };

    let client = build_client(&endpoint, &key, false).await?;
    let (db_name, container) = provision_database_and_hpk_container(&client).await?;

    let target_tenant = format!("tenant-{}", azure_core::Uuid::new_v4());
    let other_tenant = format!("tenant-{}", azure_core::Uuid::new_v4());

    // Two users × two sessions per tenant => 4 items per tenant.
    let mut expected_target_ids = Vec::new();
    for tenant in [target_tenant.as_str(), other_tenant.as_str()] {
        for user_idx in 0..2 {
            for session_idx in 0..2 {
                let user_id = format!("user-{user_idx}");
                let session_id = format!("session-{session_idx}");
                let id = format!("{tenant}-{user_id}-{session_id}");
                if tenant == target_tenant {
                    expected_target_ids.push(id.clone());
                }
                let item = Gw20HpkItem {
                    id: id.clone(),
                    tenant_id: tenant.to_string(),
                    user_id: user_id.clone(),
                    session_id: session_id.clone(),
                    value: i64::from(user_idx * 10 + session_idx),
                };
                // PartitionKey tuple impls require owned types (the underlying
                // `PartitionKeyValue: From<&'static str>` impl is the only
                // borrow-friendly one) — clone strings into the tuple.
                let pk = PartitionKey::from((tenant.to_string(), user_id, session_id));
                container.create_item(pk, &item, None).await?;
            }
        }
    }

    // Full HPK point read (3-of-3 components → EPK token path).
    let full_pk = PartitionKey::from((
        target_tenant.clone(),
        "user-0".to_string(),
        "session-0".to_string(),
    ));
    let full_id = format!("{target_tenant}-user-0-session-0");
    let read_resp = container
        .read_item::<Gw20HpkItem>(full_pk, &full_id, None)
        .await?;
    let item: Gw20HpkItem = read_resp.into_model()?;
    assert_eq!(item.id, full_id);
    assert_eq!(item.tenant_id, target_tenant);

    // Partial HPK query (1-of-3 components → range header path).
    // PartitionKey only has tuple From-impls for 2 and 3 components; for a
    // single-component prefix, construct it from a Vec<PartitionKeyValue> so
    // the dispatcher sees a 1-component value against a 3-path container.
    let partial_pk = PartitionKey::from(vec![PartitionKeyValue::from(target_tenant.clone())]);
    let query = Query::from("SELECT * FROM c");
    let mut pages = container
        .query_items::<Gw20HpkItem>(query, partial_pk, None)?
        .into_pages();

    let mut returned_ids: Vec<String> = Vec::new();
    let mut pages_seen = 0_usize;
    while let Some(page) = pages.next().await {
        let page = page?;
        pages_seen += 1;
        assert!(page.diagnostics().activity_id().is_some());
        for it in page.items() {
            assert_eq!(
                it.tenant_id, target_tenant,
                "partial-PK query must not bleed across tenants"
            );
            returned_ids.push(it.id.clone());
        }
    }
    assert!(pages_seen >= 1, "expected at least one query page");
    expected_target_ids.sort();
    returned_ids.sort();
    assert_eq!(returned_ids, expected_target_ids);

    drop_database(&client, &db_name).await;
    Ok(())
}

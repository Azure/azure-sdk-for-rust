// Copyright (c) Microsoft Corporation. All rights reserved.
// Licensed under the MIT License.

#![cfg(feature = "key_auth")]

use azure_core::credentials::Secret;
use azure_core::http::{Etag, StatusCode};
use azure_data_cosmos::diagnostics::{DiagnosticsContext, TransportKind};
use azure_data_cosmos::models::{
    ContainerProperties, PartitionKeyDefinition, ThroughputProperties,
};
use azure_data_cosmos::options::{
    CreateContainerOptions, ItemReadOptions, ItemWriteOptions, MaxItemCountHint,
    OperationOptionsBuilder, PartitionFailoverOptions, Precondition, QueryOptions,
    ReadConsistencyStrategy, Region,
};
use azure_data_cosmos::{
    AccountEndpoint, AccountReference, CosmosClient, FeedScope, Query, RoutingStrategy,
    SubStatusCode, TransactionalBatch,
};
use futures::StreamExt;
use serde::{Deserialize, Serialize};
use std::num::NonZeroU32;

fn read_env(name: &str) -> Option<String> {
    std::env::var(name).ok().filter(|v| !v.trim().is_empty())
}

/// Asserts every request recorded in `diagnostics` used `expected` transport.
fn assert_transport_kind(diagnostics: &DiagnosticsContext, expected: TransportKind) {
    let requests = diagnostics.requests();
    assert!(
        !requests.is_empty(),
        "expected at least one request in diagnostics"
    );
    for r in requests.iter() {
        // The transport kind is stamped together with the contacted endpoint at
        // request-start, and the status is written onto the same record when the
        // request completes. Requiring a terminal (non-sentinel) status proves we
        // are reading a request that actually received a response over `expected`,
        // not a pre-flight stub.
        assert_ne!(
            r.status().status_code(),
            StatusCode::from(0),
            "expected a completed request, got an uncompleted record for {:?}",
            r.transport_kind()
        );
        assert_eq!(
            r.transport_kind(),
            expected,
            "expected request to {} to use {expected:?}, got {:?}",
            r.endpoint(),
            r.transport_kind()
        );
    }
}

/// Normalizes a Gateway 2.0 endpoint string so it can be parsed by
/// `AccountEndpoint::from_str` (a thin wrapper over `Url::parse`): prepends
/// `https://` and a trailing `/` when missing.
fn normalize_gateway_v2_endpoint(raw: &str) -> String {
    let trimmed = raw.trim();
    let with_scheme = if trimmed.contains("://") {
        trimmed.to_owned()
    } else {
        format!("https://{trimmed}")
    };
    if with_scheme.ends_with('/') {
        with_scheme
    } else {
        format!("{with_scheme}/")
    }
}

/// Returns `Some((endpoint, key))` only when both env vars are set.
///
/// Multi-region tests (`test_category = "gateway_v2_multi_region"`) read the
/// multi-region GW_V2 account; single-region tests read the single-region
/// account. The pair is gated at compile time so the test code stays
/// uniform.
fn live_credentials() -> Option<(String, String)> {
    #[cfg(test_category = "gateway_v2_multi_region")]
    let (endpoint_var, key_var) = (
        "AZURE_COSMOS_GW_V2_MULTI_REGION_ENDPOINT",
        "AZURE_COSMOS_GW_V2_MULTI_REGION_KEY",
    );
    #[cfg(not(test_category = "gateway_v2_multi_region"))]
    let (endpoint_var, key_var) = ("AZURE_COSMOS_GW_V2_ENDPOINT", "AZURE_COSMOS_GW_V2_KEY");

    Some((read_env(endpoint_var)?, read_env(key_var)?))
}

/// Build a [`CosmosClient`] against the live Gateway 2.0 account.
///
/// Whether traffic actually flows over Gateway 2.0 is decided by the server
/// (account advertisement + the runtime connectivity probe), so the client
/// simply targets a Gateway 2.0 account endpoint and lets the runtime select
/// the transport.
async fn build_client(
    endpoint: &str,
    key: &str,
) -> Result<CosmosClient, Box<dyn std::error::Error>> {
    build_client_for_region(endpoint, key, Region::EAST_US).await
}

/// Like [`build_client`] but pins proximity routing to `region` so reads are
/// served from that region's replica when the account advertises it. Used by
/// the multi-region usability test to prove a collection is reachable over
/// Gateway 2.0 from every advertised read region, not just the default one.
async fn build_client_for_region(
    endpoint: &str,
    key: &str,
    region: Region,
) -> Result<CosmosClient, Box<dyn std::error::Error>> {
    let endpoint: AccountEndpoint = normalize_gateway_v2_endpoint(endpoint).parse()?;
    let account_ref =
        AccountReference::with_authentication_key(endpoint, Secret::from(key.to_string()));
    let client = CosmosClient::builder()
        .build(account_ref, RoutingStrategy::ProximityTo(region))
        .await?;
    Ok(client)
}

/// Like [`build_client`] but disables the per-partition circuit breaker (PPCB)
/// at the client level.
///
/// On a standard Session account (no `enable_per_partition_failover_behavior`
/// server property, as provisioned for these tests) this leaves both PPCB and
/// PPAF off. In that configuration the *only* reason the driver pre-resolves
/// the target partition key range id is that the client may route over Gateway
/// 2.0 — mirroring .NET's `ThinClientStoreModel::ShouldResolvePartitionKeyRange()
/// => true`. This reproduces the residual condition where a stale multi-range
/// composite session token would otherwise be sent on a single-partition
/// request and rejected by the thin-client backend with HTTP 400.
async fn build_client_ppcb_disabled(
    endpoint: &str,
    key: &str,
) -> Result<CosmosClient, Box<dyn std::error::Error>> {
    let endpoint: AccountEndpoint = normalize_gateway_v2_endpoint(endpoint).parse()?;
    let account_ref =
        AccountReference::with_authentication_key(endpoint, Secret::from(key.to_string()));
    let failover_options = PartitionFailoverOptions::builder()
        .with_circuit_breaker_enabled(false)
        .build()?;
    let client = CosmosClient::builder()
        .with_partition_failover_options(failover_options)
        .build(account_ref, RoutingStrategy::ProximityTo(Region::EAST_US))
        .await?;
    Ok(client)
}

/// Resolves a container client, retrying past the
/// `404 / 1013 CollectionCreateInProgress` window that follows a fresh
/// `create_container` on multi-region thin-client accounts. Both the
/// metadata resolution in [`DatabaseClient::container_client`] and the
/// subsequent first data-plane request can race the gateway's
/// container-create completion; this helper keeps retrying the metadata
/// resolution *and* a follow-up `read` until both succeed or until a
/// non-1013 error surfaces.
async fn wait_for_container_ready(
    db_client: &azure_data_cosmos::clients::DatabaseClient,
    container_name: &str,
) -> Result<azure_data_cosmos::clients::ContainerClient, Box<dyn std::error::Error>> {
    const MAX_ATTEMPTS: u32 = 60;
    const POLL_INTERVAL: std::time::Duration = std::time::Duration::from_millis(500);

    for attempt in 0..MAX_ATTEMPTS {
        let last_err: Box<dyn std::error::Error> =
            match db_client.container_client(container_name).await {
                Ok(container_client) => match container_client.read(None).await {
                    Ok(_) => return Ok(container_client),
                    Err(e)
                        if e.status().sub_status()
                            == Some(SubStatusCode::COLLECTION_CREATE_IN_PROGRESS) =>
                    {
                        Box::new(e)
                    }
                    Err(e) => return Err(Box::new(e)),
                },
                Err(e)
                    if e.status().sub_status()
                        == Some(SubStatusCode::COLLECTION_CREATE_IN_PROGRESS) =>
                {
                    Box::new(e)
                }
                Err(e) => return Err(Box::new(e)),
            };

        if attempt + 1 == MAX_ATTEMPTS {
            return Err(format!(
                "container '{container_name}' did not become ready after {MAX_ATTEMPTS} polls (last error: {last_err})"
            )
            .into());
        }
        tokio::time::sleep(POLL_INTERVAL).await;
    }
    unreachable!("loop above always returns on the final iteration");
}

/// Provisions a fresh database + container scoped to the test invocation and
/// returns the database name (so the caller can drop it) and a container
/// client to drive operations against.
async fn provision_database_and_container(
    client: &CosmosClient,
) -> Result<(String, azure_data_cosmos::clients::ContainerClient), Box<dyn std::error::Error>> {
    let unique = azure_core::Uuid::new_v4();
    let db_name = format!("gw_v2-test-db-{unique}");
    let container_name = format!("gw_v2-test-container-{unique}");

    client.create_database(&db_name, None).await?;
    let db_client = client.database_client(&db_name);

    let pk_def: PartitionKeyDefinition = "/pk".into();
    let properties = ContainerProperties::new(container_name.clone(), pk_def);
    db_client.create_container(properties, None).await?;
    let container_client = wait_for_container_ready(&db_client, &container_name).await?;

    Ok((db_name, container_client))
}

async fn drop_database(client: &CosmosClient, db_name: &str) {
    let db_client = client.database_client(db_name);
    let _ = db_client.delete(None).await;
}

#[derive(Debug, Deserialize, Serialize, PartialEq, Eq, Clone)]
struct GwV2TestItem {
    id: String,
    pk: String,
    value: i64,
    label: String,
}

/// Point-reads `item_id` (partition `pk`) through a client whose proximity
/// routing is pinned to `region`, retrying past the brief cross-region
/// replication / `404 NotFound` window that can follow a write on a
/// multi-region account. Asserts the read is served over Gateway 2.0 and
/// returns the matching item unchanged.
async fn assert_item_readable_from_region(
    endpoint: &str,
    key: &str,
    region: Region,
    db_name: &str,
    container_name: &str,
    expected: &GwV2TestItem,
) -> Result<(), Box<dyn std::error::Error>> {
    const MAX_ATTEMPTS: u32 = 40;
    const POLL_INTERVAL: std::time::Duration = std::time::Duration::from_millis(500);

    let client = build_client_for_region(endpoint, key, region.clone()).await?;
    let container = client
        .database_client(db_name)
        .container_client(container_name)
        .await?;

    for attempt in 0..MAX_ATTEMPTS {
        match container.read_item(&expected.pk, &expected.id, None).await {
            Ok(read_resp) => {
                assert_transport_kind(&read_resp.diagnostics(), TransportKind::GatewayV2);
                let read_item: GwV2TestItem = read_resp.into_model()?;
                assert_eq!(
                    &read_item, expected,
                    "item read from region {region:?} must match what was written",
                );
                return Ok(());
            }
            Err(e)
                if e.status().status_code() == StatusCode::NotFound
                    && attempt + 1 < MAX_ATTEMPTS =>
            {
                tokio::time::sleep(POLL_INTERVAL).await;
            }
            Err(e) => {
                return Err(format!("point read from region {region:?} failed: {e}").into());
            }
        }
    }
    unreachable!("loop above always returns on the final iteration");
}

/// Drives a point CRUD round-trip (create → read → replace → delete) against
/// the live Gateway 2.0 account.
#[tokio::test]
#[cfg_attr(
    not(any(
        test_category = "gateway_v2",
        test_category = "gateway_v2_multi_region"
    )),
    ignore = "requires test_category 'gateway_v2' and AZURE_COSMOS_GW_V2_ENDPOINT/_KEY"
)]
pub async fn gateway_v2_point_crud_round_trip() -> Result<(), Box<dyn std::error::Error>> {
    let Some((endpoint, key)) = live_credentials() else {
        return Ok(());
    };

    let client = build_client(&endpoint, &key).await?;
    let (db_name, container) = provision_database_and_container(&client).await?;

    let pk_value = format!("pk-{}", azure_core::Uuid::new_v4());
    let item_id = format!("item-{}", azure_core::Uuid::new_v4());
    let mut item = GwV2TestItem {
        id: item_id.clone(),
        pk: pk_value.clone(),
        value: 1,
        label: "initial".into(),
    };

    let create_resp = container
        .create_item(&pk_value, &item_id, &item, None)
        .await?;
    assert_transport_kind(&create_resp.diagnostics(), TransportKind::GatewayV2);
    assert!(!create_resp.diagnostics().activity_id().as_str().is_empty());
    assert!(create_resp.diagnostics().duration() > std::time::Duration::ZERO);

    let read_resp = container.read_item(&pk_value, &item_id, None).await?;
    assert_transport_kind(&read_resp.diagnostics(), TransportKind::GatewayV2);
    assert!(!read_resp.diagnostics().activity_id().as_str().is_empty());
    let read_item: GwV2TestItem = read_resp.into_model()?;
    assert_eq!(read_item, item);

    item.value = 2;
    item.label = "updated".into();
    let replace_resp = container
        .replace_item(&pk_value, &item_id, &item, None)
        .await?;
    assert_transport_kind(&replace_resp.diagnostics(), TransportKind::GatewayV2);
    assert!(!replace_resp.diagnostics().activity_id().as_str().is_empty());

    let delete_resp = container.delete_item(&pk_value, &item_id, None).await?;
    assert_transport_kind(&delete_resp.diagnostics(), TransportKind::GatewayV2);
    assert!(!delete_resp.diagnostics().activity_id().as_str().is_empty());

    drop_database(&client, &db_name).await;
    Ok(())
}

/// Exercises a transactional batch routed through Gateway 2.0.
#[tokio::test]
#[cfg_attr(
    not(any(
        test_category = "gateway_v2",
        test_category = "gateway_v2_multi_region"
    )),
    ignore = "requires test_category 'gateway_v2' and AZURE_COSMOS_GW_V2_ENDPOINT/_KEY"
)]
pub async fn gateway_v2_transactional_batch() -> Result<(), Box<dyn std::error::Error>> {
    let Some((endpoint, key)) = live_credentials() else {
        return Ok(());
    };

    let client = build_client(&endpoint, &key).await?;
    let (db_name, container) = provision_database_and_container(&client).await?;

    let pk_value = format!("pk-{}", azure_core::Uuid::new_v4());
    let item_a = GwV2TestItem {
        id: "batch-a".into(),
        pk: pk_value.clone(),
        value: 10,
        label: "a".into(),
    };
    let item_b = GwV2TestItem {
        id: "batch-b".into(),
        pk: pk_value.clone(),
        value: 20,
        label: "b".into(),
    };
    let upsert = GwV2TestItem {
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
    assert_transport_kind(&response.diagnostics(), TransportKind::GatewayV2);
    let body = response.into_model()?;
    let codes: Vec<u16> = body.results().iter().map(|r| r.status_code()).collect();
    assert_eq!(codes, vec![201, 201, 201]);

    drop_database(&client, &db_name).await;
    Ok(())
}

/// Verifies that diagnostics are populated for SDK-issued requests routed
/// through Gateway 2.0, including the `GatewayV2` transport kind.
#[tokio::test]
#[cfg_attr(
    not(any(
        test_category = "gateway_v2",
        test_category = "gateway_v2_multi_region"
    )),
    ignore = "requires test_category 'gateway_v2' and AZURE_COSMOS_GW_V2_ENDPOINT/_KEY"
)]
pub async fn gateway_v2_diagnostics_validation() -> Result<(), Box<dyn std::error::Error>> {
    let Some((endpoint, key)) = live_credentials() else {
        return Ok(());
    };

    let client = build_client(&endpoint, &key).await?;
    let (db_name, container) = provision_database_and_container(&client).await?;

    let pk_value = format!("pk-{}", azure_core::Uuid::new_v4());
    let item = GwV2TestItem {
        id: "diag-item".into(),
        pk: pk_value.clone(),
        value: 99,
        label: "diag".into(),
    };
    container
        .create_item(&pk_value, "diag-item", &item, None)
        .await?;

    let read_resp = container.read_item(&pk_value, "diag-item", None).await?;
    let diagnostics = read_resp.diagnostics();
    assert_transport_kind(&diagnostics, TransportKind::GatewayV2);
    assert!(
        !diagnostics.activity_id().as_str().is_empty(),
        "expected activity_id to be populated for a Gateway 2.0 request"
    );
    assert!(
        diagnostics.duration() > std::time::Duration::ZERO,
        "expected server_duration_ms to be populated for a Gateway 2.0 request"
    );

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
    let db_name = format!("gw_v2-test-db-{unique}");
    let container_name = format!("gw_v2-test-hpk-container-{unique}");

    client.create_database(&db_name, None).await?;
    let db_client = client.database_client(&db_name);

    let pk_def = PartitionKeyDefinition::from(("/tenantId", "/userId", "/sessionId"));
    let properties = ContainerProperties::new(container_name.clone(), pk_def);
    db_client.create_container(properties, None).await?;
    let container_client = wait_for_container_ready(&db_client, &container_name).await?;

    Ok((db_name, container_client))
}

#[derive(Debug, Deserialize, Serialize, PartialEq, Eq, Clone)]
struct GwV2HpkItem {
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
/// `gateway_v2_dispatch::tests`; this E2E test guards the SDK-public surface
/// against regressions where partial-PK queries silently degrade to
/// single-partition or fail.
#[tokio::test]
#[cfg_attr(
    not(any(
        test_category = "gateway_v2",
        test_category = "gateway_v2_multi_region"
    )),
    ignore = "requires test_category 'gateway_v2' and AZURE_COSMOS_GW_V2_ENDPOINT/_KEY"
)]
pub async fn gateway_v2_hpk_full_and_partial_partition_key_round_trip(
) -> Result<(), Box<dyn std::error::Error>> {
    use azure_data_cosmos::models::PartitionKeyValue;
    use azure_data_cosmos::PartitionKey;

    let Some((endpoint, key)) = live_credentials() else {
        return Ok(());
    };

    let client = build_client(&endpoint, &key).await?;
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
                let item = GwV2HpkItem {
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
                container.create_item(pk, &id, &item, None).await?;
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
    let read_resp = container.read_item(full_pk, &full_id, None).await?;
    assert_transport_kind(&read_resp.diagnostics(), TransportKind::GatewayV2);
    let item: GwV2HpkItem = read_resp.into_model()?;
    assert_eq!(item.id, full_id);
    assert_eq!(item.tenant_id, target_tenant);

    // Partial HPK query (1-of-3 components → range header path).
    // PartitionKey only has tuple From-impls for 2 and 3 components; for a
    // single-component prefix, construct it from a Vec<PartitionKeyValue> so
    // the dispatcher sees a 1-component value against a 3-path container.
    let partial_pk = PartitionKey::from(vec![PartitionKeyValue::from(target_tenant.clone())]);
    let query = Query::from("SELECT * FROM c");
    let mut pages = container
        .query_items::<GwV2HpkItem>(query, FeedScope::partition(partial_pk), None)
        .await?
        .into_pages();

    let mut returned_ids: Vec<String> = Vec::new();
    let mut pages_seen = 0_usize;
    while let Some(page) = pages.next().await {
        let page = page?;
        pages_seen += 1;
        assert_transport_kind(&page.diagnostics(), TransportKind::GatewayV2);
        assert!(!page.diagnostics().activity_id().as_str().is_empty());
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

// ----------------------------------------------------------------------------
// Cross-partition query coverage (PR #4440 Feed Operation Pipeline × Gateway 2.0)
// ----------------------------------------------------------------------------
//
// The single-partition query tests above exercise the `FeedScope::partition(...)`
// path, which targets exactly one physical partition. The new Feed Operation
// Pipeline (`azure_data_cosmos_driver::driver::dataflow`) introduced in PR
// #4440 plans cross-partition queries as a `SequentialDrain` over one leaf
// node per physical partition, with each leaf request flowing through the
// standard operation pipeline (and therefore the Gateway 2.0 transport when
// the operation is eligible — `Query` / `SqlQuery` / `QueryPlan` / `ReadFeed`
// on `Document` are all Gateway-2.0-eligible per `gateway_v2_eligibility.rs`).
//
// The tests below provision a **multi-partition** container (11 000 RU/s
// forces at least two physical partitions on a real Cosmos account) and then
// exercise the two cross-partition `FeedScope` variants:
//
// * `FeedScope::full_container()` — fans out to every physical partition.
// * `FeedScope::range(FeedRange::full())` — the same wire effect, but
//   constructed via the explicit `FeedRange` API rather than the convenience
//   helper.
//
// Together with the existing `FeedScope::partition(...)` tests these cover
// all three `FeedScope` constructors against Gateway 2.0.

/// Provisions a fresh database + container with enough throughput to be
/// split across multiple physical partitions on a real Cosmos account.
///
/// 11 000 RU/s is the minimum throughput that guarantees at least two
/// physical partitions on a standard provisioned-throughput account today
/// (the per-partition cap is 10 000 RU/s). This makes
/// `FeedScope::full_container()` fan out into more than one leaf request
/// at the driver layer, which is the behavior these tests need to
/// observe.
async fn provision_database_and_multi_partition_container(
    client: &CosmosClient,
) -> Result<(String, azure_data_cosmos::clients::ContainerClient), Box<dyn std::error::Error>> {
    let unique = azure_core::Uuid::new_v4();
    let db_name = format!("gw_v2-test-db-{unique}");
    let container_name = format!("gw_v2-test-xpart-container-{unique}");

    client.create_database(&db_name, None).await?;
    let db_client = client.database_client(&db_name);

    let pk_def: PartitionKeyDefinition = "/pk".into();
    let properties = ContainerProperties::new(container_name.clone(), pk_def);
    let create_options =
        CreateContainerOptions::default().with_throughput(ThroughputProperties::manual(11_000));
    db_client
        .create_container(properties, Some(create_options))
        .await?;
    let container_client = wait_for_container_ready(&db_client, &container_name).await?;

    Ok((db_name, container_client))
}

/// Cross-partition `SELECT *` over `FeedScope::full_container()` on Gateway
/// 2.0. Inserts items across many distinct logical partition keys so they
/// land on multiple physical partitions, then asserts that the fanned-out
/// query returns every item exactly once.
///
/// This is the headline regression test for PR #4440 (Feed Operation
/// Pipeline) running on the Gateway 2.0 transport: each leaf request the
/// `SequentialDrain` issues is a `Query`/`Document` operation, which
/// `gateway_v2_eligibility::is_operation_supported_by_gateway_v2` reports as
/// eligible, so all N leaf requests must route through Gateway 2.0 and
/// their pages must reassemble into the full result set with no
/// duplicates and no drops across partition boundaries.
#[tokio::test]
#[cfg_attr(
    not(any(
        test_category = "gateway_v2",
        test_category = "gateway_v2_multi_region"
    )),
    ignore = "requires test_category 'gateway_v2' and AZURE_COSMOS_GW_V2_ENDPOINT/_KEY"
)]
pub async fn gateway_v2_cross_partition_query_full_container(
) -> Result<(), Box<dyn std::error::Error>> {
    use std::collections::HashSet;

    let Some((endpoint, key)) = live_credentials() else {
        return Ok(());
    };

    let client = build_client(&endpoint, &key).await?;
    let (db_name, container) = provision_database_and_multi_partition_container(&client).await?;

    // 32 distinct logical PKs gives the partition router meaningful spread
    // across the multiple physical partitions provisioned above.
    let total_items: usize = 32;
    let mut expected_ids: HashSet<String> = HashSet::new();
    for i in 0..total_items {
        let pk = format!("pk-{i:02}-{}", azure_core::Uuid::new_v4());
        let id = format!("xpart-item-{i:02}");
        let item = GwV2TestItem {
            id: id.clone(),
            pk: pk.clone(),
            value: i as i64,
            label: format!("row-{i}"),
        };
        container.create_item(&pk, &id, &item, None).await?;
        expected_ids.insert(id);
    }

    let query = Query::from("SELECT * FROM c");
    let mut pages = container
        .query_items::<GwV2TestItem>(query, FeedScope::full_container(), None)
        .await?
        .into_pages();

    let mut pages_seen = 0_usize;
    let mut seen_ids: HashSet<String> = HashSet::new();
    while let Some(page) = pages.next().await {
        let page = page?;
        pages_seen += 1;
        assert!(
            !page.diagnostics().activity_id().as_str().is_empty(),
            "every cross-partition Gateway 2.0 page must surface an activity-id",
        );
        for item in page.items() {
            assert!(
                seen_ids.insert(item.id.clone()),
                "item {} returned twice — sequential drain over physical \
                 partitions must not duplicate items across partition boundaries",
                item.id,
            );
        }
    }

    assert!(
        pages_seen >= 1,
        "expected at least one page from the cross-partition fanout",
    );
    assert_eq!(
        seen_ids, expected_ids,
        "cross-partition query must return every inserted item exactly once",
    );

    drop_database(&client, &db_name).await;
    Ok(())
}

/// Cross-partition `SELECT *` over `FeedScope::range(FeedRange::full())` on
/// Gateway 2.0. Exercises the explicit `FeedRange` constructor on the
/// `FeedScope` enum (as opposed to the `full_container()` convenience) so
/// we know both constructors plan to the same SequentialDrain shape and
/// produce equivalent results against the Gateway 2.0 transport.
#[tokio::test]
#[cfg_attr(
    not(any(
        test_category = "gateway_v2",
        test_category = "gateway_v2_multi_region"
    )),
    ignore = "requires test_category 'gateway_v2' and AZURE_COSMOS_GW_V2_ENDPOINT/_KEY"
)]
pub async fn gateway_v2_cross_partition_query_via_feed_range_full(
) -> Result<(), Box<dyn std::error::Error>> {
    use azure_data_cosmos::feed::FeedRange;
    use std::collections::HashSet;

    let Some((endpoint, key)) = live_credentials() else {
        return Ok(());
    };

    let client = build_client(&endpoint, &key).await?;
    let (db_name, container) = provision_database_and_multi_partition_container(&client).await?;

    let total_items: usize = 16;
    let mut expected_ids: HashSet<String> = HashSet::new();
    for i in 0..total_items {
        let pk = format!("pk-{i:02}-{}", azure_core::Uuid::new_v4());
        let id = format!("xpart-range-{i:02}");
        let item = GwV2TestItem {
            id: id.clone(),
            pk: pk.clone(),
            value: i as i64,
            label: format!("row-{i}"),
        };
        container.create_item(&pk, &id, &item, None).await?;
        expected_ids.insert(id);
    }

    let query = Query::from("SELECT * FROM c");
    let mut pages = container
        .query_items::<GwV2TestItem>(query, FeedScope::range(FeedRange::full()), None)
        .await?
        .into_pages();

    let mut seen_ids: HashSet<String> = HashSet::new();
    while let Some(page) = pages.next().await {
        let page = page?;
        assert!(!page.diagnostics().activity_id().as_str().is_empty());
        for item in page.items() {
            assert!(
                seen_ids.insert(item.id.clone()),
                "item {} returned twice via FeedRange::full() — explicit \
                 feed-range fanout must not duplicate items",
                item.id,
            );
        }
    }
    assert_eq!(
        seen_ids, expected_ids,
        "FeedScope::range(FeedRange::full()) on Gateway 2.0 must yield \
         the same complete result set as FeedScope::full_container()",
    );

    drop_database(&client, &db_name).await;
    Ok(())
}

/// Read-your-writes across multiple physical partitions on Gateway 2.0 with the
/// per-partition circuit breaker (PPCB) disabled.
///
/// Regression guard for the composite-session-token 400. On the thin-client
/// (RNTBD-over-HTTP/2) path the backend rejects a multi-range composite session
/// token (`pk0:vec0,pk1:vec1`) on a request that targets a single partition with
/// `400 "Session token specified is invalid."`. Direct-mode semantics require
/// sending only the target partition's single-range segment.
///
/// The client is built with PPCB disabled (see [`build_client_ppcb_disabled`]),
/// which on these Session accounts also leaves PPAF off. In that configuration
/// the driver must *still* resolve the target partition key range id purely
/// because it may route over Gateway 2.0 — the exact residual gap this test
/// pins. The load-bearing steps:
///   1. Write 16 items across 16 distinct logical partition keys, so session
///      tokens for multiple physical partition ranges accumulate in the cache.
///   2. Point-read every item (read-your-writes). Each read targets one
///      partition; before the fix, the accumulated multi-range composite token
///      would be sent and rejected with 400. After the fix the token is scoped
///      to the read's own range, so every read returns the just-written item.
///   3. Run a `FeedRange::full()` cross-partition query and assert the complete
///      result set, exercising the per-leaf scoped-token path as well.
#[tokio::test]
#[cfg_attr(
    not(any(
        test_category = "gateway_v2",
        test_category = "gateway_v2_multi_region"
    )),
    ignore = "requires test_category 'gateway_v2' and AZURE_COSMOS_GW_V2_ENDPOINT/_KEY"
)]
pub async fn gateway_v2_session_read_your_writes_ppcb_disabled(
) -> Result<(), Box<dyn std::error::Error>> {
    use azure_data_cosmos::feed::FeedRange;
    use std::collections::HashSet;

    let Some((endpoint, key)) = live_credentials() else {
        return Ok(());
    };

    let client = build_client_ppcb_disabled(&endpoint, &key).await?;
    let (db_name, container) = provision_database_and_multi_partition_container(&client).await?;

    let total_items: usize = 16;
    let mut written: Vec<(String, String)> = Vec::with_capacity(total_items);
    let mut expected_ids: HashSet<String> = HashSet::new();
    for i in 0..total_items {
        let pk = format!("pk-{i:02}-{}", azure_core::Uuid::new_v4());
        let id = format!("ryw-item-{i:02}");
        let item = GwV2TestItem {
            id: id.clone(),
            pk: pk.clone(),
            value: i as i64,
            label: format!("row-{i}"),
        };
        let create_resp = container.create_item(&pk, &id, &item, None).await?;
        assert_transport_kind(&create_resp.diagnostics(), TransportKind::GatewayV2);
        written.push((pk, id.clone()));
        expected_ids.insert(id);
    }

    // Read-your-writes: each point read targets a single partition. With
    // multiple ranges cached, a composite session token would be rejected with
    // 400; the scoped single-range token must instead return the written item.
    for (i, (pk, id)) in written.iter().enumerate() {
        let read_resp = container.read_item(pk, id, None).await?;
        assert_transport_kind(&read_resp.diagnostics(), TransportKind::GatewayV2);
        let read_item: GwV2TestItem = read_resp.into_model()?;
        assert_eq!(
            read_item.id, *id,
            "read-your-writes must return the item just written to partition {i}",
        );
        assert_eq!(read_item.value, i as i64);
    }

    // Cross-partition query over the accumulated multi-range session state must
    // also succeed and return the complete set (per-leaf scoped tokens).
    let query = Query::from("SELECT * FROM c");
    let mut pages = container
        .query_items::<GwV2TestItem>(query, FeedScope::range(FeedRange::full()), None)
        .await?
        .into_pages();

    let mut seen_ids: HashSet<String> = HashSet::new();
    while let Some(page) = pages.next().await {
        let page = page?;
        assert_transport_kind(&page.diagnostics(), TransportKind::GatewayV2);
        for item in page.items() {
            seen_ids.insert(item.id.clone());
        }
    }
    assert_eq!(
        seen_ids, expected_ids,
        "cross-partition query with PPCB disabled must return every written item",
    );

    drop_database(&client, &db_name).await;
    Ok(())
}
// ----------------------------------------------------------------------------
//
// The three tests below close a live-coverage gap: the Gateway 2.0 wrap path
// emits the `PageSize` (0x0004), `Match` (0x0008), and `ReadConsistencyStrategy`
// (0x00FE) RNTBD tokens, but until now those were only asserted at unit level
// in `gateway_v2_dispatch::tests` — which proves *we emit id X*, not that the
// real thin-client proxy accepts id X. A wrong-on-the-wire id (the class of
// bug that hid behind the RCS `0x00F0` → `0x00FE` correction) passes every
// unit test yet would be caught here, because each test asserts a server-side
// behavior that only materializes when the token is both correctly encoded and
// honored by the gateway.

/// Validates the Gateway 2.0 `PageSize` RNTBD token (0x0004) end-to-end.
///
/// Inserts 10 items under a **single logical partition key** (so they all land
/// on one physical partition and page boundaries are deterministic) and runs a
/// `SELECT *` with `max_item_count(3)`. The query must paginate into multiple
/// pages of at most three items each. The `pages_seen >= 2` assertion is the
/// load-bearing one: without the PageSize token on the wire the gateway would
/// return all 10 items in a single page, so a dropped or mis-encoded token id
/// surfaces here as one oversized page.
#[tokio::test]
#[cfg_attr(
    not(any(
        test_category = "gateway_v2",
        test_category = "gateway_v2_multi_region"
    )),
    ignore = "requires test_category 'gateway_v2' and AZURE_COSMOS_GW_V2_ENDPOINT/_KEY"
)]
pub async fn gateway_v2_query_honors_max_item_count_page_size(
) -> Result<(), Box<dyn std::error::Error>> {
    let Some((endpoint, key)) = live_credentials() else {
        return Ok(());
    };

    let client = build_client(&endpoint, &key).await?;
    let (db_name, container) = provision_database_and_container(&client).await?;

    // One logical PK keeps every item on a single physical partition; the
    // single-leaf drain then surfaces one SDK page per server response, which
    // makes the page-size assertions below exact rather than probabilistic.
    let pk_value = format!("pk-{}", azure_core::Uuid::new_v4());
    let total_items: usize = 10;
    for i in 0..total_items {
        let id = format!("page-item-{i:02}");
        let item = GwV2TestItem {
            id: id.clone(),
            pk: pk_value.clone(),
            value: i as i64,
            label: format!("row-{i}"),
        };
        container.create_item(&pk_value, &id, &item, None).await?;
    }

    let page_size = NonZeroU32::new(3).expect("3 is non-zero");
    let options = QueryOptions::default().with_max_item_count(MaxItemCountHint::Limit(page_size));
    let mut pages = container
        .query_items::<GwV2TestItem>(
            Query::from("SELECT * FROM c"),
            FeedScope::partition(pk_value.clone()),
            Some(options),
        )
        .await?
        .into_pages();

    let mut pages_seen = 0_usize;
    let mut total_seen = 0_usize;
    let mut page_lens: Vec<usize> = Vec::new();
    while let Some(page) = pages.next().await {
        let page = page?;
        pages_seen += 1;
        assert_transport_kind(&page.diagnostics(), TransportKind::GatewayV2);
        let len = page.items().len();
        page_lens.push(len);
        total_seen += len;
        assert!(
            len <= 3,
            "page {pages_seen} returned {len} items, exceeding the requested \
             max_item_count of 3 — the PageSize token was not honored",
        );
    }

    assert_eq!(
        total_seen, total_items,
        "every inserted item must be returned across the paged result \
         (page lengths: {page_lens:?})",
    );
    assert!(
        pages_seen >= 2,
        "max_item_count(3) over {total_items} items must paginate into \
         multiple pages; got {pages_seen} page(s) with lengths {page_lens:?} \
         — the PageSize token (0x0004) was not emitted/honored on the \
         Gateway 2.0 wire",
    );

    drop_database(&client, &db_name).await;
    Ok(())
}

/// Validates the Gateway 2.0 `Match` RNTBD token (0x0008) end-to-end via
/// optimistic concurrency.
///
/// A replace guarded by the item's **current** ETag must succeed and roll the
/// ETag forward; a replace guarded by the now-**stale** ETag must fail with
/// `412 PreconditionFailed`. The 412 is the load-bearing assertion: if the
/// Match token were dropped or mis-encoded on the wire the gateway would
/// ignore the precondition and the stale-ETag replace would succeed (200)
/// instead of being rejected.
#[tokio::test]
#[cfg_attr(
    not(any(
        test_category = "gateway_v2",
        test_category = "gateway_v2_multi_region"
    )),
    ignore = "requires test_category 'gateway_v2' and AZURE_COSMOS_GW_V2_ENDPOINT/_KEY"
)]
pub async fn gateway_v2_if_match_precondition_round_trip() -> Result<(), Box<dyn std::error::Error>>
{
    let Some((endpoint, key)) = live_credentials() else {
        return Ok(());
    };

    let client = build_client(&endpoint, &key).await?;
    let (db_name, container) = provision_database_and_container(&client).await?;

    let pk_value = format!("pk-{}", azure_core::Uuid::new_v4());
    let item_id = format!("item-{}", azure_core::Uuid::new_v4());
    let mut item = GwV2TestItem {
        id: item_id.clone(),
        pk: pk_value.clone(),
        value: 1,
        label: "initial".into(),
    };

    let create_resp = container
        .create_item(&pk_value, &item_id, &item, None)
        .await?;
    assert_transport_kind(&create_resp.diagnostics(), TransportKind::GatewayV2);
    let etag_v1: Etag = create_resp
        .headers()
        .etag()
        .expect("create response must surface an ETag")
        .clone();

    // Replace guarded by the CURRENT etag → must succeed and roll the ETag.
    item.value = 2;
    item.label = "updated".into();
    let replace_resp = container
        .replace_item(
            &pk_value,
            &item_id,
            &item,
            Some(
                ItemWriteOptions::default()
                    .with_precondition(Precondition::IfMatch(etag_v1.clone())),
            ),
        )
        .await?;
    assert_transport_kind(&replace_resp.diagnostics(), TransportKind::GatewayV2);
    let etag_v2: Etag = replace_resp
        .headers()
        .etag()
        .expect("replace response must surface an ETag")
        .clone();
    assert_ne!(
        etag_v1.to_string(),
        etag_v2.to_string(),
        "a successful If-Match replace must roll the ETag forward",
    );

    // Replace guarded by the now-STALE v1 etag → must be rejected with 412.
    item.value = 3;
    item.label = "stale-attempt".into();
    let stale = container
        .replace_item(
            &pk_value,
            &item_id,
            &item,
            Some(
                ItemWriteOptions::default()
                    .with_precondition(Precondition::IfMatch(etag_v1.clone())),
            ),
        )
        .await;
    assert_eq!(
        StatusCode::PreconditionFailed,
        stale
            .expect_err("a stale If-Match must be rejected by the server")
            .status()
            .status_code(),
        "stale If-Match must return 412 PreconditionFailed; a 200 means the \
         Match token was not honored on the Gateway 2.0 wire",
    );

    drop_database(&client, &db_name).await;
    Ok(())
}

/// Exercises a non-default [`ReadConsistencyStrategy`] end-to-end on Gateway
/// 2.0.
///
/// A point read issued with `LatestCommitted` must be accepted by the
/// thin-client proxy and return the item over the Gateway 2.0 transport.
/// `LatestCommitted` upgrades the read to a quorum read on Session/Eventual
/// accounts (both live GW_V2 accounts qualify) **without changing the returned
/// payload**, so the assertion is necessarily about wire acceptance + transport
/// rather than an observable payload difference: it proves the proxy does not
/// reject the RCS RNTBD token. The exact token id (`0x00FE`) is pinned by the
/// unit test `wrap_emits_read_consistency_strategy_token_and_drops_consistency_level`
/// in `gateway_v2_dispatch`; this test guards the wire-acceptance half that a
/// unit test cannot reach.
///
/// `LatestCommitted` is a quorum read that is **not** session-effective (it does
/// not carry the session token), so it offers no read-your-writes guarantee —
/// matching .NET/Java, where it deliberately bypasses the session lane. On the
/// multi-writer live accounts an immediate read can land on a replica that has
/// not yet converged on the just-written item and legitimately return `404`, so
/// the read is polled past that brief visibility window before asserting.
#[tokio::test]
#[cfg_attr(
    not(any(
        test_category = "gateway_v2",
        test_category = "gateway_v2_multi_region"
    )),
    ignore = "requires test_category 'gateway_v2' and AZURE_COSMOS_GW_V2_ENDPOINT/_KEY"
)]
pub async fn gateway_v2_read_with_non_default_consistency_strategy(
) -> Result<(), Box<dyn std::error::Error>> {
    let Some((endpoint, key)) = live_credentials() else {
        return Ok(());
    };

    let client = build_client(&endpoint, &key).await?;
    let (db_name, container) = provision_database_and_container(&client).await?;

    let pk_value = format!("pk-{}", azure_core::Uuid::new_v4());
    let item_id = format!("item-{}", azure_core::Uuid::new_v4());
    let item = GwV2TestItem {
        id: item_id.clone(),
        pk: pk_value.clone(),
        value: 42,
        label: "rcs".into(),
    };
    container
        .create_item(&pk_value, &item_id, &item, None)
        .await?;

    const MAX_ATTEMPTS: u32 = 40;
    const POLL_INTERVAL: std::time::Duration = std::time::Duration::from_millis(500);

    for attempt in 0..MAX_ATTEMPTS {
        let read_options = ItemReadOptions::default().with_operation_options(
            OperationOptionsBuilder::new()
                .with_read_consistency_strategy(ReadConsistencyStrategy::LatestCommitted)
                .build(),
        );
        match container
            .read_item(&pk_value, &item_id, Some(read_options))
            .await
        {
            Ok(read_resp) => {
                assert_transport_kind(&read_resp.diagnostics(), TransportKind::GatewayV2);
                assert!(!read_resp.diagnostics().activity_id().as_str().is_empty());
                let read_item: GwV2TestItem = read_resp.into_model()?;
                assert_eq!(
                    read_item, item,
                    "a LatestCommitted read must return the item unchanged",
                );
                drop_database(&client, &db_name).await;
                return Ok(());
            }
            Err(e)
                if e.status().status_code() == StatusCode::NotFound
                    && attempt + 1 < MAX_ATTEMPTS =>
            {
                tokio::time::sleep(POLL_INTERVAL).await;
            }
            Err(e) => {
                return Err(format!("LatestCommitted read failed: {e}").into());
            }
        }
    }
    unreachable!("loop above always returns on the final iteration");
}

/// Proves a freshly-provisioned collection is point-readable over Gateway 2.0
/// from *every* read region the multi-region account advertises, not just the
/// region proximity routing happens to pick by default. This hardens the
/// readiness guarantee from [`wait_for_container_ready`]: a collection that
/// resolves in one region must be usable in all of them. Ignored for
/// single-region runs, where there is only one region to read from.
#[tokio::test]
#[cfg_attr(
    not(test_category = "gateway_v2_multi_region"),
    ignore = "requires the multi-region Gateway 2.0 account (test_category = \"gateway_v2_multi_region\" + AZURE_COSMOS_GW_V2_MULTI_REGION_ENDPOINT/_KEY)"
)]
pub async fn gateway_v2_point_read_usable_from_every_region(
) -> Result<(), Box<dyn std::error::Error>> {
    let Some((endpoint, key)) = live_credentials() else {
        return Ok(());
    };

    // Read regions advertised by the multi-region GW_V2 accounts
    // (`thin-client-multi-writer-ci` / `thin-client-mr-eventual-ci`).
    const REGIONS: [Region; 2] = [Region::CENTRAL_US, Region::EAST_US_2];

    let client = build_client(&endpoint, &key).await?;

    // Provision inline (rather than via `provision_database_and_container`) so
    // the per-region clients can re-resolve the same container by name.
    let unique = azure_core::Uuid::new_v4();
    let db_name = format!("gw_v2-test-db-{unique}");
    let container_name = format!("gw_v2-test-container-{unique}");
    client.create_database(&db_name, None).await?;
    let db_client = client.database_client(&db_name);
    let pk_def: PartitionKeyDefinition = "/pk".into();
    let properties = ContainerProperties::new(container_name.clone(), pk_def);
    db_client.create_container(properties, None).await?;
    let container = wait_for_container_ready(&db_client, &container_name).await?;

    let pk_value = format!("pk-{}", azure_core::Uuid::new_v4());
    let item_id = format!("item-{}", azure_core::Uuid::new_v4());
    let item = GwV2TestItem {
        id: item_id.clone(),
        pk: pk_value.clone(),
        value: 7,
        label: "multi-region".into(),
    };
    let create_resp = container
        .create_item(&pk_value, &item_id, &item, None)
        .await?;
    assert_transport_kind(&create_resp.diagnostics(), TransportKind::GatewayV2);

    for region in REGIONS {
        assert_item_readable_from_region(&endpoint, &key, region, &db_name, &container_name, &item)
            .await?;
    }

    drop_database(&client, &db_name).await;
    Ok(())
}

// Copyright (c) Microsoft Corporation. All rights reserved.
// Licensed under the MIT License.

#![cfg(feature = "key_auth")]

use azure_core::credentials::Secret;
use azure_data_cosmos::diagnostics::{DiagnosticsContext, TransportKind};
use azure_data_cosmos::models::{
    ContainerProperties, PartitionKeyDefinition, ThroughputProperties,
};
use azure_data_cosmos::options::{ConnectionPoolOptions, CreateContainerOptions, Region};
use azure_data_cosmos::{
    AccountEndpoint, AccountReference, CosmosClient, CosmosRuntime, FeedScope, Query,
    RoutingStrategy, SubStatusCode, TransactionalBatch,
};
use futures::StreamExt;
use serde::{Deserialize, Serialize};

fn read_env(name: &str) -> Option<String> {
    std::env::var(name).ok().filter(|v| !v.trim().is_empty())
}

/// Asserts every request recorded in `diagnostics` used `expected` transport.
fn assert_transport_kind(diagnostics: &DiagnosticsContext, expected: TransportKind) {
    let kinds: Vec<TransportKind> = diagnostics
        .requests()
        .iter()
        .map(|r| r.transport_kind())
        .collect();
    assert!(
        !kinds.is_empty(),
        "expected at least one request in diagnostics"
    );
    assert!(
        kinds.iter().all(|k| *k == expected),
        "expected all requests to use {expected:?}, got {kinds:?}"
    );
}

/// Normalizes a Gateway 2.0 endpoint string so it can be parsed by
/// `AccountEndpoint::from_str` (a thin wrapper over `Url::parse`): prepends
/// `https://` and a trailing `/` when missing.
fn normalize_gateway20_endpoint(raw: &str) -> String {
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
/// Multi-region tests (`test_category = "gateway20_multi_region"`) read the
/// multi-region GW20 account; single-region tests read the single-region
/// account. The pair is gated at compile time so the test code stays
/// uniform.
fn live_credentials() -> Option<(String, String)> {
    #[cfg(test_category = "gateway20_multi_region")]
    let (endpoint_var, key_var) = (
        "AZURE_COSMOS_GW20_MULTI_REGION_ENDPOINT",
        "AZURE_COSMOS_GW20_MULTI_REGION_KEY",
    );
    #[cfg(not(test_category = "gateway20_multi_region"))]
    let (endpoint_var, key_var) = ("AZURE_COSMOS_GW20_ENDPOINT", "AZURE_COSMOS_GW20_KEY");

    Some((read_env(endpoint_var)?, read_env(key_var)?))
}

/// Build a [`CosmosClient`] against the live Gateway 2.0 account.
///
/// `gateway20_disabled = false` opts the client in to Gateway 2.0; passing
/// `true` exercises the operator-override path that pins the client to the
/// standard gateway even when the account advertises a Gateway 2.0 endpoint.
async fn build_client(
    endpoint: &str,
    key: &str,
    gateway20_disabled: bool,
) -> Result<CosmosClient, Box<dyn std::error::Error>> {
    let endpoint: AccountEndpoint = normalize_gateway20_endpoint(endpoint).parse()?;
    let account_ref =
        AccountReference::with_authentication_key(endpoint, Secret::from(key.to_string()));
    // Gateway 2.0 disablement is a connection-pool (runtime) concern, so it is
    // configured on a dedicated CosmosRuntime rather than the client builder.
    let runtime = CosmosRuntime::builder()
        .with_connection_pool(
            ConnectionPoolOptions::builder()
                .with_gateway20_disabled(gateway20_disabled)
                .build()?,
        )
        .build()
        .await?;
    let client = CosmosClient::builder()
        .with_runtime(runtime)
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
    let db_name = format!("gw20-test-db-{unique}");
    let container_name = format!("gw20-test-container-{unique}");

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
struct Gw20TestItem {
    id: String,
    pk: String,
    value: i64,
    label: String,
}

/// Drives a point CRUD round-trip (create → read → replace → delete) against
/// the live Gateway 2.0 account.
#[tokio::test]
#[cfg_attr(
    not(any(test_category = "gateway20", test_category = "gateway20_multi_region")),
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

    let create_resp = container
        .create_item(&pk_value, &item_id, &item, None)
        .await?;
    assert_transport_kind(&create_resp.diagnostics(), TransportKind::Gateway20);
    assert!(!create_resp.diagnostics().activity_id().as_str().is_empty());
    assert!(create_resp.diagnostics().duration() > std::time::Duration::ZERO);

    let read_resp = container.read_item(&pk_value, &item_id, None).await?;
    assert_transport_kind(&read_resp.diagnostics(), TransportKind::Gateway20);
    assert!(!read_resp.diagnostics().activity_id().as_str().is_empty());
    let read_item: Gw20TestItem = read_resp.into_model()?;
    assert_eq!(read_item, item);

    item.value = 2;
    item.label = "updated".into();
    let replace_resp = container
        .replace_item(&pk_value, &item_id, &item, None)
        .await?;
    assert_transport_kind(&replace_resp.diagnostics(), TransportKind::Gateway20);
    assert!(!replace_resp.diagnostics().activity_id().as_str().is_empty());

    let delete_resp = container.delete_item(&pk_value, &item_id, None).await?;
    assert_transport_kind(&delete_resp.diagnostics(), TransportKind::Gateway20);
    assert!(!delete_resp.diagnostics().activity_id().as_str().is_empty());

    drop_database(&client, &db_name).await;
    Ok(())
}

/// Exercises a transactional batch routed through Gateway 2.0.
#[tokio::test]
#[cfg_attr(
    not(any(test_category = "gateway20", test_category = "gateway20_multi_region")),
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
    assert_transport_kind(&response.diagnostics(), TransportKind::Gateway20);
    let body = response.into_model()?;
    let codes: Vec<u16> = body.results().iter().map(|r| r.status_code()).collect();
    assert_eq!(codes, vec![201, 201, 201]);

    drop_database(&client, &db_name).await;
    Ok(())
}

/// Verifies that diagnostics are populated for SDK-issued requests routed
/// through Gateway 2.0, including the `Gateway20` transport kind.
#[tokio::test]
#[cfg_attr(
    not(any(test_category = "gateway20", test_category = "gateway20_multi_region")),
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
    container
        .create_item(&pk_value, "diag-item", &item, None)
        .await?;

    let read_resp = container.read_item(&pk_value, "diag-item", None).await?;
    let diagnostics = read_resp.diagnostics();
    assert_transport_kind(&diagnostics, TransportKind::Gateway20);
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

/// Verifies the operator override at the SDK boundary: when the client is
/// built from a [`CosmosRuntime`] whose connection pool sets
/// `with_gateway20_disabled(true)`, every request must route through the
/// standard gateway even though the account advertises a Gateway 2.0 endpoint.
#[tokio::test]
#[cfg_attr(
    not(any(test_category = "gateway20", test_category = "gateway20_multi_region")),
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
    container
        .create_item(&pk_value, "override-item", &item, None)
        .await?;

    let read_resp = container
        .read_item(&pk_value, "override-item", None)
        .await?;
    let diagnostics = read_resp.diagnostics();
    assert_transport_kind(&diagnostics, TransportKind::Gateway);
    assert!(!diagnostics.activity_id().as_str().is_empty());

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
    let container_client = wait_for_container_ready(&db_client, &container_name).await?;

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
#[tokio::test]
#[cfg_attr(
    not(any(test_category = "gateway20", test_category = "gateway20_multi_region")),
    ignore = "requires test_category 'gateway20' and AZURE_COSMOS_GW20_ENDPOINT/_KEY"
)]
pub async fn gateway20_hpk_full_and_partial_partition_key_round_trip(
) -> Result<(), Box<dyn std::error::Error>> {
    use azure_data_cosmos::models::PartitionKeyValue;
    use azure_data_cosmos::PartitionKey;

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
    assert_transport_kind(&read_resp.diagnostics(), TransportKind::Gateway20);
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
        .query_items::<Gw20HpkItem>(query, FeedScope::partition(partial_pk), None)
        .await?
        .into_pages();

    let mut returned_ids: Vec<String> = Vec::new();
    let mut pages_seen = 0_usize;
    while let Some(page) = pages.next().await {
        let page = page?;
        pages_seen += 1;
        assert_transport_kind(&page.diagnostics(), TransportKind::Gateway20);
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
// on `Document` are all Gateway-2.0-eligible per `gateway20_eligibility.rs`).
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
/// at the driver layer, which is the behaviour these tests need to
/// observe.
async fn provision_database_and_multi_partition_container(
    client: &CosmosClient,
) -> Result<(String, azure_data_cosmos::clients::ContainerClient), Box<dyn std::error::Error>> {
    let unique = azure_core::Uuid::new_v4();
    let db_name = format!("gw20-test-db-{unique}");
    let container_name = format!("gw20-test-xpart-container-{unique}");

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
/// `gateway20_eligibility::is_operation_supported_by_gateway20` reports as
/// eligible, so all N leaf requests must route through Gateway 2.0 and
/// their pages must reassemble into the full result set with no
/// duplicates and no drops across partition boundaries.
#[tokio::test]
#[cfg_attr(
    not(any(test_category = "gateway20", test_category = "gateway20_multi_region")),
    ignore = "requires test_category 'gateway20' and AZURE_COSMOS_GW20_ENDPOINT/_KEY"
)]
pub async fn gateway20_cross_partition_query_full_container(
) -> Result<(), Box<dyn std::error::Error>> {
    use std::collections::HashSet;

    let Some((endpoint, key)) = live_credentials() else {
        return Ok(());
    };

    let client = build_client(&endpoint, &key, false).await?;
    let (db_name, container) = provision_database_and_multi_partition_container(&client).await?;

    // 32 distinct logical PKs gives the partition router meaningful spread
    // across the multiple physical partitions provisioned above.
    let total_items: usize = 32;
    let mut expected_ids: HashSet<String> = HashSet::new();
    for i in 0..total_items {
        let pk = format!("pk-{i:02}-{}", azure_core::Uuid::new_v4());
        let id = format!("xpart-item-{i:02}");
        let item = Gw20TestItem {
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
        .query_items::<Gw20TestItem>(query, FeedScope::full_container(), None)
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
    not(any(test_category = "gateway20", test_category = "gateway20_multi_region")),
    ignore = "requires test_category 'gateway20' and AZURE_COSMOS_GW20_ENDPOINT/_KEY"
)]
pub async fn gateway20_cross_partition_query_via_feed_range_full(
) -> Result<(), Box<dyn std::error::Error>> {
    use azure_data_cosmos::feed::FeedRange;
    use std::collections::HashSet;

    let Some((endpoint, key)) = live_credentials() else {
        return Ok(());
    };

    let client = build_client(&endpoint, &key, false).await?;
    let (db_name, container) = provision_database_and_multi_partition_container(&client).await?;

    let total_items: usize = 16;
    let mut expected_ids: HashSet<String> = HashSet::new();
    for i in 0..total_items {
        let pk = format!("pk-{i:02}-{}", azure_core::Uuid::new_v4());
        let id = format!("xpart-range-{i:02}");
        let item = Gw20TestItem {
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
        .query_items::<Gw20TestItem>(query, FeedScope::range(FeedRange::full()), None)
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

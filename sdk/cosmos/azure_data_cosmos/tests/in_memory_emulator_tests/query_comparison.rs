// Copyright (c) Microsoft Corporation. All rights reserved.
// Licensed under the MIT License.

//! Query comparison tests for the in-memory emulator.

#![allow(clippy::large_futures)]

use std::{
    borrow::Cow, collections::BTreeSet, error::Error, num::NonZeroU32, sync::Arc, time::Duration,
};

use azure_core::{
    credentials::Secret,
    http::{Method, Request, StatusCode, Url},
};
use azure_data_cosmos::{
    feed::{ContinuationToken, FeedRange},
    models::{ContainerProperties, PartitionKeyDefinition, PartitionKeyVersion},
    options::{
        ConnectionPoolOptions, ExcludedRegions, MaxItemCountHint, OperationOptions, QueryOptions,
        Region, ServerCertificateValidation,
    },
    AccountEndpoint, AccountReference, ContainerClient, CosmosClient, CosmosClientBuilder,
    CosmosRuntimeBuilder, FeedScope, PartitionKey, Query, RoutingStrategy,
};
use azure_data_cosmos_driver::{
    driver::CosmosDriverRuntime,
    in_memory_emulator::{
        ConsistencyLevel, ContainerConfig, InMemoryEmulatorHttpClient, VirtualAccountConfig,
        VirtualRegion,
    },
    models::partition_key_range::PartitionKeyRange as DriverPartitionKeyRange,
    models::{
        AccountReference as DriverAccountReference, ConnectionString,
        ContainerReference as DriverContainerReference, CosmosOperation, CosmosResponseHeaders,
        EffectivePartitionKey,
    },
    options::{DriverOptions, OperationOptions as DriverOperationOptions},
    CosmosDriver,
};
use futures::StreamExt;
use serde_json::{json, Value};
use uuid::Uuid;

use super::validation::{compare_headers, HeaderValidationSpec};

const EMULATOR_GATEWAY_URL: &str = "https://eastus.emulator.local";
const CONNECTION_STRING_ENV_VAR: &str = "AZURE_COSMOS_CONNECTION_STRING";
const TEST_MODE_ENV_VAR: &str = "AZURE_COSMOS_TEST_MODE";
const EMULATOR_CONNECTION_STRING: &str = "AccountEndpoint=https://127.0.0.1:8081;AccountKey=C2y6yDjf5/R+ob0N8A7Cgv30VRDJIWEHLM+4QDU5DE2nQ9nDuVTqobD4b8mGGyPMbIZnqyMsEcaGQy67XIw/Jw==;";
const HUB_REGION: Region = Region::EAST_US_2;

struct Backend {
    client: CosmosClient,
    driver: Arc<CosmosDriver>,
}

struct QueryComparisonHarness {
    emulator: Backend,
    emulator_http: Arc<InMemoryEmulatorHttpClient>,
    emulator_store: Arc<azure_data_cosmos_driver::in_memory_emulator::EmulatorStore>,
    external: Option<Backend>,
    run_id: String,
}

impl QueryComparisonHarness {
    async fn setup() -> Result<Self, Box<dyn Error>> {
        Self::setup_with_external(true).await
    }

    async fn setup_in_memory_only() -> Result<Self, Box<dyn Error>> {
        Self::setup_with_external(false).await
    }

    async fn setup_with_external(include_external: bool) -> Result<Self, Box<dyn Error>> {
        let _ = tracing_subscriber::fmt::fmt()
            .with_env_filter(tracing_subscriber::EnvFilter::from_default_env())
            .try_init();

        let run_id = Uuid::new_v4().to_string()[..8].to_string();
        let config = VirtualAccountConfig::new(vec![VirtualRegion::new(
            "East US",
            azure_core::http::Url::parse(EMULATOR_GATEWAY_URL)?,
        )])?
        .with_consistency(ConsistencyLevel::Session);
        let emulator = Arc::new(InMemoryEmulatorHttpClient::new(config));
        let emulator_store = emulator.store();

        let emulator_driver_runtime = emulator.runtime_builder().build().await?;
        let emulator_driver_account = DriverAccountReference::with_master_key(
            azure_core::http::Url::parse(EMULATOR_GATEWAY_URL)?,
            "dGVzdGtleQ==",
        );
        let emulator_driver = emulator_driver_runtime
            .create_driver(DriverOptions::builder(emulator_driver_account).build())
            .await?;

        let emulator_account = AccountReference::with_authentication_key(
            EMULATOR_GATEWAY_URL.parse::<AccountEndpoint>()?,
            Secret::new("dGVzdGtleQ=="),
        );
        let emulator_client = CosmosClientBuilder::new()
            .with_runtime(
                CosmosRuntimeBuilder::from(emulator.runtime_builder())
                    .build()
                    .await?,
            )
            .build(
                emulator_account,
                RoutingStrategy::ProximityTo(Region::EAST_US),
            )
            .await?;

        Ok(Self {
            emulator: Backend {
                client: emulator_client,
                driver: emulator_driver,
            },
            emulator_http: Arc::clone(&emulator),
            emulator_store,
            external: if include_external {
                resolve_external_backend().await?
            } else {
                None
            },
            run_id,
        })
    }

    fn database_name(&self) -> String {
        format!("query-cmp-{}", self.run_id)
    }
}

async fn resolve_external_backend() -> Result<Option<Backend>, Box<dyn Error>> {
    let mode = std::env::var(TEST_MODE_ENV_VAR)
        .unwrap_or_default()
        .to_lowercase();
    if mode == "skipped" {
        return Ok(None);
    }

    let raw = match std::env::var(CONNECTION_STRING_ENV_VAR) {
        Ok(value) if !value.is_empty() => value,
        _ if mode == "required" => {
            panic!("{CONNECTION_STRING_ENV_VAR} is not set but {TEST_MODE_ENV_VAR}=required")
        }
        _ => return Ok(None),
    };
    let raw = if raw.eq_ignore_ascii_case("emulator") {
        EMULATOR_CONNECTION_STRING.to_owned()
    } else {
        raw
    };
    let connection: ConnectionString = raw.parse()?;
    let endpoint = connection.account_endpoint().to_owned();
    let key = connection.account_key().secret().to_string();

    let initial_driver =
        build_external_driver(&endpoint, &key, OperationOptions::default(), None).await?;
    let (hub_region, excluded_regions) = resolve_hub_region_and_exclusions(&initial_driver).await;
    let mut default_options = OperationOptions::default();
    default_options.excluded_regions = excluded_regions;

    let client =
        build_external_client(&endpoint, &key, default_options.clone(), hub_region.clone()).await?;
    let driver = build_external_driver(&endpoint, &key, default_options, Some(hub_region)).await?;
    Ok(Some(Backend { client, driver }))
}

async fn build_external_client(
    endpoint: &str,
    key: &str,
    default_options: OperationOptions,
    hub_region: Region,
) -> Result<CosmosClient, Box<dyn Error>> {
    let runtime = CosmosRuntimeBuilder::new()
        .with_connection_pool(connection_pool(endpoint)?)
        .with_default_operation_options(default_options.clone())
        .build()
        .await?;
    let account = AccountReference::with_authentication_key(
        endpoint.parse::<AccountEndpoint>()?,
        Secret::new(key.to_owned()),
    );
    Ok(CosmosClientBuilder::new()
        .with_runtime(runtime)
        .with_default_operation_options(default_options)
        .build(account, RoutingStrategy::ProximityTo(hub_region))
        .await?)
}

async fn build_external_driver(
    endpoint: &str,
    key: &str,
    default_options: DriverOperationOptions,
    hub_region: Option<Region>,
) -> Result<Arc<CosmosDriver>, Box<dyn Error>> {
    let runtime = CosmosDriverRuntime::builder()
        .with_connection_pool(connection_pool(endpoint)?)
        .with_default_operation_options(default_options)
        .build()
        .await?;
    let account = DriverAccountReference::with_master_key(endpoint.parse()?, key.to_owned());
    let mut builder = DriverOptions::builder(account);
    if let Some(region) = hub_region {
        builder = builder.with_preferred_regions(vec![region]);
    }
    Ok(runtime.create_driver(builder.build()).await?)
}

fn connection_pool(endpoint: &str) -> Result<ConnectionPoolOptions, Box<dyn Error>> {
    let mut builder = ConnectionPoolOptions::builder();
    if endpoint.contains("localhost") || endpoint.contains("127.0.0.1") {
        builder = builder.with_server_certificate_validation(
            ServerCertificateValidation::RequiredUnlessEmulator,
        );
    }
    Ok(builder.build()?)
}

async fn resolve_hub_region_and_exclusions(
    driver: &CosmosDriver,
) -> (Region, Option<ExcludedRegions>) {
    let Some((writable, readable)) = driver.cached_account_regions_for_testing().await else {
        return (HUB_REGION, None);
    };
    let mut regions = Vec::<Region>::new();
    for region in writable.into_iter().chain(readable) {
        if !regions.contains(&region) {
            regions.push(region);
        }
    }
    if regions.len() <= 1 {
        return (regions.into_iter().next().unwrap_or(HUB_REGION), None);
    }
    let hub = regions
        .iter()
        .find(|region| **region == HUB_REGION)
        .cloned()
        .unwrap_or_else(|| regions[0].clone());
    let excluded: ExcludedRegions = regions
        .into_iter()
        .filter(|region| *region != hub)
        .collect();
    eprintln!(
        "[query-comparison] pinning external account to hub region {:?}; excluded {} region(s)",
        hub,
        excluded.len()
    );
    (hub, (!excluded.is_empty()).then_some(excluded))
}

#[derive(Clone, Copy)]
enum FixtureKind {
    HashV1,
    HashV2,
    Hpk,
}

impl FixtureKind {
    fn container_name(self) -> &'static str {
        match self {
            FixtureKind::HashV1 => "hash-v1",
            FixtureKind::HashV2 => "hash-v2",
            FixtureKind::Hpk => "hpk",
        }
    }

    fn partition_key_definition(self) -> PartitionKeyDefinition {
        match self {
            FixtureKind::HashV1 => {
                PartitionKeyDefinition::from("/pk").with_version(PartitionKeyVersion::V1)
            }
            FixtureKind::HashV2 => "/pk".into(),
            FixtureKind::Hpk => ("/tenant", "/user", "/session").into(),
        }
    }

    fn documents(self) -> Vec<Value> {
        match self {
            FixtureKind::HashV1 | FixtureKind::HashV2 => vec![
                json!({"id":"hash-a-0","pk":"pk-a","value":0}),
                json!({"id":"hash-a-1","pk":"pk-a","value":1}),
                json!({"id":"hash-a-2","pk":"pk-a","value":2}),
                json!({"id":"hash-b-0","pk":"pk-b","value":10}),
                json!({"id":"hash-b-1","pk":"pk-b","value":11}),
                json!({"id":"hash-c-0","pk":"pk-c","value":20}),
                json!({"id":"hash-d-0","pk":"pk-d","value":30}),
                json!({"id":"hash-d-1","pk":"pk-d","value":31}),
                json!({"id":"hash-e-0","pk":"pk-e","value":40}),
            ],
            FixtureKind::Hpk => vec![
                json!({"id":"hpk-a-u1-s1","tenant":"tenant-a","user":"user-1","session":"session-1","value":0}),
                json!({"id":"hpk-a-u1-s2","tenant":"tenant-a","user":"user-1","session":"session-2","value":1}),
                json!({"id":"hpk-a-u2-s1","tenant":"tenant-a","user":"user-2","session":"session-1","value":2}),
                json!({"id":"hpk-a-u2-s2","tenant":"tenant-a","user":"user-2","session":"session-2","value":3}),
                json!({"id":"hpk-a-u3-s1","tenant":"tenant-a","user":"user-3","session":"session-1","value":4}),
                json!({"id":"hpk-a-u3-s2","tenant":"tenant-a","user":"user-3","session":"session-2","value":5}),
                json!({"id":"hpk-a-u4-s1","tenant":"tenant-a","user":"user-4","session":"session-1","value":6}),
                json!({"id":"hpk-b-u1-s1","tenant":"tenant-b","user":"user-1","session":"session-1","value":10}),
                json!({"id":"hpk-b-u2-s1","tenant":"tenant-b","user":"user-2","session":"session-1","value":11}),
                json!({"id":"hpk-c-u1-s1","tenant":"tenant-c","user":"user-1","session":"session-1","value":20}),
            ],
        }
    }
}

struct FixtureHandles {
    emulator_container: ContainerClient,
    external_container: Option<ContainerClient>,
    emulator_driver_container: DriverContainerReference,
    external_driver_container: Option<DriverContainerReference>,
    documents: Vec<Value>,
    pk_definition: PartitionKeyDefinition,
}

async fn provision_fixture(
    harness: &QueryComparisonHarness,
    db_name: &str,
    fixture: FixtureKind,
) -> Result<FixtureHandles, Box<dyn Error>> {
    provision_fixture_with_topology(harness, db_name, fixture, None, &[]).await
}

async fn provision_fixture_with_topology(
    harness: &QueryComparisonHarness,
    db_name: &str,
    fixture: FixtureKind,
    container_config: Option<ContainerConfig>,
    split_points: &[EffectivePartitionKey],
) -> Result<FixtureHandles, Box<dyn Error>> {
    let pk_definition = fixture.partition_key_definition();
    let container_name = fixture.container_name();
    harness.emulator_store.create_database(db_name);
    if let Some(config) = container_config {
        harness.emulator_store.create_container_with_config(
            db_name,
            container_name,
            pk_definition.clone(),
            config,
        );
    } else {
        harness
            .emulator_store
            .create_container(db_name, container_name, pk_definition.clone());
    }

    create_database_if_needed(&harness.emulator.client, db_name).await?;
    let emulator_driver_container = harness
        .emulator
        .driver
        .resolve_container(db_name, container_name)
        .await?;
    split_physical_partitions_at_points(harness, db_name, container_name, split_points).await?;

    if let Some(external) = &harness.external {
        create_database_if_needed(&external.client, db_name).await?;
        create_container_if_needed(
            &external.client,
            db_name,
            container_name,
            pk_definition.clone(),
        )
        .await?;
    }

    let emulator_container = harness
        .emulator
        .client
        .database_client(db_name)
        .container_client(container_name)
        .await?;
    let external_container = if let Some(external) = &harness.external {
        Some(resolve_container_when_ready(&external.client, db_name, container_name).await?)
    } else {
        None
    };

    let docs = fixture.documents();
    seed_documents(&emulator_container, fixture, &docs).await?;
    if let Some(container) = &external_container {
        seed_documents(container, fixture, &docs).await?;
    }

    let external_driver_container = if let Some(external) = &harness.external {
        Some(
            external
                .driver
                .resolve_container(db_name, container_name)
                .await?,
        )
    } else {
        None
    };

    Ok(FixtureHandles {
        emulator_container,
        external_container,
        emulator_driver_container,
        external_driver_container,
        documents: docs,
        pk_definition,
    })
}

async fn split_physical_partitions_at_points(
    harness: &QueryComparisonHarness,
    db_name: &str,
    container_name: &str,
    split_points: &[EffectivePartitionKey],
) -> Result<(), Box<dyn Error>> {
    for split_epk in split_points {
        let ranges =
            read_emulator_physical_partition_ranges(harness, db_name, container_name).await?;
        if ranges
            .iter()
            .any(|range| range.min_inclusive == *split_epk || range.max_exclusive == *split_epk)
        {
            continue;
        }
        let partition_id = partition_containing_split_epk(&ranges, split_epk)?;
        harness.emulator_store.split_partition_at_epk(
            db_name,
            container_name,
            partition_id,
            split_epk.clone(),
            Duration::ZERO,
        );
        harness
            .emulator_store
            .wait_for_split(db_name, container_name, partition_id)
            .await;
    }
    Ok(())
}

async fn read_emulator_physical_partition_ranges(
    harness: &QueryComparisonHarness,
    db_name: &str,
    container_name: &str,
) -> Result<Vec<DriverPartitionKeyRange>, Box<dyn Error>> {
    let url = format!("{EMULATOR_GATEWAY_URL}/dbs/{db_name}/colls/{container_name}/pkranges");
    let request = Request::new(Url::parse(&url)?, Method::Get);
    let response = harness.emulator_http.execute_request(&request).await?;
    let status = response.status();
    let raw = response.try_into_raw_response().await?;
    if status != StatusCode::Ok {
        return Err(std::io::Error::other(format!(
            "reading pkranges returned {status:?}: {}",
            String::from_utf8_lossy(raw.body().as_ref())
        ))
        .into());
    }
    let body: Value = serde_json::from_slice(raw.body().as_ref())?;
    let ranges = body["PartitionKeyRanges"].as_array().ok_or_else(|| {
        std::io::Error::other(format!(
            "pkranges response did not contain PartitionKeyRanges: {body}"
        ))
    })?;
    let mut ranges = ranges
        .iter()
        .map(|range| serde_json::from_value::<DriverPartitionKeyRange>(range.clone()))
        .collect::<Result<Vec<_>, _>>()?;
    ranges.sort_by(|left, right| left.min_inclusive.cmp(&right.min_inclusive));
    Ok(ranges)
}

fn partition_containing_split_epk(
    ranges: &[DriverPartitionKeyRange],
    split_epk: &EffectivePartitionKey,
) -> Result<u32, Box<dyn Error>> {
    let range = ranges
        .iter()
        .find(|range| range.min_inclusive < *split_epk && *split_epk < range.max_exclusive)
        .ok_or_else(|| {
            std::io::Error::other(format!(
                "split EPK {} is not strictly inside an existing physical partition",
                split_epk.to_hex()
            ))
        })?;
    range.id.parse::<u32>().map_err(|e| -> Box<dyn Error> {
        std::io::Error::other(format!(
            "physical partition id {:?} containing split EPK {} is not a valid u32: {e}",
            range.id,
            split_epk.to_hex(),
        ))
        .into()
    })
}

async fn create_database_if_needed(
    client: &CosmosClient,
    db_name: &str,
) -> Result<(), Box<dyn Error>> {
    match client.create_database(db_name, None).await {
        Ok(_) => Ok(()),
        Err(e) if e.status().status_code() == StatusCode::Conflict => Ok(()),
        Err(e) => Err(Box::new(e)),
    }
}

async fn create_container_if_needed(
    client: &CosmosClient,
    db_name: &str,
    container_name: &str,
    pk_definition: PartitionKeyDefinition,
) -> Result<(), Box<dyn Error>> {
    let props = ContainerProperties::new(container_name.to_owned(), pk_definition);
    match client
        .database_client(db_name)
        .create_container(props, None)
        .await
    {
        Ok(_) => Ok(()),
        Err(e) if e.status().status_code() == StatusCode::Conflict => Ok(()),
        Err(e) => Err(Box::new(e)),
    }
}

async fn resolve_container_when_ready(
    client: &CosmosClient,
    db_name: &str,
    container_name: &str,
) -> Result<ContainerClient, Box<dyn Error>> {
    let deadline = std::time::Instant::now() + super::setup_timeout();
    let mut backoff = Duration::from_millis(250);
    loop {
        match client
            .database_client(db_name)
            .container_client(container_name)
            .await
        {
            Ok(container) => return Ok(container),
            Err(e) if std::time::Instant::now() < deadline => {
                eprintln!("[query-comparison] waiting for container readiness: {e}");
                tokio::time::sleep(backoff).await;
                backoff = (backoff * 2).min(Duration::from_secs(5));
            }
            Err(e) => return Err(Box::new(e)),
        }
    }
}

async fn seed_documents(
    container: &ContainerClient,
    fixture: FixtureKind,
    docs: &[Value],
) -> Result<(), Box<dyn Error>> {
    for doc in docs {
        let id = doc["id"].as_str().expect("seed doc has id");
        container
            .create_item(partition_key_for_doc(fixture, doc), id, doc, None)
            .await?;
    }
    Ok(())
}

fn partition_key_for_doc(fixture: FixtureKind, doc: &Value) -> PartitionKey {
    match fixture {
        FixtureKind::HashV1 | FixtureKind::HashV2 => {
            PartitionKey::from(doc["pk"].as_str().unwrap().to_owned())
        }
        FixtureKind::Hpk => PartitionKey::from((
            doc["tenant"].as_str().unwrap().to_owned(),
            doc["user"].as_str().unwrap().to_owned(),
            doc["session"].as_str().unwrap().to_owned(),
        )),
    }
}

#[derive(Clone, Copy)]
enum Projection {
    Full,
    Fields(&'static [&'static str]),
}

struct Scenario {
    name: &'static str,
    query: Query,
    scope: FeedScope,
    expected_ids: &'static [&'static str],
    projection: Projection,
    compare_external_results: bool,
    compare_external_page_headers: bool,
}

#[tokio::test]
#[cfg_attr(
    test_category = "emulator_vnext",
    ignore = "skipped on vnext emulator: vnext returns the full ['', 'FF') query range for a partition-key equality filter instead of the bounded point EPK range that real Cosmos and the in-memory emulator produce, so the query-plan comparison diverges"
)]
async fn query_results_plans_and_resume_paths_match() -> Result<(), Box<dyn Error>> {
    let harness = QueryComparisonHarness::setup().await?;
    let db_name = harness.database_name();

    let result = async {
        let hash_v1 = provision_fixture(&harness, &db_name, FixtureKind::HashV1).await?;
        let hash_v2 = provision_fixture(&harness, &db_name, FixtureKind::HashV2).await?;
        let hpk = provision_fixture(&harness, &db_name, FixtureKind::Hpk).await?;

        let hash_v1_scenarios = hash_scenarios(&hash_v1.pk_definition)?;
        run_scenarios(&harness, &hash_v1, &hash_v1_scenarios).await?;

        let hash_scenarios = hash_scenarios(&hash_v2.pk_definition)?;
        run_scenarios(&harness, &hash_v2, &hash_scenarios).await?;

        let hpk_scenarios = hpk_scenarios(&hpk.pk_definition)?;
        run_scenarios(&harness, &hpk, &hpk_scenarios).await?;

        Ok::<(), Box<dyn Error>>(())
    }
    .await;

    if let Some(external) = &harness.external {
        let _ = external.client.database_client(&db_name).delete(None).await;
    }
    result
}

#[tokio::test]
async fn query_results_match_across_physical_partition_topologies() -> Result<(), Box<dyn Error>> {
    let harness = QueryComparisonHarness::setup_in_memory_only().await?;

    let hash_single = provision_fixture_with_topology(
        &harness,
        &format!("{}-hash-single", harness.database_name()),
        FixtureKind::HashV2,
        Some(ContainerConfig::new().with_partition_count(1).build()?),
        &[],
    )
    .await?;
    run_topology_scenarios(
        &harness,
        &hash_single,
        &hash_scenarios(&hash_single.pk_definition)?,
    )
    .await?;

    let hash_multi = provision_fixture_with_topology(
        &harness,
        &format!("{}-hash-multi", harness.database_name()),
        FixtureKind::HashV2,
        Some(ContainerConfig::new().with_partition_count(4).build()?),
        &[],
    )
    .await?;
    run_topology_scenarios(
        &harness,
        &hash_multi,
        &hash_scenarios(&hash_multi.pk_definition)?,
    )
    .await?;

    let hpk_single = provision_fixture_with_topology(
        &harness,
        &format!("{}-hpk-single", harness.database_name()),
        FixtureKind::Hpk,
        Some(ContainerConfig::new().with_partition_count(1).build()?),
        &[],
    )
    .await?;
    run_topology_scenarios(
        &harness,
        &hpk_single,
        &hpk_scenarios(&hpk_single.pk_definition)?,
    )
    .await?;

    let hpk_split_points = hpk_tenant_a_split_points(&FixtureKind::Hpk.partition_key_definition());
    let hpk_split = provision_fixture_with_topology(
        &harness,
        &format!("{}-hpk-split", harness.database_name()),
        FixtureKind::Hpk,
        Some(ContainerConfig::new().with_partition_count(1).build()?),
        &hpk_split_points,
    )
    .await?;
    assert_tenant_a_level2_spans_multiple_physical_partitions(&harness, &hpk_split).await?;
    run_topology_scenarios(
        &harness,
        &hpk_split,
        &hpk_scenarios(&hpk_split.pk_definition)?,
    )
    .await?;

    Ok(())
}

// TODO(cosmos): remove `#[ignore]` once the driver's mid-query-split resume bug
// is fixed in a separate PR. The emulator side is complete: it now returns
// `410/1002 PartitionKeyRangeGone` for a query pinned to a split-away pkrange and
// bumps the routing-map ETag on split, so the driver correctly enters split
// recovery. The remaining failure is a driver bug — resuming a continuation
// across the split drops the last leaf's tail document (returns 8/9) — that lives
// in the split-recovery continuation snapshot, not in this test or the emulator.
#[tokio::test]
#[ignore = "driver bug (separate PR): resume across a mid-query partition split drops the last document (8/9); emulator 410-Gone + ETag-on-split fidelity is in place"]
async fn query_resume_survives_mid_query_split() -> Result<(), Box<dyn Error>> {
    // Regression coverage for resuming a query continuation ACROSS a physical
    // partition split. The container starts as a single physical partition; we
    // drain one page, split that partition underneath the outstanding
    // continuation, then resume from the pre-split token. The driver must
    // re-resolve the now-gone parent partition-key range into its children and
    // finish the scan with no lost and no unbounded duplicate documents.
    let harness = QueryComparisonHarness::setup_in_memory_only().await?;
    let db_name = format!("{}-resume-split", harness.database_name());
    let fixture = provision_fixture_with_topology(
        &harness,
        &db_name,
        FixtureKind::HashV2,
        Some(ContainerConfig::new().with_partition_count(1).build()?),
        &[],
    )
    .await?;

    let query = Query::from("SELECT * FROM c");
    let scope = FeedScope::full_container();
    let expected: BTreeSet<String> = fixture
        .documents
        .iter()
        .map(|doc| doc["id"].as_str().expect("seed doc has id").to_owned())
        .collect();

    let mut collected: Vec<Value> = Vec::new();

    // Drain the first page under the single-partition layout, then snapshot the
    // continuation token before mutating the topology.
    let mut pages = fixture
        .emulator_container
        .query_items::<Value>(query.clone(), scope.clone(), Some(query_options(None)))
        .await?
        .into_pages();
    let first_page = pages
        .next()
        .await
        .ok_or_else(|| std::io::Error::other("expected at least one page before splitting"))??;
    collected.extend(first_page.into_items());
    let token = pages.to_continuation_token()?;
    let raw = token.as_str().to_owned();
    drop(pages);
    let mut continuation = Some(ContinuationToken::from_string(raw));

    // Split the single physical partition WHILE the continuation is outstanding.
    split_first_physical_partition(&harness, &db_name, FixtureKind::HashV2.container_name())
        .await?;

    // Resume from the pre-split token under the new two-partition layout.
    let max_pages = expected.len() + 64;
    let mut drained = false;
    for _ in 0..max_pages {
        let mut pages = fixture
            .emulator_container
            .query_items::<Value>(
                query.clone(),
                scope.clone(),
                Some(query_options(continuation.take())),
            )
            .await?
            .into_pages();
        let Some(page) = pages.next().await else {
            drained = true;
            break;
        };
        let page = page?;
        collected.extend(page.into_items());
        let token = pages.to_continuation_token()?;
        let raw = token.as_str().to_owned();
        drop(pages);
        continuation = Some(ContinuationToken::from_string(raw));
    }
    assert!(
        drained,
        "resume across mid-query split did not drain within {max_pages} pages"
    );

    // No loss: every seeded document is observed at least once across the pre-
    // and post-split pages.
    let unique: BTreeSet<String> = collected
        .iter()
        .map(|doc| doc["id"].as_str().expect("query item has id").to_owned())
        .collect();
    assert_eq!(
        expected, unique,
        "resume across mid-query split lost or gained documents"
    );

    // Bounded replay: a split may cause a partition's already-read prefix to be
    // re-scanned, but total observed rows must stay within a small multiple of
    // the document count (no unbounded duplication).
    assert!(
        collected.len() <= expected.len() * 2,
        "resume across mid-query split produced excessive duplication: {} rows for {} documents",
        collected.len(),
        expected.len()
    );

    Ok(())
}

/// Splits the first physical partition of a container at its midpoint, waiting
/// for the split to fully apply. Used to mutate topology while a query
/// continuation is outstanding.
async fn split_first_physical_partition(
    harness: &QueryComparisonHarness,
    db_name: &str,
    container_name: &str,
) -> Result<(), Box<dyn Error>> {
    let ranges = read_emulator_physical_partition_ranges(harness, db_name, container_name).await?;
    let partition_id: u32 = ranges
        .first()
        .ok_or_else(|| std::io::Error::other("container has no physical partitions to split"))?
        .id
        .parse()?;
    harness
        .emulator_store
        .split_partition(db_name, container_name, partition_id, Duration::ZERO);
    harness
        .emulator_store
        .wait_for_split(db_name, container_name, partition_id)
        .await;
    Ok(())
}

fn hash_scenarios(pk_definition: &PartitionKeyDefinition) -> Result<Vec<Scenario>, Box<dyn Error>> {
    let pk_range = FeedRange::for_partition(PartitionKey::from("pk-a"), pk_definition);
    Ok(vec![
        Scenario {
            name: "hash_full_container",
            query: Query::from("SELECT * FROM c"),
            scope: FeedScope::full_container(),
            expected_ids: &[
                "hash-a-0", "hash-a-1", "hash-a-2", "hash-b-0", "hash-b-1", "hash-c-0", "hash-d-0",
                "hash-d-1", "hash-e-0",
            ],
            projection: Projection::Full,
            compare_external_results: true,
            compare_external_page_headers: false,
        },
        Scenario {
            name: "hash_partition_scope",
            query: Query::from("SELECT * FROM c"),
            scope: FeedScope::partition("pk-a"),
            expected_ids: &["hash-a-0", "hash-a-1", "hash-a-2"],
            projection: Projection::Full,
            compare_external_results: true,
            compare_external_page_headers: true,
        },
        Scenario {
            name: "hash_range_scope_with_where",
            query: Query::from("SELECT * FROM c WHERE c.pk = @pk").with_parameter("@pk", "pk-a")?,
            scope: FeedScope::range(pk_range),
            expected_ids: &["hash-a-0", "hash-a-1", "hash-a-2"],
            projection: Projection::Full,
            compare_external_results: true,
            compare_external_page_headers: true,
        },
        Scenario {
            name: "hash_projection_with_where",
            query: Query::from("SELECT c.id, c.pk FROM c WHERE c.pk = @pk")
                .with_parameter("@pk", "pk-a")?,
            scope: FeedScope::partition("pk-a"),
            expected_ids: &["hash-a-0", "hash-a-1", "hash-a-2"],
            projection: Projection::Fields(&["id", "pk"]),
            compare_external_results: true,
            compare_external_page_headers: true,
        },
    ])
}

fn hpk_scenarios(pk_definition: &PartitionKeyDefinition) -> Result<Vec<Scenario>, Box<dyn Error>> {
    let tenant_range =
        explicit_feed_range_for_partition(PartitionKey::from("tenant-a"), pk_definition)?;
    let user_range = explicit_feed_range_for_partition(
        PartitionKey::from(("tenant-a", "user-1")),
        pk_definition,
    )?;
    Ok(vec![
        Scenario {
            name: "hpk_full_scope",
            query: Query::from("SELECT * FROM c"),
            scope: FeedScope::partition(PartitionKey::from(("tenant-a", "user-1", "session-1"))),
            expected_ids: &["hpk-a-u1-s1"],
            projection: Projection::Full,
            compare_external_results: true,
            compare_external_page_headers: true,
        },
        Scenario {
            name: "hpk_full_scope_with_where",
            query: Query::from(
                "SELECT * FROM c WHERE c.tenant = @tenant AND c.user = @user AND c.session = @session",
            )
            .with_parameter("@tenant", "tenant-a")?
            .with_parameter("@user", "user-1")?
            .with_parameter("@session", "session-1")?,
            scope: FeedScope::partition(PartitionKey::from((
                "tenant-a",
                "user-1",
                "session-1",
            ))),
            expected_ids: &["hpk-a-u1-s1"],
            projection: Projection::Full,
            compare_external_results: true,
            compare_external_page_headers: true,
        },
        Scenario {
            name: "hpk_tenant_prefix_where_full_scope",
            query: Query::from("SELECT * FROM c WHERE c.tenant = @tenant")
                .with_parameter("@tenant", "tenant-a")?,
            scope: FeedScope::full_container(),
            expected_ids: &[
                "hpk-a-u1-s1",
                "hpk-a-u1-s2",
                "hpk-a-u2-s1",
                "hpk-a-u2-s2",
                "hpk-a-u3-s1",
                "hpk-a-u3-s2",
                "hpk-a-u4-s1",
            ],
            projection: Projection::Full,
            compare_external_results: false,
            compare_external_page_headers: false,
        },
        Scenario {
            name: "hpk_tenant_prefix_where_and_scope",
            query: Query::from("SELECT * FROM c WHERE c.tenant = @tenant")
                .with_parameter("@tenant", "tenant-a")?,
            scope: FeedScope::range(tenant_range),
            expected_ids: &[
                "hpk-a-u1-s1",
                "hpk-a-u1-s2",
                "hpk-a-u2-s1",
                "hpk-a-u2-s2",
                "hpk-a-u3-s1",
                "hpk-a-u3-s2",
                "hpk-a-u4-s1",
            ],
            projection: Projection::Full,
            compare_external_results: false,
            compare_external_page_headers: false,
        },
        Scenario {
            name: "hpk_level2_prefix_where_and_scope",
            query: Query::from("SELECT * FROM c WHERE c.tenant = @tenant AND c.user = @user")
                .with_parameter("@tenant", "tenant-a")?
                .with_parameter("@user", "user-1")?,
            scope: FeedScope::range(user_range),
            expected_ids: &["hpk-a-u1-s1", "hpk-a-u1-s2"],
            projection: Projection::Full,
            compare_external_results: false,
            compare_external_page_headers: false,
        },
        Scenario {
            name: "hpk_projection_tenant_prefix",
            query: Query::from("SELECT c.id, c.tenant FROM c WHERE c.tenant = @tenant")
                .with_parameter("@tenant", "tenant-a")?,
            scope: FeedScope::range(explicit_feed_range_for_partition(
                PartitionKey::from("tenant-a"),
                pk_definition,
            )?),
            expected_ids: &[
                "hpk-a-u1-s1",
                "hpk-a-u1-s2",
                "hpk-a-u2-s1",
                "hpk-a-u2-s2",
                "hpk-a-u3-s1",
                "hpk-a-u3-s2",
                "hpk-a-u4-s1",
            ],
            projection: Projection::Fields(&["id", "tenant"]),
            compare_external_results: false,
            compare_external_page_headers: false,
        },
        Scenario {
            // Pure prefix-key scope with NO WHERE predicate: tenant isolation is
            // enforced solely by the EPK prefix range derived from the partial
            // partition key, not by any SQL filter. This is the exact code path
            // the HPK over-span bug lived in, so it is exercised on its own.
            name: "hpk_tenant_prefix_partition_scope_no_where",
            query: Query::from("SELECT * FROM c"),
            scope: FeedScope::partition(PartitionKey::from("tenant-a")),
            expected_ids: &[
                "hpk-a-u1-s1",
                "hpk-a-u1-s2",
                "hpk-a-u2-s1",
                "hpk-a-u2-s2",
                "hpk-a-u3-s1",
                "hpk-a-u3-s2",
                "hpk-a-u4-s1",
            ],
            projection: Projection::Full,
            // The standard gateway rejects partial-HPK EPK-range execution, so
            // the external comparison is skipped for this emulator-only path.
            compare_external_results: false,
            compare_external_page_headers: false,
        },
        Scenario {
            // Absent top-level prefix: a never-seeded tenant prefix must return
            // zero documents (no over-span into co-located tenants sharing the
            // same physical partition after a split) while still targeting only
            // the partition(s) covering that prefix range.
            name: "hpk_absent_tenant_prefix",
            query: Query::from("SELECT * FROM c"),
            scope: FeedScope::partition(PartitionKey::from("tenant-absent")),
            expected_ids: &[],
            projection: Projection::Full,
            compare_external_results: false,
            compare_external_page_headers: false,
        },
    ])
}

fn explicit_feed_range_for_partition(
    partition_key: PartitionKey,
    pk_definition: &PartitionKeyDefinition,
) -> Result<FeedRange, Box<dyn Error>> {
    let logical = FeedRange::for_partition(partition_key, pk_definition);
    Ok(FeedRange::new(
        logical.min_inclusive().clone(),
        logical.max_exclusive().clone(),
    )?)
}

fn hpk_tenant_a_split_points(pk_definition: &PartitionKeyDefinition) -> Vec<EffectivePartitionKey> {
    let tenant_range = FeedRange::for_partition(PartitionKey::from("tenant-a"), pk_definition);
    let mut user_boundaries: Vec<_> = ["user-1", "user-2", "user-3", "user-4"]
        .into_iter()
        .map(|user| {
            FeedRange::for_partition(PartitionKey::from(("tenant-a", user)), pk_definition)
                .min_inclusive()
                .clone()
        })
        .collect();
    user_boundaries.sort();
    vec![
        tenant_range.min_inclusive().clone(),
        tenant_range.max_exclusive().clone(),
        user_boundaries[user_boundaries.len() / 2].clone(),
    ]
}

async fn assert_tenant_a_level2_spans_multiple_physical_partitions(
    harness: &QueryComparisonHarness,
    fixture: &FixtureHandles,
) -> Result<(), Box<dyn Error>> {
    let physical_ranges = read_emulator_physical_partition_ranges(
        harness,
        fixture.emulator_driver_container.database_name(),
        fixture.emulator_driver_container.name(),
    )
    .await?;
    let tenant_range =
        FeedRange::for_partition(PartitionKey::from("tenant-a"), &fixture.pk_definition);
    let tenant_ranges =
        physical_partition_ids_overlapping_feed_range(&physical_ranges, &tenant_range);
    assert_eq!(
        2,
        tenant_ranges.len(),
        "tenant-a prefix should span exactly the two physical partitions split inside the top-level HPK"
    );

    let mut level2_partition_ids = BTreeSet::new();
    for user in ["user-1", "user-2", "user-3", "user-4"] {
        let user_range = FeedRange::for_partition(
            PartitionKey::from(("tenant-a", user)),
            &fixture.pk_definition,
        );
        level2_partition_ids.extend(physical_partition_ids_overlapping_feed_range(
            &physical_ranges,
            &user_range,
        ));
    }
    assert!(
        level2_partition_ids.len() > 1,
        "tenant-a level-2 prefixes should land on different physical partitions"
    );
    Ok(())
}

#[derive(Clone)]
struct EpkRangeBounds {
    min: EffectivePartitionKey,
    max: EffectivePartitionKey,
}

impl EpkRangeBounds {
    fn from_feed_range(range: &FeedRange) -> Self {
        Self {
            min: range.min_inclusive().clone(),
            max: range.max_exclusive().clone(),
        }
    }

    fn from_partition_key_range(range: &DriverPartitionKeyRange) -> Self {
        Self {
            min: range.min_inclusive.clone(),
            max: range.max_exclusive.clone(),
        }
    }
}

fn physical_partition_ids_overlapping_feed_range(
    physical_ranges: &[DriverPartitionKeyRange],
    feed_range: &FeedRange,
) -> BTreeSet<String> {
    let target = EpkRangeBounds::from_feed_range(feed_range);
    physical_ranges
        .iter()
        .filter(|physical_range| {
            ranges_overlap(
                &EpkRangeBounds::from_partition_key_range(physical_range),
                &target,
            )
        })
        .map(|physical_range| physical_range.id.clone())
        .collect()
}

async fn expected_touched_partition_ids(
    harness: &QueryComparisonHarness,
    fixture: &FixtureHandles,
    scenario: &Scenario,
) -> Result<BTreeSet<String>, Box<dyn Error>> {
    let plan = fetch_query_plan(
        &harness.emulator.driver,
        &fixture.emulator_driver_container,
        &scenario.query,
    )
    .await?;
    let query_ranges = query_plan_ranges(&plan)?;
    let scope_range = scope_feed_range(&scenario.scope, &fixture.pk_definition);
    let scope_bounds = EpkRangeBounds::from_feed_range(&scope_range);
    let physical_ranges = read_emulator_physical_partition_ranges(
        harness,
        fixture.emulator_driver_container.database_name(),
        fixture.emulator_driver_container.name(),
    )
    .await?;

    let mut expected = BTreeSet::new();
    for physical_range in physical_ranges {
        let physical_bounds = EpkRangeBounds::from_partition_key_range(&physical_range);
        if query_ranges.iter().any(|query_bounds| {
            ranges_overlap(query_bounds, &scope_bounds)
                && ranges_overlap(&physical_bounds, query_bounds)
                && ranges_overlap(&physical_bounds, &scope_bounds)
        }) {
            expected.insert(physical_range.id);
        }
    }
    Ok(expected)
}

fn query_plan_ranges(plan: &Value) -> Result<Vec<EpkRangeBounds>, Box<dyn Error>> {
    let ranges = plan["queryRanges"].as_array().ok_or_else(|| {
        std::io::Error::other(format!("query plan did not contain queryRanges: {plan}"))
    })?;
    ranges
        .iter()
        .map(|range| {
            let min = range["min"].as_str().ok_or_else(|| {
                std::io::Error::other(format!("query range missing min: {range}"))
            })?;
            let max = range["max"].as_str().ok_or_else(|| {
                std::io::Error::other(format!("query range missing max: {range}"))
            })?;
            Ok(EpkRangeBounds {
                min: EffectivePartitionKey::from(min),
                max: EffectivePartitionKey::from(max),
            })
        })
        .collect()
}

fn scope_feed_range(scope: &FeedScope, pk_definition: &PartitionKeyDefinition) -> FeedRange {
    match scope.clone() {
        FeedScope::Partition(partition_key) => {
            FeedRange::for_partition(partition_key, pk_definition)
        }
        FeedScope::Range(range) => range,
        // `FeedScope` is `#[non_exhaustive]`, so this catch-all is required to
        // compile. Any future variant must be handled explicitly here (and in
        // `expected_touched_partition_ids`); silently widening it to
        // `FeedRange::full()` would over-broaden the expected touched-partition
        // set and could mask a routing regression, so fail loudly instead.
        _ => unreachable!(
            "scope_feed_range encountered an unhandled FeedScope variant; \
             update this helper and expected_touched_partition_ids to cover it"
        ),
    }
}

fn ranges_overlap(left: &EpkRangeBounds, right: &EpkRangeBounds) -> bool {
    match (left.min == left.max, right.min == right.max) {
        (true, true) => left.min == right.min,
        (true, false) => right.min <= left.min && left.min < right.max,
        (false, true) => left.min <= right.min && right.min < left.max,
        (false, false) => left.min < right.max && right.min < left.max,
    }
}

fn assert_touched_partition_ids(
    scenario: &Scenario,
    mode: &str,
    headers: &[CosmosResponseHeaders],
    expected: &BTreeSet<String>,
) -> Result<(), Box<dyn Error>> {
    let mut actual = BTreeSet::new();
    for (page_index, headers) in headers.iter().enumerate() {
        let partition_id = headers.partition_key_range_id.as_ref().ok_or_else(|| {
            std::io::Error::other(format!(
                "scenario={} mode={mode} page={page_index} did not return x-ms-documentdb-partitionkeyrangeid",
                scenario.name
            ))
        })?;
        actual.insert(partition_id.clone());
    }
    assert_eq!(
        expected, &actual,
        "{} {mode} touched physical partitions",
        scenario.name
    );
    Ok(())
}

async fn run_scenarios(
    harness: &QueryComparisonHarness,
    fixture: &FixtureHandles,
    scenarios: &[Scenario],
) -> Result<(), Box<dyn Error>> {
    for scenario in scenarios {
        compare_query_plan(harness, fixture, scenario).await?;
        let expected = expected_items(
            &fixture.documents,
            scenario.expected_ids,
            scenario.projection,
        );
        let emulator_collect = drain_collect(
            &fixture.emulator_container,
            scenario,
            "in-memory",
            "collect",
        )
        .await?;
        assert_eq!(
            expected, emulator_collect.items,
            "{} collect",
            scenario.name
        );

        let emulator_resume =
            drain_resume(&fixture.emulator_container, scenario, "in-memory", "resume").await?;
        assert_eq!(expected, emulator_resume.items, "{} resume", scenario.name);

        if let Some(external_container) = &fixture.external_container {
            if !scenario.compare_external_results {
                eprintln!(
                    "[query-comparison] skipping external result drain for scenario={} because standard gateway rejects partial HPK EPK-range execution; query plan is still compared",
                    scenario.name
                );
                continue;
            }
            let external_collect =
                drain_collect(external_container, scenario, "external", "collect").await?;
            assert_eq!(
                expected, external_collect.items,
                "{} external collect",
                scenario.name
            );
            compare_page_headers_if_aligned(
                scenario,
                &external_collect.headers,
                &emulator_collect.headers,
            );

            let external_resume =
                drain_resume(external_container, scenario, "external", "resume").await?;
            assert_eq!(
                expected, external_resume.items,
                "{} external resume",
                scenario.name
            );
            compare_page_headers_if_aligned(
                scenario,
                &external_resume.headers,
                &emulator_resume.headers,
            );
        }
    }
    Ok(())
}

async fn run_topology_scenarios(
    harness: &QueryComparisonHarness,
    fixture: &FixtureHandles,
    scenarios: &[Scenario],
) -> Result<(), Box<dyn Error>> {
    for scenario in scenarios {
        let expected = expected_items(
            &fixture.documents,
            scenario.expected_ids,
            scenario.projection,
        );
        let expected_partition_ids =
            expected_touched_partition_ids(harness, fixture, scenario).await?;

        let collect = drain_collect(
            &fixture.emulator_container,
            scenario,
            "in-memory-topology",
            "collect",
        )
        .await?;
        assert_eq!(
            expected, collect.items,
            "{} topology collect",
            scenario.name
        );
        assert_touched_partition_ids(
            scenario,
            "collect",
            &collect.headers,
            &expected_partition_ids,
        )?;

        let resume = drain_resume(
            &fixture.emulator_container,
            scenario,
            "in-memory-topology",
            "resume",
        )
        .await?;
        assert_eq!(expected, resume.items, "{} topology resume", scenario.name);
        assert_touched_partition_ids(scenario, "resume", &resume.headers, &expected_partition_ids)?;
    }
    Ok(())
}

struct DrainResult {
    items: Vec<Value>,
    headers: Vec<CosmosResponseHeaders>,
}

async fn drain_collect(
    container: &ContainerClient,
    scenario: &Scenario,
    backend: &str,
    mode: &str,
) -> Result<DrainResult, Box<dyn Error>> {
    let mut pages = container
        .query_items::<Value>(
            scenario.query.clone(),
            scenario.scope.clone(),
            Some(query_options(None)),
        )
        .await?
        .into_pages();
    let mut items = Vec::new();
    let mut headers = Vec::new();
    let mut page_index = 0;
    while let Some(page) = pages.next().await {
        let page = page?;
        let page_headers = page.headers().clone().__into_driver_headers();
        log_query_page(
            scenario.name,
            backend,
            mode,
            page_index,
            &page_headers,
            page.query_metrics(),
            page.index_metrics(),
        );
        headers.push(page_headers);
        items.extend(page.into_items());
        page_index += 1;
    }
    Ok(DrainResult {
        items: normalize_items(items, scenario.projection),
        headers,
    })
}

async fn drain_resume(
    container: &ContainerClient,
    scenario: &Scenario,
    backend: &str,
    mode: &str,
) -> Result<DrainResult, Box<dyn Error>> {
    // Upper bound on resume round-trips. With `max_item_count=1` every page
    // yields at most one item, plus a bounded number of empty pages (one per
    // targeted-but-empty physical partition) and one terminal empty page.
    // Derive a generous cap from the expected item count so a runaway or
    // under-draining continuation loop fails with a clear diagnostic instead of
    // silently truncating the result set (which would otherwise surface as a
    // confusing item-count mismatch downstream).
    let max_pages = scenario.expected_ids.len() + 64;
    let mut continuation: Option<ContinuationToken> = None;
    let mut items = Vec::new();
    let mut headers = Vec::new();
    let mut drained = false;
    for page_index in 0..max_pages {
        let mut pages = container
            .query_items::<Value>(
                scenario.query.clone(),
                scenario.scope.clone(),
                Some(query_options(continuation.take())),
            )
            .await?
            .into_pages();
        let Some(page) = pages.next().await else {
            drained = true;
            break;
        };
        let page = page?;
        let page_headers = page.headers().clone().__into_driver_headers();
        log_query_page(
            scenario.name,
            backend,
            mode,
            page_index,
            &page_headers,
            page.query_metrics(),
            page.index_metrics(),
        );
        headers.push(page_headers);
        items.extend(page.into_items());

        let token = pages.to_continuation_token()?;
        let raw = token.as_str().to_owned();
        drop(pages);
        continuation = Some(ContinuationToken::from_string(raw));
    }
    if !drained {
        return Err(std::io::Error::other(format!(
            "scenario={} backend={backend} mode={mode} did not drain within {max_pages} resume round-trips; the continuation loop is under-draining or looping",
            scenario.name
        ))
        .into());
    }
    Ok(DrainResult {
        items: normalize_items(items, scenario.projection),
        headers,
    })
}

fn query_options(continuation: Option<ContinuationToken>) -> QueryOptions {
    let mut options = QueryOptions::default()
        .with_max_item_count(MaxItemCountHint::Limit(NonZeroU32::new(1).unwrap()))
        .with_populate_query_metrics(true)
        .with_populate_index_metrics(true);
    if let Some(token) = continuation {
        options = options.with_continuation_token(token);
    }
    options
}

fn log_query_page(
    scenario: &str,
    backend: &str,
    mode: &str,
    page_index: usize,
    headers: &CosmosResponseHeaders,
    query_metrics: Option<&str>,
    index_metrics: Option<&str>,
) {
    eprintln!(
        "[query-comparison] scenario={scenario} backend={backend} mode={mode} page={page_index} ru={:?} item_count={:?} continuation={:?} activity_id={:?} query_metrics={:?} index_metrics={:?}",
        headers.request_charge.as_ref().map(|c| c.value()),
        headers.item_count,
        headers.continuation,
        headers.activity_id,
        query_metrics.or(headers.query_metrics.as_deref()),
        index_metrics.or(headers.index_metrics.as_deref()),
    );
}

fn compare_page_headers_if_aligned(
    scenario: &Scenario,
    external: &[CosmosResponseHeaders],
    emulator: &[CosmosResponseHeaders],
) {
    if !scenario.compare_external_page_headers {
        eprintln!(
            "[query-comparison] skipping page-header parity for scenario={} because page boundaries are backend-dependent; headers were logged",
            scenario.name
        );
        return;
    }
    if external.len() != emulator.len() {
        eprintln!(
            "[query-comparison] scenario={} page-count differs for header comparison: external={} emulator={}",
            scenario.name,
            external.len(),
            emulator.len()
        );
        return;
    }
    let spec = HeaderValidationSpec::for_query_operation();
    for (idx, (external_headers, emulator_headers)) in external.iter().zip(emulator).enumerate() {
        eprintln!(
            "[query-comparison] comparing page headers scenario={} page={idx}",
            scenario.name
        );
        compare_headers(external_headers, emulator_headers, &spec);
    }
}

async fn compare_query_plan(
    harness: &QueryComparisonHarness,
    fixture: &FixtureHandles,
    scenario: &Scenario,
) -> Result<(), Box<dyn Error>> {
    let emulator_plan = fetch_query_plan(
        &harness.emulator.driver,
        &fixture.emulator_driver_container,
        &scenario.query,
    )
    .await?;
    if let (Some(external), Some(container)) =
        (&harness.external, &fixture.external_driver_container)
    {
        let external_plan = fetch_query_plan(&external.driver, container, &scenario.query).await?;
        assert_eq!(
            external_plan, emulator_plan,
            "query plan mismatch for {}",
            scenario.name
        );
    }
    Ok(())
}

async fn fetch_query_plan(
    driver: &CosmosDriver,
    container: &DriverContainerReference,
    query: &Query,
) -> Result<Value, Box<dyn Error>> {
    let body = serde_json::to_vec(query)?;
    let response = driver
        .execute_singleton_operation(
            CosmosOperation::query_plan(container.clone(), Cow::Borrowed("None")).with_body(body),
            DriverOperationOptions::default(),
        )
        .await?;
    Ok(response.into_body().into_single()?)
}

fn expected_items(docs: &[Value], ids: &[&str], projection: Projection) -> Vec<Value> {
    let mut out: Vec<Value> = ids
        .iter()
        .map(|id| {
            let doc = docs
                .iter()
                .find(|doc| doc["id"].as_str() == Some(*id))
                .expect("expected id exists in fixture");
            project_user_fields(doc.clone(), projection)
        })
        .collect();
    sort_items(&mut out);
    out
}

fn normalize_items(items: Vec<Value>, projection: Projection) -> Vec<Value> {
    let mut out: Vec<Value> = items
        .into_iter()
        .map(|item| project_user_fields(item, projection))
        .collect();
    sort_items(&mut out);
    out
}

fn project_user_fields(mut item: Value, projection: Projection) -> Value {
    let object = item.as_object_mut().expect("query item is object");
    object.retain(|key, _| {
        !matches!(
            key.as_str(),
            "_rid" | "_self" | "_etag" | "_attachments" | "_ts"
        )
    });
    if let Projection::Fields(fields) = projection {
        object.retain(|key, _| fields.contains(&key.as_str()));
    }
    item
}

fn sort_items(items: &mut [Value]) {
    items.sort_by(|left, right| {
        left["id"]
            .as_str()
            .unwrap_or_default()
            .cmp(right["id"].as_str().unwrap_or_default())
    });
}

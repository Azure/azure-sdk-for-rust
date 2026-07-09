// Copyright (c) Microsoft Corporation. All rights reserved.
// Licensed under the MIT License.

//! Query comparison tests for the in-memory emulator.

#![allow(clippy::large_futures)]

use std::{borrow::Cow, error::Error, num::NonZeroU32, sync::Arc, time::Duration};

use azure_core::{credentials::Secret, http::StatusCode};
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
        ConsistencyLevel, InMemoryEmulatorHttpClient, VirtualAccountConfig, VirtualRegion,
    },
    models::{
        AccountReference as DriverAccountReference, ConnectionString,
        ContainerReference as DriverContainerReference, CosmosOperation, CosmosResponseHeaders,
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
    emulator_store: Arc<azure_data_cosmos_driver::in_memory_emulator::EmulatorStore>,
    external: Option<Backend>,
    run_id: String,
}

impl QueryComparisonHarness {
    async fn setup() -> Result<Self, Box<dyn Error>> {
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
            emulator_store,
            external: resolve_external_backend().await?,
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
    let Some((writable, readable)) = driver.__internal_cached_account_regions().await else {
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
            FixtureKind::HashV1 => "hashv1",
            FixtureKind::HashV2 => "hashv2",
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
                json!({"id":"hash-c-0","pk":"pk-c","value":20}),
            ],
            FixtureKind::Hpk => vec![
                json!({"id":"hpk-a-u1-s1","tenant":"tenant-a","user":"user-1","session":"session-1","value":0}),
                json!({"id":"hpk-a-u1-s2","tenant":"tenant-a","user":"user-1","session":"session-2","value":1}),
                json!({"id":"hpk-a-u2-s1","tenant":"tenant-a","user":"user-2","session":"session-1","value":2}),
                json!({"id":"hpk-b-u1-s1","tenant":"tenant-b","user":"user-1","session":"session-1","value":10}),
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
    let pk_definition = fixture.partition_key_definition();
    let container_name = fixture.container_name();
    harness.emulator_store.create_database(db_name);
    harness
        .emulator_store
        .create_container(db_name, container_name, pk_definition.clone());

    create_database_if_needed(&harness.emulator.client, db_name).await?;
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

    let emulator_driver_container = harness
        .emulator
        .driver
        .resolve_container(db_name, container_name)
        .await?;
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
    let deadline = std::time::Instant::now() + Duration::from_secs(120);
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
async fn query_results_plans_and_resume_paths_match() -> Result<(), Box<dyn Error>> {
    let harness = QueryComparisonHarness::setup().await?;
    let db_name = harness.database_name();

    let hash_v1 = provision_fixture(&harness, &db_name, FixtureKind::HashV1).await?;
    let hash_v2 = provision_fixture(&harness, &db_name, FixtureKind::HashV2).await?;
    let hpk = provision_fixture(&harness, &db_name, FixtureKind::Hpk).await?;

    let hash_v1_scenarios = hash_scenarios(&hash_v1.pk_definition)?;
    run_scenarios(&harness, &hash_v1, &hash_v1_scenarios).await?;

    let hash_scenarios = hash_scenarios(&hash_v2.pk_definition)?;
    run_scenarios(&harness, &hash_v2, &hash_scenarios).await?;

    let hpk_scenarios = hpk_scenarios(&hpk.pk_definition)?;
    run_scenarios(&harness, &hpk, &hpk_scenarios).await?;

    if let Some(external) = &harness.external {
        let _ = external.client.database_client(&db_name).delete(None).await;
    }
    Ok(())
}

fn hash_scenarios(pk_definition: &PartitionKeyDefinition) -> Result<Vec<Scenario>, Box<dyn Error>> {
    let pk_range = FeedRange::for_partition(PartitionKey::from("pk-a"), pk_definition);
    Ok(vec![
        Scenario {
            name: "hash_full_container",
            query: Query::from("SELECT * FROM c"),
            scope: FeedScope::full_container(),
            expected_ids: &["hash-a-0", "hash-a-1", "hash-a-2", "hash-b-0", "hash-c-0"],
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
            expected_ids: &["hpk-a-u1-s1", "hpk-a-u1-s2", "hpk-a-u2-s1"],
            projection: Projection::Full,
            compare_external_results: false,
            compare_external_page_headers: false,
        },
        Scenario {
            name: "hpk_tenant_prefix_where_and_scope",
            query: Query::from("SELECT * FROM c WHERE c.tenant = @tenant")
                .with_parameter("@tenant", "tenant-a")?,
            scope: FeedScope::range(tenant_range),
            expected_ids: &["hpk-a-u1-s1", "hpk-a-u1-s2", "hpk-a-u2-s1"],
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
            expected_ids: &["hpk-a-u1-s1", "hpk-a-u1-s2", "hpk-a-u2-s1"],
            projection: Projection::Fields(&["id", "tenant"]),
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
    let mut continuation: Option<ContinuationToken> = None;
    let mut items = Vec::new();
    let mut headers = Vec::new();
    for page_index in 0..100 {
        let mut pages = container
            .query_items::<Value>(
                scenario.query.clone(),
                scenario.scope.clone(),
                Some(query_options(continuation.take())),
            )
            .await?
            .into_pages();
        let Some(page) = pages.next().await else {
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

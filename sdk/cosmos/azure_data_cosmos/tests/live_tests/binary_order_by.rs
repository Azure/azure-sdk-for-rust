// Copyright (c) Microsoft Corporation. All rights reserved.
// Licensed under the MIT License.

//! Live binary ORDER BY comparisons using isolated, matched physical partitions.

use crate::framework::{self, seeded_json::SplitMix64, TestClient, TestOptions, TestRunContext};
use azure_core::fmt::SafeDebug;
use azure_data_cosmos::{
    clients::{ContainerClient, DatabaseClient},
    feed::FeedScope,
    models::{
        CompositeIndex, CompositeIndexOrder, CompositeIndexProperty, ContainerProperties,
        IndexingPolicy, ThroughputProperties,
    },
    options::{
        BinaryEncodingOptions, ConnectionPoolOptions, CreateContainerOptions, MaxItemCountHint,
        QueryOptions, QueryPlanMode, Region,
    },
    AccountReference, CosmosClient, CosmosRuntime, RoutingStrategy,
};
use azure_data_cosmos_driver::{
    binary_json,
    models::{
        AccountReference as DriverAccountReference, ConnectionString, ContainerReference,
        CosmosOperation, FeedRange, ItemView, PartitionKey, ResponseBody,
    },
    options::{DriverOptions, OperationOptions, OperationOptionsBuilder, PlanOptions},
    CosmosDriver, CosmosDriverRuntime,
};
use futures::{stream, StreamExt, TryStreamExt};
use serde::{de::DeserializeOwned, Deserialize, Serialize};
use serde_json::{json, Value};
use std::{
    collections::{BTreeMap, BTreeSet},
    error::Error,
    num::NonZeroU32,
    sync::Arc,
    time::{Duration, Instant},
};

type TestResult<T = ()> = Result<T, Box<dyn Error>>;

const THROUGHPUT_APPROVAL: &str = "AZURE_COSMOS_ORDER_BY_LIVE_THROUGHPUT";
const THROUGHPUT: u64 = 20_000;
const ITEM_COUNT: i64 = 160;
const PAGE_SIZE: u32 = 7;
const FIXTURE_SEED: u64 = 0x5283_5409;

#[derive(Clone, SafeDebug, Serialize, Deserialize, PartialEq)]
struct LiveItem {
    id: String,
    pk: String,
    rank: i64,
    bucket: i64,
    label: String,
    payload: String,
    number: f64,
    nested: serde_json::Value,
    sort_key: String,
}

struct QueryCase {
    name: &'static str,
    query: &'static str,
    expected: Vec<Value>,
    tied: bool,
    resume: bool,
    whole_document: bool,
}

#[derive(Clone, Copy, Debug)]
enum Resume {
    Never,
    Repeated,
    FreshRuntime,
}

#[derive(Clone, Copy)]
struct RunSettings {
    mode: QueryPlanMode,
    binary: bool,
    text_response: bool,
    page_size: u32,
    resume: Resume,
}

struct DriverSource {
    driver: Arc<CosmosDriver>,
    container: ContainerReference,
    connection: ConnectionString,
    database: String,
    name: &'static str,
}

impl Resume {
    fn checkpoint(self, page: u32) -> bool {
        !matches!(self, Self::Never) && page <= 3
    }
}

#[derive(Default)]
struct DriverMeasurements {
    pages: u32,
    resumes: u32,
    contextual_items: u64,
    standalone_bytes: u64,
    maximum_retained_page_bytes: usize,
    request_charge: f64,
}

fn require(condition: bool, message: impl Into<String>) -> TestResult {
    if !condition {
        return Err(message.into().into());
    }
    Ok(())
}

fn items() -> Vec<LiveItem> {
    let mut rng = SplitMix64::new(FIXTURE_SEED);
    (0..ITEM_COUNT)
        .map(|rank| LiveItem {
            id: format!("item-{rank:04}"),
            pk: format!("pk-{}", rank % 64),
            rank,
            bucket: rank % 7,
            label: "repeated label with quotes \" and backslash \\".to_owned(),
            payload: "repeated-string-".repeat(if rank % 16 == 0 { 2048 } else { 128 }),
            number: 9_000_000_000.0 + rank as f64 / 4.0,
            sort_key: format!("{rank:04}-quote \" slash \\ \u{e9}\u{2603}\u{1d11e}"),
            nested: serde_json::json!({
                "same": ["repeated label", "repeated label", ""],
                "active": rank % 2 == 0,
                "optional": null,
                "unicode": "\u{e9}\u{2603}\u{1d11e}",
                "numbers": [0, 31, 32, 255, 256, -1],
                "generated": rng.document(),
            }),
        })
        .collect()
}

fn values(items: &[LiveItem]) -> Vec<Value> {
    items
        .iter()
        .map(|item| serde_json::to_value(item).unwrap())
        .collect()
}

fn cases(items: &[LiveItem]) -> Vec<QueryCase> {
    let mut descending = items.to_vec();
    descending.reverse();
    let mut composite = items.to_vec();
    composite.sort_by_key(|item| (item.bucket, std::cmp::Reverse(item.rank)));
    let mut tied = items.to_vec();
    tied.sort_by_key(|item| (item.bucket, item.rank));
    let mut cases = vec![
        QueryCase {
            name: "ascending",
            query: "SELECT * FROM c ORDER BY c.rank ASC",
            expected: values(items),
            tied: false,
            resume: true,
            whole_document: true,
        },
        QueryCase {
            name: "descending",
            query: "SELECT * FROM c ORDER BY c.rank DESC",
            expected: values(&descending),
            tied: false,
            resume: true,
            whole_document: true,
        },
        QueryCase {
            name: "top",
            query: "SELECT TOP 11 * FROM c ORDER BY c.rank ASC",
            expected: values(&items[..11]),
            tied: false,
            resume: true,
            whole_document: true,
        },
        QueryCase {
            name: "offset-limit",
            query: "SELECT * FROM c ORDER BY c.rank ASC OFFSET 17 LIMIT 23",
            expected: values(&items[17..40]),
            tied: false,
            resume: true,
            whole_document: true,
        },
        QueryCase {
            name: "descending-offset-limit",
            query: "SELECT * FROM c ORDER BY c.rank DESC OFFSET 9 LIMIT 13",
            expected: values(&descending[9..22]),
            tied: false,
            resume: true,
            whole_document: true,
        },
        QueryCase {
            name: "projection",
            query: "SELECT c.id, c.pk, c.rank, c.bucket, c.label, c.payload, c.number, c.nested, c.sort_key FROM c ORDER BY c.rank",
            expected: values(items),
            tied: false,
            resume: false,
            whole_document: true,
        },
        QueryCase {
            name: "value-projection",
            query: "SELECT VALUE c FROM c ORDER BY c.rank",
            expected: values(items),
            tied: false,
            resume: false,
            whole_document: true,
        },
        QueryCase {
            name: "filtered",
            query: "SELECT * FROM c WHERE c.rank >= 80 ORDER BY c.rank",
            expected: values(&items[80..]),
            tied: false,
            resume: false,
            whole_document: true,
        },
        QueryCase {
            name: "mixed-direction-composite",
            query: "SELECT * FROM c ORDER BY c.bucket ASC, c.rank DESC",
            expected: values(&composite),
            tied: false,
            resume: true,
            whole_document: true,
        },
        QueryCase {
            name: "tied-keys",
            query: "SELECT * FROM c ORDER BY c.bucket",
            expected: values(&tied),
            tied: true,
            resume: true,
            whole_document: true,
        },
        QueryCase {
            name: "ordered-distinct",
            query: "SELECT DISTINCT TOP 11 * FROM c ORDER BY c.rank",
            expected: values(&items[..11]),
            tied: false,
            resume: false,
            whole_document: true,
        },
        QueryCase {
            name: "empty",
            query: "SELECT * FROM c WHERE c.rank < 0 ORDER BY c.rank",
            expected: Vec::new(),
            tied: false,
            resume: false,
            whole_document: true,
        },
        QueryCase {
            name: "offset-beyond-end",
            query: "SELECT * FROM c ORDER BY c.rank OFFSET 200 LIMIT 5",
            expected: Vec::new(),
            tied: false,
            resume: false,
            whole_document: true,
        },
    ];
    for (name, query, expected) in [
        (
            "scalar-number",
            "SELECT VALUE c.rank FROM c ORDER BY c.rank",
            items.iter().map(|item| json!(item.rank)).collect(),
        ),
        (
            "scalar-string",
            "SELECT VALUE c.sort_key FROM c ORDER BY c.sort_key",
            items.iter().map(|item| json!(item.sort_key)).collect(),
        ),
        (
            "scalar-boolean",
            "SELECT VALUE c.nested.active FROM c ORDER BY c.rank",
            items.iter().map(|item| json!(item.rank % 2 == 0)).collect(),
        ),
        (
            "scalar-null",
            "SELECT VALUE c.nested.optional FROM c ORDER BY c.rank",
            vec![Value::Null; items.len()],
        ),
        (
            "array-projection",
            "SELECT VALUE c.nested.same FROM c ORDER BY c.rank",
            items
                .iter()
                .map(|item| item.nested["same"].clone())
                .collect(),
        ),
        (
            "generated-projection",
            "SELECT VALUE c.nested.generated FROM c ORDER BY c.rank",
            items
                .iter()
                .map(|item| item.nested["generated"].clone())
                .collect(),
        ),
        (
            "distinct-buckets",
            "SELECT DISTINCT VALUE c.bucket FROM c ORDER BY c.bucket",
            (0..7).map(|bucket| json!(bucket)).collect(),
        ),
        (
            "distinct-window",
            "SELECT DISTINCT VALUE c.bucket FROM c ORDER BY c.bucket OFFSET 2 LIMIT 3",
            (2..5).map(|bucket| json!(bucket)).collect(),
        ),
    ] {
        cases.push(QueryCase {
            name,
            query,
            expected,
            tied: false,
            resume: true,
            whole_document: false,
        });
    }
    cases
}

fn compare(mut actual: Vec<Value>, case: &QueryCase, source: &str) -> TestResult {
    if case.tied {
        require(
            actual
                .windows(2)
                .all(|pair| pair[0]["bucket"].as_i64() <= pair[1]["bucket"].as_i64()),
            format!("{source}/{}: tied keys are not globally ordered", case.name),
        )?;
        // Separate containers have different RIDs, so equal-key order can differ.
        actual.sort_by_key(|item| (item["bucket"].as_i64(), item["rank"].as_i64()));
    }
    require(
        actual == case.expected,
        format!(
            "{source}/{}: item contents, ordering, or count differ",
            case.name
        ),
    )
}

async fn build_sdk_client(connection: &ConnectionString, binary: bool) -> TestResult<CosmosClient> {
    let runtime = CosmosRuntime::builder()
        .with_connection_pool(
            ConnectionPoolOptions::builder()
                .with_gateway_v2_disabled(true)
                .build()?,
        )
        .build()
        .await?;
    Ok(CosmosClient::builder()
        .with_runtime(runtime)
        .with_binary_encoding_options(BinaryEncodingOptions::new().with_enabled(binary))
        .build(
            AccountReference::with_authentication_key(
                connection.account_endpoint().parse()?,
                connection.account_key().clone(),
            ),
            RoutingStrategy::ProximityTo(Region::EAST_US),
        )
        .await?)
}

async fn create_container(
    context: &TestRunContext,
    database: &DatabaseClient,
    name: &'static str,
) -> TestResult<ContainerClient> {
    let index = CompositeIndex::default()
        .with_property(CompositeIndexProperty::new(
            "/bucket",
            CompositeIndexOrder::Ascending,
        ))
        .with_property(CompositeIndexProperty::new(
            "/rank",
            CompositeIndexOrder::Descending,
        ));
    let properties = ContainerProperties::new(name, "/pk".into()).with_indexing_policy(
        IndexingPolicy::default()
            .with_included_path("/*")
            .with_composite_index(index),
    );
    Ok(context
        .create_container(
            database,
            properties,
            Some(
                CreateContainerOptions::default()
                    .with_throughput(ThroughputProperties::manual(THROUGHPUT)),
            ),
        )
        .await?)
}

async fn driver_container(
    connection: &ConnectionString,
    database: &str,
    container: &str,
) -> TestResult<(Arc<CosmosDriver>, ContainerReference)> {
    let runtime = CosmosDriverRuntime::builder()
        .with_connection_pool(
            ConnectionPoolOptions::builder()
                .with_gateway_v2_disabled(true)
                .build()?,
        )
        .build()
        .await?;
    let driver = runtime
        .create_driver(
            DriverOptions::builder(DriverAccountReference::with_account_key(
                connection.account_endpoint().parse()?,
                connection.account_key().clone(),
            ))
            .with_preferred_regions(vec![Region::EAST_US])
            .build(),
        )
        .await?;
    let container = driver
        .resolve_container(database, container, OperationOptions::default())
        .await?;
    Ok((driver, container))
}

async fn validate_ranges(
    driver: &CosmosDriver,
    container: &ContainerReference,
    items: &[LiveItem],
) -> TestResult<Vec<(String, String)>> {
    let ranges = driver
        .resolve_all_partition_key_ranges(container, true)
        .await?
        .ok_or("live container returned no physical ranges")?;
    require(
        ranges.len() >= 2,
        "live container must have at least two physical ranges",
    )?;
    let mut populated = BTreeSet::new();
    for pk in items.iter().map(|item| &item.pk).collect::<BTreeSet<_>>() {
        let mapped = driver
            .resolve_partition_key_ranges_for_key(container, &PartitionKey::from(pk.clone()), false)
            .await?
            .ok_or("test partition key could not be mapped to a physical range")?;
        require(
            mapped.len() == 1,
            "full partition key must map to one physical range",
        )?;
        populated.insert(mapped[0].id.clone());
    }
    require(
        populated.len() >= 2,
        "seeded items must occupy multiple physical ranges",
    )?;
    println!(
        "physical_ranges={} populated_ranges={}",
        ranges.len(),
        populated.len()
    );
    Ok(ranges
        .into_iter()
        .map(|range| {
            (
                range.min_inclusive.to_string(),
                range.max_exclusive.to_string(),
            )
        })
        .collect())
}

async fn run_driver_case(
    source: &DriverSource,
    case: &QueryCase,
    settings: RunSettings,
) -> TestResult<Vec<Value>> {
    let RunSettings {
        mode,
        binary,
        text_response,
        page_size,
        resume,
    } = settings;
    let mut driver = source.driver.clone();
    let mut container = source.container.clone();
    let operation = CosmosOperation::query_items(container.clone(), Some(FeedRange::full()))
        .with_body(serde_json::to_vec(
            &serde_json::json!({"query": case.query, "parameters": []}),
        )?)
        .with_max_item_count(MaxItemCountHint::Limit(NonZeroU32::new(page_size).unwrap()));
    let options = OperationOptionsBuilder::new()
        .with_binary_encoding(
            BinaryEncodingOptions::new()
                .with_enabled(binary)
                .with_request_text_response(text_response),
        )
        .build();
    let plan_options = PlanOptions::default().with_query_plan_mode(mode);
    let mut plan = driver
        .plan_operation(operation.clone(), &options, None, &plan_options)
        .await?;
    let mut actual = Vec::new();
    let mut retained: Option<ItemView> = None;
    let mut measurements = DriverMeasurements::default();
    let started = Instant::now();
    while let Some(response) = driver
        .execute_plan(&mut plan, Some(container.clone()), options.clone())
        .await?
    {
        measurements.pages += 1;
        require(
            measurements.pages <= 500,
            "query did not terminate within 500 pages",
        )?;
        measurements.request_charge += response
            .headers()
            .request_charge
            .map(f64::from)
            .unwrap_or(0.0);
        let body = response.into_body();
        let values: Vec<Value> = if case.whole_document {
            values(&body.clone().into_items::<LiveItem>()?)
        } else {
            body.clone().into_items()?
        };
        require(
            values.len() <= page_size as usize,
            "emitted page exceeded item-count hint",
        )?;
        if !values.is_empty() {
            if binary && !text_response {
                let ResponseBody::ContextualItems(views) = &body else {
                    return Err(format!(
                        "{}: service did not supply page-backed binary ORDER BY items",
                        case.name
                    )
                    .into());
                };
                let mut backings = BTreeMap::new();
                for view in views {
                    require(
                        binary_json::is_binary(view.source_page()),
                        "source page is not binary",
                    )?;
                    require(
                        view.raw_value() == &view.source_page()[view.value_range()],
                        "item span differs from source bytes",
                    )?;
                    let contextual: serde_json::Value = view.deserialize()?;
                    let standalone = view.to_standalone()?;
                    let standalone_value: Value = binary_json::from_slice(&standalone)?;
                    require(
                        standalone_value == contextual,
                        "standalone materialization changed the item",
                    )?;
                    measurements.standalone_bytes += standalone.len() as u64;
                    measurements.contextual_items += 1;
                    backings.insert(view.source_page().as_ptr(), view.source_page().len());
                    retained.get_or_insert_with(|| view.clone());
                }
                measurements.maximum_retained_page_bytes = measurements
                    .maximum_retained_page_bytes
                    .max(backings.values().sum());
            } else {
                for item in body.items()? {
                    require(
                        !binary_json::is_binary(&item),
                        "text output unexpectedly contains binary",
                    )?;
                }
            }
        }
        actual.extend(values);
        if resume.checkpoint(measurements.pages) && actual.len() < case.expected.len() {
            let token = plan.to_continuation_token()?;
            drop(plan);
            if matches!(resume, Resume::FreshRuntime) {
                (driver, container) =
                    driver_container(&source.connection, &source.database, source.name).await?;
            }
            plan = driver
                .plan_operation(operation.clone(), &options, Some(&token), &plan_options)
                .await?;
            measurements.resumes += 1;
        }
    }
    drop(plan);
    drop(driver);
    if let Some(view) = retained {
        let _: Value = view.deserialize()?;
    }
    require(
        matches!(resume, Resume::Never) || measurements.resumes > 0,
        format!("{}: requested driver resume was not exercised", case.name),
    )?;
    compare(actual.clone(), case, "driver")?;
    println!(
        "driver case={} plan={mode:?} binary={binary} text_response={text_response} page_size={page_size} resume={resume:?} resumes={} items={} pages={} contextual_items={} retained_output_page_bytes={} materialized_bytes={} ru={:.2} elapsed_ms={:.2}",
        case.name, measurements.resumes, case.expected.len(), measurements.pages, measurements.contextual_items,
        measurements.maximum_retained_page_bytes, measurements.standalone_bytes,
        measurements.request_charge, started.elapsed().as_secs_f64() * 1000.0,
    );
    Ok(actual)
}

async fn run_sdk_case(
    container: &ContainerClient,
    connection: &ConnectionString,
    database: &str,
    name: &str,
    case: &QueryCase,
    settings: RunSettings,
) -> TestResult<Vec<Value>> {
    if case.whole_document {
        run_sdk_typed::<LiveItem>(container, connection, database, name, case, settings).await
    } else {
        run_sdk_typed::<Value>(container, connection, database, name, case, settings).await
    }
}

async fn run_sdk_typed<T: DeserializeOwned + Serialize + Send + 'static>(
    container: &ContainerClient,
    connection: &ConnectionString,
    database: &str,
    name: &str,
    case: &QueryCase,
    settings: RunSettings,
) -> TestResult<Vec<Value>> {
    let RunSettings {
        mode,
        page_size,
        resume,
        binary,
        ..
    } = settings;
    let mut container = container.clone();
    let options = QueryOptions::default()
        .with_query_plan_mode(mode)
        .with_max_item_count(MaxItemCountHint::Limit(NonZeroU32::new(page_size).unwrap()));
    let mut pages = container
        .query_items::<T>(
            case.query,
            FeedScope::full_container(),
            Some(options.clone()),
        )
        .await?
        .into_pages();
    let mut actual = Vec::new();
    let mut count = 0;
    let mut resumes = 0;
    while let Some(page) = pages.next().await {
        count += 1;
        require(count <= 500, "SDK query did not terminate within 500 pages")?;
        let values = page?.into_items();
        require(
            values.len() <= page_size as usize,
            "SDK page exceeded item-count hint",
        )?;
        actual.extend(
            values
                .iter()
                .map(serde_json::to_value)
                .collect::<Result<Vec<_>, _>>()?,
        );
        if resume.checkpoint(count) && actual.len() < case.expected.len() {
            let token = pages.to_continuation_token()?;
            drop(pages);
            if matches!(resume, Resume::FreshRuntime) {
                let client = build_sdk_client(connection, binary).await?;
                container = client
                    .database_client(database)
                    .container_client(name, None)
                    .await?;
            }
            pages = container
                .query_items::<T>(
                    case.query,
                    FeedScope::full_container(),
                    Some(options.clone().with_continuation_token(token)),
                )
                .await?
                .into_pages();
            resumes += 1;
        }
    }
    require(
        matches!(resume, Resume::Never) || resumes > 0,
        format!("{}: requested SDK resume was not exercised", case.name),
    )?;
    compare(actual.clone(), case, "SDK")?;
    println!(
        "sdk case={} plan={mode:?} binary={binary} page_size={page_size} resume={resume:?} resumes={resumes} items={} pages={count}",
        case.name, case.expected.len(),
    );
    Ok(actual)
}

#[tokio::test]
#[cfg_attr(
    not(test_category = "live"),
    ignore = "requires live credentials and explicit 40,000 RU/s provisioning approval"
)]
async fn live_binary_order_by_matches_text_across_physical_ranges() -> TestResult {
    require(
        std::env::var(THROUGHPUT_APPROVAL).as_deref() == Ok("20000"),
        format!("set {THROUGHPUT_APPROVAL}=20000 only after approving two temporary 20,000 RU/s containers"),
    )?;
    let connection =
        framework::resolve_connection_string().ok_or("live credentials are required")?;
    require(
        !framework::targets_emulator(),
        "this test requires a live account",
    )?;
    require(
        std::env::var("AZURE_COSMOS_TEST_MODE").as_deref() == Ok("required"),
        "live validation requires AZURE_COSMOS_TEST_MODE=required",
    )?;
    TestClient::run_with_unique_db(
        async |context, database| {
            println!("Owned live validation database: {}", context.db_name());
            let _ = create_container(context, database, "order-by-text").await?;
            let _ = create_container(context, database, "order-by-binary").await?;
            let text_client = build_sdk_client(&connection, false).await?;
            let binary_client = build_sdk_client(&connection, true).await?;
            let text = text_client
                .database_client(context.db_name())
                .container_client("order-by-text", None)
                .await?;
            let binary = binary_client
                .database_client(context.db_name())
                .container_client("order-by-binary", None)
                .await?;
            let fixture = items();
            stream::iter(fixture.iter().rev().map(|item| {
                let text = &text;
                let binary = &binary;
                async move {
                    text.create_item(&item.pk, &item.id, item, None).await?;
                    binary.create_item(&item.pk, &item.id, item, None).await?;
                    Ok::<_, azure_data_cosmos::CosmosError>(())
                }
            }))
            .buffer_unordered(8)
            .try_collect::<Vec<_>>()
            .await?;
            let (text_driver, text_ref) =
                driver_container(&connection, &context.db_name(), "order-by-text").await?;
            let (binary_driver, binary_ref) =
                driver_container(&connection, &context.db_name(), "order-by-binary").await?;
            let text_ranges = validate_ranges(&text_driver, &text_ref, &fixture).await?;
            let binary_ranges = validate_ranges(&binary_driver, &binary_ref, &fixture).await?;
            require(
                text_ranges == binary_ranges,
                "paired containers have different physical range boundaries",
            )?;
            let text_source = DriverSource { driver: text_driver, container: text_ref, connection: connection.clone(), database: context.db_name(), name: "order-by-text" };
            let binary_source = DriverSource { driver: binary_driver, container: binary_ref, connection: connection.clone(), database: context.db_name(), name: "order-by-binary" };
            let query_cases = cases(&fixture);
            let database_name = context.db_name();
            println!("fixture_seed={FIXTURE_SEED} query_shapes={} concurrent_shapes=3", query_cases.len());
            for mode in [QueryPlanMode::GatewayOnly, QueryPlanMode::LocalPreferred] {
                stream::iter(query_cases.iter().map(|case| {
                    let text_source = &text_source;
                    let binary_source = &binary_source;
                    let text = &text;
                    let binary = &binary;
                    let connection = &connection;
                    let database_name = &database_name;
                    async move {
                    for page_size in [1, PAGE_SIZE, 64] {
                        let base = RunSettings { mode, binary: false, text_response: false, page_size, resume: Resume::Never };
                        let binary_settings = RunSettings { binary: true, ..base };
                        let text_expected = run_driver_case(text_source, case, base).await?;
                        let binary_expected = run_driver_case(binary_source, case, binary_settings).await?;
                        run_driver_case(binary_source, case, RunSettings { text_response: true, ..binary_settings }).await?;
                        let sdk_text_expected = run_sdk_case(text, connection, database_name, "order-by-text", case, base).await?;
                        let sdk_binary_expected = run_sdk_case(binary, connection, database_name, "order-by-binary", case, binary_settings).await?;
                        let resume_page_size = if case.name.starts_with("distinct-") { 1 } else { PAGE_SIZE };
                        if case.resume && page_size == resume_page_size {
                            for resume in [Resume::Repeated, Resume::FreshRuntime] {
                                let text_actual = run_driver_case(text_source, case, RunSettings { resume, ..base }).await?;
                                let binary_actual = run_driver_case(binary_source, case, RunSettings { resume, ..binary_settings }).await?;
                                let sdk_text_actual = run_sdk_case(text, connection, database_name, "order-by-text", case, RunSettings { resume, ..base }).await?;
                                let sdk_binary_actual = run_sdk_case(binary, connection, database_name, "order-by-binary", case, RunSettings { resume, ..binary_settings }).await?;
                                require(text_actual == text_expected && binary_actual == binary_expected, format!("{}: driver resume differs from same-container uninterrupted order", case.name))?;
                                require(sdk_text_actual == sdk_text_expected && sdk_binary_actual == sdk_binary_expected, format!("{}: SDK resume differs from same-container uninterrupted order", case.name))?;
                            }
                        }
                    }
                    println!(
                        "PASS {} {mode:?}: driver/SDK text/binary and text-response parity",
                        case.name
                    );
                    Ok::<_, Box<dyn Error>>(())
                    }
                }))
                .buffer_unordered(3)
                .try_collect::<Vec<_>>()
                .await?;
            }
            Ok(())
        },
        Some(
            TestOptions::new()
                .with_gateway_v2_disabled(true)
                .with_timeout(Duration::from_secs(1800)),
        ),
    )
    .await
}

#[cfg(test)]
mod tests {
    use super::{cases, compare, items, ITEM_COUNT};

    #[test]
    fn fixture_and_expected_windows_are_consistent() {
        let fixture = items();
        assert_eq!(fixture.len(), ITEM_COUNT as usize);
        for case in cases(&fixture) {
            compare(case.expected.clone(), &case, "fixture").unwrap();
            if case.whole_document {
                let fixture_values = super::values(&fixture);
                assert!(case
                    .expected
                    .iter()
                    .all(|item| fixture_values.contains(item)));
            }
        }
    }

    #[test]
    fn tied_keys_allow_only_in_group_reordering() {
        let fixture = items();
        let case = cases(&fixture).into_iter().find(|case| case.tied).unwrap();
        let mut tied = case.expected.clone();
        tied[..23].reverse();
        compare(tied, &case, "fixture").unwrap();
        let mut wrong = case.expected.clone();
        wrong.swap(0, 159);
        assert!(compare(wrong, &case, "fixture").is_err());
    }
}

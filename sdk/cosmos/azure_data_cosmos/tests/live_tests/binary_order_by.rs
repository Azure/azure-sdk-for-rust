// Copyright (c) Microsoft Corporation. All rights reserved.
// Licensed under the MIT License.

//! Live binary ORDER BY comparisons using isolated, matched physical partitions.

#[path = "binary_order_by/fixtures.rs"]
mod fixtures;

use crate::framework::{self, TestClient, TestOptions, TestRunContext};
use azure_core::{
    http::headers::{HeaderName, HeaderValue, Headers},
    Bytes,
};
use azure_data_cosmos::{
    clients::{ContainerClient, DatabaseClient},
    feed::FeedScope,
    models::{
        CompositeIndex, CompositeIndexOrder, CompositeIndexProperty, ContainerProperties,
        IndexingPolicy, ThroughputProperties,
    },
    options::{
        BinaryEncodingOptions, ConnectionPoolOptions, MaxItemCountHint, QueryOptions,
        QueryPlanMode, Region,
    },
    AccountReference, CosmosClient, CosmosRuntime, RoutingStrategy,
};
use azure_data_cosmos_driver::{
    binary_json,
    diagnostics::RequestSentStatus,
    error::{status_codes, CosmosError},
    models::{
        AccountReference as DriverAccountReference, ConnectionString, ContainerReference,
        CosmosOperation, FeedRange, PartitionKey, ResponseBody,
    },
    options::{DriverOptions, OperationOptions, OperationOptionsBuilder, PlanOptions},
    test::{
        HttpClientConfig, HttpClientFactory, HttpRequest, HttpResponse, TransportClient,
        TransportError,
    },
    CosmosDriver, CosmosDriverRuntime,
};
use fixtures::{cases, compare, items_with_count, values, LiveItem, QueryCase, FIXTURE_SEED};
use futures::{stream, StreamExt, TryStreamExt};
use serde::{de::DeserializeOwned, Serialize};
use serde_json::Value;
use std::{
    collections::BTreeSet,
    error::Error,
    num::NonZeroU32,
    sync::{
        atomic::{AtomicU64, Ordering},
        Arc,
    },
    time::Duration,
};

type TestResult<T = ()> = Result<T, Box<dyn Error>>;

const THROUGHPUT_APPROVAL: &str = "AZURE_COSMOS_ORDER_BY_LIVE_THROUGHPUT";
const PLAN_MODE: &str = "AZURE_COSMOS_ORDER_BY_LIVE_PLAN_MODE";
const THROUGHPUT: u64 = 20_000;
const ITEM_COUNT: i64 = 160;
const PAGE_SIZE: u32 = 7;
const DRIVER_CASES: [&str; 5] = [
    "ascending",
    "offset-limit",
    "distinct-window",
    "array-projection",
    "scalar-null",
];

fn planning_modes(value: Option<&str>) -> TestResult<Vec<QueryPlanMode>> {
    match value {
        None => Ok(vec![
            QueryPlanMode::GatewayOnly,
            QueryPlanMode::LocalPreferred,
        ]),
        Some("GatewayOnly") => Ok(vec![QueryPlanMode::GatewayOnly]),
        Some("LocalPreferred") => Ok(vec![QueryPlanMode::LocalPreferred]),
        Some(value) => Err(format!(
            "invalid {PLAN_MODE}={value:?}; expected GatewayOnly or LocalPreferred"
        )
        .into()),
    }
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
    page_size: u32,
    resume: Resume,
}

#[derive(Debug)]
struct ObservedGatewayFactory(Arc<AtomicU64>);

#[derive(Debug)]
struct ObservedGatewayClient {
    client: reqwest::Client,
    binary_pages: Arc<AtomicU64>,
}

impl HttpClientFactory for ObservedGatewayFactory {
    fn build(
        &self,
        pool: &ConnectionPoolOptions,
        _config: HttpClientConfig,
    ) -> azure_data_cosmos_driver::error::Result<Arc<dyn TransportClient>> {
        let client = reqwest::Client::builder()
            .http1_only()
            .tls_backend_rustls()
            .connect_timeout(pool.max_connect_timeout())
            .timeout(
                pool.max_dataplane_request_timeout()
                    .max(pool.max_metadata_request_timeout()),
            )
            .build()
            .map_err(|source| {
                CosmosError::builder()
                    .with_status(status_codes::CLIENT_HTTP_CLIENT_CONSTRUCTION_FAILED)
                    .with_message("failed to create live observation transport")
                    .with_source(source)
                    .build()
            })?;
        Ok(Arc::new(ObservedGatewayClient {
            client,
            binary_pages: self.0.clone(),
        }))
    }
}

#[async_trait::async_trait]
impl TransportClient for ObservedGatewayClient {
    async fn send(&self, request: &HttpRequest) -> Result<HttpResponse, TransportError> {
        let method = reqwest::Method::from_bytes(request.method.as_str().as_bytes())
            .expect("driver HTTP method is valid");
        let mut builder = self.client.request(method, request.url.clone());
        for (name, value) in request.headers.iter() {
            builder = builder.header(name.as_str(), value.as_str());
        }
        if let Some(body) = &request.body {
            builder = builder.body(body.clone());
        }
        if let Some(timeout) = request.timeout {
            builder = builder.timeout(timeout);
        }
        let response = builder.send().await.map_err(|source| {
            let (status, sent) = if source.is_connect() {
                (
                    status_codes::TRANSPORT_CONNECTION_FAILED,
                    RequestSentStatus::NotSent,
                )
            } else {
                (
                    status_codes::TRANSPORT_IO_FAILED,
                    RequestSentStatus::Unknown,
                )
            };
            TransportError::new(
                CosmosError::builder()
                    .with_status(status)
                    .with_message("live observation request failed")
                    .with_source(source)
                    .build(),
                sent,
            )
        })?;
        let status = response.status().as_u16();
        let mut headers = Headers::new();
        for (name, value) in response.headers() {
            let value = value.to_str().map_err(|source| {
                TransportError::new(
                    CosmosError::builder()
                        .with_status(status_codes::TRANSPORT_IO_FAILED)
                        .with_message("invalid live response header")
                        .with_source(source)
                        .build(),
                    RequestSentStatus::Sent,
                )
            })?;
            headers.insert(
                HeaderName::from(name.as_str().to_owned()),
                HeaderValue::from(value.to_owned()),
            );
        }
        let body = response.bytes().await.map_err(|source| {
            TransportError::new(
                CosmosError::builder()
                    .with_status(status_codes::TRANSPORT_BODY_READ_FAILED)
                    .with_message("failed to read live observation response")
                    .with_source(source)
                    .build(),
                RequestSentStatus::Sent,
            )
        })?;
        if (200..300).contains(&status)
            && binary_json::is_binary(&body)
            && request.headers.iter().any(|(name, value)| {
                name.as_str() == "x-ms-documentdb-isquery"
                    && value.as_str().eq_ignore_ascii_case("true")
            })
        {
            self.binary_pages.fetch_add(1, Ordering::Relaxed);
        }
        Ok(HttpResponse {
            status,
            headers,
            body: body.to_vec(),
        })
    }
}

impl Resume {
    fn checkpoint(self, page: u32) -> bool {
        !matches!(self, Self::Never) && page <= 3
    }
}

fn require(condition: bool, message: impl Into<String>) -> TestResult {
    if !condition {
        return Err(message.into().into());
    }
    Ok(())
}

fn items() -> Vec<LiveItem> {
    items_with_count(ITEM_COUNT)
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
            Some(ThroughputProperties::manual(THROUGHPUT)),
        )
        .await?)
}

async fn driver_container(
    connection: &ConnectionString,
    database: &str,
    container: &str,
    binary_pages: Arc<AtomicU64>,
) -> TestResult<(Arc<CosmosDriver>, ContainerReference)> {
    driver_container_with_factory(
        connection,
        database,
        container,
        Arc::new(ObservedGatewayFactory(binary_pages)),
    )
    .await
}

async fn driver_container_with_factory(
    connection: &ConnectionString,
    database: &str,
    container: &str,
    factory: Arc<dyn HttpClientFactory>,
) -> TestResult<(Arc<CosmosDriver>, ContainerReference)> {
    let runtime = CosmosDriverRuntime::builder()
        .with_connection_pool(
            ConnectionPoolOptions::builder()
                .with_gateway_v2_disabled(true)
                .with_is_http2_allowed(false)
                .build()?,
        )
        .with_mock_http_client_factory(factory)
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

async fn check_driver_output(
    driver: &CosmosDriver,
    container: &ContainerReference,
    case: &QueryCase,
    mode: QueryPlanMode,
    encoding: BinaryEncodingOptions,
) -> TestResult {
    let binary_output = encoding.enabled && !encoding.request_text_response;
    let operation = CosmosOperation::query_items(container.clone(), Some(FeedRange::full()))
        .with_body(serde_json::to_vec(
            &serde_json::json!({"query": case.query, "parameters": []}),
        )?)
        .with_max_item_count(MaxItemCountHint::Limit(NonZeroU32::new(PAGE_SIZE).unwrap()));
    let options = OperationOptionsBuilder::new()
        .with_binary_encoding(encoding)
        .build();
    let plan_options = PlanOptions::default().with_query_plan_mode(mode);
    let mut plan = driver
        .plan_operation(operation, &options, None, &plan_options)
        .await?;
    let mut actual = Vec::new();
    let mut retained: Option<Bytes> = None;
    let mut pages = 0;
    while let Some(response) = driver
        .execute_plan(&mut plan, Some(container.clone()), options.clone())
        .await?
    {
        pages += 1;
        require(
            pages <= 500,
            "driver query did not terminate within 500 pages",
        )?;
        let items = match response.into_body() {
            ResponseBody::Items(items) => items,
            ResponseBody::NoPayload => Vec::new(),
            ResponseBody::Bytes(_) => {
                return Err(format!("{}: expected standalone ORDER BY items", case.name).into());
            }
        };
        require(
            items.len() <= PAGE_SIZE as usize,
            "emitted page exceeded item-count hint",
        )?;
        for item in &items {
            require(
                binary_json::is_binary(item) == binary_output,
                "standalone item has the wrong encoding",
            )?;
            if binary_output {
                retained.get_or_insert_with(|| item.clone());
            }
        }
        actual.extend(ResponseBody::Items(items).into_items::<Value>()?);
    }
    drop(plan);
    if let Some(item) = retained {
        let _: Value = binary_json::from_slice(&item)?;
    }
    compare(actual, case, "driver")
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
    // Selecting live tests is not approval to provision 40,000 RU/s.
    // Require a separate cost opt-in before creating either container.
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
    let selected_mode = match std::env::var(PLAN_MODE) {
        Ok(value) => Some(value),
        Err(std::env::VarError::NotPresent) => None,
        Err(error) => return Err(error.into()),
    };
    let modes = planning_modes(selected_mode.as_deref())?;
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
            let text_binary_pages = Arc::new(AtomicU64::new(0));
            let binary_pages = Arc::new(AtomicU64::new(0));
            let (text_driver, text_ref) = driver_container(
                &connection,
                &context.db_name(),
                "order-by-text",
                text_binary_pages.clone(),
            ).await?;
            let (binary_driver, binary_ref) = driver_container(
                &connection,
                &context.db_name(),
                "order-by-binary",
                binary_pages.clone(),
            ).await?;
            let text_ranges = validate_ranges(&text_driver, &text_ref, &fixture).await?;
            let binary_ranges = validate_ranges(&binary_driver, &binary_ref, &fixture).await?;
            require(
                text_ranges == binary_ranges,
                "paired containers have different physical range boundaries",
            )?;
            let query_cases = cases(&fixture);
            let database_name = context.db_name();
            let sources = [
                (false, &text, "order-by-text"),
                (true, &binary, "order-by-binary"),
            ];
            println!(
                "fixture_seed={FIXTURE_SEED} query_shapes={} concurrent_shapes=3",
                query_cases.len(),
            );
            for mode in modes.iter().copied() {
                stream::iter(query_cases.iter().map(|case| {
                    let sources = &sources;
                    let connection = &connection;
                    let database_name = &database_name;
                    async move {
                        for &(binary, container, name) in sources {
                            for page_size in [1, PAGE_SIZE, 64] {
                                let settings = RunSettings {
                                    mode,
                                    binary,
                                    page_size,
                                    resume: Resume::Never,
                                };
                                let expected = run_sdk_case(
                                    container, connection, database_name, name, case, settings,
                                ).await?;
                                let resume_page_size = if case.name.starts_with("distinct-") {
                                    1
                                } else {
                                    PAGE_SIZE
                                };
                                if case.resume && page_size == resume_page_size {
                                    for resume in [Resume::Repeated, Resume::FreshRuntime] {
                                        let actual = run_sdk_case(
                                            container, connection, database_name, name, case,
                                            RunSettings { resume, ..settings },
                                        ).await?;
                                        require(
                                            actual == expected,
                                            format!("{}: SDK resume differs from same-container uninterrupted order", case.name),
                                        )?;
                                    }
                                }
                            }
                        }
                        println!("PASS {} {mode:?}: SDK text/binary parity", case.name);
                        Ok::<_, Box<dyn Error>>(())
                    }
                }))
                .buffer_unordered(3)
                .try_collect::<Vec<_>>()
                .await?;

                // Raw output contracts are not visible through typed SDK results.
                for name in DRIVER_CASES {
                    let case = query_cases
                        .iter()
                        .find(|case| case.name == name)
                        .ok_or_else(|| format!("missing driver contract case {name}"))?;
                    for text_response in [false, true] {
                        check_driver_output(
                            &binary_driver,
                            &binary_ref,
                            case,
                            mode,
                            BinaryEncodingOptions::new().with_request_text_response(text_response),
                        ).await?;
                    }
                }
                check_driver_output(
                    &text_driver,
                    &text_ref,
                    &query_cases[0],
                    mode,
                    BinaryEncodingOptions::new().with_enabled(false),
                ).await?;
            }
            let observed = binary_pages.load(Ordering::Relaxed);
            require(
                observed > 0,
                "service supplied no binary query pages on the observed transport",
            )?;
            require(
                text_binary_pages.load(Ordering::Relaxed) == 0,
                "text-only driver received binary query pages",
            )?;
            println!("observed_binary_wire_query_pages={observed}");
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
    use super::{cases, compare, items, planning_modes, values, DRIVER_CASES, ITEM_COUNT};
    use azure_data_cosmos::options::QueryPlanMode;

    #[test]
    fn planning_mode_selection_preserves_full_default_matrix_and_rejects_invalid_values() {
        assert!(matches!(
            planning_modes(None).unwrap().as_slice(),
            [QueryPlanMode::GatewayOnly, QueryPlanMode::LocalPreferred]
        ));
        assert!(matches!(
            planning_modes(Some("GatewayOnly")).unwrap().as_slice(),
            [QueryPlanMode::GatewayOnly]
        ));
        assert!(matches!(
            planning_modes(Some("LocalPreferred")).unwrap().as_slice(),
            [QueryPlanMode::LocalPreferred]
        ));
        assert!(planning_modes(Some("invalid")).is_err());
    }

    #[test]
    fn fixture_and_expected_windows_are_consistent() {
        let fixture = items();
        assert_eq!(fixture.len(), ITEM_COUNT as usize);
        let cases = cases(&fixture);
        assert_eq!(cases.len(), 21);
        for name in DRIVER_CASES {
            assert!(cases.iter().any(|case| case.name == name));
        }
        for case in cases {
            compare(case.expected.clone(), &case, "fixture").unwrap();
            if case.whole_document {
                let fixture_values = values(&fixture);
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

// Copyright (c) Microsoft Corporation. All rights reserved.
// Licensed under the MIT License.

//! Account-free aggregation through the real driver, with independently expected results.

use std::{
    num::NonZeroU32,
    sync::{
        atomic::{AtomicBool, Ordering},
        mpsc, Arc, Mutex,
    },
    time::Duration,
};

use azure_core::http::{
    headers::{HeaderName, HeaderValue},
    Method, Request, Url,
};
use azure_data_cosmos_driver::{
    driver::CosmosDriver,
    error::status_codes,
    in_memory_emulator::{
        ContainerConfig, EmulatorStore, InMemoryEmulatorHttpClient, RequestObserver,
        VirtualAccountConfig, VirtualRegion,
    },
    models::{
        AccountReference, ContainerReference, CosmosOperation, EffectivePartitionKey, FeedRange,
        ItemReference, MaxItemCountHint, PartitionKey, PartitionKeyDefinition, ResponseBody,
    },
    options::{
        BinaryEncodingOptions, DriverOptions, OperationOptions, OperationOptionsBuilder,
        PlanOptions, QueryPlanMode,
    },
};
use serde_json::{json, Value};

use super::{parse_json_body, GATEWAY_URL};

static QUERY: HeaderName = HeaderName::from_static("x-ms-documentdb-isquery");
static PLAN: HeaderName = HeaderName::from_static("x-ms-cosmos-is-query-plan-request");
static RANGE: HeaderName = HeaderName::from_static("x-ms-documentdb-partitionkeyrangeid");
static CONTINUATION: HeaderName = HeaderName::from_static("x-ms-continuation");
static FORMATS: HeaderName = HeaderName::from_static("x-ms-cosmos-supported-serialization-formats");

#[derive(Debug, Default)]
struct Recorder {
    store: Mutex<Option<Arc<EmulatorStore>>>,
    queries: Mutex<Vec<(String, Option<String>, bool)>>,
    split: AtomicBool,
}

impl RequestObserver for Recorder {
    fn on_request(&self, request: &Request) {
        let headers = request.headers();
        if headers.get_optional_str(&QUERY).is_none() || headers.get_optional_str(&PLAN).is_some() {
            return;
        }
        let range = headers.get_optional_str(&RANGE).unwrap_or("").to_owned();
        let token = headers.get_optional_str(&CONTINUATION).map(str::to_owned);
        let binary = headers
            .get_optional_str(&FORMATS)
            .is_some_and(|value| value.contains("CosmosBinary"));
        self.queries
            .lock()
            .unwrap()
            .push((range.clone(), token.clone(), binary));
        if token.is_some() && self.split.swap(false, Ordering::SeqCst) {
            let store = self.store.lock().unwrap().as_ref().unwrap().clone();
            let id = range
                .parse()
                .expect("split must target a physical partition");
            let boundary = store.midpoint_split_epk("testdb", "testcoll", id).unwrap();
            let operation = store
                .begin_manual_split_partition("testdb", "testcoll", id, boundary)
                .unwrap();
            // The split test uses worker threads so this dispatch barrier can
            // wait for the control-plane task, rather than race its retry budget.
            let (sender, receiver) = mpsc::channel();
            tokio::spawn(async move {
                sender.send(operation.complete().await).unwrap();
            });
            receiver
                .recv_timeout(Duration::from_secs(10))
                .unwrap()
                .unwrap();
        }
    }
}

struct Context {
    emulator: Arc<InMemoryEmulatorHttpClient>,
    driver: Arc<CosmosDriver>,
    container: ContainerReference,
    recorder: Arc<Recorder>,
}

async fn topology(emulator: &InMemoryEmulatorHttpClient) -> Vec<Value> {
    let request = Request::new(
        Url::parse(&format!("{GATEWAY_URL}/dbs/testdb/colls/testcoll/pkranges")).unwrap(),
        Method::Get,
    );
    let response = emulator
        .execute_request(&request)
        .await
        .unwrap()
        .try_into_raw_response()
        .await
        .unwrap();
    let body: Value = serde_json::from_slice(response.body()).unwrap();
    body["PartitionKeyRanges"].as_array().unwrap().clone()
}

async fn setup(documents: Vec<(usize, Value)>) -> Context {
    let config = VirtualAccountConfig::new(vec![VirtualRegion::new(
        "East US",
        Url::parse(GATEWAY_URL).unwrap(),
    )])
    .unwrap();
    let recorder = Arc::new(Recorder::default());
    let emulator =
        Arc::new(InMemoryEmulatorHttpClient::new(config).with_request_observer(recorder.clone()));
    let definition = PartitionKeyDefinition::from("/pk");
    emulator.store().create_database("testdb");
    emulator.store().create_container_with_config(
        "testdb",
        "testcoll",
        definition.clone(),
        ContainerConfig::new()
            .with_partition_count(2)
            .build()
            .unwrap(),
    );
    *recorder.store.lock().unwrap() = Some(emulator.store());
    let runtime = emulator.runtime_builder().build().await.unwrap();
    let driver = runtime
        .create_driver(
            DriverOptions::builder(AccountReference::with_account_key(
                Url::parse(GATEWAY_URL).unwrap(),
                "ZW11bGF0b3Ita2V5",
            ))
            .build(),
        )
        .await
        .unwrap();
    let container = driver
        .resolve_container("testdb", "testcoll", OperationOptions::default())
        .await
        .unwrap();
    let ranges = topology(&emulator).await;
    assert_eq!(ranges.len(), 2);
    let mut candidates = [Vec::new(), Vec::new()];
    for index in 0..10000 {
        let pk = format!("key-{index}");
        let epk = FeedRange::for_partition(PartitionKey::from(pk.clone()), &definition)
            .min_inclusive()
            .clone();
        for (partition, range) in ranges.iter().enumerate() {
            let min =
                EffectivePartitionKey::try_from(range["minInclusive"].as_str().unwrap()).unwrap();
            let max =
                EffectivePartitionKey::try_from(range["maxExclusive"].as_str().unwrap()).unwrap();
            if min <= epk && epk < max {
                candidates[partition].push((epk.clone(), pk.clone()));
            }
        }
    }
    for values in &mut candidates {
        values.sort();
        assert!(
            values.len() >= 20,
            "both physical partitions must have verified logical keys"
        );
    }
    let mut counts = [0usize; 2];
    for (index, (partition, mut document)) in documents.into_iter().enumerate() {
        let keys = &candidates[partition];
        // Alternate low/high EPKs so a split has remaining documents in both children.
        let ordinal = counts[partition];
        let key_index = if ordinal % 2 == 0 {
            ordinal / 2
        } else {
            keys.len() - 1 - ordinal / 2
        };
        let pk = keys[key_index].1.clone();
        document["pk"] = json!(pk);
        document["id"] = json!(format!("item-{index}"));
        let response = driver
            .execute_singleton_operation(
                CosmosOperation::create_item(ItemReference::from_name(
                    &container,
                    PartitionKey::from(pk),
                    format!("item-{index}"),
                ))
                .with_body(serde_json::to_vec(&document).unwrap()),
                OperationOptions::default(),
            )
            .await
            .unwrap();
        let session = response
            .headers()
            .session_token
            .as_ref()
            .unwrap()
            .to_string();
        assert!(session.starts_with(&format!("{}:", ranges[partition]["id"].as_str().unwrap())));
        counts[partition] += 1;
    }
    recorder.queries.lock().unwrap().clear();
    Context {
        emulator,
        driver,
        container,
        recorder,
    }
}

fn options(binary: bool) -> OperationOptions {
    OperationOptionsBuilder::new()
        .with_binary_encoding(BinaryEncodingOptions::new().with_enabled(binary))
        .build()
}

fn operation(context: &Context, query: &str, parameters: Value, size: u32) -> CosmosOperation {
    CosmosOperation::query_items(context.container.clone(), Some(FeedRange::full()))
        .with_body(serde_json::to_vec(&json!({"query": query, "parameters": parameters})).unwrap())
        .with_max_item_count(MaxItemCountHint::Limit(NonZeroU32::new(size).unwrap()))
}

async fn execute(
    context: &Context,
    query: &str,
    parameters: Value,
    size: u32,
    binary: bool,
) -> Vec<Value> {
    execute_operation(context, operation(context, query, parameters, size), binary).await
}

async fn execute_operation(
    context: &Context,
    operation: CosmosOperation,
    binary: bool,
) -> Vec<Value> {
    context.recorder.queries.lock().unwrap().clear();
    let options = options(binary);
    let mut plan = context
        .driver
        .plan_operation(
            operation,
            &options,
            None,
            &PlanOptions::default().with_query_plan_mode(QueryPlanMode::GatewayOnly),
        )
        .await
        .unwrap();
    let mut values = Vec::new();
    while let Some(response) = context
        .driver
        .execute_plan(&mut plan, Some(context.container.clone()), options.clone())
        .await
        .unwrap()
    {
        match response.into_body() {
            ResponseBody::Items(items) => {
                for item in items {
                    assert_eq!(
                        item.first() == Some(&0x80),
                        binary,
                        "final encoding must follow the operation"
                    );
                    values.push(parse_json_body(&item).unwrap());
                }
            }
            ResponseBody::Bytes(bytes) => {
                let body = parse_json_body(&bytes).unwrap();
                values.extend(body["Documents"].as_array().unwrap().clone());
            }
            ResponseBody::NoPayload => {}
        }
    }
    let queries = context.recorder.queries.lock().unwrap();
    assert!(!queries.is_empty());
    assert!(
        queries.iter().all(|(_, _, mode)| *mode == binary),
        "backend encoding must also be exercised"
    );
    for value in &mut values {
        normalize_numbers(value);
    }
    values
}

#[tokio::test]
async fn logical_partition_small_pages_return_complete_query_results() {
    let context = setup(vec![]).await;
    for (id, pk, key, v) in [
        ("a", "target", "shared", 1),
        ("b", "target", "shared", 3),
        ("c", "target", "other", 5),
        ("excluded", "different", "shared", 100),
    ] {
        context
            .driver
            .execute_singleton_operation(
                CosmosOperation::create_item(ItemReference::from_name(
                    &context.container,
                    PartitionKey::from(pk),
                    id,
                ))
                .with_body(serde_json::to_vec(&json!({"id":id,"pk":pk,"key":key,"v":v})).unwrap()),
                OperationOptions::default(),
            )
            .await
            .unwrap();
    }
    for binary in [false, true] {
        for size in [1, 2, 100] {
            for (query, expected) in [
                ("SELECT VALUE COUNT(1) FROM c", vec![json!(3)]),
                ("SELECT VALUE SUM(c.v) FROM c", vec![json!(9)]),
                ("SELECT VALUE AVG(c.v) FROM c", vec![json!(3)]),
                ("SELECT VALUE MIN(c.v) FROM c", vec![json!(1)]),
                ("SELECT VALUE MAX(c.v) FROM c", vec![json!(5)]),
                (
                    "SELECT c.key, COUNT(1) AS n FROM c GROUP BY c.key",
                    vec![json!({"key":"shared","n":2}), json!({"key":"other","n":1})],
                ),
                (
                    "SELECT DISTINCT VALUE c.key FROM c",
                    vec![json!("shared"), json!("other")],
                ),
            ] {
                let operation = CosmosOperation::query_items(
                    context.container.clone(),
                    Some(FeedRange::for_partition(
                        PartitionKey::from("target"),
                        context.container.partition_key_definition(),
                    )),
                )
                .with_body(serde_json::to_vec(&json!({"query":query,"parameters":[]})).unwrap())
                .with_max_item_count(MaxItemCountHint::Limit(NonZeroU32::new(size).unwrap()));
                assert!(
                    operation.is_trivial(),
                    "regression must exercise the unreduced logical-partition path"
                );
                let rows = execute_operation(&context, operation, binary).await;
                assert_eq!(
                    sorted(rows),
                    sorted(expected),
                    "{query}, size={size}, binary={binary}"
                );
                assert!(
                    context
                        .recorder
                        .queries
                        .lock()
                        .unwrap()
                        .iter()
                        .filter_map(|(_, token, _)| token.as_deref())
                        .all(|token| !token.contains("partial_query_cursor")),
                    "unreduced queries must not receive partial continuations"
                );
            }
        }
    }
}

#[tokio::test]
async fn quoted_aliases_and_float_predicates_execute_in_both_formats() {
    let context = setup(vec![
        (0, json!({"v":1e20,"key":"shared"})),
        (1, json!({"v":2e20,"key":"shared"})),
        (1, json!({"v":1,"key":"other"})),
    ])
    .await;
    for binary in [false, true] {
        for size in [1, 100] {
            for (query, expected) in [
                (
                    r#"SELECT VALUE COUNT(1) FROM c AS "my alias""#,
                    vec![json!(3)],
                ),
                (
                    r#"SELECT VALUE COUNT(1) FROM c AS "my alias" WHERE "my alias".v >= 1e20"#,
                    vec![json!(2)],
                ),
                (
                    r#"SELECT "my alias".key, COUNT(1) AS n FROM c AS "my alias" WHERE "my alias".v >= 1e20 GROUP BY "my alias".key"#,
                    vec![json!({"key":"shared","n":2})],
                ),
                (
                    r#"SELECT VALUE COUNT(1) FROM (SELECT DISTINCT VALUE "my alias".key FROM c AS "my alias" WHERE "my alias".v >= 1e20) AS t"#,
                    vec![json!(1)],
                ),
                (
                    "SELECT VALUE COUNT(1) FROM c WHERE c.v >= 1e20",
                    vec![json!(2)],
                ),
            ] {
                assert_eq!(
                    execute(&context, query, json!([]), size, binary).await,
                    expected,
                    "{query}"
                );
            }
        }
    }
}

fn normalize_numbers(value: &mut Value) {
    match value {
        Value::Number(number) if number.is_f64() => {
            let value = number.as_f64().unwrap();
            if value.fract() == 0.0
                && (-9_223_372_036_854_775_808.0..9_223_372_036_854_775_808.0).contains(&value)
            {
                *number = (value as i64).into();
            }
        }
        Value::Array(values) => values.iter_mut().for_each(normalize_numbers),
        Value::Object(values) => values.values_mut().for_each(normalize_numbers),
        _ => {}
    }
}

fn sorted(mut values: Vec<Value>) -> Vec<Value> {
    values.sort_by_key(Value::to_string);
    values
}

#[tokio::test]
async fn five_aggregates_weighted_average_and_aliases() {
    let context = setup(vec![
        (0, json!({"v": 10, "key": "shared"})),
        (1, json!({"v": 20, "key": "shared"})),
        (1, json!({"v": 30, "key": "shared"})),
        (1, json!({"v": 40, "key": "shared"})),
    ])
    .await;
    for binary in [false, true] {
        for size in [1, 3, 100] {
            for (operator, expected) in [
                ("COUNT", json!(4)),
                ("SUM", json!(100)),
                ("AVG", json!(25)),
                ("MIN", json!(10)),
                ("MAX", json!(40)),
            ] {
                let rows = execute(
                    &context,
                    &format!("SELECT VALUE {operator}(c.v) FROM c"),
                    json!([]),
                    size,
                    binary,
                )
                .await;
                assert_eq!(rows, vec![expected], "{operator}, size={size}");
            }
            let rows = execute(&context, "SELECT COUNT(1), SUM(c.v) AS total, AVG(c.v) AS mean, MIN(c.v) AS low, MAX(c.v) AS high FROM c", json!([]), size, binary).await;
            assert_eq!(
                rows,
                vec![json!({"$1":4,"total":100,"mean":25,"low":10,"high":40})]
            );
            let rows = execute(&context, "SELECT c.key AS category, COUNT(1) AS n, SUM(c.v) AS total, AVG(c.v) AS mean, MIN(c.v) AS low, MAX(c.v) AS high FROM c GROUP BY c.key", json!([]), size, binary).await;
            assert_eq!(
                rows,
                vec![json!({"category":"shared","n":4,"total":100,"mean":25,"low":10,"high":40})]
            );
            let rows = execute(
                &context,
                "SELECT UPPER(c.key) AS label, COUNT(1) AS n FROM c GROUP BY UPPER(c.key)",
                json!([]),
                size,
                binary,
            )
            .await;
            assert_eq!(rows, vec![json!({"label":"SHARED","n":4})]);
            let rows = execute(
                &context,
                "SELECT VALUE {\"label\":c.key} FROM c GROUP BY c.key",
                json!([]),
                size,
                binary,
            )
            .await;
            assert_eq!(rows, vec![json!({"label":"shared"})]);
            let rows = execute(
                &context,
                "SELECT VALUE AVG(c.v) FROM c WHERE c.v >= @min",
                json!([{"name":"@min","value":20}]),
                size,
                binary,
            )
            .await;
            assert_eq!(
                rows,
                vec![json!(30)],
                "empty filtered partitions must not poison AVG"
            );
        }
    }
}

#[tokio::test]
async fn keys_undefined_null_numeric_equivalence_and_group_paging() {
    let context = setup(vec![
        (0, json!({"key":1,"v":2})),
        (1, json!({"key":1.0,"v":3})),
        (0, json!({"key":null,"v":4})),
        (1, json!({"v":5})),
        (0, json!({"key":-0.0,"v":6})),
        (1, json!({"key":0,"v":7})),
        (0, json!({"key":{"a":1,"b":2},"v":8})),
        (1, json!({"key":{"b":2,"a":1.0},"v":9})),
    ])
    .await;
    let expected = vec![
        json!({"key":1,"total":5}),
        json!({"key":null,"total":4}),
        json!({"total":5}),
        json!({"key":0,"total":13}),
        json!({"key":{"a":1,"b":2},"total":17}),
    ];
    for binary in [false, true] {
        for size in [1, 100] {
            let rows = execute(
                &context,
                "SELECT c.key, c.other, SUM(c.v) AS total FROM c GROUP BY c.key, c.other",
                json!([]),
                size,
                binary,
            )
            .await;
            assert_eq!(sorted(rows), sorted(expected.clone()));
            assert_eq!(
                execute(
                    &context,
                    "SELECT VALUE COUNT(1) FROM (SELECT DISTINCT VALUE c.key FROM c) AS t",
                    json!([]),
                    size,
                    binary
                )
                .await,
                vec![json!(4)]
            );
        }
        let options = options(binary);
        let mut plan = context
            .driver
            .plan_operation(
                operation(
                    &context,
                    "SELECT c.key, COUNT(1) AS n FROM c GROUP BY c.key",
                    json!([]),
                    1,
                ),
                &options,
                None,
                &PlanOptions::default().with_query_plan_mode(QueryPlanMode::GatewayOnly),
            )
            .await
            .unwrap();
        assert_eq!(
            plan.to_continuation_token().unwrap_err().status(),
            status_codes::CLIENT_BUFFERED_QUERY_CONTINUATION_UNSUPPORTED
        );
        context
            .driver
            .execute_plan(&mut plan, Some(context.container.clone()), options.clone())
            .await
            .unwrap()
            .unwrap();
        assert_eq!(
            plan.to_continuation_token().unwrap_err().status(),
            status_codes::CLIENT_BUFFERED_QUERY_CONTINUATION_UNSUPPORTED
        );
        while context
            .driver
            .execute_plan(&mut plan, Some(context.container.clone()), options.clone())
            .await
            .unwrap()
            .is_some()
        {}
        assert!(plan.to_continuation_token().is_ok());
    }
}

#[tokio::test]
async fn distinct_count_overlapping_values_and_output_aliases() {
    let context = setup(vec![
        (0, json!({"category":"red"})),
        (0, json!({"category":"blue"})),
        (1, json!({"category":"red"})),
        (1, json!({"category":"green"})),
        (0, json!({})),
    ])
    .await;
    for binary in [false, true] {
        for size in [1, 100] {
            for (projection, expected) in [
                ("VALUE COUNT(1)", json!(3)),
                ("COUNT(1)", json!({"$1":3})),
                ("COUNT(1) AS n", json!({"n":3})),
            ] {
                let query = format!(
                    "SELECT {projection} FROM (SELECT DISTINCT VALUE c.category FROM c) AS t"
                );
                assert_eq!(
                    execute(&context, &query, json!([]), size, binary).await,
                    vec![expected]
                );
            }
            assert_eq!(execute(&context, "SELECT VALUE COUNT(1) FROM (SELECT DISTINCT VALUE c.category FROM c WHERE c.category = @value) AS t", json!([{"name":"@value","value":"red"}]), size, binary).await, vec![json!(1)]);
            assert_eq!(execute(&context, "SELECT VALUE COUNT(1) FROM (SELECT DISTINCT VALUE c.category FROM c ORDER BY c.category) AS t", json!([]), size, binary).await, vec![json!(3)]);
        }
    }
}

#[tokio::test]
async fn empty_and_missing_values_have_service_identities() {
    for documents in [vec![], vec![(0, json!({})), (1, json!({}))]] {
        let context = setup(documents).await;
        for binary in [false, true] {
            for (operator, expected) in [
                ("COUNT", vec![json!(0)]),
                ("SUM", vec![json!(0)]),
                ("AVG", vec![]),
                ("MIN", vec![]),
                ("MAX", vec![]),
            ] {
                assert_eq!(
                    execute(
                        &context,
                        &format!("SELECT VALUE {operator}(c.v) FROM c"),
                        json!([]),
                        1,
                        binary
                    )
                    .await,
                    expected,
                    "{operator}"
                );
            }
            assert_eq!(
                execute(
                    &context,
                    "SELECT VALUE COUNT(1) FROM (SELECT DISTINCT VALUE c.v FROM c) AS t",
                    json!([]),
                    1,
                    binary
                )
                .await,
                vec![json!(0)]
            );
            assert!(execute(
                &context,
                "SELECT c.v FROM c WHERE false GROUP BY c.v",
                json!([]),
                1,
                binary
            )
            .await
            .is_empty());
        }
    }
    let context = setup(vec![(0, json!({"v":2})), (1, json!({"v":null}))]).await;
    for binary in [false, true] {
        assert!(execute(
            &context,
            "SELECT VALUE SUM(c.v) FROM c",
            json!([]),
            1,
            binary
        )
        .await
        .is_empty());
        assert!(execute(
            &context,
            "SELECT VALUE AVG(c.v) FROM c",
            json!([]),
            1,
            binary
        )
        .await
        .is_empty());
        assert_eq!(
            execute(
                &context,
                "SELECT VALUE COUNT(c.v) FROM c",
                json!([]),
                1,
                binary
            )
            .await,
            vec![json!(2)]
        );
        assert_eq!(
            execute(
                &context,
                "SELECT VALUE MIN(c.v) FROM c",
                json!([]),
                1,
                binary
            )
            .await,
            vec![Value::Null]
        );
    }
}

#[tokio::test]
async fn ordinary_aggregate_live_checkpoints_remain_unsupported() {
    let context = setup(vec![(0, json!({"v":10})), (1, json!({"v":20}))]).await;
    for binary in [false, true] {
        for query in [
            "SELECT VALUE SUM(c.v) FROM c",
            "SELECT COUNT(1) AS n FROM c",
        ] {
            let options = options(binary);
            let mut plan = context
                .driver
                .plan_operation(
                    operation(&context, query, json!([]), 1),
                    &options,
                    None,
                    &PlanOptions::default().with_query_plan_mode(QueryPlanMode::GatewayOnly),
                )
                .await
                .unwrap();
            assert_eq!(
                plan.to_continuation_token().unwrap_err().status(),
                status_codes::CLIENT_BUFFERED_QUERY_CONTINUATION_UNSUPPORTED
            );
            assert!(context
                .driver
                .execute_plan(&mut plan, Some(context.container.clone()), options.clone())
                .await
                .unwrap()
                .is_some());
            assert!(plan.to_continuation_token().is_ok());
            assert!(context
                .driver
                .execute_plan(&mut plan, Some(context.container.clone()), options)
                .await
                .unwrap()
                .is_none());
        }
    }
}

#[tokio::test(flavor = "multi_thread", worker_threads = 2)]
async fn actual_split_after_consumed_partial_preserves_results() {
    for binary in [false, true] {
        for (query, expected) in [
            ("SELECT COUNT(1) AS n, SUM(c.v) AS sum, AVG(c.v) AS avg, MIN(c.v) AS min, MAX(c.v) AS max FROM c", vec![json!({"n":8,"sum":36,"avg":4.5,"min":1,"max":8})]),
            ("SELECT c.key, COUNT(1) AS n, AVG(c.v) AS avg FROM c GROUP BY c.key", vec![json!({"key":"shared","n":8,"avg":4.5})]),
            ("SELECT VALUE COUNT(1) FROM (SELECT DISTINCT VALUE c.category FROM c) AS t", vec![json!(3)]),
        ] {
            let documents = (0..8).map(|i| (if i < 4 {0} else {1}, json!({"v":i+1,"key":"shared","category": (["red","blue","red","green"][i % 4])}))).collect();
            let context = setup(documents).await;
            context.recorder.split.store(true, Ordering::SeqCst);
            let rows = execute(&context, query, json!([]), 1, binary).await;
            assert_eq!(rows, expected);
            context.emulator.store().drain_pending_control_plane().await;
            let ranges = topology(&context.emulator).await;
            assert_eq!(ranges.len(), 3, "the split must actually replace the parent");
            let queries = context.recorder.queries.lock().unwrap();
            assert!(queries.iter().any(|(range, token, _)| range == "0" && token.is_some()), "a parent prefix must be consumed before the split");
            for child in ranges.iter().filter(|range| range["id"] != "1") {
                let id = child["id"].as_str().unwrap();
                assert!(queries.iter().any(|(range, token, _)| range == id && token.is_some()), "each child must inherit the parent continuation");
            }

        }
    }
}

fn raw_request(query: &str, range: &str, size: u32, token: Option<&str>, binary: bool) -> Request {
    let mut request = Request::new(
        Url::parse(&format!("{GATEWAY_URL}/dbs/testdb/colls/testcoll/docs")).unwrap(),
        Method::Post,
    );
    request.set_body(serde_json::to_vec(&json!({"query":query,"parameters":[]})).unwrap());
    request
        .headers_mut()
        .insert(QUERY.clone(), HeaderValue::from_static("True"));
    request
        .headers_mut()
        .insert(RANGE.clone(), HeaderValue::from(range.to_owned()));
    request.headers_mut().insert(
        HeaderName::from_static("x-ms-max-item-count"),
        HeaderValue::from(size.to_string()),
    );
    request.headers_mut().insert(
        FORMATS.clone(),
        HeaderValue::from_static(if binary { "CosmosBinary" } else { "JsonText" }),
    );
    if let Some(token) = token {
        request
            .headers_mut()
            .insert(CONTINUATION.clone(), HeaderValue::from(token.to_owned()));
    }
    request
}

async fn raw_page(
    context: &Context,
    request: &Request,
    size: u32,
    binary: bool,
) -> (Vec<Value>, Option<String>) {
    let response = context
        .emulator
        .execute_request(request)
        .await
        .unwrap()
        .try_into_raw_response()
        .await
        .unwrap();
    assert_eq!(response.status(), azure_core::http::StatusCode::Ok);
    assert_eq!(
        azure_data_cosmos_driver::binary_json::is_binary(response.body()),
        binary
    );
    let next = response
        .headers()
        .get_optional_str(&CONTINUATION)
        .map(str::to_owned);
    let body = parse_json_body(response.body()).unwrap();
    let rows = body["Documents"].as_array().unwrap().clone();
    assert!(rows.len() <= size as usize);
    (rows, next)
}

#[tokio::test]
async fn original_unscoped_queries_page_final_results_not_partial_rows() {
    let context = setup(vec![
        (0, json!({"key":"shared"})),
        (0, json!({"key":"shared"})),
        (0, json!({"key":"other"})),
    ])
    .await;
    for binary in [false, true] {
        for (query, expected) in [
            ("SELECT VALUE COUNT(1) FROM c", vec![json!(3)]),
            (
                "SELECT c.key, COUNT(1) AS n FROM c GROUP BY c.key",
                vec![json!({"key":"shared","n":2}), json!({"key":"other","n":1})],
            ),
            (
                "SELECT DISTINCT VALUE c.key FROM c",
                vec![json!("shared"), json!("other")],
            ),
        ] {
            let mut continuation = None;
            let mut results = Vec::new();
            loop {
                let mut request = raw_request(query, "0", 1, continuation.as_deref(), binary);
                request.headers_mut().remove(RANGE.clone());
                let (mut page, next) = raw_page(&context, &request, 1, binary).await;
                page.iter_mut().for_each(normalize_numbers);
                results.extend(page);
                let Some(next) = next else { break };
                assert!(!next.contains("partial_query_cursor"));
                continuation = Some(next);
            }
            assert_eq!(sorted(results), sorted(expected), "{query}");
        }
    }
}

#[tokio::test]
async fn partial_cursor_survives_child_ranges_before_inside_and_after_boundary() {
    for binary in [false, true] {
        for prefix_size in [1, 2, 3] {
            let context = setup((0..4).map(|_| (0, json!({}))).collect()).await;
            let original = "SELECT c.id FROM c GROUP BY c.id";
            let mut request = raw_request(original, "0", prefix_size, None, false);
            request
                .headers_mut()
                .insert(PLAN.clone(), HeaderValue::from_static("True"));
            let response = context
                .emulator
                .execute_request(&request)
                .await
                .unwrap()
                .try_into_raw_response()
                .await
                .unwrap();
            let plan: Value = serde_json::from_slice(response.body()).unwrap();
            let rewrite = plan["queryInfo"]["rewrittenQuery"].as_str().unwrap();
            assert_eq!(plan["queryInfo"]["groupByAliases"], json!(["id"]));
            assert!(plan["queryInfo"]["groupByAliasToAggregateType"]["id"].is_null());
            let (prefix, token) = raw_page(
                &context,
                &raw_request(rewrite, "0", prefix_size, None, binary),
                prefix_size,
                binary,
            )
            .await;
            assert_eq!(prefix.len(), prefix_size as usize);
            let token = token.unwrap();
            assert!(token.contains("partial_query_cursor_v1"));
            let boundary = context
                .emulator
                .store()
                .midpoint_split_epk("testdb", "testcoll", 0)
                .unwrap();
            context
                .emulator
                .store()
                .begin_manual_split_partition("testdb", "testcoll", 0, boundary)
                .unwrap()
                .complete()
                .await
                .unwrap();
            let mut all = prefix;
            for range in topology(&context.emulator)
                .await
                .into_iter()
                .filter(|range| range["id"] != "1")
            {
                let mut continuation = Some(token.clone());
                let mut iterations = 0;
                loop {
                    let (rows, next) = raw_page(
                        &context,
                        &raw_request(
                            rewrite,
                            range["id"].as_str().unwrap(),
                            1,
                            continuation.as_deref(),
                            binary,
                        ),
                        1,
                        binary,
                    )
                    .await;
                    all.extend(rows);
                    iterations += 1;
                    assert!(iterations <= 4, "child cursor must make progress");
                    let Some(next) = next else { break };
                    assert_ne!(continuation.as_deref(), Some(next.as_str()));
                    continuation = Some(next);
                }
            }
            let ids: Vec<_> = all.iter().map(|row| row["payload"]["id"].clone()).collect();
            assert_eq!(
                sorted(ids),
                sorted((0..4).map(|id| json!(format!("item-{id}"))).collect())
            );
            let response = context
                .emulator
                .execute_request(&raw_request(
                    "SELECT VALUE COUNT(1) FROM c",
                    "1",
                    1,
                    Some(&token),
                    binary,
                ))
                .await
                .unwrap();
            assert_eq!(response.status(), azure_core::http::StatusCode::BadRequest);
        }
    }
}

#[tokio::test]
async fn filtered_empty_partial_pages_advance_and_malformed_cursors_fail() {
    let context = setup(vec![(0, json!({"v":1})), (0, json!({"v":2}))]).await;
    let rewrite =
        "SELECT [{\"item\":c.v}] AS groupByItems, c.v AS payload FROM c WHERE false GROUP BY c.v";
    let (rows, token) = raw_page(
        &context,
        &raw_request(rewrite, "0", 1, None, false),
        1,
        false,
    )
    .await;
    assert!(rows.is_empty());
    let token = token.unwrap();
    let (rows, next) = raw_page(
        &context,
        &raw_request(rewrite, "0", 1, Some(&token), false),
        1,
        false,
    )
    .await;
    assert!(rows.is_empty());
    assert!(next.is_none());
    for token in ["1", "{}", "{\"kind\":\"partial_query_cursor_v2\"}"] {
        let response = context
            .emulator
            .execute_request(&raw_request(rewrite, "0", 1, Some(token), false))
            .await
            .unwrap();
        assert_eq!(response.status(), azure_core::http::StatusCode::BadRequest);
    }
}

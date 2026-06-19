// Copyright (c) Microsoft Corporation. All rights reserved.
// Licensed under the MIT License.

// Use the shared test framework declared in `tests/emulator/mod.rs`.
use super::framework;

use std::error::Error;

use azure_data_cosmos::feed::ContinuationToken;
use azure_data_cosmos::{
    clients::DatabaseClient,
    feed::FeedScope,
    models::CosmosStatus,
    options::{MaxItemCountHint, QueryOptions},
    Query,
};
use framework::{test_data, MockItem, TestClient, TestOptions};
use futures::StreamExt;
use serde::de::DeserializeOwned;

fn collect_matching_items(
    items: &[MockItem],
    predicate: impl Fn(&MockItem) -> bool,
) -> Vec<MockItem> {
    items.iter().filter(|p| predicate(p)).cloned().collect()
}

#[derive(Default)]
struct QueryTestOptions {
    max_item_count: Option<u32>,
    use_continuation_token_resume: bool,
}

async fn execute_query_test<T>(
    db_client: &DatabaseClient,
    items: Vec<MockItem>,
    query: impl Into<Query>,
    scope: FeedScope,
    expected_items: Vec<T>,
    options: QueryTestOptions,
) -> Result<(), Box<dyn Error>>
where
    T: DeserializeOwned + Send + Eq + std::fmt::Debug + 'static,
{
    let container_client = test_data::create_container_with_items(db_client, items, None).await?;
    let query: Query = query.into();

    let build_options = || -> QueryOptions {
        let mut o = QueryOptions::default();
        if let Some(max_item_count) = options.max_item_count {
            if let Some(n) = std::num::NonZeroU32::new(max_item_count) {
                o = o.with_max_item_count(MaxItemCountHint::Limit(n));
            }
        }
        o
    };

    let mut actual_items = Vec::new();

    if options.use_continuation_token_resume {
        // Fetch one page at a time, taking a continuation token after each
        // page and resuming a brand-new iterator from the token. This
        // exercises the suspend/resume path end-to-end.
        let mut continuation: Option<ContinuationToken> = None;
        loop {
            let mut query_options = build_options();
            if let Some(token) = continuation.take() {
                query_options = query_options.with_continuation_token(token);
            }
            let mut pages = container_client
                .query_items::<T>(query.clone(), scope.clone(), Some(query_options))
                .await?
                .into_pages();

            let Some(page) = pages.next().await else {
                break;
            };
            let page = page?;
            actual_items.extend(page.into_items());

            // Round-trip the continuation token through string form to
            // mimic real usage (e.g. persisting it across processes).
            let token = pages.to_continuation_token()?;
            let serialized = token.as_str().to_owned();
            let restored = ContinuationToken::from_string(serialized);
            // Drop the iterator before checking for termination — we want to
            // observe the snapshot taken right after the page was emitted.
            drop(pages);

            // The pipeline reports its own terminal state via
            // `to_continuation_token` returning a token whose decoded
            // snapshot is `Drained`. We can't introspect that here, so we
            // detect termination by attempting one more poll on a fresh
            // iterator: if it yields no page, we're done.
            //
            // To avoid an extra round-trip when the snapshot is trivially
            // drained, we still always set `continuation` and let the
            // planner short-circuit to a `DrainedLeaf`.
            continuation = Some(restored);
        }
    } else {
        let mut pages = container_client
            .query_items::<T>(query, scope, Some(build_options()))
            .await?
            .into_pages();
        while let Some(page) = pages.next().await {
            actual_items.extend(page?.into_items());
        }
    }

    assert_eq!(expected_items, actual_items);
    Ok(())
}

#[derive(serde::Deserialize, Debug, PartialEq, Eq)]
#[serde(rename_all = "camelCase")]
struct ItemProjection {
    id: String,
    merge_order: usize,
}

#[tokio::test]
#[cfg_attr(
    not(any(test_category = "emulator", test_category = "emulator_vnext")),
    ignore = "requires test_category 'emulator' or 'emulator_vnext'"
)]
pub async fn single_partition_query_simple() -> Result<(), Box<dyn Error>> {
    TestClient::run_with_unique_db(
        async |_, db_client| {
            let items = test_data::generate_mock_items(10, 10);
            let expected_items =
                collect_matching_items(&items, |p| p.partition_key == "partition0");

            execute_query_test(
                db_client,
                items,
                "select * from docs c",
                FeedScope::partition("partition0"),
                expected_items,
                QueryTestOptions::default(),
            )
            .await?;

            Ok(())
        },
        Some(TestOptions::for_emulator()),
    )
    .await
}

#[tokio::test]
#[cfg_attr(
    not(any(test_category = "emulator", test_category = "emulator_vnext")),
    ignore = "requires test_category 'emulator' or 'emulator_vnext'"
)]
pub async fn single_partition_query_with_parameters() -> Result<(), Box<dyn Error>> {
    TestClient::run_with_unique_db(
        async |_, db_client| {
            let items = test_data::generate_mock_items(10, 10);

            // Find a merge order value in partition1's items
            let merge_order = items
                .iter()
                .find(|p| p.partition_key == "partition1")
                .expect("No items in partition1")
                .merge_order;

            // Query for items with that merge order
            let query = Query::from("select * from c where c.mergeOrder = @some_value")
                .with_parameter("@some_value", merge_order)?;
            let expected_items = collect_matching_items(&items, |p| p.merge_order == merge_order);

            execute_query_test(
                db_client,
                items,
                query,
                FeedScope::partition("partition1"),
                expected_items,
                QueryTestOptions::default(),
            )
            .await?;

            Ok(())
        },
        Some(TestOptions::for_emulator()),
    )
    .await
}

#[tokio::test]
#[cfg_attr(
    not(any(test_category = "emulator", test_category = "emulator_vnext")),
    ignore = "requires test_category 'emulator' or 'emulator_vnext'"
)]
pub async fn single_partition_query_with_projection() -> Result<(), Box<dyn Error>> {
    TestClient::run_with_unique_db(
        async |_, db_client| {
            let items = test_data::generate_mock_items(10, 10);
            let expected_items = items
                .iter()
                .filter(|p| p.partition_key == "partition1")
                .map(|p| ItemProjection {
                    id: p.id.to_string(),
                    merge_order: p.merge_order,
                })
                .collect::<Vec<_>>();

            execute_query_test(
                db_client,
                items,
                "select c.id, c.mergeOrder from c",
                FeedScope::partition("partition1"),
                expected_items,
                QueryTestOptions::default(),
            )
            .await?;

            Ok(())
        },
        Some(TestOptions::for_emulator()),
    )
    .await
}

#[tokio::test]
#[cfg_attr(
    not(any(test_category = "emulator", test_category = "emulator_vnext")),
    ignore = "requires test_category 'emulator' or 'emulator_vnext'"
)]
pub async fn cross_partition_query_with_projection_and_filter() -> Result<(), Box<dyn Error>> {
    TestClient::run_with_unique_db(
        async |_, db_client| {
            let items = test_data::generate_mock_items(10, 2);
            let expected_items = items
                .iter()
                .filter(|p| p.merge_order >= 40 && p.merge_order <= 60)
                .map(|p| p.id.to_string())
                .collect::<Vec<_>>();

            execute_query_test(
                db_client,
                items,
                "select value c.id from c where c.mergeOrder between 40 and 60",
                FeedScope::full_container(),
                expected_items,
                QueryTestOptions::default(),
            )
            .await?;

            Ok(())
        },
        Some(TestOptions::for_emulator()),
    )
    .await
}

#[tokio::test]
#[cfg_attr(
    not(any(test_category = "emulator", test_category = "emulator_vnext")),
    ignore = "requires test_category 'emulator' or 'emulator_vnext'"
)]
// #[cfg_attr(
//     test_category = "emulator_vnext",
//     ignore = "skipped on vnext emulator: behavioral divergence"
// )]
pub async fn cross_partition_query_with_order_by_fails() -> Result<(), Box<dyn Error>> {
    TestClient::run_with_unique_db(
        async |_, db_client| {
            let items = test_data::generate_mock_items(10, 10);
            let container_client =
                test_data::create_container_with_items(db_client, items.clone(), None).await?;

            let Err(err) = container_client
                .query_items::<String>(
                    "select value c.id from c order by c.mergeOrder",
                    FeedScope::full_container(),
                    None,
                )
                .await
            else {
                panic!("Expected query to fail due to cross-partition ORDER BY");
            };
            assert_eq!(
                err.status(),
                CosmosStatus::CROSS_PARTITION_QUERY_NOT_SERVABLE,
                "Expected 400 / 1004 (CrossPartitionQueryNotServable) for cross-partition ORDER BY"
            );

            let body = err
                .response()
                .and_then(|r| match r.body() {
                    azure_data_cosmos_driver::models::ResponseBody::Bytes(b) => Some(b.as_ref()),
                    _ => None,
                })
                .expect("service error should carry a response body");
            #[derive(serde::Deserialize)]
            struct ErrorDetail {
                code: String,
                message: String,
            }
            let error_detail: ErrorDetail =
                serde_json::from_slice(body).expect("response body must be JSON");
            assert_eq!(error_detail.code, "BadRequest");

            // Take only the first two lines of the message for comparison, since the full message may contain additional details that could change over time
            let clean_message = error_detail
                .message
                .lines()
                .take(2)
                .collect::<Vec<_>>()
                .join("\n");
            assert_eq!(
                clean_message,
                "Query contains 1 or more unsupported features. Upgrade your SDK to a version that does support the requested features:\nQuery contained OrderBy, which the calling client does not support."
            );
            Ok(())
        },
        Some(TestOptions::for_emulator()),
    )
    .await
}

#[tokio::test]
#[cfg_attr(
    not(any(test_category = "emulator", test_category = "emulator_vnext")),
    ignore = "requires test_category 'emulator' or 'emulator_vnext'"
)]
// #[cfg_attr(
//     test_category = "emulator_vnext",
//     ignore = "skipped on vnext emulator: behavioral divergence"
// )]
pub async fn query_returns_index_and_query_metrics() -> Result<(), Box<dyn Error>> {
    TestClient::run_with_unique_db(
        async |_, db_client| {
            let items = test_data::generate_mock_items(5, 1);
            let container_client =
                test_data::create_container_with_items(db_client, items.clone(), None).await?;

            // Enable both index metrics and query metrics via typed options.
            let options = QueryOptions::default()
                .with_populate_index_metrics(true)
                .with_populate_query_metrics(true);

            let mut pages = container_client
                .query_items::<MockItem>(
                    "select * from c",
                    FeedScope::partition("partition0"),
                    Some(options),
                )
                .await?
                .into_pages();

            // Get the first page and check metrics headers
            let page = pages
                .next()
                .await
                .expect("expected at least one page")?;

            assert!(!page.items().is_empty(), "expected items in first page");

            // Query metrics should be populated (semicolon-delimited key=value pairs)
            let query_metrics = page.query_metrics();
            assert!(
                query_metrics.is_some(),
                "expected query metrics to be present when x-ms-documentdb-populatequerymetrics is set"
            );
            assert!(
                query_metrics.unwrap().contains("totalExecutionTimeInMs"),
                "expected query metrics to contain totalExecutionTimeInMs"
            );

            // Index metrics should be populated (base64-decoded JSON from service)
            let index_metrics = page.index_metrics();
            assert!(
                index_metrics.is_some(),
                "expected index metrics to be present when x-ms-cosmos-populateindexmetrics is set"
            );

            // Verify common response metadata is also available on QueryFeedPage
            assert!(
                page.headers().request_charge().is_some(),
                "expected request charge on feed page"
            );
            let diagnostics = page.diagnostics();
            assert!(
                !diagnostics.activity_id().as_str().is_empty(),
                "expected activity ID on feed page"
            );
            assert!(
                diagnostics.request_count() >= 1,
                "expected at least one tracked request in feed page diagnostics"
            );
            let server_duration = diagnostics
                .requests()
                .iter()
                .filter_map(|r| r.server_duration_ms())
                .next();
            assert!(
                server_duration.is_some(),
                "expected at least one tracked request to report server_duration_ms"
            );
            assert!(
                f64::from(diagnostics.total_request_charge()) > 0.0,
                "expected positive total request charge in feed page diagnostics"
            );
            assert!(
                page.headers().session_token().is_some(),
                "expected session token on feed page"
            );

            Ok(())
        },
        Some(TestOptions::for_emulator()),
    )
    .await
}

#[tokio::test]
#[cfg_attr(
    not(any(test_category = "emulator", test_category = "emulator_vnext")),
    ignore = "requires test_category 'emulator' or 'emulator_vnext'"
)]
pub async fn single_partition_query_pagination() -> Result<(), Box<dyn Error>> {
    TestClient::run_with_unique_db(
        async |_, db_client| {
            let items = test_data::generate_mock_items(1, 5);
            let expected_items =
                collect_matching_items(&items, |p| p.partition_key == "partition0");
            assert!(
                expected_items.len() > 1,
                "need multiple items to test pagination"
            );

            execute_query_test(
                db_client,
                items,
                "select * from c",
                FeedScope::partition("partition0"),
                expected_items,
                QueryTestOptions {
                    max_item_count: Some(1),
                    use_continuation_token_resume: false,
                },
            )
            .await?;

            Ok(())
        },
        Some(TestOptions::for_emulator()),
    )
    .await
}

#[tokio::test]
#[cfg_attr(
    not(any(test_category = "emulator", test_category = "emulator_vnext")),
    ignore = "requires test_category 'emulator' or 'emulator_vnext'"
)]
pub async fn cross_partition_query_pagination() -> Result<(), Box<dyn Error>> {
    TestClient::run_with_unique_db(
        async |_, db_client| {
            let items = test_data::generate_mock_items(3, 3);
            execute_query_test(
                db_client,
                items.clone(),
                "select * from c",
                FeedScope::full_container(),
                items,
                QueryTestOptions {
                    max_item_count: Some(1),
                    use_continuation_token_resume: false,
                },
            )
            .await?;

            Ok(())
        },
        Some(TestOptions::for_emulator()),
    )
    .await
}

#[tokio::test]
#[cfg_attr(
    not(test_category = "emulator"),
    ignore = "requires test_category 'emulator'"
)]
pub async fn cross_partition_query_suspend_resume() -> Result<(), Box<dyn Error>> {
    TestClient::run_with_unique_db(
        async |_, db_client| {
            // Four logical partitions × three items per partition. With a
            // page size of one, this exercises both intra-partition and
            // cross-partition resume points.
            let items = test_data::generate_mock_items(4, 3);

            execute_query_test(
                db_client,
                items.clone(),
                "select * from c",
                FeedScope::full_container(),
                items,
                QueryTestOptions {
                    max_item_count: Some(1),
                    use_continuation_token_resume: true,
                },
            )
            .await?;

            Ok(())
        },
        Some(TestOptions::for_emulator()),
    )
    .await
}

#[tokio::test]
#[cfg_attr(
    not(test_category = "emulator"),
    ignore = "requires test_category 'emulator'"
)]
pub async fn query_rejects_newer_sdk_continuation_token() -> Result<(), Box<dyn Error>> {
    TestClient::run_with_unique_db(
        async |_, db_client| {
            let items = test_data::generate_mock_items(1, 1);
            let container_client =
                test_data::create_container_with_items(db_client, items, None).await?;

            // A `c2.` prefix indicates the token was issued by a future
            // SDK version this client does not understand.
            let token = ContinuationToken::from_string("c2.something".to_string());
            let options = QueryOptions::default().with_continuation_token(token);

            let Err(err) = container_client
                .query_items::<MockItem>(
                    "select * from c",
                    FeedScope::full_container(),
                    Some(options),
                )
                .await
            else {
                panic!("expected newer-SDK token to be rejected");
            };
            let message = err.to_string();
            assert!(
                message.contains("newer SDK") || message.contains("c2"),
                "unexpected error: {message}"
            );

            Ok(())
        },
        Some(TestOptions::for_emulator()),
    )
    .await
}

#[tokio::test]
#[cfg_attr(
    not(test_category = "emulator"),
    ignore = "requires test_category 'emulator'"
)]
pub async fn query_rejects_server_token_for_cross_partition() -> Result<(), Box<dyn Error>> {
    TestClient::run_with_unique_db(
        async |_, db_client| {
            let items = test_data::generate_mock_items(2, 1);
            let container_client =
                test_data::create_container_with_items(db_client, items, None).await?;

            // An un-prefixed token is treated as an opaque server
            // continuation, which is only valid for trivial (single-
            // partition) queries.
            let token = ContinuationToken::from_string("opaque-server-blob".to_string());
            let options = QueryOptions::default().with_continuation_token(token);

            let Err(err) = container_client
                .query_items::<MockItem>(
                    "select * from c",
                    FeedScope::full_container(),
                    Some(options),
                )
                .await
            else {
                panic!("expected opaque server token to be rejected for cross-partition query");
            };
            let message = err.to_string();
            assert!(
                message.contains("opaque server continuation token"),
                "unexpected error: {message}"
            );

            Ok(())
        },
        Some(TestOptions::for_emulator()),
    )
    .await
}

#[tokio::test]
#[cfg_attr(
    not(test_category = "emulator"),
    ignore = "requires test_category 'emulator'"
)]
pub async fn single_partition_query_resumes_with_raw_server_token() -> Result<(), Box<dyn Error>> {
    use base64::engine::general_purpose::URL_SAFE_NO_PAD;
    use base64::Engine as _;

    TestClient::run_with_unique_db(
        async |_, db_client| {
            // One logical partition × five items so we get multiple pages
            // with `max_item_count(1)`.
            let items = test_data::generate_mock_items(1, 5);
            let expected: Vec<MockItem> =
                collect_matching_items(&items, |p| p.partition_key == "partition0");
            assert!(
                expected.len() > 1,
                "need multiple items to exercise pagination"
            );

            let container_client =
                test_data::create_container_with_items(db_client, items, None).await?;
            let scope = FeedScope::partition("partition0");

            // --- Round 1: fetch the first page through the SDK and pull
            // the SDK-issued `c1.` token. ---
            let mut pages = container_client
                .query_items::<MockItem>(
                    "select * from c",
                    scope.clone(),
                    Some(QueryOptions::default().with_max_item_count(MaxItemCountHint::Limit(
                        std::num::NonZeroU32::new(1).unwrap(),
                    ))),
                )
                .await?
                .into_pages();

            let first_page = pages
                .next()
                .await
                .expect("expected at least one page from the server")?;
            let mut actual: Vec<MockItem> = first_page.into_items();

            let token = pages.to_continuation_token()?;
            let raw = token.as_str().to_owned();
            drop(pages);

            assert!(
                raw.starts_with("c1."),
                "expected SDK to emit a c1.-prefixed token, got: {raw}"
            );

            // Crack the SDK token open. We deliberately couple this test
            // to the on-the-wire format so we can recover the underlying
            // server continuation without exposing extra public APIs.
            //
            // Format: `c1.` + base64url-no-pad(JSON envelope). The envelope
            // is shaped like `{"op":"Query","rid":"<rid>","root":<node>}`,
            // and for a trivial single-partition query the `root` node is
            // `{"kind":"request","server_continuation":"<token>"}`.
            let payload = raw.strip_prefix("c1.").unwrap();
            let json_bytes = URL_SAFE_NO_PAD
                .decode(payload)
                .expect("c1. payload must be valid base64url-no-pad");
            let envelope: serde_json::Value = serde_json::from_slice(&json_bytes)
                .expect("decoded c1. payload must be valid JSON");
            let snapshot = envelope
                .get("root")
                .expect("c1. envelope must contain a `root` node");
            assert_eq!(
                snapshot.get("kind").and_then(|v| v.as_str()),
                Some("request"),
                "trivial single-partition pipeline should snapshot as a single Request node, got: {envelope}"
            );
            let server_token = snapshot
                .get("server_continuation")
                .and_then(|v| v.as_str())
                .expect("Request node must carry a server_continuation after the first page")
                .to_owned();
            assert!(
                !server_token.is_empty(),
                "server continuation token should not be empty"
            );
            assert!(
                !server_token.starts_with("c1.") && !server_token.starts_with("c2."),
                "server continuation must not look like an SDK token, got: {server_token}"
            );

            // --- Round 2: drain the rest of the query using the raw
            // server token directly (no `c1.` prefix). The SDK accepts
            // un-prefixed tokens as an opaque server fallback for trivial
            // single-partition queries. ---
            let mut continuation = Some(ContinuationToken::from_string(server_token));
            let mut page_count: usize = 1;
            loop {
                let mut options = QueryOptions::default().with_max_item_count(
                    MaxItemCountHint::Limit(std::num::NonZeroU32::new(1).unwrap()),
                );
                if let Some(t) = continuation.take() {
                    options = options.with_continuation_token(t);
                }

                let mut pages = container_client
                    .query_items::<MockItem>("select * from c", scope.clone(), Some(options))
                    .await?
                    .into_pages();

                let Some(page) = pages.next().await else {
                    break;
                };
                let page = page?;
                let items_in_page = page.into_items();
                let was_empty = items_in_page.is_empty();
                actual.extend(items_in_page);
                page_count += 1;

                let next_token = pages.to_continuation_token()?;
                let raw_next = next_token.as_str().to_owned();
                drop(pages);

                // Subsequent SDK-issued tokens must still be `c1.`-prefixed.
                assert!(
                    raw_next.starts_with("c1."),
                    "follow-up token must remain c1.-prefixed, got: {raw_next}"
                );

                // Decode again to detect end-of-stream: when the inner
                // snapshot is `{"kind":"drained"}` we are done.
                let payload = raw_next.strip_prefix("c1.").unwrap();
                let json_bytes = URL_SAFE_NO_PAD
                    .decode(payload)
                    .expect("c1. payload must be valid base64url-no-pad");
                let envelope: serde_json::Value =
                    serde_json::from_slice(&json_bytes).expect("payload must be valid JSON");
                let kind = envelope
                    .get("root")
                    .and_then(|root| root.get("kind"))
                    .and_then(|v| v.as_str())
                    .unwrap_or("");
                if kind == "drained" || was_empty {
                    break;
                }

                // Continue feeding the SDK its own next-token.
                continuation = Some(ContinuationToken::from_string(raw_next));

                assert!(
                    page_count <= expected.len() + 2,
                    "fetched more pages ({page_count}) than expected ({})",
                    expected.len()
                );
            }

            assert_eq!(expected, actual);
            Ok(())
        },
        Some(TestOptions::for_emulator()),
    )
    .await
}

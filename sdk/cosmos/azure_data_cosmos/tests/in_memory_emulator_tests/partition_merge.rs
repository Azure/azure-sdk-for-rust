// Copyright (c) Microsoft Corporation. All rights reserved.
// Licensed under the MIT License.

//! Public-SDK coverage for real partition merges in the hosted in-memory emulator.

use std::{
    collections::{BTreeSet, HashMap},
    error::Error,
    num::NonZeroU32,
    time::Duration,
};

use azure_core::Uuid;
use azure_data_cosmos::{
    clients::ContainerClient,
    feed::{ContinuationToken, FeedScope},
    models::{ChangeFeedItem, ContainerProperties},
    options::{
        ChangeFeedOptions, ChangeFeedStartFrom, ConnectionPoolOptions, MaxItemCountHint,
        QueryOptions, ReadFeedRangesOptions, Region, ServerCertificateValidation,
    },
    AccountEndpoint, AccountReference, CosmosClient, CosmosRuntime, RoutingStrategy,
};
use azure_data_cosmos_driver::models::ConnectionString;
use futures::StreamExt;
use serde::{de::DeserializeOwned, Deserialize, Serialize};

const CONTAINER_NAME: &str = "HostedPartitionMerge";
const EXPECTED_INITIAL_RANGES: usize = 4;
const QUERY_PAGE_SIZE: u32 = 7;
const MANAGEMENT_POLL_LIMIT: usize = 400;
const MANAGEMENT_POLL_INTERVAL: Duration = Duration::from_millis(25);
const NETWORK_TIMEOUT: Duration = Duration::from_secs(10);
const TEST_TIMEOUT: Duration = Duration::from_secs(90);

#[derive(Clone, Debug, Deserialize, Eq, PartialEq, Serialize)]
#[serde(rename_all = "camelCase")]
struct MergeItem {
    id: String,
    pk: String,
    merge_order: usize,
}

#[derive(Debug, Deserialize)]
#[serde(rename_all = "camelCase")]
struct HostHealth {
    gateway20_enabled: bool,
    connectivity_probes: usize,
    gateway20_requests: usize,
}

async fn public_client_from_env() -> Result<CosmosClient, Box<dyn Error>> {
    let connection_string: ConnectionString =
        std::env::var("AZURE_COSMOS_CONNECTION_STRING")?.parse()?;
    let endpoint: AccountEndpoint = connection_string.account_endpoint().parse()?;
    let runtime = CosmosRuntime::builder()
        .with_connection_pool(
            ConnectionPoolOptions::builder()
                .with_server_certificate_validation(
                    ServerCertificateValidation::RequiredUnlessEmulator,
                )
                .build()?,
        )
        .build()
        .await?;

    Ok(CosmosClient::builder()
        .with_runtime(runtime)
        .build(
            AccountReference::with_authentication_key(
                endpoint,
                connection_string.account_key().clone(),
            ),
            RoutingStrategy::ProximityTo(Region::EAST_US_2),
        )
        .await?)
}

async fn host_health(
    client: &reqwest::Client,
    management_endpoint: &str,
) -> Result<HostHealth, Box<dyn Error>> {
    let body = client
        .get(format!(
            "{}/health",
            management_endpoint.trim_end_matches('/')
        ))
        .send()
        .await?
        .error_for_status()?
        .bytes()
        .await?;
    Ok(serde_json::from_slice(&body)?)
}

async fn merge_first_two_partitions(
    client: &reqwest::Client,
    management_endpoint: &str,
    database: &str,
    container: &str,
) -> Result<(), Box<dyn Error>> {
    let management_endpoint = management_endpoint.trim_end_matches('/');
    let response = client
        .post(format!(
            "{management_endpoint}/databases/{database}/containers/{container}/partitions/merge"
        ))
        .header("content-type", "application/json")
        .body(serde_json::to_vec(
            &serde_json::json!({ "partitionIds": [0, 1] }),
        )?)
        .send()
        .await?
        .error_for_status()?
        .bytes()
        .await?;
    let response: serde_json::Value = serde_json::from_slice(&response)?;
    let operation_id = response["operationId"]
        .as_str()
        .ok_or("merge response did not include operationId")?;

    let operation_url = format!("{management_endpoint}/operations/{operation_id}");
    for _ in 0..MANAGEMENT_POLL_LIMIT {
        let body = client
            .get(&operation_url)
            .send()
            .await?
            .error_for_status()?
            .bytes()
            .await?;
        let operation: serde_json::Value = serde_json::from_slice(&body)?;
        match operation["phase"].as_str() {
            Some("Succeeded") => {
                assert_eq!(operation["merged"], serde_json::json!([0, 1]));
                assert_eq!(
                    operation["into"].as_u64(),
                    Some(EXPECTED_INITIAL_RANGES as u64),
                    "the merge must replace its parents with the next physical partition id"
                );
                return Ok(());
            }
            Some("Failed") => {
                return Err(format!("partition merge failed: {operation}").into());
            }
            _ => tokio::time::sleep(MANAGEMENT_POLL_INTERVAL).await,
        }
    }

    Err("partition merge did not reach a terminal phase".into())
}

fn query_options(continuation: Option<ContinuationToken>) -> QueryOptions {
    let mut options = QueryOptions::default().with_max_item_count(MaxItemCountHint::Limit(
        NonZeroU32::new(QUERY_PAGE_SIZE).unwrap(),
    ));
    if let Some(continuation) = continuation {
        options = options.with_continuation_token(continuation);
    }
    options
}

async fn capture_query_page<T>(
    container: &ContainerClient,
    query: &str,
) -> Result<(Vec<T>, ContinuationToken), Box<dyn Error>>
where
    T: DeserializeOwned + Send + 'static,
{
    let mut pages = container
        .query_items::<T>(
            query,
            FeedScope::full_container(),
            Some(query_options(None)),
        )
        .await?
        .into_pages();
    let page = pages
        .next()
        .await
        .ok_or("query did not return a first page")??;
    let token = pages.to_continuation_token()?.as_str().to_owned();
    Ok((page.into_items(), ContinuationToken::from_string(token)))
}

async fn resume_query<T>(
    container: &ContainerClient,
    query: &str,
    mut items: Vec<T>,
    mut continuation: Option<ContinuationToken>,
) -> Result<Vec<T>, Box<dyn Error>>
where
    T: DeserializeOwned + Send + 'static,
{
    loop {
        let mut pages = container
            .query_items::<T>(
                query,
                FeedScope::full_container(),
                Some(query_options(continuation.take())),
            )
            .await?
            .into_pages();
        let Some(page) = pages.next().await else {
            return Ok(items);
        };
        items.extend(page?.into_items());
        let token = pages.to_continuation_token()?.as_str().to_owned();
        continuation = Some(ContinuationToken::from_string(token));
    }
}

async fn fresh_point_in_time_snapshot(
    container: &ContainerClient,
    start: time::OffsetDateTime,
    expected: &BTreeSet<String>,
) -> Result<(), Box<dyn Error>> {
    const POLL_LIMIT: usize = 32;
    let options = ChangeFeedOptions::default()
        .with_max_item_count(MaxItemCountHint::Limit(NonZeroU32::new(1000).unwrap()));
    let mut pages = container
        .query_change_feed::<MergeItem>(
            FeedScope::full_container(),
            ChangeFeedStartFrom::PointInTime(start),
            Some(options),
        )
        .await?;
    let mut seen = BTreeSet::new();

    for _ in 0..POLL_LIMIT {
        let page = pages
            .next()
            .await
            .ok_or("change feed ended before returning the current snapshot")??;
        for item in page.into_items() {
            let item = current_document(item)?;
            assert!(
                seen.insert(item.id.clone()),
                "PointInTime replayed item {} before completing the snapshot",
                item.id
            );
        }
        if seen.len() == expected.len() {
            assert_eq!(&seen, expected);
            return Ok(());
        }
        assert!(
            seen.len() < expected.len(),
            "PointInTime returned more items than expected"
        );
    }

    Err(format!(
        "PointInTime did not return the complete snapshot after {POLL_LIMIT} bounded polls; \
         expected {}, saw {}",
        expected.len(),
        seen.len()
    )
    .into())
}

fn current_document(item: ChangeFeedItem<MergeItem>) -> Result<MergeItem, Box<dyn Error>> {
    item.current()
        .cloned()
        .ok_or_else(|| "LatestVersion change feed item did not contain current".into())
}

fn assert_exact_items(actual: &[MergeItem], expected: &BTreeSet<String>, label: &str) {
    let actual_ids: BTreeSet<_> = actual.iter().map(|item| item.id.clone()).collect();
    assert_eq!(
        actual.len(),
        actual_ids.len(),
        "{label} replayed at least one item"
    );
    assert_eq!(&actual_ids, expected, "{label} lost or added items");
}

fn assert_host_flavor(before: &HostHealth, after: &HostHealth) {
    match std::env::var("AZURE_COSMOS_EMULATOR_FLAVOR").as_deref() {
        Ok("inmemory-v1") => assert!(!after.gateway20_enabled),
        Ok("inmemory-v2") => {
            assert!(after.gateway20_enabled);
            assert!(after.connectivity_probes > 0);
            assert!(
                after.gateway20_requests > before.gateway20_requests,
                "V2 host metrics must prove that public SDK traffic used Gateway 2.0"
            );
        }
        flavor => panic!("unexpected hosted emulator flavor: {flavor:?}"),
    }
}

#[tokio::test]
#[cfg_attr(
    not(test_category = "emulator_inmemory"),
    ignore = "requires test_category 'emulator_inmemory'"
)]
async fn public_sdk_paths_survive_real_partition_merge() -> Result<(), Box<dyn Error>> {
    const ITEM_COUNT: usize = 120;
    const OFFSET: usize = 9;
    const LIMIT: usize = 53;
    const NATURAL_QUERY: &str = "SELECT * FROM c";
    const ORDERED_QUERY: &str = "SELECT * FROM c ORDER BY c.mergeOrder";
    const OFFSET_LIMIT_QUERY: &str = "SELECT * FROM c ORDER BY c.mergeOrder OFFSET 9 LIMIT 53";

    let management_endpoint = std::env::var("AZURE_COSMOS_INMEMORY_MANAGEMENT_ENDPOINT")?;
    let management_client = reqwest::Client::builder()
        .timeout(NETWORK_TIMEOUT)
        .build()?;
    let health_before = host_health(&management_client, &management_endpoint).await?;
    let point_in_time = time::OffsetDateTime::now_utc() - time::Duration::seconds(1);
    let cosmos_client = tokio::time::timeout(NETWORK_TIMEOUT, public_client_from_env())
        .await
        .map_err(|_| "public Cosmos client setup timed out")??;
    let database_name = format!("merge-{}", Uuid::new_v4());
    tokio::time::timeout(
        NETWORK_TIMEOUT,
        cosmos_client.create_database(&database_name, None),
    )
    .await
    .map_err(|_| "database creation timed out")??;
    let database = cosmos_client.database_client(&database_name);

    let scenario = tokio::time::timeout(TEST_TIMEOUT, async {
        let created = database
            .create_container(ContainerProperties::new(CONTAINER_NAME, "/pk".into()), None)
            .await?
            .into_model()?;
        let container = database.container_client(created.id.as_ref(), None).await?;
        let database_name = database
            .name()
            .ok_or("partition merge test requires a name-addressed database")?;
        let initial_ranges = container.read_feed_ranges(None).await?.len();
        assert_eq!(
            initial_ranges, EXPECTED_INITIAL_RANGES,
            "hosted merge scenarios require the default four-partition topology"
        );

        let mut expected_ids = BTreeSet::new();
        let mut probes = HashMap::<u32, MergeItem>::new();
        for rank in 0..ITEM_COUNT {
            let item = MergeItem {
                id: format!("item-{rank:03}"),
                pk: format!("pk-{rank:03}"),
                merge_order: rank,
            };
            expected_ids.insert(item.id.clone());
            let response = container
                .create_item(item.pk.clone(), &item.id, item.clone(), None)
                .await?;
            if let Some(range_id) = response
                .headers()
                .partition_key_range_id()
                .and_then(|id| id.parse::<u32>().ok())
            {
                if range_id <= 1 {
                    probes.entry(range_id).or_insert(item);
                }
            }
        }
        assert_eq!(
            probes.len(),
            2,
            "seed data must exercise both physical partitions being merged"
        );

        for range_id in [0, 1] {
            let probe = &probes[&range_id];
            let read: MergeItem = container
                .read_item(probe.pk.clone(), &probe.id, None)
                .await?
                .into_model()?;
            assert_eq!(&read, probe);
        }

        let (natural_first, natural_token) =
            capture_query_page::<MergeItem>(&container, NATURAL_QUERY).await?;
        let (ordered_first, ordered_token) =
            capture_query_page::<MergeItem>(&container, ORDERED_QUERY).await?;
        let (offset_first, offset_token) =
            capture_query_page::<MergeItem>(&container, OFFSET_LIMIT_QUERY).await?;
        let natural_first_len = natural_first.len();
        let ordered_first_len = ordered_first.len();
        let offset_first_len = offset_first.len();
        assert!(
            (1..ITEM_COUNT).contains(&natural_first_len),
            "natural query must capture a partial pre-merge page"
        );
        assert!(
            (1..ITEM_COUNT).contains(&ordered_first_len),
            "ORDER BY query must capture a partial pre-merge page"
        );
        assert!(
            (1..LIMIT).contains(&offset_first_len),
            "OFFSET/LIMIT query must capture a partial pre-merge page"
        );

        merge_first_two_partitions(
            &management_client,
            &management_endpoint,
            database_name,
            CONTAINER_NAME,
        )
        .await?;

        let removed_range_probe = &probes[&1];
        container
            .replace_item(
                removed_range_probe.pk.clone(),
                &removed_range_probe.id,
                removed_range_probe,
                None,
            )
            .await?;

        for range_id in [1, 0] {
            let probe = &probes[&range_id];
            let read: MergeItem = container
                .read_item(probe.pk.clone(), &probe.id, None)
                .await?
                .into_model()?;
            assert_eq!(
                &read, probe,
                "warm point read must recover from the stale range {range_id} route"
            );
        }

        let natural = resume_query(
            &container,
            NATURAL_QUERY,
            natural_first,
            Some(natural_token),
        )
        .await?;
        assert!(
            natural.len() > natural_first_len,
            "natural query resume must return post-merge pages"
        );
        assert_exact_items(&natural, &expected_ids, "natural query continuation");

        let ordered = resume_query(
            &container,
            ORDERED_QUERY,
            ordered_first,
            Some(ordered_token),
        )
        .await?;
        assert!(
            ordered.len() > ordered_first_len,
            "ORDER BY resume must return post-merge pages"
        );
        assert_exact_items(&ordered, &expected_ids, "ORDER BY continuation");
        assert_eq!(
            ordered
                .iter()
                .map(|item| item.merge_order)
                .collect::<Vec<_>>(),
            (0..ITEM_COUNT).collect::<Vec<_>>(),
            "ORDER BY must preserve exact global order across the merge"
        );

        let offset = resume_query(
            &container,
            OFFSET_LIMIT_QUERY,
            offset_first,
            Some(offset_token),
        )
        .await?;
        assert!(
            offset.len() > offset_first_len,
            "OFFSET/LIMIT resume must return post-merge pages"
        );
        assert_eq!(
            offset
                .iter()
                .map(|item| item.merge_order)
                .collect::<Vec<_>>(),
            (OFFSET..OFFSET + LIMIT).collect::<Vec<_>>(),
            "OFFSET/LIMIT must preserve its exact ordered window without replay or loss"
        );

        for suffix in ["a", "b"] {
            let item = MergeItem {
                id: format!("post-merge-{suffix}"),
                pk: removed_range_probe.pk.clone(),
                merge_order: ITEM_COUNT + expected_ids.len(),
            };
            container
                .create_item(item.pk.clone(), &item.id, item.clone(), None)
                .await?;
            let read: MergeItem = container
                .read_item(item.pk.clone(), &item.id, None)
                .await?
                .into_model()?;
            assert_eq!(read, item);
            expected_ids.insert(item.id);
        }

        let final_ranges = container
            .read_feed_ranges(Some(
                ReadFeedRangesOptions::default().with_force_refresh(true),
            ))
            .await?
            .len();
        assert_eq!(
            final_ranges,
            EXPECTED_INITIAL_RANGES - 1,
            "the management merge must remove exactly one physical partition"
        );

        fresh_point_in_time_snapshot(&container, point_in_time, &expected_ids).await?;
        Ok::<(), Box<dyn Error>>(())
    })
    .await;

    let cleanup = tokio::time::timeout(NETWORK_TIMEOUT, database.delete(None)).await;
    match scenario {
        Ok(result) => {
            result?;
            cleanup.map_err(|_| "database cleanup timed out")??;
        }
        Err(_) => {
            let _ = cleanup;
            return Err(
                format!("partition merge scenario timed out after {TEST_TIMEOUT:?}").into(),
            );
        }
    }

    let health_after = host_health(&management_client, &management_endpoint).await?;
    assert_host_flavor(&health_before, &health_after);
    Ok(())
}

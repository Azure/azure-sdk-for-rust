// Copyright (c) Microsoft Corporation. All rights reserved.
// Licensed under the MIT License.

//! Opt-in live validation for the Azure Cosmos DB partition merge preview.

use super::{
    cosmos_change_feed_split::drain_changes,
    cosmos_query_split::force_split_and_wait,
    framework::{MockItem, TestClient, TestOptions},
};
use azure_data_cosmos::{
    feed::FeedScope,
    models::{ContainerProperties, ThroughputProperties},
    options::{
        ChangeFeedStartFrom, CreateContainerOptions, MaxItemCountHint, QueryOptions,
        ReadFeedRangesOptions,
    },
};
use futures::{StreamExt, TryStreamExt};
use std::{collections::BTreeSet, error::Error, num::NonZeroU32, time::Duration};

const CONTAINER_NAME: &str = "PartitionMergeLive";
const MANAGEMENT_SCOPE: &str = "https://management.azure.com/.default";
const MERGE_API_VERSION: &str = "2026-04-01-preview";
const MERGE_TIMEOUT: Duration = Duration::from_secs(60 * 60);
const MERGE_POLL_INTERVAL: Duration = Duration::from_secs(30);

async fn invoke_partition_merge(
    database_name: &str,
    container_name: &str,
) -> Result<(), Box<dyn Error>> {
    let subscription_id = std::env::var("COSMOS_SUBSCRIPTION_ID")?;
    let resource_group = std::env::var("COSMOS_RESOURCE_GROUP")?;
    let account_name = std::env::var("COSMOS_ACCOUNT_NAME")?;
    let credential = azure_core_test::credentials::from_env(None)?;
    let token = credential.get_token(&[MANAGEMENT_SCOPE], None).await?;
    let client = reqwest::Client::new();
    let merge_started = time::OffsetDateTime::now_utc();
    let resource_id = format!(
        "/subscriptions/{subscription_id}/resourceGroups/{resource_group}/providers/Microsoft.DocumentDB/databaseAccounts/{account_name}/sqlDatabases/{database_name}/containers/{container_name}"
    );
    let merge_url = format!(
        "https://management.azure.com{resource_id}/partitionMerge?api-version={MERGE_API_VERSION}"
    );

    let response = client
        .post(merge_url)
        .bearer_auth(token.token.secret())
        .header("content-type", "application/json")
        .body(r#"{"isDryRun":false}"#)
        .send()
        .await?;
    let status = response.status();
    if !status.is_success() {
        return Err(format!(
            "partition merge request failed with {status}: {}",
            response.text().await?
        )
        .into());
    }

    let deadline = tokio::time::Instant::now() + MERGE_TIMEOUT;
    let mut poll_count = 0usize;
    if status == reqwest::StatusCode::ACCEPTED {
        let operation_url = response
            .headers()
            .get("azure-asyncoperation")
            .or_else(|| response.headers().get("location"))
            .ok_or("partition merge response did not include an operation URL")?
            .to_str()?
            .to_owned();
        loop {
            if tokio::time::Instant::now() >= deadline {
                return Err(
                    "partition merge ARM operation did not complete within 60 minutes".into(),
                );
            }

            let token = credential.get_token(&[MANAGEMENT_SCOPE], None).await?;
            let response = client
                .get(&operation_url)
                .bearer_auth(token.token.secret())
                .send()
                .await?
                .error_for_status()?;
            let body: serde_json::Value = serde_json::from_slice(&response.bytes().await?)?;
            if body.get("physicalPartitionStorageInfoCollection").is_some() {
                break;
            }
            let status = body
                .get("status")
                .and_then(serde_json::Value::as_str)
                .or_else(|| {
                    body.pointer("/status/code")
                        .and_then(serde_json::Value::as_str)
                })
                .or_else(|| {
                    body.pointer("/properties/status")
                        .and_then(serde_json::Value::as_str)
                })
                .or_else(|| {
                    body.pointer("/properties/provisioningState")
                        .and_then(serde_json::Value::as_str)
                })
                .map(str::to_ascii_lowercase)
                .unwrap_or_default();
            match status.as_str() {
                "succeeded" | "completed" => break,
                "failed" | "canceled" | "cancelled" => {
                    return Err(format!("partition merge operation failed: {body}").into());
                }
                _ => {
                    if poll_count.is_multiple_of(10) {
                        println!("Partition merge ARM operation is still running: {body}");
                    }
                    poll_count += 1;
                    tokio::time::sleep(MERGE_POLL_INTERVAL).await;
                }
            }
        }
    }

    let merge_started = merge_started.format(&time::format_description::well_known::Rfc3339)?;
    let activity_url = reqwest::Url::parse(&format!(
        "https://management.azure.com/subscriptions/{subscription_id}/providers/microsoft.insights/eventtypes/management/values"
    ))?;

    poll_count = 0;
    loop {
        if tokio::time::Instant::now() >= deadline {
            return Err("partition merge backend did not complete within 60 minutes".into());
        }
        let merge_ended = time::OffsetDateTime::now_utc()
            .format(&time::format_description::well_known::Rfc3339)?;
        let activity_filter = format!(
            "eventTimestamp ge '{merge_started}' and eventTimestamp le '{merge_ended}' and resourceGroupName eq '{resource_group}'"
        );
        let mut request_url = activity_url.clone();
        request_url
            .query_pairs_mut()
            .append_pair("api-version", "2015-04-01")
            .append_pair("$filter", &activity_filter);
        let token = credential.get_token(&[MANAGEMENT_SCOPE], None).await?;
        let response = client
            .get(request_url)
            .bearer_auth(token.token.secret())
            .send()
            .await?
            .error_for_status()?;
        let body: serde_json::Value = serde_json::from_slice(&response.bytes().await?)?;
        let completed = body["value"].as_array().is_some_and(|events| {
            events.iter().any(|event| {
                event["resourceId"]
                    .as_str()
                    .is_some_and(|id| id.eq_ignore_ascii_case(&resource_id))
                    && ["value", "localizedValue"].iter().any(|field| {
                        event["operationName"][field]
                            == "PartitionCoalescer Merge operation for Container"
                    })
                    && event["status"]["value"] == "Succeeded"
            })
        });
        if completed {
            return Ok(());
        }
        if poll_count.is_multiple_of(4) {
            println!("Waiting for the partition merge backend to complete...");
        }
        poll_count += 1;
        tokio::time::sleep(MERGE_POLL_INTERVAL).await;
    }
}

#[tokio::test]
#[cfg_attr(
    not(test_category = "merge"),
    ignore = "requires test_category 'merge'"
)]
async fn routing_query_and_point_in_time_feed_survive_merge() -> Result<(), Box<dyn Error>> {
    const PK_COUNT: usize = 40;
    const ITEMS_PER_PK: usize = 4;
    const PAGE_SIZE: u32 = 11;

    TestClient::run_with_unique_db(
        async |run_context, db_client| {
            let properties = ContainerProperties::new(CONTAINER_NAME, "/partitionKey".into());
            let container = run_context
                .create_container(
                    db_client,
                    properties,
                    Some(
                        CreateContainerOptions::default()
                            .with_throughput(ThroughputProperties::manual(1000)),
                    ),
                )
                .await?;

            let mut expected = BTreeSet::new();
            for partition in 0..PK_COUNT {
                let partition_key = format!("pk-{partition}");
                for item_index in 0..ITEMS_PER_PK {
                    let item = MockItem {
                        id: format!("{partition}-{item_index}"),
                        partition_key: partition_key.clone(),
                        merge_order: partition * ITEMS_PER_PK + item_index,
                    };
                    expected.insert(item.id.clone());
                    container
                        .create_item(partition_key.clone(), &item.id.clone(), item, None)
                        .await?;
                }
            }

            let partitions_before = container.read_feed_ranges(None).await?.len();
            let partitions_after_split =
                force_split_and_wait(&container, partitions_before).await?;
            assert!(partitions_after_split > partitions_before);

            let mut pages = container
                .query_items::<MockItem>(
                    "SELECT * FROM c",
                    FeedScope::full_container(),
                    Some(
                        QueryOptions::default().with_max_item_count(MaxItemCountHint::Limit(
                            NonZeroU32::new(PAGE_SIZE).unwrap(),
                        )),
                    ),
                )
                .await?
                .into_pages();
            let first_page = pages.next().await.ok_or("query returned no first page")??;
            let mut query_ids: Vec<String> = first_page
                .into_items()
                .into_iter()
                .map(|item| item.id)
                .collect();
            let query_token = pages.to_continuation_token()?;
            drop(pages);

            let mut throughput = container
                .begin_replace_throughput(ThroughputProperties::manual(4000), None)
                .await?;
            while let Some(status) = throughput.try_next().await? {
                assert!(status.status().is_success());
            }

            let point_in_time = time::OffsetDateTime::now_utc();
            invoke_partition_merge(
                db_client
                    .name()
                    .ok_or("partition merge test requires a name-addressed database")?,
                CONTAINER_NAME,
            )
            .await?;

            let merge_deadline = tokio::time::Instant::now() + Duration::from_secs(10 * 60);
            loop {
                let count = container
                    .read_feed_ranges(Some(
                        ReadFeedRangesOptions::default().with_force_refresh(true),
                    ))
                    .await?
                    .len();
                if count < partitions_after_split {
                    break;
                }
                if tokio::time::Instant::now() >= merge_deadline {
                    return Err("merge completed but partition topology did not shrink".into());
                }
                tokio::time::sleep(Duration::from_secs(15)).await;
            }

            let mut resume_options = QueryOptions::default()
                .with_max_item_count(MaxItemCountHint::Limit(NonZeroU32::new(PAGE_SIZE).unwrap()));
            resume_options = resume_options.with_continuation_token(query_token);
            let mut resumed = container
                .query_items::<MockItem>(
                    "SELECT * FROM c",
                    FeedScope::full_container(),
                    Some(resume_options),
                )
                .await?
                .into_pages();
            while let Some(page) = resumed.next().await {
                query_ids.extend(page?.into_items().into_iter().map(|item| item.id));
            }
            assert_eq!(
                query_ids.len(),
                expected.len(),
                "query continuation replayed or lost items across the merge"
            );
            assert_eq!(
                query_ids.into_iter().collect::<BTreeSet<_>>(),
                expected,
                "query continuation returned the wrong item set across the merge"
            );

            let probe_id = "post-merge-probe";
            let probe_pk = "post-merge-pk";
            container
                .create_item(
                    probe_pk,
                    probe_id,
                    MockItem {
                        id: probe_id.to_owned(),
                        partition_key: probe_pk.to_owned(),
                        merge_order: usize::MAX,
                    },
                    None,
                )
                .await?;
            let probe: MockItem = container
                .read_item(probe_pk, probe_id, None)
                .await?
                .into_model()?;
            assert_eq!(probe.id, probe_id);

            let post_merge_id = "post-merge-change";
            container
                .create_item(
                    probe_pk,
                    post_merge_id,
                    MockItem {
                        id: post_merge_id.to_owned(),
                        partition_key: probe_pk.to_owned(),
                        merge_order: usize::MAX - 1,
                    },
                    None,
                )
                .await?;
            let mut feed = container
                .query_change_feed::<MockItem>(
                    FeedScope::full_container(),
                    ChangeFeedStartFrom::PointInTime(point_in_time),
                    None,
                )
                .await?;
            let changes = drain_changes(&mut feed).await?;
            let change_ids: BTreeSet<_> = changes.into_iter().map(|item| item.id).collect();
            assert!(change_ids.contains(probe_id));
            assert!(change_ids.contains(post_merge_id));
            assert!(change_ids.is_disjoint(&expected));

            Ok(())
        },
        Some(TestOptions::new().with_timeout(Duration::from_secs(85 * 60))),
    )
    .await
}

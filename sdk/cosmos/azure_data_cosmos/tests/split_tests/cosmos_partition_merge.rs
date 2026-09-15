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
    options::{ChangeFeedStartFrom, MaxItemCountHint, QueryOptions, ReadFeedRangesOptions},
};
use futures::StreamExt;
use std::{collections::BTreeSet, error::Error, num::NonZeroU32, time::Duration};

const CONTAINER_NAME: &str = "PartitionMergeLive";

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
                    Some(ThroughputProperties::manual(1000)),
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
            let partitions_after_split = force_split_and_wait(
                run_context,
                db_client,
                &container,
                CONTAINER_NAME,
                partitions_before,
            )
            .await?;
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

            run_context
                .replace_container_throughput(
                    db_client,
                    CONTAINER_NAME,
                    ThroughputProperties::manual(4000),
                )
                .await?;

            let point_in_time = time::OffsetDateTime::now_utc();
            run_context
                .arm_client()
                .ok_or("partition merge requires AAD-backed ARM resource management")?
                .merge_partitions(
                    db_client
                        .name()
                        .ok_or("partition merge test requires a name-addressed database")?,
                    CONTAINER_NAME,
                )
                .await?;

            let merge_deadline = tokio::time::Instant::now() + Duration::from_secs(60 * 60);
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
                        merge_order: 100_000,
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
                        merge_order: 100_001,
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
        Some(TestOptions::new().with_timeout(Duration::from_secs(150 * 60))),
    )
    .await
}

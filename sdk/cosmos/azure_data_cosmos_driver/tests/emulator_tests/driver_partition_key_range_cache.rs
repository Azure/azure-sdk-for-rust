// Copyright (c) Microsoft Corporation. All rights reserved.
// Licensed under the MIT License.

#![cfg(feature = "__internal_testing")]

//! Live-service coverage for the partition key range cache.

use std::{collections::BTreeSet, num::NonZeroU32};

use azure_data_cosmos_driver::{
    driver::CosmosDriver,
    models::{
        ChangeFeedStartFrom, ContainerReference, CosmosOperation, CosmosResponse, FeedRange,
        MaxItemCountHint, PartitionKey, ResponseBody,
    },
    options::{OperationOptions, PlanOptions},
};

use crate::framework::DriverTestClient;

#[tokio::test]
#[cfg_attr(
    not(any(test_category = "emulator", test_category = "emulator_vnext")),
    ignore = "requires test_category 'emulator' or 'emulator_vnext'"
)]
async fn live_routing_map_cold_warm_and_refresh_paths() -> Result<(), Box<dyn std::error::Error>> {
    DriverTestClient::run_with_unique_db(async |context, database| {
        let container_name = context.unique_container_name();
        let container = context
            .create_container(&database, &container_name, "/pk")
            .await?;
        let driver = context.create_persistent_driver().await?;

        let cold = driver
            .resolve_all_partition_key_ranges(&container, false)
            .await?
            .ok_or("cold PK-range resolution returned no map")?;
        assert!(!cold.is_empty(), "a live container must have PK ranges");
        let expected_ids: BTreeSet<_> = cold.iter().map(|range| range.id.clone()).collect();

        let warm = driver
            .resolve_all_partition_key_ranges(&container, false)
            .await?
            .ok_or("warm PK-range resolution returned no map")?;
        let warm_ids: BTreeSet<_> = warm.iter().map(|range| range.id.clone()).collect();
        assert_eq!(warm_ids, expected_ids);

        let refreshed = driver
            .resolve_all_partition_key_ranges(&container, true)
            .await?
            .ok_or("refreshed PK-range resolution returned no map")?;
        let refreshed_ids: BTreeSet<_> = refreshed.iter().map(|range| range.id.clone()).collect();
        assert_eq!(refreshed_ids, expected_ids);

        let owning = driver
            .resolve_partition_key_ranges_for_key(
                &container,
                &PartitionKey::from("live-routing-key"),
                false,
            )
            .await?
            .ok_or("point-key resolution returned no owning range")?;
        assert_eq!(owning.len(), 1);
        assert!(expected_ids.contains(&owning[0].id));

        Ok(())
    })
    .await
}

fn page_documents(response: CosmosResponse) -> Vec<serde_json::Value> {
    fn parse(bytes: &[u8]) -> serde_json::Value {
        if azure_data_cosmos_driver::binary_json::is_binary(bytes) {
            azure_data_cosmos_driver::binary_json::decode(bytes)
                .expect("binary query response should decode")
        } else {
            serde_json::from_slice(bytes).expect("text query response should decode")
        }
    }

    match response.into_body() {
        ResponseBody::NoPayload => Vec::new(),
        ResponseBody::Items(items) => items.iter().map(|item| parse(item)).collect(),
        ResponseBody::Bytes(bytes) => parse(&bytes)["Documents"]
            .as_array()
            .cloned()
            .unwrap_or_default(),
    }
}

fn document_id(document: &serde_json::Value) -> Option<&str> {
    document["id"]
        .as_str()
        .or_else(|| document["current"]["id"].as_str())
}

fn query_operation(container: &ContainerReference, query: &str) -> CosmosOperation {
    CosmosOperation::query_items(container.clone(), Some(FeedRange::full()))
        .with_body(
            serde_json::to_vec(&serde_json::json!({
                "query": query,
                "parameters": [],
            }))
            .expect("query body should serialize"),
        )
        .with_max_item_count(MaxItemCountHint::Limit(
            NonZeroU32::new(1).expect("page size is non-zero"),
        ))
}

async fn execute_query_across_resume(
    driver: &CosmosDriver,
    container: &ContainerReference,
    query: &str,
) -> Result<Vec<serde_json::Value>, Box<dyn std::error::Error>> {
    let mut initial = Box::pin(
        driver
            .plan_operation(
                query_operation(container, query),
                &OperationOptions::default(),
                None,
                &PlanOptions::default(),
            )
            .await?,
    );
    let first = driver
        .execute_plan(
            &mut initial,
            Some(container.clone()),
            OperationOptions::default(),
        )
        .await?
        .ok_or("query must emit a first page before resume")?;
    let mut documents = page_documents(first);
    let token = initial.to_continuation_token()?;

    let mut resumed = Box::pin(
        driver
            .plan_operation(
                query_operation(container, query),
                &OperationOptions::default(),
                Some(&token),
                &PlanOptions::default(),
            )
            .await?,
    );
    while let Some(response) = driver
        .execute_plan(
            &mut resumed,
            Some(container.clone()),
            OperationOptions::default(),
        )
        .await?
    {
        documents.extend(page_documents(response));
    }

    Ok(documents)
}

#[tokio::test]
#[cfg_attr(
    not(any(test_category = "emulator", test_category = "emulator_vnext")),
    ignore = "requires test_category 'emulator' or 'emulator_vnext'"
)]
async fn live_valid_continuation_epk_bounds_round_trip() -> Result<(), Box<dyn std::error::Error>> {
    DriverTestClient::run_with_unique_db(async |context, database| {
        let container_name = context.unique_container_name();
        let container = context
            .create_container(&database, &container_name, "/pk")
            .await?;

        for rank in 0..6 {
            let id = format!("item-{rank}");
            let pk = format!("pk-{rank}");
            let body = serde_json::to_vec(&serde_json::json!({
                "id": id,
                "pk": pk,
                "rank": rank,
            }))?;
            context
                .create_seed_item(&container, &id, PartitionKey::from(pk), &body)
                .await?;
        }

        let driver = context.create_persistent_driver().await?;

        let sequential =
            execute_query_across_resume(&driver, &container, "SELECT * FROM c").await?;
        let sequential_ids: BTreeSet<_> = sequential
            .iter()
            .filter_map(|item| item["id"].as_str())
            .collect();
        assert_eq!(sequential.len(), 6);
        assert_eq!(sequential_ids.len(), 6);

        let ordered =
            execute_query_across_resume(&driver, &container, "SELECT * FROM c ORDER BY c.rank ASC")
                .await?;
        let ordered_ranks: Vec<_> = ordered
            .iter()
            .filter_map(|item| item["rank"].as_i64())
            .collect();
        assert_eq!(ordered_ranks, vec![0, 1, 2, 3, 4, 5]);

        let change_feed = CosmosOperation::change_feed(container.clone(), Some(FeedRange::full()))
            .with_change_feed_start(ChangeFeedStartFrom::Beginning)
            .with_max_item_count(MaxItemCountHint::Limit(
                NonZeroU32::new(1).expect("page size is non-zero"),
            ));
        let mut initial_feed = Box::pin(
            driver
                .plan_operation(
                    change_feed.clone(),
                    &OperationOptions::default(),
                    None,
                    &PlanOptions::default(),
                )
                .await?,
        );
        let first_feed_page = driver
            .execute_plan(
                &mut initial_feed,
                Some(container.clone()),
                OperationOptions::default(),
            )
            .await?
            .ok_or("change feed must emit a first page before resume")?;
        let mut feed_documents = page_documents(first_feed_page);
        assert!(
            feed_documents.len() < 6,
            "the first change-feed page must leave work for the resumed plan"
        );
        let feed_token = initial_feed.to_continuation_token()?;
        let mut resumed_feed = Box::pin(
            driver
                .plan_operation(
                    change_feed,
                    &OperationOptions::default(),
                    Some(&feed_token),
                    &PlanOptions::default(),
                )
                .await?,
        );
        let first_page_count = feed_documents.len();
        for _ in 0..64 {
            let response = driver
                .execute_plan(
                    &mut resumed_feed,
                    Some(container.clone()),
                    OperationOptions::default(),
                )
                .await?
                .ok_or("cross-partition change feed must remain active after resume")?;
            feed_documents.extend(page_documents(response));
            if feed_documents
                .iter()
                .filter_map(document_id)
                .collect::<BTreeSet<_>>()
                .len()
                == 6
            {
                break;
            }
        }
        let feed_ids: Vec<_> = feed_documents.iter().filter_map(document_id).collect();
        let unique_feed_ids: BTreeSet<_> = feed_ids.iter().copied().collect();
        assert!(
            feed_documents.len() > first_page_count,
            "the resumed change feed must emit additional documents"
        );
        assert_eq!(
            feed_ids.len(),
            6,
            "change-feed resume must not duplicate items"
        );
        assert_eq!(
            unique_feed_ids,
            BTreeSet::from(["item-0", "item-1", "item-2", "item-3", "item-4", "item-5",])
        );

        Ok(())
    })
    .await
}

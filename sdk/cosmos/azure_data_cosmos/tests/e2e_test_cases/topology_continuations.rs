// Copyright (c) Microsoft Corporation. All rights reserved.
// Licensed under the MIT License.

use std::{collections::BTreeSet, num::NonZeroU32};

use azure_data_cosmos::{
    feed::{ContinuationToken, FeedScope},
    models::ChangeFeedItem,
    options::{ChangeFeedOptions, ChangeFeedStartFrom, MaxItemCountHint, QueryOptions, Region},
    Query, RoutingStrategy,
};
use futures::StreamExt;

use crate::e2e_test_cases::{
    fixture::{build_client_with_defaults, ClientSetup, E2eTest, TestResult},
    support::{item, selected_scenario_profile, Item},
    topology_support::{with_manual_operation, ManagementOperation, TopologyManager},
};

const ITEM_COUNT: usize = 40;
const PAGE_SIZE: u32 = 5;

#[tokio::test]
#[cfg_attr(
    not(any(test_category = "emulator_inmemory", test_category = "e2e")),
    ignore = "requires the externally hosted in-memory emulator"
)]
async fn query_and_change_feed_resume_across_split_and_merge() -> TestResult {
    let Some(profile) = selected_scenario_profile("topology.continuation-transitions").await?
    else {
        return Ok(());
    };
    let client = build_client_with_defaults(ClientSetup::from_profile(
        profile.selected_runtime()?,
        profile.selected_client()?,
        RoutingStrategy::PreferredRegions(vec![
            Region::new("East US"),
            Region::new("West US"),
            Region::new("North Europe"),
        ]),
    )?)
    .await?;
    let manager = TopologyManager::from_env()?;

    E2eTest::builder()
        .with_client(client)
        .run(async |fixture| {
            for value in 0..ITEM_COUNT {
                let value = value as i64;
                let document = item(
                    &format!("transition-{value:02}"),
                    &format!("pk-{value}"),
                    value,
                );
                fixture
                    .container
                    .create_item(&document.pk, &document.id, &document, None)
                    .await?;
            }

            let (query_first, query_token) = capture_query(&fixture.container).await?;
            let (change_first, change_token) = capture_changes(&fixture.container).await?;
            let split = manager
                .start_split(&fixture.database_id, &fixture.container_id, 0)
                .await?;
            let split = finish_manual(&manager, split).await?;
            assert_eq!(split.children.len(), 2);
            assert_query_complete(
                resume_query(&fixture.container, query_first, query_token).await?,
            );
            assert_change_feed_complete(
                resume_changes(&fixture.container, change_first, change_token).await?,
            );

            let (query_first, query_token) = capture_query(&fixture.container).await?;
            let (change_first, change_token) = capture_changes(&fixture.container).await?;
            let merge = manager
                .start_merge(
                    &fixture.database_id,
                    &fixture.container_id,
                    [split.children[0], split.children[1]],
                )
                .await?;
            let merge = finish_manual(&manager, merge).await?;
            assert!(merge.into.is_some());
            assert_query_complete(
                resume_query(&fixture.container, query_first, query_token).await?,
            );
            assert_change_feed_complete(
                resume_changes(&fixture.container, change_first, change_token).await?,
            );
            Ok(())
        })
        .await
}

async fn finish_manual(
    manager: &TopologyManager,
    operation: ManagementOperation,
) -> TestResult<ManagementOperation> {
    with_manual_operation(manager, &operation.operation_id, async {
        assert_eq!(operation.phase, "Preparing");
        assert_eq!(
            manager.advance(&operation.operation_id).await?.phase,
            "Swapping"
        );
        manager.advance(&operation.operation_id).await?;
        manager.wait_for_succeeded(&operation.operation_id).await
    })
    .await
}

async fn capture_query(
    container: &azure_data_cosmos::clients::ContainerClient,
) -> TestResult<(Vec<Item>, ContinuationToken)> {
    let mut pages = container
        .query_items::<Item>(
            Query::from("SELECT * FROM c ORDER BY c.value"),
            FeedScope::full_container(),
            Some(query_options(None)),
        )
        .await?
        .into_pages();
    let page = pages.next().await.expect("query must yield a first page")?;
    let token = pages.to_continuation_token()?;
    Ok((
        page.into_items(),
        ContinuationToken::from_string(token.as_str().to_owned()),
    ))
}

async fn resume_query(
    container: &azure_data_cosmos::clients::ContainerClient,
    mut items: Vec<Item>,
    token: ContinuationToken,
) -> TestResult<Vec<Item>> {
    let mut pages = container
        .query_items::<Item>(
            Query::from("SELECT * FROM c ORDER BY c.value"),
            FeedScope::full_container(),
            Some(query_options(Some(token))),
        )
        .await?
        .into_pages();
    while let Some(page) = pages.next().await {
        items.extend(page?.into_items());
    }
    Ok(items)
}

fn query_options(token: Option<ContinuationToken>) -> QueryOptions {
    let mut options = QueryOptions::default()
        .with_max_item_count(MaxItemCountHint::Limit(NonZeroU32::new(PAGE_SIZE).unwrap()));
    if let Some(token) = token {
        options = options.with_continuation_token(token);
    }
    options
}

async fn capture_changes(
    container: &azure_data_cosmos::clients::ContainerClient,
) -> TestResult<(Vec<Item>, ContinuationToken)> {
    let mut pages = container
        .query_change_feed::<Item>(
            FeedScope::full_container(),
            ChangeFeedStartFrom::Beginning,
            Some(change_options(None)),
        )
        .await?;
    let page = pages
        .next()
        .await
        .expect("change feed must yield a first page")?;
    let token = pages.to_continuation_token()?;
    Ok((
        current_items(page.into_items()),
        ContinuationToken::from_string(token.as_str().to_owned()),
    ))
}

async fn resume_changes(
    container: &azure_data_cosmos::clients::ContainerClient,
    mut items: Vec<Item>,
    token: ContinuationToken,
) -> TestResult<Vec<Item>> {
    let mut pages = container
        .query_change_feed::<Item>(
            FeedScope::full_container(),
            ChangeFeedStartFrom::Beginning,
            Some(change_options(Some(token))),
        )
        .await?;
    for _ in 0..32 {
        if items.len() == ITEM_COUNT {
            return Ok(items);
        }
        let page = pages.next().await.expect("change feed remains pollable")?;
        items.extend(current_items(page.into_items()));
    }
    Err("change feed did not converge within the bounded poll count".into())
}

fn change_options(token: Option<ContinuationToken>) -> ChangeFeedOptions {
    let mut options = ChangeFeedOptions::default()
        .with_max_item_count(MaxItemCountHint::Limit(NonZeroU32::new(PAGE_SIZE).unwrap()));
    if let Some(token) = token {
        options = options.with_continuation_token(token);
    }
    options
}

fn current_items(changes: Vec<ChangeFeedItem<Item>>) -> Vec<Item> {
    changes
        .into_iter()
        .map(|change| {
            change
                .current()
                .cloned()
                .expect("LatestVersion change feed entries carry current documents")
        })
        .collect()
}

fn assert_query_complete(items: Vec<Item>) {
    assert_eq!(items.len(), ITEM_COUNT);
    assert_eq!(
        items.iter().map(|item| item.value).collect::<Vec<_>>(),
        (0..ITEM_COUNT as i64).collect::<Vec<_>>()
    );
}

fn assert_change_feed_complete(items: Vec<Item>) {
    let ids: BTreeSet<_> = items.iter().map(|item| item.id.as_str()).collect();
    assert_eq!(items.len(), ids.len(), "change feed replayed an item");
    assert_eq!(ids.len(), ITEM_COUNT, "change feed lost an item");
}

// Copyright (c) Microsoft Corporation. All rights reserved.
// Licensed under the MIT License.

use azure_data_cosmos::{
    options::{MaxItemCountHint, QueryOptions},
    FeedScope, Query,
};
use cosspec::{
    assert_feed_ranges, spec_test, testdata::Item, ExclusiveContainer, SharedContainer, SpecResult,
};
use futures::{StreamExt, TryStreamExt};
use std::num::NonZeroU32;

fn pages_of(size: u32) -> QueryOptions {
    QueryOptions::default()
        .with_max_item_count(MaxItemCountHint::Limit(NonZeroU32::new(size).unwrap()))
}

/// A query resumed from a continuation token returns the remaining items, with
/// none lost or repeated.
#[spec_test]
pub async fn continuation_resumes_without_loss(
    #[profile(basic_items)] container: SharedContainer,
) -> SpecResult {
    let client = container.client();

    let mut pages = client
        .query_items::<Item>(
            "SELECT * FROM c ORDER BY c.id",
            FeedScope::full_container(),
            Some(pages_of(5)),
        )
        .await?
        .into_pages();
    let mut items = pages.next().await.expect("a first page")?.into_items();
    let token = pages.to_continuation_token()?;
    drop(pages);

    let options = pages_of(5).with_continuation_token(token);
    let resumed = client
        .query_items::<Item>(
            Query::from("SELECT * FROM c ORDER BY c.id"),
            FeedScope::full_container(),
            Some(options),
        )
        .await?;
    items.extend(resumed.try_collect::<Vec<_>>().await?);

    assert_eq!(items, container.dataset().all());
    Ok(())
}

/// A continuation token taken before a partition split still resumes the query
/// after the split, with no item lost or repeated.
#[spec_test]
#[requires(TopologyControl)]
pub async fn continuation_survives_split(
    #[profile(basic_items)] container: ExclusiveContainer,
) -> SpecResult {
    let client = container.client();
    assert_feed_ranges!(container, 1);

    let mut pages = client
        .query_items::<Item>(
            "SELECT * FROM c ORDER BY c.id",
            FeedScope::full_container(),
            Some(pages_of(5)),
        )
        .await?
        .into_pages();
    let mut items = pages.next().await.expect("a first page")?.into_items();
    let token = pages.to_continuation_token()?;
    drop(pages);

    container.split().await?;
    assert_feed_ranges!(container, 2);

    let options = pages_of(5).with_continuation_token(token);
    let resumed = client
        .query_items::<Item>(
            "SELECT * FROM c ORDER BY c.id",
            FeedScope::full_container(),
            Some(options),
        )
        .await?;
    items.extend(resumed.try_collect::<Vec<_>>().await?);

    assert_eq!(items, container.dataset().all());
    Ok(())
}

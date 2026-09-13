// Copyright (c) Microsoft Corporation. All rights reserved.
// Licensed under the MIT License.

use std::{collections::BTreeSet, num::NonZeroU32};

use azure_data_cosmos::{
    feed::{ContinuationToken, FeedScope},
    options::{MaxItemCountHint, QueryOptions},
    Query,
};
use futures::StreamExt;

use crate::e2e_test_cases::{
    fixture::{E2eTestFixture, TestResult},
    support::{gateway_request_counts, item, should_run, Item},
};

#[tokio::test]
#[cfg_attr(
    not(any(test_category = "emulator_inmemory", test_category = "e2e")),
    ignore = "requires the externally hosted in-memory emulator"
)]
async fn query_resumes_without_loss_or_duplication() -> TestResult {
    if !should_run("query.pagination-resume").await? {
        return Ok(());
    }

    E2eTestFixture::run(async |fixture| {
        const ITEM_COUNT: i64 = 24;
        let partition_keys: Vec<_> = (0..6).map(|value| format!("partition-{value}")).collect();
        let mut mapped_ranges = BTreeSet::new();
        for partition_key in &partition_keys {
            let ranges = fixture
                .container
                .feed_range_from_partition_key(partition_key.clone(), None)
                .await?;
            assert_eq!(ranges.len(), 1);
            mapped_ranges.insert(ranges[0].to_string());
        }
        assert!(
            mapped_ranges.len() >= 2,
            "query fixture must span at least two physical feed ranges"
        );

        for value in 0..ITEM_COUNT {
            let id = format!("item-{value}");
            let partition_key = &partition_keys[value as usize % partition_keys.len()];
            fixture
                .container
                .create_item(
                    partition_key.clone(),
                    &id,
                    item(&id, partition_key, value),
                    None,
                )
                .await?;
        }

        let query = Query::from("SELECT * FROM c ORDER BY c.value ASC");
        let page_size = MaxItemCountHint::Limit(NonZeroU32::new(2).unwrap());
        let mut first_iterator = fixture
            .container
            .query_items::<Item>(
                query.clone(),
                FeedScope::full_container(),
                Some(QueryOptions::default().with_max_item_count(page_size)),
            )
            .await?
            .into_pages();
        // Query planning and routing metadata may use the standard gateway.
        // Capture counters only after planning so the page-fetch delta proves
        // every data request remained on Gateway V2.
        let gateway_before_pages = gateway_request_counts().await?;
        let first_page = first_iterator
            .next()
            .await
            .expect("five results must produce a first page")?;
        assert!(!first_page.items().is_empty());
        assert!(first_page.items().len() < ITEM_COUNT as usize);
        let token = first_iterator.to_continuation_token()?;
        let token = ContinuationToken::from_string(token.as_str().to_owned());
        let mut actual = first_page.into_items();
        drop(first_iterator);

        let options = QueryOptions::default()
            .with_max_item_count(page_size)
            .with_continuation_token(token);
        let mut resumed = fixture
            .container
            .query_items::<Item>(query, FeedScope::full_container(), Some(options))
            .await?
            .into_pages();
        while let Some(page) = resumed.next().await {
            let page = page?;
            actual.extend(page.into_items());
        }

        assert_eq!(
            actual.iter().map(|value| value.value).collect::<Vec<_>>(),
            (0..ITEM_COUNT).collect::<Vec<_>>()
        );
        if let Some(before) = gateway_before_pages {
            let after = gateway_request_counts()
                .await?
                .expect("Gateway V2 metrics must remain available");
            assert!(
                after.gateway_v2 > before.gateway_v2,
                "query pages must issue Gateway V2 requests"
            );
            assert_eq!(
                after.gateway, before.gateway,
                "query page requests must not fall back to the standard gateway"
            );
        }
        Ok(())
    })
    .await
}

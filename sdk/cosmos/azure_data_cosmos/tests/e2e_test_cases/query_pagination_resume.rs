// Copyright (c) Microsoft Corporation. All rights reserved.
// Licensed under the MIT License.

use std::num::NonZeroU32;

use azure_data_cosmos::{
    feed::{ContinuationToken, FeedScope},
    options::{MaxItemCountHint, QueryOptions},
    Query,
};
use futures::StreamExt;

use crate::e2e_test_cases::{
    fixture::{E2eTestFixture, TestResult},
    support::{assert_configured_transport, item, should_run, Item},
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
        let first_page = first_iterator
            .next()
            .await
            .expect("query results must produce a first page")?;
        assert_configured_transport(first_page.diagnostics().as_ref());
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
        let mut resumed_request_observed = false;
        while let Some(page) = resumed.next().await {
            let page = page?;
            if page.diagnostics().request_count() > 0 {
                assert_configured_transport(page.diagnostics().as_ref());
                resumed_request_observed = true;
            }
            actual.extend(page.into_items());
        }
        assert!(
            resumed_request_observed,
            "resumed query must issue at least one backend request"
        );

        assert_eq!(
            actual.iter().map(|value| value.value).collect::<Vec<_>>(),
            (0..ITEM_COUNT).collect::<Vec<_>>()
        );
        Ok(())
    })
    .await
}

// Copyright (c) Microsoft Corporation. All rights reserved.
// Licensed under the MIT License.

use std::num::NonZeroU32;

use azure_data_cosmos::{
    feed::{ContinuationToken, FeedScope},
    models::ChangeFeedItem,
    options::{ChangeFeedOptions, ChangeFeedStartFrom, MaxItemCountHint},
};
use futures::StreamExt;

use crate::e2e_test_cases::{
    fixture::{E2eTestFixture, TestResult},
    support::{assert_critical_diagnostics, item, should_run, Item},
};

#[tokio::test]
#[cfg_attr(
    not(any(test_category = "emulator_inmemory", test_category = "e2e")),
    ignore = "requires the externally hosted in-memory emulator"
)]
async fn change_feed_resumes_without_replay() -> TestResult {
    if !should_run("changefeed.pagination-resume").await? {
        return Ok(());
    }

    E2eTestFixture::run(async |fixture| {
        for value in 0..5 {
            let id = format!("change-{value}");
            fixture
                .container
                .create_item("A", &id, item(&id, "A", value), None)
                .await?;
        }

        let page_size = MaxItemCountHint::Limit(NonZeroU32::new(2).unwrap());
        let mut first_iterator = fixture
            .container
            .query_change_feed::<Item>(
                FeedScope::partition("A"),
                ChangeFeedStartFrom::Beginning,
                Some(ChangeFeedOptions::default().with_max_item_count(page_size)),
            )
            .await?;
        let first_page = first_iterator
            .next()
            .await
            .expect("five changes must produce a first page")?;
        assert_critical_diagnostics(
            first_page.diagnostics().as_ref(),
            "query_change_feed",
            azure_core::http::StatusCode::Ok,
        );
        assert!(!first_page.items().is_empty());
        assert!(first_page.items().len() < 5);
        let token = first_iterator.to_continuation_token()?;
        let token = ContinuationToken::from_string(token.as_str().to_owned());
        let mut actual = current_items(first_page.into_items());
        drop(first_iterator);

        let options = ChangeFeedOptions::default()
            .with_max_item_count(page_size)
            .with_continuation_token(token);
        let mut resumed = fixture
            .container
            .query_change_feed::<Item>(
                FeedScope::partition("A"),
                ChangeFeedStartFrom::Beginning,
                Some(options),
            )
            .await?;
        for _ in 0..10 {
            if actual.len() == 5 {
                break;
            }
            let page = resumed
                .next()
                .await
                .expect("change feed remains pollable")?;
            assert_critical_diagnostics(
                page.diagnostics().as_ref(),
                "query_change_feed",
                azure_core::http::StatusCode::Ok,
            );
            actual.extend(current_items(page.into_items()));
        }

        actual.sort_by_key(|value| value.value);
        assert_eq!(
            actual.iter().map(|value| value.value).collect::<Vec<_>>(),
            [0, 1, 2, 3, 4]
        );
        Ok(())
    })
    .await?;

    E2eTestFixture::run(async |fixture| {
        let mut now = fixture
            .container
            .query_change_feed::<Item>(FeedScope::partition("A"), ChangeFeedStartFrom::Now, None)
            .await?;
        let empty = now.next().await.expect("change feed remains pollable")?;
        assert_critical_diagnostics(
            empty.diagnostics().as_ref(),
            "query_change_feed",
            azure_core::http::StatusCode::NotModified,
        );
        assert!(empty.items().is_empty());
        let token = now.to_continuation_token()?;
        let token = ContinuationToken::from_string(token.as_str().to_owned());
        drop(now);

        let expected = item("after-now", "A", 42);
        fixture
            .container
            .create_item("A", &expected.id, &expected, None)
            .await?;

        let options = ChangeFeedOptions::default().with_continuation_token(token);
        let mut resumed = fixture
            .container
            .query_change_feed::<Item>(
                FeedScope::partition("A"),
                ChangeFeedStartFrom::Beginning,
                Some(options),
            )
            .await?;
        let page = resumed
            .next()
            .await
            .expect("post-checkpoint change must produce a page")?;
        assert_critical_diagnostics(
            page.diagnostics().as_ref(),
            "query_change_feed",
            azure_core::http::StatusCode::Ok,
        );
        assert_eq!(current_items(page.into_items()), [expected]);
        Ok(())
    })
    .await
}

fn current_items(changes: Vec<ChangeFeedItem<Item>>) -> Vec<Item> {
    changes
        .into_iter()
        .map(|change| {
            assert!(
                change.operation_type().is_none(),
                "LatestVersion metadata must not expose an operation type"
            );
            change
                .current()
                .cloned()
                .expect("LatestVersion changes must carry a current item")
        })
        .collect()
}

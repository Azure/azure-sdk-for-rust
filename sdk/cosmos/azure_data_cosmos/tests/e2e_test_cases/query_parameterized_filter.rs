// Copyright (c) Microsoft Corporation. All rights reserved.
// Licensed under the MIT License.

use azure_data_cosmos::{feed::FeedScope, Query};
use futures::{StreamExt, TryStreamExt};

use crate::e2e_test_cases::{
    fixture::{E2eTestFixture, TestResult},
    support::{item, should_run, Item},
};

#[tokio::test]
#[cfg_attr(
    not(any(test_category = "emulator_inmemory", test_category = "e2e")),
    ignore = "requires the externally hosted in-memory emulator"
)]
async fn parameterized_query_filters_and_orders() -> TestResult {
    if !should_run("query.parameterized-filter").await? {
        return Ok(());
    }
    E2eTestFixture::run(async |fixture| {
        // Arrange IDs and scores in different orders so ORDER BY is observable.
        for (id, score) in [("a", 3), ("b", 1), ("c", 2)] {
            let mut value = item(id, "A", score);
            value.score = Some(score);
            fixture.container.create_item("A", id, value, None).await?;
        }

        // Bind values as parameters and restrict execution to partition A.
        let query = Query::from(
            "SELECT * FROM c WHERE c.pk = @pk AND c.score >= @min ORDER BY c.score ASC",
        )
        .with_parameter("@pk", "A")?
        .with_parameter("@min", 2)?;
        let mut results = fixture
            .container
            .query_items::<Item>(query, FeedScope::partition("A"), None)
            .await?;
        let items: Vec<Item> = results.by_ref().try_collect().await?;

        // The filter excludes score 1 and ordering differs from ID/insertion order.
        assert_eq!(
            items
                .iter()
                .map(|item| item.id.as_str())
                .collect::<Vec<_>>(),
            ["c", "a"]
        );

        let descending = Query::from(
            "SELECT * FROM c WHERE c.pk = @pk AND c.score >= @min ORDER BY c.score DESC",
        )
        .with_parameter("@pk", "A")?
        .with_parameter("@min", 2)?;
        let descending_items: Vec<Item> = fixture
            .container
            .query_items::<Item>(descending, FeedScope::partition("A"), None)
            .await?
            .try_collect()
            .await?;
        assert_eq!(
            descending_items
                .iter()
                .map(|item| item.id.as_str())
                .collect::<Vec<_>>(),
            ["a", "c"]
        );
        Ok(())
    })
    .await
}

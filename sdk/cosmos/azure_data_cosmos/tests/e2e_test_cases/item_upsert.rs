// Copyright (c) Microsoft Corporation. All rights reserved.
// Licensed under the MIT License.

use azure_core::http::StatusCode;
use azure_data_cosmos::{feed::FeedScope, Query};
use futures::TryStreamExt;

use crate::e2e_test_cases::{
    fixture::{E2eTestFixture, TestResult},
    support::{assert_critical_diagnostics, item, should_run, write_options_with_content, Item},
};

#[tokio::test]
#[cfg_attr(
    not(any(test_category = "emulator_inmemory", test_category = "e2e")),
    ignore = "requires the externally hosted in-memory emulator"
)]
async fn upsert_creates_then_updates() -> TestResult {
    if !should_run("item.upsert-create-update")? {
        return Ok(());
    }
    E2eTestFixture::run(async |fixture| {
        // Case 1: upserting a missing identity creates value 1 and returns 201.
        let created = fixture
            .container
            .upsert_item(
                "A",
                "upsert-1",
                item("upsert-1", "A", 1),
                Some(write_options_with_content()),
            )
            .await?;
        assert_eq!(created.status().status_code(), StatusCode::Created);
        assert_critical_diagnostics(&created.diagnostics(), "upsert_item", StatusCode::Created);

        // Case 2: upserting the same identity replaces it with value 2 and returns 200.
        let updated = fixture
            .container
            .upsert_item(
                "A",
                "upsert-1",
                item("upsert-1", "A", 2),
                Some(write_options_with_content()),
            )
            .await?;
        assert_eq!(updated.status().status_code(), StatusCode::Ok);
        assert_critical_diagnostics(&updated.diagnostics(), "upsert_item", StatusCode::Ok);
        assert_eq!(updated.into_model::<Item>()?, item("upsert-1", "A", 2));

        // Both operations addressed one identity; no duplicate document was created.
        let items: Vec<Item> = fixture
            .container
            .query_items(
                Query::from("SELECT * FROM c WHERE c.id = @id")
                    .with_parameter("@id", "upsert-1")?,
                FeedScope::partition("A"),
                None,
            )
            .await?
            .try_collect()
            .await?;
        assert_eq!(items, [item("upsert-1", "A", 2)]);
        Ok(())
    })
    .await
}

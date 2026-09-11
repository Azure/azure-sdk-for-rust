// Copyright (c) Microsoft Corporation. All rights reserved.
// Licensed under the MIT License.

use azure_core::http::StatusCode;
use azure_data_cosmos::options::{ItemWriteOptions, Precondition};

use crate::e2e_test_cases::{
    fixture::{E2eTestFixture, TestResult},
    support::{assert_critical_diagnostics, item, should_run, Item},
};

#[tokio::test]
#[cfg_attr(
    not(any(test_category = "emulator_inmemory", test_category = "e2e")),
    ignore = "requires the externally hosted in-memory emulator"
)]
async fn stale_etag_preserves_successful_update() -> TestResult {
    if !should_run("item.optimistic-concurrency").await? {
        return Ok(());
    }
    E2eTestFixture::run(async |fixture| {
        // Arrange an item and retain its initial ETag.
        let created = fixture
            .container
            .create_item("A", "etag-1", item("etag-1", "A", 1), None)
            .await?;
        assert_critical_diagnostics(&created.diagnostics(), "create_item", StatusCode::Created);
        let initial_etag = created
            .headers()
            .etag()
            .expect("create must return an ETag")
            .clone();

        // Case 1: the current ETag permits the update from value 1 to value 2.
        let current_options = ItemWriteOptions::default()
            .with_precondition(Precondition::IfMatch(initial_etag.clone()));
        let replaced = fixture
            .container
            .replace_item("A", "etag-1", item("etag-1", "A", 2), Some(current_options))
            .await?;
        assert_eq!(replaced.status().status_code(), StatusCode::Ok);
        assert_critical_diagnostics(&replaced.diagnostics(), "replace_item", StatusCode::Ok);

        // Case 2: reusing the now-stale initial ETag cannot overwrite value 2.
        let stale_options =
            ItemWriteOptions::default().with_precondition(Precondition::IfMatch(initial_etag));
        let error = fixture
            .container
            .replace_item("A", "etag-1", item("etag-1", "A", 3), Some(stale_options))
            .await
            .expect_err("stale ETag must fail");
        assert_eq!(error.status().status_code(), StatusCode::PreconditionFailed);
        assert_eq!(
            error
                .status()
                .sub_status()
                .map(|value| value.value())
                .unwrap_or(0),
            0
        );
        assert_critical_diagnostics(
            &error
                .diagnostics()
                .expect("service error must carry diagnostics"),
            "replace_item",
            StatusCode::PreconditionFailed,
        );

        // The rejected write leaves the successful value 2 update intact.
        assert_eq!(
            fixture
                .container
                .read_item("A", "etag-1", None)
                .await?
                .into_model::<Item>()?,
            item("etag-1", "A", 2)
        );
        Ok(())
    })
    .await
}

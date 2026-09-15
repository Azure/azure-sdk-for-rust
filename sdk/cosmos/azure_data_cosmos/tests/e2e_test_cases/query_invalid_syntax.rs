// Copyright (c) Microsoft Corporation. All rights reserved.
// Licensed under the MIT License.

use azure_core::http::StatusCode;
use azure_data_cosmos::feed::FeedScope;
use futures::StreamExt;

use crate::e2e_test_cases::{
    fixture::{E2eTestFixture, TestResult},
    support::{should_run, Item},
};

#[tokio::test]
#[cfg_attr(
    not(any(test_category = "emulator_inmemory", test_category = "e2e")),
    ignore = "requires the externally hosted in-memory emulator"
)]
async fn invalid_query_is_not_an_empty_feed() -> TestResult {
    if !should_run("query.invalid-syntax").await? {
        return Ok(());
    }
    E2eTestFixture::run(async |fixture| {
        // Query failures may surface while creating the iterator or while reading its first page.
        let result = fixture
            .container
            .query_items::<Item>("SELECT FROM", FeedScope::partition("A"), None)
            .await;
        let error = match result {
            Err(error) => error,
            Ok(mut stream) => stream
                .next()
                .await
                .expect("invalid query must produce an error")
                .expect_err("invalid query must not produce a page"),
        };

        // Invalid syntax is a typed bad request, never an empty successful feed.
        assert_eq!(error.status().status_code(), StatusCode::BadRequest);
        Ok(())
    })
    .await
}

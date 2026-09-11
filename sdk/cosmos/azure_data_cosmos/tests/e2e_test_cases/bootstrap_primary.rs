// Copyright (c) Microsoft Corporation. All rights reserved.
// Licensed under the MIT License.

use crate::e2e_test_cases::{
    fixture::{E2eTestFixture, TestResult},
    support::{item, should_run, Item},
};

#[tokio::test]
#[cfg_attr(
    not(any(test_category = "emulator_inmemory", test_category = "e2e")),
    ignore = "requires the externally hosted in-memory emulator"
)]
async fn bootstrap_primary_endpoint() -> TestResult {
    if !should_run("bootstrap.primary-success").await? {
        return Ok(());
    }

    E2eTestFixture::run(async |fixture| {
        let expected = item("bootstrap-1", "A", 1);
        fixture
            .container
            .create_item("A", &expected.id, &expected, None)
            .await?;
        let actual = fixture
            .container
            .read_item("A", &expected.id, None)
            .await?
            .into_model::<Item>()?;
        assert_eq!(actual, expected);
        Ok(())
    })
    .await
}

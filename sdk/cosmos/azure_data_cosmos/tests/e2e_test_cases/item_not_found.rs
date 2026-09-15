// Copyright (c) Microsoft Corporation. All rights reserved.
// Licensed under the MIT License.

use azure_core::http::StatusCode;

use crate::e2e_test_cases::{
    fixture::{E2eTestFixture, TestResult},
    support::{assert_critical_diagnostics, item, should_run, Item},
};

#[tokio::test]
#[cfg_attr(
    not(any(test_category = "emulator_inmemory", test_category = "e2e")),
    ignore = "requires the externally hosted in-memory emulator"
)]
async fn not_found_does_not_cross_partition_keys() -> TestResult {
    if !should_run("item.not-found-wrong-partition-key").await? {
        return Ok(());
    }
    E2eTestFixture::run(async |fixture| {
        // Arrange one item in logical partition A.
        fixture
            .container
            .create_item("A", "item-1", item("item-1", "A", 1), None)
            .await?;

        // Case 1: a missing ID in the correct partition returns a plain 404.
        let missing = fixture
            .container
            .read_item("A", "missing", None)
            .await
            .expect_err("missing item read must fail");
        assert_plain_not_found(&missing);

        // Case 2: the existing ID cannot be read through a different partition key.
        let wrong_partition = fixture
            .container
            .read_item("B", "item-1", None)
            .await
            .expect_err("wrong-partition-key read must fail");
        assert_plain_not_found(&wrong_partition);

        // Neither failed read mutated or hid the correctly addressed item.
        let read = fixture.container.read_item("A", "item-1", None).await?;
        assert_eq!(read.into_model::<Item>()?, item("item-1", "A", 1));
        Ok(())
    })
    .await
}

fn assert_plain_not_found(error: &azure_data_cosmos::CosmosError) {
    assert_eq!(error.status().status_code(), StatusCode::NotFound);
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
        "read_item",
        StatusCode::NotFound,
    );
}

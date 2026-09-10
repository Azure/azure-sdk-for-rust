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
    if !should_run("item.not-found-wrong-partition-key")? {
        return Ok(());
    }
    E2eTestFixture::run(async |fixture| {
        fixture
            .container
            .create_item("A", "item-1", item("item-1", "A", 1), None)
            .await?;
        for (id, pk) in [("missing", "A"), ("item-1", "B")] {
            let error = fixture
                .container
                .read_item(pk, id, None)
                .await
                .expect_err("read must return not found");
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
        let read = fixture.container.read_item("A", "item-1", None).await?;
        assert_eq!(read.into_model::<Item>()?.value, 1);
        Ok(())
    })
    .await
}

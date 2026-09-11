// Copyright (c) Microsoft Corporation. All rights reserved.
// Licensed under the MIT License.

use azure_core::http::StatusCode;
use azure_data_cosmos::options::Region;

use crate::e2e_test_cases::{
    fixture::{E2eTestFixture, TestResult},
    support::{assert_critical_diagnostics, item, should_run},
};

#[tokio::test]
#[cfg_attr(
    not(any(test_category = "emulator_inmemory", test_category = "e2e")),
    ignore = "requires the externally hosted in-memory emulator"
)]
async fn diagnostics_cover_success_and_error() -> TestResult {
    if !should_run("diagnostics.success-and-error").await? {
        return Ok(());
    }
    E2eTestFixture::run(async |fixture| {
        // Arrange one readable item in East US.
        fixture
            .container
            .create_item("A", "item-1", item("item-1", "A", 1), None)
            .await?;

        // Success diagnostics identify the operation, 200 status, activity, request, and region.
        let success = fixture.container.read_item("A", "item-1", None).await?;
        assert_eq!(success.status().status_code(), StatusCode::Ok);
        assert_critical_diagnostics(&success.diagnostics(), "read_item", StatusCode::Ok);
        assert_eq!(
            success.diagnostics().regions_contacted(),
            vec![Region::EAST_US]
        );

        // Error diagnostics preserve the same fields while reporting the terminal plain 404.
        let error = fixture
            .container
            .read_item("A", "missing", None)
            .await
            .expect_err("missing read must fail");
        assert_eq!(error.status().status_code(), StatusCode::NotFound);
        assert_eq!(
            error
                .status()
                .sub_status()
                .map(|value| value.value())
                .unwrap_or(0),
            0
        );
        let diagnostics = error
            .diagnostics()
            .expect("service error must carry diagnostics");
        assert_critical_diagnostics(&diagnostics, "read_item", StatusCode::NotFound);
        assert_eq!(diagnostics.regions_contacted(), vec![Region::EAST_US]);
        Ok(())
    })
    .await
}

// Copyright (c) Microsoft Corporation. All rights reserved.
// Licensed under the MIT License.

use azure_data_cosmos::models::{PatchInstructions, PatchOperation};

use crate::e2e_test_cases::{
    fixture::{E2eTestFixture, TestResult},
    support::{assert_critical_diagnostics, item, should_run, Item},
};

#[tokio::test]
#[cfg_attr(
    not(any(test_category = "emulator_inmemory", test_category = "e2e")),
    ignore = "requires the externally hosted in-memory emulator"
)]
async fn patch_paths_persist_exact_post_images() -> TestResult {
    if !should_run("item.patch-state").await? {
        return Ok(());
    }

    E2eTestFixture::run(async |fixture| {
        let original = item("patch", "A", 1);
        fixture
            .container
            .create_item("A", &original.id, &original, None)
            .await?;

        let safe_patch =
            PatchInstructions::from(vec![PatchOperation::set("/score", serde_json::json!(10))]);
        let safe_response = fixture
            .container
            .patch_item("A", &original.id, safe_patch, None)
            .await?;
        assert_critical_diagnostics(
            safe_response.diagnostics().as_ref(),
            "patch_item",
            azure_core::http::StatusCode::Ok,
        );
        assert_eq!(safe_response.diagnostics().request_count(), 1);
        let safe_post_image: Item = safe_response.into_model()?;
        assert_eq!(safe_post_image.score, Some(10));
        assert_eq!(safe_post_image.value, 1);

        let read_modify_write =
            PatchInstructions::from(vec![PatchOperation::increment("/value", 4_i64)]);
        let rmw_response = fixture
            .container
            .patch_item("A", &original.id, read_modify_write, None)
            .await?;
        assert_critical_diagnostics(
            rmw_response.diagnostics().as_ref(),
            "patch_item",
            azure_core::http::StatusCode::Ok,
        );
        assert!(rmw_response.diagnostics().request_count() >= 2);
        let rmw_post_image: Item = rmw_response.into_model()?;
        assert_eq!(rmw_post_image.value, 5);
        assert_eq!(rmw_post_image.score, Some(10));

        let stored: Item = fixture
            .container
            .read_item("A", &original.id, None)
            .await?
            .into_model()?;
        assert_eq!(stored, rmw_post_image);
        Ok(())
    })
    .await
}

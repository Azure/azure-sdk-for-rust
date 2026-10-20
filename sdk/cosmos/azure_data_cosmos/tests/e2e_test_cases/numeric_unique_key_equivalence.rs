// Copyright (c) Microsoft Corporation. All rights reserved.
// Licensed under the MIT License.

use azure_core::{http::StatusCode, Uuid};
use azure_data_cosmos::models::{ContainerProperties, UniqueKey, UniqueKeyPolicy};

use crate::e2e_test_cases::{
    fixture::{E2eTestFixture, TestResult},
    support::should_run,
};

#[tokio::test]
#[cfg_attr(
    not(any(test_category = "emulator_inmemory", test_category = "e2e")),
    ignore = "requires the externally hosted in-memory emulator or Azure Live"
)]
async fn wide_integers_follow_service_unique_key_equivalence() -> TestResult {
    if !should_run("item.numeric-unique-key-equivalence").await? {
        return Ok(());
    }

    let container_id = format!("unique-numeric-{}", Uuid::new_v4());
    let properties = ContainerProperties::new(container_id, "/pk".into()).with_unique_key_policy(
        UniqueKeyPolicy::default().with_unique_key(UniqueKey::default().with_path("/value")),
    );
    E2eTestFixture::run_with_container_properties(properties, async |fixture| {
        let first = serde_json::json!({
            "id": "wide-a",
            "pk": "A",
            "value": 9_007_199_254_740_992_u64,
        });
        let second = serde_json::json!({
            "id": "wide-b",
            "pk": "A",
            "value": 9_007_199_254_740_993_u64,
        });

        let response = fixture
            .container
            .create_item("A", "wide-a", first, None)
            .await?;
        assert_eq!(response.status(), StatusCode::Created);

        let error = fixture
            .container
            .create_item("A", "wide-b", second, None)
            .await
            .expect_err("service-equivalent wide integers must conflict under a unique key");
        assert_eq!(error.status().status_code(), StatusCode::Conflict);
        Ok(())
    })
    .await
}

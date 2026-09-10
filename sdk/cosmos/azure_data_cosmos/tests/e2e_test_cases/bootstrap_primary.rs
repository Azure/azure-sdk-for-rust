// Copyright (c) Microsoft Corporation. All rights reserved.
// Licensed under the MIT License.

use azure_core::http::StatusCode;

use crate::e2e_test_cases::{
    fixture::{build_client, TestResult},
    support::{hosted_only, should_run},
};

#[tokio::test]
#[cfg_attr(
    not(any(test_category = "emulator_inmemory", test_category = "e2e")),
    ignore = "requires the externally hosted in-memory emulator"
)]
async fn bootstrap_primary_endpoint() -> TestResult {
    if !should_run("bootstrap.primary-success")? {
        return Ok(());
    }

    // Building the public SDK client against the reachable primary endpoint succeeds.
    let client = build_client().await?;
    assert!(hosted_only());

    // The initialized client can complete its first account operation.
    let database_id = format!("e2e-bootstrap-{}", azure_core::Uuid::new_v4());
    let response = client.create_database(&database_id, None).await?;
    assert_eq!(response.status().status_code(), StatusCode::Created);
    response.into_model()?;

    // Remove the resource created only to prove successful bootstrap.
    client.database_client(&database_id).delete(None).await?;
    Ok(())
}

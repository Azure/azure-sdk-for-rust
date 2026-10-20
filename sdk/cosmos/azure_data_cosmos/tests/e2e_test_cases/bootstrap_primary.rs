// Copyright (c) Microsoft Corporation. All rights reserved.
// Licensed under the MIT License.

use std::panic::AssertUnwindSafe;

use azure_core::http::StatusCode;
use futures::FutureExt;

use crate::e2e_test_cases::{
    fixture::{build_client, TestResult},
    support::should_run,
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

    // Building the public SDK client against the reachable primary endpoint succeeds.
    let client = build_client().await?;

    // The initialized client can complete its first account operation.
    let database_id = format!("e2e-bootstrap-{}", azure_core::Uuid::new_v4());
    let response = client.create_database(&database_id, None).await?;
    let database = client.database_client(&database_id);
    let outcome = AssertUnwindSafe(async {
        assert_eq!(response.status().status_code(), StatusCode::Created);
        response.into_model()?;
        Ok::<_, Box<dyn std::error::Error>>(())
    })
    .catch_unwind()
    .await;
    let cleanup = database.delete(None).await;

    match outcome {
        Ok(Ok(())) => {
            cleanup?;
            Ok(())
        }
        Ok(Err(test_error)) => match cleanup {
            Ok(_) => Err(test_error),
            Err(cleanup_error) => Err(format!(
                "bootstrap validation failed: {test_error}; database cleanup also failed: {cleanup_error}"
            )
            .into()),
        },
        Err(panic) => {
            if let Err(error) = cleanup {
                eprintln!("E2E database cleanup after bootstrap panic failed: {error}");
            }
            std::panic::resume_unwind(panic)
        }
    }
}

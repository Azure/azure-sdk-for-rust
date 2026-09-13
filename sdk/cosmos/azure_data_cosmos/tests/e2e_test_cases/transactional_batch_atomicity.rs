// Copyright (c) Microsoft Corporation. All rights reserved.
// Licensed under the MIT License.

use azure_core::http::StatusCode;
use azure_data_cosmos::TransactionalBatch;

use crate::e2e_test_cases::{
    fixture::{E2eTestFixture, TestResult},
    support::{assert_critical_diagnostics, item, should_run, Item},
};

#[tokio::test]
#[cfg_attr(
    not(any(test_category = "emulator_inmemory", test_category = "e2e")),
    ignore = "requires the externally hosted in-memory emulator"
)]
async fn batch_success_and_failure_are_atomic() -> TestResult {
    if !should_run("batch.atomicity").await? {
        return Ok(());
    }

    E2eTestFixture::run(async |fixture| {
        let committed = item("committed", "A", 1);
        let successful = TransactionalBatch::new("A")
            .create_item(&committed)?
            .read_item(committed.id.clone(), None);
        let success = fixture
            .container
            .execute_transactional_batch(successful, None)
            .await?;
        assert_eq!(success.status(), StatusCode::Ok);
        assert_critical_diagnostics(
            success.diagnostics().as_ref(),
            "execute_batch",
            StatusCode::Ok,
        );
        let success = success.into_model()?;
        assert_eq!(
            success
                .results()
                .iter()
                .map(|result| result.status_code())
                .collect::<Vec<_>>(),
            [201, 200]
        );
        let read_in_batch: Item = success.results()[1]
            .into_model()?
            .expect("batch read must return the committed item");
        assert_eq!(read_in_batch, committed);

        let rolled_back = item("rolled-back", "A", 2);
        let failing = TransactionalBatch::new("A")
            .create_item(&rolled_back)?
            .delete_item("missing", None);
        let failure = fixture
            .container
            .execute_transactional_batch(failing, None)
            .await?;
        assert_eq!(failure.status(), StatusCode::MultiStatus);
        assert_critical_diagnostics(
            failure.diagnostics().as_ref(),
            "execute_batch",
            StatusCode::MultiStatus,
        );
        let failure = failure.into_model()?;
        assert_eq!(
            failure
                .results()
                .iter()
                .map(|result| result.status_code())
                .collect::<Vec<_>>(),
            [424, 404]
        );

        let committed_after_failure: Item = fixture
            .container
            .read_item("A", &committed.id, None)
            .await?
            .into_model()?;
        assert_eq!(committed_after_failure, committed);
        let error = fixture
            .container
            .read_item("A", &rolled_back.id, None)
            .await
            .expect_err("failed batch must roll back the create");
        assert_eq!(error.status().status_code(), StatusCode::NotFound);
        Ok(())
    })
    .await
}

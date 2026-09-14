// Copyright (c) Microsoft Corporation. All rights reserved.
// Licensed under the MIT License.

use azure_core::http::StatusCode;
use azure_data_cosmos::{
    diagnostics::TransportKind,
    feed::FeedScope,
    options::{ ChangeFeedMode, ChangeFeedOptions, ChangeFeedStartFrom },
};
use futures::StreamExt;
use time::OffsetDateTime;

use crate::e2e_test_cases::{
    fixture::{ E2eTestFixture, TestResult },
    support::{ assert_transport, should_run, Item },
};

#[tokio::test]
#[cfg_attr(
    not(any(test_category = "emulator_inmemory", test_category = "e2e")),
    ignore = "requires the externally hosted in-memory emulator"
)]
async fn all_versions_rejects_unsupported_starts() -> TestResult {
    if !should_run("changefeed.all-versions-start-validation").await? {
        return Ok(());
    }

    E2eTestFixture::run(async |fixture| {
        let options = ChangeFeedOptions::default().with_mode(ChangeFeedMode::AllVersionsAndDeletes);
        let mut available = fixture.container.query_change_feed::<Item>(
            FeedScope::partition("A"),
            ChangeFeedStartFrom::Now,
            Some(options)
        ).await?;
        let page = available
            .next().await
            .expect("all-versions change feed must return an initial page")?;
        assert!(page.items().is_empty());
        assert_transport(page.diagnostics().as_ref(), TransportKind::Gateway);

        for start in [
            ChangeFeedStartFrom::Beginning,
            ChangeFeedStartFrom::PointInTime(OffsetDateTime::now_utc()),
        ] {
            let options = ChangeFeedOptions::default().with_mode(
                ChangeFeedMode::AllVersionsAndDeletes
            );
            let mut changes = fixture.container.query_change_feed::<Item>(
                FeedScope::partition("A"),
                start,
                Some(options)
            ).await?;
            let error = changes
                .next().await
                .expect("change feed must return an error page")
                .expect_err("unsupported all-versions start must fail");
            assert_eq!(error.status().status_code(), StatusCode::BadRequest);
            assert_transport(
                error.diagnostics().expect("change-feed error must include diagnostics").as_ref(),
                TransportKind::Gateway
            );
        }
        Ok(())
    }).await
}

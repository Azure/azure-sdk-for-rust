// Copyright (c) Microsoft Corporation. All rights reserved.
// Licensed under the MIT License.

use std::num::NonZeroU32;

use azure_core::http::StatusCode;
use azure_data_cosmos::{
    feed::{ContinuationToken, FeedScope},
    options::{
        AvailabilityStrategy, ItemReadOptions, MaxItemCountHint, OperationOptions, QueryOptions,
        ReadConsistencyStrategy, Region,
    },
    Query, RoutingStrategy,
};
use futures::StreamExt;

use crate::e2e_test_cases::{
    fixture::{build_client_with_customizer, ClientSetup, E2eTest, TestResult},
    support::{
        item, selected_scenario_profile, wait_for_item_replication, with_replication_paused_if,
        Item,
    },
};

#[tokio::test]
#[cfg_attr(
    not(any(test_category = "emulator_inmemory", test_category = "e2e")),
    ignore = "requires the externally hosted in-memory emulator"
)]
async fn explicit_tokens_work_when_automatic_capture_is_disabled() -> TestResult {
    let Some(profile) = selected_scenario_profile("consistency.session-management").await? else {
        return Ok(());
    };
    let setup = ClientSetup::from_profile(
        profile.selected_runtime()?,
        profile.selected_client()?,
        RoutingStrategy::PreferredRegions(vec![Region::WEST_US, Region::EAST_US]),
    )?;
    let client = build_client_with_customizer(setup, |builder| {
        let mut defaults = OperationOptions::default();
        defaults.session_capturing_disabled = Some(true);
        Ok(builder.with_default_operation_options(defaults))
    })
    .await?;

    E2eTest::builder()
        .with_client(client)
        .run(async |fixture| {
            let expected = item("session-management", "A", 21);
            let created = fixture
                .container
                .create_item("A", &expected.id, &expected, None)
                .await?;
            let token = created
                .headers()
                .session_token()
                .map(|value| value.as_str().to_owned())
                .ok_or("write response must expose an explicit token")?;

            let mut session = OperationOptions::default();
            session.read_consistency_strategy = Some(ReadConsistencyStrategy::Session);
            session.availability_strategy = Some(AvailabilityStrategy::Disabled);
            let explicit = fixture
                .container
                .read_item(
                    "A",
                    &expected.id,
                    Some(
                        ItemReadOptions::default()
                            .with_session_token(token.clone())
                            .with_operation_options(session.clone()),
                    ),
                )
                .await?;
            assert_eq!(explicit.into_model::<Item>()?, expected);

            wait_for_item_replication(&fixture.container, &expected.id, &expected).await?;
            for value in 0..3 {
                let id = format!("session-page-{value}");
                fixture
                    .container
                    .create_item("A", &id, item(&id, "A", value), None)
                    .await?;
            }
            let last_page_item = item("session-page-2", "A", 2);
            wait_for_item_replication(&fixture.container, &last_page_item.id, &last_page_item)
                .await?;

            let page_size = MaxItemCountHint::Limit(NonZeroU32::new(1).unwrap());
            let options = QueryOptions::default()
                .with_session_token(token)
                .with_max_item_count(page_size)
                .with_operation_options(session.clone());
            let mut pages = fixture
                .container
                .query_items::<Item>(
                    Query::from("SELECT * FROM c ORDER BY c.id"),
                    FeedScope::partition("A"),
                    Some(options),
                )
                .await?
                .into_pages();
            let first = pages.next().await.expect("query must yield a first page")?;
            let page_token = first
                .headers()
                .session_token()
                .map(|value| value.as_str().to_owned())
                .ok_or("first query page must expose a session token")?;
            let mut actual = first.into_items();
            let continuation = pages.to_continuation_token()?;
            drop(pages);

            let resumed_options = QueryOptions::default()
                .with_session_token(page_token)
                .with_continuation_token(ContinuationToken::from_string(
                    continuation.as_str().to_owned(),
                ))
                .with_max_item_count(page_size)
                .with_operation_options(session);
            let mut resumed = fixture
                .container
                .query_items::<Item>(
                    Query::from("SELECT * FROM c ORDER BY c.id"),
                    FeedScope::partition("A"),
                    Some(resumed_options),
                )
                .await?
                .into_pages();
            while let Some(page) = resumed.next().await {
                let page = page?;
                assert!(page.headers().session_token().is_some());
                actual.extend(page.into_items());
            }
            let actual_ids: Vec<_> = actual.iter().map(|value| value.id.as_str()).collect();
            assert_eq!(
                actual_ids,
                [
                    "session-management",
                    "session-page-0",
                    "session-page-1",
                    "session-page-2"
                ]
            );
            Ok(())
        })
        .await
}

#[tokio::test]
#[cfg_attr(
    not(any(test_category = "emulator_inmemory", test_category = "e2e")),
    ignore = "requires the externally hosted in-memory emulator"
)]
async fn disabled_capture_exposes_delayed_replica() -> TestResult {
    let Some(profile) = selected_scenario_profile("consistency.session-staleness").await? else {
        return Ok(());
    };
    let setup = ClientSetup::from_profile(
        profile.selected_runtime()?,
        profile.selected_client()?,
        RoutingStrategy::PreferredRegions(vec![Region::WEST_US, Region::EAST_US]),
    )?;
    let client = build_client_with_customizer(setup, |builder| {
        let mut defaults = OperationOptions::default();
        defaults.session_capturing_disabled = Some(true);
        Ok(builder.with_default_operation_options(defaults))
    })
    .await?;

    E2eTest::builder()
        .with_client(client)
        .run(async |fixture| {
            let expected = item("session-staleness", "A", 22);
            let (without_token, with_token) = with_replication_paused_if(true, "West US", async {
                let created = fixture
                    .container
                    .create_item("A", &expected.id, &expected, None)
                    .await?;
                let token = created
                    .headers()
                    .session_token()
                    .map(|value| value.as_str().to_owned())
                    .ok_or("write response must expose an explicit token")?;
                let mut session = OperationOptions::default();
                session.read_consistency_strategy = Some(ReadConsistencyStrategy::Session);
                session.max_session_retry_count = Some(0);
                session.max_failover_retry_count = Some(0);
                session.availability_strategy = Some(AvailabilityStrategy::Disabled);
                let without_token = fixture
                    .container
                    .read_item(
                        "A",
                        &expected.id,
                        Some(ItemReadOptions::default().with_operation_options(session.clone())),
                    )
                    .await;
                let with_token = fixture
                    .container
                    .read_item(
                        "A",
                        &expected.id,
                        Some(
                            ItemReadOptions::default()
                                .with_session_token(token)
                                .with_operation_options(session),
                        ),
                    )
                    .await;
                Ok((without_token, with_token))
            })
            .await?;
            let stale = without_token
                .expect_err("disabled capture must expose the delayed West US replica");
            assert_eq!(stale.status().status_code(), StatusCode::NotFound);
            assert_eq!(
                stale
                    .status()
                    .sub_status()
                    .map(|value| value.value())
                    .unwrap_or(0),
                0
            );
            let unavailable = with_token
                .expect_err("an explicit token must identify the delayed session replica");
            assert_eq!(unavailable.status().status_code(), StatusCode::NotFound);
            assert_eq!(
                unavailable.status().sub_status(),
                Some(azure_data_cosmos::SubStatusCode::READ_SESSION_NOT_AVAILABLE)
            );
            Ok(())
        })
        .await
}

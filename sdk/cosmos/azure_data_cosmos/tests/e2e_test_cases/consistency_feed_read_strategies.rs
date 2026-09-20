// Copyright (c) Microsoft Corporation. All rights reserved.
// Licensed under the MIT License.

use azure_core::http::StatusCode;
use azure_data_cosmos::{
    feed::FeedScope,
    options::{
        AvailabilityStrategy, ChangeFeedOptions, ChangeFeedStartFrom, OperationOptions,
        QueryOptions, ReadConsistencyStrategy, Region,
    },
    Query, RoutingStrategy, SubStatusCode,
};
use futures::StreamExt;

use crate::e2e_test_cases::{
    fixture::{build_client_with_defaults, ClientSetup, E2eTest, TestResult},
    support::{
        hosted_wire_counts, item, selected_scenario_profile, wait_for_item_replication,
        with_replication_paused_if, HostedWireCounts, Item,
    },
};

#[tokio::test]
#[cfg_attr(
    not(any(test_category = "emulator_inmemory", test_category = "e2e")),
    ignore = "requires the externally hosted in-memory emulator"
)]
async fn feeds_honor_account_and_operation_consistency() -> TestResult {
    let Some(profile) = selected_scenario_profile("consistency.feed-read-strategies").await? else {
        return Ok(());
    };
    let account = profile.selected_account()?;
    let runtime = profile.selected_runtime()?;
    let client_definition = profile.selected_client()?;
    let client = build_client_with_defaults(ClientSetup::from_profile(
        runtime,
        client_definition,
        RoutingStrategy::PreferredRegions(vec![Region::WEST_US, Region::EAST_US]),
    )?)
    .await?;
    let is_strong = account.consistency == "strong";

    E2eTest::builder()
        .with_client(client)
        .run(async |fixture| {
            let expected = item("feed-consistency", "A", 11);
            let token = with_replication_paused_if(!is_strong, "West US", async {
                let created = fixture
                    .container
                    .create_item("A", &expected.id, &expected, None)
                    .await?;
                let token = created
                    .headers()
                    .session_token()
                    .map(|value| value.as_str().to_owned())
                    .ok_or("create response must expose a session token")?;

                // No operation override: this is the account-default contract.
                let mut default_query = fixture
                    .container
                    .query_items::<Item>(
                        Query::from("SELECT * FROM c"),
                        FeedScope::partition("A"),
                        None,
                    )
                    .await?
                    .into_pages();
                let default_page = default_query
                    .next()
                    .await
                    .expect("default query must yield a page")?;
                let default_is_session_effective =
                    matches!(account.consistency.as_str(), "strong" | "session");
                if default_is_session_effective {
                    assert_eq!(default_page.items(), std::slice::from_ref(&expected));
                } else {
                    assert!(default_page.items().is_empty());
                }

                let mut default_changes = fixture
                    .container
                    .query_change_feed::<Item>(
                        FeedScope::partition("A"),
                        ChangeFeedStartFrom::Beginning,
                        None,
                    )
                    .await?;
                let default_change_page = default_changes
                    .next()
                    .await
                    .expect("default change feed must yield a page")?;
                if default_is_session_effective {
                    assert_eq!(default_change_page.items().len(), 1);
                } else {
                    assert!(default_change_page.items().is_empty());
                }
                Ok(token)
            })
            .await?;

            wait_for_item_replication(&fixture.container, &expected.id, &expected).await?;

            for strategy in [
                ReadConsistencyStrategy::Default,
                ReadConsistencyStrategy::Eventual,
                ReadConsistencyStrategy::Session,
                ReadConsistencyStrategy::LatestCommitted,
            ] {
                assert_converged_query_strategy(&fixture.container, strategy, &expected, &token)
                    .await?;
                assert_converged_change_feed_strategy(
                    &fixture.container,
                    strategy,
                    &expected,
                    &token,
                )
                .await?;
            }
            assert_invalid_session_tokens_rejected(&fixture.container, &token).await?;

            let mut operation = OperationOptions::default();
            operation.read_consistency_strategy = Some(ReadConsistencyStrategy::GlobalStrong);
            operation.availability_strategy = Some(AvailabilityStrategy::Disabled);
            let query_wire_before = hosted_wire_counts()
                .await?
                .expect("hosted emulator must expose consistency counters");
            let global = fixture
                .container
                .query_items::<Item>(
                    Query::from("SELECT * FROM c"),
                    FeedScope::partition("A"),
                    Some(QueryOptions::default().with_operation_options(operation.clone())),
                )
                .await;
            if is_strong {
                let mut pages = global?.into_pages();
                let page = pages
                    .next()
                    .await
                    .expect("Strong query must yield a page")?;
                assert_eq!(page.items(), std::slice::from_ref(&expected));
                assert_strategy_reached_transport(
                    query_wire_before,
                    hosted_wire_counts()
                        .await?
                        .expect("hosted emulator must expose consistency counters"),
                    ReadConsistencyStrategy::GlobalStrong,
                );
            } else {
                let mut pages = global?.into_pages();
                let error = match pages.next().await {
                    Some(Err(error)) => error,
                    Some(Ok(_)) | None => panic!("GlobalStrong requires a Strong account"),
                };
                assert_eq!(error.status().status_code(), StatusCode::BadRequest);
                assert!(error.response().is_none());
                assert_eq!(
                    error
                        .diagnostics()
                        .expect("validation error must carry diagnostics")
                        .request_count(),
                    0
                );
                assert_eq!(
                    hosted_wire_counts()
                        .await?
                        .expect("hosted emulator must expose consistency counters")
                        .consistency_requests(ReadConsistencyStrategy::GlobalStrong),
                    query_wire_before.consistency_requests(ReadConsistencyStrategy::GlobalStrong),
                    "client-side GlobalStrong validation must not reach transport"
                );
            }

            let change_options = ChangeFeedOptions::default().with_operation_options(operation);
            let change_wire_before = hosted_wire_counts()
                .await?
                .expect("hosted emulator must expose consistency counters");
            let mut changes = fixture
                .container
                .query_change_feed::<Item>(
                    FeedScope::partition("A"),
                    ChangeFeedStartFrom::Beginning,
                    Some(change_options),
                )
                .await?;
            if is_strong {
                let page = changes
                    .next()
                    .await
                    .expect("Strong change feed must yield a page")?;
                assert_eq!(page.items().len(), 1);
                assert_eq!(page.items()[0].current(), Some(&expected));
                assert_strategy_reached_transport(
                    change_wire_before,
                    hosted_wire_counts()
                        .await?
                        .expect("hosted emulator must expose consistency counters"),
                    ReadConsistencyStrategy::GlobalStrong,
                );
            } else {
                let error = match changes.next().await {
                    Some(Err(error)) => error,
                    Some(Ok(_)) | None => panic!("GlobalStrong requires a Strong account"),
                };
                assert_eq!(error.status().status_code(), StatusCode::BadRequest);
                assert!(error.response().is_none());
                assert_eq!(
                    error
                        .diagnostics()
                        .expect("validation error must carry diagnostics")
                        .request_count(),
                    0
                );
                assert_eq!(
                    hosted_wire_counts()
                        .await?
                        .expect("hosted emulator must expose consistency counters")
                        .consistency_requests(ReadConsistencyStrategy::GlobalStrong),
                    change_wire_before.consistency_requests(ReadConsistencyStrategy::GlobalStrong),
                    "client-side GlobalStrong validation must not reach transport"
                );
            }
            Ok(())
        })
        .await
}

async fn assert_converged_query_strategy(
    container: &azure_data_cosmos::clients::ContainerClient,
    strategy: ReadConsistencyStrategy,
    expected: &Item,
    token: &str,
) -> TestResult {
    let wire_before = hosted_wire_counts()
        .await?
        .expect("hosted emulator must expose consistency counters");
    let mut operation = OperationOptions::default();
    operation.read_consistency_strategy = Some(strategy);
    operation.availability_strategy = Some(AvailabilityStrategy::Disabled);
    let mut options = QueryOptions::default().with_operation_options(operation);
    if strategy == ReadConsistencyStrategy::Session {
        options = options.with_session_token(token.to_owned());
    }
    let mut pages = container
        .query_items::<Item>(
            Query::from("SELECT * FROM c"),
            FeedScope::partition("A"),
            Some(options),
        )
        .await?
        .into_pages();
    let page = pages
        .next()
        .await
        .expect("strategy query must yield a page")?;
    assert_eq!(page.items(), std::slice::from_ref(expected));
    assert_strategy_reached_transport(
        wire_before,
        hosted_wire_counts()
            .await?
            .expect("hosted emulator must expose consistency counters"),
        strategy,
    );
    Ok(())
}

async fn assert_converged_change_feed_strategy(
    container: &azure_data_cosmos::clients::ContainerClient,
    strategy: ReadConsistencyStrategy,
    expected: &Item,
    token: &str,
) -> TestResult {
    let wire_before = hosted_wire_counts()
        .await?
        .expect("hosted emulator must expose consistency counters");
    let mut operation = OperationOptions::default();
    operation.read_consistency_strategy = Some(strategy);
    operation.availability_strategy = Some(AvailabilityStrategy::Disabled);
    let mut options = ChangeFeedOptions::default().with_operation_options(operation);
    if strategy == ReadConsistencyStrategy::Session {
        options = options.with_session_token(token.to_owned());
    }
    let mut changes = container
        .query_change_feed::<Item>(
            FeedScope::partition("A"),
            ChangeFeedStartFrom::Beginning,
            Some(options),
        )
        .await?;
    let page = changes
        .next()
        .await
        .expect("strategy change feed must yield a page")?;
    assert_eq!(page.items().len(), 1);
    assert_eq!(page.items()[0].current(), Some(expected));
    if strategy == ReadConsistencyStrategy::Session {
        assert!(page.headers().session_token().is_some());
    }
    assert_strategy_reached_transport(
        wire_before,
        hosted_wire_counts()
            .await?
            .expect("hosted emulator must expose consistency counters"),
        strategy,
    );
    Ok(())
}

fn assert_strategy_reached_transport(
    before: HostedWireCounts,
    after: HostedWireCounts,
    strategy: ReadConsistencyStrategy,
) {
    assert!(
        after.consistency_requests(strategy) > before.consistency_requests(strategy),
        "{strategy:?} must be observed on the hosted emulator wire"
    );
}

async fn assert_invalid_session_tokens_rejected(
    container: &azure_data_cosmos::clients::ContainerClient,
    valid_token: &str,
) -> TestResult {
    let mut operation = OperationOptions::default();
    operation.read_consistency_strategy = Some(ReadConsistencyStrategy::Session);
    operation.availability_strategy = Some(AvailabilityStrategy::Disabled);

    let query_options = QueryOptions::default()
        .with_session_token("not-a-session-token")
        .with_operation_options(operation.clone());
    let mut query = container
        .query_items::<Item>(
            Query::from("SELECT * FROM c"),
            FeedScope::partition("A"),
            Some(query_options),
        )
        .await?
        .into_pages();
    let query_error = query
        .next()
        .await
        .expect("malformed-token query must yield an error")
        .expect_err("malformed query session token must be rejected");
    assert_eq!(query_error.status().status_code(), StatusCode::BadRequest);

    let change_options = ChangeFeedOptions::default()
        .with_session_token("not-a-session-token")
        .with_operation_options(operation);
    let mut changes = container
        .query_change_feed::<Item>(
            FeedScope::partition("A"),
            ChangeFeedStartFrom::Beginning,
            Some(change_options),
        )
        .await?;
    let change_error = changes
        .next()
        .await
        .expect("malformed-token change feed must yield an error")
        .expect_err("malformed change-feed session token must be rejected");
    assert_eq!(change_error.status().status_code(), StatusCode::BadRequest);

    let partition_id = valid_token
        .split(':')
        .next()
        .ok_or("session token must contain a partition-range ID")?;
    let future_token = format!("{partition_id}:-1#999999");
    let mut no_retry_operation = OperationOptions::default();
    no_retry_operation.read_consistency_strategy = Some(ReadConsistencyStrategy::Session);
    no_retry_operation.availability_strategy = Some(AvailabilityStrategy::Disabled);
    no_retry_operation.max_session_retry_count = Some(0);
    no_retry_operation.max_failover_retry_count = Some(0);

    let query_options = QueryOptions::default()
        .with_session_token(future_token.clone())
        .with_operation_options(no_retry_operation.clone());
    let mut query = container
        .query_items::<Item>(
            Query::from("SELECT * FROM c"),
            FeedScope::partition("A"),
            Some(query_options),
        )
        .await?
        .into_pages();
    let query_error = query
        .next()
        .await
        .expect("future-token query must yield an error")
        .expect_err("future query session token must be rejected");
    assert_eq!(query_error.status().status_code(), StatusCode::NotFound);
    assert_eq!(
        query_error.status().sub_status(),
        Some(SubStatusCode::READ_SESSION_NOT_AVAILABLE)
    );

    let change_options = ChangeFeedOptions::default()
        .with_session_token(future_token)
        .with_operation_options(no_retry_operation);
    let mut changes = container
        .query_change_feed::<Item>(
            FeedScope::partition("A"),
            ChangeFeedStartFrom::Beginning,
            Some(change_options),
        )
        .await?;
    let change_error = changes
        .next()
        .await
        .expect("future-token change feed must yield an error")
        .expect_err("future change-feed session token must be rejected");
    assert_eq!(change_error.status().status_code(), StatusCode::NotFound);
    assert_eq!(
        change_error.status().sub_status(),
        Some(SubStatusCode::READ_SESSION_NOT_AVAILABLE)
    );
    Ok(())
}

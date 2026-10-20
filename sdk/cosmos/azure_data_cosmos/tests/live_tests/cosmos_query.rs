// Copyright (c) Microsoft Corporation. All rights reserved.
// Licensed under the MIT License.

use crate::framework::{self, test_client::TEST_MODE_ENV_VAR, test_data, TestClient, TestOptions};
use azure_data_cosmos::{
    feed::FeedScope,
    options::{
        BinaryEncodingOptions, MaxItemCountHint, OperationOptions, QueryOptions, QueryPlanMode,
        Region,
    },
    AccountReference, CosmosClient, Query, RoutingStrategy,
};
use futures::StreamExt;
use std::error::Error;

#[tokio::test]
#[cfg_attr(not(test_category = "live"), ignore = "requires live account")]
async fn live_distinct_admission_and_per_query_options() -> Result<(), Box<dyn Error>> {
    let connection = framework::resolve_connection_string()
        .ok_or("live tests require a valid AZURE_COSMOS_CONNECTION_STRING")?;
    assert!(
        !framework::targets_emulator(),
        "live DISTINCT admission coverage requires a live account, not an emulator"
    );
    assert!(
        !std::env::var(TEST_MODE_ENV_VAR).is_ok_and(|mode| mode.eq_ignore_ascii_case("skipped")),
        "explicitly selected live tests cannot use AZURE_COSMOS_TEST_MODE=skipped"
    );
    TestClient::run_with_unique_db(
        async |run_context, db_client| {
            println!("Live DISTINCT test database: {}", run_context.db_name());
            test_data::create_container_with_items(
                db_client,
                test_data::generate_mock_items(4, 3),
                None,
            )
            .await?;
            let account = AccountReference::with_authentication_key(
                connection.account_endpoint().parse()?,
                connection.account_key().clone(),
            );
            let expected: Vec<String> = (0..4).map(|i| format!("partition{i}")).collect();
            let unbounded = "SELECT DISTINCT VALUE c.partitionKey FROM c";
            for mode in [QueryPlanMode::LocalPreferred, QueryPlanMode::GatewayOnly] {
                let client = CosmosClient::builder()
                    .build(
                        account.clone(),
                        RoutingStrategy::ProximityTo(Region::EAST_US),
                    )
                    .await?;
                let container = client
                    .database_client(run_context.db_name())
                    .container_client("TestContainer", None)
                    .await?;
                for binary in [false, true] {
                    let mut operation = OperationOptions::default();
                    operation.binary_encoding =
                        Some(BinaryEncodingOptions::new().with_enabled(binary));
                    let options = QueryOptions::default()
                        .with_query_plan_mode(mode)
                        .with_operation_options(operation)
                        .with_max_item_count(MaxItemCountHint::Limit(
                            std::num::NonZeroU32::new(1).unwrap(),
                        ));
                    for (query, maximum, denied) in [
                        (unbounded, None, true),
                        (unbounded, Some(u64::MAX), true),
                        (
                            "SELECT DISTINCT TOP 1001 VALUE c.partitionKey FROM c",
                            None,
                            true,
                        ),
                        (
                            "SELECT DISTINCT TOP 1000 VALUE c.partitionKey FROM c",
                            None,
                            false,
                        ),
                        (
                            "SELECT DISTINCT TOP 1001 VALUE c.partitionKey FROM c",
                            Some(1001),
                            false,
                        ),
                        (
                            "SELECT DISTINCT TOP 4 VALUE c.partitionKey FROM c",
                            Some(3),
                            true,
                        ),
                        (
                            "SELECT DISTINCT TOP 4 VALUE c.partitionKey FROM c",
                            Some(4),
                            false,
                        ),
                    ] {
                        let mut options = options.clone();
                        if let Some(maximum) = maximum {
                            options = options.with_max_buffered_query_window(maximum);
                        }
                        let result = container
                            .query_items::<String>(
                                query,
                                FeedScope::full_container(),
                                Some(options),
                            )
                            .await;
                        if denied {
                            let error = match result {
                                Err(error) => error,
                                Ok(_) => {
                                    panic!("DISTINCT outside finite window must fail at admission")
                                }
                            };
                            assert_eq!(
                                error.status(),
                                azure_data_cosmos_driver::error::status_codes::CLIENT_BUFFERED_QUERY_REQUIRES_FINITE_WINDOW,
                                "{mode:?}, maximum={maximum:?}, binary={binary}"
                            );
                        } else {
                            let mut pages = result?.into_pages();
                            assert_eq!(
                                pages.to_continuation_token().unwrap_err().status(),
                                azure_data_cosmos_driver::error::status_codes::CLIENT_DISTINCT_CONTINUATION_UNSUPPORTED
                            );
                            let mut actual = Vec::new();
                            while let Some(page) = pages.next().await {
                                actual.extend(page?.into_items());
                            }
                            actual.sort();
                            assert_eq!(actual, expected);
                        }
                    }
                    for query in [
                        "SELECT DISTINCT TOP @bound VALUE c.partitionKey FROM c",
                        "SELECT DISTINCT VALUE c.partitionKey FROM c OFFSET 1 LIMIT @bound",
                    ] {
                        let mut pages = container
                            .query_items::<String>(
                                Query::from(query).with_parameter("@bound", 2)?,
                                FeedScope::full_container(),
                                Some(options.clone().with_max_buffered_query_window(3)),
                            )
                            .await?
                            .into_pages();
                        let mut actual = Vec::new();
                        while let Some(page) = pages.next().await {
                            actual.extend(page?.into_items());
                        }
                        assert_eq!(actual.len(), 2);
                        actual.sort();
                        actual.dedup();
                        assert_eq!(actual.len(), 2);
                        assert!(actual.iter().all(|value| expected.contains(value)));
                    }
                }
            }
            Ok(())
        },
        Some(TestOptions::default()),
    )
    .await
}

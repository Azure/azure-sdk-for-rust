// Copyright (c) Microsoft Corporation. All rights reserved.
// Licensed under the MIT License.

use super::{
    cosmos_query_split::force_split_and_wait,
    framework::{MockItem, TestClient, TestOptions},
};
use azure_core::http::StatusCode;
use azure_data_cosmos::{
    diagnostics::TransportKind,
    models::{ContainerProperties, ItemResponse, ThroughputProperties},
    options::{
        AvailabilityStrategy, CreateContainerOptions, ItemReadOptions, ItemWriteOptions,
        OperationOptions, ReadConsistencyStrategy, ReadFeedRangesOptions,
    },
};
use futures::FutureExt;
use std::{collections::BTreeSet, error::Error, panic::AssertUnwindSafe, time::Duration};

fn response_range(response: &ItemResponse) -> Result<String, Box<dyn Error>> {
    let diagnostics = response.diagnostics();
    let requests = diagnostics.requests();
    if requests.is_empty()
        || requests.iter().any(|request| {
            request.transport_kind() != TransportKind::Gateway
                || request.status().status_code() == StatusCode::from(0)
        })
    {
        return Err("expected completed classic Gateway requests".into());
    }
    let token = response
        .headers()
        .session_token()
        .ok_or("point response has no session token")?
        .as_str();
    if token.contains(',') {
        return Err("expected a single-range point response token".into());
    }
    let (range, _) = token
        .split_once(':')
        .filter(|(range, value)| !range.is_empty() && !value.is_empty())
        .ok_or("point response token has no range identity")?;
    Ok(range.to_owned())
}

#[tokio::test]
#[cfg_attr(
    not(test_category = "split"),
    ignore = "requires test_category 'split'"
)]
async fn classic_gateway_session_reads_across_service_split() -> Result<(), Box<dyn Error>> {
    TestClient::run_with_options(
        async |run| {
            println!("Session split test database: {}", run.db_name());
            AssertUnwindSafe(async {
                let database = run.create_db().await?;
                let container = run
                    .create_container(
                        &database,
                        ContainerProperties::new("SessionSplit", "/partitionKey".into()),
                        Some(
                            CreateContainerOptions::default()
                                .with_throughput(ThroughputProperties::manual(1000)),
                        ),
                    )
                    .await?;
                let before = container.read_feed_ranges(None).await?;
                if before.len() != 1 {
                    return Err("session split test requires one initial physical partition".into());
                }
                let mut session = OperationOptions::default();
                session.read_consistency_strategy = Some(ReadConsistencyStrategy::Session);
                session.availability_strategy = Some(AvailabilityStrategy::Disabled);
                session.session_capturing_disabled = Some(false);
                let write_options =
                    ItemWriteOptions::default().with_operation_options(session.clone());
                let mut items = Vec::new();
                let mut parent_ids = BTreeSet::new();
                for index in 0..32 {
                    let item = MockItem {
                        id: format!("session-{index}"),
                        partition_key: format!("pk-{index}"),
                        merge_order: 0,
                    };
                    let response = container
                        .create_item(
                            &item.partition_key,
                            &item.id,
                            &item,
                            Some(write_options.clone()),
                        )
                        .await?;
                    parent_ids.insert(response_range(&response)?);
                    items.push(item);
                }
                if parent_ids.len() != 1 {
                    return Err("seed writes did not establish a single parent session".into());
                }

                // Poll topology through another runtime, without refreshing the reader's map.
                let management = run.fresh_management_client().await?;
                let management_container = management
                    .database_client(database.id())
                    .container_client("SessionSplit", None)
                    .await?;
                let after = force_split_and_wait(&management_container, before.len()).await?;
                let read_options =
                    ItemReadOptions::default().with_operation_options(session.clone());
                let mut child_ids = BTreeSet::new();

                for item in &mut items {
                    item.merge_order = 1;
                    let written = container
                        .replace_item(
                            &item.partition_key,
                            &item.id,
                            &*item,
                            Some(write_options.clone()),
                        )
                        .await?;
                    child_ids.insert(response_range(&written)?);
                    let explicit = written
                        .headers()
                        .session_token()
                        .ok_or("child write has no session token")?
                        .clone();
                    let automatic = container
                        .read_item(&item.partition_key, &item.id, Some(read_options.clone()))
                        .await?;
                    response_range(&automatic)?;
                    if automatic.into_model::<MockItem>()? != *item {
                        return Err("automatic session read lost a post-split write".into());
                    }
                    let mut disabled = session.clone();
                    disabled.session_capturing_disabled = Some(true);
                    let supplied = container
                        .read_item(
                            &item.partition_key,
                            &item.id,
                            Some(
                                ItemReadOptions::default()
                                    .with_operation_options(disabled)
                                    .with_session_token(explicit),
                            ),
                        )
                        .await?;
                    response_range(&supplied)?;
                    if supplied.into_model::<MockItem>()? != *item {
                        return Err("explicit session read lost a post-split write".into());
                    }
                }
                if child_ids.len() != after || !child_ids.is_disjoint(&parent_ids) {
                    return Err("post-split writes did not cover every replacement range".into());
                }

                let refreshed = container
                    .read_feed_ranges(Some(
                        ReadFeedRangesOptions::default().with_force_refresh(true),
                    ))
                    .await?;
                if refreshed.len() != after {
                    return Err("reader topology did not converge after explicit refresh".into());
                }
                for item in &items {
                    let response = container
                        .read_item(&item.partition_key, &item.id, Some(read_options.clone()))
                        .await?;
                    response_range(&response)?;
                    if response.into_model::<MockItem>()? != *item {
                        return Err("refreshed session read lost a post-split write".into());
                    }
                }
                println!(
                    "Session split validated: {} parent, {} children, {} items",
                    parent_ids.len(),
                    child_ids.len(),
                    items.len()
                );
                Ok(())
            })
            .catch_unwind()
            .await
            .unwrap_or_else(|_| Err("session split test panicked; running database cleanup".into()))
        },
        TestOptions::new()
            .with_gateway_v2_disabled(true)
            .with_timeout(Duration::from_secs(20 * 60)),
    )
    .await
}

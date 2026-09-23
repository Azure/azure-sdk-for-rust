// Copyright (c) Microsoft Corporation. All rights reserved.
// Licensed under the MIT License.

use azure_core::http::StatusCode;
use azure_data_cosmos::{
    feed::FeedScope,
    options::{
        AvailabilityStrategy, ExcludedRegions, ItemReadOptions, OperationOptions, QueryOptions,
        ReadConsistencyStrategy, Region,
    },
    Query, RoutingStrategy,
};
use futures::StreamExt;

use crate::e2e_test_cases::{
    fixture::{build_client_with_defaults, ClientSetup, E2eTest, TestResult},
    support::{item, selected_scenario_profile, wait_for_item_replication_in_region, Item},
    topology_support::{with_topology_reset, TopologyManager},
};

#[tokio::test]
#[cfg_attr(
    not(any(test_category = "emulator_inmemory", test_category = "e2e")),
    ignore = "requires the externally hosted in-memory emulator"
)]
async fn paused_replica_converges_without_loss_or_duplication() -> TestResult {
    let Some(profile) = selected_scenario_profile("topology.replication-pause-resume").await?
    else {
        return Ok(());
    };
    let client = build_client_with_defaults(ClientSetup::from_profile(
        profile.selected_runtime()?,
        profile.selected_client()?,
        RoutingStrategy::PreferredRegions(vec![
            Region::new("East US"),
            Region::new("West US"),
            Region::new("North Europe"),
        ]),
    )?)
    .await?;
    let manager = TopologyManager::from_env()?;

    E2eTest::builder()
        .with_client(client)
        .run(async |fixture| {
            with_topology_reset(&manager, async {
                manager.pause_replication("West US").await?;
                let expected: Vec<_> = (0..8)
                    .map(|value| item(&format!("replication-{value}"), "A", value))
                    .collect();
                for value in &expected {
                    fixture
                        .container
                        .create_item("A", &value.id, value, None)
                        .await?;
                }

                let operation = west_only_operation();
                let stale = fixture
                    .container
                    .read_item(
                        "A",
                        &expected[0].id,
                        Some(ItemReadOptions::default().with_operation_options(operation.clone())),
                    )
                    .await
                    .expect_err("paused West US must remain stale");
                assert_eq!(stale.status().status_code(), StatusCode::NotFound);

                manager.resume_replication("West US").await?;
                wait_for_item_replication_in_region(
                    &fixture.container,
                    "A",
                    &expected[7].id,
                    &expected[7],
                    Region::new("West US"),
                    [
                        Region::new("East US"),
                        Region::new("West US"),
                        Region::new("North Europe"),
                    ],
                )
                .await?;

                let mut pages = fixture
                    .container
                    .query_items::<Item>(
                        Query::from("SELECT * FROM c ORDER BY c.value"),
                        FeedScope::partition("A"),
                        Some(QueryOptions::default().with_operation_options(operation)),
                    )
                    .await?
                    .into_pages();
                let mut actual = Vec::new();
                while let Some(page) = pages.next().await {
                    actual.extend(page?.into_items());
                }
                assert_eq!(actual, expected);
                Ok(())
            })
            .await
        })
        .await
}

fn west_only_operation() -> OperationOptions {
    let mut operation = OperationOptions::default();
    operation.read_consistency_strategy = Some(ReadConsistencyStrategy::Eventual);
    operation.availability_strategy = Some(AvailabilityStrategy::Disabled);
    operation.max_failover_retry_count = Some(0);
    operation.max_session_retry_count = Some(0);
    operation.excluded_regions = Some(
        ExcludedRegions::new()
            .with_region(Region::new("East US"))
            .with_region(Region::new("North Europe")),
    );
    operation
}

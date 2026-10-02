// Copyright (c) Microsoft Corporation. All rights reserved.
// Licensed under the MIT License.

use azure_core::http::StatusCode;
use azure_data_cosmos::{options::Region, RoutingStrategy, SubStatusCode};

use crate::e2e_test_cases::{
    fixture::{build_client_with_defaults, ClientSetup, E2eTest, TestResult},
    support::{item, selected_scenario_profile, Item},
    topology_support::{with_manual_operation, TopologyManager},
};

#[tokio::test]
#[cfg_attr(
    not(any(test_category = "emulator_inmemory", test_category = "e2e")),
    ignore = "requires the externally hosted in-memory emulator"
)]
async fn manual_split_and_merge_preserve_point_state() -> TestResult {
    let Some(profile) = selected_scenario_profile("topology.partition-split-merge").await? else {
        return Ok(());
    };
    let client = topology_client(&profile).await?;
    let manager = TopologyManager::from_env(profile.selected_account()?)?;

    E2eTest::builder()
        .with_client(client)
        .run(async |fixture| {
            let probe = seed_partition_zero(fixture).await?;
            let initial_ranges = fixture.container.read_feed_ranges(None).await?.len();

            let split = manager
                .start_split(&fixture.database_id, &fixture.container_id, 0)
                .await?;
            let split = with_manual_operation(&manager, &split.operation_id, async {
                assert_eq!(
                    manager.advance(&split.operation_id).await?.phase,
                    "Swapping"
                );
                let split_error = fixture
                    .container
                    .read_item(&probe.pk, &probe.id, None)
                    .await
                    .expect_err("split lock must be observable");
                assert_transition_error(&split_error);
                manager.advance(&split.operation_id).await?;
                manager.wait_for_succeeded(&split.operation_id).await
            })
            .await?;
            assert_eq!(split.children.len(), 2);
            let split_response = fixture
                .container
                .read_item(&probe.pk, &probe.id, None)
                .await?;
            assert_eq!(split_response.into_model::<Item>()?, probe);
            let split_client = topology_client(&profile).await?;
            let split_container = split_client
                .database_client(&fixture.database_id)
                .container_client(&fixture.container_id, None)
                .await?;
            assert_eq!(
                split_container.read_feed_ranges(None).await?.len(),
                initial_ranges + 1
            );

            let merge = manager
                .start_merge(
                    &fixture.database_id,
                    &fixture.container_id,
                    [split.children[0], split.children[1]],
                )
                .await?;
            let merge = with_manual_operation(&manager, &merge.operation_id, async {
                assert_eq!(
                    manager.advance(&merge.operation_id).await?.phase,
                    "Swapping"
                );
                let merge_error = fixture
                    .container
                    .read_item(&probe.pk, &probe.id, None)
                    .await
                    .expect_err("merge lock must be observable");
                assert_transition_error(&merge_error);
                manager.advance(&merge.operation_id).await?;
                manager.wait_for_succeeded(&merge.operation_id).await
            })
            .await?;
            assert!(merge.into.is_some());
            let merge_response = fixture
                .container
                .read_item(&probe.pk, &probe.id, None)
                .await?;
            assert_eq!(merge_response.into_model::<Item>()?, probe);
            let merge_client = topology_client(&profile).await?;
            let merge_container = merge_client
                .database_client(&fixture.database_id)
                .container_client(&fixture.container_id, None)
                .await?;
            assert_eq!(
                merge_container.read_feed_ranges(None).await?.len(),
                initial_ranges
            );
            Ok(())
        })
        .await
}

async fn topology_client(
    profile: &crate::e2e_test_cases::catalog::Profile,
) -> TestResult<azure_data_cosmos::CosmosClient> {
    build_client_with_defaults(ClientSetup::from_profile(
        profile.selected_runtime()?,
        profile.selected_client()?,
        RoutingStrategy::PreferredRegions(vec![
            Region::new("East US"),
            Region::new("West US"),
            Region::new("North Europe"),
        ]),
    )?)
    .await
}

async fn seed_partition_zero(
    fixture: &crate::e2e_test_cases::fixture::E2eTestFixture,
) -> TestResult<Item> {
    for value in 0..512 {
        let candidate = item(
            &format!("partition-zero-{value}"),
            &format!("pk-{value}"),
            value,
        );
        let response = fixture
            .container
            .create_item(&candidate.pk, &candidate.id, &candidate, None)
            .await?;
        if response.headers().partition_key_range_id() == Some("0") {
            return Ok(candidate);
        }
    }
    Err("could not seed an item in physical partition 0".into())
}

fn assert_transition_error(error: &azure_data_cosmos::CosmosError) {
    assert_eq!(error.status().status_code(), StatusCode::Gone);
    assert_eq!(
        error.status().sub_status(),
        Some(SubStatusCode::COMPLETING_SPLIT)
    );
    let diagnostics = error
        .diagnostics()
        .expect("transition error must retain diagnostics");
    assert!(diagnostics.requests().iter().all(|request| {
        request.status().status_code() == StatusCode::Gone
            && request.status().sub_status() == Some(SubStatusCode::COMPLETING_SPLIT)
    }));
}

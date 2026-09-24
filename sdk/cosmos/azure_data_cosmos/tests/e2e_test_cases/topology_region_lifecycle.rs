// Copyright (c) Microsoft Corporation. All rights reserved.
// Licensed under the MIT License.

use std::time::Duration;

use azure_core::http::StatusCode;
use azure_data_cosmos::{
    options::{ItemReadOptions, OperationOptions, Region},
    RoutingStrategy, SubStatusCode,
};

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
async fn preferred_region_recovers_across_offline_remove_and_readd() -> TestResult {
    let Some(profile) = selected_scenario_profile("topology.region-lifecycle").await? else {
        return Ok(());
    };
    let account = profile.selected_account()?;
    let runtime = profile.selected_runtime()?;
    let client_definition = profile.selected_client()?;
    let regions: Vec<_> = account
        .region_names()
        .map(|name| Region::new(name.to_owned()))
        .collect();
    let preferred = vec![
        Region::new("North Europe"),
        Region::new("West US"),
        Region::new("East US"),
    ];
    let client = build_client_with_defaults(ClientSetup::from_profile(
        runtime,
        client_definition,
        RoutingStrategy::PreferredRegions(preferred),
    )?)
    .await?;
    let manager = TopologyManager::from_env()?;

    E2eTest::builder()
        .with_client(client)
        .run(async |fixture| {
            with_topology_reset(&manager, async {
                let expected = item("region-lifecycle", "A", 61);
                fixture
                    .container
                    .create_item("A", &expected.id, &expected, None)
                    .await?;
                wait_for_item_replication_in_region(
                    &fixture.container,
                    "A",
                    &expected.id,
                    &expected,
                    Region::new("North Europe"),
                    regions.clone(),
                )
                .await?;

                manager.offline("North Europe").await?;
                let mut offline_operation = OperationOptions::default();
                offline_operation.endpoint_unavailability_ttl = Some(Duration::from_millis(100));
                let offline = fixture
                    .container
                    .read_item(
                        "A",
                        &expected.id,
                        Some(
                            ItemReadOptions::default()
                                .with_operation_options(offline_operation.clone()),
                        ),
                    )
                    .await?;
                let offline_diagnostics = offline.diagnostics();
                assert_eq!(offline.into_model::<Item>()?, expected);
                assert!(offline_diagnostics
                    .regions_contacted()
                    .iter()
                    .any(|region| region != &Region::new("North Europe")));

                manager.online("North Europe").await?;
                read_until_region(
                    &fixture.container,
                    &expected,
                    Region::new("North Europe"),
                    offline_operation.clone(),
                )
                .await?;

                manager.begin_removal("North Europe").await?;
                manager.remove("North Europe").await?;
                let removed = fixture
                    .container
                    .read_item(
                        "A",
                        &expected.id,
                        Some(
                            ItemReadOptions::default()
                                .with_operation_options(offline_operation.clone()),
                        ),
                    )
                    .await?;
                let removed_diagnostics = removed.diagnostics();
                assert_eq!(removed.into_model::<Item>()?, expected);
                assert!(removed_diagnostics.requests().iter().any(|request| {
                    request.status().status_code() == StatusCode::Forbidden
                        && request.status().sub_status()
                            == Some(SubStatusCode::DATABASE_ACCOUNT_NOT_FOUND)
                }));

                manager.add("North Europe").await?;
                let account = manager.account().await?;
                assert!(account
                    .regions
                    .iter()
                    .any(|region| region.name == "North Europe"));
                read_until_region(
                    &fixture.container,
                    &expected,
                    Region::new("North Europe"),
                    offline_operation,
                )
                .await?;
                Ok(())
            })
            .await
        })
        .await
}

async fn read_until_region(
    container: &azure_data_cosmos::clients::ContainerClient,
    expected: &Item,
    region: Region,
    operation: OperationOptions,
) -> TestResult {
    let deadline = tokio::time::Instant::now() + Duration::from_secs(6);
    loop {
        let response = container
            .read_item(
                "A",
                &expected.id,
                Some(ItemReadOptions::default().with_operation_options(operation.clone())),
            )
            .await?;
        let diagnostics = response.diagnostics();
        assert_eq!(response.into_model::<Item>()?, *expected);
        if diagnostics.request_count() == 1 && diagnostics.requests()[0].region() == Some(&region) {
            return Ok(());
        }
        if tokio::time::Instant::now() >= deadline {
            return Err(format!("running client did not recover preferred region {region}").into());
        }
        tokio::time::sleep(Duration::from_millis(100)).await;
    }
}

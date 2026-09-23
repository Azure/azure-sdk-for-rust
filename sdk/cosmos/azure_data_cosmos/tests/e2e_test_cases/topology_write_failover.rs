// Copyright (c) Microsoft Corporation. All rights reserved.
// Licensed under the MIT License.

use azure_core::http::StatusCode;
use azure_data_cosmos::{options::Region, RoutingStrategy, SubStatusCode};

use crate::e2e_test_cases::{
    fixture::{build_client_with_defaults, ClientSetup, E2eTest, TestResult},
    support::{item, selected_scenario_profile},
    topology_support::{with_topology_reset, TopologyManager},
};

#[tokio::test]
#[cfg_attr(
    not(any(test_category = "emulator_inmemory", test_category = "e2e")),
    ignore = "requires the externally hosted in-memory emulator"
)]
async fn writes_follow_phased_failover_and_failback() -> TestResult {
    let Some(profile) = selected_scenario_profile("topology.write-failover").await? else {
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
                manager.announce_failover("West US").await?;
                let announced = manager.account().await?;
                assert_eq!(announced.write_region, "East US");
                assert!(announced
                    .regions
                    .iter()
                    .filter(|region| region.writable)
                    .any(|region| region.name == "West US"));
                let announced_item = item("write-during-announcement", "A", 70);
                let announced_write = fixture
                    .container
                    .create_item("A", &announced_item.id, &announced_item, None)
                    .await?;
                assert_eq!(
                    announced_write
                        .diagnostics()
                        .requests()
                        .last()
                        .and_then(|request| request.region()),
                    Some(&Region::new("East US")),
                    "announcement must not move write ownership"
                );

                manager.begin_failover("West US").await?;
                let failover_item = item("write-after-failover", "A", 71);
                let failover = fixture
                    .container
                    .create_item("A", &failover_item.id, &failover_item, None)
                    .await?;
                assert_eq!(failover.status().status_code(), StatusCode::Created);
                assert!(failover.diagnostics().requests().iter().any(|request| {
                    request.region() == Some(&Region::new("East US"))
                        && request.status().status_code() == StatusCode::Forbidden
                        && request.status().sub_status() == Some(SubStatusCode::WRITE_FORBIDDEN)
                }));
                assert_eq!(
                    failover
                        .diagnostics()
                        .requests()
                        .last()
                        .and_then(|request| request.region()),
                    Some(&Region::new("West US"))
                );
                manager.complete_failover().await?;
                let completed = manager.account().await?;
                assert_eq!(completed.write_region, "West US");
                assert_eq!(
                    completed
                        .regions
                        .iter()
                        .filter(|region| region.writable)
                        .map(|region| region.name.as_str())
                        .collect::<Vec<_>>(),
                    ["West US"]
                );
                create_until_direct_region(
                    &fixture.container,
                    "write-after-failover-complete",
                    73,
                    Region::new("West US"),
                )
                .await?;

                manager.announce_failover("East US").await?;
                let failback_announced_item = item("write-during-failback-announcement", "A", 74);
                let failback_announced = fixture
                    .container
                    .create_item(
                        "A",
                        &failback_announced_item.id,
                        &failback_announced_item,
                        None,
                    )
                    .await?;
                assert_eq!(
                    failback_announced
                        .diagnostics()
                        .requests()
                        .last()
                        .and_then(|request| request.region()),
                    Some(&Region::new("West US")),
                    "failback announcement must retain current write ownership"
                );
                manager.begin_failover("East US").await?;
                let failback_item = item("write-after-failback", "A", 72);
                let failback = fixture
                    .container
                    .create_item("A", &failback_item.id, &failback_item, None)
                    .await?;
                assert_eq!(failback.status().status_code(), StatusCode::Created);
                assert!(failback.diagnostics().requests().iter().any(|request| {
                    request.region() == Some(&Region::new("West US"))
                        && request.status().status_code() == StatusCode::Forbidden
                        && request.status().sub_status() == Some(SubStatusCode::WRITE_FORBIDDEN)
                }));
                assert_eq!(
                    failback
                        .diagnostics()
                        .requests()
                        .last()
                        .and_then(|request| request.region()),
                    Some(&Region::new("East US"))
                );
                manager.complete_failover().await?;
                assert_eq!(manager.account().await?.write_region, "East US");
                create_until_direct_region(
                    &fixture.container,
                    "write-after-failback-complete",
                    75,
                    Region::new("East US"),
                )
                .await?;
                Ok(())
            })
            .await
        })
        .await
}

async fn create_until_direct_region(
    container: &azure_data_cosmos::clients::ContainerClient,
    id_prefix: &str,
    value: i64,
    expected_region: Region,
) -> TestResult {
    let deadline = tokio::time::Instant::now() + std::time::Duration::from_secs(6);
    for attempt in 0..32 {
        let document = item(&format!("{id_prefix}-{attempt}"), "A", value);
        let response = container
            .create_item("A", &document.id, &document, None)
            .await?;
        let diagnostics = response.diagnostics();
        if diagnostics.request_count() == 1
            && diagnostics.requests()[0].region() == Some(&expected_region)
        {
            return Ok(());
        }
        if tokio::time::Instant::now() >= deadline {
            break;
        }
        tokio::time::sleep(std::time::Duration::from_millis(100)).await;
    }
    Err(format!("client did not converge to direct writes in {expected_region}").into())
}

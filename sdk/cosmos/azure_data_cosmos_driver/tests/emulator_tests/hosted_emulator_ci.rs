// Copyright (c) Microsoft Corporation. All rights reserved.
// Licensed under the MIT License.

//! CI contract for the out-of-process in-memory emulator host.

use serde::Deserialize;

use crate::framework::DriverTestClient;

#[derive(Debug, Deserialize)]
#[serde(rename_all = "camelCase")]
struct HostHealth {
    gateway20_enabled: bool,
    connectivity_probes: usize,
    gateway20_requests: usize,
}

#[tokio::test]
#[cfg_attr(
    not(test_category = "emulator_inmemory"),
    ignore = "requires test_category 'emulator_inmemory'"
)]
async fn configured_host_mode_is_exercised() -> Result<(), Box<dyn std::error::Error>> {
    DriverTestClient::run_with_unique_db(async |context, database| {
        let container_name = context.unique_container_name();
        let container = context
            .create_container(&database, &container_name, "/pk")
            .await?;
        let body = br#"{"id":"host-mode-item","pk":"pk1","value":42}"#;
        context
            .create_item(&container, "host-mode-item", "pk1", body)
            .await?;
        context
            .read_item(&container, "host-mode-item", "pk1")
            .await?;
        Ok(())
    })
    .await?;

    let management_endpoint = std::env::var("AZURE_COSMOS_INMEMORY_MANAGEMENT_ENDPOINT")?;
    let health_endpoint = url::Url::parse(&management_endpoint)?.join("health")?;
    let health = reqwest::Client::new()
        .get(health_endpoint)
        .send()
        .await?
        .error_for_status()?
        .bytes()
        .await?;
    let health: HostHealth = serde_json::from_slice(&health)?;
    match std::env::var("AZURE_COSMOS_EMULATOR_FLAVOR").as_deref() {
        Ok("inmemory-v1") => assert!(!health.gateway20_enabled),
        Ok("inmemory-v2") => {
            assert!(health.gateway20_enabled);
            assert!(health.connectivity_probes > 0);
            assert!(health.gateway20_requests > 0);
        }
        flavor => panic!("unexpected hosted emulator flavor: {flavor:?}"),
    }
    Ok(())
}

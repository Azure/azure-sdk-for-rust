// Copyright (c) Microsoft Corporation. All rights reserved.
// Licensed under the MIT License.

// Use the shared test framework declared in `tests/emulator/mod.rs`.
use super::framework;

use std::error::Error;

use azure_data_cosmos::models::{ContainerProperties, ThroughputProperties};
use azure_data_cosmos::options::CreateContainerOptions;
use framework::{TestClient, TestOptions};

#[tokio::test]
#[cfg_attr(
    not(any(test_category = "emulator", test_category = "emulator_vnext")),
    ignore = "requires test_category 'emulator' or 'emulator_vnext'"
)]
#[cfg_attr(
    test_category = "emulator_vnext",
    ignore = "skipped on vnext emulator: behavioral divergence"
)]
pub async fn container_throughput_crud_manual() -> Result<(), Box<dyn Error>> {
    TestClient::run_with_unique_db(
        async |run_context, db_client| {
            let properties = ContainerProperties::new("TheContainer", "/id".into());

            let throughput = ThroughputProperties::manual(400);

            let _container_client = run_context
                .create_container(
                    db_client,
                    properties.clone(),
                    Some(CreateContainerOptions::default().with_throughput(throughput)),
                )
                .await?;

            // Throughput/offer operations are control-plane and are not covered
            // by the data-plane RBAC role used on the AAD live leg, so route them
            // through the management (key) client. In key mode this is the same
            // credential as `container_client`.
            let offer_client = run_context
                .management_container_client(db_client, "TheContainer")
                .await?;

            // Read throughput
            let current_throughput = offer_client
                .read_throughput(None)
                .await?
                .expect("throughput should be present");

            assert_eq!(Some(400), current_throughput.throughput());

            // Replace throughput
            let new_throughput = ThroughputProperties::manual(500);
            let throughput_response = offer_client
                .begin_replace_throughput(new_throughput, None)
                .await?
                .await?
                .into_model()?;
            assert_eq!(Some(500), throughput_response.throughput());

            Ok(())
        },
        Some(TestOptions::for_emulator()),
    )
    .await
}

#[tokio::test]
#[cfg_attr(
    not(any(test_category = "emulator", test_category = "emulator_vnext")),
    ignore = "requires test_category 'emulator' or 'emulator_vnext'"
)]
#[cfg_attr(
    test_category = "emulator_vnext",
    ignore = "skipped on vnext emulator: behavioral divergence"
)]
pub async fn container_throughput_crud_autoscale() -> Result<(), Box<dyn Error>> {
    TestClient::run_with_unique_db(
        async |run_context, db_client| {
            let properties = ContainerProperties::new("TheContainer", "/id".into());

            let throughput = ThroughputProperties::autoscale(5000, Some(42));

            let _container_client = run_context
                .create_container(
                    db_client,
                    properties.clone(),
                    Some(CreateContainerOptions::default().with_throughput(throughput)),
                )
                .await?;

            // Throughput/offer operations are control-plane and are not covered
            // by the data-plane RBAC role used on the AAD live leg, so route them
            // through the management (key) client.
            let offer_client = run_context
                .management_container_client(db_client, "TheContainer")
                .await?;

            // Read throughput
            let current_throughput = offer_client
                .read_throughput(None)
                .await?
                .expect("throughput should be present");
            assert_eq!(Some(500), current_throughput.throughput());
            assert_eq!(Some(5000), current_throughput.autoscale_maximum());
            assert_eq!(Some(42), current_throughput.autoscale_increment());

            Ok(())
        },
        Some(TestOptions::for_emulator()),
    )
    .await
}

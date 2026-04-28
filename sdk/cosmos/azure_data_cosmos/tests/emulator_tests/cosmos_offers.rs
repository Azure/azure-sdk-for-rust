// Copyright (c) Microsoft Corporation. All rights reserved.
// Licensed under the MIT License.
#![cfg(feature = "key_auth")]

// Use the shared test framework declared in `tests/emulator/mod.rs`.
use super::framework;

use std::error::Error;

use azure_data_cosmos::{
    models::{ContainerProperties, ThroughputProperties},
    CreateContainerOptions,
};
use framework::TestClient;

#[tokio::test]
#[cfg_attr(
    not(test_category = "emulator"),
    ignore = "requires test_category 'emulator'"
)]
pub async fn container_throughput_crud_manual() -> Result<(), Box<dyn Error>> {
    TestClient::run_with_unique_db(
        async |run_context, db_client| {
            let properties = ContainerProperties::new("TheContainer", "/id".into());

            let throughput = ThroughputProperties::manual(400);

            let container_client = run_context
                .create_container(
                    db_client,
                    properties.clone(),
                    Some(CreateContainerOptions::default().with_throughput(throughput)),
                )
                .await?;

            // Read throughput
            let current_throughput = container_client
                .read_throughput(None)
                .await?
                .expect("throughput should be present");

            assert_eq!(Some(400), current_throughput.throughput());

            // Replace throughput
            let new_throughput = ThroughputProperties::manual(500);
            let throughput_response = container_client
                .begin_replace_throughput(new_throughput, None)
                .await?
                .await?
                .into_model()?;
            assert_eq!(Some(500), throughput_response.throughput());

            Ok(())
        },
        None,
    )
    .await
}

#[tokio::test]
#[cfg_attr(
    not(test_category = "emulator"),
    ignore = "requires test_category 'emulator'"
)]
pub async fn container_throughput_crud_autoscale() -> Result<(), Box<dyn Error>> {
    TestClient::run_with_unique_db(
        async |run_context, db_client| {
            let properties = ContainerProperties::new("TheContainer", "/id".into());

            let throughput = ThroughputProperties::autoscale(5000, Some(42));

            let container_client = run_context
                .create_container(
                    db_client,
                    properties.clone(),
                    Some(CreateContainerOptions::default().with_throughput(throughput)),
                )
                .await?;

            // Read throughput
            let current_throughput = container_client
                .read_throughput(None)
                .await?
                .expect("throughput should be present");

            assert_eq!(Some(500), current_throughput.throughput());
            assert_eq!(Some(5000), current_throughput.autoscale_maximum());
            assert_eq!(Some(42), current_throughput.autoscale_increment());

            Ok(())
        },
        None,
    )
    .await
}

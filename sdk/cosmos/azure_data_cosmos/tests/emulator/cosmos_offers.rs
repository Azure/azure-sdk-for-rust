// Copyright (c) Microsoft Corporation. All rights reserved.
// Licensed under the MIT License.
#![cfg(feature = "key_auth")]

// Use the shared test framework declared in `tests/emulator/mod.rs`.
use super::framework;

use std::error::Error;

use azure_data_cosmos::{
    models::{ContainerProperties, ThroughputProperties},
    CreateContainerOptions, CreateDatabaseOptions,
};
use framework::TestClient;

#[tokio::test]
pub async fn database_throughput_crud() -> Result<(), Box<dyn Error>> {
    TestClient::run(
        async |run_context| {
            let cosmos_client = run_context.client();

            let test_db_id = run_context.db_name();
            let throughput = ThroughputProperties::manual(400);

            // Create a database with throughput
            let properties = cosmos_client
                .create_database(
                    &test_db_id,
                    Some(CreateDatabaseOptions {
                        throughput: Some(throughput),
                        ..Default::default()
                    }),
                )
                .await?
                .into_model()?;

            assert_eq!(&test_db_id, &properties.id);

            let db_client = cosmos_client.database_client(&test_db_id);

            // Read throughput
            let current_throughput = db_client
                .read_throughput(None)
                .await?
                .ok_or("expected a throughput offer")?;
            assert_eq!(Some(400), current_throughput.throughput());
            assert!(current_throughput.autoscale_increment().is_none());
            assert!(current_throughput.autoscale_maximum().is_none());

            // Replace throughput
            let new_throughput = db_client
                .replace_throughput(ThroughputProperties::manual(500), None)
                .await?
                .into_model()?;
            assert_eq!(Some(500), new_throughput.throughput());
            assert!(new_throughput.autoscale_increment().is_none());
            assert!(new_throughput.autoscale_maximum().is_none());

            Ok(())
        },
        None,
    )
    .await
}

#[tokio::test]
pub async fn container_throughput_crud_manual() -> Result<(), Box<dyn Error>> {
    TestClient::run_with_db(
        async |_, db_client| {
            let properties = ContainerProperties {
                id: "TheContainer".into(),
                partition_key: "/id".into(),
                ..Default::default()
            };

            let throughput = ThroughputProperties::manual(400);

            db_client
                .create_container(
                    properties.clone(),
                    Some(CreateContainerOptions {
                        throughput: Some(throughput),
                        ..Default::default()
                    }),
                )
                .await?
                .into_model()?;

            let container_client = db_client.container_client(&properties.id);

            // Read throughput
            let current_throughput = container_client
                .read_throughput(None)
                .await?
                .expect("throughput should be present");

            assert_eq!(Some(400), current_throughput.throughput());

            // Replace throughput
            let new_throughput = ThroughputProperties::manual(500);
            let throughput_response = container_client
                .replace_throughput(new_throughput, None)
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
pub async fn container_throughput_crud_autoscale() -> Result<(), Box<dyn Error>> {
    TestClient::run_with_db(
        async |_, db_client| {
            let properties = ContainerProperties {
                id: "TheContainer".into(),
                partition_key: "/id".into(),
                ..Default::default()
            };

            let throughput = ThroughputProperties::autoscale(5000, Some(42));

            db_client
                .create_container(
                    properties.clone(),
                    Some(CreateContainerOptions {
                        throughput: Some(throughput),
                        ..Default::default()
                    }),
                )
                .await?
                .into_model()?;

            let container_client = db_client.container_client(&properties.id);

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

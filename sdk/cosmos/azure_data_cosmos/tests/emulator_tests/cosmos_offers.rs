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
use futures::TryStreamExt;

#[tokio::test]
pub async fn database_throughput_crud() -> Result<(), Box<dyn Error>> {
    TestClient::run(async |run_context| {
        let cosmos_client = run_context.client();

        let test_db_id = run_context.db_name();
        let throughput = ThroughputProperties::manual(400);

        // Create a database with throughput
        let properties = cosmos_client
            .create_database(
                &test_db_id,
                Some(CreateDatabaseOptions::default().with_throughput(throughput)),
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
            .begin_replace_throughput(ThroughputProperties::manual(500), None)
            .await?
            .await?
            .into_model()?;
        assert_eq!(Some(500), new_throughput.throughput());
        assert!(new_throughput.autoscale_increment().is_none());
        assert!(new_throughput.autoscale_maximum().is_none());

        Ok(())
    })
    .await
}

#[tokio::test]
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
pub async fn container_throughput_high_ru() -> Result<(), Box<dyn Error>> {
    TestClient::run_with_unique_db(
        async |run_context, db_client| {
            let properties = ContainerProperties::new("HighRuContainer", "/id".into());

            let throughput = ThroughputProperties::manual(400);

            let container_client = run_context
                .create_container(
                    db_client,
                    properties.clone(),
                    Some(CreateContainerOptions::default().with_throughput(throughput)),
                )
                .await?;

            // Read initial throughput
            let current_throughput = container_client
                .read_throughput(None)
                .await?
                .expect("throughput should be present");
            assert_eq!(Some(400), current_throughput.throughput());

            // Replace to a high throughput (11000 RU/s) which may trigger async processing
            let new_throughput = ThroughputProperties::manual(11000);
            let throughput_response = container_client
                .begin_replace_throughput(new_throughput, None)
                .await?
                .await?
                .into_model()?;
            assert_eq!(Some(11000), throughput_response.throughput());

            Ok(())
        },
        None,
    )
    .await
}

#[tokio::test]
pub async fn container_throughput_stream_polling() -> Result<(), Box<dyn Error>> {
    TestClient::run_with_unique_db(
        async |run_context, db_client| {
            let properties = ContainerProperties::new("StreamPollContainer", "/id".into());

            let throughput = ThroughputProperties::manual(400);

            let container_client = run_context
                .create_container(
                    db_client,
                    properties.clone(),
                    Some(CreateContainerOptions::default().with_throughput(throughput)),
                )
                .await?;

            // Use the Stream interface to poll for throughput replacement progress
            let new_throughput = ThroughputProperties::manual(500);
            let mut poller = container_client
                .begin_replace_throughput(new_throughput, None)
                .await?;

            let mut count = 0;
            let mut last_throughput = None;
            while let Some(status) = poller.try_next().await? {
                count += 1;
                assert!(status.status().is_success());
                last_throughput = Some(status.into_model()?);
            }

            assert!(count >= 1, "stream should yield at least one response");
            let final_throughput =
                last_throughput.expect("stream should have yielded a throughput response");
            assert_eq!(Some(500), final_throughput.throughput());

            Ok(())
        },
        None,
    )
    .await
}

#[tokio::test]
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

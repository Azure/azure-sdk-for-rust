// Copyright (c) Microsoft Corporation. All rights reserved.
// Licensed under the MIT License.
#![cfg(all(feature = "key_auth"))]

// Use the shared test framework declared in `tests/emulator/mod.rs`.
use super::framework;

use std::error::Error;

use azure_data_cosmos::{models::ThroughputProperties, CreateDatabaseOptions, Query};
use framework::TestClient;
use futures::TryStreamExt;

#[tokio::test]
pub async fn database_crud() -> Result<(), Box<dyn Error>> {
    TestClient::run(
        async |run_context| {
            let cosmos_client = run_context.client();

            let test_db_id = run_context.db_name();

            // Create a database
            let properties = cosmos_client
                .create_database(&test_db_id, None)
                .await?
                .into_model()?;

            assert_eq!(&test_db_id, &properties.id);

            let db_client = cosmos_client.database_client(&test_db_id);
            let read_properties = db_client.read(None).await?.into_model()?;

            assert_eq!(&test_db_id, &read_properties.id);

            let query = Query::from("SELECT * FROM root r WHERE r.id = @id")
                .with_parameter("@id", &test_db_id)?;
            let mut pager = cosmos_client.query_databases(query.clone(), None)?;
            let mut ids = Vec::new();
            while let Some(db) = pager.try_next().await? {
                ids.push(db.id);
            }
            assert_eq!(vec![test_db_id.clone()], ids);

            let current_throughput = db_client.read_throughput(None).await?;
            assert!(current_throughput.is_none());

            // We're testing delete, so we want to manually delete the DB rather than letting the clean-up process do it.
            db_client.delete(None).await?;

            let mut pager = cosmos_client.query_databases(query, None)?;
            let mut ids = Vec::new();
            while let Some(db) = pager.try_next().await? {
                ids.push(db.id);
            }
            assert!(ids.is_empty());

            Ok(())
        },
        None,
    )
    .await
}
#[tokio::test]
#[cfg(feature = "key_auth")]
pub async fn database_with_offer_crud() -> Result<(), Box<dyn Error>> {
    TestClient::run(
        async |run_context| {
            let cosmos_client = run_context.client();

            let test_db_id = run_context.db_name();
            let throughput = ThroughputProperties::manual(400);

            // Create a database
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
            let read_properties = db_client.read(None).await?.into_model()?;
            assert_eq!(&test_db_id, &read_properties.id);

            // Read and then replace throughput
            let current_throughput = db_client
                .read_throughput(None)
                .await?
                .ok_or("expected a throughput offer")?;
            assert_eq!(Some(400), current_throughput.throughput());
            assert!(current_throughput.autoscale_increment().is_none());
            assert!(current_throughput.autoscale_maximum().is_none());

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

// Copyright (c) Microsoft Corporation. All rights reserved.
// Licensed under the MIT License.
#![cfg(feature = "key_auth")]

// Use the shared test framework declared in `tests/multi_write/mod.rs`.
use super::framework;

use std::borrow::Cow;
use std::error::Error;

use azure_data_cosmos::{
    clients::DatabaseClient,
    models::{ContainerProperties, ThroughputProperties},
    CosmosClientOptions, CreateContainerOptions,
};

use framework::{TestClient, HUB_REGION, SATELLITE_REGION};

// Helper to avoid duplicating the same preferred-locations setup.
fn options_with_preferred_locations(locations: Vec<Cow<'static, str>>) -> CosmosClientOptions {
    CosmosClientOptions {
        application_preferred_regions: locations,
        ..Default::default()
    }
}

async fn create_container_and_write_item(
    db_client: &DatabaseClient,
    container_id: &str,
) -> Result<(), Box<dyn Error>> {
    let properties = ContainerProperties {
        id: Cow::Owned(String::from(container_id)),
        partition_key: "/id".into(),
        ..Default::default()
    };

    let throughput = ThroughputProperties::manual(400);

    let created_properties = db_client
        .create_container(
            properties,
            Some(CreateContainerOptions {
                throughput: Some(throughput),
                ..Default::default()
            }),
        )
        .await?
        .into_model()?;
    // keep reading container until it is fully created
    loop {
        match db_client
            .container_client(&created_properties.id)
            .read(None)
            .await
        {
            Ok(_) => break,
            Err(e) => {
                println!("waiting for container to be created: {}", e);
                tokio::time::sleep(std::time::Duration::from_secs(1)).await;
            }
        }
    }

    let container_client = db_client.container_client(&created_properties.id);
    let _create_result = container_client
        .upsert_item(
            "item1",
            &serde_json::json!({"id": "item1", "value": "test"}),
            None,
        )
        .await?;

    Ok(())
}

#[tokio::test]
pub async fn multi_write_preferred_locations() -> Result<(), Box<dyn Error>> {
    const CONTAINER_ID: &str = "MultiWritePreferredLocations";

    // write to hub region
    TestClient::run_with_db(
        async |_, db_client| create_container_and_write_item(db_client, CONTAINER_ID).await,
        Some(options_with_preferred_locations(vec![
            HUB_REGION.into(),
            SATELLITE_REGION.into(),
        ])),
    )
    .await?;

    // write to satellite region
    TestClient::run_with_db(
        async |_, db_client| create_container_and_write_item(db_client, CONTAINER_ID).await,
        Some(options_with_preferred_locations(vec![
            SATELLITE_REGION.into(),
            HUB_REGION.into(),
        ])),
    )
    .await?;

    Ok(())
}

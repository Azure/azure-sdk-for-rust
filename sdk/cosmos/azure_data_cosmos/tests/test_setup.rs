// Copyright (c) Microsoft Corporation. All rights reserved.
// Licensed under the MIT License.

//! Test file to create shared test resources before running integration tests.
//! Run with: cargo test --test test_setup --features key_auth -- --ignored

#![cfg(feature = "key_auth")]

mod framework;

use azure_core::http::StatusCode;
use azure_data_cosmos::models::ContainerProperties;
use tracing_subscriber::EnvFilter;

use crate::framework::SHARED_PARTITION_KEY;
use framework::test_client::{create_client, SHARED_CONTAINER_ID, SHARED_DATABASE_ID};

#[tokio::test]
#[ignore = "Run manually to set up shared test resources"]
async fn setup_shared_resources() -> Result<(), Box<dyn std::error::Error>> {
    // Initialize tracing
    _ = tracing_subscriber::fmt()
        .with_env_filter(EnvFilter::from_default_env())
        .try_init();

    let Some(client) = create_client()? else {
        println!("No connection string provided. Skipping test setup.");
        return Ok(());
    };

    println!("Setting up shared test resources...");

    // Create shared database
    println!("Creating shared database: {}", SHARED_DATABASE_ID);
    match client.create_database(SHARED_DATABASE_ID, None).await {
        Ok(_) => println!("Shared database created successfully."),
        Err(e) if e.http_status() == Some(StatusCode::Conflict) => {
            println!("Shared database already exists, skipping creation.");
        }
        Err(e) => return Err(e.into()),
    }

    // Create shared container
    let db_client = client.database_client(SHARED_DATABASE_ID);
    println!("Creating shared container: {}", SHARED_CONTAINER_ID);

    let properties = ContainerProperties {
        id: SHARED_CONTAINER_ID.into(),
        partition_key: ("/".to_owned() + SHARED_PARTITION_KEY).into(),
        ..Default::default()
    };
    match db_client.create_container(properties, None).await {
        Ok(_) => println!("Shared container created successfully."),
        Err(e) if e.http_status() == Some(StatusCode::Conflict) => {
            println!("Shared container already exists, skipping creation.");
        }
        Err(e) => return Err(e.into()),
    }

    println!("Shared test resources setup complete.");
    Ok(())
}

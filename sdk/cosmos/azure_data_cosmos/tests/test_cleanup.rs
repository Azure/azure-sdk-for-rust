// Copyright (c) Microsoft Corporation. All rights reserved.
// Licensed under the MIT License.

//! Test file to delete shared test resources after running integration tests.
//! Run with: cargo test --test test_cleanup --features key_auth -- --ignored

#![cfg(feature = "key_auth")]

mod framework;

use azure_core::http::StatusCode;
use tracing_subscriber::EnvFilter;

use framework::test_client::{create_client, SHARED_DATABASE_ID};

#[tokio::test]
#[ignore = "Run manually to clean up shared test resources"]
async fn cleanup_shared_resources() -> Result<(), Box<dyn std::error::Error>> {
    // Initialize tracing
    _ = tracing_subscriber::fmt()
        .with_env_filter(EnvFilter::from_default_env())
        .try_init();

    let Some(client) = create_client()? else {
        println!("No connection string provided. Skipping test cleanup.");
        return Ok(());
    };

    println!("Cleaning up shared test resources...");

    // Delete shared database (this will also delete all containers)
    println!("Deleting shared database: {}", SHARED_DATABASE_ID);
    let db_client = client.database_client(SHARED_DATABASE_ID);
    match db_client.delete(None).await {
        Ok(_) => println!("Shared database deleted successfully."),
        Err(e) if e.http_status() == Some(StatusCode::NotFound) => {
            println!("Shared database does not exist, skipping deletion.");
        }
        Err(e) => return Err(e.into()),
    }

    println!("Shared test resources cleanup complete.");
    Ok(())
}

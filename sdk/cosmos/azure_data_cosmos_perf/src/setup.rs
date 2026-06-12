// Copyright (c) Microsoft Corporation. All rights reserved.
// Licensed under the MIT License.

//! Robust database and container setup with retry logic for multi-region consistency.

use std::time::Duration;

use azure_core::http::StatusCode;
use azure_data_cosmos::models::{ContainerProperties, ThroughputProperties};
use azure_data_cosmos::{clients::ContainerClient, clients::DatabaseClient, models::TimeToLive};
use azure_data_cosmos::{options::CreateContainerOptions, CosmosClient};

const MAX_RETRIES: u32 = 10;
const INITIAL_BACKOFF: Duration = Duration::from_secs(1);
const MAX_BACKOFF: Duration = Duration::from_secs(30);

/// Ensures a container exists, creating it if necessary.
///
/// Handles multi-region race conditions:
/// - First tries `container_client`, which eagerly resolves container
///   metadata and itself returns 404 when the container does not exist
///   yet — that's what lets us detect the missing case here.
/// - On 404, creates the container (treating a 409 as "another instance
///   created it concurrently").
/// - After creation, polls `container_client` with backoff until the
///   driver can resolve the new container's metadata, then verifies it
///   is readable.
pub async fn ensure_container(
    db_client: &DatabaseClient,
    container_name: &str,
    throughput: usize,
    default_ttl: Option<TimeToLive>,
) -> Result<ContainerClient, Box<dyn std::error::Error>> {
    // Try to resolve the existing container first. `container_client` is
    // not a constructor — it surfaces the same 404 that `read` would,
    // so we have to branch on the error here instead of unconditionally
    // `?`-propagating it (which would short-circuit the create path).
    match db_client.container_client(container_name).await {
        Ok(container_client) => match container_client.read(None).await {
            Ok(_) => {
                println!("Container '{container_name}' already exists.");
                return Ok(container_client);
            }
            Err(e) if e.status().status_code() == StatusCode::NotFound => {
                println!(
                    "Container '{container_name}' metadata resolved but read returned 404, creating with {throughput} RU/s..."
                );
            }
            Err(e) => return Err(e.into()),
        },
        Err(e) if e.status().status_code() == StatusCode::NotFound => {
            println!("Container '{container_name}' not found, creating with {throughput} RU/s...");
        }
        Err(e) => return Err(e.into()),
    }

    let mut props = ContainerProperties::new(container_name.to_string(), "/partition_key".into());
    if let Some(ttl) = default_ttl {
        props = props.with_default_ttl(ttl);
    }
    let create_opts =
        CreateContainerOptions::default().with_throughput(ThroughputProperties::manual(throughput));

    match db_client.create_container(props, Some(create_opts)).await {
        Ok(_) => {
            println!("Container '{container_name}' created.");
        }
        Err(e) if e.status().status_code() == StatusCode::Conflict => {
            println!("Container '{container_name}' was created concurrently.");
        }
        Err(e) => return Err(e.into()),
    }

    // Poll until the driver can resolve the new container (handles
    // multi-region replication lag) and confirm it's readable.
    let mut backoff = INITIAL_BACKOFF;

    for attempt in 1..=MAX_RETRIES {
        match db_client.container_client(container_name).await {
            Ok(container_client) => match container_client.read(None).await {
                Ok(_) => {
                    println!("Container '{container_name}' confirmed readable.");
                    return Ok(container_client);
                }
                Err(e) if e.status().status_code() == StatusCode::NotFound => {
                    println!(
                        "Container not yet readable (attempt {attempt}/{MAX_RETRIES}), retrying in {backoff:?}..."
                    );
                    tokio::time::sleep(backoff).await;
                    backoff = (backoff * 2).min(MAX_BACKOFF);
                }
                Err(e) => return Err(e.into()),
            },
            Err(e) if e.status().status_code() == StatusCode::NotFound => {
                println!(
                    "Container metadata not yet visible (attempt {attempt}/{MAX_RETRIES}), retrying in {backoff:?}..."
                );
                tokio::time::sleep(backoff).await;
                backoff = (backoff * 2).min(MAX_BACKOFF);
            }
            Err(e) => return Err(e.into()),
        }
    }

    Err(format!("Container '{container_name}' not readable after {MAX_RETRIES} retries").into())
}

/// Ensures a database exists, creating it if necessary.
///
/// Handles multi-region race conditions the same way as [`ensure_container`]:
/// - On 404, creates the database.
/// - On create conflict (409), assumes another instance created it.
/// - After creation, polls until the database is readable.
pub async fn ensure_database(
    client: &CosmosClient,
    db_name: &str,
) -> Result<(), Box<dyn std::error::Error>> {
    let db_client = client.database_client(db_name);

    match db_client.read(None).await {
        Ok(_) => {
            println!("Database '{db_name}' already exists.");
            return Ok(());
        }
        Err(e) if e.status().status_code() == StatusCode::NotFound => {
            println!("Database '{db_name}' not found, creating...");
        }
        Err(e) => return Err(e.into()),
    }

    match client.create_database(db_name, None).await {
        Ok(_) => {
            println!("Database '{db_name}' created.");
        }
        Err(e) if e.status().status_code() == StatusCode::Conflict => {
            println!("Database '{db_name}' was created concurrently.");
        }
        Err(e) => return Err(e.into()),
    }

    // Poll until the database is readable (handles multi-region replication lag).
    let mut backoff = INITIAL_BACKOFF;

    for attempt in 1..=MAX_RETRIES {
        match db_client.read(None).await {
            Ok(_) => {
                println!("Database '{db_name}' confirmed readable.");
                return Ok(());
            }
            Err(e) if e.status().status_code() == StatusCode::NotFound => {
                println!(
                    "Database not yet visible (attempt {attempt}/{MAX_RETRIES}), retrying in {backoff:?}..."
                );
                tokio::time::sleep(backoff).await;
                backoff = (backoff * 2).min(MAX_BACKOFF);
            }
            Err(e) => return Err(e.into()),
        }
    }

    Err(format!("Database '{db_name}' not readable after {MAX_RETRIES} retries").into())
}

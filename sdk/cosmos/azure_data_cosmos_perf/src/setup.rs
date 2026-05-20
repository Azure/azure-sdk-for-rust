// Copyright (c) Microsoft Corporation. All rights reserved.
// Licensed under the MIT License.

//! Database and container provisioning for the perf harness.
//!
//! Both helpers use a "create-or-get" pattern: attempt creation first,
//! treat `409 Conflict` as "already exists", then poll until the resource
//! is readable in the current region (multi-region replication lag).
//!
//! Why not check-then-create: in this SDK, `container_client(name)`
//! eagerly resolves the container's metadata via the driver, so calling
//! it before the container exists fails with `404 NotFound`. Going
//! straight to `create_container` avoids that chicken-and-egg problem.

use std::time::Duration;

use azure_core::http::StatusCode;
use azure_data_cosmos::clients::ContainerClient;
use azure_data_cosmos::models::{ContainerProperties, ThroughputProperties};
use azure_data_cosmos::{clients::DatabaseClient, models::TimeToLive};
use azure_data_cosmos::{CosmosClient, CreateContainerOptions};

const MAX_RETRIES: u32 = 10;
const INITIAL_BACKOFF: Duration = Duration::from_secs(1);
const MAX_BACKOFF: Duration = Duration::from_secs(30);

/// Ensures a container exists, creating it if necessary, and returns a
/// [`ContainerClient`] for it once the container is readable in the
/// current region.
pub async fn ensure_container(
    db_client: &DatabaseClient,
    container_name: &str,
    throughput: usize,
    default_ttl: Option<TimeToLive>,
) -> Result<ContainerClient, Box<dyn std::error::Error>> {
    let mut props = ContainerProperties::new(container_name.to_string(), "/partition_key".into());
    if let Some(ttl) = default_ttl {
        props = props.with_default_ttl(ttl);
    }
    let create_opts =
        CreateContainerOptions::default().with_throughput(ThroughputProperties::manual(throughput));

    // Attempt creation; 409 means another instance (or a prior run) already
    // created the container. Do NOT pre-flight with `container_client(name)`
    // — that call resolves metadata eagerly and would 404 on a brand-new
    // database.
    match db_client.create_container(props, Some(create_opts)).await {
        Ok(_) => println!("Container '{container_name}' created with {throughput} RU/s."),
        Err(e) if e.http_status() == Some(StatusCode::Conflict) => {
            println!("Container '{container_name}' already exists.");
        }
        Err(e) => return Err(e.into()),
    }

    // Poll until the container is readable locally. Building the
    // `ContainerClient` is itself a metadata resolution, so we treat
    // both the build and the subsequent `read` as the same "is it
    // readable yet?" probe.
    let mut backoff = INITIAL_BACKOFF;
    let mut last_err: Option<Box<dyn std::error::Error>> = None;
    for attempt in 1..=MAX_RETRIES {
        match db_client.container_client(container_name).await {
            Ok(client) => match client.read(None).await {
                Ok(_) => {
                    println!("Container '{container_name}' confirmed readable.");
                    return Ok(client);
                }
                Err(e) if e.http_status() == Some(StatusCode::NotFound) => {
                    println!(
                        "Container not yet visible (attempt {attempt}/{MAX_RETRIES}), retrying in {backoff:?}..."
                    );
                    last_err = Some(e.into());
                }
                Err(e) => return Err(e.into()),
            },
            Err(e) if e.http_status() == Some(StatusCode::NotFound) => {
                println!(
                    "Container metadata not yet visible (attempt {attempt}/{MAX_RETRIES}), retrying in {backoff:?}..."
                );
                last_err = Some(e.into());
            }
            Err(e) => return Err(e.into()),
        }
        tokio::time::sleep(backoff).await;
        backoff = (backoff * 2).min(MAX_BACKOFF);
    }

    Err(format!(
        "Container '{container_name}' not readable after {MAX_RETRIES} retries: {last_err:?}"
    )
    .into())
}

/// Ensures a database exists, creating it if necessary.
pub async fn ensure_database(
    client: &CosmosClient,
    db_name: &str,
) -> Result<(), Box<dyn std::error::Error>> {
    let db_client = client.database_client(db_name);

    // Same create-or-get pattern as `ensure_container`. `database_client`
    // is sync and does NOT resolve metadata, so it's safe to use upfront
    // for the `read`/`create` calls.
    match db_client.read(None).await {
        Ok(_) => {
            println!("Database '{db_name}' already exists.");
            return Ok(());
        }
        Err(e) if e.http_status() == Some(StatusCode::NotFound) => {
            println!("Database '{db_name}' not found, creating...");
        }
        Err(e) => return Err(e.into()),
    }

    match client.create_database(db_name, None).await {
        Ok(_) => println!("Database '{db_name}' created."),
        Err(e) if e.http_status() == Some(StatusCode::Conflict) => {
            println!("Database '{db_name}' was created concurrently.");
        }
        Err(e) => return Err(e.into()),
    }

    let mut backoff = INITIAL_BACKOFF;
    for attempt in 1..=MAX_RETRIES {
        match db_client.read(None).await {
            Ok(_) => {
                println!("Database '{db_name}' confirmed readable.");
                return Ok(());
            }
            Err(e) if e.http_status() == Some(StatusCode::NotFound) => {
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

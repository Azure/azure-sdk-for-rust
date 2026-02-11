// Copyright (c) Microsoft Corporation. All rights reserved.
// Licensed under the MIT License.

mod config;
mod operations;
mod runner;
mod seed;
mod stats;

use std::sync::Arc;
use std::time::Duration;

use azure_core::credentials::Secret;
use azure_data_cosmos::models::{ContainerProperties, ThroughputProperties};
use azure_data_cosmos::{CosmosClient, CosmosClientOptions, CreateContainerOptions};
use clap::Parser;

use crate::config::{AuthMethod, Config};
use crate::operations::create_operations;
use crate::stats::Stats;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let config = Config::parse();

    // Validate configuration
    if config.no_reads && config.no_queries && config.no_upserts {
        eprintln!("Error: all operations are disabled. Enable at least one.");
        std::process::exit(1);
    }

    // Build client options
    let options = CosmosClientOptions {
        application_preferred_regions: config
            .preferred_regions
            .iter()
            .map(|r| r.clone().into())
            .collect(),
        excluded_regions: config
            .excluded_regions
            .iter()
            .map(|r| r.clone().into())
            .collect(),
        ..Default::default()
    };

    // Create the Cosmos client
    let client = match &config.auth {
        AuthMethod::Key => {
            let key = config.key.as_deref().ok_or(
                "Account key is required for key auth. Use --key or set AZURE_COSMOS_KEY env var.",
            )?;
            CosmosClient::with_key(
                &config.endpoint,
                Secret::from(key.to_string()),
                Some(options),
            )?
        }
        AuthMethod::Aad => {
            let credential = azure_identity::DeveloperToolsCredential::new(None)?;
            CosmosClient::new(&config.endpoint, credential, Some(options))?
        }
    };

    let db_client = client.database_client(&config.database);
    let container_client = db_client.container_client(&config.container);

    // Create the container if it doesn't exist
    match container_client.read(None).await {
        Ok(_) => {
            println!("Container '{}' already exists.", config.container);
        }
        Err(_) => {
            println!(
                "Container '{}' not found, creating with {} RU/s...",
                config.container, config.throughput
            );
            let props = ContainerProperties {
                id: config.container.clone().into(),
                partition_key: "/partition_key".into(),
                ..Default::default()
            };
            let create_opts = CreateContainerOptions {
                throughput: Some(ThroughputProperties::manual(config.throughput)),
                ..Default::default()
            };
            db_client.create_container(props, Some(create_opts)).await?;
            println!("Container '{}' created.", config.container);
        }
    }

    // Seed the container
    seed::seed_container(&container_client, config.seed_count, config.concurrency).await?;

    // Create enabled operations
    let ops = create_operations(&config, config.seed_count);
    println!(
        "\nStarting perf test: {} operation(s), concurrency={}",
        ops.len(),
        config.concurrency
    );
    for op in &ops {
        println!("  - {}", op.name());
    }

    let duration = config.duration.map(Duration::from_secs);
    if let Some(d) = duration {
        println!("  Duration: {:.0}s", d.as_secs_f64());
    } else {
        println!("  Duration: indefinite (Ctrl+C to stop)");
    }
    println!();

    // Run the perf test
    let stats = Arc::new(Stats::new());
    runner::run(
        container_client,
        ops,
        stats,
        config.concurrency,
        duration,
        Duration::from_secs(config.report_interval),
    )
    .await;

    Ok(())
}

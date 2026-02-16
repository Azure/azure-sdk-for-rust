// Copyright (c) Microsoft Corporation. All rights reserved.
// Licensed under the MIT License.

mod config;
mod operations;
mod runner;
mod seed;
mod setup;
mod stats;

use std::sync::Arc;
use std::time::Duration;

use azure_core::credentials::Secret;
use azure_data_cosmos::{CosmosClient, CosmosClientOptions};
use clap::Parser;

use crate::config::{AuthMethod, Config};
use crate::operations::create_operations;
use crate::runner::RunConfig;
use crate::stats::Stats;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let config = Config::parse();

    // Validate configuration
    if config.no_reads && config.no_queries && config.no_upserts {
        eprintln!("Error: all operations are disabled. Enable at least one.");
        std::process::exit(1);
    }
    if config.concurrency == 0 {
        eprintln!("Error: --concurrency must be at least 1.");
        std::process::exit(1);
    }
    if config.concurrency > u32::MAX as usize {
        eprintln!("Error: --concurrency cannot exceed {}.", u32::MAX);
        std::process::exit(1);
    }
    if config.seed_count == 0 {
        eprintln!("Error: --seed-count must be at least 1.");
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

    // Ensure the database exists (with retry logic for multi-region setups)
    setup::ensure_database(&client, &config.database).await?;

    // Ensure the container exists (with retry logic for multi-region setups)
    setup::ensure_container(&db_client, &config.container, config.throughput).await?;

    // Seed the container
    let seeded_items =
        seed::seed_container(&container_client, config.seed_count, config.concurrency).await?;
    let seeded_items = Arc::new(seeded_items);

    // Create enabled operations
    let ops = create_operations(&config, seeded_items);
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

    // Ensure the results container exists for storing metrics
    setup::ensure_container(&db_client, &config.results_container, 400).await?;
    let results_container = db_client.container_client(&config.results_container);
    println!(
        "Perf results will be stored in container '{}'. Workload ID: {}",
        config.results_container, config.workload_id,
    );

    // Run the perf test
    let stats = Arc::new(Stats::new());
    runner::run(RunConfig {
        container: container_client,
        operations: ops,
        stats,
        concurrency: config.concurrency,
        duration,
        report_interval: Duration::from_secs(config.report_interval),
        results_container,
        workload_id: config.workload_id,
    })
    .await;

    Ok(())
}

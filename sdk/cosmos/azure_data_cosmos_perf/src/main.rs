// Copyright (c) Microsoft Corporation. All rights reserved.
// Licensed under the MIT License.

mod config;
mod operations;
mod runner;
mod seed;
mod setup;
mod stats;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    use std::sync::Arc;
    use std::time::Duration;

    use azure_core::credentials::Secret;
    use azure_data_cosmos::{CosmosClient, CosmosClientOptions};
    use clap::Parser;

    use crate::config::{AuthMethod, Config};
    use crate::operations::create_operations;
    use crate::runner::RunConfig;
    use crate::stats::Stats;

    let config = Config::parse();

    // Validate configuration
    if config.no_reads && config.no_queries && config.no_upserts && config.no_creates {
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
    let options = CosmosClientOptions::default()
        .with_preferred_regions(
            config
                .preferred_regions
                .iter()
                .map(|r| r.clone().into())
                .collect(),
        )
        .with_excluded_regions(
            config
                .excluded_regions
                .iter()
                .map(|r| r.clone().into())
                .collect(),
        );

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
            let credential = azure_identity::ManagedIdentityCredential::new(None)?;
            CosmosClient::new(&config.endpoint, credential, Some(options))?
        }
    };

    let db_client = client.database_client(&config.database);
    let container_client = db_client.container_client(&config.container);

    // Ensure the database exists (with retry logic for multi-region setups)
    setup::ensure_database(&client, &config.database).await?;

    // Convert TTL: 0 means disabled (None), >0 means that duration
    let default_ttl = if config.default_ttl == 0 {
        None
    } else {
        Some(Duration::from_secs(config.default_ttl))
    };

    // Ensure the container exists (with retry logic for multi-region setups)
    setup::ensure_container(
        &db_client,
        &config.container,
        config.throughput,
        default_ttl,
    )
    .await?;

    // Seed the container
    let seeded_items =
        seed::seed_container(&container_client, config.seed_count, config.concurrency).await?;
    let seeded_items = seed::SharedItems::new(seeded_items);

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

    // Set up results container — either on the same account or a separate one
    let results_container = if let Some(ref results_endpoint) = config.results_endpoint {
        let results_auth = config.results_auth.as_ref().unwrap_or(&config.auth);
        let results_client = match results_auth {
            AuthMethod::Key => {
                let key = config.results_key.as_deref().ok_or(
                    "Results account key is required. Use --results-key or set AZURE_COSMOS_RESULTS_KEY.",
                )?;
                CosmosClient::with_key(results_endpoint, Secret::from(key.to_string()), None)?
            }
            AuthMethod::Aad => {
                let credential = azure_identity::ManagedIdentityCredential::new(None)?;
                CosmosClient::new(results_endpoint, credential, None)?
            }
        };
        setup::ensure_database(&results_client, &config.results_database).await?;
        let results_db = results_client.database_client(&config.results_database);
        setup::ensure_container(&results_db, &config.results_container, 400, default_ttl).await?;
        println!(
            "Perf results will be stored on separate account '{}' in '{}/{}'. Workload ID: {}",
            results_endpoint, config.results_database, config.results_container, config.workload_id,
        );
        results_db.container_client(&config.results_container)
    } else {
        setup::ensure_container(&db_client, &config.results_container, 400, default_ttl).await?;
        println!(
            "Perf results will be stored in container '{}'. Workload ID: {}",
            config.results_container, config.workload_id,
        );
        db_client.container_client(&config.results_container)
    };

    // Run the perf test
    let op_names: Vec<&str> = ops.iter().map(|op| op.name()).collect();
    let stats = Arc::new(Stats::new(&op_names));
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

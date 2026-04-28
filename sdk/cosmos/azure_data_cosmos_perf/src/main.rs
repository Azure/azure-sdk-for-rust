// Copyright (c) Microsoft Corporation. All rights reserved.
// Licensed under the MIT License.

use azure_data_cosmos::models::TimeToLive;

mod config;
mod operations;
mod runner;
mod seed;
mod setup;
mod stats;

/// Creates an AAD credential using WorkloadIdentity (AKS) with fallback to ManagedIdentity (VMs).
fn create_aad_credential(
) -> Result<std::sync::Arc<dyn azure_core::credentials::TokenCredential>, Box<dyn std::error::Error>>
{
    azure_identity::WorkloadIdentityCredential::new(None)
        .map(|c| c as std::sync::Arc<dyn azure_core::credentials::TokenCredential>)
        .or_else(|workload_err| {
            azure_identity::ManagedIdentityCredential::new(None)
                .map(|c| c as std::sync::Arc<dyn azure_core::credentials::TokenCredential>)
                .map_err(|managed_err| {
                    azure_core::Error::with_message(
                        azure_core::error::ErrorKind::Credential,
                        format!(
                            "Failed to create AAD credential. \
                             WorkloadIdentityCredential: {workload_err}, \
                             ManagedIdentityCredential: {managed_err}"
                        ),
                    )
                })
        })
        .map_err(|e| e.into())
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    // Initialize tokio-console subscriber when the feature is enabled.
    // This must happen before any tokio tasks are spawned.
    #[cfg(feature = "tokio-console")]
    {
        use std::net::{IpAddr, Ipv4Addr};

        let console_addr: IpAddr = match std::env::var("TOKIO_CONSOLE_ADDR") {
            Ok(val) => val.parse().unwrap_or_else(|e| {
                eprintln!(
                    "WARNING: invalid TOKIO_CONSOLE_ADDR={val:?}: {e}; defaulting to 127.0.0.1"
                );
                IpAddr::V4(Ipv4Addr::LOCALHOST)
            }),
            Err(_) => IpAddr::V4(Ipv4Addr::LOCALHOST),
        };
        let console_port: u16 = match std::env::var("TOKIO_CONSOLE_PORT") {
            Ok(val) => val.parse().unwrap_or_else(|e| {
                eprintln!("WARNING: invalid TOKIO_CONSOLE_PORT={val:?}: {e}; defaulting to 6669");
                6669
            }),
            Err(_) => 6669,
        };

        console_subscriber::ConsoleLayer::builder()
            .server_addr((console_addr, console_port))
            .init();

        let addr_display = if console_addr.is_ipv6() {
            format!("[{}]", console_addr)
        } else {
            console_addr.to_string()
        };

        if console_addr.is_loopback() {
            eprintln!(
                "tokio-console enabled on loopback — connect with: tokio-console http://{}:{}",
                addr_display, console_port
            );
        } else {
            let scope = if console_addr.is_unspecified() {
                "all interfaces"
            } else {
                "a non-loopback address"
            };
            eprintln!(
                "WARNING: tokio-console enabled on {}:{} ({scope}). \
                 Set TOKIO_CONSOLE_ADDR=127.0.0.1 to restrict to loopback.",
                addr_display, console_port
            );
        }
    }

    // Log Pyroscope status (profiling is handled externally via eBPF auto-instrumentation)
    if std::env::var("PYROSCOPE_SERVER_URL")
        .map(|v| !v.is_empty())
        .unwrap_or(false)
    {
        eprintln!("Pyroscope server configured — profiles collected via eBPF auto-instrumentation");
    }

    use std::sync::Arc;
    use std::time::Duration;

    use azure_core::credentials::Secret;
    use azure_data_cosmos::{
        CosmosAccountEndpoint, CosmosAccountReference, CosmosClientBuilder, RoutingStrategy,
    };
    use clap::Parser;

    use crate::config::{AuthMethod, Config};
    use crate::operations::create_operations;
    use crate::runner::{ConfigSnapshot, RunConfig};
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

    // Build the Cosmos client using the builder pattern
    let application_region: azure_data_cosmos::regions::Region =
        config.application_region.clone().into();
    let strategy = RoutingStrategy::ProximityTo(application_region.clone());

    let builder = CosmosClientBuilder::new();

    let endpoint: CosmosAccountEndpoint = config.endpoint.parse()?;
    let client = match &config.auth {
        AuthMethod::Key => {
            let key = config.key.as_deref().ok_or(
                "Account key is required for key auth. Use --key or set AZURE_COSMOS_KEY env var.",
            )?;
            let account =
                CosmosAccountReference::with_master_key(endpoint, Secret::from(key.to_string()));
            builder.build(account, strategy).await?
        }
        AuthMethod::Aad => {
            let credential = create_aad_credential()?;
            let account = CosmosAccountReference::with_credential(endpoint, credential);
            builder.build(account, strategy).await?
        }
    };

    let db_client = client.database_client(&config.database);
    let container_client = db_client.container_client(&config.container).await?;

    // Ensure the database exists (with retry logic for multi-region setups)
    setup::ensure_database(&client, &config.database).await?;

    // Convert TTL: 0 means disabled (None), >0 means that duration
    let default_ttl = if config.default_ttl == 0 {
        None
    } else {
        Some(TimeToLive::Seconds(config.default_ttl as u32))
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
        let results_ep: CosmosAccountEndpoint = results_endpoint.parse()?;
        let results_builder = CosmosClientBuilder::new();
        let results_strategy = RoutingStrategy::ProximityTo(application_region.clone());
        let results_client = match results_auth {
            AuthMethod::Key => {
                let key = config.results_key.as_deref().ok_or(
                    "Results account key is required. Use --results-key or set AZURE_COSMOS_RESULTS_KEY.",
                )?;
                let account = CosmosAccountReference::with_master_key(
                    results_ep,
                    Secret::from(key.to_string()),
                );
                results_builder.build(account, results_strategy).await?
            }
            AuthMethod::Aad => {
                let credential = create_aad_credential()?;
                let account = CosmosAccountReference::with_credential(results_ep, credential);
                results_builder.build(account, results_strategy).await?
            }
        };
        setup::ensure_database(&results_client, &config.results_database).await?;
        let results_db = results_client.database_client(&config.results_database);
        setup::ensure_container(
            &results_db,
            &config.results_container,
            10000,
            Some(TimeToLive::Seconds(86400)),
        )
        .await?;
        println!(
            "Perf results will be stored on separate account '{}' in '{}/{}'. Workload ID: {}",
            results_endpoint, config.results_database, config.results_container, config.workload_id,
        );
        results_db
            .container_client(&config.results_container)
            .await?
    } else {
        setup::ensure_container(
            &db_client,
            &config.results_container,
            10000,
            Some(TimeToLive::Seconds(86400)),
        )
        .await?;
        println!(
            "Perf results will be stored in container '{}'. Workload ID: {}",
            config.results_container, config.workload_id,
        );
        db_client
            .container_client(&config.results_container)
            .await?
    };

    // Resolve commit SHA: use CLI arg or auto-detect from git
    let commit_sha = config.commit_sha.unwrap_or_else(|| {
        std::process::Command::new("git")
            .args(["rev-parse", "--short", "HEAD"])
            .output()
            .ok()
            .filter(|o| o.status.success())
            .and_then(|o| String::from_utf8(o.stdout).ok())
            .map(|s| s.trim().to_string())
            .unwrap_or_else(|| "unknown".to_string())
    });

    // Resolve hostname for machine identification in results
    let hostname = hostname::get()
        .map(|h| h.to_string_lossy().to_string())
        .unwrap_or_else(|_| "unknown".to_string());

    // Build config snapshot for Grafana dashboard visibility
    let config_snapshot = ConfigSnapshot {
        concurrency: config.concurrency as u64,
        application_region: config.application_region.clone(),
        excluded_regions: config.excluded_regions.join(", "),
        tokio_threads: tokio::runtime::Handle::current().metrics().num_workers() as u64,
        ppcb_enabled: std::env::var("AZURE_COSMOS_PER_PARTITION_CIRCUIT_BREAKER_ENABLED")
            .ok()
            .and_then(|v| v.parse::<bool>().ok())
            .unwrap_or(true),
        gateway20_allowed: std::env::var("AZURE_COSMOS_CONNECTION_POOL_IS_GATEWAY20_ALLOWED")
            .ok()
            .and_then(|v| v.parse::<bool>().ok())
            .unwrap_or(false),
        pyroscope_enabled: std::env::var("PYROSCOPE_SERVER_URL")
            .map(|v| !v.is_empty())
            .unwrap_or(false),
        tokio_console_enabled: cfg!(feature = "tokio-console"),
        tokio_metrics_enabled: cfg!(feature = "tokio-metrics"),
        valgrind_tool: std::env::var("VALGRIND_TOOL").unwrap_or_default(),
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
        commit_sha,
        hostname,
        config_snapshot,
    })
    .await;

    Ok(())
}

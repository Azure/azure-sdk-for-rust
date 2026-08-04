// Copyright (c) Microsoft Corporation. All rights reserved.
// Licensed under the MIT License.

//! Observability soak/load harness for the Azure Cosmos DB Rust SDK.
//!
//! Registers the built-in diagnostics handlers (metrics, distributed tracing,
//! sampled logging) on a [`CosmosClient`](azure_data_cosmos::CosmosClient), wires
//! up an OpenTelemetry exporter (stdout, OTLP, or none), and drives a
//! configurable read/write/query workload — optionally with fault injection — so
//! the diagnostics layer can be validated end-to-end under sustained load.
//!
//! See the crate README for usage, environment variables, and collector wiring.

mod client;
mod config;
mod telemetry;
mod workload;

use clap::Parser;

use crate::config::Config;
use crate::telemetry::Telemetry;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let config = Config::parse();
    if let Err(msg) = config.validate() {
        eprintln!("error: {msg}");
        std::process::exit(2);
    }

    // Install telemetry first so the metrics handler binds to a live meter and
    // the tracing subscriber is ready before any operation runs.
    let telemetry = Telemetry::install(&config)?;

    print_banner(&config);

    // Run client construction *and* the workload under one result so that a
    // client-construction failure (exporter/auth/account errors) still goes
    // through the telemetry shutdown/flush path below, matching the README's
    // promise that exporters are flushed on the error path.
    let result = async {
        let (client, fault_activation) = client::build_client(&config).await?;
        workload::run(&client, &config, fault_activation).await
    }
    .await;

    // Flush and shut down the exporters so the final metric/span batch is
    // emitted even on the error path. The workload's own error takes precedence,
    // but a telemetry-shutdown failure (e.g. a collector outage) is surfaced
    // rather than exiting cleanly with missing telemetry.
    let shutdown = telemetry.shutdown();
    result?;
    shutdown
}

/// Prints a short summary of the active configuration and compiled handlers.
fn print_banner(config: &Config) {
    let endpoint = config
        .resolve_endpoint_and_key()
        .map(|(endpoint, _)| endpoint)
        .unwrap_or_else(|_| config.endpoint.clone());

    println!("Azure Cosmos DB observability soak harness");
    println!(
        "  endpoint: {endpoint} (emulator={})",
        config.is_emulator(&endpoint)
    );
    println!("  handlers: {}", enabled_handlers().join(", "));
    println!("  exporter: {:?}", config.exporter);
}

/// Lists the diagnostics handlers compiled into this build.
fn enabled_handlers() -> Vec<&'static str> {
    let mut handlers = Vec::new();
    if cfg!(feature = "metrics") {
        handlers.push("metrics");
    }
    if cfg!(feature = "distributed_tracing") {
        handlers.push("distributed_tracing");
    }
    // The sampled log handler is always registered.
    handlers.push("sampled_logging");
    if cfg!(feature = "fault_injection") {
        handlers.push("fault_injection(available)");
    }
    handlers
}

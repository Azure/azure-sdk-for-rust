// Copyright (c) Microsoft Corporation. All rights reserved.
// Licensed under the MIT License.

// cspell:ignore appender

//! Basic example showing how to create and use a blob checkpoint store.

use azure_core::http::{ClientOptions, InstrumentationOptions};
use azure_core_opentelemetry::OpenTelemetryTracerProvider;
use azure_identity::DeveloperToolsCredential;
use azure_messaging_eventhubs::{
    models::{Checkpoint, Ownership},
    CheckpointStore,
};
use azure_messaging_eventhubs_checkpointstore_blob::BlobCheckpointStore;
use azure_storage_blob::{BlobContainerClient, BlobContainerClientOptions};
use tracing::info;

use opentelemetry_appender_tracing::layer;
use opentelemetry_sdk::{logs::SdkLoggerProvider, Resource};
use tracing_subscriber::{prelude::*, EnvFilter};

// Configure tracing with OpenTelemetry and a stdout exporter as well as with the
// `tracing` crate.
fn configure_tracing() {
    let exporter = opentelemetry_stdout::LogExporter::default();
    let provider: SdkLoggerProvider = SdkLoggerProvider::builder()
        .with_resource(
            Resource::builder()
                .with_service_name("log-appender-tracing-example")
                .build(),
        )
        .with_simple_exporter(exporter)
        .build();

    // To prevent a telemetry-induced-telemetry loop, OpenTelemetry's own internal
    // logging is properly suppressed. However, logs emitted by external components
    // (such as reqwest, tonic, etc.) are not suppressed as they do not propagate
    // OpenTelemetry context. Until this issue is addressed
    // (https://github.com/open-telemetry/opentelemetry-rust/issues/2877),
    // filtering like this is the best way to suppress such logs.
    //
    // The filter levels are set as follows:
    // - Allow `info` level and above by default.
    // - Completely restrict logs from `hyper`, `tonic`, `h2`, and `reqwest`.
    //
    // Note: This filtering will also drop logs from these components even when
    // they are used outside of the OTLP Exporter.
    let filter_otel = EnvFilter::new("trace")
        .add_directive("hyper=off".parse().unwrap())
        .add_directive("hyper_util=off".parse().unwrap())
        .add_directive("opentelemetry=off".parse().unwrap())
        .add_directive("tonic=off".parse().unwrap())
        .add_directive("reqwest=off".parse().unwrap())
        .add_directive("h2=off".parse().unwrap());
    let otel_layer = layer::OpenTelemetryTracingBridge::new(&provider).with_filter(filter_otel);

    let opentelemetry_sdk = opentelemetry_sdk::trace::SdkTracerProvider::builder()
        .with_simple_exporter(opentelemetry_stdout::SpanExporter::default())
        .build();

    opentelemetry::global::set_tracer_provider(opentelemetry_sdk);

    // Create a new tracing::Fmt layer to print the logs to stdout. It has a
    // default filter of `info` level and above, and `debug` and above for logs
    // from OpenTelemetry crates. The filter levels can be customized as needed.
    //let filter_fmt = EnvFilter::new("trace").add_directive("opentelemetry=trace".parse().unwrap());
    // let fmt_layer = tracing_subscriber::fmt::layer()
    //     .with_thread_names(true)
    //     .with_filter(filter_fmt);

    tracing_subscriber::registry()
        .with(otel_layer)
        .init();
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    // Initialize tracing
    configure_tracing();
    //    tracing_subscriber::fmt::init();

    // Configuration - replace with your actual values
    let storage_account_url =
        std::env::var("AZURE_STORAGE_BLOB_ENDPOINT").expect("Missing AZURE_STORAGE_BLOB_ENDPOINT");

    let container = std::env::var("AZURE_STORAGE_BLOB_CONTAINER")
        .expect("Missing AZURE_STORAGE_BLOB_CONTAINER");

    info!("Creating blob checkpoint store...");

    // Create Azure credential and blob service client
    let credential = DeveloperToolsCredential::new(None)?;

    // Instantiate a blob client with OpenTelemetry instrumentation enabled
    let blob_container_client = BlobContainerClient::new(
        &storage_account_url,
        container,
        credential,
        Some(BlobContainerClientOptions {
            client_options: ClientOptions {
                instrumentation: Some(InstrumentationOptions {
                    tracer_provider: Some(OpenTelemetryTracerProvider::from_global_provider()),
                }),
                ..Default::default()
            },
            ..Default::default()
        }),
    )?;

    // Create the checkpoint store
    let checkpoint_store = BlobCheckpointStore::new(blob_container_client);

    // Example Event Hub configuration
    let fully_qualified_namespace = "your-eventhubs-namespace.servicebus.windows.net";
    let eventhub_name = "my-eventhub";
    let consumer_group = "$Default";
    let partition_id = "0";
    let owner_id = "example-processor-1";

    info!("Testing ownership operations...");

    // Create and claim ownership
    let ownership = Ownership {
        fully_qualified_namespace: fully_qualified_namespace.to_string(),
        event_hub_name: eventhub_name.to_string(),
        consumer_group: consumer_group.to_string(),
        partition_id: partition_id.to_string(),
        owner_id: Some(owner_id.to_string()),
        ..Default::default()
    };

    let claimed_ownerships = checkpoint_store.claim_ownership(&[ownership]).await?;

    info!("Claimed {} ownerships", claimed_ownerships.len());

    // List all ownerships
    let ownerships = checkpoint_store
        .list_ownerships(fully_qualified_namespace, eventhub_name, consumer_group)
        .await?;

    info!("Found {} existing ownerships", ownerships.len());

    info!("Testing checkpoint operations...");

    // Create and update a checkpoint
    let mut checkpoint = Checkpoint {
        fully_qualified_namespace: fully_qualified_namespace.to_string(),
        event_hub_name: eventhub_name.to_string(),
        consumer_group: consumer_group.to_string(),
        partition_id: partition_id.to_string(),
        ..Default::default()
    };

    // Simulate processing some events
    // This is where you would normally process events from the Event Hub
    // For this example, we'll just update the checkpoint with some dummy data
    checkpoint.sequence_number = Some(42);
    checkpoint.offset = Some("100".to_string());

    // Save the checkpoint
    checkpoint_store.update_checkpoint(checkpoint).await?;

    // List all checkpoints
    let checkpoints = checkpoint_store
        .list_checkpoints(fully_qualified_namespace, eventhub_name, consumer_group)
        .await?;

    info!("Found {} existing checkpoints", checkpoints.len());

    for checkpoint in &checkpoints {
        info!(
            "Checkpoint for partition {}: sequence={:?}, offset={:?}",
            checkpoint.partition_id, checkpoint.sequence_number, checkpoint.offset,
        );
    }

    info!("Example completed successfully!");

    Ok(())
}

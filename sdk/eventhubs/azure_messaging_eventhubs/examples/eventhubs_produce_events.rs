// Copyright (c) Microsoft Corporation.
// Licensed under the MIT license.
// cspell:ignore appender

//! This sample demonstrates how to send events to an Event Hub partition using the `ProducerClient`.

use azure_core::Uuid;
use azure_identity::DeveloperToolsCredential;
use azure_messaging_eventhubs::{models::EventData, ProducerClient, SendEventOptions};
use core::f32;
use opentelemetry_appender_tracing::layer;
use opentelemetry_sdk::{logs::SdkLoggerProvider, Resource};
use tracing_subscriber::{prelude::*, EnvFilter};

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
    let filter_otel = EnvFilter::new("info")
        .add_directive("hyper=off".parse().unwrap())
        .add_directive("opentelemetry=off".parse().unwrap())
        .add_directive("tonic=off".parse().unwrap())
        .add_directive("h2=off".parse().unwrap())
        .add_directive("reqwest=off".parse().unwrap());
    let otel_layer = layer::OpenTelemetryTracingBridge::new(&provider).with_filter(filter_otel);

    // Create a new tracing::Fmt layer to print the logs to stdout. It has a
    // default filter of `info` level and above, and `debug` and above for logs
    // from OpenTelemetry crates. The filter levels can be customized as needed.
    let filter_fmt = EnvFilter::new("info").add_directive("opentelemetry=debug".parse().unwrap());
    let fmt_layer = tracing_subscriber::fmt::layer()
        .with_thread_names(true)
        .with_filter(filter_fmt);

    tracing_subscriber::registry()
        .with(otel_layer)
        .with(fmt_layer)
        .init();
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    configure_tracing();
    // Set up the Event Hub client
    let eventhub_namespace = std::env::var("EVENTHUBS_HOST")?;
    let eventhub_name = std::env::var("EVENTHUB_NAME")?;
    let credential = DeveloperToolsCredential::new(None)?;

    let client = ProducerClient::builder()
        .open(
            eventhub_namespace.as_str(),
            eventhub_name.as_str(),
            credential.clone(),
        )
        .await?;

    println!("Created producer client.");

    // Send an event to an eventhub instance directly. The message will be sent to a random partition.
    // Note that this uses an implicit builder to create the EventData being sent to the service.
    client.send_event("Hello, Event Hub!", None).await?;

    // Send an array of bytes to partition 0 of the Event Hubs instance.
    // Note that this uses an implicit builder to create the EventData being sent to the service.
    client
        .send_event(
            vec![2, 4, 8, 16],
            Some(SendEventOptions {
                partition_id: Some("0".to_string()),
            }),
        )
        .await?;

    // Send an event built using the `EventData` builder which allows for more control over the event.
    // This message will be sent to a random partition.
    client
        .send_event(
            EventData::builder()
                .with_content_type("text/plain".to_string())
                .with_correlation_id(Uuid::new_v4())
                .with_body("This is some text")
                .add_property("Event Property".to_string(), "Property Value")
                .add_property("Pi".to_string(), f32::consts::PI)
                .add_property("Binary".to_string(), vec![0x08, 0x09, 0x0A])
                .build(),
            None,
        )
        .await?;

    println!("Sent messages. Closing client.");

    client.close().await?;
    Ok(())
}

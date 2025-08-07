// Copyright (c) Microsoft Corporation. All Rights reserved
// Licensed under the MIT license.
// cspell: ignore callsite

//! This sample shows using the `tracing` crate to log messages to Azure Event Hubs
//! using a custom `Layer`. It demonstrates how to create a custom layer that formats log messages
//! and sends them to Event Hubs asynchronously.

use azure_identity::DeveloperToolsCredential;
use azure_messaging_eventhubs::SendEventOptions;
use azure_messaging_eventhubs::{models::EventData, ProducerClient};
use std::error::Error;
use std::fmt::Debug;
use std::sync::Arc;
use std::{env, time::SystemTime};
use tokio::sync::mpsc;
use tracing::{
    field::Visit,
    span, {Event, Level, Subscriber},
};
use tracing_subscriber::{
    fmt,
    fmt::format::FmtSpan,
    layer::{Context, Layer},
    prelude::*,
    registry::{LookupSpan, Registry},
};

/// A custom tracing layer that sends log messages to Azure Event Hubs.
struct EventHubsLayer {
    sender: mpsc::Sender<EventData>,
}

impl EventHubsLayer {
    async fn new(
        fully_qualified_namespace: &str,
        eventhub_name: &str,
    ) -> Result<Self, Box<dyn Error>> {
        let producer = ProducerClient::builder()
            .open(
                fully_qualified_namespace,
                eventhub_name,
                DeveloperToolsCredential::new(None)?,
            )
            .await?;

        let (sender, mut receiver) = mpsc::channel::<EventData>(100);
        let producer_arc = Arc::new(producer);
        let producer_clone = producer_arc.clone();

        // Spawn a task that receives log messages and sends them to EventHubs
        tokio::spawn(async move {
            while let Some(event) = receiver.recv().await {
                // When we send the event, specify partition 0 so we can receive
                // all the events - otherwise they will be distributed across all partitions.
                if let Err(err) = producer_clone
                    .send_event(
                        event,
                        Some(SendEventOptions {
                            partition_id: Some("0".to_string()),
                        }),
                    )
                    .await
                {
                    eprintln!("Failed to send event to EventHubs: {err}");
                }
            }
        });

        Ok(Self { sender })
    }
}

/// Constants for property names in the EventData.
/// These constants are used to set properties in the EventData object.
const LOG_LEVEL_PROPERTY: &str = "log_level";
const NAME_PROPERTY: &str = "name";
const TARGET_PROPERTY: &str = "target";
const MODULE_PATH_PROPERTY: &str = "module_path";
const FILE_PROPERTY: &str = "file";
const LINE_PROPERTY: &str = "line";
const FIELDS_PROPERTY: &str = "fields";
const IS_SPAN_PROPERTY: &str = "is_span";
const IS_EVENT_PROPERTY: &str = "is_event";
const TIMESTAMP_PROPERTY: &str = "timestamp";
const EVENT_TYPE_PROPERTY: &str = "event_type";
const SPAN_ID_PROPERTY: &str = "span_id";

const EVENT_TYPE: &str = "event";
const SPAN_OPEN: &str = "span_open";
const SPAN_CLOSE: &str = "span_close";

impl<S> Layer<S> for EventHubsLayer
where
    S: Subscriber + for<'a> LookupSpan<'a>,
{
    fn on_event(&self, event: &Event<'_>, _ctx: Context<'_, S>) {
        // Format the event into a string
        let mut visitor = EventVisitor {
            buffer: String::new(),
        };

        event.record(&mut visitor);

        // Add timestamp, level, and target
        let metadata = event.metadata();

        // Build an EventData object containing both the event body and properties reflecting the event's metadata.
        let mut event_data_builder = EventData::builder()
            .with_body(visitor.buffer)
            .add_property(EVENT_TYPE_PROPERTY.into(), EVENT_TYPE)
            .add_property(LOG_LEVEL_PROPERTY.into(), metadata.level().as_str())
            .add_property(
                TIMESTAMP_PROPERTY.to_string(),
                SystemTime::now()
                    .duration_since(SystemTime::UNIX_EPOCH)
                    .unwrap()
                    .as_secs(),
            )
            .add_property(TARGET_PROPERTY.to_string(), metadata.target())
            .add_property(NAME_PROPERTY.to_string(), metadata.name())
            .add_property(FIELDS_PROPERTY.to_string(), metadata.fields().to_string())
            .add_property(IS_EVENT_PROPERTY.to_string(), metadata.is_event())
            .add_property(IS_SPAN_PROPERTY.to_string(), metadata.is_span());

        // Handle optional metadata fields.
        if let Some(module_path) = metadata.module_path() {
            event_data_builder =
                event_data_builder.add_property(MODULE_PATH_PROPERTY.to_string(), module_path);
        }
        if let Some(file) = metadata.file() {
            event_data_builder = event_data_builder.add_property(FILE_PROPERTY.to_string(), file);
        }
        if let Some(line) = metadata.line() {
            event_data_builder = event_data_builder.add_property(LINE_PROPERTY.to_string(), line);
        }

        let event_data = event_data_builder.build();

        // Send the event to Event Hubs asynchronously
        self.sender.try_send(event_data).unwrap_or_else(|_| {
            eprintln!("Failed to send event to EventHubs");
        });
    }

    fn on_new_span(&self, attrs: &span::Attributes<'_>, id: &span::Id, _ctx: Context<'_, S>) {
        let event_data = EventData::builder()
            .add_property(EVENT_TYPE_PROPERTY.into(), SPAN_OPEN)
            .add_property(LOG_LEVEL_PROPERTY.into(), attrs.metadata().level().as_str())
            .add_property(SPAN_ID_PROPERTY.into(), id.into_u64())
            .add_property(
                TIMESTAMP_PROPERTY.to_string(),
                SystemTime::now()
                    .duration_since(SystemTime::UNIX_EPOCH)
                    .unwrap()
                    .as_secs(),
            )
            .add_property(NAME_PROPERTY.to_string(), attrs.metadata().name())
            .build();
        self.sender.try_send(event_data).unwrap_or_else(|_| {
            eprintln!("Failed to send span open event to EventHubs");
        });
    }
    fn on_close(&self, id: span::Id, _ctx: Context<'_, S>) {
        // This method is called when a span is closed.
        let event_data = EventData::builder()
            .add_property(EVENT_TYPE_PROPERTY.into(), SPAN_CLOSE)
            .add_property(SPAN_ID_PROPERTY.into(), id.into_u64())
            .add_property(
                TIMESTAMP_PROPERTY.to_string(),
                SystemTime::now()
                    .duration_since(SystemTime::UNIX_EPOCH)
                    .unwrap()
                    .as_secs(),
            )
            .build();
        self.sender.try_send(event_data).unwrap_or_else(|_| {
            eprintln!("Failed to send span close event to EventHubs");
        });
    }
}

/// A visitor that formats log messages and collects them into a buffer.
struct EventVisitor {
    buffer: String,
}

impl Visit for EventVisitor {
    fn record_str(&mut self, field: &tracing::field::Field, value: &str) {
        // Append the field name and value to the buffer
        self.buffer
            .push_str(&format!("{}: {}", field.name(), value));
    }

    fn record_debug(&mut self, field: &tracing::field::Field, value: &dyn std::fmt::Debug) {
        self.buffer
            .push_str(&format!("{}: {:?}", field.name(), value));
    }
}

/// Test struct to demonstrate structured data logging.
#[derive(Debug)]
struct StructuredData {
    body: String,
    properties: std::collections::HashMap<String, String>,
}

impl std::fmt::Display for StructuredData {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "StructuredData: {}", self.body)
    }
}

/// Enable a tracing subscriber that sends logs to the Azure Event Hubs service.
async fn enable_eventhubs_logging() -> Result<(), Box<dyn Error>> {
    // Get EventHubs fully qualified domain name and hub name from environment variables
    let fully_qualified_domain_name =
        env::var("EVENTHUBS_HOST").expect("EVENTHUBS_HOST must be set");
    let eventhub_name = env::var("EVENTHUB_NAME").expect("EVENTHUB_NAME must be set");

    // Create our custom EventHubsLayer
    let eventhubs_layer = EventHubsLayer::new(&fully_qualified_domain_name, &eventhub_name).await?;

    // Set up tracing subscriber with both console and EventHubs outputs. Note that the components of the EventHubs service (which also uses tracing)
    // need to be disabled.
    let subscriber = Registry::default()
        .with(fmt::layer().with_target(true).with_span_events(
            FmtSpan::NEW | FmtSpan::ENTER | FmtSpan::EXIT | FmtSpan::CLOSE | FmtSpan::FULL,
        ))
        // Add filters to exclude the AMQP and EventHubs logs to avoid recursive logging.
        .with(
            tracing_subscriber::EnvFilter::from_default_env()
                .add_directive("azure_messaging_eventhubs=off".parse()?)
                .add_directive("azure_core_amqp=off".parse()?)
                .add_directive("fe2o3_amqp=off".parse()?),
        )
        .with(eventhubs_layer);

    // Initialize the global tracing subscriber
    tracing::subscriber::set_global_default(subscriber)?;
    Ok(())
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn Error>> {
    enable_eventhubs_logging().await?;

    // Application code
    tracing::info!("Application started");
    // ... your application code here ...
    tracing::info!("Processing some data");
    tracing::warn!("This is a warning message");
    tracing::error!("This is an error message");

    let mut data = StructuredData {
        body: "This is a structured log message".to_string(),
        properties: std::collections::HashMap::new(),
    };

    data.properties
        .insert("key1".to_string(), "value1".to_string());
    data.properties
        .insert("key2".to_string(), "value2".to_string());

    tracing::info!("Sending structured data to EventHubs");
    tracing::info!("Structured data: {:?}", data);
    tracing::event!(Level::TRACE, "Structured data2: {}", data);

    tracing::info!(field1 = 1, field2 = "string", "logged a couple fields");

    tracing::span!(Level::TRACE, "example_span").in_scope(|| {
        tracing::info!("Inside a span");
        tracing::debug!("Debug message inside a span");
    });
    tracing::info!("Exiting the span");

    // Sleep to allow logs to be sent
    tokio::time::sleep(std::time::Duration::from_secs(5)).await;

    tracing::info!("Application shutting down");
    Ok(())
}

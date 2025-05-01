// Copyright (c) Microsoft Corporation. All Rights reserved
// Licensed under the MIT license.
// cspell: ignore callsite

//! This sample shows using the `tracing` crate to log messages to Azure Event Hubs
//! using a custom `Layer`. It demonstrates how to create a custom layer that formats log messages
//! and sends them to Event Hubs asynchronously.

use azure_identity::DefaultAzureCredential;
use azure_messaging_eventhubs::{models::EventData, ProducerClient};
use std::error::Error;
use std::sync::Arc;
use std::{env, time::SystemTime};
use tokio::sync::mpsc;
use tracing::field::Visit;
use tracing::{Event, Level, Subscriber};
use tracing_subscriber::{
    fmt,
    layer::{Context, Layer},
    prelude::*,
    registry::{LookupSpan, Registry},
};

struct EventHubsLayer {
    sender: mpsc::Sender<EventData>,
}

struct EventVisitor {
    buffer: String,
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
                DefaultAzureCredential::new()?,
            )
            .await?;

        let (sender, mut receiver) = mpsc::channel::<EventData>(100);
        let producer_arc = Arc::new(producer);
        let producer_clone = producer_arc.clone();

        // Spawn a task that receives log messages and sends them to EventHubs
        tokio::spawn(async move {
            while let Some(event) = receiver.recv().await {
                println!("Sending event to EventHubs: {:?}", event);
                if let Err(err) = producer_clone.send_event(event, None).await {
                    eprintln!("Failed to send event to EventHubs: {err}");
                }
            }
        });

        Ok(Self { sender })
    }
}

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
        let buffer = visitor.buffer;

        // Add timestamp, level, and target
        let metadata = event.metadata();

        let mut event_data_builder = EventData::builder()
            .with_body(buffer)
            .add_property(LOG_LEVEL_PROPERTY.to_string(), metadata.level().as_str())
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
        // Send to EventHubs asynchronously
        self.sender.try_send(event_data).unwrap_or_else(|_| {
            eprintln!("Failed to send event to EventHubs");
        });
    }
}

impl Visit for EventVisitor {
    fn record_str(&mut self, field: &tracing::field::Field, value: &str) {
        println!("record_str: {}: {}", field.name(), value);
        // Append the field name and value to the buffer
        self.buffer
            .push_str(&format!("{}: {}", field.name(), value));
    }

    fn record_debug(&mut self, field: &tracing::field::Field, value: &dyn std::fmt::Debug) {
        println!("record_debug: {}: {:?}", field.name(), value);
        self.buffer
            .push_str(&format!("{}: {:?}", field.name(), value));
    }
}

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

#[tokio::main]
async fn main() -> Result<(), Box<dyn Error>> {
    // Get EventHubs connection string and hub name from environment variables
    let fully_qualified_domain_name =
        env::var("EVENTHUBS_HOST").expect("EVENTHUBS_HOST must be set");
    let eventhub_name = env::var("EVENTHUB_NAME").expect("EVENTHUB_NAME must be set");

    // Create our custom EventHubsLayer
    let eventhubs_layer = EventHubsLayer::new(&fully_qualified_domain_name, &eventhub_name).await?;

    // Set up tracing subscriber with both console and EventHubs outputs
    let subscriber = Registry::default()
        .with(fmt::layer().with_target(true))
        .with(eventhubs_layer)
        .with(
            tracing_subscriber::EnvFilter::from_default_env()
                .add_directive(Level::DEBUG.into())
                .add_directive("azure_messaging_eventhubs=off".parse()?) // disable eventhubs logs because they become recursive.
                .add_directive("azure_core_amqp=off".parse()?) // disable AMQP logs because they become recursive.
                .add_directive("fe2o3_amqp=off".parse()?), // disable fe2o3 AMQP logs because they become recursive.
        );

    tracing::subscriber::set_global_default(subscriber)?;

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
    tracing::info!("Structured data2: {}", data);

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

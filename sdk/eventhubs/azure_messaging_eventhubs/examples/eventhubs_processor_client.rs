// Copyright (c) Microsoft Corporation.
// Licensed under the MIT license.

// cspell: words checkpointing

//! This sample demonstrates how to use an Event Hubs Processor to manage receiving
//! events from an Event Hub partition and checkpointing the events that have been
//! processed.

use azure_identity::DeveloperToolsCredential;
use azure_messaging_eventhubs::{ConsumerClient, EventProcessor, InMemoryCheckpointStore};
use futures::StreamExt;
use std::sync::Arc;
use tokio::sync::Mutex as AsyncMutex;

struct BackgroundProcessor {
    processor: Arc<EventProcessor>,
    background_task:
        AsyncMutex<Option<tokio::task::JoinHandle<azure_messaging_eventhubs::Result<()>>>>,
}

impl BackgroundProcessor {
    async fn new(processor: Arc<EventProcessor>) -> Result<Self, Box<dyn std::error::Error>> {
        Ok(Self {
            background_task: AsyncMutex::new(None),
            processor: processor.clone(),
        })
    }

    async fn start(&self) -> Result<(), Box<dyn std::error::Error>> {
        // Start the background task to process events
        let mut task = self.background_task.lock().await;
        //        let mut task = task.as_mut();
        let processor = self.processor.clone();
        task.replace(tokio::spawn(async move { processor.run().await }));
        Ok(())
    }

    async fn stop(self) -> Result<(), Box<dyn std::error::Error>> {
        println!("Shutting background processor down.");
        self.processor.shutdown().await?;
        // Stop the background task
        let mut task = self.background_task.lock().await;
        if let Some(handle) = task.take() {
            // Wait for the task to finish
            println!("Waiting for background task to finish.");
            handle.await??;
        }
        Ok(())
    }
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    // Initialize tracing subscriber from environment.
    tracing_subscriber::fmt().init();

    // Set up the Event Hub client
    let eventhub_namespace = std::env::var("EVENTHUBS_HOST")?;
    let eventhub_name = std::env::var("EVENTHUB_NAME")?;

    let consumer = ConsumerClient::builder()
        .open(
            eventhub_namespace.as_str(),
            eventhub_name,
            DeveloperToolsCredential::new(None)?.clone(),
        )
        .await?;

    println!("Opened consumer client");

    let checkpoint_store = Arc::new(InMemoryCheckpointStore::new());
    let processor = EventProcessor::builder()
        .build(consumer, checkpoint_store)
        .await?;
    let background_processor = BackgroundProcessor::new(processor.clone()).await?;

    background_processor
        .start()
        .await
        .expect("Failed to start background processor");
    println!("Started background processor");

    let partition_client = processor.next_partition_client().await?;
    println!(
        "Received a partition client for partition : {}",
        partition_client.get_partition_id()
    );

    let mut event_stream = partition_client.stream_events();
    let mut event_count = 0;
    while let Some(event) = event_stream.next().await {
        println!("Received message {event_count}");
        event_count += 1;
        if event_count > 10 {
            println!("Received 10 events, stopping the processor.");
            break;
        }
        match event {
            Ok(event) => {
                println!("Received event: {:?}", event);
                // Process the received event
                println!("Partition key: {:?}", event.partition_key());
                println!("Event offset: {:?}", event.offset());
                println!("Event sequence number: {:?}", event.sequence_number());
            }
            Err(err) => {
                // Handle the error
                eprintln!("Error receiving event: {:?}", err);
            }
        }
    }

    println!("Stopping background processor");
    background_processor.stop().await?;
    println!("Stopped background processor");

    // Close the processor
    if let Ok(processor) = Arc::try_unwrap(processor) {
        processor.close().await?;
        println!("Closed processor");
    } else {
        println!("Processor still has references, not closing.");
    }

    Ok(())
}

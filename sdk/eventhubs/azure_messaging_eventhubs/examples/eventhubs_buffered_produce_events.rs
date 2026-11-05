// Copyright (c) Microsoft Corporation.
// Licensed under the MIT license.

// cspell: ignore retryable

//! This sample shows how to publish events with the buffered producer client.
//!
//! The caller enqueues single events. The client buffers them, groups them into
//! batches for each partition, and publishes them in the background.
//!
//! A successful enqueue means only that the local buffer accepted the event. It
//! does not mean that Event Hubs accepted the event. The client reports the real
//! outcome through the two handlers below.

use azure_core::time::Duration;
use azure_identity::DeveloperToolsCredential;
use azure_messaging_eventhubs::{BufferedProducerClient, EnqueueEventOptions};
use std::sync::{
    atomic::{AtomicUsize, Ordering},
    Arc,
};

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let eventhub_namespace = std::env::var("EVENTHUBS_HOST")?;
    let eventhub_name = std::env::var("EVENTHUB_NAME")?;
    let credential = DeveloperToolsCredential::new(None)?;

    // The handlers report the outcome of each batch. Count the events so the
    // sample can print a summary at the end.
    let accepted = Arc::new(AtomicUsize::new(0));
    let rejected = Arc::new(AtomicUsize::new(0));

    let for_success = accepted.clone();
    let for_failure = rejected.clone();

    let producer = BufferedProducerClient::builder()
        // Send a batch that is not full after this time.
        .with_max_wait_time(Duration::seconds(1))
        // Wait for space once a partition buffer holds this many events.
        .with_max_buffered_event_count_per_partition(1500)
        .with_on_send_succeeded(move |context| {
            let accepted = for_success.clone();
            async move {
                accepted.fetch_add(context.events.len(), Ordering::AcqRel);
                println!(
                    "The service accepted {} events on partition {}.",
                    context.events.len(),
                    context.partition_id
                );
            }
        })
        // A handler for failed batches is required. The client calls it only
        // after the retry policy is exhausted, or when the error is not
        // retryable. The client does not enqueue the events again, so the
        // application decides what to do with them.
        .with_on_send_failed(move |context| {
            let rejected = for_failure.clone();
            async move {
                rejected.fetch_add(context.events.len(), Ordering::AcqRel);
                eprintln!(
                    "{} events failed on partition {}: {}",
                    context.events.len(),
                    context.partition_id,
                    context.error
                );
            }
        })
        .open(
            eventhub_namespace.as_str(),
            eventhub_name.as_str(),
            credential.clone(),
        )
        .await?;

    // The client assigns these events to the partitions in round-robin order.
    for index in 0..100 {
        producer
            .enqueue_event(format!("automatic event {index}"), None)
            .await?;
    }

    // These events all go to partition 0.
    producer
        .enqueue_events(
            vec!["first", "second", "third"],
            Some(EnqueueEventOptions {
                partition_id: Some("0".to_string()),
                ..Default::default()
            }),
        )
        .await?;

    // Every event with the same key goes to the same partition.
    producer
        .enqueue_event(
            "an event for one customer",
            Some(EnqueueEventOptions {
                partition_key: Some("customer-17".to_string()),
                ..Default::default()
            }),
        )
        .await?;

    println!(
        "{} events are waiting in the buffer.",
        producer.total_buffered_event_count()
    );

    // The flush completes once every event that the client accepted before this
    // call reaches a terminal outcome.
    producer.flush().await?;

    // A graceful close sends what is left, then it releases the connection. Use
    // `abort` instead to shut down at once and abandon the buffered events.
    producer.close().await?;

    println!(
        "Done. The service accepted {} events and rejected {} events.",
        accepted.load(Ordering::Acquire),
        rejected.load(Ordering::Acquire)
    );
    Ok(())
}

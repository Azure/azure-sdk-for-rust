// Copyright (c) Microsoft Corporation. All rights reserved.
// Licensed under the MIT license.

mod in_memory_checkpoint_store;
use azure_core_test::{recorded, TestContext};
use azure_messaging_eventhubs::{ConsumerClient, EventProcessor, ProcessorStrategy};
use in_memory_checkpoint_store::InMemoryCheckpointStore;
use std::sync::Arc;
use std::time::Duration;
use tokio::task::JoinHandle;
use tracing::info;
mod common;

#[recorded::test(live)]
async fn start_processor(ctx: TestContext) -> azure_core::Result<()> {
    common::setup();

    let recording = ctx.recording();

    let consumer_client = ConsumerClient::builder()
        .open(
            recording.var("EVENTHUBS_HOST", None).as_str(),
            recording.var("EVENTHUB_NAME", None).as_str(),
            recording.credential().clone(),
        )
        .await?;

    let event_processor = EventProcessor::builder()
        .with_load_balancing_strategy(ProcessorStrategy::Balanced)
        .with_update_interval(Duration::from_secs(5))
        .with_partition_expiration_duration(Duration::from_secs(10))
        .with_prefetch(300)
        .build(
            Arc::new(consumer_client),
            Arc::new(InMemoryCheckpointStore::new()),
        )?;

    {
        const TIMEOUT: Duration = Duration::from_secs(30);
        let event_processor_clone = event_processor.clone();
        let jh = tokio::spawn(async move { event_processor_clone.run().await });
        let timeout_handle = tokio::spawn(async move {
            tokio::time::sleep(TIMEOUT).await;
            info!("Timeout reached");
        });

        info!("Started event processor");
        info!("Waiting for event processor to finish");
        info!("Timeout set to {:?}", TIMEOUT);

        tokio::select! {
            result = jh => {
                info!("Event processor finished: {:?}", result);
                if let Err(e) = result {
                    info!("Event processor failed: {:?}", e);
                } else {
                    info!("Event processor finished successfully");
                }
            }
            _ = timeout_handle => {
                info!("Timeout reached, event processor may not have started");
            }
        }
    }

    info!("Shutdown signal sent to event processor");
    let r = event_processor.shutdown().await;
    if let Err(e) = r {
        info!("Failed to shutdown event processor: {:?}", e);
    } else {
        info!("Event processor shutdown sent successfully");
    }

    info!("Sleeping to let the processor task finish.");
    tokio::time::sleep(Duration::from_secs(5)).await;

    info!("Closing the processor.");
    let processor = Arc::into_inner(event_processor);
    if let Some(processor) = processor {
        info!("Stopping event processor");
        processor.close().await?;
    } else {
        info!("Event processor still running..");
    }

    Ok(())
}

async fn create_processor(ctx: TestContext) -> azure_core::Result<Arc<EventProcessor>> {
    common::setup();

    let recording = ctx.recording();

    let consumer_client = ConsumerClient::builder()
        .open(
            recording.var("EVENTHUBS_HOST", None).as_str(),
            recording.var("EVENTHUB_NAME", None).as_str(),
            recording.credential().clone(),
        )
        .await?;

    EventProcessor::builder()
        .with_load_balancing_strategy(ProcessorStrategy::Balanced)
        .with_update_interval(Duration::from_secs(5))
        .with_partition_expiration_duration(Duration::from_secs(30))
        .with_prefetch(300)
        .build(
            Arc::new(consumer_client),
            Arc::new(InMemoryCheckpointStore::new()),
        )
}

async fn start_processor_running(
    event_processor: &Arc<EventProcessor>,
) -> JoinHandle<azure_core::Result<()>> {
    let event_processor_clone = event_processor.clone();
    let jh = tokio::spawn(async move { event_processor_clone.run().await });
    // Wait 5 seconds to give the processor a chance to get running.
    tokio::time::sleep(Duration::from_secs(5)).await;
    jh
}

#[recorded::test(live)]
async fn get_next_partition_client(ctx: TestContext) -> azure_core::Result<()> {
    let processor = create_processor(ctx).await?;

    let running_processor = start_processor_running(&processor).await;

    let partition_client = processor
        .next_partition_client()
        .await
        .expect("Failed to get next partition client");
    info!(
        "Received partition client for partition {}",
        partition_client.get_partition_id()
    );

    running_processor.abort();
    let _ = running_processor.await;
    info!("Processor task aborted");

    Ok(())
}

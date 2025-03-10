// Copyright (c) Microsoft Corporation. All rights reserved.
// Licensed under the MIT license.

mod in_memory_checkpoint_store;
use azure_core_test::{recorded, TestContext};
use azure_messaging_eventhubs::{ConsumerClient, EventProcessor, ProcessorStrategy};
use in_memory_checkpoint_store::InMemoryCheckpointStore;
use std::sync::Arc;
use std::time::Duration;
use tracing::event;

#[recorded::test]
async fn start_processor(ctx: TestContext) -> azure_core::Result<()> {
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
        .with_update_interval(Duration::from_secs(30))
        .with_partition_expiration_duration(Duration::from_secs(10))
        .with_prefetch(300)
        .build(
            Arc::new(consumer_client),
            Arc::new(InMemoryCheckpointStore::new()),
        )?;

    {
        //            let event_processor_clone = Arc::clone(&event_processor);
        let event_processor_clone = event_processor.clone();
        let jh = tokio::spawn(async move { event_processor_clone.run().await });

        let r = jh.await;

        match r {
            Ok(_) => {
                event!(tracing::Level::INFO, "Event processor ran successfully");
            }
            Err(e) => {
                event!(
                    tracing::Level::ERROR,
                    "Failed to run event processor: {}",
                    e
                );
            }
        }
    }

    event_processor.close().await?;

    // Start the event processor
    // Wait for the event processor to finish
    //        let partition_manager = event_processor.run().await?;

    Ok(())
}

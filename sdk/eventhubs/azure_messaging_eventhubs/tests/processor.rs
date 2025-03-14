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
        )
        .await?;

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

async fn create_consumer_client(ctx: TestContext) -> azure_core::Result<Arc<ConsumerClient>> {
    common::setup();

    let recording = ctx.recording();

    Ok(Arc::new(
        ConsumerClient::builder()
            .open(
                recording.var("EVENTHUBS_HOST", None).as_str(),
                recording.var("EVENTHUB_NAME", None).as_str(),
                recording.credential().clone(),
            )
            .await?,
    ))
}

async fn create_processor(
    consumer_client: Arc<ConsumerClient>,
    update_interval: Duration,
) -> azure_core::Result<Arc<EventProcessor>> {
    EventProcessor::builder()
        .with_load_balancing_strategy(ProcessorStrategy::Balanced)
        .with_update_interval(update_interval)
        .with_partition_expiration_duration(Duration::from_secs(120))
        .with_prefetch(300)
        .build(consumer_client, Arc::new(InMemoryCheckpointStore::new()))
        .await
}

async fn start_processor_running(
    event_processor: &Arc<EventProcessor>,
) -> JoinHandle<azure_core::Result<()>> {
    let event_processor_clone = event_processor.clone();
    tokio::spawn(async move { event_processor_clone.run().await })
}

#[recorded::test(live)]
async fn get_next_partition_client(ctx: TestContext) -> azure_core::Result<()> {
    let consumer_client = create_consumer_client(ctx).await?;
    let processor = create_processor(consumer_client, Duration::from_secs(20)).await?;

    let running_processor = start_processor_running(&processor).await;

    info!("Getting the first partition client.");
    let partition_client = processor
        .next_partition_client()
        .await
        .expect("Failed to get next partition client");
    info!(
        "Received partition client for partition {}",
        partition_client.get_partition_id()
    );

    running_processor.abort();
    info!("Processor task aborted");
    let _ = running_processor.await;
    info!("Processor task joined");

    Ok(())
}

#[recorded::test(live)]
async fn get_all_partition_clients(ctx: TestContext) -> azure_core::Result<()> {
    use std::collections::HashSet;

    common::setup();

    let consumer_client = create_consumer_client(ctx).await?;
    // The processor only adds one client as needed up to the max, so we block waiting
    // on all the clients to become available.
    let processor = create_processor(consumer_client.clone(), Duration::from_secs(3)).await?;

    let running_processor = start_processor_running(&processor).await;

    let eh_properties = consumer_client.get_eventhub_properties().await?;

    let mut found_clients = HashSet::new();
    let mut partition_clients = Vec::new();
    for partition in 0..eh_properties.partition_ids.len() {
        info!("Partition ID: {}", partition);

        let next_client = processor.next_partition_client().await?;
        if found_clients.contains(next_client.get_partition_id()) {
            panic!(
                "Duplicate partition client found: {}",
                next_client.get_partition_id()
            );
        }
        info!(
            "Received partition client for partition {}",
            next_client.get_partition_id()
        );
        found_clients.insert(next_client.get_partition_id().clone());
        partition_clients.push(next_client);
    }

    info!("Received {} partition clients", partition_clients.len());

    for client in partition_clients.iter() {
        info!(
            "Received partition client for partition {}",
            client.get_partition_id()
        );
    }

    {
        info!("Retrieving one more processor client than possible.");
        let processor_clone = processor.clone();
        tokio::select! {
            _ = tokio::spawn(async move {
                tokio::time::sleep(Duration::from_secs(5)).await;
                info!("Timeout reached");
            }) => {
                info!("Timeout reached - event processor has no more partitions.");
            }

            result = tokio::spawn(async move { processor_clone.next_partition_client().await}) => {
                match result {
                    Ok(Ok(_)) => {
                        panic!("Event processor has more partitions. It shouldn't.");
                    }
                    Ok(Err(e)) => {
                        panic!("Event processor has no more partitions: {:?}", e);
                    }
                    Err(e) => {
                        panic!("Failed to get next partition client: {:?}", e);
                    }
                }
            }
        }
    }
    // Now drop one of the partition clients.
    let partition_client = partition_clients.pop().unwrap();
    info!(
        "Dropping partition client for partition {}",
        partition_client.get_partition_id()
    );

    if let Some(partition_client) = Arc::into_inner(partition_client) {
        info!("All references to partition client dropped");
        partition_client.close().await?;
        info!("Partition client closed");
    } else {
        panic!("Partition client not dropped: Arc has multiple strong references (this should not happen).");
    }

    info!("Partition client dropped, getting another partition client.");

    // Wait for the processor to notice the partition client is dropped.
    let partition_client = tokio::select! {
        result = processor.next_partition_client() => {
            info!("Received next partition client");
            result?
        }
        _ = tokio::time::sleep(Duration::from_secs(15)) => {
            info!("Timeout reached - event processor has no more partitions.");
            return Err(azure_core::Error::message(
                azure_core::error::ErrorKind::Other,
                "Timeout waiting for next partition client"
            ));
        }
    };
    info!(
        "Received partition client for partition {}",
        partition_client.get_partition_id()
    );

    running_processor.abort();
    info!("Processor task aborted");
    let _ = running_processor.await;
    info!("Processor task joined");

    Ok(())
}

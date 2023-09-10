#![allow(unused)]

use std::path::PathBuf;

use azeventhubs::{
    consumer::{EventHubConsumerClient, EventPosition, ReadEventOptions},
    producer::{EventHubProducerClient, SendEventOptions},
    BasicRetryPolicy, ReceivedEventData,
};
use futures_util::StreamExt;

pub type Producer = EventHubProducerClient<BasicRetryPolicy>;
pub type Consumer = EventHubConsumerClient<BasicRetryPolicy>;

pub fn setup_dotenv() -> Result<PathBuf, dotenv::Error> {
    dotenv::from_filename("./sdk/messaging_eventhubs/.env")
}

pub fn and_nested_result<T, E1, E2>(
    left: Result<Result<T, E1>, E2>,
    right: Result<Result<T, E1>, E2>,
) -> Result<Result<T, E1>, E2> {
    match (left, right) {
        (Ok(Ok(_)), Ok(r)) => Ok(r),
        (Ok(_), Err(err)) => Err(err),
        (Ok(Err(err)), _) => Ok(Err(err)),
        (Err(err), _) => Err(err),
    }
}

pub async fn fill_partition(
    producer: &mut Producer,
    partition_id: &str,
    n: usize,
) -> Result<(), Box<dyn std::error::Error>> {
    let mut batch = producer.create_batch(Default::default()).await?;

    let options = SendEventOptions::default().with_partition_id(partition_id);
    for i in 0..n {
        let event = format!("Benchmark event {}", i);
        if let Err(_) = batch.try_add(event) {
            producer.send_batch(batch, options.clone()).await?;
            batch = producer.create_batch(Default::default()).await?;
        }
    }

    producer.send_batch(batch, options).await?;

    Ok(())
}

/// Fill all partition with 100 messages
pub async fn fill_partitions(
    producer: &mut Producer,
    n: usize,
) -> Result<Vec<String>, Box<dyn std::error::Error>> {
    let properties = producer.get_event_hub_properties().await?;

    for partition_id in properties.partition_ids() {
        fill_partition(producer, &partition_id, n).await?;
    }

    Ok(properties.partition_ids().to_vec())
}

pub async fn consume_events(
    mut consumer: &mut Consumer,
    partition_id: String,
    n: usize,
    read_event_options: ReadEventOptions, // TODO: benchmark different options
    process_event: impl Fn(ReceivedEventData),
) -> Result<(), azure_core::Error> {
    let starting_position = EventPosition::earliest();
    let mut stream = consumer
        .read_events_from_partition(&partition_id, starting_position, read_event_options)
        .await?;

    let mut counter = 0;
    while let Some(Ok(event)) = stream.next().await {
        process_event(event);
        counter += 1;
        if counter > n {
            break;
        }
    }
    stream.close().await?;
    Ok(())
}

pub async fn prepare_events_on_all_partitions(n: usize) -> Vec<String> {
    let connection_string = std::env::var("EVENT_HUBS_CONNECTION_STRING").unwrap();
    let event_hub_name = std::env::var("EVENT_HUB_BENCHMARK_NAME").unwrap();

    // prepare events
    let mut producer = EventHubProducerClient::new_from_connection_string(
        connection_string.clone(),
        event_hub_name.clone(),
        Default::default(),
    )
    .await
    .unwrap();
    let partitions = fill_partitions(&mut producer, n).await.unwrap();
    producer.close().await.unwrap();

    partitions
}

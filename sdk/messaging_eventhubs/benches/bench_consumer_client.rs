use std::time::Duration;

use azeventhubs::{
    consumer::{
        EventHubConsumerClient, EventHubConsumerClientOptions, EventPosition, ReadEventOptions,
    },
    producer::{EventHubProducerClient, SendEventOptions},
    BasicRetryPolicy, EventHubConnection, EventHubConnectionOptions, EventHubsRetryOptions,
    ReceivedEventData,
};
use criterion::{criterion_group, criterion_main, Criterion};
use futures_util::StreamExt;
use utils::{and_nested_result, setup_dotenv};

mod utils;

type Producer = EventHubProducerClient<BasicRetryPolicy>;
type Consumer = EventHubConsumerClient<BasicRetryPolicy>;

async fn fill_partition(
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
async fn fill_partitions(
    producer: &mut Producer,
    n: usize,
) -> Result<Vec<String>, Box<dyn std::error::Error>> {
    let properties = producer.get_event_hub_properties().await?;

    for partition_id in properties.partition_ids() {
        fill_partition(producer, &partition_id, n).await?;
    }

    Ok(properties.partition_ids().to_vec())
}

async fn prepare_events_on_all_partitions(n: usize) -> Vec<String> {
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

async fn consume_events(
    mut consumer: Consumer,
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
    consumer.close().await?;
    Ok(())
}

async fn bench_dedicated_connection_consumers(
    partitions: Vec<String>,
    n: usize,
    cache_event_count: u32,      // TODO: benchmark different cache count
    maximum_wait_time: Duration, // TODO: benchmark different wait time
) {
    let connection_string = std::env::var("EVENT_HUBS_CONNECTION_STRING").unwrap();
    let event_hub_name = std::env::var("EVENT_HUB_BENCHMARK_NAME").unwrap();
    let read_event_options = ReadEventOptions {
        cache_event_count,
        maximum_wait_time: Some(maximum_wait_time),
        ..Default::default()
    };

    let consumer_group = EventHubConsumerClient::DEFAULT_CONSUMER_GROUP_NAME;
    let retry_options = EventHubsRetryOptions {
        try_timeout: Duration::from_secs(5), // fail early for benchmark
        ..Default::default()
    };
    let client_options = EventHubConsumerClientOptions {
        retry_options,
        ..Default::default()
    };

    let mut consumers = Vec::new();
    for _ in partitions.iter() {
        let consumer = EventHubConsumerClient::new_from_connection_string(
            consumer_group,
            connection_string.clone(),
            event_hub_name.clone(),
            client_options.clone(),
        )
        .await.unwrap();
        consumers.push(consumer);
    }

    let handles = consumers.into_iter().zip(partitions)
        .into_iter()
        .map(|(consumer, partition_id)| {
            let partition_id = partition_id;
            tokio::spawn(consume_events(
                consumer,
                partition_id,
                n,
                read_event_options.clone(),
                |_| {}, // TODO: bench process event
            ))
        })
        .collect::<Vec<_>>();

    futures_util::future::join_all(handles)
        .await
        .into_iter()
        .fold(Ok(Ok(())), |acc, res| and_nested_result(acc, res))
        .unwrap()
        .unwrap();
}

async fn bench_shared_connection_consumers(
    partitions: Vec<String>,
    n: usize,
    cache_event_count: u32,      // TODO: benchmark different cache count
    maximum_wait_time: Duration, // TODO: benchmark different wait time
) {
    let connection_string = std::env::var("EVENT_HUBS_CONNECTION_STRING").unwrap();
    let event_hub_name = std::env::var("EVENT_HUB_BENCHMARK_NAME").unwrap();

    let options = EventHubConnectionOptions::default();
    let mut connection =
        EventHubConnection::new_from_connection_string(connection_string, event_hub_name, options)
            .await
            .unwrap();

    let consumer_group = EventHubConsumerClient::DEFAULT_CONSUMER_GROUP_NAME;
    let retry_options = EventHubsRetryOptions {
        try_timeout: Duration::from_secs(5), // fail early for benchmark
        ..Default::default()
    };
    let client_options = EventHubConsumerClientOptions {
        retry_options,
        ..Default::default()
    };
    let read_event_options = ReadEventOptions {
        cache_event_count,
        maximum_wait_time: Some(maximum_wait_time),
        ..Default::default()
    };

    let handles = partitions
        .into_iter()
        .map(|partition_id| {
            let consumer = EventHubConsumerClient::with_connection(
                consumer_group,
                &mut connection,
                client_options.clone(),
            );
            tokio::spawn(consume_events(
                consumer,
                partition_id,
                n,
                read_event_options.clone(),
                |_| {}, // TODO: bench process event
            ))
        })
        .collect::<Vec<_>>();

    futures_util::future::join_all(handles)
        .await
        .into_iter()
        .fold(Ok(Ok(())), |acc, res| and_nested_result(acc, res))
        .unwrap()
        .unwrap();
}

fn criterion_benchmark(c: &mut Criterion) {
    setup_dotenv().unwrap();
    let rt = tokio::runtime::Runtime::new().unwrap();

    let n = 100;
    let cache_event_count = 5;
    let maximum_wait_time = Duration::from_secs(1);
    let n_prep = (n as f32 * 1.1).ceil() as usize; // Make sure we have enough events
    let partitions = rt.block_on(prepare_events_on_all_partitions(n_prep));

    let mut bench_group = c.benchmark_group("consumer_client");
    bench_group.sample_size(10);
    bench_group.bench_function("dedicated_connection", |b| {
        b.to_async(&rt).iter(|| {
            bench_dedicated_connection_consumers(
                partitions.clone(),
                n,
                cache_event_count,
                maximum_wait_time,
            )
        })
    });
    bench_group.bench_function("shared_connection", |b| {
        b.to_async(&rt).iter(|| {
            bench_shared_connection_consumers(
                partitions.clone(),
                n,
                cache_event_count,
                maximum_wait_time,
            )
        })
    });
    bench_group.finish();
}

criterion_group!(benches, criterion_benchmark);
criterion_main!(benches);

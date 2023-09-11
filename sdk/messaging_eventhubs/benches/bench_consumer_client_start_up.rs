use std::time::Duration;

use azeventhubs::{
    consumer::{EventHubConsumerClient, EventHubConsumerClientOptions, ReadEventOptions},
    EventHubConnection, EventHubConnectionOptions, EventHubsRetryOptions,
};
use criterion::{criterion_group, criterion_main, Criterion};
use utils::setup_dotenv;

mod utils;
use utils::*;

async fn bench_dedicated_connection_consumers_concurrent(
    partitions: Vec<String>,
    n: usize,
    maximum_wait_time: Duration, // TODO: benchmark different wait time
) {
    let connection_string = std::env::var("EVENT_HUBS_CONNECTION_STRING").unwrap();
    let event_hub_name = std::env::var("EVENT_HUB_BENCHMARK_NAME").unwrap();
    let read_event_options = ReadEventOptions {
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

    let futures = partitions
        .into_iter()
        .map(|partition_id| async {
            let partition_id = partition_id;
            let mut consumer = EventHubConsumerClient::new_from_connection_string(
                consumer_group,
                connection_string.clone(),
                event_hub_name.clone(),
                client_options.clone(),
            )
            .await?;
            consume_events(
                &mut consumer,
                partition_id,
                n,
                read_event_options.clone(),
                |_| {}, // TODO: bench process event
            )
            .await?;
            consumer.close().await
        })
        .collect::<Vec<_>>();

    futures_util::future::join_all(futures)
        .await
        .into_iter()
        .fold(Ok(()), |acc, res| acc.and(res))
        .unwrap();
}

async fn bench_dedicated_connection_consumers_sequential(
    partitions: Vec<String>,
    n: usize,
    maximum_wait_time: Duration, // TODO: benchmark different wait time
) {
    let connection_string = std::env::var("EVENT_HUBS_CONNECTION_STRING").unwrap();
    let event_hub_name = std::env::var("EVENT_HUB_BENCHMARK_NAME").unwrap();
    let read_event_options = ReadEventOptions {
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
        .await
        .unwrap();
        consumers.push(consumer);
    }

    let futures = consumers
        .into_iter()
        .zip(partitions)
        .into_iter()
        .map(|(mut consumer, partition_id)| async {
            let partition_id = partition_id;
            consume_events(
                &mut consumer,
                partition_id,
                n,
                read_event_options.clone(),
                |_| {}, // TODO: bench process event
            )
            .await?;
            consumer.close().await
        })
        .collect::<Vec<_>>();

    futures_util::future::join_all(futures)
        .await
        .into_iter()
        .fold(Ok(()), |acc, res| acc.and(res))
        .unwrap();
}

async fn bench_shared_connection_consumers(
    partitions: Vec<String>,
    n: usize,
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
        maximum_wait_time: Some(maximum_wait_time),
        ..Default::default()
    };

    let futures = partitions
        .into_iter()
        .map(|partition_id| {
            let mut consumer = EventHubConsumerClient::with_connection(
                consumer_group,
                &mut connection,
                client_options.clone(),
            );
            async {
                consume_events(
                    &mut consumer,
                    partition_id,
                    n,
                    read_event_options.clone(),
                    |_| {}, // TODO: bench process event
                )
                .await?;
                consumer.close().await
            }
        })
        .collect::<Vec<_>>();

    futures_util::future::join_all(futures)
        .await
        .into_iter()
        .fold(Ok(()), |acc, res| acc.and(res))
        .unwrap();
}

fn criterion_benchmark(c: &mut Criterion) {
    setup_dotenv().unwrap();
    let rt = tokio::runtime::Runtime::new().unwrap();

    // Receive only one event because we are only interested in the time taken to setup the consumer
    // clients.
    let sample_size = 10;
    let n = 1;
    let maximum_wait_time = Duration::from_secs(1);
    let n_prep = sample_size * n;
    let partitions = rt.block_on(prepare_events_on_all_partitions(n_prep));

    let mut bench_group = c.benchmark_group("consumer_client_start_up");
    bench_group.sample_size(sample_size);
    bench_group.bench_function("dedicated_connection_concurrent", |b| {
        b.to_async(&rt).iter(|| {
            bench_dedicated_connection_consumers_concurrent(
                partitions.clone(),
                n,
                maximum_wait_time,
            )
        })
    });
    bench_group.bench_function("dedicated_connection_sequential", |b| {
        b.to_async(&rt).iter(|| {
            bench_dedicated_connection_consumers_sequential(
                partitions.clone(),
                n,
                maximum_wait_time,
            )
        })
    });
    bench_group.bench_function("shared_connection", |b| {
        b.to_async(&rt).iter(|| {
            bench_shared_connection_consumers(
                partitions.clone(),
                n,
                maximum_wait_time,
            )
        })
    });
    bench_group.finish();
}

criterion_group!(benches, criterion_benchmark);
criterion_main!(benches);

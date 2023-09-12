use std::{
    sync::{Arc, Mutex},
    time::Duration,
};

use azeventhubs::{
    consumer::{EventHubConsumerClient, EventHubConsumerClientOptions, ReadEventOptions},
    EventHubsRetryOptions,
};
use criterion::{criterion_group, criterion_main, Criterion};
use utils::{
    consume_events, create_dedicated_connection_consumer, create_shared_connection_consumer,
    setup_dotenv, Consumer,
};

mod utils;

// TODO: This will still have cost establishing new AMQP Session and AMQP Link.
async fn bench_connection_consumer_streams(
    consumer_clients: Arc<Mutex<Vec<Consumer>>>,
    partitions: Vec<String>,
    n: usize,
) {
    // There should be just one bench running at a time.
    // The use of Arc is to fit the benchmark API.
    let mut consumers = consumer_clients.try_lock().unwrap();

    let read_event_options = ReadEventOptions::default();

    let futures = consumers
        .iter_mut()
        .zip(partitions.into_iter())
        .map(|(c, partition_id)| {
            consume_events(c, partition_id, n, read_event_options.clone(), |_| {})
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
    let connection_string = std::env::var("EVENT_HUBS_CONNECTION_STRING").unwrap();
    let event_hub_name = std::env::var("EVENT_HUB_BENCHMARK_NAME").unwrap();

    let rt = tokio::runtime::Runtime::new().unwrap();

    // Use a small number because we are benchmarking the start up time.
    let sample_size = 10;
    let n = 1;
    let n_prep = 5 * sample_size * n;
    let partitions = rt.block_on(utils::prepare_events_on_all_partitions(n_prep));

    let consumer_group = EventHubConsumerClient::DEFAULT_CONSUMER_GROUP_NAME;
    let retry_options = EventHubsRetryOptions {
        try_timeout: Duration::from_secs(1), // fail early for benchmark
        ..Default::default()
    };
    let client_options = EventHubConsumerClientOptions {
        retry_options,
        ..Default::default()
    };

    // Bench dedicated connection consumer streams
    let consumer_clients = rt
        .block_on(create_dedicated_connection_consumer(
            partitions.len(),
            consumer_group,
            connection_string.clone(),
            event_hub_name.clone(),
            client_options.clone(),
        ))
        .unwrap();
    let consumer_clients = Arc::new(Mutex::new(consumer_clients));

    let mut bench_group = c.benchmark_group("event_stream_start_up");
    bench_group.sample_size(sample_size);
    bench_group.measurement_time(Duration::from_millis(1000)); // this has effect on the number of iterations
    bench_group.bench_function("dedicated_connection_consumer_stream", |b| {
        b.to_async(&rt).iter(|| {
            bench_connection_consumer_streams(consumer_clients.clone(), partitions.clone(), n)
        })
    });
    rt.block_on(async {
        let consumers = Arc::try_unwrap(consumer_clients)
            .unwrap()
            .into_inner()
            .unwrap();
        for consumer in consumers {
            consumer.close().await.unwrap();
        }
    });

    // Bench shared connection consumer streams
    let (connection, consumer_clients) = rt
        .block_on(create_shared_connection_consumer(
            partitions.len(),
            consumer_group,
            connection_string,
            event_hub_name,
            client_options,
        ))
        .unwrap();
    let consumer_clients = Arc::new(Mutex::new(consumer_clients));

    bench_group.bench_function("shared_connection_consumer_stream", |b| {
        b.to_async(&rt).iter(|| {
            bench_connection_consumer_streams(consumer_clients.clone(), partitions.clone(), n)
        })
    });
    rt.block_on(async {
        let consumers = Arc::try_unwrap(consumer_clients)
            .unwrap()
            .into_inner()
            .unwrap();
        for consumer in consumers {
            consumer.close().await.unwrap();
        }
        connection.close().await.unwrap();
    });
}

criterion_group!(benches, criterion_benchmark);
criterion_main!(benches);

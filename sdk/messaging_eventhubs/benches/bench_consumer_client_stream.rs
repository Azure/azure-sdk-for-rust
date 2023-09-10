use std::{
    sync::{Arc, Mutex},
    time::Duration,
};

use azeventhubs::{
    consumer::{EventHubConsumerClient, EventHubConsumerClientOptions, ReadEventOptions},
    EventHubsRetryOptions, EventHubConnection,
};
use criterion::{criterion_group, criterion_main, Criterion};
use utils::{consume_events, setup_dotenv, Consumer};

mod utils;

// TODO: This will still have cost establishing new AMQP Session and AMQP Link.
async fn bench_connection_consumer_streams(
    consumer_clients: Arc<Mutex<Vec<Consumer>>>,
    partitions: Vec<String>,
    n: usize,
    cache_event_count: u32,
    maximum_wait_time: Duration,
) {
    // There should be just one bench running at a time.
    // The use of Arc is to fit the benchmark API.
    let mut consumers = consumer_clients.try_lock().unwrap();

    let read_event_options = ReadEventOptions::default()
        .with_cache_event_count(cache_event_count)
        .with_maximum_wait_time(maximum_wait_time);

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

    let n = 100;
    let cache_event_count = 5;
    let maximum_wait_time = Duration::from_secs(1);
    // let n_prep = (n as f32 * 1.1).ceil() as usize; // Make sure we have enough events
    let n_prep = 2 * n;
    let partitions = rt.block_on(utils::prepare_events_on_all_partitions(n_prep));

    let consumer_group = EventHubConsumerClient::DEFAULT_CONSUMER_GROUP_NAME;
    let retry_options = EventHubsRetryOptions {
        try_timeout: Duration::from_secs(5), // fail early for benchmark
        ..Default::default()
    };
    let client_options = EventHubConsumerClientOptions {
        retry_options,
        ..Default::default()
    };

    // Bench dedicated connection consumer streams
    let consumer_clients = rt.block_on(async {
        let mut consumer_clients = Vec::new();
        for _ in partitions.iter() {
            let consumer = EventHubConsumerClient::new_from_connection_string(
                consumer_group,
                connection_string.clone(),
                event_hub_name.clone(),
                client_options.clone(),
            )
            .await?;
            consumer_clients.push(consumer);
        }
        Result::<Vec<Consumer>, azure_core::Error>::Ok(consumer_clients)
    }).unwrap();
    let consumer_clients = Arc::new(Mutex::new(consumer_clients));

    let mut bench_group = c.benchmark_group("consumer_client_stream");
    bench_group.sample_size(10);
    bench_group.bench_function("dedicated_connection_consumer_stream", |b| {
        b.to_async(&rt)
            .iter(|| bench_connection_consumer_streams(
                consumer_clients.clone(),
                partitions.clone(),
                n,
                cache_event_count,
                maximum_wait_time,
            ))
    });
    rt.block_on(async {
        let consumers = Arc::try_unwrap(consumer_clients).unwrap()
            .into_inner().unwrap();
        for consumer in consumers {
            consumer.close().await.unwrap();
        }
    });

    // Bench shared connection consumer streams
    let consumer_clients = rt.block_on(async {
        let mut consumer_clients = Vec::new();
        let mut connection = EventHubConnection::new_from_connection_string(
            connection_string,
            event_hub_name,
            Default::default(),
        ).await?;
        for _ in partitions.iter() {
            let consumer = EventHubConsumerClient::with_connection(
                consumer_group,
                &mut connection,
                client_options.clone(),
            );
            consumer_clients.push(consumer);
        }
        Result::<Vec<Consumer>, azure_core::Error>::Ok(consumer_clients)
    });
    let consumer_clients = Arc::new(Mutex::new(consumer_clients.unwrap()));

    bench_group.bench_function("shared_connection_consumer_stream", |b| {
        b.to_async(&rt)
            .iter(|| bench_connection_consumer_streams(
                consumer_clients.clone(),
                partitions.clone(),
                n,
                cache_event_count,
                maximum_wait_time,
            ))
    });
    rt.block_on(async {
        let consumers = Arc::try_unwrap(consumer_clients).unwrap()
            .into_inner().unwrap();
        for consumer in consumers {
            consumer.close().await.unwrap();
        }
    });
}

criterion_group!(benches, criterion_benchmark);
criterion_main!(benches);

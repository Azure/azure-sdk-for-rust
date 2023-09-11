use std::{sync::Arc, time::Duration};

use azeventhubs::{
    consumer::{EventHubConsumerClient, EventHubConsumerClientOptions, ReadEventOptions},
    EventHubConnection, EventHubsRetryOptions, ReceivedEventData,
};
use criterion::{criterion_group, criterion_main, Criterion};
use futures_util::{lock::Mutex, Stream};
use utils::{create_dedicated_connection_consumer, create_streams, setup_dotenv, Consumer};

mod utils;

async fn bench_event_stream<'a, S>(streams: Arc<Mutex<Vec<S>>>, n: usize)
where
    S: Stream<Item = Result<ReceivedEventData, azure_core::Error>> + Unpin + 'a,
{
    let mut streams = streams.try_lock().unwrap();
    let futures = streams
        .iter_mut()
        .map(|stream| utils::consume_event_stream(stream, n, |_| {}));

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

    let sample_size = 10;
    let n = 1;
    let maximum_wait_time = Duration::from_secs(1);
    let n_prep = 1000; // prepare 2x events for benchmark
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

    let read_event_options = ReadEventOptions::default()
        .with_maximum_wait_time(maximum_wait_time);

    // Bench dedicated connection consumer streams
    let mut consumer_clients = rt
        .block_on(create_dedicated_connection_consumer(
            partitions.len(),
            consumer_group,
            connection_string.clone(),
            event_hub_name.clone(),
            client_options.clone(),
        ))
        .unwrap();
    let streams = rt
        .block_on(create_streams(
            &mut consumer_clients,
            &partitions,
            read_event_options.clone(),
        ))
        .unwrap();
    let streams = Arc::new(Mutex::new(streams));

    let mut bench_group = c.benchmark_group("event_stream");
    bench_group.sample_size(sample_size);
    bench_group.bench_function("dedicated_connection_consumer_stream", |b| {
        b.to_async(&rt)
            .iter(|| bench_event_stream(streams.clone(), n))
    });

    rt.block_on(async {
        let streams = Arc::try_unwrap(streams).unwrap().into_inner();
        for stream in streams {
            stream.close().await.unwrap();
        }
    });

    rt.block_on(async {
        for consumer in consumer_clients {
            consumer.close().await.unwrap();
        }
    });

    // Bench shared connection consumer streams
    let mut consumer_clients = rt
        .block_on(async {
            let mut consumer_clients = Vec::new();
            let mut connection = EventHubConnection::new_from_connection_string(
                connection_string,
                event_hub_name,
                Default::default(),
            )
            .await?;
            for _ in partitions.iter() {
                let consumer = EventHubConsumerClient::with_connection(
                    consumer_group,
                    &mut connection,
                    client_options.clone(),
                );
                consumer_clients.push(consumer);
            }
            Result::<Vec<Consumer>, azure_core::Error>::Ok(consumer_clients)
        })
        .unwrap();
    let streams = rt
        .block_on(create_streams(
            &mut consumer_clients,
            &partitions,
            read_event_options,
        ))
        .unwrap();
    let streams = Arc::new(Mutex::new(streams));

    bench_group.bench_function("shared_connection_consumer_stream", |b| {
        b.to_async(&rt)
            .iter(|| bench_event_stream(streams.clone(), n))
    });

    rt.block_on(async {
        let streams = Arc::try_unwrap(streams).unwrap().into_inner();
        for stream in streams {
            stream.close().await.unwrap();
        }
    });

    rt.block_on(async {
        for consumer in consumer_clients {
            consumer.close().await.unwrap();
        }
    });
}

criterion_group!(benches, criterion_benchmark);
criterion_main!(benches);

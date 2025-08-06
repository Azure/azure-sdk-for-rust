// Copyright (c) Microsoft Corporation.
// Licensed under the MIT license.

use azure_identity::DefaultAzureCredential;
use azure_messaging_eventhubs::{models::EventData, ProducerClient, SendEventOptions};
use criterion::{criterion_group, criterion_main, Criterion};
use std::{env, sync::Arc};
use tokio::runtime::Runtime;

static INIT_LOGGING: std::sync::Once = std::sync::Once::new();

fn setup() {
    INIT_LOGGING.call_once(|| {
        println!("Setting up test logger...");

        use tracing_subscriber::{fmt::format::FmtSpan, EnvFilter};
        tracing_subscriber::fmt()
            .with_env_filter(EnvFilter::from_default_env())
            .with_span_events(FmtSpan::NEW | FmtSpan::CLOSE)
            .with_ansi(std::env::var("NO_COLOR").map_or(true, |v| v.is_empty()))
            .with_writer(std::io::stderr)
            .init();
    });

    azure_core_test::load_dotenv_file(env!("CARGO_MANIFEST_DIR"))
        .expect("Failed to load environment variables from .env file");
}

fn send_batch_benchmark(c: &mut Criterion) {
    setup();

    // Check if the environment variable is set thus allowing the benchmarks to run
    if azure_core_test::TestMode::current().unwrap_or_default() != azure_core_test::TestMode::Live {
        println!("Skipping benchmarks. Set AZURE_TEST_MODE to run.");
        return;
    }

    let rt = Runtime::new().unwrap();

    let namespace = env::var("EVENTHUBS_HOST")
        .expect("Set EVENTHUBS_HOST in env (e.g. my-namespace.servicebus.windows.net)");
    let event_hub = env::var("EVENTHUB_NAME").expect("Set EVENTHUB_NAME in env");

    let credential =
        DefaultAzureCredential::new().expect("Failed to create DefaultAzureCredential");

    let client = rt.block_on(async move {
        Arc::new(
            ProducerClient::builder()
                .open(namespace.as_str(), event_hub.as_str(), credential)
                .await
                .expect("Failed to create ProducerClient"),
        )
    });

    let num_events = 1000;
    let payload = b"perf-test-payload";

    let client = client.clone();
    c.bench_function("eventhubs_batch_send", |b| {
        let client = client.clone();
        b.to_async(&rt).iter(move || {
            let client = client.clone();
            async move {
                let batch = client.create_batch(None).await.unwrap();
                for _ in 0..num_events {
                    batch
                        .try_add_event_data(EventData::from(payload.as_ref()), None)
                        .unwrap();
                }
                let _ = client.send_batch(batch, None).await;
            }
        });
    });
}

fn get_eventhub_properties(c: &mut Criterion) {
    setup();

    // Check if the environment variable is set thus allowing the benchmarks to run
    if azure_core_test::TestMode::current().unwrap_or_default() != azure_core_test::TestMode::Live {
        println!("Skipping benchmarks. Set AZURE_TEST_MODE to run.");
        return;
    }

    let rt = Runtime::new().unwrap();

    let namespace = env::var("EVENTHUBS_HOST")
        .expect("Set EVENTHUBS_HOST in env (e.g. my-namespace.servicebus.windows.net)");
    let event_hub = env::var("EVENTHUB_NAME").expect("Set EVENTHUB_NAME in env");

    let credential =
        DefaultAzureCredential::new().expect("Failed to create DefaultAzureCredential");

    let client = rt.block_on(async move {
        Arc::new(
            ProducerClient::builder()
                .open(namespace.as_str(), event_hub.as_str(), credential)
                .await
                .expect("Failed to create ProducerClient"),
        )
    });

    let client = client.clone();
    c.bench_function("eventhubs_get_properties", |b| {
        let client = client.clone();
        b.to_async(&rt).iter(move || {
            let client = client.clone();
            async move {
                let _ = client
                    .get_eventhub_properties()
                    .await
                    .expect("Failed to get event hub properties");
            }
        });
    });
}

fn get_eventhub_partition_properties(c: &mut Criterion) {
    setup();

    // Check if the environment variable is set thus allowing the benchmarks to run
    if azure_core_test::TestMode::current().unwrap_or_default() != azure_core_test::TestMode::Live {
        println!("Skipping benchmarks. Set AZURE_TEST_MODE to run.");
        return;
    }

    let rt = Runtime::new().unwrap();

    let namespace = env::var("EVENTHUBS_HOST")
        .expect("Set EVENTHUBS_HOST in env (e.g. my-namespace.servicebus.windows.net)");
    let event_hub = env::var("EVENTHUB_NAME").expect("Set EVENTHUB_NAME in env");

    let credential =
        DefaultAzureCredential::new().expect("Failed to create DefaultAzureCredential");

    let client = rt.block_on(async move {
        Arc::new(
            ProducerClient::builder()
                .open(namespace.as_str(), event_hub.as_str(), credential)
                .await
                .expect("Failed to create ProducerClient"),
        )
    });

    let client = client.clone();
    c.bench_function("eventhubs_get_partition_properties", |b| {
        let client = client.clone();
        b.to_async(&rt).iter(move || {
            let client = client.clone();
            async move {
                let _ = client
                    .get_partition_properties("0")
                    .await
                    .expect("Failed to get event hub properties");
            }
        });
    });
}

criterion_group!(
    name = send_batch_benchmarks;
    config = Criterion::default()
        .sample_size(100)
        .warm_up_time(std::time::Duration::new(1, 0))
        .measurement_time(std::time::Duration::new(500, 0));
    targets = send_batch_benchmark, get_eventhub_properties, get_eventhub_partition_properties
);

fn send_benchmark(c: &mut Criterion) {
    setup();

    // Check if the environment variable is set thus allowing the benchmarks to run
    if azure_core_test::TestMode::current().unwrap_or_default() != azure_core_test::TestMode::Live {
        println!("Skipping benchmarks. Set AZURE_TEST_MODE to run.");
        return;
    }

    let rt = Runtime::new().unwrap();

    let namespace = env::var("EVENTHUBS_HOST")
        .expect("Set EVENTHUBS_HOST in env (e.g. my-namespace.servicebus.windows.net)");
    let event_hub = env::var("EVENTHUB_NAME").expect("Set EVENTHUB_NAME in env");

    let credential =
        DefaultAzureCredential::new().expect("Failed to create DefaultAzureCredential");

    let client = rt.block_on(async move {
        Arc::new(
            ProducerClient::builder()
                .open(namespace.as_str(), event_hub.as_str(), credential)
                .await
                .expect("Failed to create ProducerClient"),
        )
    });

    c.bench_function("eventhubs_send", |b| {
        let client = client.clone();
        let mut index: u64 = 0;
        b.to_async(&rt).iter(move || {
            let client = client.clone();
            async move {
                index += 1;
                let payload = b"a";
                let event_data = EventData::builder()
                    .with_body(payload)
                    .add_property("Number".to_string(), index)
                    .add_property("PartitionId".to_string(), "0")
                    .build();
                client
                    .send_event(
                        event_data,
                        Some(SendEventOptions {
                            partition_id: Some("0".to_string()),
                        }),
                    )
                    .await
            }
        });
    });
}

criterion_group!(
    name = send_benchmarks;
    config = Criterion::default()
        .sample_size(1000)
        .warm_up_time(std::time::Duration::new(1, 0))
        .measurement_time(std::time::Duration::new(2500, 0));
    targets =  send_benchmark
);

criterion_main!(send_batch_benchmarks, send_benchmarks);

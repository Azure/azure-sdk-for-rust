// Copyright (c) Microsoft Corporation. All Rights reserved
// Licensed under the MIT license.

// cspell:ignore rngs Seedable

use super::StressTestSpec;
use azure_core_amqp::AmqpSimpleValue;
use azure_messaging_eventhubs::{
    models::EventData, ConsumerClient, EventDataBatchOptions, OpenReceiverOptions, ProducerClient,
    StartLocation, StartPosition,
};
use clap::{Arg, ArgMatches, Command};
use futures::StreamExt;
use rand::{rngs::StdRng, RngExt, SeedableRng};
use std::{
    collections::HashMap,
    error::Error,
    sync::Arc,
    time::{Duration, Instant},
};
use tokio::sync::Mutex;
use tracing::{info, warn};

#[derive(Debug, Clone)]
pub struct ContinuousStressConfig {
    pub duration: Duration,
    pub min_batch_size: usize,
    pub max_batch_size: usize,
    pub min_delay_secs: u64,
    pub max_delay_secs: u64,
}

impl Default for ContinuousStressConfig {
    fn default() -> Self {
        Self {
            duration: Duration::from_secs(72 * 60 * 60),
            min_batch_size: 20,
            max_batch_size: 100,
            min_delay_secs: 1,
            max_delay_secs: 10,
        }
    }
}

#[derive(Debug, Default, Clone)]
pub struct ContinuousStressMetrics {
    pub batches_sent: u64,
    pub events_sent: u64,
    pub events_received: u64,
    pub corrupted_body_failures: u64,
    pub corrupted_properties_failures: u64,
    pub producer_failures: u64,
    pub consumer_failures: u64,
    pub lost_events: usize,
}

#[derive(Debug, Clone)]
struct ExpectedEvent {
    batch_index: u64,
    batch_size: u32,
    index: u32,
}

struct SharedState {
    missing_events: Mutex<HashMap<String, ExpectedEvent>>,
    last_sequence: Mutex<HashMap<String, i64>>,
    metrics: Mutex<ContinuousStressMetrics>,
    start_instant: Instant,
}

impl SharedState {
    fn new() -> Self {
        Self {
            missing_events: Mutex::new(HashMap::new()),
            last_sequence: Mutex::new(HashMap::new()),
            metrics: Mutex::new(ContinuousStressMetrics::default()),
            start_instant: Instant::now(),
        }
    }
}

/// Run a continuous send/receive stress test similar to the C# sample.
pub async fn run_continuous_stress(
    host: String,
    eventhub: String,
    credential: Arc<dyn azure_core::credentials::TokenCredential>,
    config: ContinuousStressConfig,
) -> Result<ContinuousStressMetrics, Box<dyn Error + Send + Sync>> {
    let state = Arc::new(SharedState::new());

    let producer = ProducerClient::builder()
        .with_application_id("continuous-stress-producer".to_string())
        .open(host.as_str(), eventhub.as_str(), credential.clone())
        .await?;
    let producer = Arc::new(producer);

    let partition_ids = producer.get_eventhub_properties().await?.partition_ids;

    let end_instant = Instant::now() + config.duration;
    let sender = tokio::spawn(send_loop(
        producer.clone(),
        state.clone(),
        config.clone(),
        end_instant,
    ));

    let mut receiver_handles = Vec::new();
    for partition_id in partition_ids {
        let state = state.clone();
        let host = host.clone();
        let eventhub = eventhub.clone();
        let credential = credential.clone();

        receiver_handles.push(tokio::spawn(async move {
            receive_loop(host, eventhub, credential, partition_id, state, end_instant).await
        }));
    }

    let reporter = {
        let reporter_state = state.clone();
        tokio::spawn(async move {
            periodic_report(reporter_state, end_instant).await;
        })
    };

    let sender_result = sender
        .await
        .unwrap_or_else(|e| Err(Box::new(e) as Box<dyn Error + Send + Sync>));

    let mut receiver_results = Vec::new();
    for handle in receiver_handles {
        receiver_results.push(
            handle
                .await
                .unwrap_or_else(|e| Err(Box::new(e) as Box<dyn Error + Send + Sync>)),
        );
    }

    reporter.await.ok();

    if let Err(e) = sender_result {
        let mut metrics = state.metrics.lock().await;
        metrics.producer_failures += 1;
        warn!("Producer failure: {:?}", e);
    }

    for result in receiver_results {
        if let Err(e) = result {
            let mut metrics = state.metrics.lock().await;
            metrics.consumer_failures += 1;
            warn!("Consumer failure: {:?}", e);
        }
    }

    finalize_metrics(state).await
}

pub fn continuous_send_receive_spec() -> StressTestSpec {
    StressTestSpec {
        name: "continuous_send_receive_stress",
        description: "Long-running send/receive durability loop",
        configure: |cmd: Command| {
            cmd.arg(
                Arg::new("duration-hours")
                    .long("duration-hours")
                    .help("Duration of the continuous stress test in hours")
                    .value_parser(clap::value_parser!(u64))
                    .default_value("72"),
            )
            .arg(
                Arg::new("min-batch")
                    .long("min-batch")
                    .help("Minimum batch size for continuous stress test")
                    .value_parser(clap::value_parser!(u32))
                    .default_value("20"),
            )
            .arg(
                Arg::new("max-batch")
                    .long("max-batch")
                    .help("Maximum batch size for continuous stress test")
                    .value_parser(clap::value_parser!(u32))
                    .default_value("100"),
            )
            .arg(
                Arg::new("min-delay")
                    .long("min-delay")
                    .help("Minimum delay between batches (seconds) for continuous stress test")
                    .value_parser(clap::value_parser!(u64))
                    .default_value("1"),
            )
            .arg(
                Arg::new("max-delay")
                    .long("max-delay")
                    .help("Maximum delay between batches (seconds) for continuous stress test")
                    .value_parser(clap::value_parser!(u64))
                    .default_value("10"),
            )
        },
        run: |matches: ArgMatches| Box::pin(async move { run_standalone(matches).await }),
    }
}

async fn run_standalone(matches: ArgMatches) -> Result<(), Box<dyn Error + Send + Sync>> {
    use azure_identity::{DeveloperToolsCredential, DeveloperToolsCredentialOptions};
    use dotenvy::{dotenv, var};

    dotenv().ok();

    let host =
        var("EVENTHUBS_HOST").map_err(|_| "EVENTHUBS_HOST environment variable is required")?;
    let eventhub =
        var("EVENTHUB_NAME").map_err(|_| "EVENTHUB_NAME environment variable is required")?;

    let credential: Arc<dyn azure_core::credentials::TokenCredential> =
        DeveloperToolsCredential::new(Some(DeveloperToolsCredentialOptions::default()))?;

    let duration_hours = *matches.get_one::<u64>("duration-hours").unwrap_or(&72);
    let min_batch_size = *matches.get_one::<u32>("min-batch").unwrap_or(&20) as usize;
    let max_batch_size = *matches.get_one::<u32>("max-batch").unwrap_or(&100) as usize;
    let min_delay = *matches.get_one::<u64>("min-delay").unwrap_or(&1);
    let max_delay = *matches.get_one::<u64>("max-delay").unwrap_or(&10);

    let config = ContinuousStressConfig {
        duration: Duration::from_secs(duration_hours * 60 * 60),
        min_batch_size: min_batch_size.min(max_batch_size),
        max_batch_size: max_batch_size.max(min_batch_size),
        min_delay_secs: min_delay.min(max_delay),
        max_delay_secs: max_delay.max(min_delay),
    };

    info!("Running continuous stress with config: {:?}", config);

    let metrics = run_continuous_stress(host, eventhub, credential, config).await?;

    info!(
        "Continuous results: batches={}, sent={}, received={}, lost={}, corrupted_body={}, corrupted_props={}, producer_failures={}, consumer_failures={}",
        metrics.batches_sent,
        metrics.events_sent,
        metrics.events_received,
        metrics.lost_events,
        metrics.corrupted_body_failures,
        metrics.corrupted_properties_failures,
        metrics.producer_failures,
        metrics.consumer_failures
    );

    Ok(())
}

async fn finalize_metrics(
    state: Arc<SharedState>,
) -> Result<ContinuousStressMetrics, Box<dyn Error + Send + Sync>> {
    let missing = state.missing_events.lock().await;
    let mut metrics = state.metrics.lock().await;
    metrics.lost_events = missing.len();
    Ok(metrics.clone())
}

async fn send_loop(
    producer: Arc<ProducerClient>,
    state: Arc<SharedState>,
    config: ContinuousStressConfig,
    end_instant: Instant,
) -> Result<(), Box<dyn Error + Send + Sync>> {
    let seed: [u8; 32] = rand::random();
    let mut rng = StdRng::from_seed(seed);
    let mut batch_index: u64 = 0;

    while Instant::now() < end_instant {
        let batch_size = rng.random_range(config.min_batch_size..=config.max_batch_size);
        let mut batch = producer
            .create_batch(Some(EventDataBatchOptions::default()))
            .await?;

        for event_index in 0..batch_size {
            let key = format!(
                "evt-{}-{}-{}",
                batch_index,
                event_index,
                rng.random::<u64>()
            );
            let expected = ExpectedEvent {
                batch_index,
                batch_size: batch_size as u32,
                index: event_index as u32,
            };

            let event = EventData::builder()
                .with_body(key.as_bytes())
                .add_property("batch_index".to_string(), expected.batch_index)
                .add_property("batch_size".to_string(), expected.batch_size)
                .add_property("index".to_string(), expected.index)
                .with_message_id(key.clone())
                .build();

            match batch.try_add_event_data(event, None)? {
                true => {
                    // Event was successfully added, record it as expected
                    let mut missing = state.missing_events.lock().await;
                    missing.insert(key, expected);
                }
                false => {
                    // Batch is full, send it and create a new one
                    producer.send_batch(batch, None).await?;

                    {
                        let mut metrics = state.metrics.lock().await;
                        metrics.batches_sent += 1;
                    }

                    batch = producer
                        .create_batch(Some(EventDataBatchOptions::default()))
                        .await?;

                    // Add the current event to the new batch
                    let event = EventData::builder()
                        .with_body(key.as_bytes())
                        .add_property("batch_index".to_string(), expected.batch_index)
                        .add_property("batch_size".to_string(), expected.batch_size)
                        .add_property("index".to_string(), expected.index)
                        .with_message_id(key.clone())
                        .build();

                    batch.try_add_event_data(event, None)?;

                    // Record it as expected now that it's in a batch
                    let mut missing = state.missing_events.lock().await;
                    missing.insert(key, expected);
                }
            }
        }

        producer.send_batch(batch, None).await?;

        {
            let mut metrics = state.metrics.lock().await;
            metrics.batches_sent += 1;
            metrics.events_sent += batch_size as u64;

            // Light heartbeat so we know the sender is making progress without overwhelming logs.
            if metrics.batches_sent % 10 == 0 {
                info!(
                    "Sender progress: batches sent={}, events sent={} (last batch size={})",
                    metrics.batches_sent, metrics.events_sent, batch_size
                );
            }
        }

        batch_index += 1;

        let delay = rng.random_range(config.min_delay_secs..=config.max_delay_secs);
        tokio::time::sleep(Duration::from_secs(delay)).await;
    }

    Ok(())
}

async fn receive_loop(
    host: String,
    eventhub: String,
    credential: Arc<dyn azure_core::credentials::TokenCredential>,
    partition_id: String,
    state: Arc<SharedState>,
    end_instant: Instant,
) -> Result<(), Box<dyn Error + Send + Sync>> {
    let consumer = ConsumerClient::builder()
        .with_application_id(format!("continuous-stress-consumer-{}", &partition_id))
        .open(host.as_str(), eventhub, credential)
        .await?;

    let start_position = {
        let last = state.last_sequence.lock().await;
        if let Some(seq) = last.get(&partition_id) {
            StartPosition {
                location: StartLocation::SequenceNumber(*seq),
                ..Default::default()
            }
        } else {
            StartPosition {
                location: StartLocation::Latest,
                ..Default::default()
            }
        }
    };

    let receiver = consumer
        .open_receiver_on_partition(
            partition_id.clone(),
            Some(OpenReceiverOptions {
                start_position: Some(start_position),
                ..Default::default()
            }),
        )
        .await?;

    let mut stream = receiver.stream_events();

    while Instant::now() < end_instant {
        match tokio::time::timeout(Duration::from_secs(10), stream.next()).await {
            Ok(Some(Ok(partition_event))) => {
                let event_data = partition_event.event_data();
                let body = String::from_utf8_lossy(event_data.body().unwrap_or(&[])).to_string();

                if let Some(expected) = {
                    let mut missing = state.missing_events.lock().await;
                    missing.remove(&body)
                } {
                    if !properties_match(event_data, &expected) {
                        let mut metrics = state.metrics.lock().await;
                        metrics.corrupted_properties_failures += 1;
                        warn!(
                            "Partition {} received event with mismatched properties",
                            partition_id
                        );
                    } else {
                        let mut metrics = state.metrics.lock().await;
                        metrics.events_received += 1;

                        // Light heartbeat to show consumer activity.
                        if metrics.events_received % 500 == 0 {
                            info!(
                                "Consumer {} progress: events received={}, pending={}",
                                partition_id,
                                metrics.events_received,
                                state.missing_events.lock().await.len()
                            );
                        }
                    }
                } else {
                    let mut metrics = state.metrics.lock().await;
                    metrics.corrupted_body_failures += 1;
                    warn!("Partition {} received unknown event body", partition_id);
                }

                if let Some(sequence) = partition_event.sequence_number() {
                    let mut last = state.last_sequence.lock().await;
                    last.insert(partition_id.clone(), sequence);
                }
            }
            Ok(Some(Err(err))) => {
                let mut metrics = state.metrics.lock().await;
                metrics.consumer_failures += 1;
                warn!("Consumer for partition {} error: {:?}", partition_id, err);
            }
            Ok(None) => {
                break;
            }
            Err(_) => {
                // Timeout waiting for events; continue until end_instant
            }
        }
    }

    Ok(())
}

fn properties_match(event_data: &EventData, expected: &ExpectedEvent) -> bool {
    let properties = match event_data.properties() {
        Some(props) => props,
        None => return false,
    };

    let batch_index = extract_u64(properties.get("batch_index"));
    let batch_size = extract_u64(properties.get("batch_size"));
    let index = extract_u64(properties.get("index"));

    batch_index == Some(expected.batch_index)
        && batch_size == Some(expected.batch_size as u64)
        && index == Some(expected.index as u64)
}

fn extract_u64(value: Option<&AmqpSimpleValue>) -> Option<u64> {
    match value? {
        AmqpSimpleValue::UByte(v) => Some(*v as u64),
        AmqpSimpleValue::UShort(v) => Some(*v as u64),
        AmqpSimpleValue::UInt(v) => Some(*v as u64),
        AmqpSimpleValue::ULong(v) => Some(*v),
        AmqpSimpleValue::Byte(v) => (*v).try_into().ok(),
        AmqpSimpleValue::Short(v) => (*v).try_into().ok(),
        AmqpSimpleValue::Int(v) => (*v).try_into().ok(),
        AmqpSimpleValue::Long(v) => (*v).try_into().ok(),
        AmqpSimpleValue::Boolean(v) => Some(*v as u64),
        AmqpSimpleValue::Float(v) => Some((*v as f64).round() as u64),
        AmqpSimpleValue::Double(v) => Some((*v).round() as u64),
        AmqpSimpleValue::Char(v) => Some(*v as u64),
        _ => None,
    }
}

async fn periodic_report(state: Arc<SharedState>, end_instant: Instant) {
    let mut last_print = Instant::now();

    while Instant::now() < end_instant {
        if last_print.elapsed() >= Duration::from_secs(30) {
            let metrics = state.metrics.lock().await.clone();
            let missing = state.missing_events.lock().await.len();
            let elapsed = Instant::now().saturating_duration_since(state.start_instant);

            info!(
                "Elapsed: {:?}, batches sent: {}, events sent: {}, received: {}, lost (pending): {}, corrupted body: {}, corrupted props: {}, producer failures: {}, consumer failures: {}",
                elapsed,
                metrics.batches_sent,
                metrics.events_sent,
                metrics.events_received,
                missing,
                metrics.corrupted_body_failures,
                metrics.corrupted_properties_failures,
                metrics.producer_failures,
                metrics.consumer_failures
            );

            last_print = Instant::now();
        }

        tokio::time::sleep(Duration::from_secs(5)).await;
    }
}

// Copyright (c) Microsoft Corporation. All Rights reserved
// Licensed under the MIT license.

use super::StressTestSpec;
use azure_messaging_eventhubs::{
    models::{EventData, MessageId},
    ConsumerClient, EventDataBatchOptions, OpenReceiverOptions, ProducerClient, StartLocation,
    StartPosition,
};
use clap::{Arg, ArgMatches, Command};
use futures::stream::StreamExt;
use std::{
    collections::HashMap,
    error::Error,
    sync::{Arc, Mutex},
    time::{Duration, Instant},
};
use tokio::time::timeout;
use tracing::{info, warn};

/// Configuration for the basic publish/read stress test
#[derive(Debug, Clone)]
pub struct TestConfig {
    /// Number of events to publish and read
    pub event_count: usize,
    /// Number of concurrent producer tasks
    pub producer_count: usize,
    /// Number of concurrent consumer tasks
    pub consumer_count: usize,
    /// Timeout for the entire test
    pub test_timeout: Duration,
    /// Size of each event payload in bytes
    pub event_size: usize,
    /// Batch size for publishing events
    pub batch_size: usize,
}

impl Default for TestConfig {
    fn default() -> Self {
        Self {
            event_count: 1000,
            producer_count: 2,
            consumer_count: 2,
            test_timeout: Duration::from_secs(300), // 5 minutes
            event_size: 1024,                       // 1KB
            batch_size: 100,
        }
    }
}

/// Metrics collected during the stress test
#[derive(Debug, Default)]
pub struct TestMetrics {
    pub events_published: usize,
    pub events_consumed: usize,
    pub publish_duration: Duration,
    pub consume_duration: Duration,
    pub publish_throughput: f64, // events per second
    pub consume_throughput: f64, // events per second
    pub errors: Vec<String>,
}

pub async fn run_stress_test_inner(
    host: String,
    eventhub: String,
    credential: Arc<dyn azure_core::credentials::TokenCredential>,
    config: TestConfig,
) -> Result<TestMetrics, Box<dyn Error + Send + Sync>> {
    // Wrap the entire stress test with the configured timeout
    let result = timeout(config.test_timeout, async {
        run_stress_test_inner_impl(host, eventhub, credential, config).await
    })
    .await;

    match result {
        Ok(metrics) => metrics,
        Err(_) => Err("Stress test timed out".into()),
    }
}

pub fn basic_publish_read_spec() -> StressTestSpec {
    StressTestSpec {
        name: "basic_publish_read_test",
        description: "Bounded publish/read throughput validation",
        configure: |cmd: Command| {
            cmd.arg(
                Arg::new("events")
                    .long("events")
                    .short('e')
                    .help("Number of events to publish and read")
                    .value_parser(clap::value_parser!(u32))
                    .default_value("100"),
            )
            .arg(
                Arg::new("producers")
                    .long("producers")
                    .short('p')
                    .help("Number of concurrent producer tasks")
                    .value_parser(clap::value_parser!(u32))
                    .default_value("1"),
            )
            .arg(
                Arg::new("consumers")
                    .long("consumers")
                    .short('c')
                    .help("Number of concurrent consumer tasks")
                    .value_parser(clap::value_parser!(u32))
                    .default_value("1"),
            )
            .arg(
                Arg::new("timeout")
                    .long("timeout")
                    .short('t')
                    .help("Timeout for the entire test in seconds")
                    .value_parser(clap::value_parser!(u64))
                    .default_value("120"),
            )
            .arg(
                Arg::new("event-size")
                    .long("event-size")
                    .help("Size of each event payload in bytes")
                    .value_parser(clap::value_parser!(u32))
                    .default_value("512"),
            )
            .arg(
                Arg::new("batch-size")
                    .long("batch-size")
                    .help("Batch size for publishing events")
                    .value_parser(clap::value_parser!(u32))
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

    let config = TestConfig {
        event_count: *matches.get_one::<u32>("events").unwrap_or(&100) as usize,
        producer_count: *matches.get_one::<u32>("producers").unwrap_or(&1) as usize,
        consumer_count: *matches.get_one::<u32>("consumers").unwrap_or(&1) as usize,
        test_timeout: std::time::Duration::from_secs(
            *matches.get_one::<u64>("timeout").unwrap_or(&120),
        ),
        event_size: *matches.get_one::<u32>("event-size").unwrap_or(&512) as usize,
        batch_size: *matches.get_one::<u32>("batch-size").unwrap_or(&10) as usize,
    };

    info!("Running basic publish/read with config: {:?}", config);

    let metrics = run_stress_test_inner(host, eventhub, credential, config).await?;

    log_test_results(&metrics);
    validate_test_results(&metrics)?;

    Ok(())
}

async fn run_stress_test_inner_impl(
    host: String,
    eventhub: String,
    credential: Arc<dyn azure_core::credentials::TokenCredential>,
    config: TestConfig,
) -> Result<TestMetrics, Box<dyn Error + Send + Sync>> {
    let mut metrics = TestMetrics::default();

    // Create a unique test run identifier
    let test_id = format!(
        "stress-{}",
        std::time::SystemTime::now()
            .duration_since(std::time::UNIX_EPOCH)
            .unwrap()
            .as_secs()
    );
    info!("Test ID: {}", test_id);

    // Track published events for validation
    let published_events = Arc::new(Mutex::new(HashMap::<String, String>::new()));
    let consumed_events = Arc::new(Mutex::new(HashMap::<String, String>::new()));

    // Start consumers first
    let consume_start = Instant::now();
    let consumer_handles = start_consumers(
        host.clone(),
        eventhub.clone(),
        credential.clone(),
        &config,
        consumed_events.clone(),
    )
    .await?;

    // Wait a moment for consumers to be ready
    tokio::time::sleep(Duration::from_secs(2)).await;

    // Start producers
    let publish_start = Instant::now();
    let producer_handles = start_producers(
        host,
        eventhub,
        credential,
        &config,
        test_id,
        published_events.clone(),
    )
    .await?;

    // Wait for all producers to complete
    for handle in producer_handles {
        match handle.await {
            Ok(Ok(events_published)) => {
                metrics.events_published += events_published;
            }
            Ok(Err(e)) => {
                warn!("Producer error: {:?}", e);
                metrics.errors.push(format!("Producer error: {:?}", e));
            }
            Err(e) => {
                warn!("Producer join error: {:?}", e);
                metrics.errors.push(format!("Producer join error: {:?}", e));
            }
        }
    }
    metrics.publish_duration = publish_start.elapsed();

    info!(
        "Published {} events in {:?}",
        metrics.events_published, metrics.publish_duration
    );

    // Wait for consumers to finish (with timeout)
    let consume_timeout = Duration::from_secs(60);
    let consume_result = timeout(consume_timeout, async {
        for handle in consumer_handles {
            match handle.await {
                Ok(Ok(events_consumed)) => {
                    metrics.events_consumed += events_consumed;
                }
                Ok(Err(e)) => {
                    warn!("Consumer error: {:?}", e);
                    metrics.errors.push(format!("Consumer error: {:?}", e));
                }
                Err(e) => {
                    warn!("Consumer join error: {:?}", e);
                    metrics.errors.push(format!("Consumer join error: {:?}", e));
                }
            }
        }
        Ok::<(), Box<dyn Error + Send + Sync>>(())
    })
    .await;

    match consume_result {
        Ok(_) => {
            metrics.consume_duration = consume_start.elapsed();
        }
        Err(_) => {
            warn!("Consumer timeout - some events may not have been consumed");
            metrics.consume_duration = consume_timeout;
            // Get current count
            let consumed = consumed_events.lock().unwrap();
            metrics.events_consumed = consumed.len();
        }
    }

    // Calculate throughput
    if metrics.publish_duration.as_secs_f64() > 0.0 {
        metrics.publish_throughput =
            metrics.events_published as f64 / metrics.publish_duration.as_secs_f64();
    }
    if metrics.consume_duration.as_secs_f64() > 0.0 {
        metrics.consume_throughput =
            metrics.events_consumed as f64 / metrics.consume_duration.as_secs_f64();
    }

    Ok(metrics)
}

async fn start_producers(
    host: String,
    eventhub: String,
    credential: Arc<dyn azure_core::credentials::TokenCredential>,
    config: &TestConfig,
    test_id: String,
    published_events: Arc<Mutex<HashMap<String, String>>>,
) -> Result<
    Vec<tokio::task::JoinHandle<Result<usize, Box<dyn Error + Send + Sync>>>>,
    Box<dyn Error + Send + Sync>,
> {
    let base_events_per_producer = config.event_count / config.producer_count;
    let remainder = config.event_count % config.producer_count;
    let mut handles = Vec::new();

    for producer_id in 0..config.producer_count {
        let host = host.clone();
        let eventhub = eventhub.clone();
        let credential = credential.clone();
        let config = config.clone();
        let test_id = test_id.clone();
        let published_events = published_events.clone();

        // Distribute remainder across first producers
        let events_for_this_producer =
            base_events_per_producer + if producer_id < remainder { 1 } else { 0 };

        let handle = tokio::spawn(async move {
            let ctx = ProducerContext {
                host,
                eventhub,
                credential,
            };
            run_producer_task(
                ctx,
                producer_id,
                events_for_this_producer,
                &config,
                test_id,
                published_events,
            )
            .await
        });
        handles.push(handle);
    }

    Ok(handles)
}

struct ProducerContext {
    host: String,
    eventhub: String,
    credential: Arc<dyn azure_core::credentials::TokenCredential>,
}

async fn run_producer_task(
    ctx: ProducerContext,
    producer_id: usize,
    event_count: usize,
    config: &TestConfig,
    test_id: String,
    published_events: Arc<Mutex<HashMap<String, String>>>,
) -> Result<usize, Box<dyn Error + Send + Sync>> {
    let producer = ProducerClient::builder()
        .with_application_id(format!("stress-producer-{}", producer_id))
        .open(ctx.host.as_str(), ctx.eventhub.as_str(), ctx.credential)
        .await
        .map_err(|e| Box::new(e) as Box<dyn Error + Send + Sync>)?;

    let mut events_published = 0;
    let payload = "x".repeat(config.event_size);

    // Send events in batches
    let mut batch_start = 0;
    while batch_start < event_count {
        let batch_end = std::cmp::min(batch_start + config.batch_size, event_count);

        let mut batch = producer
            .create_batch(Some(EventDataBatchOptions::default()))
            .await
            .map_err(|e| Box::new(e) as Box<dyn Error + Send + Sync>)?;

        for i in batch_start..batch_end {
            let event_id = format!("{}-producer-{}-event-{}", test_id, producer_id, i);
            let event_data = EventData::builder()
                .with_body(payload.as_bytes())
                .with_message_id(i as u64)
                .add_property("event_id".to_string(), event_id.clone())
                .build();

            // Track the event
            {
                let mut published = published_events.lock().unwrap();
                published.insert(event_id.clone(), payload.clone());
            }

            if !batch
                .try_add_event_data(event_data, None)
                .map_err(|e| Box::new(e) as Box<dyn Error + Send + Sync>)?
            {
                // Batch is full, send it and start a new one
                if !batch.is_empty() {
                    let batch_len = batch.len();
                    producer
                        .send_batch(batch, None)
                        .await
                        .map_err(|e| Box::new(e) as Box<dyn Error + Send + Sync>)?;
                    events_published += batch_len;
                }

                // Create a new batch and add the current event
                batch = producer
                    .create_batch(Some(EventDataBatchOptions::default()))
                    .await
                    .map_err(|e| Box::new(e) as Box<dyn Error + Send + Sync>)?;
                let event_data = EventData::builder()
                    .with_body(payload.as_bytes())
                    .with_message_id(i as u64)
                    .add_property("event_id".to_string(), event_id)
                    .build();
                batch
                    .try_add_event_data(event_data, None)
                    .map_err(|e| Box::new(e) as Box<dyn Error + Send + Sync>)?;
            }
        }

        // Send the final batch if it has events
        if !batch.is_empty() {
            let batch_len = batch.len();
            producer
                .send_batch(batch, None)
                .await
                .map_err(|e| Box::new(e) as Box<dyn Error + Send + Sync>)?;
            events_published += batch_len;
        }

        batch_start = batch_end;

        // Small delay between batches to avoid overwhelming
        tokio::time::sleep(Duration::from_millis(10)).await;
    }

    info!(
        "Producer {} published {} events",
        producer_id, events_published
    );
    Ok(events_published)
}

async fn start_consumers(
    host: String,
    eventhub: String,
    credential: Arc<dyn azure_core::credentials::TokenCredential>,
    config: &TestConfig,
    consumed_events: Arc<Mutex<HashMap<String, String>>>,
) -> Result<
    Vec<tokio::task::JoinHandle<Result<usize, Box<dyn Error + Send + Sync>>>>,
    Box<dyn Error + Send + Sync>,
> {
    // Discover partitions up-front so we can spawn one task per partition.
    // Each partition gets its own dedicated consumer client and polling loop,
    // which avoids the stream-multiplexing issues that come with select_all.
    let discovery_client = ConsumerClient::builder()
        .with_application_id("stress-consumer-discovery".to_string())
        .open(host.as_str(), eventhub.clone(), credential.clone())
        .await
        .map_err(|e| Box::new(e) as Box<dyn Error + Send + Sync>)?;

    let eventhub_properties = discovery_client
        .get_eventhub_properties()
        .await
        .map_err(|e| Box::new(e) as Box<dyn Error + Send + Sync>)?;

    let partition_ids = eventhub_properties.partition_ids.clone();
    if partition_ids.is_empty() {
        return Err(Box::new(std::io::Error::other(
            "Event Hub has no partitions",
        )));
    }

    info!(
        "Discovered {} partitions: {:?} (consumer_count hint: {})",
        partition_ids.len(),
        partition_ids,
        config.consumer_count,
    );

    let mut handles = Vec::new();
    let total_expected_events = config.event_count;

    // Spawn one consumer task per partition for full coverage
    for (idx, partition_id) in partition_ids.into_iter().enumerate() {
        let host = host.clone();
        let eventhub = eventhub.clone();
        let credential = credential.clone();
        let consumed_events = consumed_events.clone();

        let handle = tokio::spawn(async move {
            run_consumer_task(
                host,
                eventhub,
                credential,
                idx,
                partition_id,
                total_expected_events,
                consumed_events,
            )
            .await
        });
        handles.push(handle);
    }

    Ok(handles)
}

async fn run_consumer_task(
    host: String,
    eventhub: String,
    credential: Arc<dyn azure_core::credentials::TokenCredential>,
    consumer_id: usize,
    partition_id: String,
    total_expected_events: usize,
    consumed_events: Arc<Mutex<HashMap<String, String>>>,
) -> Result<usize, Box<dyn Error + Send + Sync>> {
    let consumer = ConsumerClient::builder()
        .with_application_id(format!("stress-consumer-{}", consumer_id))
        .open(host.as_str(), eventhub, credential)
        .await
        .map_err(|e| Box::new(e) as Box<dyn Error + Send + Sync>)?;

    let receiver = consumer
        .open_receiver_on_partition(
            partition_id.clone(),
            Some(OpenReceiverOptions {
                start_position: Some(StartPosition {
                    location: StartLocation::Latest,
                    ..Default::default()
                }),
                ..Default::default()
            }),
        )
        .await
        .map_err(|e| Box::new(e) as Box<dyn Error + Send + Sync>)?;

    info!(
        "Consumer {} listening on partition {}",
        consumer_id, partition_id
    );

    let mut receive_stream = receiver.stream_events();
    let mut events_consumed = 0;
    let start_time = Instant::now();
    let timeout_duration = Duration::from_secs(120);
    let mut consecutive_timeouts = 0;
    let max_consecutive_timeouts = 6;

    while start_time.elapsed() < timeout_duration {
        // Check shared consumed count to see if all expected events have been received across all consumers
        {
            let consumed = consumed_events.lock().unwrap();
            if consumed.len() >= total_expected_events {
                break;
            }
        }

        match timeout(Duration::from_secs(10), receive_stream.next()).await {
            Ok(Some(Ok(partition_event))) => {
                consecutive_timeouts = 0;
                let event_data = partition_event.event_data();

                // Try to get event_id from properties first
                let event_id = if let Some(properties) = event_data.properties() {
                    if let Some(id) = properties.get("event_id") {
                        match id {
                            azure_core_amqp::AmqpSimpleValue::String(s) => s.clone(),
                            azure_core_amqp::AmqpSimpleValue::ULong(u) => u.to_string(),
                            azure_core_amqp::AmqpSimpleValue::Long(l) => l.to_string(),
                            azure_core_amqp::AmqpSimpleValue::Int(i) => i.to_string(),
                            azure_core_amqp::AmqpSimpleValue::UInt(ui) => ui.to_string(),
                            _ => format!("event-{}", events_consumed),
                        }
                    } else {
                        format!("event-{}", events_consumed)
                    }
                } else if let Some(message_id) = event_data.message_id() {
                    match message_id {
                        MessageId::String(s) => s.clone(),
                        MessageId::Uuid(u) => u.to_string(),
                        MessageId::Ulong(u) => u.to_string(),
                        MessageId::Binary(b) => String::from_utf8_lossy(b).to_string(),
                    }
                } else {
                    format!("event-{}", events_consumed)
                };

                let body = String::from_utf8_lossy(event_data.body().unwrap_or(&[]));

                // Track the consumed event
                {
                    let mut consumed = consumed_events.lock().unwrap();
                    consumed.insert(event_id.clone(), body.to_string());
                }

                events_consumed += 1;

                if events_consumed % 100 == 0 {
                    info!(
                        "Consumer {} consumed {} events",
                        consumer_id, events_consumed
                    );
                }
            }
            Ok(Some(Err(e))) => {
                warn!("Consumer {} error receiving event: {:?}", consumer_id, e);
            }
            Ok(None) => {
                info!("Consumer {} stream ended", consumer_id);
                break;
            }
            Err(_) => {
                consecutive_timeouts += 1;
                info!(
                    "Consumer {} timeout waiting for events ({}/{})",
                    consumer_id, consecutive_timeouts, max_consecutive_timeouts
                );
                if consecutive_timeouts >= max_consecutive_timeouts {
                    info!(
                        "Consumer {} exiting after {} consecutive timeouts",
                        consumer_id, consecutive_timeouts
                    );
                    break;
                }
            }
        }
    }

    info!(
        "Consumer {} finished with {} events consumed",
        consumer_id, events_consumed
    );
    Ok(events_consumed)
}

pub fn log_test_results(metrics: &TestMetrics) {
    info!("=== Basic Publish/Read Stress Test Results ===");
    info!("Events Published: {}", metrics.events_published);
    info!("Events Consumed: {}", metrics.events_consumed);
    info!("Publish Duration: {:?}", metrics.publish_duration);
    info!("Consume Duration: {:?}", metrics.consume_duration);
    info!(
        "Publish Throughput: {:.2} events/sec",
        metrics.publish_throughput
    );
    info!(
        "Consume Throughput: {:.2} events/sec",
        metrics.consume_throughput
    );

    if !metrics.errors.is_empty() {
        warn!("Errors encountered: {:?}", metrics.errors);
    }
}

pub fn validate_test_results(metrics: &TestMetrics) -> Result<(), Box<dyn Error + Send + Sync>> {
    // Basic validation - we should have published some events
    if metrics.events_published == 0 {
        return Err("No events were published".into());
    }

    // We should have consumed at least some events (allowing for some loss in stress scenarios)
    let min_expected_consumed = metrics.events_published / 2; // Allow 50% loss for stress test
    if metrics.events_consumed < min_expected_consumed {
        return Err(format!(
            "Too few events consumed: {} < {} (minimum expected)",
            metrics.events_consumed, min_expected_consumed
        )
        .into());
    }

    // Throughput should be reasonable (at least 1 event per second)
    if metrics.publish_throughput < 1.0 {
        return Err(format!(
            "Publish throughput too low: {:.2} events/sec",
            metrics.publish_throughput
        )
        .into());
    }

    info!("Stress test validation passed");
    Ok(())
}

#[cfg(test)]
mod tests {
    #[test]
    fn test_config_default() {
        use super::TestConfig;
        let config = TestConfig::default();
        assert_eq!(config.event_count, 1000);
        assert_eq!(config.producer_count, 2);
        assert_eq!(config.consumer_count, 2);
    }

    #[test]
    fn test_metrics_default() {
        use super::TestMetrics;
        let metrics = TestMetrics::default();
        assert_eq!(metrics.events_published, 0);
        assert_eq!(metrics.events_consumed, 0);
        assert_eq!(metrics.publish_throughput, 0.0);
        assert_eq!(metrics.consume_throughput, 0.0);
    }
}

// Copyright (c) Microsoft Corporation. All rights reserved.
// Licensed under the MIT License.

//! Real AMQP loopback validation of public clients, with and without providers.

use azure_core::{time::Duration as AzureDuration, tracing::TracerProvider};
use azure_core_opentelemetry::OpenTelemetryTracerProvider;
use azure_core_test::credentials::MockCredential;
use azure_messaging_eventhubs::{
    ConsumerClient, EventDataBatchOptions, ProducerClient, RetryOptions, SendEventOptions,
};
use fe2o3_amqp::{
    acceptor::{
        ConnectionAcceptor, LinkAcceptor, LinkEndpoint, SaslAnonymousMechanism, SessionAcceptor,
    },
    types::{
        messaging::{
            message::__private::Deserializable, ApplicationProperties, Body, Message, Properties,
        },
        primitives::Value,
    },
};
use futures::StreamExt;
use opentelemetry_appender_tracing::layer::OpenTelemetryTracingBridge;
use opentelemetry_sdk::{
    logs::{InMemoryLogExporter, SdkLoggerProvider},
    metrics::{
        data::{AggregatedMetrics, MetricData},
        InMemoryMetricExporter, SdkMeterProvider,
    },
    trace::{InMemorySpanExporter, SdkTracerProvider},
};
use std::{
    fmt::Write,
    future::pending,
    sync::{Arc, Mutex},
    time::Duration,
};
use tokio::{net::TcpListener, task::JoinSet, time::timeout};
use tracing_subscriber::{layer::SubscriberExt, util::SubscriberInitExt, EnvFilter, Layer};

struct Telemetry {
    traces: SdkTracerProvider,
    spans: InMemorySpanExporter,
    logs: SdkLoggerProvider,
    records: InMemoryLogExporter,
    meters: SdkMeterProvider,
    metrics: InMemoryMetricExporter,
}

impl Telemetry {
    fn new() -> Self {
        let spans = InMemorySpanExporter::default();
        let traces = SdkTracerProvider::builder()
            .with_simple_exporter(spans.clone())
            .build();
        let records = InMemoryLogExporter::default();
        let logs = SdkLoggerProvider::builder()
            .with_simple_exporter(records.clone())
            .build();
        let metrics = InMemoryMetricExporter::default();
        let meters = SdkMeterProvider::builder()
            .with_periodic_exporter(metrics.clone())
            .build();
        opentelemetry::global::set_meter_provider(meters.clone());
        tracing_subscriber::registry()
            .with(
                OpenTelemetryTracingBridge::new(&logs)
                    .with_filter(EnvFilter::new("azure_messaging_eventhubs=info")),
            )
            .try_init()
            .unwrap();
        Self {
            traces,
            spans,
            logs,
            records,
            meters,
            metrics,
        }
    }

    fn provider(&self) -> Arc<dyn TracerProvider> {
        OpenTelemetryTracerProvider::new(Arc::new(self.traces.clone()))
    }

    fn validate_and_report(&self) -> String {
        self.traces.force_flush().unwrap();
        self.logs.force_flush().unwrap();
        self.meters.force_flush().unwrap();
        let spans = self.spans.get_finished_spans().unwrap();
        let sends: Vec<_> = spans
            .iter()
            .filter(|s| s.name == "ProducerClient.send")
            .collect();
        let receives: Vec<_> = spans
            .iter()
            .filter(|s| s.name == "EventReceiver.receive")
            .collect();
        assert_eq!(
            sends.len(),
            5,
            "three single sends, one batch, one rejected oversized event"
        );
        assert_eq!(receives.len(), 5);
        assert_eq!(
            sends
                .iter()
                .filter(|s| matches!(s.status, opentelemetry::trace::Status::Error { .. }))
                .count(),
            1
        );
        assert!(sends.iter().any(|span| {
            span.attributes.iter().any(|a| {
                a.key.as_str() == "messaging.batch.message_count" && a.value == 2_i64.into()
            })
        }));
        for (name, count) in [
            ("EventHubs.sender.attach", 1),
            ("EventHubs.receiver.attach", 1),
            ("EventHubs.send.disposition", 4),
            ("EventHubs.receive.wait", 5),
        ] {
            assert_eq!(
                spans.iter().filter(|span| span.name == name).count(),
                count,
                "{name}"
            );
        }
        for span in &spans {
            let parents = match span.name.as_ref() {
                "EventHubs.send.disposition" | "EventHubs.sender.attach" => Some(&sends),
                "EventHubs.receive.wait" | "EventHubs.receiver.attach" => Some(&receives),
                _ => None,
            };
            if let Some(parents) = parents {
                assert!(
                    parents.iter().any(|parent| parent.span_context.span_id()
                        == span.parent_span_id
                        && parent.span_context.trace_id() == span.span_context.trace_id()),
                    "{} must be a child of the logical operation",
                    span.name
                );
            }
        }
        let mut report = String::from("Scenario: three single sends, one two-event batch, one oversized send rejected locally, five echoed receives, clean shutdown.\n\nTRACES\n");
        for span in &spans {
            writeln!(report, "{} trace={} span={} parent={} kind={:?} status={:?} duration_ms={:.3} attributes={:?}",
                span.name, span.span_context.trace_id(), span.span_context.span_id(), span.parent_span_id,
                span.span_kind, span.status, span.end_time.duration_since(span.start_time).unwrap().as_secs_f64()*1000.0,
                span.attributes).unwrap();
        }
        let logs = self.records.get_emitted_logs().unwrap();
        assert!(logs.iter().any(|l| l.record.body().is_some()));
        writeln!(report, "\nLOGS").unwrap();
        for log in logs {
            writeln!(
                report,
                "severity={:?} trace_context={:?} body={:?} attributes={:?}",
                log.record.severity_text(),
                log.record.trace_context(),
                log.record.body(),
                log.record.attributes_iter().collect::<Vec<_>>()
            )
            .unwrap();
        }
        writeln!(report, "\nMETRICS").unwrap();
        let metrics = self.metrics.get_finished_metrics().unwrap();
        let mut operation_count = 0;
        let mut event_count = 0;
        for metric in metrics
            .iter()
            .flat_map(|r| r.scope_metrics())
            .flat_map(|s| s.metrics())
        {
            writeln!(report, "{} unit={}", metric.name(), metric.unit()).unwrap();
            match metric.data() {
                AggregatedMetrics::U64(MetricData::Sum(sum)) => {
                    for point in sum.data_points() {
                        writeln!(
                            report,
                            "  value={} attributes={:?}",
                            point.value(),
                            point.attributes().collect::<Vec<_>>()
                        )
                        .unwrap();
                        let operation = point
                            .attributes()
                            .find(|a| a.key.as_str() == "operation")
                            .map(|a| a.value.as_str().into_owned());
                        let outcome = point
                            .attributes()
                            .find(|a| a.key.as_str() == "outcome")
                            .map(|a| a.value.as_str().into_owned());
                        if metric.name() == "az.eventhubs.operations" {
                            operation_count += point.value();
                            let expected = match (operation.as_deref(), outcome.as_deref()) {
                                (Some("send"), Some("success")) => 4,
                                (Some("send"), Some("error")) => 1,
                                (Some("receive"), Some("success")) => 5,
                                other => panic!("unexpected operation metric {other:?}"),
                            };
                            assert_eq!(point.value(), expected);
                        }
                        if metric.name() == "az.eventhubs.events" {
                            event_count += point.value();
                            let expected = if outcome.as_deref() == Some("error") {
                                0
                            } else {
                                5
                            };
                            assert_eq!(point.value(), expected);
                        }
                    }
                }
                AggregatedMetrics::F64(MetricData::Histogram(histogram)) => {
                    for point in histogram.data_points() {
                        writeln!(
                            report,
                            "  count={} sum_seconds={} attributes={:?}",
                            point.count(),
                            point.sum(),
                            point.attributes().collect::<Vec<_>>()
                        )
                        .unwrap();
                    }
                }
                _ => panic!("unexpected metric aggregation"),
            }
        }
        #[cfg(feature = "metrics")]
        {
            assert_eq!(operation_count, 10);
            assert_eq!(event_count, 10);
        }
        #[cfg(not(feature = "metrics"))]
        {
            assert_eq!(operation_count, 0);
            assert_eq!(event_count, 0);
        }
        report
    }
}

/// A local peer authenticates synthetic tokens, decodes batches, and echoes events.
async fn round_trip(provider: Option<Arc<dyn TracerProvider>>) {
    let listener = TcpListener::bind("127.0.0.1:0").await.unwrap();
    let address = listener.local_addr().unwrap();
    let messages: Arc<Mutex<Vec<Vec<u8>>>> = Arc::default();
    let batch_sizes: Arc<Mutex<Vec<usize>>> = Arc::default();
    let mut peer = JoinSet::new();
    let peer_messages = messages.clone();
    let peer_batches = batch_sizes.clone();
    peer.spawn(async move {
        let mut connections = JoinSet::new();
        loop {
            let (socket, _) = listener.accept().await.unwrap();
            let messages = peer_messages.clone();
            let batches = peer_batches.clone();
            connections.spawn(async move {
                let mut connection = ConnectionAcceptor::builder()
                    .container_id("observability-peer")
                    .sasl_acceptor(SaslAnonymousMechanism {})
                    .build()
                    .accept(socket)
                    .await
                    .unwrap();
                let mut sessions = JoinSet::new();
                while let Ok(mut session) = SessionAcceptor::new().accept(&mut connection).await {
                    let messages = messages.clone();
                    let batches = batches.clone();
                    sessions.spawn(async move {
                        let acceptor = LinkAcceptor::builder().max_message_size(1024_u64).build();
                        let Ok(link) = acceptor.accept(&mut session).await else {
                            // SDK shutdown can retire a session before attaching a link.
                            return;
                        };
                        match link {
                            LinkEndpoint::Receiver(mut receiver) => {
                                let target = receiver
                                    .target()
                                    .as_ref()
                                    .unwrap()
                                    .address
                                    .as_deref()
                                    .unwrap_or("")
                                    .to_string();
                                if target == "$cbs" {
                                    let LinkEndpoint::Sender(mut sender) =
                                        acceptor.accept(&mut session).await.unwrap()
                                    else {
                                        panic!("CBS response sender expected")
                                    };
                                    while let Ok(delivery) = receiver.recv::<Body<Value>>().await {
                                        let id = delivery
                                            .message()
                                            .properties
                                            .as_ref()
                                            .unwrap()
                                            .message_id
                                            .clone()
                                            .unwrap();
                                        receiver.accept(&delivery).await.unwrap();
                                        let response = Message::builder()
                                            .properties(
                                                Properties::builder().correlation_id(id).build(),
                                            )
                                            .application_properties(
                                                ApplicationProperties::builder()
                                                    .insert("status-code", 202u32)
                                                    .insert("status-description", "Accepted")
                                                    .build(),
                                            )
                                            .value(Value::Null)
                                            .build();
                                        sender.send(response).await.unwrap();
                                    }
                                } else {
                                    while let Ok(delivery) = receiver.recv::<Body<Value>>().await {
                                        let Body::Data(data) = delivery.body() else {
                                            panic!("binary body expected")
                                        };
                                        if *delivery.message_format() == Some(0x80013700) {
                                            batches.lock().unwrap().push(data.len());
                                            for encoded in data.iter() {
                                                let decoded: Deserializable<Message<Body<Value>>> =
                                                    serde_amqp::from_slice(encoded.0.as_ref())
                                                        .unwrap();
                                                let Body::Data(body) = decoded.0.body else {
                                                    panic!("binary event body expected")
                                                };
                                                messages.lock().unwrap().push(body[0].0.to_vec());
                                            }
                                        } else {
                                            messages.lock().unwrap().push(data[0].0.to_vec());
                                        }
                                        receiver.accept(&delivery).await.unwrap();
                                    }
                                }
                            }
                            LinkEndpoint::Sender(mut sender) => {
                                let bodies = messages.lock().unwrap().clone();
                                for body in bodies {
                                    sender
                                        .send(Message::builder().data(body).build())
                                        .await
                                        .unwrap();
                                }
                                pending::<()>().await;
                            }
                        }
                    });
                }
                // Keep established sessions alive until test cleanup.
                pending::<()>().await;
            });
        }
    });

    let endpoint = format!("amqp://{address}");
    let options = RetryOptions {
        max_retries: 0,
        max_total_elapsed: AzureDuration::seconds(3),
        ..Default::default()
    };
    let mut builder = ProducerClient::builder()
        .with_custom_endpoint(endpoint.clone())
        .with_retry_options(options.clone());
    if let Some(provider) = &provider {
        builder = builder.with_tracing_provider(provider.clone());
    }
    let producer = builder
        .open("local.test", "hub", Arc::new(MockCredential))
        .await
        .unwrap();
    for body in ["alpha", "bravo", "charlie"] {
        producer
            .send_event(
                body,
                Some(SendEventOptions {
                    partition_id: Some("0".into()),
                }),
            )
            .await
            .unwrap();
    }
    let batch = producer
        .create_batch(Some(EventDataBatchOptions {
            partition_id: Some("0".into()),
            ..Default::default()
        }))
        .await
        .unwrap();
    assert!(batch.try_add_event_data("batch-one", None).unwrap());
    assert!(batch.try_add_event_data("batch-two", None).unwrap());
    producer.send_batch(batch, None).await.unwrap();
    assert!(producer
        .send_event(
            vec![b'x'; 2048],
            Some(SendEventOptions {
                partition_id: Some("0".into())
            })
        )
        .await
        .is_err());
    assert_eq!(*batch_sizes.lock().unwrap(), vec![2]);
    assert_eq!(
        *messages.lock().unwrap(),
        [
            b"alpha".to_vec(),
            b"bravo".to_vec(),
            b"charlie".to_vec(),
            b"batch-one".to_vec(),
            b"batch-two".to_vec()
        ]
    );

    let mut builder = ConsumerClient::builder()
        .with_custom_endpoint(endpoint)
        .with_retry_options(options);
    if let Some(provider) = provider {
        builder = builder.with_tracing_provider(provider);
    }
    let consumer = builder
        .open("local.test", "hub".into(), Arc::new(MockCredential))
        .await
        .unwrap();
    let receiver = consumer
        .open_receiver_on_partition("0".into(), None)
        .await
        .unwrap();
    {
        let stream = receiver.stream_events();
        futures::pin_mut!(stream);
        for expected in ["alpha", "bravo", "charlie", "batch-one", "batch-two"] {
            let event = stream.next().await.unwrap().unwrap();
            assert_eq!(event.event_data().body().unwrap(), expected.as_bytes());
        }
    }
    consumer.close().await.unwrap();
    producer.close().await.unwrap();
    peer.abort_all();
}

#[tokio::test]
async fn loopback_send_receive_observability() {
    let no_provider = std::env::var_os("EVENTHUBS_OBSERVABILITY_NO_PROVIDER").is_some();
    let telemetry = (!no_provider).then(Telemetry::new);
    timeout(
        Duration::from_secs(15),
        round_trip(telemetry.as_ref().map(Telemetry::provider)),
    )
    .await
    .expect("loopback scenario timed out");
    let report = telemetry.as_ref().map(Telemetry::validate_and_report).unwrap_or_else(||
        format!("NO PROVIDERS CONFIGURED: successful public-client open, three single sends, two-event batch send, oversize error, five echoed receives, close; metrics feature enabled={}\n", cfg!(feature = "metrics")));
    println!("{report}");
    if let Some(directory) = std::env::var_os("EVENTHUBS_OBSERVABILITY_ARTIFACT_DIR") {
        let directory = std::path::PathBuf::from(directory);
        std::fs::create_dir_all(&directory).unwrap();
        std::fs::write(
            directory.join(if no_provider {
                "no-provider.txt"
            } else {
                "telemetry.txt"
            }),
            report,
        )
        .unwrap();
    }
}

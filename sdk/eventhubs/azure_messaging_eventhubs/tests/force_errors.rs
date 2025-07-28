// Copyright (c) Microsoft Corporation. All Rights reserved
// Licensed under the MIT license.

use azure_core::{sleep::sleep, time::Duration, Result};
use azure_core_amqp::error::AmqpErrorKind;
use azure_core_test::{recorded, TestContext};
use azure_messaging_eventhubs::{
    models::EventData,
    {EventDataBatchOptions, ProducerClient},
};
use std::{env, sync::Arc};
use tokio::select;
use tracing::info;

/// This function is used to force errors in an EventHubs client.
///
/// It will run the provided test function and then force an error on the producer after a stable duration.
async fn force_errors<C: Clone, T: AsyncFn(C), E: Fn(C)>(
    context: C,
    test: T,
    force_error: E,
    force_error_duration: Duration,
    test_duration: Duration,
) -> Result<()> {
    // This function is used to force errors in the event hub producer.
    // It will be used in tests to ensure that the producer can handle errors gracefully.
    select! {
        // Run the test function. Normally this function is not expected to return.
        _ = async {
            test(context.clone()).await;
            info!("Test completed successfully");
        } => {
            info!("Returning from test");
            Ok::<(),azure_core::Error>(())
        },
        // Force an error on the producer after waiting for the client to stabilize.
        _ = async {
        sleep(force_error_duration).await;
        info!("Forcing error on producer");
        force_error(context.clone());
        sleep(test_duration).await;

    } => { info!("Forcing error on producer"); Ok(()) }
    // Overall test duration - this ensures that we recover to a stable state with no errors.
    _ = sleep(test_duration) => { info!("Test expired"); Ok(()) }
    }
}

#[recorded::test(live)]
async fn force_errors_send_batch_link_error(ctx: TestContext) -> Result<()> {
    const EVENTHUB_PARTITION: &str = "1";
    const TEST_NAME: &str = "force_errors_send_batch_link_error";
    let recording = ctx.recording();
    let host = recording.var("EVENTHUBS_HOST", None);
    let eventhub = recording.var("EVENTHUB_NAME", None);
    let credential = recording.credential();
    let producer = Arc::new(
        ProducerClient::builder()
            .with_application_id(TEST_NAME.to_string())
            .open(host.as_str(), eventhub.as_str(), credential.clone())
            .await?,
    );

    force_errors(
        producer.clone(),
        |producer: Arc<ProducerClient>| {
            let producer = producer.clone();
            async move {
                loop {
                    let batch = producer
                        .create_batch(Some(EventDataBatchOptions {
                            partition_id: Some(EVENTHUB_PARTITION.to_string()),
                            partition_key: Some("My Partition Key.".to_string()),
                            ..Default::default()
                        }))
                        .await
                        .unwrap();

                    for i in 1..200 {
                        assert!(batch
                            .try_add_event_data(
                                EventData::builder()
                                    .with_body(b"Hello, World!")
                                    .add_property("Message#".to_string(), i)
                                    .with_message_id(i)
                                    .build(),
                                None
                            )
                            .unwrap());
                    }
                    producer.send_batch(batch, None).await.unwrap()
                }
            }
        },
        |producer| {
            producer
                .force_error(azure_core::Error::new(
                    azure_core::error::ErrorKind::Amqp,
                    azure_core_amqp::AmqpError::from(AmqpErrorKind::LinkClosedByRemote(Box::new(
                        azure_core::error::Error::new(
                            azure_core::error::ErrorKind::Other,
                            "Forced error",
                        ),
                    ))),
                ))
                .unwrap();
        },
        Duration::seconds(10), // Seconds until stable state.
        Duration::seconds(30), // Seconds until test timeout.
    )
    .await?;

    Ok(())
}

#[recorded::test(live)]
async fn force_errors_send_batch_session_error(ctx: TestContext) -> Result<()> {
    const EVENTHUB_PARTITION: &str = "1";
    const TEST_NAME: &str = "force_errors_send_batch_session_error";
    let recording = ctx.recording();
    let host = recording.var("EVENTHUBS_HOST", None);
    let eventhub = recording.var("EVENTHUB_NAME", None);
    let credential = recording.credential();
    let producer = Arc::new(
        ProducerClient::builder()
            .with_application_id(TEST_NAME.to_string())
            .open(host.as_str(), eventhub.as_str(), credential.clone())
            .await?,
    );

    force_errors(
        producer.clone(),
        |producer: Arc<ProducerClient>| {
            let producer = producer.clone();
            async move {
                loop {
                    let batch = producer
                        .create_batch(Some(EventDataBatchOptions {
                            partition_id: Some(EVENTHUB_PARTITION.to_string()),
                            partition_key: Some("My Partition Key.".to_string()),
                            ..Default::default()
                        }))
                        .await
                        .unwrap();

                    for i in 1..200 {
                        assert!(batch
                            .try_add_event_data(
                                EventData::builder()
                                    .with_body(b"Hello, World!")
                                    .add_property("Message#".to_string(), i)
                                    .with_message_id(i)
                                    .build(),
                                None
                            )
                            .unwrap());
                    }
                    producer.send_batch(batch, None).await.unwrap()
                }
            }
        },
        |producer| {
            producer
                .force_error(azure_core::Error::new(
                    azure_core::error::ErrorKind::Amqp,
                    azure_core_amqp::AmqpError::from(AmqpErrorKind::SessionDetachedByRemote(
                        Box::new(azure_core::error::Error::new(
                            azure_core::error::ErrorKind::Other,
                            "Forced error",
                        )),
                    )),
                ))
                .unwrap();
        },
        Duration::seconds(10), // Seconds until stable state.
        Duration::seconds(30), // Seconds until test timeout.
    )
    .await?;

    Ok(())
}

#[recorded::test(live)]
async fn force_errors_send_batch_connection_error(ctx: TestContext) -> Result<()> {
    const EVENTHUB_PARTITION: &str = "1";
    const TEST_NAME: &str = "force_errors_send_batch_connection_error";
    let recording = ctx.recording();
    let host = recording.var("EVENTHUBS_HOST", None);
    let eventhub = recording.var("EVENTHUB_NAME", None);
    let credential = recording.credential();
    let producer = Arc::new(
        ProducerClient::builder()
            .with_application_id(TEST_NAME.to_string())
            .open(host.as_str(), eventhub.as_str(), credential.clone())
            .await?,
    );

    force_errors(
        producer.clone(),
        |producer: Arc<ProducerClient>| {
            let producer = producer.clone();
            async move {
                loop {
                    let batch = producer
                        .create_batch(Some(EventDataBatchOptions {
                            partition_id: Some(EVENTHUB_PARTITION.to_string()),
                            partition_key: Some("My Partition Key.".to_string()),
                            ..Default::default()
                        }))
                        .await
                        .unwrap();

                    for i in 1..200 {
                        assert!(batch
                            .try_add_event_data(
                                EventData::builder()
                                    .with_body(b"Hello, World!")
                                    .add_property("Message#".to_string(), i)
                                    .with_message_id(i)
                                    .build(),
                                None
                            )
                            .unwrap());
                    }
                    producer.send_batch(batch, None).await.unwrap()
                }
            }
        },
        |producer| {
            producer
                .force_error(azure_core::Error::new(
                    azure_core::error::ErrorKind::Amqp,
                    azure_core_amqp::AmqpError::from(AmqpErrorKind::ConnectionClosedByRemote(
                        Box::new(azure_core::error::Error::new(
                            azure_core::error::ErrorKind::Other,
                            "Forced error",
                        )),
                    )),
                ))
                .unwrap();
        },
        Duration::seconds(10), // Seconds until forcing the error.
        Duration::seconds(30), // Seconds until test timeout.
    )
    .await?;

    Ok(())
}

#[recorded::test(live)]
async fn force_errors_producer_properties_connection(ctx: TestContext) -> Result<()> {
    const TEST_NAME: &str = "force_errors_producer_properties_connection";
    let recording = ctx.recording();
    let host = recording.var("EVENTHUBS_HOST", None);
    let eventhub = recording.var("EVENTHUB_NAME", None);
    let credential = recording.credential();
    let producer = Arc::new(
        ProducerClient::builder()
            .with_application_id(TEST_NAME.to_string())
            .open(host.as_str(), eventhub.as_str(), credential.clone())
            .await?,
    );

    force_errors(
        producer.clone(),
        |producer: Arc<ProducerClient>| {
            let producer = producer.clone();
            async move {
                loop {
                    producer.get_eventhub_properties().await.unwrap();
                }
            }
        },
        |producer| {
            producer
                .force_error(azure_core::Error::new(
                    azure_core::error::ErrorKind::Amqp,
                    azure_core_amqp::AmqpError::from(AmqpErrorKind::ConnectionClosedByRemote(
                        Box::new(azure_core::error::Error::new(
                            azure_core::error::ErrorKind::Other,
                            "Forced error",
                        )),
                    )),
                ))
                .unwrap();
        },
        Duration::seconds(10), // Seconds until forcing the error.
        Duration::seconds(20), // Seconds until test timeout.
    )
    .await?;

    Ok(())
}

#[recorded::test(live)]
async fn force_errors_producer_properties_session(ctx: TestContext) -> Result<()> {
    const TEST_NAME: &str = "force_errors_producer_properties_session";
    let recording = ctx.recording();
    let host = recording.var("EVENTHUBS_HOST", None);
    let eventhub = recording.var("EVENTHUB_NAME", None);
    let credential = recording.credential();
    let producer = Arc::new(
        ProducerClient::builder()
            .with_application_id(TEST_NAME.to_string())
            .open(host.as_str(), eventhub.as_str(), credential.clone())
            .await?,
    );

    force_errors(
        producer.clone(),
        |producer: Arc<ProducerClient>| {
            let producer = producer.clone();
            async move {
                loop {
                    producer.get_eventhub_properties().await.unwrap();
                }
            }
        },
        |producer| {
            producer
                .force_error(azure_core::Error::new(
                    azure_core::error::ErrorKind::Amqp,
                    azure_core_amqp::AmqpError::from(AmqpErrorKind::SessionClosedByRemote(
                        Box::new(azure_core::error::Error::new(
                            azure_core::error::ErrorKind::Other,
                            "Forced error",
                        )),
                    )),
                ))
                .unwrap();
        },
        Duration::seconds(10), // Seconds until forcing the error.
        Duration::seconds(20), // Seconds until test timeout.
    )
    .await?;

    Ok(())
}

#[recorded::test(live)]
async fn force_errors_producer_properties_link(ctx: TestContext) -> Result<()> {
    const TEST_NAME: &str = "force_errors_producer_properties_link";
    let recording = ctx.recording();
    let host = recording.var("EVENTHUBS_HOST", None);
    let eventhub = recording.var("EVENTHUB_NAME", None);
    let credential = recording.credential();
    let producer = Arc::new(
        ProducerClient::builder()
            .with_application_id(TEST_NAME.to_string())
            .open(host.as_str(), eventhub.as_str(), credential.clone())
            .await?,
    );

    force_errors(
        producer.clone(),
        |producer: Arc<ProducerClient>| {
            let producer = producer.clone();
            async move {
                loop {
                    producer.get_eventhub_properties().await.unwrap();
                }
            }
        },
        |producer| {
            producer
                .force_error(azure_core::Error::new(
                    azure_core::error::ErrorKind::Amqp,
                    azure_core_amqp::AmqpError::from(AmqpErrorKind::LinkClosedByRemote(Box::new(
                        azure_core::error::Error::new(
                            azure_core::error::ErrorKind::Other,
                            "Forced error",
                        ),
                    ))),
                ))
                .unwrap();
        },
        Duration::seconds(10), // Seconds until forcing the error.
        Duration::seconds(20), // Seconds until test timeout.
    )
    .await?;

    Ok(())
}

#[recorded::test(live)]
async fn force_errors_consumer_properties_link(ctx: TestContext) -> Result<()> {
    use azure_messaging_eventhubs::ConsumerClient;

    const TEST_NAME: &str = "force_errors_consumer_properties_link";
    let recording = ctx.recording();
    let host = recording.var("EVENTHUBS_HOST", None);
    let eventhub = recording.var("EVENTHUB_NAME", None);
    let credential = recording.credential();
    let consumer = Arc::new(
        ConsumerClient::builder()
            .with_application_id(TEST_NAME.to_string())
            .open(host.as_str(), eventhub, credential.clone())
            .await?,
    );

    force_errors(
        consumer.clone(),
        |consumer: Arc<ConsumerClient>| {
            let consumer = consumer.clone();
            async move {
                loop {
                    consumer.get_eventhub_properties().await.unwrap();
                }
            }
        },
        |consumer| {
            consumer
                .force_error(azure_core::Error::new(
                    azure_core::error::ErrorKind::Amqp,
                    azure_core_amqp::AmqpError::from(AmqpErrorKind::LinkClosedByRemote(Box::new(
                        azure_core::error::Error::new(
                            azure_core::error::ErrorKind::Other,
                            "Forced error",
                        ),
                    ))),
                ))
                .unwrap();
        },
        Duration::seconds(10), // Seconds until forcing the error.
        Duration::seconds(20), // Seconds until test timeout.
    )
    .await?;

    Ok(())
}

#[recorded::test(live)]
async fn force_errors_consumer_properties_session(ctx: TestContext) -> Result<()> {
    use azure_messaging_eventhubs::ConsumerClient;

    const TEST_NAME: &str = "force_errors_consumer_properties_session";
    let recording = ctx.recording();
    let host = recording.var("EVENTHUBS_HOST", None);
    let eventhub = recording.var("EVENTHUB_NAME", None);
    let credential = recording.credential();
    let consumer = Arc::new(
        ConsumerClient::builder()
            .with_application_id(TEST_NAME.to_string())
            .open(host.as_str(), eventhub, credential.clone())
            .await?,
    );

    force_errors(
        consumer.clone(),
        |consumer: Arc<ConsumerClient>| {
            let consumer = consumer.clone();
            async move {
                loop {
                    consumer.get_eventhub_properties().await.unwrap();
                }
            }
        },
        |consumer| {
            consumer
                .force_error(azure_core::Error::new(
                    azure_core::error::ErrorKind::Amqp,
                    azure_core_amqp::AmqpError::from(AmqpErrorKind::SessionClosedByRemote(
                        Box::new(azure_core::error::Error::new(
                            azure_core::error::ErrorKind::Other,
                            "Forced error",
                        )),
                    )),
                ))
                .unwrap();
        },
        Duration::seconds(10), // Seconds until forcing the error.
        Duration::seconds(20), // Seconds until test timeout.
    )
    .await?;

    Ok(())
}
#[recorded::test(live)]
async fn force_errors_consumer_properties_connection(ctx: TestContext) -> Result<()> {
    use azure_messaging_eventhubs::ConsumerClient;

    const TEST_NAME: &str = "force_errors_consumer_properties_connection";
    let recording = ctx.recording();
    let host = recording.var("EVENTHUBS_HOST", None);
    let eventhub = recording.var("EVENTHUB_NAME", None);
    let credential = recording.credential();
    let consumer = Arc::new(
        ConsumerClient::builder()
            .with_application_id(TEST_NAME.to_string())
            .open(host.as_str(), eventhub, credential.clone())
            .await?,
    );

    force_errors(
        consumer.clone(),
        |consumer: Arc<ConsumerClient>| {
            let consumer = consumer.clone();
            async move {
                loop {
                    consumer.get_eventhub_properties().await.unwrap();
                }
            }
        },
        |consumer| {
            consumer
                .force_error(azure_core::Error::new(
                    azure_core::error::ErrorKind::Amqp,
                    azure_core_amqp::AmqpError::from(AmqpErrorKind::ConnectionClosedByRemote(
                        Box::new(azure_core::error::Error::new(
                            azure_core::error::ErrorKind::Other,
                            "Forced error",
                        )),
                    )),
                ))
                .unwrap();
        },
        Duration::seconds(10), // Seconds until forcing the error.
        Duration::seconds(20), // Seconds until test timeout.
    )
    .await?;

    Ok(())
}

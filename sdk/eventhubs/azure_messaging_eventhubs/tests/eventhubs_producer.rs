// Copyright (c) Microsoft Corporation. All Rights reserved
// Licensed under the MIT license.

use azure_core::http::StatusCode;
use azure_core_amqp::{message::AmqpMessageProperties, AmqpError, AmqpList, AmqpSimpleValue};
use azure_core_test::{recorded, TestContext};
use azure_messaging_eventhubs::{
    error::ErrorKind, EventDataBatchOptions, ProducerClient, SendEventOptions,
};
use std::{env, error::Error, sync::Arc};
use tracing::{info, trace};

#[recorded::test(live)]
async fn test_new(ctx: TestContext) -> Result<(), Box<dyn Error>> {
    let recording = ctx.recording();
    let host = env::var("EVENTHUBS_HOST")?;
    let eventhub = env::var("EVENTHUB_NAME")?;
    let credential = recording.credential();
    let _client = ProducerClient::builder()
        .with_application_id("test_new".to_string())
        .open(host.as_str(), eventhub.as_str(), credential.clone())
        .await?;

    Ok(())
}

#[recorded::test(live)]
async fn test_new_with_error(ctx: TestContext) -> Result<(), Box<dyn Error>> {
    let recording = ctx.recording();
    let eventhub = env::var("EVENTHUB_NAME")?;
    let result = ProducerClient::builder()
        .with_application_id("test_new_with_error".to_string())
        .open("invalid_host", eventhub.as_str(), recording.credential())
        .await;
    assert!(result.is_err());
    info!("Error: {:?}", result.err());

    Ok(())
}

#[recorded::test(live)]
async fn open(ctx: TestContext) -> Result<(), Box<dyn Error>> {
    let recording = ctx.recording();
    let host = env::var("EVENTHUBS_HOST")?;
    let eventhub = env::var("EVENTHUB_NAME")?;
    let credential = recording.credential();
    let _client = ProducerClient::builder()
        .with_application_id("test_open".to_string())
        .open(host.as_str(), eventhub.as_str(), credential.clone())
        .await?;

    Ok(())
}

#[recorded::test(live)]
async fn close(ctx: TestContext) -> Result<(), Box<dyn Error>> {
    let recording = ctx.recording();
    let host = env::var("EVENTHUBS_HOST")?;
    let eventhub = env::var("EVENTHUB_NAME")?;
    let credential = recording.credential();
    let client = ProducerClient::builder()
        .with_application_id("test_close".to_string())
        .open(host.as_str(), eventhub.as_str(), credential.clone())
        .await?;
    client.close().await?;

    Ok(())
}

#[recorded::test(live)]
async fn get_properties(ctx: TestContext) -> Result<(), Box<dyn Error>> {
    let recording = ctx.recording();
    let host = env::var("EVENTHUBS_HOST")?;
    let eventhub = env::var("EVENTHUB_NAME")?;

    let credential = recording.credential();

    let client = ProducerClient::builder()
        .with_application_id("test_get_properties".to_string())
        .open(host.as_str(), eventhub.as_str(), credential.clone())
        .await?;
    let properties = client.get_eventhub_properties().await?;
    info!("Properties: {:?}", properties);
    assert_eq!(properties.name, eventhub);

    client.close().await?;

    Ok(())
}

#[recorded::test(live)]
async fn get_partition_properties(ctx: TestContext) -> Result<(), Box<dyn Error>> {
    use azure_core_amqp::error::AmqpErrorKind;

    let recording = ctx.recording();
    let host = env::var("EVENTHUBS_HOST")?;
    let eventhub = env::var("EVENTHUB_NAME")?;

    let credential = recording.credential();

    let client = ProducerClient::builder()
        .with_application_id("test_get_partition_properties".to_string())
        .open(host.as_str(), eventhub.as_str(), credential.clone())
        .await?;
    let properties = client.get_eventhub_properties().await?;

    for partition_id in properties.partition_ids {
        let partition_properties = client.get_partition_properties(&partition_id).await?;
        info!("Partition properties: {:?}", partition_properties);
        assert_eq!(partition_properties.id, partition_id);
    }

    let result = client.get_partition_properties("partition_id").await;
    assert!(result.is_err());
    if let Err(err) = result {
        info!("Error: {:?}", err);
        let kind = &err.kind;
        assert!(matches!(
            kind,
            azure_messaging_eventhubs::error::ErrorKind::AmqpError(_)
        ));
        let amqp_error = err.source();
        assert!(amqp_error.is_some());
        let amqp_error = amqp_error.unwrap();
        assert!(amqp_error.is::<Box<AmqpError>>() || amqp_error.is::<AmqpError>());

        let amqp_error = if amqp_error.is::<Box<AmqpError>>() {
            let error = amqp_error.downcast_ref::<Box<AmqpError>>();
            assert!(error.is_some());
            error.unwrap().as_ref()
        } else {
            let error = amqp_error.downcast_ref::<AmqpError>();
            assert!(error.is_some());
            error.unwrap()
        };
        info!("AMQP error: {:?}", amqp_error);
        if let AmqpErrorKind::ManagementStatusCode(code, _) = amqp_error.kind() {
            assert_eq!(*code, StatusCode::BadRequest);
        } else {
            panic!("Expected AmqpErrorKind::ManagementStatusCode");
        }

        // Simplest form of the above:
        let amqp_error = err.source().unwrap().downcast_ref::<AmqpError>().unwrap();
        info!("AMQP error: {:?}", amqp_error);
    }

    client.close().await?;

    Ok(())
}

#[test]
fn create_eventdata() -> Result<(), Box<dyn Error>> {
    let data = b"hello world";
    let ed1 = azure_messaging_eventhubs::models::EventData::builder()
        .with_body(data.to_vec())
        .build();

    assert_eq!(ed1.body().unwrap(), data.to_vec());
    assert!(ed1.content_type().is_none());
    assert!(ed1.correlation_id().is_none());
    assert!(ed1.message_id().is_none());
    assert!(ed1.properties().is_none());

    let data = b"hello world";
    let _ = azure_messaging_eventhubs::models::EventData::builder()
        .with_body(data.to_vec())
        .with_content_type("text/plain".to_string())
        .with_correlation_id("correlation_id")
        .with_message_id(35u64)
        .add_property("key".to_string(), "value")
        .build();

    Ok(())
}

#[recorded::test(live)]
async fn send_eventdata(ctx: TestContext) -> Result<(), Box<dyn Error>> {
    let recording = ctx.recording();
    let host = env::var("EVENTHUBS_HOST")?;
    let eventhub = env::var("EVENTHUB_NAME")?;

    let credential = recording.credential();

    let client = ProducerClient::builder()
        .with_application_id("send_eventdata".to_string())
        .open(host.as_str(), eventhub.as_str(), credential.clone())
        .await?;
    {
        let data = b"hello world";
        let ed1 = azure_messaging_eventhubs::models::EventData::builder()
            .with_body(data.to_vec())
            .build();

        let res = client.send_event(ed1, None).await;
        assert!(res.is_ok());
    }
    {
        let data = b"hello world";
        let ed1 = azure_messaging_eventhubs::models::EventData::builder()
            .with_body(data.to_vec())
            .with_content_type("text/plain".to_string())
            .with_correlation_id("correlation_id")
            .with_message_id(35u64)
            .add_property("key".to_string(), "value")
            .build();

        let res = client.send_event(ed1, None).await;
        assert!(res.is_ok());
    }

    // Simple send.
    assert!(client.send_event("Hello, Event Hub!", None).await.is_ok());

    client.close().await?;

    Ok(())
}

#[recorded::test(live)]
async fn send_message(ctx: TestContext) -> Result<(), Box<dyn Error>> {
    use azure_messaging_eventhubs::models::{AmqpMessage, AmqpValue};
    let recording = ctx.recording();
    let host = env::var("EVENTHUBS_HOST")?;
    let eventhub = env::var("EVENTHUB_NAME")?;

    let credential = recording.credential();

    let client = ProducerClient::builder()
        .with_application_id("send_eventdata".to_string())
        .open(host.as_str(), eventhub.as_str(), credential.clone())
        .await?;
    {
        let data = b"hello world";
        let em1 = AmqpMessage::builder()
            .with_body(vec![data.to_vec()])
            .build();

        let res = client.send_message(em1, None).await;
        assert!(res.is_ok());
    }
    {
        let data = b"hello world";
        let em1 = AmqpMessage::builder()
            .with_body(AmqpValue::Binary(data.to_vec()))
            .add_application_property("key".to_string(), AmqpSimpleValue::from("value"))
            .with_properties(AmqpMessageProperties {
                message_id: Some(35u64.into()),
                content_type: Some("text/plain".into()),
                correlation_id: Some("correlation_id".into()),
                ..Default::default()
            })
            .build();

        let res = client.send_message(em1, None).await;
        assert!(res.is_ok());
    }

    // Simple send.
    assert!(client.send_event("Hello, Event Hub!", None).await.is_ok());

    client.close().await?;

    Ok(())
}

#[recorded::test(live)]
async fn test_create_batch(ctx: TestContext) -> Result<(), Box<dyn Error>> {
    let recording = ctx.recording();
    let host = env::var("EVENTHUBS_HOST")?;
    let eventhub = env::var("EVENTHUB_NAME")?;

    let credential = recording.credential();

    let client = ProducerClient::builder()
        .with_application_id("test_create_batch".to_string())
        .open(host.as_str(), eventhub.as_str(), credential.clone())
        .await?;
    {
        let batch = client.create_batch(None).await?;
        assert_eq!(batch.len(), 0);
    }
    client.close().await?;

    Ok(())
}

#[recorded::test(live)]
async fn test_create_and_send_batch(ctx: TestContext) -> Result<(), Box<dyn Error>> {
    let recording = ctx.recording();
    let host = env::var("EVENTHUBS_HOST")?;
    let eventhub = env::var("EVENTHUB_NAME")?;

    let credential = recording.credential();

    let client = ProducerClient::builder()
        .with_application_id("test_create_and_send_batch".to_string())
        .open(host.as_str(), eventhub.as_str(), credential.clone())
        .await?;

    {
        let batch = client.create_batch(None).await?;
        assert_eq!(batch.len(), 0);
        assert!(batch.try_add_event_data(vec![1, 2, 3, 4], None)?);

        let res = client.send_batch(batch, None).await;
        assert!(res.is_ok());
    }
    {
        let batch = client
            .create_batch(Some(EventDataBatchOptions {
                partition_id: Some("0".to_string()),
                ..Default::default()
            }))
            .await?;
        for i in 0..10 {
            let res = batch.try_add_event_data(vec![i as u8], None)?;
            assert!(res);
        }
        assert!(batch.try_add_event_data("This is data", None)?);
        assert!(batch.try_add_event_data([23], None)?);
        assert!(batch.try_add_event_data(vec![1, 2, 4, 8], None)?);
        assert!(batch.try_add_event_data("&data", None)?);
        assert!(batch.try_add_event_data("&data", None)?);
        assert!(batch.try_add_event_data("&data", None)?);
        assert!(batch.try_add_event_data("&data", None)?);
        assert!(batch.try_add_event_data("&data", None)?);

        let res = client.send_batch(batch, None).await;
        assert!(res.is_ok());
    }

    client.close().await?;

    Ok(())
}

#[recorded::test(live)]
async fn test_add_amqp_messages_to_batch(
    ctx: TestContext,
) -> Result<(), Box<dyn std::error::Error>> {
    use azure_messaging_eventhubs::models::{AmqpMessage, AmqpValue};

    let recording = ctx.recording();
    let host = env::var("EVENTHUBS_HOST")?;
    let eventhub = env::var("EVENTHUB_NAME")?;

    let credential = recording.credential();

    let client = ProducerClient::builder()
        .with_application_id("test_add_amqp_messages_to_batch".to_string())
        .open(host.as_str(), eventhub.as_str(), credential.clone())
        .await?;

    let batch = client.create_batch(None).await?;
    assert_eq!(batch.len(), 0);

    // Message with AMQP Value body
    assert!(batch.try_add_amqp_message(
        AmqpMessage::builder()
            .with_body(AmqpValue::from("This is data"))
            .build(),
        None
    )?);

    // Shortcut message creation logic.
    assert!(batch.try_add_amqp_message(AmqpValue::from("This is a value"), None)?);

    // Message with binary body and application property
    assert!(batch.try_add_amqp_message(
        AmqpMessage::builder()
            .with_body(vec![1, 2, 3, 4])
            .add_application_property("MessageName".to_string(), "Frederick")
            .build(),
        None
    )?);

    // Shortcut message creation logic.
    assert!(batch.try_add_amqp_message(vec![3, 5, 7], None)?);

    // Message with sequence body and application property
    assert!(batch.try_add_amqp_message(
        AmqpMessage::builder()
            .with_body(vec![
                AmqpValue::from(1),
                AmqpValue::from(2),
                AmqpValue::from(3)
            ])
            .add_application_property("MessageName".to_string(), "Frederick")
            .build(),
        None
    )?);

    // Shortcut message creation logic.
    assert!(batch.try_add_amqp_message(
        AmqpList::from(vec![
            AmqpValue::from(3),
            AmqpValue::from(5),
            AmqpValue::from(7)
        ]),
        None
    )?);

    client.send_batch(batch, None).await?;

    client.close().await?;

    Ok(())
}

#[recorded::test(live)]
async fn test_overload_batch(ctx: TestContext) -> Result<(), Box<dyn Error>> {
    let recording = ctx.recording();

    let host = env::var("EVENTHUBS_HOST")?;
    let eventhub = env::var("EVENTHUB_NAME")?;

    let credential = recording.credential();

    info!("Create producer client...");

    let client = ProducerClient::builder()
        .with_application_id("test_overload_batch".to_string())
        .open(host.as_str(), eventhub.as_str(), credential.clone())
        .await?;

    info!("Client is open.");
    {
        let mut batch = client
            .create_batch(Some(EventDataBatchOptions {
                partition_id: Some("0".to_string()),
                ..Default::default()
            }))
            .await?;
        trace!("Batch created.");
        for i in 0..25_000 {
            if i % 5_000 == 0 {
                info!("Add event data, now at {}", i);
                info!("Batch size: {}", batch.size());
            }
            if !batch.try_add_event_data(format!("Message {i}"), None)? {
                info!(
                    "Batch is full at {i} ({} bytes), sending batch",
                    batch.size()
                );
                let result = client.send_batch(batch, None).await;
                if result.is_err() {
                    info!("Batch submit failed. {:?}", result);
                }
                assert!(result.is_ok());
                // Recreate the batch to continue adding messages
                batch = client
                    .create_batch(Some(EventDataBatchOptions {
                        partition_id: Some("0".to_string()),
                        ..Default::default()
                    }))
                    .await?;
            }
        }
        let result = client.send_batch(batch, None).await;
        if result.is_err() {
            info!("Batch submit failed. {:?}", result);
        }
        assert!(result.is_ok());
    }

    client.close().await?;

    Ok(())
}

#[recorded::test(live)]
async fn send_eventdata_with_connection_string(_ctx: TestContext) -> Result<(), Box<dyn Error>> {
    // The SAS credential is derived from the connection string, so no
    // `recording.credential()` is needed. `EVENTHUB_NAME` is only required when
    // the connection string carries no `EntityPath`.
    let connection_string = env::var("EVENTHUBS_CONNECTION_STRING")?;
    let eventhub = env::var("EVENTHUB_NAME").ok();

    let client = ProducerClient::builder()
        .with_application_id("send_eventdata_with_connection_string".to_string())
        .open_with_connection_string(&connection_string, eventhub.as_deref())
        .await?;

    assert!(client
        .send_event("Hello from a SAS connection string!", None)
        .await
        .is_ok());

    client.close().await?;

    Ok(())
}

/// Sends to every partition at the same time from one client.
///
/// Each send attaches a sender, and each attach needs a claims-based-security
/// authorization. The service permits one `$cbs` link for each connection, so
/// the authorizations must run in sequence. Before that fix, the service
/// answered the overlapping authorizations with `NotAllowed` ("A link to
/// connection ... $cbs node has already been opened"), which the client
/// classifies as not retryable, and the sends failed.
#[recorded::test(live)]
async fn send_to_every_partition_at_once(ctx: TestContext) -> Result<(), Box<dyn Error>> {
    let recording = ctx.recording();

    let host = env::var("EVENTHUBS_HOST")?;
    let eventhub = env::var("EVENTHUB_NAME")?;

    let client = Arc::new(
        ProducerClient::builder()
            .with_application_id("send_to_every_partition_at_once".to_string())
            .open(host.as_str(), eventhub.as_str(), recording.credential())
            .await?,
    );

    // Read the partitions first, so the connection is open and only the sender
    // attaches overlap.
    let partitions = client.get_eventhub_properties().await?.partition_ids;
    assert!(partitions.len() > 1, "the test needs many partitions");
    info!("Send to {} partitions at the same time.", partitions.len());

    let mut tasks = Vec::new();
    for partition in partitions.iter() {
        let client = client.clone();
        let partition = partition.clone();
        tasks.push(tokio::spawn(async move {
            let result = client
                .send_event(
                    format!("Hello, partition {partition}!"),
                    Some(SendEventOptions {
                        partition_id: Some(partition.clone()),
                    }),
                )
                .await;
            (partition, result)
        }));
    }

    let mut failures = Vec::new();
    for task in tasks {
        let (partition, result) = task.await?;
        if let Err(e) = result {
            info!("Partition {partition} failed. {e:?}");
            failures.push(partition);
        }
    }

    Arc::try_unwrap(client)
        .map_err(|_| "A task still holds the client.")?
        .close()
        .await?;

    assert!(
        failures.is_empty(),
        "{} of {} sends failed: {failures:?}",
        failures.len(),
        partitions.len()
    );

    Ok(())
}

/// A batch created with a maximum size must enforce that size.
///
/// `attach` used to replace the size the caller asked for with the maximum the
/// sender link reports, so a small cap had no effect and the batch accepted
/// events well past it. This test pins the wiring, not just the arithmetic: it
/// fails if `create_batch` stops using the size the caller supplied.
#[recorded::test(live)]
async fn create_batch_honors_max_size_in_bytes(ctx: TestContext) -> Result<(), Box<dyn Error>> {
    const MAX_SIZE: u64 = 1024;

    let recording = ctx.recording();
    let host = env::var("EVENTHUBS_HOST")?;
    let eventhub = env::var("EVENTHUB_NAME")?;

    let client = ProducerClient::builder()
        .with_application_id("create_batch_honors_max_size_in_bytes".to_string())
        .open(host.as_str(), eventhub.as_str(), recording.credential())
        .await?;

    let batch = client
        .create_batch(Some(EventDataBatchOptions {
            max_size_in_bytes: Some(MAX_SIZE),
            partition_id: Some("0".to_string()),
            ..Default::default()
        }))
        .await?;

    // Add 128 byte events until one is refused. A 1 KiB cap is reached well
    // before this bound; the bound only stops the loop if the cap is ignored.
    let body = "x".repeat(128);
    let mut refused_at = None;
    for i in 0..64 {
        if !batch.try_add_event_data(body.clone(), None)? {
            refused_at = Some(i);
            break;
        }
    }

    let refused_at = refused_at.expect("a batch capped at 1024 bytes must refuse an event");
    // The cap must be the one the caller asked for, not some smaller value the
    // resolution got wrong. A batch that refuses the first event is as broken
    // as one that never refuses.
    assert!(
        refused_at > 0,
        "a 1024 byte batch refused the first 128 byte event, so the cap it got was too small"
    );
    assert!(
        batch.size() <= MAX_SIZE,
        "batch grew to {} bytes, past its {MAX_SIZE} byte cap",
        batch.size()
    );
    assert!(
        refused_at < 16,
        "a 1024 byte batch accepted {refused_at} events of 128 bytes, so the cap was ignored"
    );
    assert_eq!(
        refused_at,
        batch.len(),
        "every event before the refused one must be in the batch"
    );
    info!(
        "Batch refused event {refused_at} at {} bytes.",
        batch.size()
    );

    // The batch is never sent, so this costs nothing on the service side.
    client.close().await?;

    Ok(())
}

/// A maximum size larger than the link allows must be reported, not reduced.
///
/// The .NET, Go, and Java clients all report an error here. Reducing the value
/// silently would hide that the requested size was impossible.
#[recorded::test(live)]
async fn create_batch_rejects_size_above_link_maximum(
    ctx: TestContext,
) -> Result<(), Box<dyn Error>> {
    // Far above any Event Hubs link maximum, which is about 1 MiB.
    const TOO_LARGE: u64 = 512 * 1024 * 1024;

    let recording = ctx.recording();
    let host = env::var("EVENTHUBS_HOST")?;
    let eventhub = env::var("EVENTHUB_NAME")?;

    let client = ProducerClient::builder()
        .with_application_id("create_batch_rejects_size_above_link_maximum".to_string())
        .open(host.as_str(), eventhub.as_str(), recording.credential())
        .await?;

    let result = client
        .create_batch(Some(EventDataBatchOptions {
            max_size_in_bytes: Some(TOO_LARGE),
            partition_id: Some("0".to_string()),
            ..Default::default()
        }))
        .await;

    let error = result
        .err()
        .expect("a size above the link maximum must be rejected");
    // The caller must be able to branch on this without reading the message.
    assert!(
        matches!(
            error.kind,
            ErrorKind::InvalidBatchSize {
                requested: TOO_LARGE,
                ..
            }
        ),
        "the error must report the batch size kind, got: {error:?}"
    );
    let message = error.to_string();
    assert!(
        message.contains(&TOO_LARGE.to_string()),
        "the error must name the requested size, got: {message}"
    );
    info!("Rejected as expected: {message}");

    client.close().await?;

    Ok(())
}

/// An event larger than the sender link allows must be refused, not sent.
///
/// Both public entry points must refuse it, and the link must stay usable.
#[recorded::test(live)]
async fn send_event_rejects_message_above_link_maximum(
    ctx: TestContext,
) -> Result<(), Box<dyn Error>> {
    use azure_messaging_eventhubs::models::{AmqpMessage, EventData};

    // The size the live reproduction of issue #5101 used, against an Event Hubs
    // link maximum of 1048576 bytes.
    const TOO_LARGE_BODY: usize = 2 * 1024 * 1024;

    let recording = ctx.recording();
    let host = env::var("EVENTHUBS_HOST")?;
    let eventhub = env::var("EVENTHUB_NAME")?;

    let client = ProducerClient::builder()
        .with_application_id("send_event_rejects_message_above_link_maximum".to_string())
        .open(host.as_str(), eventhub.as_str(), recording.credential())
        .await?;

    let error = client
        .send_event(
            EventData::builder()
                .with_body(vec![b'x'; TOO_LARGE_BODY])
                .build(),
            None,
        )
        .await
        .err()
        .expect("an event above the link maximum must be refused");
    assert!(
        matches!(error.kind, ErrorKind::MessageSizeExceeded { .. }),
        "send_event must report the message size kind, got: {error:?}"
    );
    info!("send_event refused the large event: {error}");

    let error = client
        .send_message(
            AmqpMessage::builder()
                .with_body(vec![vec![b'x'; TOO_LARGE_BODY]])
                .build(),
            None,
        )
        .await
        .err()
        .expect("a message above the link maximum must be refused");
    assert!(
        matches!(error.kind, ErrorKind::MessageSizeExceeded { .. }),
        "send_message must report the message size kind, got: {error:?}"
    );

    // The refusal applies to the one large message. A normal event on the same
    // client must still go, which also shows the link is still up.
    client.send_event("Hello, Event Hub!", None).await?;

    client.close().await?;

    Ok(())
}

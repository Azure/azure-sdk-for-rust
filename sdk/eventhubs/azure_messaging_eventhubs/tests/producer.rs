// Copyright (c) Microsoft Corporation. All Rights reserved
// Licensed under the MIT license.

//cspell: words eventdata amqp

use azure_core_amqp::{
    messaging::{AmqpMessage, AmqpMessageBody},
    value::{AmqpList, AmqpValue},
};
use azure_core_test::recorded;
use azure_identity::DefaultAzureCredential;
use azure_messaging_eventhubs::producer::{
    batch::EventDataBatchOptions, ProducerClient, ProducerClientOptions,
};
use std::{env, error::Error};
use tracing::{info, trace};

mod common;

#[recorded::test(live)]
async fn test_new() -> Result<(), Box<dyn Error>> {
    common::setup();
    let host = env::var("EVENTHUBS_HOST")?;
    let eventhub = env::var("EVENTHUB_NAME")?;
    let _client = ProducerClient::new(
        host,
        eventhub,
        DefaultAzureCredential::new()?,
        Some(ProducerClientOptions {
            application_id: Some("test_new".to_string()),
            ..Default::default()
        }),
    );

    Ok(())
}

#[recorded::test(live)]
async fn test_new_with_error() -> Result<(), Box<dyn Error>> {
    common::setup();
    let eventhub = env::var("EVENTHUB_NAME")?;
    let producer = ProducerClient::new(
        "invalid_host".to_string(),
        eventhub,
        azure_identity::DefaultAzureCredential::new()?,
        Some(ProducerClientOptions {
            application_id: Some("test_new_with_error".to_string()),
            ..Default::default()
        }),
    );
    let result = producer.open().await;
    assert!(result.is_err());
    info!("Error: {:?}", result);

    Ok(())
}

#[recorded::test(live)]
async fn test_open() -> Result<(), Box<dyn Error>> {
    common::setup();
    let host = env::var("EVENTHUBS_HOST")?;
    let eventhub = env::var("EVENTHUB_NAME")?;
    let client = ProducerClient::new(
        host,
        eventhub,
        azure_identity::DefaultAzureCredential::new()?,
        Some(ProducerClientOptions {
            application_id: Some("test_open".to_string()),
            ..Default::default()
        }),
    );
    client.open().await?;

    Ok(())
}
#[recorded::test(live)]
async fn test_close() -> Result<(), Box<dyn Error>> {
    common::setup();
    let host = env::var("EVENTHUBS_HOST")?;
    let eventhub = env::var("EVENTHUB_NAME")?;
    let client = ProducerClient::new(
        host,
        eventhub,
        azure_identity::DefaultAzureCredential::new()?,
        Some(ProducerClientOptions {
            application_id: Some("test_close".to_string()),
            ..Default::default()
        }),
    );
    client.open().await?;
    client.close().await?;

    Ok(())
}

#[recorded::test(live)]
async fn test_get_properties() -> Result<(), Box<dyn Error>> {
    common::setup();
    let host = env::var("EVENTHUBS_HOST")?;
    let eventhub = env::var("EVENTHUB_NAME")?;

    let credential = DefaultAzureCredential::new()?;

    let client = ProducerClient::new(
        host,
        eventhub.clone(),
        credential,
        Some(ProducerClientOptions {
            application_id: Some("test_get_properties".to_string()),
            ..Default::default()
        }),
    );
    client.open().await?;
    let properties = client.get_eventhub_properties().await?;
    info!("Properties: {:?}", properties);
    assert_eq!(properties.name, eventhub);

    Ok(())
}

#[recorded::test(live)]
async fn test_get_partition_properties() -> Result<(), Box<dyn Error>> {
    common::setup();
    let host = env::var("EVENTHUBS_HOST")?;
    let eventhub = env::var("EVENTHUB_NAME")?;

    let credential = DefaultAzureCredential::new()?;

    let client = ProducerClient::new(
        host,
        eventhub.clone(),
        credential,
        Some(ProducerClientOptions {
            application_id: Some("test_get_partition_properties".to_string()),
            ..Default::default()
        }),
    );
    client.open().await?;
    let properties = client.get_eventhub_properties().await?;

    for partition_id in properties.partition_ids {
        let partition_properties = client
            .get_partition_properties(partition_id.clone())
            .await?;
        info!("Partition properties: {:?}", partition_properties);
        assert_eq!(partition_properties.id, partition_id);
    }

    Ok(())
}

#[recorded::test(live)]
async fn test_create_eventdata() -> Result<(), Box<dyn Error>> {
    common::setup();
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
async fn test_create_batch() -> Result<(), Box<dyn Error>> {
    common::setup();
    let host = env::var("EVENTHUBS_HOST")?;
    let eventhub = env::var("EVENTHUB_NAME")?;

    let credential = DefaultAzureCredential::new()?;

    let client = ProducerClient::new(
        host,
        eventhub.clone(),
        credential,
        Some(ProducerClientOptions {
            application_id: Some("test_create_batch".to_string()),
            ..Default::default()
        }),
    );
    client.open().await?;
    {
        let batch = client.create_batch(None).await?;
        assert_eq!(batch.len(), 0);
    }

    Ok(())
}

#[recorded::test(live)]
async fn test_create_and_send_batch() -> Result<(), Box<dyn Error>> {
    common::setup();
    let host = env::var("EVENTHUBS_HOST")?;
    let eventhub = env::var("EVENTHUB_NAME")?;

    let credential = DefaultAzureCredential::new()?;

    let client = ProducerClient::new(
        host,
        eventhub.clone(),
        credential,
        Some(ProducerClientOptions {
            application_id: Some("test_create_and_send_batch".to_string()),
            ..Default::default()
        }),
    );
    client.open().await?;
    {
        let mut batch = client.create_batch(None).await?;
        assert_eq!(batch.len(), 0);
        assert!(batch.try_add_event_data(vec![1, 2, 3, 4], None)?);

        let res = client.submit_batch(&batch).await;
        assert!(res.is_ok());
    }
    {
        let mut batch = client
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

        let res = client.submit_batch(&batch).await;
        assert!(res.is_ok());
    }

    Ok(())
}

#[recorded::test(live)]
async fn test_add_amqp_messages_to_batch() -> Result<(), Box<dyn std::error::Error>> {
    common::setup();
    let host = env::var("EVENTHUBS_HOST")?;
    let eventhub = env::var("EVENTHUB_NAME")?;

    let credential = DefaultAzureCredential::new()?;

    let client = ProducerClient::new(
        host,
        eventhub.clone(),
        credential,
        Some(ProducerClientOptions {
            application_id: Some("test_add_amqp_messages_to_batch".to_string()),
            ..Default::default()
        }),
    );
    client.open().await?;

    let batch = client.create_batch(None).await?;
    assert_eq!(batch.len(), 0);

    // Message with AMQP Value body
    assert!(batch.try_add_amqp_message(
        AmqpMessage::builder()
            .with_body(AmqpMessageBody::Value("This is data".into()))
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

    client.submit_batch(&batch).await?;

    Ok(())
}

#[recorded::test(live)]
async fn test_overload_batch() -> Result<(), Box<dyn Error>> {
    common::setup();

    let host = env::var("EVENTHUBS_HOST")?;
    let eventhub = env::var("EVENTHUB_NAME")?;

    let credential = DefaultAzureCredential::new()?;

    info!("Create producer client...");

    let client = ProducerClient::new(
        host,
        eventhub.clone(),
        credential,
        Some(ProducerClientOptions {
            application_id: Some("test_overload_batch".to_string()),
            ..Default::default()
        }),
    );

    info!("Open producer client...");
    client.open().await?;

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
                let result = client.submit_batch(&batch).await;
                if result.is_err() {
                    info!("Batch submit failed. {:?}", result);
                }
                assert!(result.is_ok());
            }
        }
        let result = client.submit_batch(&batch).await;
        if result.is_err() {
            info!("Batch submit failed. {:?}", result);
        }
        assert!(result.is_ok());
    }

    Ok(())
}

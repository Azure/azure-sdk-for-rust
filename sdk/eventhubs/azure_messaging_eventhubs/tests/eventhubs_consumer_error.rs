use azure_core_test::{recorded, TestContext};
use azure_messaging_eventhubs::{
    ConsumerClient, OpenReceiverOptions, ProducerClient, StartLocation, StartPosition,
};
use futures::StreamExt;

#[recorded::test(live)]
async fn consumer_error(ctx: TestContext) -> azure_core::Result<()> {
    let recording = ctx.recording();
    // Set up the Event Hub client
    let eventhub_namespace = recording.var("EVENTHUBS_HOST", None);
    let eventhub_name = recording.var("EVENTHUB_NAME", None);
    let credential = recording.credential();

    {
        let producer = ProducerClient::builder()
            .open(
                eventhub_namespace.as_str(),
                eventhub_name.as_str(),
                credential.clone(),
            )
            .await?;
        for i in 0..12 {
            use azure_messaging_eventhubs::SendEventOptions;

            let event = format!("Event {}", i);
            producer
                .send_event(
                    event,
                    Some(SendEventOptions {
                        partition_id: Some("0".into()),
                    }),
                )
                .await?;
        }
        producer.close().await?;
        println!("Producer closed");
    }

    let consumer = ConsumerClient::builder()
        .open(
            eventhub_namespace.as_str(),
            eventhub_name,
            credential.clone(),
        )
        .await?;

    println!("Opened consumer client");

    // Get the partition IDs
    let properties = consumer.get_eventhub_properties().await?;
    println!("EventHub Properties: {:?}", properties);

    // The default is to receive messages from the end of the partition, so specify a start position at the start of the partition.
    let receiver = consumer
        .open_receiver_on_partition(
            properties.partition_ids[0].clone(),
            Some(OpenReceiverOptions {
                start_position: Some(StartPosition {
                    location: StartLocation::Earliest,
                    ..Default::default()
                }),
                //                receive_timeout: Some(azure_core::time::Duration::seconds(1)),
                ..Default::default()
            }),
        )
        .await?;

    println!("Created receiver");

    // Create a stream of events from the receiver
    {
        let mut receive_stream = receiver.stream_events();

        println!("Created receive stream");

        // Read 10 events
        let mut count = 0;
        while let Some(event) = receive_stream.next().await {
            count += 1;
            if count > 10 {
                break;
            }

            let event = event?;
            println!("Partition ID: {:?}", event.partition_key());
            println!("Event offset: {:?}", event.offset());
        }
        println!("Read {} events", count);
    }

    receiver.close().await?;
    println!("Receiver closed");

    // Error
    match consumer.close().await {
        Ok(_) => {
            println!("Consumer closed successfully");
        }
        Err(e) => {
            return Err(e.into());
        }
    }

    Ok(())
}

use azure_core_test::{recorded, TestContext};
use azure_messaging_eventhubs::{
    ConsumerClient, OpenReceiverOptions, ProducerClient, Result, StartLocation, StartPosition,
};
use futures::StreamExt;
use tracing::trace;

#[recorded::test(live)]
async fn consumer_error(ctx: TestContext) -> Result<()> {
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
        trace!("Producer closed");
    }

    let consumer = ConsumerClient::builder()
        .open(
            eventhub_namespace.as_str(),
            eventhub_name,
            credential.clone(),
        )
        .await?;

    trace!("Opened consumer client");

    // Get the partition IDs
    let properties = consumer.get_eventhub_properties().await?;
    trace!("EventHub Properties: {:?}", properties);

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

    trace!("Created receiver");

    // Create a stream of events from the receiver
    {
        let mut receive_stream = receiver.stream_events();

        trace!("Created receive stream");

        // Read 10 events
        let mut count = 0;
        while let Some(event) = receive_stream.next().await {
            count += 1;
            if count > 10 {
                break;
            }

            let event = event?;
            trace!("Partition ID: {:?}", event.partition_key());
            trace!("Event offset: {:?}", event.offset());
        }
        trace!("Read {} events", count);
    }

    receiver.close().await?;
    trace!("Receiver closed");

    // Error
    consumer.close().await?;
    trace!("Consumer closed successfully");

    Ok(())
}

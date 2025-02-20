use core::f32;

/// This sample demonstrates how to send events to an Event Hub partition using the `ProducerClient`.
///
use azure_identity::DefaultAzureCredential;
use azure_messaging_eventhubs::{models::EventData, EventDataBatchOptions, ProducerClient};
use uuid::Uuid;

fn pretty_print_system_time(t: std::time::SystemTime) -> Result<(), Box<dyn std::error::Error>> {
    let utc = time::OffsetDateTime::UNIX_EPOCH
        + time::Duration::try_from(t.duration_since(std::time::UNIX_EPOCH).unwrap()).unwrap();
    let local = utc.to_offset(time::UtcOffset::local_offset_at(utc).unwrap());
    local.format_into(
        &mut std::io::stdout().lock(),
        time::macros::format_description!(
            "[day]-[month repr:short]-[year] [hour]:[minute]:[second]\n"
        ),
    )?;
    Ok(())
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    tracing_subscriber::fmt::init();
    // Set up the Event Hub client
    let eventhub_namespace = std::env::var("EVENTHUBS_HOST")?;
    let eventhub_name = std::env::var("EVENTHUB_NAME")?;
    let credential = DefaultAzureCredential::new()?;

    let client = ProducerClient::builder()
        .open(
            eventhub_namespace.as_str(),
            eventhub_name.as_str(),
            credential.clone(),
        )
        .await?;

    println!("Created producer client.");

    // Send an event to an eventhub instance directly. The message will be sent to a random partition.
    // Note that this uses an implicit builder to create the EventData being sent to the service.
    let batch = client
        .create_batch(Some(EventDataBatchOptions {
            partition_id: Some("0".to_string()),
            ..Default::default()
        }))
        .await?;
    batch.try_add_event_data("Hello, Event Hub!", None)?;

    // Send an array of bytes to partition 0 of the Event Hubs instance.
    // Note that this uses an implicit builder to create the EventData being sent to the service.
    batch.try_add_event_data(vec![2, 4, 8, 16], None)?;

    // Send an event built using the `EventData` builder which allows for more control over the event.
    // This message will be sent to partition 0.
    batch.try_add_event_data(
        EventData::builder()
            .with_content_type("text/plain")
            .with_correlation_id(Uuid::new_v4())
            .with_body("This is some text")
            .add_property("Event Property", "Property Value")
            .add_property("Pi", f32::consts::PI)
            .add_property("Binary", vec![0x08, 0x09, 0x0A])
            .build(),
        None,
    )?;
    client.send_batch(&batch, None).await?;

    let properties = client.get_eventhub_properties().await.unwrap();
    println!("Eventhub Properties for: {eventhub_name} {properties:?}");

    for partition in properties.partition_ids.iter() {
        let partition_properties = client
            .get_partition_properties(partition.as_str())
            .await
            .unwrap();
        println!(
            "Partition Properties for: {partition} {:?}",
            partition_properties
        );

        if let Some(last_enqueued_time_utc) = partition_properties.last_enqueued_time_utc {
            println!("Last enqueued offset: {last_enqueued_time_utc:?}");
            // Print the last enqueued time for the partition.
            print!("Last enqueued time:");
            pretty_print_system_time(last_enqueued_time_utc)?;
        }
    }

    client.close().await?;
    Ok(())
}

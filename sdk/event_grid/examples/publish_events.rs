use azure_event_grid::{Event, EventGridClient};
use serde::Serialize;
use std::env;

#[derive(Serialize)]
struct Data {
    number: i32,
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let topic_host_name =
        env::var("TOPIC_HOST_NAME").expect("Missing TOPIC_HOST_NAME environment variable.");
    let topic_key = env::var("TOPIC_KEY").expect("Missing TOPIC_KEY environment variable.");

    let client = EventGridClient::new(topic_host_name, topic_key);
    let event = Event::<Data>::new(
        None,
        "ACME.Data.DataPointCreated",
        "/acme/data",
        Data { number: 42 },
        None,
    );

    client.publish_events(&[event]).await?;
    Ok(())
}

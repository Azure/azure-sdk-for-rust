use azure_sdk_service_bus::prelude::*;
use std::error::Error;

async fn send(
    s: String,
    service_bus_namespace: String,
    event_hub_name: String,
    policy_name: String,
    policy_key: String,
) -> Result<(), Box<dyn Error>> {
    let mut client = Client::new(
        service_bus_namespace.to_owned(),
        event_hub_name.to_owned(),
        policy_name.to_owned(),
        policy_key.to_owned(),
    )?;

    println!("before {:?} message send!", s);
    match client.send_event(&s, time::Duration::days(1)).await {
        Ok(_) => println!("{:?} message sent!", s),

        Err(error) => println!("{:?} failed to send message", error),
    }

    Ok(())
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn Error>> {
    // First we retrieve the account name and master key from environment variables.
    // We expect master keys (ie, not resource constrained)
    let service_bus_namespace = std::env::var("AZURE_SERVICE_BUS_NAMESPACE")
        .expect("Set env variable AZURE_SERVICE_BUS_NAMESPACE first!");
    let event_hub_name = std::env::var("AZURE_EVENT_HUB_NAME")
        .expect("Set env variable AZURE_EVENT_HUB_NAME first!");
    let policy_name =
        std::env::var("AZURE_POLICY_NAME").expect("Set env variable AZURE_POLICY_NAME first!");
    let policy_key =
        std::env::var("AZURE_POLICY_KEY").expect("Set env variable AZURE_POLICY_KEY first!");

    let messages = vec![
        "These", "are", "useless", "messages", "provided", "for", "free", "with", "love",
    ];
    println!(
        "Sending the following messages: {:?}. \
         Please note they will be sent out of order!",
        messages
    );

    let mut v = Vec::new();
    for s in messages.into_iter() {
        v.push(send(
            s.to_owned(),
            service_bus_namespace.to_owned(),
            event_hub_name.to_owned(),
            policy_name.to_owned(),
            policy_key.to_owned(),
        ))
    }

    futures::future::join_all(v).await;

    Ok(())
}

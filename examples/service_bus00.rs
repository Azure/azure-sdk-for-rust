extern crate azure_sdk_for_rust;
extern crate futures;
extern crate hyper;
extern crate tokio_core;

use azure_sdk_for_rust::service_bus::event_hub::Client;
use futures::Future;
use std::error::Error;
use tokio_core::reactor::Core;

extern crate time;

fn main() {
    code().unwrap();
}

// We run a separate method to use the elegant quotation mark operator.
// A series of unwrap(), unwrap() would have achieved the same result.
fn code() -> Result<(), Box<Error>> {
    // First we retrieve the account name and master key from environment variables.
    // We expect master keys (ie, not resource constrained)
    let service_bus_namespace = std::env::var("AZURE_SERVICE_BUS_NAMESPACE").expect("Set env variable AZURE_SERVICE_BUS_NAMESPACE first!");
    let event_hub_name = std::env::var("AZURE_EVENT_HUB_NAME").expect("Set env variable AZURE_EVENT_HUB_NAME first!");
    let policy_name = std::env::var("AZURE_POLICY_NAME").expect("Set env variable AZURE_POLICY_NAME first!");
    let policy_key = std::env::var("AZURE_POLICY_KEY").expect("Set env variable AZURE_POLICY_KEY first!");

    let mut core = Core::new()?;

    let mut client = Client::new(service_bus_namespace, event_hub_name, policy_name, policy_key).unwrap();

    let messages = vec!["These", "are", "useless", "messages", "provided", "for", "free", "with", "love"];
    println!(
        "Sending the following messages: {:?}. \
         Please note they will be sent out of order!",
        messages
    );

    let mut v = Vec::new();
    for s in messages {
        v.push(client.send_event(s, time::Duration::days(1)).map(move |_| {
            println!("{:?} event sent!", s);
        }))
    }

    let future = futures::future::join_all(v);

    core.run(future)?;

    Ok(())
}

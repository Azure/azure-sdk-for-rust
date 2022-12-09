/*!
Azure Service Bus crate for the unofficial Microsoft Azure SDK for Rust.
This crate is part of a collection of crates: for more information please refer to [https://github.com/azure/azure-sdk-for-rust](https://github.com/azure/azure-sdk-for-rust).

# Example
```no_run,rust
use azure_messaging_servicebus::prelude::*;

#[tokio::main]
async fn main() -> azure_core::Result<()> {
    let service_bus_namespace = std::env::var("AZURE_SERVICE_BUS_NAMESPACE").expect("missing AZURE_SERVICE_BUS_NAMESPACE");
    let queue_name = std::env::var("AZURE_QUEUE_NAME").expect("missing AZURE_QUEUE_NAME");
    let policy_name = std::env::var("AZURE_POLICY_NAME").expect("missing AZURE_POLICY_NAME");
    let policy_key = std::env::var("AZURE_POLICY_KEY").expect("missing AZURE_POLICY_KEY");

    let http_client = azure_core::new_http_client();
    let mut client = QueueClient::new(
        http_client,
        service_bus_namespace,
        queue_name,
        policy_name,
        policy_key,
    )?;

    client.send_message("hello world").await?;

    let received_message = client.receive_and_delete_message().await?;
    println!("Received Message: {}", received_message);

    Ok(())
}
```
*/
#![recursion_limit = "128"]
#![deny(
    missing_docs,
    missing_debug_implementations,
)]

pub mod amqp;
pub mod client;
pub mod primitives;
pub mod receiver;
pub mod sender;

// pub mod prelude;
// pub mod service_bus;
// pub mod utils;

// TODO: reserved for future use
// pub mod administration;
// pub mod processor;

pub use primitives::service_bus_message::ServiceBusMessage;
pub use receiver::{
    service_bus_receive_mode::ServiceBusReceiveMode, service_bus_receiver::ServiceBusReceiver,
    service_bus_receiver::ServiceBusReceiverOptions,
};
pub use sender::{
    create_message_batch_options::CreateMessageBatchOptions,
    service_bus_message_batch::ServiceBusMessageBatch, service_bus_sender::ServiceBusSender,
    service_bus_sender_options::ServiceBusSenderOptions,
};

pub(crate) mod core;
pub(crate) mod constants;
pub(crate) mod diagnostics;
pub(crate) mod authorization;
pub(crate) mod entity_name_formatter;

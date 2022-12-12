/*!
Azure Service Bus crate for the unofficial Microsoft Azure SDK for Rust.
This crate is part of a collection of crates: for more information please refer to [https://github.com/azure/azure-sdk-for-rust](https://github.com/azure/azure-sdk-for-rust).

# Example

## Send messages to the queue

```no_run,rust
use azure_messaging_servicebus::prelude::*;

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    // Replace "<NAMESPACE-CONNECTION-STRING>" with your connection string, which can be found in the Azure portal
    // and should look like "Endpoint=sb://<NAMESPACE>.servicebus.windows.net/;SharedAccessKeyName=<KEY_NAME>;SharedAccessKey=<KEY_VALUE>"
    let mut client = ServiceBusClient::new("<NAMESPACE-CONNECTION-STRING>", ServiceBusClientOptions::default())
        .await?;

    // Replace "<QUEUE-NAME>" with the name of your queue
    let mut sender = client.create_sender("<QUEUE-NAME>", ServiceBusSenderOptions::default()).await?;

    // Create a batch
    let mut message_batch = sender.create_message_batch(CreateMessageBatchOptions::default()).await?;

    for i in 0..3 {
        // Try to add a message to the batch
        if let Err(e) = message_batch.try_add_message(ServiceBusMessage::new(format!("Message {}", i))) {
            // If the batch is full, an error will be returned
            println!("Failed to add message {} to batch: {:?}", i, e);
            break;
        }
    }

    // Send the batch of messages to the queue
    match sender.send_message_batch(message_batch).await {
        Ok(()) => println!("Batch sent successfully"),
        Err(e) => println!("Failed to send batch: {:?}", e),
    }

    sender.dispose().await?;
    client.dispose().await?;

    Ok(())
}
```

## Receive messages from the queue

```no_run,rust
use azure_messaging_servicebus::prelude::*;

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    // Replace "<NAMESPACE-CONNECTION-STRING>" with your connection string, which can be found in the Azure portal
    // and should look like "Endpoint=sb://<NAMESPACE>.servicebus.windows.net/;SharedAccessKeyName=<KEY_NAME>;SharedAccessKey=<KEY_VALUE>"
    let mut client = ServiceBusClient::new("<NAMESPACE-CONNECTION-STRING>", ServiceBusClientOptions::default())
        .await?;

    // Replace "<QUEUE-NAME>" with the name of your queue
    let mut receiver = client.create_receiver("<QUEUE-NAME>", ServiceBusReceiverOptions::default()).await?;

    // Receive messages from the queue using a default max wait time
    let messages = receiver.receive_messages_with_max_wait_time(3, None).await?;

    for message in &messages {
        let body = message.body()?;
        println!("Received message: {:?}", std::str::from_utf8(body)?);

        // Complete the message so that it is removed from the queue
        receiver.complete_message(message).await?;
    }

    receiver.dispose().await?;
    client.dispose().await?;

    Ok(())
}
```
*/
#![recursion_limit = "128"]
#![deny(
    missing_docs,
    missing_debug_implementations,
)]

pub(crate) mod constants;
pub(crate) mod diagnostics;
pub(crate) mod authorization;
pub(crate) mod entity_name_formatter;

pub mod amqp;
pub mod client;
pub mod primitives;
pub mod receiver;
pub mod sender;
pub mod core; // TODO: change to pub(crate)?

// pub mod prelude;
// pub mod service_bus;
// pub mod utils;

// TODO: reserved for future use
// pub mod administration;
// pub mod processor;

// =============================================================================
// Re-exports,
// =============================================================================
pub mod prelude {
    //! Re-exports

    pub use crate::primitives::{
        service_bus_message::ServiceBusMessage, service_bus_received_message::ServiceBusReceivedMessage,
        service_bus_peeked_message::ServiceBusPeekedMessage, service_bus_retry_options::ServiceBusRetryOptions,
        service_bus_connection_string_properties::ServiceBusConnectionStringProperties,
        service_bus_retry_policy::ServiceBusRetryPolicy, service_bus_message_state::ServiceBusMessageState,
        service_bus_retry_mode::ServiceBusRetryMode, service_bus_transport_type::ServiceBusTransportType,
        sub_queue::SubQueue,
    };
    pub use crate::receiver::{
        service_bus_receive_mode::ServiceBusReceiveMode, service_bus_receiver::ServiceBusReceiver,
        service_bus_receiver::ServiceBusReceiverOptions, service_bus_session_receiver::ServiceBusSessionReceiver,
        service_bus_session_receiver::ServiceBusSessionReceiverOptions,
    };
    pub use crate::sender::{
        service_bus_message_batch::CreateMessageBatchOptions,
        service_bus_message_batch::ServiceBusMessageBatch, service_bus_sender::ServiceBusSender,
        service_bus_sender::ServiceBusSenderOptions,
    };
    pub use crate::client::{
        ServiceBusClient, ServiceBusClientOptions
    };
}

// TODO: Re-export again to allow user to selectively import components
pub use prelude::*;

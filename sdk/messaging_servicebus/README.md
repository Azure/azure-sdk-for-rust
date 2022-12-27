# azure_messaging_servicebus

Azure Service Bus crate for the unofficial Microsoft Azure SDK for Rust.
This crate is part of a collection of crates: for more information please refer to [https://github.com/azure/azure-sdk-for-rust](https://github.com/azure/azure-sdk-for-rust).

## Examples

Below are two examples of sending and receiving messages from a queue. More examples can be found
in the [examples](https://github.com/Azure/azure-sdk-for-rust/tree/main/sdk/messaging_servicebus/examples)

### Send messages to queue

```no_run,rust
use azure_messaging_servicebus::prelude::*;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    // Replace "<NAMESPACE-CONNECTION-STRING>" with your connection string,
    // which can be found in the Azure portal and should look like
    // "Endpoint=sb://<NAMESPACE>.servicebus.windows.net/;SharedAccessKeyName=<KEY_NAME>;SharedAccessKey=<KEY_VALUE>"
    let mut client = ServiceBusClient::new(
        "<NAMESPACE-CONNECTION-STRING>",
        ServiceBusClientOptions::default()
    )
    .await?;

    // Replace "<QUEUE-NAME>" with the name of your queue
    let mut sender = client.create_sender(
        "<QUEUE-NAME>",
        ServiceBusSenderOptions::default()
    )
    .await?;

    // Create a batch
    let mut message_batch = sender.create_message_batch(Default::default())?;

    for i in 0..3 {
        // Create a message
        let message = ServiceBusMessage::new(format!("Message {}", i));
        // Try to add the message to the batch
        if let Err(e) = message_batch.try_add_message(message) {
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

### Receive messages from queue

```no_run,rust
use azure_messaging_servicebus::prelude::*;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    // Replace "<NAMESPACE-CONNECTION-STRING>" with your connection string,
    // which can be found in the Azure portal and should look like
    // "Endpoint=sb://<NAMESPACE>.servicebus.windows.net/;SharedAccessKeyName=<KEY_NAME>;SharedAccessKey=<KEY_VALUE>"
    let mut client = ServiceBusClient::new(
        "<NAMESPACE-CONNECTION-STRING>",
        ServiceBusClientOptions::default()
    )
    .await?;

    // Replace "<QUEUE-NAME>" with the name of your queue
    let mut receiver = client.create_receiver_for_queue(
        "<QUEUE-NAME>",
        ServiceBusReceiverOptions::default()
    )
    .await?;

    // Receive messages from the queue
    // This will wait indefinitely until at least one message is received
    let messages = receiver.receive_messages(3).await?;

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

## Supported Service Bus Features

Below shows supported Service Bus features

| Feature | Supported |
| ------- | --------- |
| Send messages to queue/topic | Yes |
| Receive messages from queue/subscription | Yes |
| Session receivers for queue/subscription | Yes |
| Prefetch | Yes |
| Schedule messages | Yes |
| Cancel scheduled messages | Yes |
| Peek messages | Yes |
| Complete messages | Yes |
| Abandon messages | Yes |
| Defer messages | Yes |
| Receive deferred messages | Yes |
| Dead-letter messages | Yes |
| Receive dead-lettered messages | Yes |
| Batching | Yes |
| Manage rule filters for subscriptions | Yes |
| Lock renewal | Yes |
| Transaction | Not yet |
| Processor | Not yet |
| Session processor | Not yet |

License: MIT

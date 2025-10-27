<!-- cspell:words pwsh yourgroup westus servicebus checkpointing  -->

# Azure Event Hubs client library for Rust

[Azure Event Hubs](https://azure.microsoft.com/services/event-hubs/) is a big data streaming platform and event ingestion service from Microsoft. For more information about Event Hubs see [this link](https://learn.microsoft.com/azure/event-hubs/event-hubs-about).

The Azure Event Hubs client library allows you to send single events or batches of events to an event hub and consume events from an event hub.

[Source code] | [Package (crates.io)] | [API reference documentation] | [Product documentation]

## Getting started

### Install the package

Install the Azure Event Hubs client library for Rust with [Cargo]:

```sh
cargo add azure_messaging_eventhubs
```

### Prerequisites

* A Rust Compiler. See [the rust compiler installation instructions](https://www.rust-lang.org/tools/install).
* An [Azure subscription]
* The [Azure CLI]
* An [Event Hub namespace](https://learn.microsoft.com/azure/event-hubs/).
* An Event Hub instance. You can create an Event Hub instance in your Event Hubs Namespace using the [Azure Portal](https://learn.microsoft.com/azure/event-hubs/event-hubs-create), or the [Azure CLI](https://learn.microsoft.com/azure/event-hubs/event-hubs-quickstart-cli).

If you use the Azure CLI, replace `<your-resource-group-name>`, `<your-eventhubs-namespace-name>`, and `<your-eventhub-name>` with your own, unique names:

Create an Event Hubs Namespace:

```azurecli
az eventhubs namespace create --resource-group <your-resource-group-name> --name <your-eventhubs-namespace-name> --sku Standard
```

Create an Event Hub Instance:

```azurecli
az eventhubs eventhub create --resource-group <your-resource-group-name> --namespace-name <your-eventhubs-namespace-name> --name <your-eventhub-name>
```

### Install dependencies

Add the following crates to your project:

```sh
cargo add azure_identity tokio
```

### Authenticate the client

In order to interact with the Azure Event Hubs service, you'll need to create an instance of the `ProducerClient` or the `ConsumerClient`. You need an **event hub namespace host URL** (which you may see as `serviceBusEndpoint` in the Azure CLI response when creating the Even Hubs Namespace), an **Event Hub name** (which you may see as `name` in the Azure CLI response when crating the Event Hub instance), and credentials to instantiate a client object.

The example shown below uses a `DeveloperToolsCredential`, which is appropriate for most local development environments. Additionally, we recommend using a managed identity for authentication in production environments. You can find more information on different ways of authenticating and their corresponding credential types in the [Azure Identity] documentation.

The `DeveloperToolsCredential` will automatically pick up on an Azure CLI authentication. Ensure you are logged in with the Azure CLI:

```azurecli
az login
```

Instantiate a `DeveloperToolsCredential` to pass to the client. The same instance of a token credential can be used with multiple clients if they will be authenticating with the same identity.

### Create an Event Hubs message producer and send an event

```rust no_run
use azure_identity::DeveloperToolsCredential;
use azure_messaging_eventhubs::ProducerClient;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let host = "<EVENTHUBS_HOST>";
    let eventhub = "<EVENTHUB_NAME>";

    // Create new credential
    let credential = DeveloperToolsCredential::new(None)?;

    // Create and open a new ProducerClient
    let producer = ProducerClient::builder()
        .open(host, eventhub, credential.clone())
        .await?;

    producer.send_event(vec![1, 2, 3, 4], None).await?;

    Ok(())
}
```

## Key concepts

An Event Hub [**namespace**](https://learn.microsoft.com/azure/event-hubs/event-hubs-features#namespace) can have multiple Event Hub instances.
Each Event Hub instance, in turn, contains [**partitions**](https://learn.microsoft.com/azure/event-hubs/event-hubs-features#partitions) which store events.

<!-- NOTE: Fix dead links -->

Events are published to an Event Hub instance using an [event publisher](https://learn.microsoft.com/azure/event-hubs/event-hubs-features#event-publishers). In this package, the event publisher is the [`ProducerClient`][producer_client]

Events can be consumed from an Event Hub instance using an [event consumer](https://learn.microsoft.com/azure/event-hubs/event-hubs-features#event-consumers).

Consuming events is done using an `EventReceiver`, which can be opened from the [`ConsumerClient`][consumer_client]. This is useful if you already known which partitions you want to receive from.

<!--
-   A distributed event consumer, which uses Azure Blobs for checkpointing and coordination. This is implemented in the [Processor](https://azure.github.io/azure-sdk-for-cpp/storage.html).
    The Processor is useful when you want to have the partition assignment be dynamically chosen, and balanced with other Processor instances.
    -->

More information about Event Hubs features and terminology can be found at the [Event Hubs features documentation]](<https://learn.microsoft.com/azure/event-hubs/event-hubs-features>)

## Examples

Additional examples for various scenarios can be found on in the examples directory in our GitHub repo for
[Event Hubs](https://github.com/Azure/azure-sdk-for-rust/tree/main/sdk/eventhubs/azure_messaging_eventhubs/examples).

<!-- no toc -->

* [Open an Event Hubs message producer on an Event Hub instance](#open-an-event-hubs-message-producer-on-an-event-hub-instance)
* [Send events](#send-events)
  * [Send events directly to the Event Hub](#send-events-directly-to-the-event-hub)
  * [Send events using a batch operation](#send-events-using-a-batch-operation)
* [Open an Event Hubs message consumer on an Event Hubs instance](#open-an-event-hubs-message-consumer-on-an-event-hub-instance)
* [Receive events](#receive-events)

### Open an Event Hubs message producer on an Event Hub instance

```rust no_run
use azure_identity::DeveloperToolsCredential;
use azure_messaging_eventhubs::ProducerClient;

async fn open_producer_client() -> Result<ProducerClient, Box<dyn std::error::Error>> {
    let host = "<EVENTHUBS_HOST>";
    let eventhub = "<EVENTHUB_NAME>";

    let credential = DeveloperToolsCredential::new(None)?;

    let producer = ProducerClient::builder()
        .open(host, eventhub, credential.clone())
        .await?;

    Ok(producer)
}
```

### Send events

There are two mechanisms used to send events to an Event Hub instance. The first directly
sends individual messages to the Event Hub, the second uses a "batch" operation to
send multiple messages in a single network request to the service.

#### Send events directly to the Event Hub

```rust no_run
use azure_messaging_eventhubs::ProducerClient;

async fn send_events(producer: &ProducerClient) -> Result<(), Box<dyn std::error::Error>> {
    producer.send_event(vec![1, 2, 3, 4], None).await?;

    Ok(())
}
```

#### Send events using a batch operation

```rust no_run
use azure_messaging_eventhubs::ProducerClient;

async fn send_events(producer: &ProducerClient) -> Result<(), Box<dyn std::error::Error>> {
    let batch = producer.create_batch(None).await?;
    assert_eq!(batch.len(), 0);
    assert!(batch.try_add_event_data(vec![1, 2, 3, 4], None)?);

    let res = producer.send_batch(batch, None).await;
    assert!(res.is_ok());

    Ok(())
}
```

### Open an Event Hubs message consumer on an Event Hub instance

```rust no_run
use azure_identity::DeveloperToolsCredential;
use azure_messaging_eventhubs::ConsumerClient;

async fn open_consumer_client() -> Result<ConsumerClient, Box<dyn std::error::Error>> {
    let host = "<EVENTHUBS_HOST>".to_string();
    let eventhub = "<EVENTHUB_NAME>".to_string();

    let credential = DeveloperToolsCredential::new(None)?;

    let consumer = azure_messaging_eventhubs::ConsumerClient::builder()
        .open(&host, eventhub, credential.clone())
        .await?;

    Ok(consumer)
}
```

### Receive events

The following example shows how to receive events from partition 0 on an Event Hubs instance.

It assumes that the caller has provided a consumer client which will be used to receive
events.

Each message receiver can only receive messages from a single Event Hubs partition

```rust no_run
use futures::stream::StreamExt;
use azure_messaging_eventhubs::{
    ConsumerClient, OpenReceiverOptions, StartLocation, StartPosition,
};

// By default, an event receiver only receives new events from the event hub. To receive events from earlier, specify
// a `start_position` which represents the position from which to start receiving events.
// In this example, events are received from the start of the partition.
async fn receive_events(client: &ConsumerClient) -> Result<(), Box<dyn std::error::Error>> {
    let message_receiver = client
        .open_receiver_on_partition(
            "0".to_string(),
            Some(OpenReceiverOptions {
                start_position: Some(StartPosition {
                    location: StartLocation::Earliest,
                    ..Default::default()
                }),
                ..Default::default()
            }),
        )
        .await?;

    let mut event_stream = message_receiver.stream_events();

    while let Some(event_result) = event_stream.next().await {
        match event_result {
            Ok(event) => {
                // Process the received event
                println!("Received event: {:?}", event);
            }
            Err(err) => {
                // Handle the error
                eprintln!("Error receiving event: {:?}", err);
            }
        }
    }

    Ok(())
}
```

## Troubleshooting

### General

When you interact with the Azure Event Hubs client library using the Rust SDK, errors returned by the service are returned as `azure_core::Error` values using `ErrorKind::Other` which are `azure_messaging_eventhubs::Error` values.

### Logging

The Event Hubs SDK client uses the [tracing](https://docs.rs/tracing/latest/tracing/) package to
enable diagnostics.

## Contributing

See the [CONTRIBUTING.md] for details on building, testing, and contributing to these libraries.

This project welcomes contributions and suggestions. Most contributions require you to agree to a Contributor License Agreement (CLA) declaring that you have the right to, and actually do, grant us the rights to use your contribution. For details, visit <https://opensource.microsoft.com/cla/>.

When you submit a pull request, a CLA-bot will automatically determine whether you need to provide a CLA and decorate the PR appropriately (e.g., label, comment). Simply follow the instructions provided by the bot. You will only need to do this once across all repos using our CLA.

This project has adopted the [Microsoft Open Source Code of Conduct]. For more information see the [Code of Conduct FAQ] or contact <opencode@microsoft.com> with any additional questions or comments.

### Reporting security issues and security bugs

Security issues and bugs should be reported privately, via email, to the Microsoft Security Response Center (MSRC) <secure@microsoft.com>. You should receive a response within 24 hours. If for some reason you do not, please follow up via email to ensure we received your original message. Further information, including the MSRC PGP key, can be found in the [Security TechCenter](https://www.microsoft.com/msrc/faqs-report-an-issue).

### License

Azure SDK for Rust is licensed under the [MIT](https://github.com/Azure/azure-sdk-for-cpp/blob/main/LICENSE.txt) license.

<!-- LINKS -->
[producer_client]: https://docs.rs/azure_messaging_eventhubs/latest/azure_messaging_eventhubs/struct.ProducerClient.html
[consumer_client]: https://docs.rs/azure_messaging_eventhubs/latest/azure_messaging_eventhubs/struct.ConsumerClient.html
[API reference documentation]: https://docs.rs/azure_messaging_eventhubs/latest/azure_messaging_eventhubs
[Azure CLI]: https://learn.microsoft.com/cli/azure
[Azure subscription]: https://azure.microsoft.com/free/
[Azure Identity]: https://aka.ms/azsdk/rust/identity/docs
[Microsoft Open Source Code of Conduct]: https://opensource.microsoft.com/codeofconduct/
[Product documentation]: https://learn.microsoft.com/azure/event-hubs/
[Cargo]: https://crates.io/
[Package (crates.io)]: https://crates.io/crates/azure_messaging_eventhubs
[Source code]: https://github.com/Azure/azure-sdk-for-rust/tree/main/sdk/eventhubs/azure_messaging_eventhubs/src
[CONTRIBUTING.md]: https://github.com/Azure/azure-sdk-for-rust/blob/main/CONTRIBUTING.md
[Code of Conduct FAQ]: https://opensource.microsoft.com/codeofconduct/faq/

<!-- cspell:words pwsh yourgroup westus servicebus checkpointing  -->

# Azure Event Hubs client library for Rust

[Azure Event Hubs](https://azure.microsoft.com/services/event-hubs/) is a big data streaming platform and event ingestion service from Microsoft. For more information about Event Hubs see: [link](https://learn.microsoft.com/azure/event-hubs/event-hubs-about).

The Azure Event Hubs client library allows you to send single events or batches of events to an event hub and consume events from an event hub.

[Source code] | [Package (crates.io)] | [API reference documentation] | [Product documentation]

## Getting started

### Install the package

Install the Azure Event Hubs client library for Rust with [Cargo]:

```sh
cargo add azure_messaging_eventhubs
```

### Prerequisites

* A Rust Compiler. See [here](https://www.rust-lang.org/tools/install) for installation instructions.
* An [Azure subscription]
* The [Azure CLI]
* An [Event Hub namespace](https://learn.microsoft.com/azure/event-hubs/).
* An Event Hub instance. You can create an Event Hub instance in your Event Hubs Namespace using the [Azure Portal](https://learn.microsoft.com/azure/event-hubs/event-hubs-create), or the [Azure CLI](https://learn.microsoft.com/azure/event-hubs/event-hubs-quickstart-cli).

If you use the Azure CLI, replace `<your-resource-group-name>`, `<your-eventhubs-namespace-name>`, and `<your-eventhub-name>` with your own, unique names:

Create an Event Hubs namespace:

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

The example shown below uses a `DefaultAzureCredential`, which is appropriate for most local development environments. Additionally, we recommend using a managed identity for authentication in production environments. You can find more information on different ways of authenticating and their corresponding credential types in the [Azure Identity] documentation.

The `DefaultAzureCredential` will automatically pick up on an Azure CLI authentication. Ensure you are logged in with the Azure CLI:

```azurecli
az login
```

Instantiate a `DefaultAzureCredential` to pass to the client. The same instance of a token credential can be used with multiple clients if they will be authenticating with the same identity.

### Create an Event Hubs message producer on an Event Hubs instance

```rust no_run
use azure_identity::DefaultAzureCredential;
use azure_messaging_eventhubs::ProducerClient;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let host = "<EVENTHUBS_NAMESPACE_HOST>";
    let eventhub = "<EVENTHUB_NAME>";

    // Create new credential
    let credential = DefaultAzureCredential::new()?;

    // Create and open new Producer Client
    let producer = ProducerClient::new(
        host.to_string(),
        eventhub.to_string(),
        credential.clone(),
        None,
    );
    producer.open().await?;

    Ok(())
}
```

## Key concepts

### Event Hub namespace

An Event Hub [**namespace**](https://learn.microsoft.com/azure/event-hubs/event-hubs-features#namespace) can have multiple Event Hub instances. Each Event Hub instance, in turn, contains [**partitions**](https://learn.microsoft.com/azure/event-hubs/event-hubs-features#partitions) which store events.

### ProducerClient

The `ProducerClient` provides asynchronous operations for sending events to an Event Hub.

### ConsumerClient

The `ConsumerClient` provides asynchronous operations for receiving events from an Event Hub.

## Examples

The following section provides several code snippets using the `ProducerClient` and `ConsumerClient`, covering some of the most common Azure Event Hubs service related tasks:

* [Send events directly to the Event Hub](#send-events-directly-to-the-event-hub)
* [Send events using a batch operation](#send-events-using-a-batch-operation)
* [Receive events](#receive-events)

### Send events directly to the Event Hub

```rust no_run
use azure_identity::DefaultAzureCredential;
use azure_messaging_eventhubs::ProducerClient;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    // Define the Event Hubs namespace host and Event Hub name
    let host = "<EVENTHUBS_NAMESPACE_HOST>";
    let eventhub = "<EVENTHUB_NAME>";

    // Create a DefaultAzureCredential instance
    let credential = DefaultAzureCredential::new()?;

    // Create a ProducerClient instance
    let producer = ProducerClient::new(
        host.to_string(),
        eventhub.to_string(),
        credential.clone(),
        None,
    );
    producer.open().await?;

    // Send an event to the Event Hub
    let _ = producer.send_event(vec![1, 2, 3, 4], None).await;

    Ok(())
}
```

### Send events using a batch operation

```rust no_run
use azure_identity::DefaultAzureCredential;
use azure_messaging_eventhubs::ProducerClient;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    // Define the Event Hubs namespace host and Event Hub name
    let host = "<EVENTHUBS_NAMESPACE_HOST>";
    let eventhub = "<EVENTHUB_NAME>";

    // Create a DefaultAzureCredential instance
    let credential = DefaultAzureCredential::new()?;

    // Create a ProducerClient instance
    let producer = ProducerClient::new(
        host.to_string(),
        eventhub.to_string(),
        credential.clone(),
        None,
    );
    producer.open().await?;

    // Create a batch of events
    let mut batch = producer.create_batch(None).await?;
    assert_eq!(batch.len(), 0);
    assert!(batch.try_add_event_data(vec![1, 2, 3, 4], None)?);

    // Submit the batch of events to the Event Hub
    let res = producer.submit_batch(&batch, None).await;
    assert!(res.is_ok());

    Ok(())
}
```

### Receive events

The following example shows how to receive events from partition 0 on an Event Hubs instance.

```rust no_run
use async_std::stream::StreamExt;
use azure_identity::DefaultAzureCredential;
use azure_messaging_eventhubs::ConsumerClient;
use futures::pin_mut;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    // Define the Event Hubs namespace host and Event Hub name
    let host = "<EVENTHUBS_NAMESPACE_HOST>";
    let eventhub = "<EVENTHUB_NAME>";

    // Create a DefaultAzureCredential instance
    let credential = DefaultAzureCredential::new()?;

    // Create a ConsumerClient instance
    let client = ConsumerClient::new(
        host.to_string(),
        eventhub.to_string(),
        None,
        credential.clone(),
        None,
    );
    client.open().await?;

    // Open a receiver on partition 0
    let message_receiver = client
        .open_receiver_on_partition(
            "0".to_string(),
            Some(azure_messaging_eventhubs::OpenReceiverOptions {
                start_position: Some(azure_messaging_eventhubs::StartPosition {
                    location: azure_messaging_eventhubs::StartLocation::Earliest,
                    ..Default::default()
                }),
                ..Default::default()
            }),
        )
        .await?;

    // Create a stream of events
    let event_stream = message_receiver.stream_events();

    pin_mut!(event_stream); // Needed for iteration.

    // Iterate over the stream of events
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

When you interact with the Azure Event Hubs client library using the Rust SDK, errors returned by the service correspond to the same HTTP status codes returned for [REST API] requests.

## Contributing

See the [CONTRIBUTING.md] for details on building, testing, and contributing to these libraries.

This project welcomes contributions and suggestions. Most contributions require you to agree to a Contributor License Agreement (CLA) declaring that you have the right to, and actually do, grant us the rights to use your contribution. For details, visit <https://opensource.microsoft.com/cla/>.

When you submit a pull request, a CLA-bot will automatically determine whether you need to provide a CLA and decorate the PR appropriately (e.g., label, comment). Simply follow the instructions provided by the bot. You will only need to do this once across all repos using our CLA.

This project has adopted the [Microsoft Open Source Code of Conduct]. For more information see the [Code of Conduct FAQ] or contact <opencode@microsoft.com> with any additional questions or comments.

<!-- LINKS -->
[API reference documentation]: https://docs.rs/azure_messaging_eventhubs/latest/azure_messaging_eventhubs
[Azure CLI]: https://learn.microsoft.com/cli/azure
[Azure subscription]: https://azure.microsoft.com/free/
[Azure Identity]: https://github.com/Azure/azure-sdk-for-rust/tree/main/sdk/identity/azure_identity
[Microsoft Open Source Code of Conduct]: https://opensource.microsoft.com/codeofconduct/
[Product documentation]: https://azure.microsoft.com/services/event-hubs/
[REST API]: https://learn.microsoft.com/rest/api/eventhub/
[Cargo]: https://crates.io/
[Package (crates.io)]: https://crates.io/crates/azure_messaging_eventhubs
[Source code]: https://github.com/Azure/azure-sdk-for-rust/tree/main/sdk/eventhubs/azure_messaging_eventhubs/src
[CONTRIBUTING.md]: https://github.com/Azure/azure-sdk-for-rust/blob/main/CONTRIBUTING.md
[Code of Conduct FAQ]: https://opensource.microsoft.com/codeofconduct/faq/

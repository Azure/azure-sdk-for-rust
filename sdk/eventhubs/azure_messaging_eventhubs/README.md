<!-- cspell:words pwsh yourgroup westus servicebus checkpointing  -->

# Azure Event Hubs Client Package for Rust

Azure Event Hubs crate for the Microsoft Azure SDK for Rust.

[Azure Event Hubs](https://azure.microsoft.com/services/event-hubs/) is a big data streaming platform and event ingestion service from Microsoft. For more information about Event Hubs see: [link](https://learn.microsoft.com/azure/event-hubs/event-hubs-about).

Use the client library `azure_messaging_eventhubs` in your application to:

-   Send events to an event hub.
-   Consume events from an event hub.

Key links:

-   [Source code][source]
-   [API Reference Documentation][rustdoc]
-   [Product documentation](https://azure.microsoft.com/services/event-hubs/)
-   [Samples][rustdoc_examples]

## Getting started

### Install the package

Add the Azure Event Hubs client package for rust to your `cargo.toml` file:

```bash
cargo add azure_messaging_eventhubs
```

### Prerequisites

-   A Rust Compiler. See [here](https://www.rust-lang.org/tools/install) for installation instructions.
-   An [Azure subscription](https://azure.microsoft.com/free/)
-   An [Event Hub namespace](https://learn.microsoft.com/azure/event-hubs/).
-   An Event Hub. You can create an event hub in your Event Hubs Namespace using the [Azure Portal](https://learn.microsoft.com/azure/event-hubs/event-hubs-create), or the [Azure CLI](https://learn.microsoft.com/azure/event-hubs/event-hubs-quickstart-cli).

#### Create a namespace using the Azure CLI

Login to the CLI:

```pwsh
az login
```

Create a resource group:

```pwsh
az group create --name <your group name> --location <your location> --subscription <your subscription>
```

This should output something like:

```json
{
    "id": "/subscriptions/<your subscription ID>/resourceGroups/<your group name>",
    "location": "<your location>",
    "managedBy": null,
    "name": "<yourgroup name>",
    "properties": {
        "provisioningState": "Succeeded"
    },
    "tags": null,
    "type": "Microsoft.Resources/resourceGroups"
}
```

Create an Event Hubs namespace:

```pwsh
 az eventhubs namespace create --resource-group <your group name> --name <your namespace name> --sku Standard  --subscription <your subscription>
```

This should output something like:

```json
{
    "createdAt": "2023-08-10T18:41:54.19Z",
    "disableLocalAuth": false,
    "id": "/subscriptions/<your subscription ID>/resourceGroups/<your group name>/providers/Microsoft.EventHub/namespaces/<your namespace>",
    "isAutoInflateEnabled": false,
    "kafkaEnabled": true,
    "location": "West US",
    "maximumThroughputUnits": 0,
    "metricId": "REDACTED",
    "minimumTlsVersion": "1.2",
    "name": "<your namespace name>",
    "provisioningState": "Succeeded",
    "publicNetworkAccess": "Enabled",
    "resourceGroup": "<your resource group>",
    "serviceBusEndpoint": "https://<your namespace name>.servicebus.windows.net:443/",
    "sku": {
        "capacity": 1,
        "name": "Standard",
        "tier": "Standard"
    },
    "status": "Active",
    "tags": {},
    "type": "Microsoft.EventHub/Namespaces",
    "updatedAt": "2023-08-10T18:42:41.343Z",
    "zoneRedundant": false
}
```

Create an EventHub:

```pwsh
az eventhubs eventhub create --resource-group <your resource group> --namespace-name <your namespace name> --name <your eventhub name>
```

That should output something like:

```json
{
    "createdAt": "2023-08-10T21:02:07.62Z",
    "id": "/subscriptions/<your subscription>/resourceGroups/<your group name>/providers/Microsoft.EventHub/namespaces/<your namespace name>/eventhubs/<your eventhub name>",
    "location": "westus",
    "messageRetentionInDays": 7,
    "name": "<your eventhub name>",
    "partitionCount": 4,
    "partitionIds": ["0", "1", "2", "3"],
    "resourceGroup": "<your group name>",
    "retentionDescription": {
        "cleanupPolicy": "Delete",
        "retentionTimeInHours": 168
    },
    "status": "Active",
    "type": "Microsoft.EventHub/namespaces/eventhubs",
    "updatedAt": "2023-08-10T21:02:16.29Z"
}
```

### Authenticate the client

Event Hub clients are created using a credential from the [Azure Identity package][azure_identity_pkg], like [DefaultAzureCredential][default_azure_credential].

# Key concepts

An Event Hub [**namespace**](https://learn.microsoft.com/azure/event-hubs/event-hubs-features#namespace) can have multiple event hubs.
Each event hub, in turn, contains [**partitions**](https://learn.microsoft.com/azure/event-hubs/event-hubs-features#partitions) which
store events.

<!-- NOTE: Fix dead links -->

Events are published to an event hub using an [event publisher](https://learn.microsoft.com/azure/event-hubs/event-hubs-features#event-publishers). In this package, the event publisher is the [ProducerClient]()

Events can be consumed from an event hub using an [event consumer](https://learn.microsoft.com/azure/event-hubs/event-hubs-features#event-consumers). In this package there are two types for consuming events:

-   The basic event consumer is the PartitionClient, in the [ConsumerClient][consumer_client]. This consumer is useful if you already known which partitions you want to receive from.
-   A distributed event consumer, which uses Azure Blobs for checkpointing and coordination. This is implemented in the [Processor](https://azure.github.io/azure-sdk-for-cpp/storage.html).
    The Processor is useful when you want to have the partition assignment be dynamically chosen, and balanced with other Processor instances.

More information about Event Hubs features and terminology can be found here: [link](https://learn.microsoft.com/azure/event-hubs/event-hubs-features)

# Examples

Examples for various scenarios can be found on in the samples directory in our GitHub repo for
[Event Hubs](https://github.com/Azure/azure-sdk-for-rust/tree/main/sdk/eventhubs/azure-messaging-eventhubs/samples).

## Send events

The following example shows how to send events to an event hub:

```rust
async fn send_events() {
    let host = std::env::var("EVENTHUBS_HOST").unwrap();
    let eventhub = std::env::var("EVENTHUB_NAME").unwrap();

    let credential = azure_identity::DefaultAzureCredential::new().unwrap();

    let client = azure_messaging_eventhubs::producer::ProducerClient::new(
        host,
        eventhub.clone(),
        credential,
        Some(azure_messaging_eventhubs::producer::ProducerClientOptions{
          application_id: Some("test_create_batch".to_string()),
          ..Default::default()
        }),
      );
    client.open().await.unwrap();
    {
        let mut batch = client.create_batch(None).await.unwrap();
        assert_eq!(batch.len(), 0);
        assert!(batch.try_add_event_data(vec![1, 2, 3, 4], None).unwrap());

        let res = client.submit_batch(&batch).await;
        assert!(res.is_ok());
    }
}
```

## Receive events

The following example shows how to receive events from partition 1 on an event hub:

```rust no_run
async fn receive_events() {
    use futures::pin_mut;
    use async_std::stream::StreamExt;

    let host = std::env::var("EVENTHUBS_HOST").unwrap();
    let eventhub = std::env::var("EVENTHUB_NAME").unwrap();

    let credential = azure_identity::DefaultAzureCredential::new().unwrap();
    let client = azure_messaging_eventhubs::consumer::ConsumerClient::new(
        host,
        eventhub,
        None,
        credential,
        Some(
            azure_messaging_eventhubs::consumer::ConsumerClientOptions{
                application_id: Some("receive_lots_of_events".to_string()),
                ..Default::default()}
        ),
    );

    client.open().await.unwrap();

    let event_stream = client
        .receive_events_on_partition(
            "0".to_string(),
            Some(
                azure_messaging_eventhubs::consumer::ReceiveOptions{
                    start_position: Some(azure_messaging_eventhubs::consumer::StartPosition{
                        location: azure_messaging_eventhubs::consumer::StartLocation::Earliest,
                        ..Default::default()
                    }),
                    ..Default::default()
                },
            ))
        .await;

    pin_mut!(event_stream); // Needed for iteration.
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
}
```

# Troubleshooting

## Logging

The Event Hubs SDK client uses the [tracing](https://docs.rs/tracing/latest/tracing/) package to
enable diagnostics.

## Contributing

For details on contributing to this repository, see the [contributing guide][azure_sdk_for_cpp_contributing].

This project welcomes contributions and suggestions. Most contributions require you to agree to a
Contributor License Agreement (CLA) declaring that you have the right to, and actually do, grant us
the rights to use your contribution. For details, visit [the contributor license agreement page](https://cla.microsoft.com).

When you submit a pull request, a CLA-bot will automatically determine whether you need to provide
a CLA and decorate the PR appropriately (e.g., label, comment). Simply follow the instructions
provided by the bot. You will only need to do this once across all repos using our CLA.

This project has adopted the [Microsoft Open Source Code of Conduct](https://opensource.microsoft.com/codeofconduct/).
For more information see the [Code of Conduct FAQ](https://opensource.microsoft.com/codeofconduct/faq/) or
contact [opencode@microsoft.com](mailto:opencode@microsoft.com) with any additional questions or comments.

### Additional Helpful Links for Contributors

Many people all over the world have helped make this project better. You'll want to check out:

-   [What are some good first issues for new contributors to the repo?](https://github.com/azure/azure-sdk-for-rust/issues?q=is%3Aopen+is%3Aissue+label%3A%22up+for+grabs%22)
-   [How to build and test your change][azure_sdk_for_cpp_contributing_developer_guide]
-   [How you can make a change happen!][azure_sdk_for_cpp_contributing_pull_requests]
-   Frequently Asked Questions (FAQ) and Conceptual Topics in the detailed [Azure SDK for C++ wiki](https://github.com/azure/azure-sdk-for-cpp/wiki).

<!-- ### Community-->

### Reporting security issues and security bugs

Security issues and bugs should be reported privately, via email, to the Microsoft Security Response Center (MSRC) <secure@microsoft.com>. You should receive a response within 24 hours. If for some reason you do not, please follow up via email to ensure we received your original message. Further information, including the MSRC PGP key, can be found in the [Security TechCenter](https://www.microsoft.com/msrc/faqs-report-an-issue).

### License

Azure SDK for C++ is licensed under the [MIT](https://github.com/Azure/azure-sdk-for-cpp/blob/main/LICENSE.txt) license.

<!-- LINKS -->

[azure_sdk_for_cpp_contributing]: https://github.com/Azure/azure-sdk-for-rust/blob/main/CONTRIBUTING.md
[azure_sdk_for_cpp_contributing_developer_guide]: https://github.com/Azure/azure-sdk-for-rust/blob/main/CONTRIBUTING.md#developer-guide
[azure_sdk_for_cpp_contributing_pull_requests]: https://github.com/Azure/azure-sdk-for-rust/blob/main/CONTRIBUTING.md#pull-requests
[consumer_client]: https://azuresdkdocs.blob.core.windows.net/$web/cpp/azure-messaging-eventhubs/latest/class_azure_1_1_messaging_1_1_event_hubs_1_1_consumer_client.html
[producer_client]: https://azuresdkdocs.blob.core.windows.net/$web/cpp/azure-messaging-eventhubs/1.0.0-beta.1/class_azure_1_1_messaging_1_1_event_hubs_1_1_producer_client.html
[source]: https://github.com/Azure/azure-sdk-for-rust/tree/feature/track2/sdk/eventhubs/azure_messaging_eventhubs
[azure_identity_pkg]: https://docs.rs/azure_identity/latest/azure_identity/
[default_azure_credential]: https://docs.rs/azure_identity/latest/azure_identity/struct.DefaultAzureCredential.html
[rustdoc]: https://docs.rs/azure_messaging_eventhubs/latest/azure_messaging_eventhubs
[rustdoc_examples]: https://github.com/Azure/azure-sdk-for-cpp/tree/main/sdk/eventhubs/azure-messaging-eventhubs/samples

![Impressions](https://azure-sdk-impressions.azurewebsites.net/api/impressions/azure-sdk-for-cpp%2Fsdk%2Feventhubs%2FREADME.png)

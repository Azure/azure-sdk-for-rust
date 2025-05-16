<!-- cspell:words pwsh yourgroup westus servicebus checkpointing  CAL MSRC TechCenter TODO -->

# Azure Service Bus client library for Rust

[Azure Service Bus](https://learn.microsoft.com/azure/service-bus-messaging) is a highly reliable cloud messaging service for providing real-time and fault-tolerant communication between distributed senders and receivers. For more information about Service Bus see: [link](https://learn.microsoft.com/azure/service-bus-messaging/service-bus-messaging-overview).

The Azure Service Bus client library allows for both sending and receiving messages using Azure Service Bus.

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
* An [Service Bus namespace](https://learn.microsoft.com/azure/service-bus-messaging/)
* An Service Bus instance. You can create an Service Bus instance in your Service Bus Namespace using the [Azure Portal](https://learn.microsoft.com/azure/service-bus-messaging/service-bus-quickstart-portal), or the [Azure CLI](https://learn.microsoft.com/azure/service-bus-messaging/service-bus-quickstart-cli).

If you use the Azure CLI, replace `<your-resource-group-name>`, `<your-service-bus-namespace-name>`, and `<your-service-bus-queue-name>` with your own, unique names:

Create an Service Bus Namespace:

```azurecli
az servicebus namespace create --resource-group <your-resource-group-name> --name <your-service-bus-namespace-name> --sku Standard
```

Create an Service Bus Queue:

```azurecli
az servicebus queue create --resource-group <your-resource-group-name> --namespace-name <your-service-bus-namespace-name> --name <your-service-bus-queue-name>
```

Get an Service Bus connection string:

```azurecli
az servicebus namespace authorization-rule keys list --resource-group <your-resource-group-name> --namespace-name <your-service-bus-namespace-name> --name RootManageSharedAccessKey --query primaryConnectionString --output tsv
```

### Install dependencies

Add the following crates to your project:

```sh
cargo add azure_identity tokio
```

### Authenticate the client

<!-- TODO: Describe general authentication flow -->
The `DefaultAzureCredential` will automatically pick up on an Azure CLI authentication. Ensure you are logged in with the Azure CLI:

```azurecli
az login
```

Instantiate a `DefaultAzureCredential` to pass to the client. The same instance of a token credential can be used with multiple clients if they will be authenticating with the same identity.

### Create an Service Bus message producer and send an event
<!-- TODO: Add example of using producer to send an event -->

# Key concepts

A Service Bus namespace can contain multiple queues and topics, which are used to organize and manage messaging.

### Queues and Topics

- Queues: Store messages that are sent by producers and received by consumers in a point-to-point communication pattern.
- Topics: Enable publish-subscribe messaging, where messages are sent to a topic and multiple subscriptions can receive copies of the messages.

### Producers and Senders

Events or messages are sent to queues or topics using a **Sender**. The sender handles message serialization and transmission to the Service Bus.

### Consumers and Receivers

Messages are received from queues or subscriptions using a **Receiver**. Receivers can be configured for different receive modes, such as peek-lock or receive-and-delete.

### Message Processing
- Peek-lock: Messages are received and locked, allowing the consumer to process and explicitly complete or abandon them.
- Receive: Messages are received and deleted immediately upon reception.

More information about Service Bus features and terminology can be found here: [link](https://learn.microsoft.com/azure/service-bus-messaging/advanced-features-overview)

# Examples

<!-- NOTE: To be done -->

# Troubleshooting

## General

When you interact with the Azure Service Bus client library using the Rust SDK, errors returned by the service are returned as `azure_core::Error` values using `ErrorKind::Other` which are `azure_messaging_servicebus::Error` values.

## Logging

The Service Bus SDK client uses the [tracing](https://docs.rs/tracing/latest/tracing/) package to
enable diagnostics.

# Contributing

See the [CONTRIBUTING.md] for details on building, testing, and contributing to these libraries.

This project welcomes contributions and suggestions. Most contributions require you to agree to a Contributor License Agreement (CLA) declaring that you have the right to, and actually do, grant us the rights to use your contribution. For details, visit <https://opensource.microsoft.com/cla/>.

When you submit a pull request, a CLA-bot will automatically determine whether you need to provide a CLA and decorate the PR appropriately (e.g., label, comment). Simply follow the instructions provided by the bot. You will only need to do this once across all repos using our CLA.

This project has adopted the [Microsoft Open Source Code of Conduct]. For more information see the [Code of Conduct FAQ] or contact <opencode@microsoft.com> with any additional questions or comments.

## Reporting security issues and security bugs

Security issues and bugs should be reported privately, via email, to the Microsoft Security Response Center (MSRC) <secure@microsoft.com>. You should receive a response within 24 hours. If for some reason you do not, please follow up via email to ensure we received your original message. Further information, including the MSRC PGP key, can be found in the [Security TechCenter](https://www.microsoft.com/msrc/faqs-report-an-issue).

## License

Azure SDK for Rust is licensed under the [MIT](https://github.com/Azure/azure-sdk-for-cpp/blob/main/LICENSE.txt) license.

<!-- LINKS -->
[API reference documentation]: https://docs.rs/azure_messaging_servicebus/latest/azure_messaging_servicebus
[Azure CLI]: https://learn.microsoft.com/cli/azure
[Azure subscription]: https://azure.microsoft.com/free/
[Azure Identity]: https://github.com/Azure/azure-sdk-for-rust/tree/main/sdk/identity/azure_identity
[Microsoft Open Source Code of Conduct]: https://opensource.microsoft.com/codeofconduct/
[Product documentation]: https://learn.microsoft.com/azure/service-bus-messaging/
[Cargo]: https://crates.io/
[Package (crates.io)]: https://crates.io/crates/azure_messaging_servicebus
[Source code]: https://github.com/Azure/azure-sdk-for-rust/tree/main/sdk/servicebus/azure_messaging_servicebus/src
[CONTRIBUTING.md]: https://github.com/Azure/azure-sdk-for-rust/blob/main/CONTRIBUTING.md
[Code of Conduct FAQ]: https://opensource.microsoft.com/codeofconduct/faq/
[default_cred_ref]: https://docs.rs/azure_identity/latest/azure_identity/struct.DefaultAzureCredential.html

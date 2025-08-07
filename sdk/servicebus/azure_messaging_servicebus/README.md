<!-- cspell:words pwsh yourgroup westus servicebus checkpointing  -->

# Azure Service Bus client library for Rust

Azure Service Bus is a fully managed enterprise message broker with message queues and publish-subscribe topics. Service Bus is used to decouple applications and services from each other, providing the following benefits:

-   Load-balancing work across competing workers
-   Safely routing and transferring data and control across service and application boundaries
-   Coordinating transactional work that requires a high-degree of reliability

[Source code] | [Package (crates.io)] | [API reference documentation] | [Product documentation]

## WARNING - Not production ready crate

This crate is in early development, it **SHOULD NOT** be used in production.

## Getting started

### Prerequisites

-   Rust 1.85.0 or later
-   An Azure subscription
-   A Service Bus namespace

### Install dependencies

Add the following crates to your project:

```sh
cargo add azure_identity tokio
```

### Authenticate the client

In order to interact with the Azure Service Bus service, you'll need to create an instance of a client class. To create a client object, you'll need the Service Bus namespace and a credential object.

#### Using Azure Identity

```rust,no_run
# async fn example() -> Result<(), Box<dyn std::error::Error>> {
use azure_messaging_servicebus::ServiceBusClient;
use azure_identity::DeveloperToolsCredential;

let credential = DeveloperToolsCredential::new(None)?;
let client = ServiceBusClient::builder()
    .open("your_namespace.servicebus.windows.net", credential.clone())
    .await?;
# Ok(())
# }
```

The `ServiceBusClient` supports various credential types from the `azure_identity` crate:

-   **DeveloperToolsCredential** (Recommended): Automatically tries multiple authentication methods
-   **ClientSecretCredential**: For service principals with client secrets
-   **ManagedIdentityCredential**: For Azure resources with managed identity
-   **AzureCliCredential**: For development using Azure CLI authentication

All credentials handle token acquisition, caching, and automatic refresh automatically.

For a comprehensive example showing all credential types, see [token_credential_auth.rs](https://github.com/Azure/azure-sdk-for-rust/blob/main/sdk/servicebus/azure_messaging_servicebus/examples/token_credential_auth.rs).

## Key concepts

-   **Namespace**: A Service Bus namespace is a scoping container for all messaging components.
-   **Queue**: A queue allows storage of messages until the receiving application is available to receive and process them.
-   **Topic**: A topic provides a one-to-many form of communication using a publish/subscribe pattern.
-   **Subscription**: A subscription is used to receive messages from a topic.
-   **Message**: A message is a package of information that contains both data and metadata.

## Examples

### Samples

See our [samples]

### Send a message to a queue

```rust,no_run
# async fn example() -> Result<(), Box<dyn std::error::Error>> {
use azure_messaging_servicebus::{ServiceBusClient, Message, CreateSenderOptions, SendMessageOptions};
use azure_identity::DeveloperToolsCredential;

let credential = DeveloperToolsCredential::new(None)?;
let client = ServiceBusClient::builder()
    .open("your_namespace.servicebus.windows.net", credential.clone())
    .await?;
let sender = client.create_sender("my_queue", None).await?;

let message = Message::from("Hello, Service Bus!");
sender.send_message(message, None).await?;
# Ok(())
# }
```

### Receive messages from a queue

```rust,no_run
# async fn example() -> Result<(), Box<dyn std::error::Error>> {
use azure_messaging_servicebus::{ServiceBusClient, CreateReceiverOptions, ReceiveMessageOptions, CompleteMessageOptions};
use azure_identity::DeveloperToolsCredential;

let credential = DeveloperToolsCredential::new(None)?;
let client = ServiceBusClient::builder()
    .open("your_namespace.servicebus.windows.net", credential.clone())
    .await?;
let receiver = client.create_receiver("my_queue", None).await?;

let messages = receiver.receive_messages(5, None).await?;
for message in messages {
    println!("Received: {}", message.body_as_string()?);
    receiver.complete_message(&message, None).await?;
}
# Ok(())
# }
```

### Send a message to a topic

```rust,no_run
# async fn example() -> Result<(), Box<dyn std::error::Error>> {
use azure_messaging_servicebus::{ServiceBusClient, Message, CreateSenderOptions, SendMessageOptions};
use azure_identity::DeveloperToolsCredential;

let credential = DeveloperToolsCredential::new(None)?;
let client = ServiceBusClient::builder()
    .open("your_namespace.servicebus.windows.net", credential.clone())
    .await?;
let sender = client.create_sender("my_topic", None).await?;

let message = Message::from("Hello, Topic subscribers!");
sender.send_message(message, None).await?;
# Ok(())
# }
```

### Receive messages from a subscription

```rust,no_run
# async fn example() -> Result<(), Box<dyn std::error::Error>> {
use azure_messaging_servicebus::{ServiceBusClient, CreateReceiverOptions, ReceiveMessageOptions, CompleteMessageOptions};
use azure_identity::DeveloperToolsCredential;

let credential = DeveloperToolsCredential::new(None)?;
let client = ServiceBusClient::builder()
    .open("your_namespace.servicebus.windows.net", credential.clone())
    .await?;
let receiver = client.create_receiver_for_subscription("my_topic", "my_subscription", None).await?;

let messages = receiver.receive_messages(5, None).await?;
for message in messages {
    println!("Received: {}", message.body_as_string()?);
    receiver.complete_message(&message, None).await?;
}
# Ok(())
# }
```

## Troubleshooting

-   Read about the different [Service Bus messaging patterns]

## Contributing

See the [CONTRIBUTING.md] for details on building, testing, and contributing to these libraries.

This project welcomes contributions and suggestions. Most contributions require you to agree to a Contributor License Agreement (CLA) declaring that you have the right to, and actually do, grant us the rights to use your contribution. For details, visit <https://opensource.microsoft.com/cla/>.

When you submit a pull request, a CLA-bot will automatically determine whether you need to provide a CLA and decorate the PR appropriately (e.g., label, comment). Simply follow the instructions provided by the bot. You will only need to do this once across all repos using our CLA.

This project has adopted the [Microsoft Open Source Code of Conduct]. For more information see the [Code of Conduct FAQ] or contact <opencode@microsoft.com> with any additional questions or comments.

### Reporting security issues and security bugs

Security issues and bugs should be reported privately, via email, to the Microsoft Security Response Center (MSRC) <secure@microsoft.com>. You should receive a response within 24 hours. If for some reason you do not, please follow up via email to ensure we received your original message. Further information, including the MSRC PGP key, can be found in the [Security TechCenter](https://www.microsoft.com/msrc/faqs-report-an-issue).

### License

Azure SDK for Rust is licensed under the [MIT](https://github.com/Azure/azure-sdk-for-rust/blob/main/LICENSE.txt) license.

[CONTRIBUTING.md]: https://github.com/Azure/azure-sdk-for-rust/blob/main/CONTRIBUTING.md
[samples]: https://github.com/Azure/azure-sdk-for-rust/tree/main/sdk/servicebus/azure_messaging_servicebus/examples
[Service Bus messaging patterns]: https://docs.microsoft.com/azure/service-bus-messaging/
[Microsoft Open Source Code of Conduct]: https://opensource.microsoft.com/codeofconduct/
[Code of Conduct FAQ]: https://opensource.microsoft.com/codeofconduct/faq/

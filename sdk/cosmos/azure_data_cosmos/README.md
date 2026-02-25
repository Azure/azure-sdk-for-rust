# Azure Cosmos DB SDK for Rust

This client library enables client applications to connect to Azure Cosmos DB via the NoSQL API. Azure Cosmos DB is a globally distributed, multi-model database service.

[Source code] | [Package (crates.io)] | [API reference documentation] | [Azure Cosmos DB for NoSQL documentation]

## Getting started

### Install the package

Install the Azure Cosmos DB SDK for Rust with cargo:

```sh
cargo add azure_data_cosmos
```

### Prerequisites

* An [Azure subscription] or free Azure Cosmos DB trial account.

Note: If you don't have an Azure subscription, create a free account before you begin.
You can Try Azure Cosmos DB for free without an Azure subscription, free of charge and commitments, or create an Azure Cosmos DB free tier account, with the first 400 RU/s and 5 GB of storage for free. You can also use the Azure Cosmos DB Emulator with a URI of <https://localhost:8081>. For the key to use with the emulator, see [how to develop with the emulator](https://learn.microsoft.com/azure/cosmos-db/how-to-develop-emulator).

### Create an Azure Cosmos DB account

You can create an Azure Cosmos DB account using:

* [Azure Portal](https://portal.azure.com).
* [Azure CLI](https://learn.microsoft.com/cli/azure).
* [Azure ARM](https://learn.microsoft.com/azure/cosmos-db/quick-create-template).

#### Authenticate the client

In order to interact with the Azure Cosmos DB service you'll need to create an instance of the `CosmosClient` struct. To make this possible you will need a URL and key of the Azure Cosmos DB service.

## Examples

The following section provides several code snippets covering some of the most common Azure Cosmos DB NoSQL API tasks, including:

* [Create Client](#create-cosmos-db-client "Create Cosmos DB client")
* [CRUD operation on Items](#crud-operation-on-items "CRUD operation on Items")

### Create Cosmos DB Client

In order to interact with the Azure Cosmos DB service, you'll need to create an instance of the `CosmosClient`. You need an endpoint URL and credentials to instantiate a client object.

#### Using Microsoft Entra ID

The example shown below use a `DeveloperToolsCredential`, which is appropriate for most local development environments. Additionally, we recommend using a managed identity for authentication in production environments. You can find more information on different ways of authenticating and their corresponding credential types in the [Azure Identity] documentation.

The `DeveloperToolsCredential` will automatically pick up on an Azure CLI authentication. Ensure you are logged in with the Azure CLI:

```sh
az login
```

Instantiate a `DeveloperToolsCredential` to pass to the client. The same instance of a token credential can be used with multiple clients if they will be authenticating with the same identity.

```rust
use azure_identity::DeveloperToolsCredential;
use azure_data_cosmos::{CosmosClient, CosmosAccountReference, CosmosAccountEndpoint};

async fn example() -> Result<(), Box<dyn std::error::Error>> {
    let credential: std::sync::Arc<dyn azure_core::credentials::TokenCredential> =
        DeveloperToolsCredential::new(None)?;
    let endpoint: CosmosAccountEndpoint = "https://myaccount.documents.azure.com/"
        .parse()?;
    let account = CosmosAccountReference::with_credential(endpoint, credential);
    let cosmos_client = CosmosClient::builder()
        .build(account).await?;
    Ok(())
}
```

#### Using account keys

Cosmos DB also supports account keys, though we strongly recommend using Entra ID authentication. To use account keys, you will need to enable the `key_auth` feature:

```sh
cargo add azure_data_cosmos --features key_auth
```

For more information, see the [API reference documentation].

### CRUD operation on Items

```rust
use serde::{Serialize, Deserialize};
use azure_data_cosmos::CosmosClient;

#[derive(Serialize, Deserialize)]
struct Item {
    pub id: String,
    pub partition_key: String,
    pub value: String,
}

async fn example(cosmos_client: CosmosClient) -> Result<(), Box<dyn std::error::Error>> {
    let item = Item {
        id: "1".into(),
        partition_key: "partition1".into(),
        value: "2".into(),
    };

    let container = cosmos_client.database_client("myDatabase").container_client("myContainer").await;

    // Create an item
    container.create_item("partition1", item, None).await?;

    // Read an item
    let item_response = container.read_item("partition1", "1", None).await?;
    let mut item: Item = item_response.into_model()?;

    item.value = "3".into();

    // Replace an item
    container.replace_item("partition1", "1", item, None).await?;

    // Delete an item
    container.delete_item("partition1", "1", None).await?;
    Ok(())
}
```

## Next steps

* [Resource Model of Azure Cosmos DB Service](https://learn.microsoft.com/azure/cosmos-db/sql-api-resources)
* [Azure Cosmos DB Resource URI](https://learn.microsoft.com/rest/api/documentdb/documentdb-resource-uri-syntax-for-rest)
* [Partitioning](https://learn.microsoft.com/azure/cosmos-db/partition-data)
* [Using emulator](https://github.com/Azure/azure-documentdb-dotnet/blob/master/docs/documentdb-nosql-local-emulator.md)

### Provide feedback

If you encounter bugs or have suggestions, [open an issue](https://github.com/Azure/azure-sdk-for-rust/issues).

## Contributing

This project welcomes contributions and suggestions. Most contributions require you to agree to a Contributor License Agreement (CLA) declaring that you have the right to, and actually do, grant us the rights to use your contribution. For details, visit [https://cla.microsoft.com](https://cla.microsoft.com).

When you submit a pull request, a CLA-bot will automatically determine whether you need to provide a CLA and decorate the PR appropriately (e.g., label, comment). Simply follow the instructions provided by the bot. You'll only need to do this once across all repos using our CLA.

This project has adopted the [Microsoft Open Source Code of Conduct](https://opensource.microsoft.com/codeofconduct/). For more information, see the [Code of Conduct FAQ](https://opensource.microsoft.com/codeofconduct/faq/) or contact [opencode@microsoft.com](mailto:opencode@microsoft.com) with any additional questions or comments.

<!-- LINKS -->
[Azure subscription]: https://azure.microsoft.com/free/
[Azure Identity]: https://github.com/Azure/azure-sdk-for-rust/tree/main/sdk/identity/azure_identity
[API reference documentation]: https://docs.rs/azure_data_cosmos/latest/azure_data_cosmos/
[Azure Cosmos DB for NoSQL documentation]: https://learn.microsoft.com/azure/cosmos-db/nosql/
[Package (crates.io)]: https://crates.io/crates/azure_data_cosmos
[Source code]: https://github.com/Azure/azure-sdk-for-rust/tree/main/sdk/cosmos/azure_data_cosmos

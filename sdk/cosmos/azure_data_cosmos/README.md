# Azure Cosmos DB SDK for Rust.

This client library enables client applications to connect to Azure Cosmos DB via the NoSQL API. Azure Cosmos DB is a globally distributed, multi-model database service.

[Source code] | [Package (crates.io)] | [API reference documentation] | [Azure Cosmos DB for NoSQL documentation]

## Getting started

### Install the package

Install the Azure Cosmos DB SDK for Rust with cargo:

```bash
cargo add azure_data_cosmos
```

### Prerequisites

* An [Azure subscription] or free Azure Cosmos DB trial account.

Note: If you don't have an Azure subscription, create a free account before you begin.
You can Try Azure Cosmos DB for free without an Azure subscription, free of charge and commitments, or create an Azure Cosmos DB free tier account, with the first 400 RU/s and 5 GB of storage for free. You can also use the Azure Cosmos DB Emulator with a URI of https://localhost:8081. For the key to use with the emulator, see [how to develop with the emulator](https://learn.microsoft.com/azure/cosmos-db/how-to-develop-emulator).

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
* [Create Database](#create-database "Create Database")
* [Create Container](#create-container "Create Container")
* [CRUD operation on Items](#crud-operation-on-items "CRUD operation on Items")

### Create Cosmos DB Client

The clients support different forms of authentication. The azcosmos library supports authorization via Microsoft Entra identities or an account key.

**Using Microsoft Entra identities**

```rust
use azure_data_cosmos::CosmosClient;

let credential = DefaultAzureCredential::new()?;
let cosmos_client = CosmosClient::new("myAccountEndpointURL", credential, None)?;
```

**Using account keys**

**IMPORTANT**: It is strongly recommended to use Microsoft Entra identities for authentication. The Azure Cosmos DB SDK for Rust does not support account keys by default. To enable support for account keys, you will need to enable the `key_auth` feature in your Cargo.toml file.

```toml
[dependencies]
azure_data_cosmos = { version = "...", features = ["key_auth"] }
```

```rust
use azure_core::credentials::Secret;
use azure_data_cosmos::CosmosClient;

const COSMOS_DB_ENDPOINT: &str = "someEndpoint";
const COSMOS_DB_KEY: &str = "someKey";

let key = Secret::from(COSMOS_DB_KEY);
let cosmos_client = CosmosClient::new(COSMOS_DB_ENDPOINT, key, None)?;
```

### Create Database

**NOTE:** Creating a database requires authenticating with an account key, which requires the `key_auth` feature to be enabled. See [Create Cosmos DB Client](#create-cosmos-db-client "Create Cosmos DB client") for more information.

Using the client created in previous example, you can create a database like this:

```rust
let response = cosmos_client.create_database(dbName, None).await?;
```

### Create Container

**NOTE:** Creating a container requires authenticating with an account key, which requires the `key_auth` feature to be enabled. See [Create Cosmos DB Client](#create-cosmos-db-client "Create Cosmos DB client") for more information.

Using the above created database for creating a container, like this:

```rust
let properties = ContainerProperties {
    id: "aContainer".into(),
    partition_key: PartitionKeyDefinition::new(vec!["/id".into()]),
}
let options = CreateContainerOptions{
    throughput: Some(ThroughputProperties::manual(400)),
    ..Default::default()
};
let response = database.create_container(context, properties, Some(options)).await?;
```

### CRUD operation on Items

```rust
use serde::Serialize;

#[derive(Serialize)]
struct Item {
    pub id: String,
    pub partition_key: String,
    pub value: String,
}

let item = Item {
    id: "1".into(),
    partition_key: "partition1".into(),
    value: "2".into(),
};

let container = cosmos_client.database_client(dbName).container_client(containerName);

// Create an item
container.create_item("partition1", item, None).await?;

// Read an item
let item_response = container.read_item("partition1", "1", None).await?;
let mut item: Item = item_response.into_json_body().await?;

item.value = "3".into();

// Replace an item
container.replace_item("partition1", "1", item, None).await?;

// Patch an item
let patch = PatchDocument::default()
    .with_add("/newField", "newValue")
    .with_remove("/oldFieldToRemove");

container.patch_item("partition1", "1", patch, None).await?;

// Delete an item
container.delete_item("partition1", "1", None).await?;
```

## Next steps

- [Resource Model of Azure Cosmos DB Service](https://learn.microsoft.com/azure/cosmos-db/sql-api-resources)
- [Azure Cosmos DB Resource URI](https://learn.microsoft.com/rest/api/documentdb/documentdb-resource-uri-syntax-for-rest)
- [Partitioning](https://learn.microsoft.com/azure/cosmos-db/partition-data)
- [Using emulator](https://github.com/Azure/azure-documentdb-dotnet/blob/master/docs/documentdb-nosql-local-emulator.md)

## Next steps

### Client library support

Client and management libraries <!-- TODO: Update link and uncomment when Rust SDK has a page on the releases site.> listed on the [Azure SDK release page](https://azure.github.io/azure-sdk/releases/latest/python.html)</!--> that support Microsoft Entra authentication accept credentials from this library. You can learn more about using these libraries in their documentation, which is <!-- TODO: uncomment when Rust SDK has a release page.>linked from the release page</!-->available at [Docs.rs](https://Docs.rs/azure_data_cosmos).

### Provide feedback

If you encounter bugs or have suggestions, [open an issue](https://github.com/Azure/azure-sdk-for-rust/issues).

## Contributing

This project welcomes contributions and suggestions. Most contributions require you to agree to a Contributor License Agreement (CLA) declaring that you have the right to, and actually do, grant us the rights to use your contribution. For details, visit [https://cla.microsoft.com](https://cla.microsoft.com).

When you submit a pull request, a CLA-bot will automatically determine whether you need to provide a CLA and decorate the PR appropriately (e.g., label, comment). Simply follow the instructions provided by the bot. You'll only need to do this once across all repos using our CLA.

This project has adopted the [Microsoft Open Source Code of Conduct](https://opensource.microsoft.com/codeofconduct/). For more information, see the [Code of Conduct FAQ](https://opensource.microsoft.com/codeofconduct/faq/) or contact [opencode@microsoft.com](mailto:opencode@microsoft.com) with any additional questions or comments.

<!-- LINKS -->
[Azure subscription]: https://azure.microsoft.com/free/
[API reference documentation]: https://docs.rs/azure_data_cosmos/latest/azure_data_cosmos/
[Azure Cosmos DB for NoSQL documentation]: https://learn.microsoft.com/en-us/azure/cosmos-db/nosql/
[Package (crates.io)]: https://crates.io/crates/azure_data_cosmos
[Source code]: https://github.com/Azure/azure-sdk-for-rust/tree/main/sdk/cosmos/azure_data_cosmos

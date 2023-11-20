# azure_data_tables

This crate is from the [Azure SDK for Rust](https://github.com/azure/azure-sdk-for-rust). It supports [Azure Table storage](https://docs.microsoft.com/azure/storage/tables/table-storage-overview).

```rust
use azure_core::StatusCode;
use azure_data_tables::{operations::InsertEntityResponse, prelude::*};
use azure_storage::prelude::*;
use futures::stream::StreamExt;
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
struct MyEntity {
    #[serde(rename = "PartitionKey")]
    pub city: String,
    pub name: String,
    #[serde(rename = "RowKey")]
    pub surname: String,
}

#[tokio::main]
async fn main() -> azure_core::Result<()> {
    env_logger::Builder::from_env(env_logger::Env::default().default_filter_or("info"));

    // First we retrieve the account name and access key from environment variables.
    let account =
        std::env::var("STORAGE_ACCOUNT").expect("Set env variable STORAGE_ACCOUNT first!");
    let access_key =
        std::env::var("STORAGE_ACCESS_KEY").expect("Set env variable STORAGE_ACCESS_KEY first!");
    let table_name = std::env::var("STORAGE_TABLE_NAME").expect("Set env variable STORAGE_TABLE_NAME first!");

    let storage_credentials = StorageCredentials::access_key(account.clone(), access_key);
    let table_service = TableServiceClient::new(account, storage_credentials);

    let table_client = table_service.table_client(table_name);
    table_client.create().await?;

    let entity = MyEntity {
        city: "Milan".to_owned(),
        name: "Francesco".to_owned(),
        surname: "A".to_owned(),
    };

    let _: InsertEntityResponse<MyEntity> = table_client.insert(&entity)?.await?;

    // Get a client that refers to the above entity
    let entity_client = table_client.partition_key_client(&entity.city).entity_client(&entity.surname);

    // Get an entity from the table
    let response = entity_client.get().await?;
    let mut entity: MyEntity = response.entity;

    // update the entity in the table
    entity.name = "Ryan".to_owned();
    entity_client.update(&entity, response.etag.into())?.await?;
    entity_client.delete().await?;

    /// delete the client now that we're done
    table_client.delete().await?;
    Ok(())
}

```


License: MIT

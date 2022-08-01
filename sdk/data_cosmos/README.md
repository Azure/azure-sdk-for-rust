# Azure SDK for Rust - Azure Cosmos DB crate

## The Cosmos DB crate.

`azure-cosmos` offers functionality needed to interact with Cosmos DB from Rust. As an abstraction over the [Cosmos DB
Rest API](https://docs.microsoft.com/rest/api/cosmos-db/), anything that is possible through that Rest API
should also be possible with this crate.

### Examples

```rust
// Using the prelude module of the Cosmos crate makes easier to use the Rust Azure SDK for Cosmos DB.
use azure_data_cosmos::prelude::*;
use azure_core::Context;
use serde::{Deserialize, Serialize};

// This is the stuct we want to use in our sample.
// Make sure to have a collection with partition key "a_number" for this example to
// work (you can create with this SDK too, check the examples folder for that task).
#[derive(Serialize, Deserialize, Debug)]
struct MySampleStruct {
    id: String,
    a_string: String,
    a_number: u64,
    a_timestamp: i64,
}

impl<'a> azure_data_cosmos::CosmosEntity<'a> for MySampleStruct {
    type Entity = u64;

    fn partition_key(&'a self) -> Self::Entity {
        self.a_number
    }
}

// This code will perform these tasks:
// 1. Create 10 documents in the collection.
#[tokio::main]
async fn main() -> azure_core::Result<()> {
    // Let's get Cosmos account and access key from env variables.
    let primary_key =
        std::env::var("COSMOS_PRIMARY_KEY").expect("Set env variable COSMOS_PRIMARY_KEY first!");
    let account = std::env::var("COSMOS_ACCOUNT").expect("Set env variable COSMOS_ACCOUNT first!");

    let database_name = std::env::args()
        .nth(1)
        .expect("please specify the database name as first command line parameter");
    let collection_name = std::env::args()
        .nth(2)
        .expect("please specify the collection name as first command line parameter");

    // First, we create an authorization token. There are two types of tokens, master and resource
    // constrained. This SDK supports both.
    // Please check the Azure documentation for details or the examples folder
    // on how to create and use token-based permissions.
    let authorization_token = AuthorizationToken::primary_from_base64(&primary_key)?;

    // Next we will create a Cosmos client.
    let client = CosmosClient::new(account.clone(), authorization_token);

    // We know the database so we can obtain a database client.
    let database = client.database_client(database_name);
    // We know the collection so we can obtain a collection client.
    let collection = database.collection_client(collection_name);

    // Insert 10 documents
    println!("Inserting 10 documents...");
    for i in 0..10 {
        // define the document.
        let document_to_insert = MySampleStruct {
            id: format!("unique_id{}", i),
            a_string: "Something here".to_owned(),
            a_number: i * 100, // this is the partition key
            a_timestamp: OffsetDateTime::now_utc().unix_timestamp(),
        };

        // insert it
        collection
            .create_document(
                Context::new(),
                &document_to_insert,
                CreateDocumentOptions::new().is_upsert(true),
            )
            .await?;
    }
    // wow that was easy and fast, wasn't it? :)
    println!("Done!");

    Ok(())
}
```

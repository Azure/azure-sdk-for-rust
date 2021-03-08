/*!
# The Cosmos DB crate.

`azure-cosmos` offers functionality needed to interact with Cosmos DB from Rust. As an abstraction over the [Cosmos DB
Rest API](https://docs.microsoft.com/en-us/rest/api/cosmos-db/), anything that is possible through that Rest API
should also be possible with this crate.

## Examples

```no_run
// Using the prelude module of the Cosmos crate makes easier to use the Rust Azure SDK for Cosmos DB.
use azure_cosmos::prelude::*;
use azure_core::HttpClient;

use serde::{Deserialize, Serialize};

use std::error::Error;
use std::sync::Arc;

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

impl<'a> azure_cosmos::CosmosEntity<'a, u64> for MySampleStruct {
    fn partition_key(&'a self) -> u64 {
        self.a_number
    }
}

// This code will perform these tasks:
// 1. Create 10 documents in the collection.
#[tokio::main]
async fn main() -> Result<(), Box<dyn Error + Send + Sync>> {
    // Let's get Cosmos account and master key from env variables.
    let master_key =
        std::env::var("COSMOS_MASTER_KEY").expect("Set env variable COSMOS_MASTER_KEY first!");
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
    let authorization_token = AuthorizationToken::primary_from_base64(&master_key)?;

    // Next we will create a Cosmos client.
    let http_client: Arc<Box<dyn HttpClient>> = Arc::new(Box::new(reqwest::Client::new()));
    let client = CosmosClient::new(http_client, account.clone(), authorization_token);

    // We know the database so we can obtain a database client.
    let database_client = client.into_database_client(database_name);
    // We know the collection so we can obtain a collection client.
    let collection_client = database_client.into_collection_client(collection_name);

    // Insert 10 documents
    println!("Inserting 10 documents...");
    for i in 0..10 {
        // define the document.
        let document_to_insert = MySampleStruct {
            id: format!("unique_id{}", i),
            a_string: "Something here".to_owned(),
            a_number: i * 100, // this is the partition key
            a_timestamp: chrono::Utc::now().timestamp(),
        };

        // insert it
        collection_client
            .create_document()
            .is_upsert(true) // this option will overwrite a preexisting document (if any)
            .execute(&document_to_insert)
            .await?;
    }
    // wow that was easy and fast, wasn't it? :)
    println!("Done!");

    Ok(())
}
```
!*/

#![warn(unused_extern_crates)]
#![deny(missing_docs)]
#![recursion_limit = "256"]
#[macro_use]
extern crate log;
#[macro_use]
extern crate serde;
#[macro_use]
extern crate failure;
#[macro_use]
extern crate azure_core;

pub mod clients;
pub mod prelude;
pub mod requests;
pub mod resources;
pub mod responses;

mod consistency_level;
mod cosmos_entity;
mod errors;
mod headers;
mod max_item_count;
mod resource_quota;
mod to_json_vector;

pub use consistency_level::ConsistencyLevel;
pub use cosmos_entity::CosmosEntity;
pub use max_item_count::MaxItemCount;
pub use resource_quota::ResourceQuota;

/// A general error having to do with Cosmos.
pub type CosmosError = Box<dyn std::error::Error + Sync + Send>;

type ReadonlyString = std::borrow::Cow<'static, str>;

#![allow(clippy::enum_variant_names)]
#![allow(clippy::new_without_default)]
#![allow(clippy::module_inception)]

/*!
# The Cosmos DB crate.

`azure-cosmos` offers functionality needed to interact with Cosmos DB from Rust. As an abstraction over the [Cosmos DB
Rest API](https://docs.microsoft.com/rest/api/cosmos-db/), anything that is possible through that Rest API
should also be possible with this crate.

## Examples

```no_run
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
    string: String,
    number: u64,
}

impl azure_data_cosmos::CosmosEntity for MySampleStruct {
    type Entity = u64;

    fn partition_key(&self) -> Self::Entity {
        self.number
    }
}

#[tokio::main]
async fn main() -> azure_core::Result<()> {
    // Let's get Cosmos primary key and account name from env variables.
    let primary_key =
        std::env::var("COSMOS_PRIMARY_KEY").expect("Set env variable COSMOS_PRIMARY_KEY first!");
    let account = std::env::var("COSMOS_ACCOUNT").expect("Set env variable COSMOS_ACCOUNT first!");

    let database_name = std::env::args()
        .nth(1)
        .expect("please specify the database name as the first command line parameter");
    let collection_name = std::env::args()
        .nth(2)
        .expect("please specify the collection name as the second command line parameter");

    // First, create an authorization token. There are two types of tokens: primary and resource constrained.
    // Please check the Azure documentation or the examples folder on how to create and use token-based permissions.
    let authorization_token = AuthorizationToken::primary_from_base64(&primary_key)?;

    // Next we will create a Cosmos client.
    let client = CosmosClient::new(account, authorization_token);

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
            string: "Something here".to_owned(),
            number: i * 100, // this is the partition key
        };

        // insert it
        collection
            .create_document(document_to_insert)
            .is_upsert(true)
            .into_future()
            .await?;
    }

    Ok(())
}
```
*/

#![warn(unused_extern_crates)]
#![deny(missing_docs)]
#![recursion_limit = "256"]
#[macro_use]
extern crate log;
#[macro_use]
extern crate serde;
#[macro_use]
extern crate azure_core;

pub mod clients;
mod operations;
pub mod prelude;
pub mod resources;

mod authorization_policy;
mod consistency_level;
mod cosmos_entity;
mod headers;
mod resource_quota;

pub(crate) use authorization_policy::AuthorizationPolicy;

pub use consistency_level::ConsistencyLevel;
pub use cosmos_entity::CosmosEntity;
pub use resource_quota::ResourceQuota;

type ReadonlyString = std::borrow::Cow<'static, str>;

# Microsoft Azure SDK for Rust

[![docs](https://docs.rs/azure_sdk_for_rust/badge.svg)](docs.rs)

[![legal](https://img.shields.io/github/license/mindflavor/AzureSDKForRust.svg)](LICENSE) 

[![Build Status](https://travis-ci.org/MindFlavor/AzureSDKForRust.svg?branch=master)](https://travis-ci.org/MindFlavor/AzureSDKForRust) [![Coverage Status](https://coveralls.io/repos/MindFlavor/AzureSDKForRust/badge.svg?branch=master&service=github)](https://coveralls.io/github/MindFlavor/AzureSDKForRust?branch=master) [![alpha](https://img.shields.io/badge/stability-alpha-yellow.svg)](https://img.shields.io/badge/stability-alpha-yellow.svg)

[![Crate](https://img.shields.io/crates/v/azure_sdk_for_rust.svg)](https://crates.io/crates/azure_sdk_for_rust) [![cratedown](https://img.shields.io/crates/d/azure_sdk_for_rust.svg)](https://crates.io/crates/azure_sdk_for_rust) [![cratelastdown](https://img.shields.io/crates/dv/azure_sdk_for_rust.svg)](https://crates.io/crates/azure_sdk_for_rust)

[![tag](https://img.shields.io/github/tag/mindflavor/AzureSDKForRust.svg)](https://github.com/MindFlavor/AzureSDKForRust/tree/0.10.0)
[![release](https://img.shields.io/github/release/mindflavor/AzureSDKForRust.svg)](https://github.com/MindFlavor/AzureSDKForRust/tree/0.10.0)
[![commitssince](https://img.shields.io/github/commits-since/mindflavor/AzureSDKForRust/0.10.0.svg)](https://img.shields.io/github/commits-since/mindflavor/AzureSDKForRust/0.10.0.svg)

![GitHub contributors](https://img.shields.io/github/contributors/mindflavor/AzureSDKForRust.svg)

## Introduction
Microsoft Azure expose its technologies via REST API. These APIs are easily consumable from any language (good) but are weakly typed. With this library and its related [crate](https://crates.io/crates/azure_sdk_for_rust/) you can exploit the power of Microsoft Azure from Rust in a idiomatic way.

This crate relies heavily on the excellent crate called [Hyper](https://github.com/hyperium/hyper). As of this library version [0.4.0](https://github.com/MindFlavor/AzureSDKForRust/releases/tag/0.4.0) all the methods are future-aware.

From version 0.8.0 for Cosmos and 0.9.0 for Storage the repo is embracing the builder pattern. As of 0.10.0, most of storage APIs have been migrated to the builder pattern but there are methods still missing. Please chech the relevant issues to follow the update process. This is still an in-progress transition but the resulting API is much more easy to use. Also most checks have been moved to compile-time. Unfortunately the changes are not backward-compatibile. I have blogged about my appoach here: [https://dev.to/mindflavor/rust-builder-pattern-with-types-3chf](https://dev.to/mindflavor/rust-builder-pattern-with-types-3chf).

> **NOTE:** This repository is under heavy development and is likely to break over time. The current releases will probabily contain bugs. As usual open issues if you find any.

## Disclaimer
Although I am a Microsoft employee, this is not a Microsoft endorsed project. It's simply a pet project of mine: I love Rust (who doesn't? :smirk:) and Microsoft Azure technologies so I thought to close the gap between them. It's also a good project for learning Rust. This library relies heavily on [Hyper](https://github.com/hyperium/hyper). We use the latest Hyper code so this library is fully async with Futures and Tokio.
 
## Example
You can find examples in the [```examples```](https://github.com/MindFlavor/AzureSDKForRust/tree/master/examples) folder. Here is a glimpse:

### main.rs

```rust
extern crate azure_sdk_for_rust;

extern crate chrono;
extern crate futures;
extern crate hyper;
extern crate hyper_tls;
extern crate tokio;
extern crate tokio_core;

use std::error::Error;

use futures::future::*;
use tokio_core::reactor::Core;

use azure_sdk_for_rust::cosmos::{AuthorizationToken, Client, TokenType};

#[macro_use]
extern crate serde_derive;
use azure_sdk_for_rust::cosmos;

#[derive(Serialize, Deserialize, Debug)]
struct MySampleStruct<'a> {
    id: &'a str,
    a_string: &'a str,
    a_number: u64,
    a_timestamp: i64,
}

const DATABASE: &'static str = "azuresdktestdb";
const COLLECTION: &'static str = "azuresdktc";

fn main() {
    code().unwrap();
}

// This code will perform these tasks:
// 1. Find an Azure Cosmos DB called *DATABASE*. If it does not exist, create it.
// 2. Find an Azure Cosmos collection called *COLLECTION* in *DATABASE*.
//      If it does not exist, create it.
// 3. Store an entry in collection *COLLECTION* of database *DATABASE*.
// 4. Delete everything.
//
// We will use multiple futures for this hoping to make the code clearer.
// There is no need to proceed this way in your code.
// You can go crazy with future combinators if you want to :)
fn code() -> Result<(), Box<Error>> {
    // Let's get Cosmos account and master key from env variables.
    // This helps automated testing.
    let master_key =
        std::env::var("COSMOSDB_MASTER_KEY").expect("Set env variable COSMOS_MASTER_KEY first!");
    let account =
        std::env::var("COSMOSDB_ACCOUNT").expect("Set env variable COSMOS_ACCOUNT first!");

    // First, we create an authorization token. There are two types of tokens, master and resource
    // constrained. Please check the Azure documentation for details. You can change tokens
    // at will and it's a good practice to raise your privileges only when needed.
    let authorization_token = AuthorizationToken::new(account, TokenType::Master, &master_key)?;

    // We will create a tokio-core reactor which will drive our futures.
    let mut core = Core::new()?;

    // Next we will create a Cosmos client. You need an authorization_token but you can later
    // change it if needed. Notice the client will be tied to your reactor.
    let client = Client::new(authorization_token)?;

    // list_databases will give us the databases available in our account. If there is
    // an error (for example, the given key is not valid) you will receive a
    // specific AzureError. In this example we will look for a specific database
    // so we chain a filter operation.
    let future = client
        .list_databases()
        .and_then(|databases| ok(databases.into_iter().find(|db| db.id == DATABASE)));

    // Now we run the future and check the answer. If the requested database
    // is not found we create it.
    let database = match core.run(future)? {
        Some(db) => db,
        None => core.run(client.create_database(DATABASE))?,
    };
    println!("database == {:?}", database);

    // Now we look for a specific collection. If is not already present
    // we will create it. The collection creation is more complex and
    // has many options (such as indexing and so on).
    let collection = {
        let collections = core.run(client.list_collections(&DATABASE))?;

        if let Some(collection) = collections.into_iter().find(|coll| coll.id == COLLECTION) {
            collection
        } else {
            let indexes = cosmos::collection::IncludedPathIndex {
                kind: cosmos::collection::KeyKind::Hash,
                data_type: cosmos::collection::DataType::String,
                precision: Some(3),
            };

            let ip = cosmos::collection::IncludedPath {
                path: "/*".to_owned(),
                indexes: vec![indexes],
            };

            let ip = cosmos::collection::IndexingPolicy {
                automatic: true,
                indexing_mode: cosmos::collection::IndexingMode::Consistent,
                included_paths: vec![ip],
                excluded_paths: vec![],
            };

            let coll = cosmos::collection::Collection::new(COLLECTION, ip);
            // Notice here we specify the expected performance level.
            // Performance levels have price impact. Also, higher
            // performance levels force you to specify an indexing
            // strategy. Consult the documentation for more details.
            core.run(client.create_collection(&DATABASE, 400, &coll))?
        }
    };

    println!("collection = {:?}", collection);

    // Now that we have a database and a collection we can insert
    // data in them. Let's create a struct. The only constraint
    // is that the struct should be Serializable.
    let doc = MySampleStruct {
        id: "unique_id1",
        a_string: "Something here",
        a_number: 100,
        a_timestamp: chrono::Utc::now().timestamp(),
    };

    // Now we store the struct in Azure Cosmos DB.
    // Notice how easy it is! :)
    // The method create_document will return, upon success,
    // the document attributes.
    let document_attributes = core.run(
        client
            .create_document(&DATABASE, &COLLECTION, &doc)
            .execute(),
    )?;
    println!("document_attributes == {:?}", document_attributes);

    // We will perform some cleanup. First we delete the collection...
    core.run(client.delete_collection(DATABASE, &COLLECTION))?;
    println!("collection deleted");

    // And then we delete the database.
    core.run(client.delete_database(DATABASE))?;
    println!("database deleted");

    Ok(())
}
```

## State of the art
Right now the key framework is in place (authentication, enumerations, parsing and so on). If you want to contribute please do!
Methods are added daily so please check the [release page](https://github.com/MindFlavor/AzureSDKForRust/releases) for updates on the progress.
Also note that the project is in early stages so the APIs are bound to change at any moment. I will strive to keep things steady but since I'm new to Rust I'm sure I'll have to correct some serious mistake before too long :smile:.
I generally build for the latest nightly and leave to Travis to check the retrocompatibility.

## Contributing
If you want to contribute please do! No formality required! :wink:. Please note that asking for a pull request you accept to yield your code as per [Apache license, version 2.0](LICENSE).

### Implemented methods

#### Storage Container

| Method           | URL                                                                                                                                                                      | Builder pattern
| ----             | ---                                                                                                                                                                      | ---
| Create container | [https://docs.microsoft.com/en-us/rest/api/storageservices/create-container](https://docs.microsoft.com/en-us/rest/api/storageservices/create-container)                 | yes
| List containers  | [https://docs.microsoft.com/en-us/rest/api/storageservices/list-containers2](https://docs.microsoft.com/en-us/rest/api/storageservices/list-containers2)                 | yes
| Delete container | [https://docs.microsoft.com/en-us/rest/api/storageservices/delete-container](https://docs.microsoft.com/en-us/rest/api/storageservices/delete-container)                 | yes
| Get ACLs         | [https://docs.microsoft.com/en-us/rest/api/storageservices/get-container-acl](https://docs.microsoft.com/en-us/rest/api/storageservices/get-container-acl)               | yes
| Set ACLs         | [https://docs.microsoft.com/en-us/rest/api/storageservices/set-container-acl](https://docs.microsoft.com/en-us/rest/api/storageservices/set-container-acl)               | yes
| Get properties   | [https://docs.microsoft.com/en-us/rest/api/storageservices/get-container-properties](https://docs.microsoft.com/en-us/rest/api/storageservices/get-container-properties) | yes
| Acquire lease    | [https://docs.microsoft.com/en-us/rest/api/storageservices/lease-container](https://docs.microsoft.com/en-us/rest/api/storageservices/lease-container)                   | yes
| Break lease      | [https://docs.microsoft.com/en-us/rest/api/storageservices/lease-container](https://docs.microsoft.com/en-us/rest/api/storageservices/lease-container)                   | yes
| Release lease    | [https://docs.microsoft.com/en-us/rest/api/storageservices/lease-container](https://docs.microsoft.com/en-us/rest/api/storageservices/lease-container)                   | yes
| Renew lease      | [https://docs.microsoft.com/en-us/rest/api/storageservices/lease-container](https://docs.microsoft.com/en-us/rest/api/storageservices/lease-container)                   | yes

#### Storage blobs

| Method             | URL                                                                                                                                                  | Builder pattern
| ----               | ---                                                                                                                                                  | ---
| List blobs         | [https://docs.microsoft.com/en-us/rest/api/storageservices/list-blobs](https://docs.microsoft.com/en-us/rest/api/storageservices/list-blobs)         | yes
| Get blob           | [https://docs.microsoft.com/en-us/rest/api/storageservices/get-blob](https://docs.microsoft.com/en-us/rest/api/storageservices/get-blob)             | yes
| Put block blob     | [https://docs.microsoft.com/en-us/rest/api/storageservices/put-blob](https://docs.microsoft.com/en-us/rest/api/storageservices/put-blob)             | yes
| Put page blob      | [https://docs.microsoft.com/en-us/rest/api/storageservices/put-blob](https://docs.microsoft.com/en-us/rest/api/storageservices/put-blob)             | yes
| Put append blob    | [https://docs.microsoft.com/en-us/rest/api/storageservices/put-blob](https://docs.microsoft.com/en-us/rest/api/storageservices/put-blob)             | yes
| Put blob page      | [https://docs.microsoft.com/en-us/rest/api/storageservices/put-page](https://docs.microsoft.com/en-us/rest/api/storageservices/put-page)             | yes (as `update_page`)
| Clear blob page    | [https://docs.microsoft.com/en-us/rest/api/storageservices/put-page](https://docs.microsoft.com/en-us/rest/api/storageservices/put-page)             | yes (as `clear_page`)
| Put block          | [https://docs.microsoft.com/en-us/rest/api/storageservices/put-block](https://docs.microsoft.com/en-us/rest/api/storageservices/put-block)           | yes
| Get block list     | [https://docs.microsoft.com/en-us/rest/api/storageservices/get-block-list](https://docs.microsoft.com/en-us/rest/api/storageservices/get-block-list) | yes
| Put block list     | [https://docs.microsoft.com/en-us/rest/api/storageservices/put-block-list](https://docs.microsoft.com/en-us/rest/api/storageservices/put-block-list) | yes
| Acquire blob lease | [https://docs.microsoft.com/en-us/rest/api/storageservices/lease-blob](https://docs.microsoft.com/en-us/rest/api/storageservices/lease-blob)         | yes
| Renew blob lease   | [https://docs.microsoft.com/en-us/rest/api/storageservices/lease-blob](https://docs.microsoft.com/en-us/rest/api/storageservices/lease-blob)         | yes
| Change blob lease  | [https://docs.microsoft.com/en-us/rest/api/storageservices/lease-blob](https://docs.microsoft.com/en-us/rest/api/storageservices/lease-blob)         | yes
| Release blob lease | [https://docs.microsoft.com/en-us/rest/api/storageservices/lease-blob](https://docs.microsoft.com/en-us/rest/api/storageservices/lease-blob)         | yes
| Break blob lease   | [https://docs.microsoft.com/en-us/rest/api/storageservices/lease-blob](https://docs.microsoft.com/en-us/rest/api/storageservices/lease-blob)         | yes
| Delete blob        | [https://docs.microsoft.com/en-us/rest/api/storageservices/delete-blob](https://docs.microsoft.com/en-us/rest/api/storageservices/delete-blob)       |

#### Event Hubs

| Method     | URL                                                                                                                          |
| ----       | ---                                                                                                                          |
| Send Event | [https://msdn.microsoft.com/en-us/library/azure/dn790664.aspx](https://msdn.microsoft.com/en-us/library/azure/dn790664.aspx) |

#### Cosmos database

##### Database

| Method          | URL                                                                                                                                                |
| ----            | ---                                                                                                                                                |
| Create database | [https://docs.microsoft.com/en-us/rest/api/documentdb/create-a-database](https://docs.microsoft.com/en-us/rest/api/documentdb/create-a-database)   |
| List database   | [https://docs.microsoft.com/en-us/rest/api/documentdb/list-databases](https://docs.microsoft.com/en-us/rest/api/documentdb/list-databases)         |
| Get database    | [https://docs.microsoft.com/en-us/rest/api/documentdb/get-a-database](https://docs.microsoft.com/en-us/rest/api/documentdb/get-a-database)         |
| Delete database | [https://docs.microsoft.com/en-us/rest/api/documentdb/delete-a-database1](https://docs.microsoft.com/en-us/rest/api/documentdb/delete-a-database1) |

##### Collection

| Method             | URL                                                                                                                                                    |
| ----               | ---                                                                                                                                                    |
| Create collection  | [https://docs.microsoft.com/en-us/rest/api/documentdb/create-a-collection](https://docs.microsoft.com/en-us/rest/api/documentdb/create-a-collection)   |
| List collections   | [https://docs.microsoft.com/en-us/rest/api/documentdb/list-collections](https://docs.microsoft.com/en-us/rest/api/documentdb/list-collections)         |
| Get collection     | [https://docs.microsoft.com/en-us/rest/api/documentdb/get-a-collection](https://docs.microsoft.com/en-us/rest/api/documentdb/get-a-collection)         |
| Delete collection  | [https://docs.microsoft.com/en-us/rest/api/documentdb/delete-a-collection](https://docs.microsoft.com/en-us/rest/api/documentdb/delete-a-collection)   |
| Replace collection | [https://docs.microsoft.com/en-us/rest/api/documentdb/replace-a-collection](https://docs.microsoft.com/en-us/rest/api/documentdb/replace-a-collection) |

##### Document

| Method             | URL                                                                                                                                                    |
| ----               | ---                                                                                                                                                    |
| Create document    | [https://docs.microsoft.com/en-us/rest/api/documentdb/create-a-document](https://docs.microsoft.com/en-us/rest/api/documentdb/create-a-document)       |
| List documents     | [https://docs.microsoft.com/en-us/rest/api/documentdb/list-documents](https://docs.microsoft.com/en-us/rest/api/documentdb/list-documents)             |
| Get document       | [https://docs.microsoft.com/en-us/rest/api/documentdb/get-a-document](https://docs.microsoft.com/en-us/rest/api/documentdb/get-a-document)             |
| Query documents    | [https://docs.microsoft.com/en-us/rest/api/documentdb/query-documents](https://docs.microsoft.com/en-us/rest/api/documentdb/query-documents)	      |
| Replace document   | [https://docs.microsoft.com/en-us/rest/api/cosmos-db/replace-a-document](https://docs.microsoft.com/en-us/rest/api/cosmos-db/replace-a-document)	      |
| Delete document    | [https://docs.microsoft.com/en-us/rest/api/cosmos-db/delete-a-document](https://docs.microsoft.com/en-us/rest/api/cosmos-db/delete-a-document)	      |

#### Azure tables

| Method         | URL                                                                                                                                                  |
| ----           | ---                                                                                                                                                  |
| Create table   | [https://docs.microsoft.com/en-us/rest/api/storageservices/create-table](https://docs.microsoft.com/en-us/rest/api/storageservices/create-table)     |
| Query tables   | [https://docs.microsoft.com/en-us/rest/api/storageservices/query-tables](https://docs.microsoft.com/en-us/rest/api/storageservices/query-tables)     |
| Query entities | [https://docs.microsoft.com/en-us/rest/api/storageservices/query-entities](https://docs.microsoft.com/en-us/rest/api/storageservices/query-entities) |
| Insert entity  | [https://docs.microsoft.com/en-us/rest/api/storageservices/insert-entity](https://docs.microsoft.com/en-us/rest/api/storageservices/insert-entity)   |
| Update entity  | [https://docs.microsoft.com/en-us/rest/api/storageservices/update-entity2](https://docs.microsoft.com/en-us/rest/api/storageservices/update-entity2) |
| Delete entity  | [https://docs.microsoft.com/en-us/rest/api/storageservices/delete-entity1](https://docs.microsoft.com/en-us/rest/api/storageservices/delete-entity1) |

Azure tables entities can be manipulated in batches. The entities are serialized in ```JSON```.

## Run E2E test 

### Linux 

```bash
export STORAGE_ACCOUNT=<account>
export STORAGE_MASTER_KEY=<key>

export AZURE_SERVICE_BUS_NAMESPACE=<azure_service_bus_namespace>
export AZURE_EVENT_HUB_NAME=<azure_event_hub_name>
export AZURE_POLICY_NAME=<azure_policy_name>
export AZURE_POLICY_KEY=<azure policy key>

cargo test --features=test_e2e
```

### Windows

```bat
set STORAGE_ACCOUNT=<account>
set STORAGE_MASTER_KEY=<key>

set AZURE_SERVICE_BUS_NAMESPACE=<azure_service_bus_namespace>
set AZURE_EVENT_HUB_NAME=<azure_event_hub_name>
set AZURE_POLICY_NAME=<azure_policy_name>
set AZURE_POLICY_KEY=<azure policy key>

cargo test --features=test_e2e
```

## License
This project is published under [Apache license, version 2.0](LICENSE).

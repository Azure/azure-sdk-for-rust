extern crate azure_sdk_for_rust;

extern crate futures;
extern crate tokio_core;
extern crate tokio;
extern crate hyper;
extern crate hyper_tls;
extern crate chrono;

use std::error::Error;

use futures::future::*;
use tokio_core::reactor::Core;

use azure_sdk_for_rust::azure::cosmos::authorization_token::{AuthorizationToken, TokenType};
use azure_sdk_for_rust::azure::cosmos::client::Client;

#[macro_use]
extern crate serde_derive;
use azure_sdk_for_rust::azure::cosmos;

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

fn code() -> Result<(), Box<Error>> {
    let master_key = std::env::var("COSMOS_MASTER_KEY")
        .expect("Set env variable COSMOS_MASTER_KEY first!");
    let account = std::env::var("COSMOS_ACCOUNT").expect("Set env variable COSMOS_ACCOUNT first!");
    let authorization_token = AuthorizationToken::new(account, TokenType::Master, master_key)?;

    let mut core = Core::new()?;
    let client = Client::new(&core.handle(), authorization_token)?;

    let future = client.list_databases().and_then(|databases| {
        ok(databases.into_iter().find(|db| db.id == DATABASE))
    });

    let database = match core.run(future)? {
        Some(db) => db,
        None => core.run(client.create_database(DATABASE))?,
    };
    println!("database == {:?}", database);

    let collection = {
        let collections = core.run(client.list_collections(&database))?;

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
            core.run(client.create_collection(&database, 400, &coll))?
        }
    };

    println!("collection = {:?}", collection);

    let doc = MySampleStruct {
        id: "unique_id1",
        a_string: "Something here",
        a_number: 100,
        a_timestamp: chrono::Utc::now().timestamp(),
    };

    let document_attributes = core.run(
        client.create_document(&database, &collection, false, None, &doc),
    )?;
    println!("document_attributes == {:?}", document_attributes);

    core.run(client.delete_collection(DATABASE, COLLECTION))?;
    println!("collection deleted");
    core.run(client.delete_database(DATABASE))?;
    println!("database deleted");

    Ok(())
}

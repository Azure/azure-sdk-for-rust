extern crate azure_sdk_for_rust;
extern crate chrono;
extern crate serde;
extern crate serde_json;

#[macro_use]
extern crate serde_derive;
use std::error::Error;

use azure_sdk_for_rust::azure::cosmos::authorization_token::{AuthorizationToken, TokenType};
use azure_sdk_for_rust::azure::cosmos::client::Client;

use azure_sdk_for_rust::azure::cosmos;

//use chrono::{DateTime, UTC};

//use serde::Serialize;

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
    let authorization_token = AuthorizationToken::new(&account, TokenType::Master, master_key)?;
    let client = Client::new(&authorization_token)?;

    let database = {
        let dbs = client.list_databases()?;
        if let Some(db) = dbs.into_iter().find(|db| db.id == DATABASE) {
            db
        } else {
            client.create_database(DATABASE)?
        }
    };
    println!("database == {:?}", database);

    let coll = {
        let colls = client.list_collections(&database)?;
        if let Some(coll) = colls.into_iter().find(|coll| coll.id == COLLECTION) {
            coll
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
            client.create_collection(DATABASE, 400, &coll)?
        }
    };
    println!("collection == {:?}", coll);

    let doc = MySampleStruct {
        id: "unique_id1",
        a_string: "Something here",
        a_number: 100,
        a_timestamp: chrono::UTC::now().timestamp(),
    };

    let document_attributes = client.create_document(&database, &coll, false, None, &doc)?;
    println!("document_attributes == {:?}", document_attributes);

    client.delete_collection(DATABASE, COLLECTION)?;
    println!("collection deleted");
    client.delete_database(DATABASE)?;
    println!("database deleted");

    Ok(())
}

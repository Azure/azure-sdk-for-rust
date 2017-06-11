extern crate azure_sdk_for_rust;

use std::error::Error;

use azure_sdk_for_rust::azure::cosmos::authorization_token::{AuthorizationToken, TokenType};
use azure_sdk_for_rust::azure::cosmos::client::Client;

use azure_sdk_for_rust::azure::cosmos;

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

    client.create_database(DATABASE)?;
    println!("database created");

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

    client.create_collection(DATABASE, 400, &coll)?;
    println!("collection created");


    client.delete_collection(DATABASE, COLLECTION)?;
    println!("collection deleted");
    client.delete_database(DATABASE)?;
    println!("database deleted");

    Ok(())
}

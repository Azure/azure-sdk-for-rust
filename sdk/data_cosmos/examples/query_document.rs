//! This example illustrates how querying for documents works.
//!
//! You can test this with any database collection that has an item that roughly has this shape:
//! ```json
//! {
//!     "id": "AndersenFamily",
//!     "lastName": "Andersen",
//!     "parents": [
//!        { "firstName": "Thomas" },
//!        { "firstName": "Mary Kay"}
//!     ],
//!     "children": [ { "firstName": "Henriette Thaulow" } ],
//! }
//! ```
//!
//! You can then query for families using a query like: "SELECT * FROM Families"
use azure_data_cosmos::prelude::*;
use clap::Parser;
use futures::StreamExt;
use serde::{Deserialize, Serialize};

#[derive(Debug, Parser)]
struct Args {
    /// Cosmos primary key name
    #[clap(env = "COSMOS_PRIMARY_KEY")]
    primary_key: String,
    /// The cosmos account your're using
    #[clap(env = "COSMOS_ACCOUNT")]
    account: String,
    /// The name of the database
    database_name: String,
    /// The name of the collection
    collection_name: String,
    /// The cosmos query you're trying to execute
    query: String,
}

#[derive(Serialize, Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
struct Family {
    id: String,
    last_name: String,
    parents: Vec<Person>,
    children: Vec<Person>,
}

#[derive(Serialize, Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
struct Person {
    first_name: String,
}

#[tokio::main]
async fn main() -> azure_core::Result<()> {
    let args = Args::parse();
    let authorization_token = AuthorizationToken::primary_from_base64(&args.primary_key)?;

    let client = CosmosClient::new(args.account, authorization_token)
        .database_client(args.database_name)
        .collection_client(args.collection_name);

    let query_obj = Query::new(args.query);

    let query = client
        .query_documents(query_obj.clone())
        .query_cross_partition(true)
        .max_item_count(1); // This is way lower than necessary but easily allows demonstratiting paging results.

    // First, we'll look at the results as JSON.
    let mut stream = query.clone().into_stream::<serde_json::Value>();
    while let Some(respo) = stream.next().await {
        println!("JSON: {:#?}", respo?.results);
    }

    // Then, we'll look at the results as `Family` structs.
    let mut stream = query.into_stream::<Family>();
    while let Some(respo) = stream.next().await {
        println!("Structs: {:#?}", respo?.results);
    }

    Ok(())
}

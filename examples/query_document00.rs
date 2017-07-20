extern crate azure_sdk_for_rust;

extern crate futures;
extern crate tokio_core;
extern crate tokio;
extern crate hyper;
extern crate hyper_tls;
extern crate chrono;

use std::error::Error;

use tokio_core::reactor::Core;

use azure_sdk_for_rust::azure::cosmos::authorization_token::{AuthorizationToken, TokenType};
use azure_sdk_for_rust::azure::cosmos::client::Client;
use azure_sdk_for_rust::azure::cosmos::query_document::QueryDocumentOptions;
use azure_sdk_for_rust::azure::cosmos::query::Query;

#[macro_use]
extern crate serde_derive;

#[derive(Serialize, Deserialize, Debug)]
struct MySampleStructOwned {
    id: String,
    a_string: String,
    a_number: u64,
    a_timestamp: i64,
}

fn main() {
    code().unwrap();
}

fn code() -> Result<(), Box<Error>> {
    let database_name = std::env::args().nth(1).expect(
        "please specify database name as first command line parameter",
    );
    let collection_name = std::env::args().nth(2).expect(
        "please specify collection name as second command line parameter",
    );
    let query = std::env::args()
        .nth(3)
        .expect("please specify requested query");

    let account = std::env::var("COSMOS_ACCOUNT").expect("Set env variable COSMOS_ACCOUNT first!");
    let master_key =
        std::env::var("COSMOS_MASTER_KEY").expect("Set env variable COSMOS_MASTER_KEY first!");

    let authorization_token = AuthorizationToken::new(account, TokenType::Master, master_key)?;

    let mut core = Core::new()?;

    let client = Client::new(&core.handle(), authorization_token)?;

    let options = QueryDocumentOptions::new();

    let future = client.query_document_json(
        &database_name,
        &collection_name,
        &Query::from(&query as &str),
        &options,
    );

    let ret = core.run(future)?;

    println!("As JSON:\n{:?}", ret);

    for doc in ret.results.into_iter() {
        println!("{}", doc.result);
    }

    let future = client.query_document::<_, _, MySampleStructOwned>(
        &database_name,
        &collection_name,
        &Query::from(&query as &str),
        &options,
    );

    let ret = core.run(future)?;

    println!("\nAs entities:\n{:?}", ret);

    for doc in ret.results.into_iter() {
        println!("{:?}", doc.result);
    }

    // test continuation token
    // only if we have more than 2 records
    let mut options = QueryDocumentOptions::new();
    options.max_item_count = Some(2);

    let future = client.query_document::<_, _, MySampleStructOwned>(
        &database_name,
        &collection_name,
        &Query::from(&query as &str),
        &options,
    );

    let ret = core.run(future)?;

    println!(
        "Received {} entries. Continuation token is == {:?}",
        ret.results.len(),
        ret.additional_headers.continuation_token
    );

    if let Some(ct) = ret.additional_headers.continuation_token {
        let ret = {
            // if we have more, let's get them
            let mut options = QueryDocumentOptions::new();
            options.max_item_count = None;
            options.continuation_token = Some(&ct);

            let future = client.query_document::<_, _, MySampleStructOwned>(
                &database_name,
                &collection_name,
                &Query::from(&query as &str),
                &options,
            );
            core.run(future)?
        };
        println!(
            "Received {} entries. Continuation token is == {:?}",
            ret.results.len(),
            ret.additional_headers.continuation_token
        );
    }

    Ok(())
}

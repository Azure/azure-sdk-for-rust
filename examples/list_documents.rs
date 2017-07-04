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
use azure_sdk_for_rust::azure::cosmos::list_documents::LIST_DOCUMENTS_OPTIONS_DEFAULT;

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

    let master_key = std::env::var("COSMOS_MASTER_KEY")
        .expect("Set env variable COSMOS_MASTER_KEY first!");
    let account = std::env::var("COSMOS_ACCOUNT").expect("Set env variable COSMOS_ACCOUNT first!");

    let authorization_token = AuthorizationToken::new(account, TokenType::Master, master_key)?;

    let mut core = Core::new()?;

    let client = Client::new(&core.handle(), authorization_token)?;



    let (body, ldah) = core.run(client.list_documents(
        database_name,
        collection_name,
        &LIST_DOCUMENTS_OPTIONS_DEFAULT,
    )).unwrap();

    println!("body == {:?}, ldah = {:?}", body, ldah);

    Ok(())
}

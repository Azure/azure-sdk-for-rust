/// This sample showcases execution of stored procedure
/// Create stored procedure called test_proc, like so:
/// function f(personToGreet) {
///     var context = getContext();
///     var response = context.getResponse();
///     response.setBody("Hello, " + personToGreet);
/// }
extern crate azure_sdk_for_rust;
extern crate chrono;
extern crate futures;
extern crate tokio_core;
#[macro_use]
extern crate serde_derive;
#[macro_use]
extern crate serde_json;

use azure_sdk_for_rust::cosmos::{AuthorizationToken, Client, TokenType};
use std::error::Error;
use tokio_core::reactor::Core;

fn main() {
    code().unwrap();
}

fn code() -> Result<(), Box<Error>> {
    let database = std::env::args()
        .nth(1)
        .expect("please specify database name as first command line parameter");
    let collection = std::env::args()
        .nth(2)
        .expect("please specify collection name as second command line parameter");

    let account = std::env::var("COSMOS_ACCOUNT").expect("Set env variable COSMOS_ACCOUNT first!");
    let master_key = std::env::var("COSMOS_MASTER_KEY").expect("Set env variable COSMOS_MASTER_KEY first!");

    let authorization_token = AuthorizationToken::new(account, TokenType::Master, &master_key)?;

    let mut core = Core::new()?;
    let client = Client::new(authorization_token)?;

    let future = client
        .execute_stored_procedure(database, collection, "test_proc", json!(["Robert"]))
        .execute::<serde_json::Value>();

    let ret = core.run(future)?;

    println!("Response object:\n{:#?}", ret);
    println!("Response as JSON:\n{}", ret.result.to_string());

    Ok(())
}

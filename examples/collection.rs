extern crate azure_sdk_for_rust;

extern crate futures;
extern crate tokio_core;
extern crate tokio;
extern crate hyper;
extern crate hyper_tls;

use std::error::Error;

use futures::future::*;
use tokio_core::reactor::Core;

use azure_sdk_for_rust::azure::cosmos::authorization_token::{AuthorizationToken, TokenType};
use azure_sdk_for_rust::azure::cosmos::client::{Client, list_databases};


fn main() {
    let mut core = Core::new().unwrap();

    code(&mut core).unwrap();
}

// We run a separate method to use the elegant quotation mark operator.
// A series of unwrap(), unwrap() would have achieved the same result.
fn code(core: &mut Core) -> Result<(), Box<Error>> {
    // First we retrieve the account name and master key from environment variables.
    // We expect master keys (ie, not resource constrained)
    let master_key =
        std::env::var("COSMOS_MASTER_KEY").expect("Set env variable COSMOS_MASTER_KEY first!");
    let account = std::env::var("COSMOS_ACCOUNT").expect("Set env variable COSMOS_ACCOUNT first!");

    // This is how you construct an authorization token.
    // Remeber to pick the correct token type.
    // Here we assume master.
    // Most methods return a ```Result<_, AzureError>```.
    // ```AzureError``` is an enum union of all the possibile undelying
    // errors, plus Azure specific ones. For example if a REST call returns the
    // unexpected result (ie NotFound instead of Ok) we return a Err telling
    // you that.
    let authorization_token = AuthorizationToken::new(account, TokenType::Master, master_key)?;

    // Once we have an authorization token you can create a client instance. You can change the
    // authorization token at later time if you need, for example, to escalate the privileges for a
    // single operation.
    let client = Client::new(&core.handle(), authorization_token)?;

    // The Cosmos' client exposes a lot of methods. This one lists the databases in the specified
    // account. Database do not implement Display but defef to &str so you can pass it to methods
    // both as struct or id.

    //let client = hyper::Client::configure()
    //    .connector(hyper_tls::HttpsConnector::new(4, &core.handle())?)
    //    .build(&core.handle());


    //let future = list_databases(&client, &authorization_token).map(|res| {
    //    println!("{:?}", res);
    //});

    let future = client.list_databases().map(|res| {
        println!("{:?}", res);
    });

    core.run(future)?;

    //println!("Account {} has {} databases", account, databases.len());

    //// Each Cosmos' database contains zero or more collections. We can enumerate them using the
    //// list_collection method.
    //for db in &databases {
    //    let collections = client.list_collections(db)?;
    //    println!("*** {} *** ({} collections)", db as &str, collections.len());
    //    for coll in &collections {
    //        // Collection does not implement Display but Deref to &str so this print works as
    //        // expected.
    //        println!("\t{}", coll as &str);
    //    }
    //}
    Ok(())
}

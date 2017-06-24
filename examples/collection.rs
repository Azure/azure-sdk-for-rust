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
use azure_sdk_for_rust::azure::cosmos::client::Client;


fn main() {
    code().unwrap();
}


// We run a separate method to use the elegant quotation mark operator.
// A series of unwrap(), unwrap() would have achieved the same result.
fn code() -> Result<(), Box<Error>> {
    // First we retrieve the account name and master key from environment variables.
    // We expect master keys (ie, not resource constrained)
    let master_key =
        std::env::var("COSMOS_MASTER_KEY").expect("Set env variable COSMOS_MASTER_KEY first!");
    let account = std::env::var("COSMOS_ACCOUNT").expect("Set env variable COSMOS_ACCOUNT first!");

    // let's create a tokio-core reactor.
    // It will drive our request. Remember, util run, futures do absolutely
    // nothing. So, run them. Also note that, in order to avoid cloning the authorization_token at
    // each request this library constructs the request **before** the future. This means the date
    // sent to the server will be the one at Future creation time, not the execution time.
    // Azure calls will block requests with time too much in the past (in order to prevent reply
    // attacks) so make sure to execute the Future as soon as possibile after having it created.
    // * This is something worth discussing *
    let mut core = Core::new()?;

    // This is how you construct an authorization token.
    // Remeber to pick the correct token type.
    // Here we assume master.
    // Most methods return a ```Result<_, AzureError>```.
    // ```AzureError``` is an enum union of all the possibile undelying
    // errors, plus Azure specific ones. For example if a REST call returns the
    // unexpected result (ie NotFound instead of Ok) we return a Err telling
    // you that.
    let authorization_token =
        AuthorizationToken::new(account.clone(), TokenType::Master, master_key)?;

    // Once we have an authorization token you can create a client instance. You can change the
    // authorization token at later time if you need, for example, to escalate the privileges for a
    // single operation.
    let client = Client::new(&core.handle(), authorization_token)?;

    // The Cosmos' client exposes a lot of methods. This one lists the databases in the specified
    // account. Database do not implement Display but defef to &str so you can pass it to methods
    // both as struct or id.
    let future = client.list_databases().map(move |databases| {
        println!("Account {} has {} databases", account, databases.len());
    });

    core.run(future)?;

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

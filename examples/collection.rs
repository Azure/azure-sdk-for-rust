extern crate azure_sdk_for_rust;

extern crate futures;
extern crate hyper;
extern crate hyper_tls;
extern crate tokio_core;

use std::error::Error;

use futures::future::*;
use tokio_core::reactor::Core;

use azure_sdk_for_rust::cosmos::{AuthorizationToken, Client, TokenType};

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
    // It will drive our request. Remember, until run, futures do absolutely
    // nothing. So, run them. Also note that, in order to avoid cloning the authorization_token at
    // each request this library constructs the request **before** the future. This means the date
    // sent to the server will be the one at Future creation time, not the execution time.
    // Azure calls will block requests with time too much in the past (in order to prevent reply
    // attacks) so make sure to execute the Future as soon as possible after having it created.
    // * This is something worth discussing *
    let mut core = Core::new()?;

    // This is how you construct an authorization token.
    // Remember to pick the correct token type.
    // Here we assume master.
    // Most methods return a ```Result<_, AzureError>```.
    // ```AzureError``` is an enum union of all the possible underlying
    // errors, plus Azure specific ones. For example if a REST call returns the
    // unexpected result (ie NotFound instead of Ok) we return an Err telling
    // you that.
    let authorization_token =
        AuthorizationToken::new(account.clone(), TokenType::Master, &master_key)?;

    // Once we have an authorization token you can create a client instance. You can change the
    // authorization token at later time if you need, for example, to escalate the privileges for a
    // single operation.
    let client = Client::new(authorization_token)?;

    // The Cosmos' client exposes a lot of methods. This one lists the databases in the specified
    // account. Database do not implement Display but deref to &str so you can pass it to methods
    // both as struct or id.
    let future = client.list_databases().and_then(move |databases| {
        println!("Account {} has {} database(s)", account, databases.len());

        let mut v = Vec::new();

        // Each Cosmos' database contains so or more collections. We can enumerate them using the
        // list_collection method.
        for db in databases {
            v.push(client.list_collections(&db.id).map(move |collections| {
                println!("database {} has {} collection(s)", db.id, collections.len());

                for collection in collections {
                    println!("\tcollection {}", collection.id);
                }
            }));
        }

        futures::future::join_all(v)
    });

    core.run(future)?;
    Ok(())
}

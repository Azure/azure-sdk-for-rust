extern crate azure_sdk_for_rust;

extern crate env_logger;
extern crate futures;
extern crate hyper;
extern crate hyper_tls;
#[macro_use]
extern crate log;
extern crate tokio_core;

use std::error::Error;

use futures::future::*;
use tokio_core::reactor::Core;

use azure_sdk_for_rust::storage::{blob::Blob, client::Client};

fn main() {
    env_logger::init();
    code().unwrap();
}

// We run a separate method to use the elegant quotation mark operator.
// A series of unwrap(), unwrap() would have achieved the same result.
fn code() -> Result<(), Box<Error>> {
    // First we retrieve the account name and master key from environment variables.
    let account =
        std::env::var("STORAGE_ACCOUNT").expect("Set env variable STORAGE_ACCOUNT first!");
    let master_key =
        std::env::var("STORAGE_MASTER_KEY").expect("Set env variable STORAGE_MASTER_KEY first!");

    let container = std::env::args()
        .nth(1)
        .expect("please specify container name as command line parameter");
    let blob = std::env::args()
        .nth(2)
        .expect("please specify blob name as command line parameter");

    let mut core = Core::new()?;

    let client = Client::new(&account, &master_key)?;

    trace!("Requesting blog");

    let future =
        Blob::get(&client, &container, &blob, None, None, None).and_then(move |(blob, content)| {
            done(String::from_utf8(content))
                .map(move |s_content| {
                    println!("blob == {:?}", blob);
                    println!("s_content == {}", s_content);
                })
                .from_err()
        });
    core.run(future)?;

    Ok(())
}

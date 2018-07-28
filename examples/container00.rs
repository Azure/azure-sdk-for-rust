extern crate azure_sdk_for_rust;
extern crate futures;
extern crate hyper;
extern crate hyper_tls;
extern crate tokio_core;

use azure_sdk_for_rust::core::ContainerNameSupport;
use azure_sdk_for_rust::storage::{client::Blob as BlobTrait, client::Client};
use futures::future::*;
use std::error::Error;
use tokio_core::reactor::Core;

fn main() {
    code().unwrap();
}

// We run a separate method to use the elegant quotation mark operator.
// A series of unwrap(), unwrap() would have achieved the same result.
fn code() -> Result<(), Box<Error>> {
    // First we retrieve the account name and master key from environment variables.
    let account = std::env::var("STORAGE_ACCOUNT").expect("Set env variable STORAGE_ACCOUNT first!");
    let master_key = std::env::var("STORAGE_MASTER_KEY").expect("Set env variable STORAGE_MASTER_KEY first!");

    let container = std::env::args()
        .nth(1)
        .expect("please specify container name as command line parameter");

    let mut core = Core::new()?;

    let client = Client::new(&account, &master_key)?;

    let future = {
        use azure_sdk_for_rust::storage::client::Container;
        client.list_containers().finalize().map(|iv| {
            println!("List containers returned {} containers.", iv.incomplete_vector.len());
            for cont in iv.incomplete_vector.iter() {
                println!("\t{}", cont.name);
            }
        })
    };

    core.run(future)?;

    let future = client.list_blobs().with_container_name(&container).finalize().map(|iv| {
        println!("List blob returned {} blobs.", iv.incomplete_vector.len());
        for cont in iv.incomplete_vector.iter() {
            println!("\t{}\t{} MB", cont.name, cont.content_length / (1024 * 1024));
        }
    });

    core.run(future)?;

    Ok(())
}

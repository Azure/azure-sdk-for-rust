extern crate azure_sdk_for_rust;

extern crate futures;
extern crate hyper;
extern crate hyper_tls;
extern crate tokio_core;

use azure_sdk_for_rust::core::{ClientRequestIdSupport, ContainerNameSupport, TimeoutSupport};
use azure_sdk_for_rust::storage::{
    client::Client,
    container::{PublicAccess, PublicAccessSupport},
};
use futures::Future;
use std::collections::HashMap;
use std::error::Error;
use tokio_core::reactor::Core;

fn main() {
    code().unwrap();
}

// We run a separate method to use the elegant question mark operator.
// A series of unwrap(), unwrap() would have achieved the same result.
fn code() -> Result<(), Box<Error>> {
    // First we retrieve the account name and master key from environment variables.
    let account = std::env::var("STORAGE_ACCOUNT").expect("Set env variable STORAGE_ACCOUNT first!");
    let master_key = std::env::var("STORAGE_MASTER_KEY").expect("Set env variable STORAGE_MASTER_KEY first!");

    let container_name = std::env::args()
        .nth(1)
        .expect("please specify container name as command line parameter");

    let mut core = Core::new()?;

    let client = Client::new(&account, &master_key)?;

    use azure_sdk_for_rust::storage::client::Container;

    let future = client.list().with_client_request_id("ciccio").include_metadata().finalize();

    core.run(future.map(|res| {
        println!("{:?}", res);
    }))?;

    let mut metadata = HashMap::new();
    metadata.insert("prova", "pollo");
    metadata.insert("canotto", "cigno");

    // This is the builder pattern. Notice two things:
    // 1 - The various parameters are clearly defined.
    // 2 - If you forget a mandatory parameter the code won't compile. Type checking at compile
    //   time is waaay better than doing it at runtime!
    let future = client
        .create()
        .with_container_name(&container_name)
        .with_public_access(PublicAccess::Container)
        .with_metadata(metadata)
        .with_timeout(100)
        .finalize();

    core.run(future)?;

    let future = client.get_acl().with_container_name(&container_name).finalize();

    let result = core.run(future)?;
    assert!(result == PublicAccess::Container);

    let future = client.delete().with_container_name(&container_name).finalize();
    core.run(future).map(|_| {
        println!("container {} deleted!", container_name);
    })?;

    Ok(())
}

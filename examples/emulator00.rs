extern crate azure_sdk_for_rust;
extern crate chrono;
extern crate env_logger;
extern crate futures;
extern crate hyper;
extern crate hyper_tls;
extern crate log;
extern crate md5;
extern crate tokio_core;
extern crate url;

use azure_sdk_for_rust::prelude::*;
use azure_sdk_for_rust::storage::container::PublicAccess;
use futures::future::*;
use std::error::Error;
use tokio_core::reactor::Core;
use url::Url;

fn main() -> Result<(), Box<Error>> {
    env_logger::init();

    let mut core = Core::new()?;

    // this is how you use the emulator.
    let blob_storage_url = "http://127.0.0.1:10000";
    let table_storage_url = "http://127.0.0.1:10002";
    let client = Client::emulator(&Url::parse(blob_storage_url)?, &Url::parse(table_storage_url)?)?;

    // create container
    let future = client
        .create_container()
        .with_container_name("emulcont")
        .with_public_access(PublicAccess::None)
        .finalize();
    core.run(future.map(|res| println!("{:?}", res)))?;

    let future = client
        .list_blobs()
        .with_container_name("emulcont")
        .with_include_metadata()
        .finalize();
    core.run(future.map(|res| println!("{:?}", res)))?;

    Ok(())
}

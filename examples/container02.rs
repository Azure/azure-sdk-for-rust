extern crate azure_sdk_for_rust;
extern crate chrono;
extern crate env_logger;
extern crate futures;
extern crate hyper;
extern crate hyper_tls;
extern crate log;
extern crate md5;
extern crate tokio_core;

use azure_sdk_for_rust::prelude::*;
use azure_sdk_for_rust::storage::container::PublicAccess;
use futures::future::*;
use std::error::Error;
use tokio_core::reactor::Core;

fn main() {
    env_logger::init();
    code().unwrap();
}

// We run a separate method to use the elegant quotation mark operator.
// A series of unwrap(), unwrap() would have achieved the same result.
fn code() -> Result<(), Box<Error>> {
    let mut core = Core::new()?;

    // this will only work with the emulator
    let client = Client::new("", "")?;

    // create container
    let future = client
        .create_container()
        .with_container_name("emulcont")
        .with_public_access(PublicAccess::None)
        .finalize();
    core.run(future.map(|res| println!("{:?}", res)))?;

    //let data = b"something";

    //// this is not mandatory but it helps preventing
    //// spurious data to be uploaded.
    //let digest = md5::compute(&data[..]);

    //let future = client
    //    .put_block_blob()
    //    .with_container_name(&container_name)
    //    .with_blob_name("blob0.txt")
    //    .with_content_type("text/plain")
    //    .with_body(&data[..])
    //    .with_content_md5(&digest[..])
    //    .finalize();
    //core.run(future.map(|res| println!("{:?}", res)))?;

    //let future = client
    //    .put_block_blob()
    //    .with_container_name(&container_name)
    //    .with_blob_name("blob1.txt")
    //    .with_content_type("text/plain")
    //    .with_body(&data[..])
    //    .with_content_md5(&digest[..])
    //    .finalize();
    //core.run(future.map(|res| println!("{:?}", res)))?;

    //let future = client
    //    .put_block_blob()
    //    .with_container_name(&container_name)
    //    .with_blob_name("blob2.txt")
    //    .with_content_type("text/plain")
    //    .with_body(&data[..])
    //    .with_content_md5(&digest[..])
    //    .finalize();
    //core.run(future.map(|res| println!("{:?}", res)))?;

    //let future = client
    //    .list_blobs()
    //    .with_container_name(&container_name)
    //    .with_include_metadata()
    //    .finalize();
    //core.run(future.map(|res| println!("{:?}", res)))?;

    Ok(())
}

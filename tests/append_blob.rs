#![cfg(all(test, feature = "test_e2e"))]
extern crate azure_sdk_for_rust;
extern crate chrono;
extern crate env_logger;
extern crate futures;
extern crate hyper;
extern crate hyper_tls;
extern crate tokio_core;
#[macro_use]
extern crate log;
extern crate md5;
extern crate serde;
extern crate uuid;

use azure_sdk_for_rust::core::errors::AzureError;
use azure_sdk_for_rust::prelude::*;
use azure_sdk_for_rust::storage::container::{PublicAccess, PublicAccessSupport};
use std::collections::HashMap;
use tokio_core::reactor::Core;

#[test]
fn put_append_blob() {
    let (client, mut core) = initialize().unwrap();

    let blob_name: &'static str = "append_blob.txt";
    let container_name: &'static str = "rust-upload-test";
    let data = b"abcdef";

    if core
        .run(client.list_containers().finalize())
        .unwrap()
        .incomplete_vector
        .iter()
        .find(|x| x.name == container_name)
        .is_none()
    {
        core.run(
            client
                .create_container()
                .with_container_name(container_name)
                .with_public_access(PublicAccess::Blob)
                .finalize(),
        ).unwrap();
    }

    let mut metadata = HashMap::new();
    metadata.insert("attrib", "value");
    metadata.insert("second", "something");

    // calculate md5 too!
    let _digest = md5::compute(&data[..]);

    let future = client
        .put_append_blob()
        .with_container_name(&container_name)
        .with_blob_name(&blob_name)
        .with_content_type("text/plain")
        .with_metadata(&metadata)
        .finalize();

    core.run(future).unwrap();

    trace!("created {:?}", blob_name);
}

fn initialize() -> Result<(Client, Core), AzureError> {
    let account = std::env::var("STORAGE_ACCOUNT").expect("Set env variable STORAGE_ACCOUNT first!");
    let master_key = std::env::var("STORAGE_MASTER_KEY").expect("Set env variable STORAGE_MASTER_KEY first!");
    let core = Core::new()?;

    Ok((Client::new(&account, &master_key)?, core))
}

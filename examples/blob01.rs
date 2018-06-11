extern crate azure_sdk_for_rust;

extern crate chrono;
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

use azure_sdk_for_rust::{
    core::lease::{LeaseState, LeaseStatus}, storage::blob::{Blob, BlobType, PUT_OPTIONS_DEFAULT},
    storage::client::Client,
};

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
    let blob_name = std::env::args()
        .nth(2)
        .expect("please specify blob name as command line parameter");

    let core = Core::new()?;

    let client = Client::new(&account, &master_key)?;

    let data = b"something";
    let b = Blob {
        name: blob_name,
        container_name: container,
        snapshot_time: None,
        last_modified: chrono::Utc::now(),
        etag: "".to_owned(),
        content_length: data.len() as u64,
        content_type: Some("text/plain".to_owned()),
        content_encoding: None,
        content_language: None,
        content_md5: None,
        cache_control: None,
        x_ms_blob_sequence_number: None,
        blob_type: BlobType::BlockBlob,
        lease_status: LeaseStatus::Unlocked,
        lease_state: LeaseState::Available,
        lease_duration: None,
        copy_id: None,
        copy_status: None,
        copy_source: None,
        copy_progress: None,
        copy_completion: None,
        copy_status_description: None,
    };

    trace!("before put");

    let future = b
        .put(&client, &PUT_OPTIONS_DEFAULT, Some(&data[..]))
        .then(move |res| {
            println!("{:?}", res);
            ok(())
        });

    let handle = core.handle();
    handle.spawn(future);

    Ok(())
}

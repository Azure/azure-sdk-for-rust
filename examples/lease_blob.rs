extern crate azure_sdk_for_rust;

extern crate chrono;
extern crate env_logger;
extern crate futures;
extern crate hyper;
extern crate hyper_tls;
extern crate tokio_core;

use std::error::Error;

use futures::future::*;
use tokio_core::reactor::Core;

use azure_sdk_for_rust::azure::storage::client::Client;
use azure_sdk_for_rust::azure::storage::blob::{Blob, BlobType, LEASE_BLOB_OPTIONS_DEFAULT};
use azure_sdk_for_rust::azure::core::lease::{LeaseAction, LeaseState, LeaseStatus};

use hyper::mime::Mime;

fn main() {
    env_logger::init().unwrap();
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

    let container_name = std::env::args()
        .nth(1)
        .expect("please specify container name as first command line parameter");
    let blob_name = std::env::args()
        .nth(2)
        .expect("please specify file name as second command line parameter");

    let mut core = Core::new()?;

    let client = Client::new(&core.handle(), &account, &master_key)?;

    let new_blob = Blob {
        name: blob_name,
        container_name: container_name.to_owned(),
        snapshot_time: None,
        last_modified: chrono::Utc::now(),
        etag: "".to_owned(),
        content_length: 0,
        content_type: Some("text/plain".parse::<Mime>().unwrap()),
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

    println!("Leasing the blob...");

    let mut lbo = LEASE_BLOB_OPTIONS_DEFAULT.clone();
    lbo.lease_duration = Some(15);
    let future = new_blob
        .lease(&client, LeaseAction::Acquire, &lbo)
        .map(|lease_id| {
            println!("Blob leased");
            lease_id
        });

    let lease_id = core.run(future)?;
    println!("lease id == {:?}", lease_id);

    Ok(())
}

extern crate azure_sdk_for_rust;

extern crate futures;
extern crate tokio_core;
extern crate tokio;
extern crate hyper;
extern crate hyper_tls;
extern crate chrono;

use std::error::Error;

use futures::future::*;
use tokio_core::reactor::Core;

use azure_sdk_for_rust::azure::storage::client::Client;
use azure_sdk_for_rust::azure::storage::blob::{BlobType, Blob, PUT_OPTIONS_DEFAULT};
use azure_sdk_for_rust::azure::core::lease::{LeaseState, LeaseStatus};
use azure_sdk_for_rust::azure::core::errors::AzureError;

use std::str;
use std::fs::metadata;
use std::fs::File;
use std::path;

use hyper::mime::Mime;
use std::io::Read;

fn main() {
    code().unwrap();
}

// We run a separate method to use the elegant quotation mark operator.
// A series of unwrap(), unwrap() would have achieved the same result.
fn code() -> Result<(), Box<Error>> {
    // First we retrieve the account name and master key from environment variables.
    let account = std::env::var("STORAGE_ACCOUNT")
        .expect("Set env variable STORAGE_ACCOUNT first!");
    let master_key = std::env::var("STORAGE_MASTER_KEY")
        .expect("Set env variable STORAGE_MASTER_KEY first!");

    let container_name = std::env::args().nth(1).expect(
        "please specify container name as first command line parameter",
    );
    let file_name = std::env::args()
        .nth(2)
        .expect("please specify file name as second command line parameter");

    let mut core = Core::new()?;

    let client = Client::new(&core.handle(), &account, &master_key)?;

    let metadata = metadata(&file_name)?;

    let name = {
        let path = path::Path::new(&file_name);

        let name = match path.file_name() {
            Some(name) => name,
            None => return Err(Box::new(AzureError::GenericError)),
        };

        match name.to_str() {
            Some(n) => n.to_owned(),
            None => return Err(Box::new(AzureError::GenericError)),
        }
    };

    let contents = {
        let mut file = File::open(file_name)?;
        let mut v = Vec::new();
        file.read_to_end(&mut v)?;
        v
    };

    let new_blob = Blob {
        name: name.to_owned(),
        container_name: container_name.to_owned(),
        snapshot_time: None,
        last_modified: chrono::Utc::now(),
        etag: "".to_owned(),
        content_length: metadata.len(),
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

    let future = new_blob
        .put(&client, &PUT_OPTIONS_DEFAULT, Some(&contents))
        .map(move |_| {
            println!("{} uploaded", name);
        });

    core.run(future)?;

    Ok(())
}

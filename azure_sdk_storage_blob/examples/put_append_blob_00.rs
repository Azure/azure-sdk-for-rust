#[macro_use]
extern crate log;

use azure_sdk_core::prelude::*;
use azure_sdk_storage_blob::prelude::*;
use azure_sdk_storage_core::prelude::*;
use std::collections::HashMap;
use std::error::Error;

#[tokio::main]
async fn main() -> Result<(), Box<dyn Error>> {
    env_logger::init();

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

    let client = client::with_access_key(&account, &master_key);

    let data = b"something";

    let mut metadata = HashMap::new();

    metadata.insert("pollo", "arrosto");
    metadata.insert("milk", "shake");

    // this is not mandatory but it helps preventing
    // spurious data to be uploaded.
    let _digest = md5::compute(&data[..]);

    // The required parameters are container_name, blob_name and body.
    // The builder supports many more optional
    // parameters (such as LeaseID, or ContentDisposition, etc...)
    // so make sure to check with the documentation.
    trace!("before put_append_blob");
    let res = client
        .put_append_blob()
        .with_container_name(&container)
        .with_blob_name(&blob_name)
        .with_content_type("text/plain")
        .with_content_language("en/us")
        .with_metadata(&metadata)
        .finalize()
        .await?;

    println!("{:?}", res);

    Ok(())
}

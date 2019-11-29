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
    trace!("example started");

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

    let client = Client::new(&account, &master_key)?;

    let data: [u8; 2000] = [51; 2000];

    let mut metadata = HashMap::new();

    metadata.insert("pollo", "arrosto");
    metadata.insert("milk", "shake");

    let slice = &data[512..1024];

    // this is not mandatory but it helps preventing
    // spurious data to be uploaded.
    let digest = md5::compute(slice);

    // The required parameters are container_name, blob_name and body.
    // The builder supports many more optional
    // parameters (such as LeaseID, or ContentDisposition, etc...)
    // so make sure to check with the documentation.
    let res = client
        .put_page_blob()
        .with_container_name(&container)
        .with_blob_name(&blob_name)
        .with_content_length(1024 * 3)?
        .with_content_type("text/plain")
        .with_metadata(&metadata)
        .finalize()
        .await?;
    println!("put_blob == {:?}", res);

    // this will update a page. The slice must be at least
    // the size of tha page or a buffer out
    // of bounds error will be thrown.
    let res = client
        .update_page()
        .with_container_name(&container)
        .with_blob_name(&blob_name)
        .with_ba512_range(&BA512Range::new(0, 511)?)
        .with_content_md5(&digest[..])
        .with_body(slice)
        .finalize()
        .await?;
    println!("update first page == {:?}", res);

    // update a second page with the same data
    let res = client
        .update_page()
        .with_container_name(&container)
        .with_blob_name(&blob_name)
        .with_ba512_range(&BA512Range::new(512, 1023)?)
        .with_content_md5(&digest[..])
        .with_body(slice)
        .finalize()
        .await?;
    println!("update second page == {:?}", res);

    // update the second page again with checks
    let res = client
        .update_page()
        .with_container_name(&container)
        .with_blob_name(&blob_name)
        .with_ba512_range(&BA512Range::new(512, 1023)?)
        .with_content_md5(&digest[..])
        .with_body(slice)
        .with_sequence_number_condition(SequenceNumberCondition::Equal(1))
        .finalize()
        .await?;
    println!("update failed sequence number condition == {:?}", res);

    let res = client
        .clear_page()
        .with_container_name(&container)
        .with_blob_name(&blob_name)
        .with_ba512_range(&BA512Range::new(0, 511)?)
        .finalize()
        .await?;
    println!("clear first page {:?}", res);

    Ok(())
}

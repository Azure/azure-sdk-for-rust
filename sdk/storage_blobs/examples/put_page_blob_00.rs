#[macro_use]
extern crate log;
use azure_core::prelude::*;
use azure_storage::core::prelude::*;
use azure_storage_blobs::{prelude::*, BA512Range};
use bytes::Bytes;

#[tokio::main]
async fn main() -> azure_core::Result<()> {
    env_logger::init();
    trace!("example started");

    // First we retrieve the account name and master key from environment variables.
    let account =
        std::env::var("STORAGE_ACCOUNT").expect("Set env variable STORAGE_ACCOUNT first!");
    let master_key =
        std::env::var("STORAGE_MASTER_KEY").expect("Set env variable STORAGE_MASTER_KEY first!");

    let container_name = std::env::args()
        .nth(1)
        .expect("please specify container_name name as command line parameter");
    let blob_name = std::env::args()
        .nth(2)
        .expect("please specify blob name as command line parameter");

    let http_client = azure_core::new_http_client();

    let blob_client =
        StorageAccountClient::new_access_key(http_client.clone(), &account, &master_key)
            .as_container_client(&container_name)
            .as_blob_client(&blob_name);

    let data = Bytes::from_static(&[51; 2000]);

    let mut metadata = Metadata::new();
    metadata.insert("pollo", "arrosto");
    metadata.insert("milk", "shake");

    let slice = data.slice(512..1024);

    // this is not mandatory but it helps preventing
    // spurious data to be uploaded.
    let digest = md5::compute(slice.clone());

    // The required parameters are container_name_name, blob_name.
    // The builder supports many more optional
    // parameters (such as LeaseID, or ContentDisposition, etc...)
    // so make sure to check with the documentation.
    let res = blob_client
        .put_page_blob(1024 * 3)
        .content_type("text/plain")
        .metadata(&metadata)
        .sequence_number(100)
        .execute()
        .await?;
    println!("put_page_blob == {:?}", res);

    // this will update a page. The slice must be at least
    // the size of tha page or a buffer out
    // of bounds error will be thrown.
    let res = blob_client
        .update_page(BA512Range::new(0, 511)?, slice.clone())
        .hash(&digest.into())
        .execute()
        .await?;
    println!("update first page == {:?}", res);

    // update a second page with the same data
    let res = blob_client
        .update_page(BA512Range::new(512, 1023)?, slice.clone())
        .hash(&digest.into())
        .execute()
        .await?;
    println!("update second page == {:?}", res);

    // update the second page again with checks
    let res = blob_client
        .update_page(BA512Range::new(512, 1023)?, slice)
        .hash(&digest.into())
        .sequence_number_condition(SequenceNumberCondition::Equal(100))
        .execute()
        .await?;
    println!("update sequence number condition == {:?}", res);

    // let's get page ranges
    let res = blob_client.get_page_ranges().execute().await?;
    println!("get page ranges == {:?}", res);

    // let's clear a page
    let res = blob_client
        .clear_page(BA512Range::new(0, 511)?)
        .execute()
        .await?;
    println!("clear first page {:?}", res);

    // let's get page ranges again
    let res = blob_client.get_page_ranges().execute().await?;
    println!("get page ranges == {:?}", res);

    Ok(())
}

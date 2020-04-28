use azure_sdk_core::prelude::*;
use azure_sdk_storage_blob::Blob;
use azure_sdk_storage_core::prelude::*;
use std::error::Error;

#[tokio::main]
async fn main() -> Result<(), Box<dyn Error>> {
    // First we retrieve the account name and master key from environment variables.
    let account =
        std::env::var("STORAGE_ACCOUNT").expect("Set env variable STORAGE_ACCOUNT first!");
    let master_key =
        std::env::var("STORAGE_MASTER_KEY").expect("Set env variable STORAGE_MASTER_KEY first!");

    let source_container = std::env::args()
        .nth(1)
        .expect("please specify source container name as first command line parameter");
    let source_blob = std::env::args()
        .nth(2)
        .expect("please specify source blob name as second command line parameter");
    let destination_container = std::env::args()
        .nth(3)
        .expect("please specify destination container name as third command line parameter");
    let destination_blob = std::env::args()
        .nth(4)
        .expect("please specify destination blob name as fourth command line parameter");

    let client = Client::new(&account, &master_key)?;

    let source_url = format!("{}/{}/{}", client.blob_uri(), source_container, source_blob);

    let response = client
        .copy_blob_from_url()
        .with_container_name(&destination_container)
        .with_blob_name(&destination_blob)
        .with_source_url(&source_url as &str)
        .with_is_synchronous(true)
        .finalize()
        .await?;

    println!("response == {:?}", response);

    Ok(())
}

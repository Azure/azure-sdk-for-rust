use azure_sdk_core::prelude::*;
use azure_storage::blob::prelude::*;
use azure_storage::core::prelude::*;
use std::error::Error;

#[tokio::main]
async fn main() -> Result<(), Box<dyn Error>> {
    env_logger::init();

    // First we retrieve the account name and master key from environment variables.
    let account =
        std::env::var("STORAGE_ACCOUNT").expect("Set env variable STORAGE_ACCOUNT first!");
    let master_key =
        std::env::var("STORAGE_MASTER_KEY").expect("Set env variable STORAGE_MASTER_KEY first!");

    let container_name = std::env::args()
        .nth(1)
        .expect("please specify container name as command line parameter");

    let client = client::with_access_key(&account, &master_key);

    let _res = client
        .list_blobs()
        .with_container_name(&container_name)
        .with_include_copy()
        .with_include_deleted()
        .with_include_metadata()
        .with_include_snapshots()
        .with_include_uncommitted_blobs()
        .finalize()
        .await?;

    let result = client
        .get_blob()
        .with_container_name(&container_name)
        .with_blob_name("SorgeniaReorganizeRebuildIndexes.zip")
        .finalize()
        .await?;

    println!("{:?}", result);

    Ok(())
}

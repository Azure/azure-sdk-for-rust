use azure_storage::core::prelude::*;
use azure_storage_blobs::prelude::*;
use std::error::Error;

#[tokio::main]
async fn main() -> Result<(), Box<dyn Error + Send + Sync>> {
    env_logger::init();

    // First we retrieve the account name and master key from environment variables.
    let account =
        std::env::var("STORAGE_ACCOUNT").expect("Set env variable STORAGE_ACCOUNT first!");
    let master_key =
        std::env::var("STORAGE_MASTER_KEY").expect("Set env variable STORAGE_MASTER_KEY first!");

    let container_name = std::env::args()
        .nth(1)
        .expect("please specify container name as command line parameter");

    let options = StorageAccountOptions::default();
    let storage_client =
        StorageAccountClient::new_access_key(account, master_key, options).as_storage_client();
    let container_client = storage_client.as_container_client(&container_name);
    let blob_client = container_client.as_blob_client("SorgeniaReorganizeRebuildIndexes.zip");

    let _res = container_client
        .list_blobs()
        .include_copy(true)
        .include_deleted(true)
        .include_metadata(true)
        .include_snapshots(true)
        .include_uncommitted_blobs(true)
        .execute()
        .await?;

    let result = blob_client.get().execute().await?;

    println!("{:?}", result);

    Ok(())
}

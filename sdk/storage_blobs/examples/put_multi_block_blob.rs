/// This example shows how to upload a multi-block blob.

#[macro_use]
extern crate log;

use azure_storage::prelude::*;
use azure_storage_blobs::prelude::*;
use futures::future::try_join_all;

#[tokio::main]
async fn main() -> azure_core::Result<()> {
    env_logger::init();
    debug!("log initialized");
    // First we retrieve the account name and access key from environment variables.
    let account =
        std::env::var("STORAGE_ACCOUNT").expect("Set env variable STORAGE_ACCOUNT first!");
    let access_key =
        std::env::var("STORAGE_ACCESS_KEY").expect("Set env variable STORAGE_ACCESS_KEY first!");

    let container = std::env::args()
        .nth(1)
        .expect("please specify container name as command line parameter");
    let blob_name = std::env::args()
        .nth(2)
        .expect("please specify blob name as command line parameter");

    let storage_credentials = StorageCredentials::access_key(account.clone(), access_key);
    let blob_client = BlobServiceClient::new(account, storage_credentials)
        .container_client(&container)
        .blob_client(&blob_name);

    let mut block_list = BlockList::default();
    let mut upload_block_futures = vec![];
    for i in 1..=3 {
        let block_id = format!("block-{i}");
        let data = block_id.as_bytes().to_vec();
        let task = blob_client.put_block(block_id.clone(), data).into_future();
        upload_block_futures.push(task);
        block_list
            .blocks
            .push(BlobBlockType::new_uncommitted(block_id));
    }

    // Allow all blocks to upload.
    try_join_all(upload_block_futures).await?;

    // Commit uploaded blocks.
    let resp = blob_client.put_block_list(block_list).await?;
    println!("PutBlockListResponse: {resp:?}");

    Ok(())
}

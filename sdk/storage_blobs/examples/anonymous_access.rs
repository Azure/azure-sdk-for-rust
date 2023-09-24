use azure_storage::prelude::*;
use azure_storage_blobs::prelude::*;
use futures::StreamExt;

#[tokio::main]
async fn main() -> azure_core::Result<()> {
    // First we retrieve the account name and access key from environment variables.
    let account = std::env::var("STORAGE_ACCOUNT").expect("Set env variable STORAGE_ACCOUNT");
    let container = std::env::var("STORAGE_CONTAINER").expect("Set env variable STORAGE_CONTAINER");

    let storage_credentials = StorageCredentials::anonymous();
    let container_client =
        BlobServiceClient::new(account, storage_credentials).container_client(container);

    let mut blob_stream = container_client.list_blobs().into_stream();
    while let Some(blob_entry) = blob_stream.next().await {
        let blob_entry = blob_entry?;
        for blob in blob_entry.blobs.blobs() {
            println!("\t{}", blob.name);
        }
    }

    Ok(())
}

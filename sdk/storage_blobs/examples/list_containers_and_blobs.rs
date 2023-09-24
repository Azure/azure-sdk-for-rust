use azure_storage::prelude::*;
use azure_storage_blobs::prelude::*;
use futures::StreamExt;

#[tokio::main]
async fn main() -> azure_core::Result<()> {
    // First we retrieve the account name and access key from environment variables.
    let account =
        std::env::var("STORAGE_ACCOUNT").expect("Set env variable STORAGE_ACCOUNT first!");
    let access_key =
        std::env::var("STORAGE_ACCESS_KEY").expect("Set env variable STORAGE_ACCESS_KEY first!");

    let storage_credentials = StorageCredentials::access_key(account.clone(), access_key);
    let service_client = BlobServiceClient::new(account, storage_credentials);

    let mut stream = service_client.list_containers().into_stream();

    while let Some(entry) = stream.next().await {
        let entry = entry?;
        for container in entry.containers {
            println!("container: {}", container.name);

            let container_client = service_client.container_client(container.name);

            let mut blob_stream = container_client.list_blobs().into_stream();
            while let Some(blob_entry) = blob_stream.next().await {
                let blob_entry = blob_entry?;
                for blob in blob_entry.blobs.blobs() {
                    println!("\t{}", blob.name);
                }
            }
        }
    }

    Ok(())
}

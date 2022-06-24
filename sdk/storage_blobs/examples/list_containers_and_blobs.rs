use azure_storage::core::prelude::*;
use azure_storage_blobs::prelude::*;
use futures::StreamExt;

#[tokio::main]
async fn main() -> azure_core::Result<()> {
    // First we retrieve the account name and access key from environment variables.
    let account =
        std::env::var("STORAGE_ACCOUNT").expect("Set env variable STORAGE_ACCOUNT first!");
    let access_key =
        std::env::var("STORAGE_ACCESS_KEY").expect("Set env variable STORAGE_ACCESS_KEY first!");

    let http_client = azure_core::new_http_client();

    let storage_client =
        StorageAccountClient::new_access_key(http_client.clone(), &account, &access_key)
            .as_storage_client();
    let blob_service_client = storage_client.as_blob_service_client();

    let mut stream = blob_service_client.list_containers().into_stream();

    while let Some(entry) = stream.next().await {
        let entry = entry?;
        for container in entry.containers {
            println!("container: {}", container.name);

            let container_client = storage_client.as_container_client(container.name);

            let mut blob_stream = container_client.list_blobs().into_stream();
            while let Some(blob_entry) = blob_stream.next().await {
                let blob_entry = blob_entry?;
                for blob in blob_entry.blobs.blobs {
                    println!("\t{}", blob.name);
                }
            }
        }
    }

    Ok(())
}

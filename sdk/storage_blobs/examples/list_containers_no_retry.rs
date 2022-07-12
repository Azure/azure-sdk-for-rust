use azure_core::{ClientOptions, RetryMode, RetryOptions};
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

    let client_options =
        ClientOptions::default().retry(RetryOptions::default().mode(RetryMode::None));
    let options = StorageOptions::default().client_options(client_options);

    let storage_client = StorageClient::new_access_key(&account, &access_key, options);
    let blob_service_client = storage_client.blob_service_client();
    let mut stream = blob_service_client.list_containers().into_stream();

    while let Some(entry) = stream.next().await {
        let entry = entry?;
        for container in entry.containers {
            println!("container: {}", container.name);
        }
    }

    Ok(())
}

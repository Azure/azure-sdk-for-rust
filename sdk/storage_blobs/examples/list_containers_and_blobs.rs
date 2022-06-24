use azure_storage::core::prelude::*;
use azure_storage_blobs::prelude::*;
use futures::StreamExt;

#[tokio::main]
async fn main() -> azure_core::Result<()> {
    env_logger::init();
    // First we retrieve the account name, container and blob name from command line args

    let account = std::env::args()
        .nth(1)
        .expect("please specify the account name as first command line parameter");

    let account_key =
        std::env::var("STORAGE_ACCOUNT_KEY").expect("Set env variable STORAGE_ACCOUNT_KEY first!");

    let http_client = azure_core::new_http_client();
    let storage_account_client =
        StorageAccountClient::new_access_key(http_client, &account, &account_key);

    let blob_service_client = storage_account_client.blob_service_client();

    let mut stream = blob_service_client.list_containers().into_stream();

    while let Some(entry) = stream.next().await {
        let entry = entry?;
        for container in entry.containers {
            println!("container: {}", container.name);

            let container_client = storage_account_client.container_client(container.name);

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

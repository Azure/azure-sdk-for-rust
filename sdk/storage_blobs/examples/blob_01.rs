use azure_storage::core::prelude::*;
use azure_storage_blobs::prelude::*;
use futures::StreamExt;

#[tokio::main]
async fn main() -> azure_core::Result<()> {
    env_logger::init();

    // First we retrieve the account name and access key from environment variables.
    let account =
        std::env::var("STORAGE_ACCOUNT").expect("Set env variable STORAGE_ACCOUNT first!");
    let access_key =
        std::env::var("STORAGE_ACCESS_KEY").expect("Set env variable STORAGE_ACCESS_KEY first!");

    let container_name = std::env::args()
        .nth(1)
        .expect("please specify container name as command line parameter");

    let http_client = azure_core::new_http_client();
    let storage_client =
        StorageAccountClient::new_access_key(http_client.clone(), &account, &access_key)
            .storage_client();
    let container_client = storage_client.container_client(&container_name);
    let blob_client = container_client.blob_client("SorgeniaReorganizeRebuildIndexes.zip");

    // only get the first chunk
    let result = blob_client
        .get()
        .into_stream()
        .next()
        .await
        .expect("stream failed")?;
    println!("{:?}", result);

    Ok(())
}

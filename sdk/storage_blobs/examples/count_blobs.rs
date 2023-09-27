use azure_storage::prelude::*;
use azure_storage_blobs::prelude::*;
use futures::stream::StreamExt;

#[tokio::main]
async fn main() -> azure_core::Result<()> {
    let account =
        std::env::var("STORAGE_ACCOUNT").expect("Set env variable STORAGE_ACCOUNT first!");
    let access_key =
        std::env::var("STORAGE_ACCESS_KEY").expect("Set env variable STORAGE_ACCESS_KEY first!");

    let container = std::env::args()
        .nth(1)
        .expect("please specify container name as command line parameter");

    let storage_credentials = StorageCredentials::access_key(account.clone(), access_key);
    let container_client =
        BlobServiceClient::new(account, storage_credentials).container_client(&container);

    let mut count: usize = 0;
    let mut list_blobs = container_client.list_blobs().into_stream();
    while let Some(list_blobs_response) = list_blobs.next().await {
        let list_blobs_response = list_blobs_response?;
        count += list_blobs_response.blobs.blobs().count();
    }

    println!("blob count {count}");

    Ok(())
}

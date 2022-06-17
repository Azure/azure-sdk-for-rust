use azure_storage::core::prelude::*;
use azure_storage_blobs::prelude::*;
use futures::stream::StreamExt;

#[tokio::main]
async fn main() -> azure_core::Result<()> {
    let account =
        std::env::var("STORAGE_ACCOUNT").expect("Set env variable STORAGE_ACCOUNT first!");
    let master_key =
        std::env::var("STORAGE_MASTER_KEY").expect("Set env variable STORAGE_MASTER_KEY first!");

    let container = std::env::args()
        .nth(1)
        .expect("please specify container name as command line parameter");

    let http_client = azure_core::new_http_client();
    let container_client =
        StorageAccountClient::new_access_key(http_client.clone(), &account, &master_key)
            .as_container_client(&container);

    let mut count: usize = 0;
    let mut list_blobs = Box::pin(container_client.list_blobs().stream());
    while let Some(list_blobs_response) = list_blobs.next().await {
        let list_blobs_response = list_blobs_response?;
        count += list_blobs_response.blobs.blobs.len();
    }

    println!("blob count {}", count);

    Ok(())
}

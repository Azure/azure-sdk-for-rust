use azure_storage::core::prelude::*;
use azure_storage_blobs::prelude::*;
use futures::StreamExt;
use std::num::NonZeroU32;

#[tokio::main]
async fn main() -> azure_core::Result<()> {
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
            .as_storage_client();
    let blob_service_client = storage_client.as_blob_service_client();
    let container_client = storage_client.as_container_client(container_name);

    let max_results = NonZeroU32::new(3).unwrap();
    let mut iv = Box::pin(
        blob_service_client
            .list_containers()
            .max_results(max_results)
            .stream(),
    );

    let mut count = 0;
    while let Some(result) = iv.next().await {
        let container = result?;
        count += container.incomplete_vector.len();
        for container in container.incomplete_vector.iter() {
            println!("\t{}", container.name);
        }
    }

    println!("List containers returned {} containers.", count);

    let mut stream = Box::pin(
        container_client
            .list_blobs()
            .max_results(max_results)
            .stream(),
    );

    let mut count = 0;
    while let Some(result) = stream.next().await {
        let result = result?;
        count += result.blobs.blobs.len();
        for blob in result.blobs.blobs.iter() {
            println!(
                "\t{}\t{} MB",
                blob.name,
                blob.properties.content_length / (1024 * 1024)
            );
        }
    }

    println!("List blob returned {} blobs.", count);

    Ok(())
}

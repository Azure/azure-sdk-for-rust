use azure_storage::prelude::*;
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

    let storage_client = StorageClient::new_access_key(&account, &access_key);
    let blob_service_client = storage_client.blob_service_client();
    let container_client = storage_client.container_client(container_name);

    let max_results = NonZeroU32::new(3).unwrap();
    let mut iv = blob_service_client
        .list_containers()
        .max_results(max_results)
        .into_stream();

    let mut count = 0;
    while let Some(result) = iv.next().await {
        let page = result?;
        count += page.containers.len();
        for container in page.containers.iter() {
            println!("\t{}", container.name);
        }
    }

    println!("List containers returned {} containers.", count);

    let mut stream = container_client
        .list_blobs()
        .max_results(max_results)
        .into_stream();

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

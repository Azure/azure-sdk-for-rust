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

    let storage_credentials = StorageCredentials::access_key(account.clone(), access_key);
    let service_client = BlobServiceClient::new(account, storage_credentials);
    let container_client = service_client.container_client(container_name);

    let max_results = NonZeroU32::new(3).unwrap();
    let mut iv = service_client
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

    println!("List containers returned {count} containers.");

    let mut stream = container_client
        .list_blobs()
        .max_results(max_results)
        .into_stream();

    let mut count = 0;
    while let Some(result) = stream.next().await {
        let result = result?;
        for blob in result.blobs.blobs() {
            count += 1;
            println!(
                "\t{}\t{} MB",
                blob.name,
                blob.properties.content_length / (1024 * 1024)
            );
        }
    }

    println!("List blob returned {count} blobs.");

    Ok(())
}

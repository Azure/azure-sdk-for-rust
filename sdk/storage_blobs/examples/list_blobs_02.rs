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

    let container_name = std::env::args()
        .nth(1)
        .expect("please specify a non-existing container name as command line parameter");

    let storage_credentials = StorageCredentials::access_key(account.clone(), access_key);
    let blob_service = BlobServiceClient::new(account, storage_credentials);
    let container_client = blob_service.container_client(&container_name);

    container_client.create().await?;

    // list empty container
    let page = container_client
        .list_blobs()
        .into_stream()
        .next()
        .await
        .expect("stream failed")?;
    println!("List blob returned {} blobs.", page.blobs.blobs().count());

    for i in 0..3 {
        container_client
            .blob_client(format!("blob{i}.txt"))
            .put_block_blob("somedata")
            .content_type("text/plain")
            .await?;
        println!("\tAdded blob {i}");
    }

    // list full container
    let page = container_client
        .list_blobs()
        .into_stream()
        .next()
        .await
        .expect("stream failed")?;
    println!("List blob returned {} blobs.", page.blobs.blobs().count());

    container_client.delete().await?;
    println!("Container {container_name} deleted");

    Ok(())
}

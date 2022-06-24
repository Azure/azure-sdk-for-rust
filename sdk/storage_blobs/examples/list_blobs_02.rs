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

    let container_name = std::env::args()
        .nth(1)
        .expect("please specify a non-existing container name as command line parameter");

    let http_client = azure_core::new_http_client();
    let storage_client =
        StorageAccountClient::new_access_key(http_client.clone(), &account, &access_key)
            .storage_client();

    create_container_and_list(storage_client, &container_name).await?;

    let storage_client = StorageAccountClient::new_emulator_default().storage_client();
    create_container_and_list(storage_client, &container_name).await?;

    Ok(())
}

async fn create_container_and_list(
    storage_client: std::sync::Arc<StorageClient>,
    container_name: &str,
) -> azure_core::Result<()> {
    let container_client = storage_client.container_client(container_name);

    container_client.create().into_future().await?;

    // list empty container
    let page = container_client
        .list_blobs()
        .into_stream()
        .next()
        .await
        .expect("stream failed")?;
    println!("List blob returned {} blobs.", page.blobs.blobs.len());

    for i in 0..3 {
        container_client
            .blob_client(format!("blob{}.txt", i))
            .put_block_blob("somedata")
            .content_type("text/plain")
            .into_future()
            .await?;
        println!("\tAdded blob {}", i);
    }

    // list full container
    let page = container_client
        .list_blobs()
        .into_stream()
        .next()
        .await
        .expect("stream failed")?;
    println!("List blob returned {} blobs.", page.blobs.blobs.len());

    container_client.delete().into_future().await?;
    println!("Container {} deleted", container_name);

    Ok(())
}

use azure_storage::core::prelude::*;
use azure_storage_blobs::prelude::*;
use std::error::Error;

#[tokio::main]
async fn main() -> Result<(), Box<dyn Error + Send + Sync>> {
    // First we retrieve the account name and master key from environment variables.
    let account =
        std::env::var("STORAGE_ACCOUNT").expect("Set env variable STORAGE_ACCOUNT first!");
    let master_key =
        std::env::var("STORAGE_MASTER_KEY").expect("Set env variable STORAGE_MASTER_KEY first!");

    let container_name = std::env::args()
        .nth(1)
        .expect("please specify a non-existing container name as command line parameter");

    let http_client = azure_core::new_http_client();
    let storage_account =
        StorageAccountClient::new_access_key(http_client.clone(), &account, &master_key)
            .as_storage_client();

    create_container_and_list(storage_account, &container_name).await?;

    let storage_account = StorageAccountClient::new_emulator_default().as_storage_client();
    create_container_and_list(storage_account, &container_name).await?;

    Ok(())
}

async fn create_container_and_list(
    storage: std::sync::Arc<StorageClient>,
    container_name: &str,
) -> Result<(), Box<dyn Error + Send + Sync>> {
    let container = storage.as_container_client(container_name);

    container.create().execute().await?;

    // list empty container
    let iv = container.list_blobs().execute().await?;
    println!("List blob returned {} blobs.", iv.blobs.blobs.len());

    for i in 0..3 {
        container
            .as_blob_client(format!("blob{}.txt", i))
            .put_block_blob("somedata")
            .content_type("text/plain")
            .execute()
            .await?;
        println!("\tAdded blob {}", i);
    }

    // list full container
    let iv = container.list_blobs().execute().await?;
    println!("List blob returned {} blobs.", iv.blobs.blobs.len());

    container.delete().execute().await?;
    println!("Container {} deleted", container_name);

    Ok(())
}

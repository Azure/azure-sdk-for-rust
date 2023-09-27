use azure_storage::prelude::*;
use azure_storage_blobs::prelude::*;
use std::collections::HashMap;
use uuid::Uuid;

#[tokio::main]
async fn main() -> azure_core::Result<()> {
    // First we retrieve the account name and access key from environment variables.
    let account =
        std::env::var("STORAGE_ACCOUNT").expect("Set env variable STORAGE_ACCOUNT first!");
    let access_key =
        std::env::var("STORAGE_ACCESS_KEY").expect("Set env variable STORAGE_ACCESS_KEY first!");

    let container_name = format!("example-{}", Uuid::new_v4());
    let blob_name = format!("file-{}.txt", Uuid::new_v4());
    let blob_notags_name = format!("file-{}.txt", Uuid::new_v4());

    let storage_credentials = StorageCredentials::access_key(account.clone(), access_key);
    let container_client =
        BlobServiceClient::new(account, storage_credentials).container_client(container_name);
    container_client.create().await?;

    let blob_client = container_client.blob_client(&blob_name);

    let mut tags = Tags::new();
    tags.insert("tag1", "value1");

    blob_client.put_block_blob("hello world").tags(tags).await?;

    let result = blob_client.get_tags().await?;
    println!("get tags result: {result:?}");

    let mut new_tags = HashMap::new();
    new_tags.insert("tag2", "value2");
    new_tags.insert("tag3", "value3");
    let result = blob_client.set_tags(new_tags).await?;
    println!("set tags result: {result:?}");

    let result = blob_client.get_tags().await?;
    println!("get tags result: {result:?}");

    for (key, value) in result.tags.into_iter() {
        println!("key:{key} value:{value}");
    }

    let blob_client = container_client.blob_client(&blob_notags_name);
    blob_client.put_block_blob("hello world").await?;
    let result = blob_client.get_tags().await?;
    println!("get tags without tags result: {result:?}");

    container_client.delete().await?;

    Ok(())
}

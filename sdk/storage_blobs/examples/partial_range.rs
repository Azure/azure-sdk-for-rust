use azure_storage::prelude::*;
use azure_storage_blobs::prelude::*;
use futures::stream::StreamExt;
use uuid::Uuid;

#[tokio::main]
async fn main() -> azure_core::Result<()> {
    tracing_subscriber::fmt().init();

    // First we retrieve the account name and access key from environment variables.
    let account =
        std::env::var("STORAGE_ACCOUNT").expect("Set env variable STORAGE_ACCOUNT first!");
    let access_key =
        std::env::var("STORAGE_ACCESS_KEY").expect("Set env variable STORAGE_ACCESS_KEY first!");

    let container_name = format!("range-example-{}", Uuid::new_v4());
    let blob_name = format!("blob-{}.txt", Uuid::new_v4());

    let storage_credentials = StorageCredentials::access_key(account.clone(), access_key);
    let container_client =
        BlobServiceClient::new(account, storage_credentials).container_client(container_name);
    container_client.create().await?;

    let blob_client = container_client.blob_client(&blob_name);

    let buf = "0123456789".repeat(100);

    blob_client.put_block_blob(buf.clone()).await?;

    let range = 3usize..;
    let mut stream = blob_client.get().range(range.clone()).into_stream();

    let mut data: Vec<u8> = vec![];
    while let Some(value) = stream.next().await {
        let value = value?.data.collect().await?;
        println!("{}", value.len());
        data.extend(&value);
    }
    let value = String::from_utf8(data)?;
    assert_eq!(&buf[range.clone()], value);

    container_client.delete().await?;

    Ok(())
}

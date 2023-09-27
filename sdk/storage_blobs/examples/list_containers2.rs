use azure_storage::prelude::*;
use azure_storage_blobs::prelude::*;
use futures::StreamExt;
use serde::Serialize;

#[derive(Debug, Clone, Serialize)]
#[serde(rename_all = "snake_case")]
struct SampleEntity {
    pub something: String,
}

#[tokio::main]
async fn main() -> azure_core::Result<()> {
    // First we retrieve the account name and access key from environment variables.
    let account =
        std::env::var("STORAGE_ACCOUNT").expect("Set env variable STORAGE_ACCOUNT first!");
    let access_key =
        std::env::var("STORAGE_ACCESS_KEY").expect("Set env variable STORAGE_ACCESS_KEY first!");

    let storage_credentials = StorageCredentials::access_key(account.clone(), access_key);
    let service_client = BlobServiceClient::new(account, storage_credentials);

    let response = service_client
        .list_containers()
        .into_stream()
        .next()
        .await
        .expect("stream failed")?;
    println!("response = {response:#?}");

    let response = service_client
        .container_client("$logs")
        .list_blobs()
        .into_stream()
        .next()
        .await
        .expect("stream failed")?;
    println!("response = {response:#?}");

    Ok(())
}

use azure_storage::core::prelude::*;
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

    let http_client = azure_core::new_http_client();

    let storage_client =
        StorageAccountClient::new_access_key(http_client.clone(), &account, &access_key)
            .as_storage_client();
    let blob_service_client = storage_client.as_blob_service_client();

    let response = blob_service_client
        .list_containers()
        .into_stream()
        .next()
        .await
        .expect("stream failed")?;
    println!("response = {:#?}", response);

    let response = storage_client
        .as_container_client("$logs")
        .list_blobs()
        .into_stream()
        .next()
        .await
        .expect("stream failed")?;
    println!("response = {:#?}", response);

    Ok(())
}

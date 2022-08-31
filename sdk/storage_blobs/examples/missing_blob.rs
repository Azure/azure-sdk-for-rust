use azure_core::{error::ErrorKind, StatusCode};
use azure_storage::prelude::*;
use azure_storage_blobs::prelude::*;
use uuid::Uuid;

#[tokio::main]
async fn main() -> azure_core::Result<()> {
    // First we retrieve the account name and access key from environment variables.
    let account =
        std::env::var("STORAGE_ACCOUNT").expect("Set env variable STORAGE_ACCOUNT first!");
    let access_key =
        std::env::var("STORAGE_ACCESS_KEY").expect("Set env variable STORAGE_ACCESS_KEY first!");

    let container_name = format!("example-{}", Uuid::new_v4());
    let blob_name = format!("missing-{}.txt", Uuid::new_v4());

    let storage_credentials = StorageCredentials::Key(account.clone(), access_key);
    let service_client = BlobServiceClient::new(&account, storage_credentials);
    let container_client = service_client.container_client(&container_name);
    println!("creating container {}", container_name);
    container_client.create().into_future().await?;

    let blob_client = container_client.blob_client(&blob_name);

    println!("getting properties for {}/{}", container_name, blob_name);
    let result = blob_client.get_properties().into_future().await;
    let error = result.expect_err("get_properties on missing blob should fail");
    if let ErrorKind::HttpResponse {
        status: StatusCode::NotFound,
        ..
    } = error.kind()
    {
        println!("{}/{} does not exist", container_name, blob_name);
    } else {
        panic!("unexpected error: {}", error);
    }

    // cleanup
    container_client.delete().into_future().await?;

    Ok(())
}

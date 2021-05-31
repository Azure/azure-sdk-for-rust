use azure_core::prelude::*;
use azure_storage::blob::prelude::*;
use azure_storage::core::prelude::*;
use std::error::Error;
use std::sync::Arc;

#[tokio::main]
async fn main() -> Result<(), Box<dyn Error + Send + Sync>> {
    env_logger::init();

    // First we retrieve the account name and master key from environment variables.
    let account =
        std::env::var("STORAGE_ACCOUNT").expect("Set env variable STORAGE_ACCOUNT first!");
    let master_key =
        std::env::var("STORAGE_MASTER_KEY").expect("Set env variable STORAGE_MASTER_KEY first!");

    let container_name = std::env::args()
        .nth(1)
        .expect("please specify container name as command line parameter");

    let http_client: Arc<dyn HttpClient> = Arc::new(reqwest::Client::new());
    let storage_account =
        StorageAccountClient::new_access_key(http_client.clone(), &account, &master_key)
            .as_storage_client();
    let container = storage_account.as_container_client(&container_name);
    let blob = container.as_blob_client("SorgeniaReorganizeRebuildIndexes.zip");

    let _res = container
        .list_blobs()
        .include_copy(true)
        .include_deleted(true)
        .include_metadata(true)
        .include_snapshots(true)
        .include_uncommitted_blobs(true)
        .execute()
        .await?;

    let result = blob.get().execute().await?;

    println!("{:?}", result);

    Ok(())
}

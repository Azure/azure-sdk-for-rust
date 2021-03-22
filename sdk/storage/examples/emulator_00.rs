use azure_core::prelude::*;
use azure_storage::blob_storage::prelude::*;
use azure_storage::core::prelude::*;
use std::error::Error;
use std::sync::Arc;
use url::Url;

#[tokio::main]
async fn main() -> Result<(), Box<dyn Error + Send + Sync>> {
    env_logger::init();

    // this is how you use the emulator.
    let blob_storage_url = Url::parse("http://127.0.0.1:10000")?;
    let table_storage_url = Url::parse("http://127.0.0.1:10002")?;

    let http_client: Arc<Box<dyn HttpClient>> = Arc::new(Box::new(reqwest::Client::new()));
    let storage_account =
        StorageAccountClient::new_emulator(http_client, &blob_storage_url, &table_storage_url)
            .as_storage_client();
    let container = storage_account.as_container_client("emulcont");

    // create container
    let res = container
        .create()
        .public_access(PublicAccess::None)
        .execute()
        .await?;
    println!("{:?}", res);

    let res = container
        .list_blobs()
        .include_metadata(true)
        .execute()
        .await?;
    println!("{:?}", res);

    Ok(())
}

use azure_core::prelude::*;
use azure_storage::blob::prelude::*;
use azure_storage::core::prelude::*;
use std::error::Error;
use url::Url;

#[tokio::main]
async fn main() -> Result<(), Box<dyn Error + Send + Sync>> {
    env_logger::init();

    // this is how you use the emulator.
    let blob_storage_url = Url::parse("http://127.0.0.1:10000")?;
    let queue_storage_url = Url::parse("http://127.0.0.1:10001")?;
    let table_storage_url = Url::parse("http://127.0.0.1:10002")?;
    let filesystem_url = Url::parse("http://127.0.0.1:10004")?;

    let http_client = new_http_client();
    let storage_account = StorageAccountClient::new_emulator(
        http_client,
        &blob_storage_url,
        &table_storage_url,
        &queue_storage_url,
        &filesystem_url,
    )
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

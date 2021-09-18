use azure_storage::blob::prelude::*;
use azure_storage::core::prelude::*;
use std::error::Error;

#[tokio::main]
async fn main() -> Result<(), Box<dyn Error + Send + Sync>> {
    env_logger::init();

    // this is how you use the emulator.
    let storage_account = StorageAccountClient::new_emulator_default().as_storage_client();
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

use azure_core::error::Result;
use azure_storage::core::prelude::*;
use azure_storage_blobs::prelude::*;

#[tokio::main]
async fn main() -> Result<()> {
    env_logger::init();

    // this is how you use the emulator.
    let storage_account = StorageAccountClient::new_emulator_default().as_storage_client();
    let container_client = storage_account.as_container_client("emulcont");

    // create container
    let res = container_client
        .create()
        .public_access(PublicAccess::None)
        .execute()
        .await?;
    println!("{:?}", res);

    let res = container_client
        .list_blobs()
        .include_metadata(true)
        .execute()
        .await?;
    println!("{:?}", res);

    Ok(())
}

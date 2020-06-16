use azure_sdk_core::prelude::*;
use azure_sdk_storage_blob::prelude::*;
use azure_sdk_storage_core::prelude::*;
use std::error::Error;
use url::Url;

#[tokio::main]
async fn main() -> Result<(), Box<dyn Error>> {
    env_logger::init();

    // this is how you use the emulator.
    let blob_storage_url = "http://127.0.0.1:10000";
    let table_storage_url = "http://127.0.0.1:10002";
    let client = client::with_emulator(
        &Url::parse(blob_storage_url)?,
        &Url::parse(table_storage_url)?,
    );

    // create container
    let res = client
        .create_container()
        .with_container_name("emulcont")
        .with_public_access(PublicAccess::None)
        .finalize()
        .await?;
    println!("{:?}", res);

    let res = client
        .list_blobs()
        .with_container_name("emulcont")
        .with_include_metadata()
        .finalize()
        .await?;
    println!("{:?}", res);

    Ok(())
}

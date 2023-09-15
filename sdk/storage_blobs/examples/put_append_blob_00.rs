#[macro_use]
extern crate log;

use azure_core::prelude::*;
use azure_storage::prelude::*;
use azure_storage_blobs::prelude::*;

#[tokio::main]
async fn main() -> azure_core::Result<()> {
    env_logger::init();

    // First we retrieve the account name and access key from environment variables.
    let account =
        std::env::var("STORAGE_ACCOUNT").expect("Set env variable STORAGE_ACCOUNT first!");
    let access_key =
        std::env::var("STORAGE_ACCESS_KEY").expect("Set env variable STORAGE_ACCESS_KEY first!");

    let container = std::env::args()
        .nth(1)
        .expect("please specify container name as command line parameter");
    let blob_name = std::env::args()
        .nth(2)
        .expect("please specify blob name as command line parameter");

    let storage_credentials = StorageCredentials::Key(account.clone(), access_key);
    let blob_client =
        ClientBuilder::new(account, storage_credentials).blob_client(&container, &blob_name);

    let mut metadata = Metadata::new();

    metadata.insert("pollo", "arrosto");
    metadata.insert("milk", "shake");

    // The required parameters are container_name, blob_name and body.
    // The builder supports many more optional
    // parameters (such as LeaseID, or ContentDisposition, etc...)
    // so make sure to check with the documentation.
    trace!("before put_append_blob");
    let res = blob_client
        .put_append_blob()
        .content_type("text/plain")
        .content_language("en/us")
        .metadata(metadata)
        .await?;

    println!("{res:?}");

    // let get back the metadata
    let res = blob_client.get_metadata().await?;
    println!("{res:?}");

    Ok(())
}

#[macro_use]
extern crate log;

use azure_storage::prelude::*;
use azure_storage_blobs::prelude::*;
use bytes::Bytes;

#[tokio::main]
async fn main() -> azure_core::Result<()> {
    env_logger::init();
    debug!("log initialized");
    // First we retrieve the account name and access key from environment variables.
    let account =
        std::env::var("STORAGE_ACCOUNT").expect("Set env variable STORAGE_ACCOUNT first!");
    let access_key =
        std::env::var("STORAGE_ACCESS_KEY").expect("Set env variable STORAGE_ACCESS_KEY first!");

    let container_name = std::env::args()
        .nth(1)
        .expect("please specify container name as command line parameter");
    let blob_name = std::env::args()
        .nth(2)
        .expect("please specify blob name as command line parameter");

    let storage_credentials = StorageCredentials::Key(account.clone(), access_key);
    let blob_client =
        ClientBuilder::new(account, storage_credentials).blob_client(&container_name, &blob_name);

    let data = Bytes::from_static(b"something");

    // this is not mandatory but it helps preventing
    // spurious data to be uploaded.
    let hash = md5::compute(&data[..]).0;

    // The required parameters are container_name, blob_name and body.
    // The builder supports many more optional
    // parameters (such as LeaseID, or ContentDisposition, MD5 etc...)
    // so make sure to check with the documentation.
    let res = blob_client
        .put_block_blob(data.clone())
        .content_type("text/plain")
        .hash(hash)
        .await?;
    println!("put_blob {res:?}");

    let res = blob_client.snapshot().await?;
    println!("blob snapshot: {:?}", res.snapshot);

    let res = blob_client
        .delete()
        .delete_snapshots_method(DeleteSnapshotsMethod::Include)
        .await?;
    println!("Delete blob == {res:?}");

    Ok(())
}

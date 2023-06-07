#[macro_use]
extern crate log;

use azure_storage::prelude::*;
use azure_storage_blobs::prelude::*;
use bytes::Bytes;
use std::time::Duration;

#[tokio::main]
async fn main() -> azure_core::Result<()> {
    env_logger::init();
    debug!("log initialized");
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
    let blob_client = BlobServiceClient::new(account, storage_credentials)
        .container_client(&container)
        .blob_client(&blob_name);

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
    println!("1-put_block_blob {res:?}");

    let mut block_list = BlockList::default();
    block_list
        .blocks
        .push(BlobBlockType::new_uncommitted("satanasso"));
    block_list
        .blocks
        .push(BlobBlockType::new_uncommitted("pollastro"));

    let res = blob_client.put_block("satanasso", data.clone()).await?;
    println!("2-put_block {res:?}");

    let res = blob_client.put_block("pollastro", data).await?;
    println!("3-put_block {res:?}");

    let ret = blob_client
        .get_block_list()
        .block_list_type(BlockListType::All)
        .await?;

    println!("GetBlockList == {ret:?}");

    let bl = ret.block_with_size_list.into();
    println!("bl == {bl:?}");

    let res = blob_client.put_block_list(bl).await?;
    println!("PutBlockList == {res:?}");

    let res = blob_client.acquire_lease(Duration::from_secs(60)).await?;
    println!("Acquire lease == {res:?}");

    let lease = blob_client.blob_lease_client(res.lease_id);

    let res = lease.renew().await?;
    println!("Renew lease == {res:?}");

    let res = blob_client
        .break_lease()
        .lease_break_period(Duration::from_secs(15))
        .await?;
    println!("Break lease == {res:?}");

    let res = lease.release().await?;
    println!("Release lease == {res:?}");

    let res = blob_client
        .delete()
        .delete_snapshots_method(DeleteSnapshotsMethod::Include)
        .await?;
    println!("Delete blob == {res:?}");

    Ok(())
}

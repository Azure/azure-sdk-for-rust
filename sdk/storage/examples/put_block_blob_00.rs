#[macro_use]
extern crate log;

use azure_storage::blob::prelude::*;
use azure_storage::core::prelude::*;
use bytes::Bytes;
use std::error::Error;
use std::time::Duration;

#[tokio::main]
async fn main() -> Result<(), Box<dyn Error + Send + Sync>> {
    env_logger::init();
    debug!("log initialized");
    // First we retrieve the account name and master key from environment variables.
    let account =
        std::env::var("STORAGE_ACCOUNT").expect("Set env variable STORAGE_ACCOUNT first!");
    let master_key =
        std::env::var("STORAGE_MASTER_KEY").expect("Set env variable STORAGE_MASTER_KEY first!");

    let container = std::env::args()
        .nth(1)
        .expect("please specify container name as command line parameter");
    let blob_name = std::env::args()
        .nth(2)
        .expect("please specify blob name as command line parameter");

    let http_client = azure_core::new_http_client();

    let storage_account_client =
        StorageAccountClient::new_access_key(http_client.clone(), &account, &master_key);
    let storage_client = storage_account_client.as_storage_client();
    let blob = storage_client
        .as_container_client(&container)
        .as_blob_client(&blob_name);

    let data = Bytes::from_static(b"something");

    // this is not mandatory but it helps preventing
    // spurious data to be uploaded.
    let hash = md5::compute(&data[..]).into();

    // The required parameters are container_name, blob_name and body.
    // The builder supports many more optional
    // parameters (such as LeaseID, or ContentDisposition, MD5 etc...)
    // so make sure to check with the documentation.
    let res = blob
        .put_block_blob(data.clone())
        .content_type("text/plain")
        .hash(&hash)
        .execute()
        .await?;
    println!("1-put_block_blob {:?}", res);

    let mut block_list = BlockList::default();
    block_list
        .blocks
        .push(BlobBlockType::new_uncommitted("satanasso"));
    block_list
        .blocks
        .push(BlobBlockType::new_uncommitted("pollastro"));

    let res = blob.put_block("satanasso", data.clone()).execute().await?;
    println!("2-put_block {:?}", res);

    let res = blob.put_block("pollastro", data).execute().await?;
    println!("3-put_block {:?}", res);

    let ret = blob
        .get_block_list()
        .block_list_type(BlockListType::All)
        .execute()
        .await?;

    println!("GetBlockList == {:?}", ret);

    let bl = ret.block_with_size_list.into();
    println!("bl == {:?}", bl);

    let res = blob.put_block_list(&bl).execute().await?;
    println!("PutBlockList == {:?}", res);

    let res = blob
        .acquire_lease(Duration::from_secs(60))
        .execute()
        .await?;
    println!("Acquire lease == {:?}", res);

    let lease = blob.as_blob_lease_client(res.lease_id);

    let res = lease.renew().execute().await?;
    println!("Renew lease == {:?}", res);

    let res = blob
        .break_lease()
        .lease_break_period(Duration::from_secs(15))
        .execute()
        .await?;
    println!("Break lease == {:?}", res);

    let res = lease.release().execute().await?;
    println!("Release lease == {:?}", res);

    let res = blob
        .delete()
        .delete_snapshots_method(DeleteSnapshotsMethod::Include)
        .execute()
        .await?;
    println!("Delete blob == {:?}", res);

    Ok(())
}

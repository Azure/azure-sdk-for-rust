#[macro_use]
extern crate log;

use azure_sdk_core::prelude::*;
use azure_sdk_storage_blob::prelude::*;
use azure_sdk_storage_core::prelude::*;
use std::error::Error;

#[tokio::main]
async fn main() -> Result<(), Box<dyn Error>> {
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

    let client = Client::new(&account, &master_key)?;

    let data = b"something";

    // this is not mandatory but it helps preventing
    // spurious data to be uploaded.
    let digest = md5::compute(&data[..]);

    // The required parameters are container_name, blob_name and body.
    // The builder supports many more optional
    // parameters (such as LeaseID, or ContentDisposition, MD5 etc...)
    // so make sure to check with the documentation.
    let res = client
        .put_block_blob()
        .with_container_name(&container)
        .with_blob_name(&blob_name)
        .with_content_type("text/plain")
        .with_body(&data[..])
        .with_content_md5(&digest[..])
        .finalize()
        .await?;
    println!("{:?}", res);

    let mut block_list = BlockList::default();
    block_list
        .blocks
        .push(BlobBlockType::Uncommitted(b"satanasso" as &[u8]));
    block_list
        .blocks
        .push(BlobBlockType::Uncommitted(b"pollastro" as &[u8]));

    let res = client
        .put_block()
        .with_container_name(&container)
        .with_blob_name(&blob_name)
        .with_body(&data[..])
        .with_block_id(b"satanasso" as &[u8])
        .finalize()
        .await?;
    println!("{:?}", res);

    client
        .put_block()
        .with_container_name(&container)
        .with_blob_name(&blob_name)
        .with_body(&data[..])
        .with_block_id(b"pollastro" as &[u8])
        .finalize()
        .await?;

    let ret = client
        .get_block_list()
        .with_container_name(&container)
        .with_blob_name(&blob_name)
        .with_block_list_type(BlockListType::All)
        .finalize()
        .await?;

    println!("GetBlockList == {:?}", ret);

    let bl = ret.block_with_size_list.into();
    println!("bl == {:?}", bl);

    let res = client
        .put_block_list()
        .with_container_name(&container)
        .with_blob_name(&blob_name)
        .with_block_list(&bl)
        .finalize()
        .await?;
    println!("PutBlockList == {:?}", res);

    let res = client
        .acquire_blob_lease()
        .with_container_name(&container)
        .with_blob_name(&blob_name)
        .with_lease_duration(60)
        .finalize()
        .await?;
    println!("Acquire lease == {:?}", res);

    let lease_id = res.lease_id;

    let res = client
        .renew_blob_lease()
        .with_container_name(&container)
        .with_blob_name(&blob_name)
        .with_lease_id(&lease_id)
        .finalize()
        .await?;
    println!("Renew lease == {:?}", res);

    let res = client
        .break_blob_lease()
        .with_container_name(&container)
        .with_blob_name(&blob_name)
        .with_lease_break_period(15)
        .finalize()
        .await?;
    println!("Break lease == {:?}", res);

    let res = client
        .release_blob_lease()
        .with_container_name(&container)
        .with_blob_name(&blob_name)
        .with_lease_id(&lease_id)
        .finalize()
        .await?;
    println!("Release lease == {:?}", res);

    let res = client
        .delete_blob()
        .with_container_name(&container)
        .with_blob_name(&blob_name)
        .with_delete_snapshots_method(DeleteSnapshotsMethod::Include)
        .finalize()
        .await?;
    println!("Delete blob == {:?}", res);

    Ok(())
}

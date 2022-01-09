use azure_storage::core::prelude::*;
use azure_storage_blobs::prelude::*;
use bytes::Bytes;
use std::error::Error;

#[tokio::main]
async fn main() -> Result<(), Box<dyn Error + Send + Sync>> {
    env_logger::init();
    // First we retrieve the account name and master key from environment variables.
    let account =
        std::env::var("STORAGE_ACCOUNT").expect("Set env variable STORAGE_ACCOUNT first!");
    let master_key =
        std::env::var("STORAGE_MASTER_KEY").expect("Set env variable STORAGE_MASTER_KEY first!");

    let container_name = std::env::args()
        .nth(1)
        .expect("please specify container name as command line parameter");

    let options = StorageAccountOptions::default();
    let storage_client =
        StorageAccountClient::new_access_key(account, master_key, options).as_storage_client();
    let container_client = storage_client.as_container_client(&container_name);

    // create container
    let res = container_client
        .create()
        .public_access(PublicAccess::None)
        .execute()
        .await?;
    println!("{:?}", res);

    let data = Bytes::from_static(b"something");

    // this is not mandatory but it helps preventing
    // spurious data to be uploaded.
    let hash = md5::compute(&data[..]).into();

    let res = container_client
        .as_blob_client("blob0.txt")
        .put_block_blob(data.clone())
        .content_type("text/plain")
        .hash(&hash)
        .execute()
        .await?;
    println!("{:?}", res);

    let res = container_client
        .as_blob_client("blob1.txt")
        .put_block_blob(data.clone())
        .content_type("text/plain")
        .hash(&hash)
        .execute()
        .await?;
    println!("{:?}", res);

    let res = container_client
        .as_blob_client("blob2.txt")
        .put_block_blob(data)
        .content_type("text/plain")
        .hash(&hash)
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

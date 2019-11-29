use azure_sdk_core::prelude::*;
use azure_sdk_storage_blob::prelude::*;
use azure_sdk_storage_core::prelude::*;
use std::error::Error;

#[tokio::main]
async fn main() -> Result<(), Box<dyn Error>> {
    env_logger::init();
    // First we retrieve the account name and master key from environment variables.
    let account =
        std::env::var("STORAGE_ACCOUNT").expect("Set env variable STORAGE_ACCOUNT first!");
    let master_key =
        std::env::var("STORAGE_MASTER_KEY").expect("Set env variable STORAGE_MASTER_KEY first!");

    let container_name = std::env::args()
        .nth(1)
        .expect("please specify container name as command line parameter");

    let client = Client::new(&account, &master_key)?;

    // create container
    let res = client
        .create_container()
        .with_container_name(&container_name)
        .with_public_access(PublicAccess::None)
        .finalize()
        .await?;
    println!("{:?}", res);

    let data = b"something";

    // this is not mandatory but it helps preventing
    // spurious data to be uploaded.
    let digest = md5::compute(&data[..]);

    let res = client
        .put_block_blob()
        .with_container_name(&container_name)
        .with_blob_name("blob0.txt")
        .with_content_type("text/plain")
        .with_body(&data[..])
        .with_content_md5(&digest[..])
        .finalize()
        .await?;
    println!("{:?}", res);

    let res = client
        .put_block_blob()
        .with_container_name(&container_name)
        .with_blob_name("blob1.txt")
        .with_content_type("text/plain")
        .with_body(&data[..])
        .with_content_md5(&digest[..])
        .finalize()
        .await?;
    println!("{:?}", res);

    let res = client
        .put_block_blob()
        .with_container_name(&container_name)
        .with_blob_name("blob2.txt")
        .with_content_type("text/plain")
        .with_body(&data[..])
        .with_content_md5(&digest[..])
        .finalize()
        .await?;
    println!("{:?}", res);

    let res = client
        .list_blobs()
        .with_container_name(&container_name)
        .with_include_metadata()
        .finalize()
        .await?;
    println!("{:?}", res);

    Ok(())
}

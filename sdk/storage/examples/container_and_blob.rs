use azure_core::prelude::*;
use azure_storage::blob::prelude::*;
use azure_storage::core::prelude::*;
use std::error::Error;
use std::sync::Arc;

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

    let client = client::with_access_key(&account, &master_key);

    let http_client: Arc<Box<dyn HttpClient>> = Arc::new(Box::new(reqwest::Client::new()));
    let storage_account =
        StorageAccountClient::new_access_key(http_client.clone(), &account, &master_key)
            .as_storage_client();
    let container = storage_account.as_container_client(&container_name);

    // create container
    let res = container
        .create()
        .with_public_access(PublicAccess::None)
        .execute()
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

    let res = container
        .list_blobs()
        .with_include_metadata(true)
        .execute()
        .await?;
    println!("{:?}", res);

    Ok(())
}

use azure_storage::prelude::*;
use azure_storage_blobs::prelude::*;
use bytes::Bytes;
use futures::StreamExt;

#[tokio::main]
async fn main() -> azure_core::Result<()> {
    env_logger::init();
    // First we retrieve the account name and access key from environment variables.
    let account =
        std::env::var("STORAGE_ACCOUNT").expect("Set env variable STORAGE_ACCOUNT first!");
    let access_key =
        std::env::var("STORAGE_ACCESS_KEY").expect("Set env variable STORAGE_ACCESS_KEY first!");

    let container_name = std::env::args()
        .nth(1)
        .expect("please specify container name as command line parameter");

    let storage_credentials = StorageCredentials::access_key(account.clone(), access_key);
    let container_client =
        BlobServiceClient::new(account, storage_credentials).container_client(container_name);

    // create container
    container_client
        .create()
        .public_access(PublicAccess::None)
        .await?;

    let data = Bytes::from_static(b"something");

    // this is not mandatory but it helps preventing
    // spurious data to be uploaded.
    let hash = md5::compute(&data[..]).0;

    let res = container_client
        .blob_client("blob0.txt")
        .put_block_blob(data.clone())
        .content_type("text/plain")
        .hash(hash)
        .await?;
    println!("{res:?}");

    let res = container_client
        .blob_client("blob1.txt")
        .put_block_blob(data.clone())
        .content_type("text/plain")
        .hash(hash)
        .await?;
    println!("{res:?}");

    let res = container_client
        .blob_client("blob2.txt")
        .put_block_blob(data)
        .content_type("text/plain")
        .hash(hash)
        .await?;
    println!("{res:?}");

    // only get the first set of blobs in the list
    let res = container_client
        .list_blobs()
        .include_metadata(true)
        .into_stream()
        .next()
        .await
        .expect("stream failed")?;
    println!("{res:?}");

    Ok(())
}

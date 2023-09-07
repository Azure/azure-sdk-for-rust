use azure_core::error::{ErrorKind, ResultExt};
use azure_storage::prelude::*;
use azure_storage_blobs::prelude::*;
use bytes::{BufMut, Bytes};

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

    let storage_credentials = StorageCredentials::Key(account.clone(), access_key);
    let blob_client = BlobServiceClient::new(account, storage_credentials)
        .container_client(container_name)
        .blob_client("test1");

    // this example fills a 1 KB file with ASCII text and
    // sends it in chunks of 256 bytes (4 chunks).
    // It then finalizes the block blob by calling
    // PutBlockList. Finally it gets back
    // the blob as a whole.
    let mut data = bytes::BytesMut::with_capacity(1024);
    for _ in 0..1024 / 64 {
        data.put("the brown fox jumped over the lazy dog. 123456789Pangram12345678".as_bytes());
    }
    let data = data.freeze();

    println!("data to send is {} bytes.", data.len());

    let mut block_ids = Vec::new();
    for i in 0..(1024 / 256) {
        let slice = data.slice(i * 256..(i + 1) * 256);

        let block_id = Bytes::from(format!("{i}"));
        block_ids.push(block_id.clone());
        let hash = md5::compute(slice.clone()).0;

        let put_block_response = blob_client.put_block(block_id, slice).hash(hash).await?;

        println!("put_block_response == {put_block_response:#?}");
    }

    let mut block_list = BlockList::default();
    for id in block_ids.into_iter() {
        block_list.blocks.push(BlobBlockType::new_uncommitted(id));
    }

    let res = blob_client
        .put_block_list(block_list)
        .content_md5(md5::compute(data).0)
        .await?;
    println!("PutBlockList == {res:?}");

    let blob = blob_client.get_content().await?;

    let s = String::from_utf8(blob).map_kind(ErrorKind::DataConversion)?;
    println!("retrieved contents == {s}");

    Ok(())
}

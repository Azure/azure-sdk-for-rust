use azure_storage::core::prelude::*;
use azure_storage_blobs::prelude::*;
use bytes::{BufMut, Bytes};
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
    let blob_client = StorageAccountClient::new_access_key(account, master_key, options)
        .as_container_client(&container_name)
        .as_blob_client("test1");

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

        let block_id = Bytes::from(format!("{}", i));
        block_ids.push(block_id.clone());
        let hash = md5::compute(slice.clone()).into();

        let put_block_response = blob_client
            .put_block(block_id, slice)
            .hash(&hash)
            .execute()
            .await?;

        println!("put_block_response == {:#?}", put_block_response);
    }

    let mut block_list = BlockList::default();
    for id in block_ids.into_iter() {
        block_list.blocks.push(BlobBlockType::new_uncommitted(id));
    }

    let res = blob_client
        .put_block_list(&block_list)
        .content_md5(md5::compute(data))
        .execute()
        .await?;
    println!("PutBlockList == {:?}", res);

    let retrieved_blob = blob_client.get().execute().await?;
    println!("retrieved_blob == {:?}", retrieved_blob);

    let s = String::from_utf8(retrieved_blob.data.to_vec())?;
    println!("retrieved contents == {}", s);

    Ok(())
}

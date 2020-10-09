use azure_sdk_core::prelude::*;
use azure_storage::blob::prelude::*;
use azure_storage::core::prelude::*;
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

    let client = client::with_access_key(&account, &master_key);

    let data = b"1337 azure blob test";
    let blob = "test1";
    let mut block_ids = Vec::new();
    for (i, block) in data.chunks(64 * 1024 * 1024 /* 64 MiB */).enumerate() {
        block_ids.push(i.to_be_bytes());
        let digest = md5::compute(block);
        let put_block_response = client
            .put_block()
            .with_container_name(&container_name)
            .with_blob_name(blob)
            .with_body(block)
            .with_block_id(&i.to_be_bytes()[..])
            .with_content_md5(&digest[..])
            .finalize()
            .await?;

        println!("put_block_response == {:#?}", put_block_response);
    }

    let mut block_list = BlockList::default();
    for id in block_ids.iter() {
        block_list.blocks.push(BlobBlockType::Uncommitted(&id[..]));
    }

    Ok(())
}

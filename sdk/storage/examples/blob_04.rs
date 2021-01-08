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

    let http_client: Arc<Box<dyn HttpClient>> = Arc::new(Box::new(reqwest::Client::new()));

    let storage_account_client =
        StorageAccountClient::new_access_key(http_client.clone(), &account, &master_key);
    let storage_client = storage_account_client.as_storage_client();
    let blob = storage_client
        .as_container_client(&container_name)
        .as_blob_client("test1");

    let data = b"1337 azure blob test";
    let mut block_ids = Vec::new();
    for (i, block) in data.chunks(64 * 1024 * 1024 /* 64 MiB */).enumerate() {
        block_ids.push(i.to_be_bytes());
        let hash = md5::compute(block).into();
        let block_id = (&i.to_be_bytes() as &[u8]).into();

        let put_block_response = blob
            .put_block(&block_id, block)
            .hash(&hash)
            .execute()
            .await?;

        println!("put_block_response == {:#?}", put_block_response);
    }

    let mut block_list = BlockList::default();
    for id in block_ids.iter() {
        block_list.blocks.push(BlobBlockType::Uncommitted(&id[..]));
    }

    Ok(())
}

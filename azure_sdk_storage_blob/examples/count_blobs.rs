use azure_sdk_core::prelude::*;
use azure_sdk_storage_blob::prelude::*;
use azure_sdk_storage_core::prelude::*;
use futures::stream::StreamExt;
use std::error::Error;

#[tokio::main]
async fn main() -> Result<(), Box<dyn Error>> {
    let account =
        std::env::var("STORAGE_ACCOUNT").expect("Set env variable STORAGE_ACCOUNT first!");
    let master_key =
        std::env::var("STORAGE_MASTER_KEY").expect("Set env variable STORAGE_MASTER_KEY first!");

    let container = std::env::args()
        .nth(1)
        .expect("please specify container name as command line parameter");

    let client = Client::new(&account, &master_key)?;

    let mut count: u32 = 0;
    let mut list_blobs = Box::pin(
        client
            .stream_list_blobs()
            .with_container_name(&container)
            .finalize(),
    );
    while let Some(_blob) = list_blobs.next().await {
        count += 1;
    }

    println!("blob count {}", count);

    Ok(())
}

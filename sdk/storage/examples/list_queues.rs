#[macro_use]
extern crate log;

use azure_core::prelude::*;
use azure_storage::core::prelude::*;
use futures::stream::StreamExt;
use std::error::Error;
use std::num::NonZeroU32;
use std::sync::Arc;

#[tokio::main]
async fn main() -> Result<(), Box<dyn Error + Send + Sync>> {
    // First we retrieve the account name and master key from environment variables.
    let account =
        std::env::var("STORAGE_ACCOUNT").expect("Set env variable STORAGE_ACCOUNT first!");
    let master_key =
        std::env::var("STORAGE_MASTER_KEY").expect("Set env variable STORAGE_MASTER_KEY first!");

    let http_client: Arc<Box<dyn HttpClient>> = Arc::new(Box::new(reqwest::Client::new()));

    let storage_account_client =
        StorageAccountClient::new_access_key(http_client.clone(), &account, &master_key);
    let storage_client = storage_account_client.as_storage_client();

    trace!("enumerating queues");

    let response = storage_client
        .list_queues()
        .prefix("a")
        .include_metadata(true)
        .max_results(NonZeroU32::new(2u32).unwrap())
        .execute()
        .await?;

    println!("response == {:#?}", response);

    let mut stream = Box::pin(
        storage_client
            .list_queues()
            .max_results(NonZeroU32::new(3u32).unwrap())
            .stream(),
    );

    while let Some(value) = stream.next().await {
        let value = value?;
        let len = value.queues.queues.len();
        println!("received {} queues", len);

        value
            .queues
            .queues
            .iter()
            .for_each(|queue| println!("{}", queue.name));
    }

    Ok(())
}

use azure_storage::prelude::*;
use azure_storage_queues::prelude::*;
use futures::stream::StreamExt;
use std::num::NonZeroU32;

#[tokio::main]
async fn main() -> azure_core::Result<()> {
    // First we retrieve the account name and access key from environment variables.
    let account =
        std::env::var("STORAGE_ACCOUNT").expect("Set env variable STORAGE_ACCOUNT first!");
    let access_key =
        std::env::var("STORAGE_ACCESS_KEY").expect("Set env variable STORAGE_ACCESS_KEY first!");

    let storage_credentials = StorageCredentials::access_key(account.clone(), access_key);
    let queue_service = QueueServiceClient::new(account, storage_credentials);

    println!("getting service stats");
    let response = queue_service.get_queue_service_stats().await?;
    println!("get_queue_service_properties.response == {response:#?}");

    println!("getting service properties");
    let response = queue_service.get_queue_service_properties().await?;
    println!("get_queue_service_stats.response == {response:#?}");

    println!("enumerating queues starting with a");
    let response = queue_service
        .list_queues()
        .prefix("a")
        .include_metadata(true)
        .max_results(NonZeroU32::new(2u32).unwrap())
        .into_stream()
        .next()
        .await;
    println!("response == {response:#?}");

    println!("streaming queues");
    let mut stream = queue_service
        .list_queues()
        .max_results(NonZeroU32::new(3u32).unwrap())
        .into_stream();

    while let Some(value) = stream.next().await {
        let value = value?;
        let len = value.queues.len();
        println!("received {len} queues");

        value.queues.iter().for_each(|queue| println!("{queue:#?}"));
    }

    Ok(())
}

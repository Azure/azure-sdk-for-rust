use azure_core::error::Result;
use azure_storage::core::prelude::*;
use azure_storage_queues::prelude::*;
use futures::stream::StreamExt;
use std::num::NonZeroU32;

#[tokio::main]
async fn main() -> Result<()> {
    // First we retrieve the account name and master key from environment variables.
    let account =
        std::env::var("STORAGE_ACCOUNT").expect("Set env variable STORAGE_ACCOUNT first!");
    let master_key =
        std::env::var("STORAGE_MASTER_KEY").expect("Set env variable STORAGE_MASTER_KEY first!");

    let http_client = azure_core::new_http_client();

    let storage_account =
        StorageAccountClient::new_access_key(http_client.clone(), &account, &master_key);

    let queue_service = storage_account.as_queue_service_client();

    println!("getting service stats");
    let response = queue_service.get_queue_service_stats().execute().await?;
    println!("get_queue_service_properties.response == {:#?}", response);

    println!("getting service properties");
    let response = queue_service
        .get_queue_service_properties()
        .execute()
        .await?;
    println!("get_queue_service_stats.response == {:#?}", response);

    println!("enumerating queues starting with a");
    let response = queue_service
        .list_queues()
        .prefix("a")
        .include_metadata(true)
        .max_results(NonZeroU32::new(2u32).unwrap())
        .execute()
        .await?;
    println!("response == {:#?}", response);

    println!("streaming queues");
    let mut stream = Box::pin(
        queue_service
            .list_queues()
            .max_results(NonZeroU32::new(3u32).unwrap())
            .stream(),
    );

    while let Some(value) = stream.next().await {
        let value = value?;
        let len = value.queues.len();
        println!("received {} queues", len);

        value
            .queues
            .iter()
            .for_each(|queue| println!("{:#?}", queue));
    }

    Ok(())
}

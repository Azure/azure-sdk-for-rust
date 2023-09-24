#[macro_use]
extern crate log;

use azure_storage::prelude::*;
use azure_storage_queues::prelude::*;
use time::OffsetDateTime;

#[tokio::main]
async fn main() -> azure_core::Result<()> {
    // First we retrieve the account name and access key from environment variables.
    let account =
        std::env::var("STORAGE_ACCOUNT").expect("Set env variable STORAGE_ACCOUNT first!");
    let access_key =
        std::env::var("STORAGE_ACCESS_KEY").expect("Set env variable STORAGE_ACCESS_KEY first!");

    let queue_name = std::env::args()
        .nth(1)
        .expect("Please pass the queue name as first parameter");

    let storage_credentials = StorageCredentials::access_key(account.clone(), access_key);
    let queue_service = QueueServiceClient::new(account, storage_credentials);

    let queue = queue_service.queue_client(queue_name);

    trace!("putting message");
    let response = queue
        .put_message(format!(
            "Azure SDK for Rust rocks! {}",
            OffsetDateTime::now_utc()
        ))
        .await?;

    println!("response == {response:#?}");

    Ok(())
}

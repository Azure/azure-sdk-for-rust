#[macro_use]
extern crate log;

use azure_storage::core::prelude::*;
use azure_storage_queues::prelude::*;

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

    let http_client = azure_core::new_http_client();

    let storage_account = StorageClient::new_access_key(http_client.clone(), &account, &access_key);

    let queue = storage_account.queue_client(queue_name);

    trace!("putting message");
    let response = queue
        .put_message(format!("Azure SDK for Rust rocks! {}", chrono::Utc::now()))
        .into_future()
        .await?;

    println!("response == {:#?}", response);

    Ok(())
}

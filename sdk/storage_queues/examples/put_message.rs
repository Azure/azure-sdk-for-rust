#[macro_use]
extern crate log;
use azure_core::prelude::*;
use azure_storage::core::prelude::*;
use azure_storage_queues::prelude::*;
use std::error::Error;

#[tokio::main]
async fn main() -> Result<(), Box<dyn Error + Send + Sync>> {
    // First we retrieve the account name and master key from environment variables.
    let account =
        std::env::var("STORAGE_ACCOUNT").expect("Set env variable STORAGE_ACCOUNT first!");
    let master_key =
        std::env::var("STORAGE_MASTER_KEY").expect("Set env variable STORAGE_MASTER_KEY first!");

    let queue_name = std::env::args()
        .nth(1)
        .expect("Please pass the queue name as first parameter");

    let http_client = new_http_client();

    let storage_account =
        StorageAccountClient::new_access_key(http_client.clone(), &account, &master_key);

    let queue = storage_account.as_queue_client(queue_name);

    trace!("putting message");
    let response = queue
        .put_message()
        .client_request_id("optional correlation token")
        .execute(format!("Azure SDK for Rust rocks! {}", chrono::Utc::now()))
        .await?;

    println!("response == {:#?}", response);

    Ok(())
}

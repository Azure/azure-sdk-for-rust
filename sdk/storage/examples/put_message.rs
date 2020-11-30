#[macro_use]
extern crate log;
use azure_core::prelude::*;
use azure_storage::core::prelude::*;
use azure_storage::queue::prelude::*;
use std::error::Error;

#[tokio::main]
async fn main() -> Result<(), Box<dyn Error>> {
    // First we retrieve the account name and master key from environment variables.
    let account =
        std::env::var("STORAGE_ACCOUNT").expect("Set env variable STORAGE_ACCOUNT first!");
    let master_key =
        std::env::var("STORAGE_MASTER_KEY").expect("Set env variable STORAGE_MASTER_KEY first!");

    let queue_name = std::env::args()
        .nth(1)
        .expect("Please pass the queue name as first parameter");

    let client: QueueServiceClient<_> = client::with_access_key(&account, &master_key).into();

    trace!("putting message");

    let response = client
        .into_queue_name_client(&queue_name)
        .put_message(format!("Azure SDK for Rust rocks! {}", chrono::Utc::now()))
        .with_client_request_id("optional correlation token")
        .execute()
        .await?;

    println!("response == {:#?}", response);

    Ok(())
}

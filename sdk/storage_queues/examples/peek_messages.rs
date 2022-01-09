#[macro_use]
extern crate log;
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

    let options = StorageAccountOptions::default();

    let storage_account = StorageAccountClient::new_access_key(account, master_key, options);

    let queue = storage_account.as_queue_client(queue_name);
    println!("{:#?}", queue);

    trace!("peeking messages");

    let response = queue
        .peek_messages()
        .number_of_messages(2)
        .execute()
        .await?;

    println!("response == {:#?}", response);

    Ok(())
}

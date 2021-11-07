#[macro_use]
extern crate log;
use azure_core::prelude::*;
use azure_messaging_queues::prelude::*;
use azure_storage::core::prelude::*;
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

    let queue = StorageAccountClient::new_access_key(http_client.clone(), &account, &master_key)
        .as_storage_client()
        .as_queue_client(queue_name);
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

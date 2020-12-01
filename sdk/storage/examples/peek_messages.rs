#[macro_use]
extern crate log;
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

    let queue = QueueAccountClient::new(client::with_access_key(&account, &master_key))
        .into_queue_client(queue_name);

    println!("{:#?}", queue);

    trace!("peeking messages");

    let response = queue
        .peek_messages()
        .with_number_of_messages(2)
        .execute()
        .await?;

    println!("response == {:#?}", response);

    Ok(())
}

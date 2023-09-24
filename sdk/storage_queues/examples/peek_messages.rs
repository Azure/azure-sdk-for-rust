#[macro_use]
extern crate log;

use azure_storage::prelude::*;
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

    let storage_credentials = StorageCredentials::access_key(account.clone(), access_key);
    let queue_service = QueueServiceClient::new(account, storage_credentials);

    let queue = queue_service.queue_client(queue_name);
    println!("{queue:#?}");

    trace!("peeking messages");

    let response = queue.peek_messages().number_of_messages(2).await?;

    println!("response == {response:#?}");

    Ok(())
}

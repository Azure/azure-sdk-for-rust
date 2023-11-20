#[macro_use]
extern crate log;

use azure_storage::prelude::*;
use azure_storage_queues::prelude::*;
use std::time::Duration;

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

    trace!("getting messages");

    let get_response = queue
        .get_messages()
        .number_of_messages(2)
        .visibility_timeout(Duration::from_secs(5)) // the message will become visible again after 5 secs
        .await?;

    println!("get_response == {get_response:#?}");

    if get_response.messages.is_empty() {
        println!("no message to delete");
    } else {
        for message_to_delete in get_response.messages {
            println!("deleting message {message_to_delete:?}");

            let delete_response = queue.pop_receipt_client(message_to_delete).delete().await?;

            println!("delete_response == {delete_response:#?}");
        }
    }

    Ok(())
}

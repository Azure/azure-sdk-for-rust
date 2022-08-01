#[macro_use]
extern crate log;

use azure_storage::core::prelude::*;
use azure_storage_queues::prelude::*;
use std::time::Duration;
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

    let storage_account = StorageClient::new_access_key(&account, &access_key);

    let queue = storage_account.queue_client(queue_name);

    trace!("getting messages");

    let response = queue
        .get_messages()
        .number_of_messages(2)
        .visibility_timeout(Duration::from_secs(5)) // the message will become visible again after 5 secs
        .into_future()
        .await?;

    println!("response == {:#?}", response);

    let get_messages_response = queue
        .get_messages()
        .number_of_messages(2)
        .visibility_timeout(Duration::from_secs(10)) // the message will become visible again after 10 secs
        .into_future()
        .await?;
    println!("get_messages_response == {:#?}", get_messages_response);

    // we will now update the contents of the retrieved messages
    // Note that we have to specify how long the message will stay
    // "hidden" before being visible again in the queue.
    for message_to_update in get_messages_response.messages.into_iter() {
        let pop_receipt = queue.pop_receipt_client(message_to_update);

        let response = pop_receipt
            .update(
                format!("new body at {}", OffsetDateTime::now_utc()),
                Duration::from_secs(4),
            )
            .into_future()
            .await?;
        println!("response == {:#?}", response);
    }

    Ok(())
}

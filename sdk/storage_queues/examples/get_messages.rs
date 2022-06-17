#[macro_use]
extern crate log;

use azure_storage::core::prelude::*;
use azure_storage_queues::prelude::*;
use std::time::Duration;

#[tokio::main]
async fn main() -> azure_core::Result<()> {
    // First we retrieve the account name and master key from environment variables.
    let account =
        std::env::var("STORAGE_ACCOUNT").expect("Set env variable STORAGE_ACCOUNT first!");
    let master_key =
        std::env::var("STORAGE_MASTER_KEY").expect("Set env variable STORAGE_MASTER_KEY first!");

    let queue_name = std::env::args()
        .nth(1)
        .expect("Please pass the queue name as first parameter");

    let http_client = azure_core::new_http_client();

    let storage_account =
        StorageAccountClient::new_access_key(http_client.clone(), &account, &master_key);

    let queue = storage_account.as_queue_client(queue_name);

    trace!("getting messages");

    let response = queue
        .get_messages()
        .number_of_messages(2)
        .visibility_timeout(Duration::from_secs(5)) // the message will become visible again after 5 secs
        .execute()
        .await?;

    println!("response == {:#?}", response);

    let get_messages_response = queue
        .get_messages()
        .number_of_messages(2)
        .visibility_timeout(Duration::from_secs(10)) // the message will become visible again after 10 secs
        .execute()
        .await?;
    println!("get_messages_response == {:#?}", get_messages_response);

    // we will now update the contents of the retrieved messages
    // Note that we have to specify how long the message will stay
    // "hidden" before being visible again in the queue.
    for message_to_update in get_messages_response.messages.into_iter() {
        let pop_receipt = queue.as_pop_receipt_client(message_to_update);

        let response = pop_receipt
            .update(Duration::from_secs(4))
            .execute(format!("new body at {}", chrono::Utc::now()))
            .await?;
        println!("response == {:#?}", response);
    }

    Ok(())
}

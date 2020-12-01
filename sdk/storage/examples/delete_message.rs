#[macro_use]
extern crate log;
use azure_storage::core::prelude::*;
use azure_storage::queue::prelude::*;
use std::error::Error;
use std::time::Duration;

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

    let queue: QueueAccountClient<_> = client::with_access_key(&account, &master_key).into();
    let queue = queue.into_queue_client(&queue_name);

    trace!("getting messages");

    let get_response = queue
        .get_messages()
        .with_number_of_messages(2)
        .with_visibility_timeout(Duration::from_secs(5)) // the message will become visible again after 5 secs
        .execute()
        .await?;

    println!("get_response == {:#?}", get_response);

    if get_response.messages.is_empty() {
        trace!("no message to delete");
    } else {
        for message in get_response.messages {
            trace!("deleting message {}", message.message_id);

            let delete_response = queue.delete_message(message.into()).execute().await?;

            println!("delete_response == {:#?}", delete_response);
        }
    }

    Ok(())
}

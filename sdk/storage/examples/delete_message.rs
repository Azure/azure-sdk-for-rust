#[macro_use]
extern crate log;
use azure_core::prelude::*;
use azure_storage::core::prelude::*;
use azure_storage::queue::prelude::*;
use std::error::Error;
use std::sync::Arc;
use std::time::Duration;

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

    let http_client: Arc<dyn HttpClient> = Arc::new(reqwest::Client::new());

    let queue = StorageAccountClient::new_access_key(http_client.clone(), &account, &master_key)
        .as_storage_client()
        .as_queue_client(queue_name);

    trace!("getting messages");

    let get_response = queue
        .get_messages()
        .number_of_messages(2)
        .visibility_timeout(Duration::from_secs(5)) // the message will become visible again after 5 secs
        .execute()
        .await?;

    println!("get_response == {:#?}", get_response);

    if get_response.messages.is_empty() {
        println!("no message to delete");
    } else {
        for message_to_delete in get_response.messages {
            println!("deleting message {:?}", message_to_delete);

            let delete_response = queue
                .as_pop_receipt_client(message_to_delete)
                .delete()
                .execute()
                .await?;

            println!("delete_response == {:#?}", delete_response);
        }
    }

    Ok(())
}

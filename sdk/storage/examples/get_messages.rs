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

    let http_client: Arc<Box<dyn HttpClient>> = Arc::new(Box::new(reqwest::Client::new()));

    let queue = StorageAccountClient::new_access_key(http_client.clone(), &account, &master_key)
        .as_storage_client()
        .as_queue_client(queue_name);

    trace!("getting messages");

    let response = queue
        .get_messages()
        .number_of_messages(2)
        .visibility_timeout(Duration::from_secs(5)) // the message will become visible again after 5 secs
        .execute()
        .await?;

    println!("response == {:#?}", response);

    let response = queue
        .get_messages()
        .number_of_messages(1)
        .visibility_timeout(Duration::from_secs(10)) // the message will become visible again after 10 secs
        .execute()
        .await?;
    println!("response == {:#?}", response);

    let response = queue
        .update_message(Duration::from_secs(4))
        .execute(&response.messages[0], "new body") // for this example there is no check on messages returned!
        .await?;
    println!("response == {:#?}", response);

    Ok(())
}

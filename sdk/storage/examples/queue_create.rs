#[macro_use]
extern crate log;
use azure_core::prelude::*;
use azure_storage::core::prelude::*;
use azure_storage::queue::prelude::*;
use std::collections::HashMap;
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

    let queue_client = QueueAccountClient::new(client::with_access_key(&account, &master_key))
        .into_queue_client(&queue_name);

    trace!("creating queue");

    // this step is optional but here we show
    // how to add metadata to a new queue.
    let mut hm = HashMap::new();
    hm.insert("source", "azure-sdk-for-rust");

    let response = queue_client
        .create_queue()
        .with_metadata(&hm)
        .execute()
        .await?;
    println!("response == {:#?}", response);

    let mut metadata = Metadata::new();
    metadata
        .as_mut()
        .insert("version".to_owned(), "TBD".to_owned());
    metadata
        .as_mut()
        .insert("date".to_owned(), "Stardate".to_owned());

    let response = queue_client.set_queue_metadata(&metadata).execute().await?;
    println!("response == {:#?}", response);

    // now let's delete it
    let response = queue_client
        .delete_queue()
        .with_client_request_id("myclientid")
        .execute()
        .await?;
    println!("response == {:#?}", response);

    Ok(())
}

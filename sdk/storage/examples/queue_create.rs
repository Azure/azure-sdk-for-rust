#[macro_use]
extern crate log;
use azure_core::prelude::*;
use azure_storage::core::prelude::*;
use azure_storage::queue::prelude::*;
use std::error::Error;
use std::sync::Arc;

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

    let storage_account_client =
        StorageAccountClient::new_access_key(http_client.clone(), &account, &master_key);
    let queue = storage_account_client
        .as_storage_client()
        .as_queue_client(queue_name);

    trace!("creating queue");

    // this step is optional but here we show
    // how to add metadata to a new queue.
    let mut metadata = Metadata::new();
    metadata
        .as_mut()
        .insert("source".into(), "azure-sdk-for-rust".into());
    metadata
        .as_mut()
        .insert("created".into(), format!("{:?}", chrono::Utc::now()).into());

    let response = queue.create().metadata(&metadata).execute().await?;
    println!("response == {:#?}", response);

    // let's add some more metadata
    metadata.insert("version".to_owned(), "TBD".to_owned());
    metadata.insert("updated".to_owned(), format!("{:?}", chrono::Utc::now()));

    println!("metadata == {:#?}", metadata);

    let response = queue.set_metadata(&metadata).execute().await?;
    println!("response == {:#?}", response);

    // now let's delete it
    let response = queue
        .delete()
        .client_request_id("myclientid")
        .execute()
        .await?;
    println!("response == {:#?}", response);

    Ok(())
}

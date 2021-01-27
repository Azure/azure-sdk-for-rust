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

    let client: QueueAccountClient<_> = client::with_access_key(&account, &master_key).into();

    trace!("enumerating queues");

    let response = client
        .list_queues()
        .with_prefix("a")
        .with_include_metadata()
        .with_max_results(2)
        .execute()
        .await?;

    println!("response == {:#?}", response);

    Ok(())
}

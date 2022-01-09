use azure_storage::core::prelude::*;
use std::error::Error;

#[tokio::main]
async fn main() -> Result<(), Box<dyn Error + Send + Sync>> {
    // First we retrieve the account name and master key from environment variables.
    let account =
        std::env::var("STORAGE_ACCOUNT").expect("Set env variable STORAGE_ACCOUNT first!");
    let master_key =
        std::env::var("STORAGE_MASTER_KEY").expect("Set env variable STORAGE_MASTER_KEY first!");

    let options = StorageAccountOptions::default();

    let storage_client =
        StorageAccountClient::new_access_key(&account, &master_key, options).as_storage_client();

    let response = storage_client.get_account_information().execute().await?;
    println!("{:?}", response);

    Ok(())
}

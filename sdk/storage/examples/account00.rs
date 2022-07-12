use azure_storage::core::prelude::*;

#[tokio::main]
async fn main() -> azure_core::Result<()> {
    // First we retrieve the account name and access key from environment variables.
    let account =
        std::env::var("STORAGE_ACCOUNT").expect("Set env variable STORAGE_ACCOUNT first!");
    let access_key =
        std::env::var("STORAGE_ACCESS_KEY").expect("Set env variable STORAGE_ACCESS_KEY first!");

    let storage_client =
        StorageClient::new_access_key(&account, &access_key, StorageOptions::default());

    let response = storage_client
        .get_account_information()
        .into_future()
        .await?;
    println!("{:?}", response);

    Ok(())
}

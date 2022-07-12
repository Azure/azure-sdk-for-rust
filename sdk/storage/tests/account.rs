#![cfg(all(test, feature = "test_e2e"))]
use azure_storage::core::prelude::*;

#[tokio::test]
async fn get_account_information() {
    let account =
        std::env::var("STORAGE_ACCOUNT").expect("Set env variable STORAGE_ACCOUNT first!");
    let access_key =
        std::env::var("STORAGE_ACCESS_KEY").expect("Set env variable STORAGE_ACCESS_KEY first!");

    let storage_client =
        StorageClient::new_access_key(&account, &access_key, StorageOptions::default());

    storage_client
        .get_account_information()
        .into_future()
        .await
        .unwrap();
}

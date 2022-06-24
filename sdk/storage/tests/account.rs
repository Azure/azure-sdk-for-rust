#![cfg(all(test, feature = "test_e2e"))]
use azure_storage::core::prelude::*;

#[tokio::test]
async fn get_account_information() {
    let account =
        std::env::var("STORAGE_ACCOUNT").expect("Set env variable STORAGE_ACCOUNT first!");
    let access_key =
        std::env::var("STORAGE_ACCESS_KEY").expect("Set env variable STORAGE_ACCESS_KEY first!");

    let http_client = azure_core::new_http_client();

    let storage_client =
        StorageAccountClient::new_access_key(http_client.clone(), &account, &access_key)
            .storage_client();

    storage_client
        .get_account_information()
        .into_future()
        .await
        .unwrap();
}

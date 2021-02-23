#![cfg(all(test, feature = "test_e2e"))]
use azure_core::prelude::*;
use azure_storage::core::prelude::*;
use std::sync::Arc;

#[tokio::test]
async fn get_account_information() {
    let account =
        std::env::var("STORAGE_ACCOUNT").expect("Set env variable STORAGE_ACCOUNT first!");
    let master_key =
        std::env::var("STORAGE_MASTER_KEY").expect("Set env variable STORAGE_MASTER_KEY first!");

    let http_client: Arc<Box<dyn HttpClient>> = Arc::new(Box::new(reqwest::Client::new()));

    let storage_client =
        StorageAccountClient::new_access_key(http_client.clone(), &account, &master_key)
            .as_storage_client();

    storage_client
        .get_account_information()
        .execute()
        .await
        .unwrap();
}

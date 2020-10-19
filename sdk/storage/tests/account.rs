#![cfg(all(test, feature = "test_e2e"))]
use azure_storage::account::prelude::*;
use azure_storage::core::prelude::*;

#[tokio::test]
async fn get_account_information() {
    let account =
        std::env::var("STORAGE_ACCOUNT").expect("Set env variable STORAGE_ACCOUNT first!");
    let master_key =
        std::env::var("STORAGE_MASTER_KEY").expect("Set env variable STORAGE_MASTER_KEY first!");

    let client = client::with_access_key(&account, &master_key);

    // we just test the call works, we don't check the return value since the
    // values depend on the Azure storage account
    client.get_account_information().finalize().await.unwrap();
}

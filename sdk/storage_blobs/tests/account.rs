#![cfg(all(test, feature = "test_e2e"))]
use azure_storage::prelude::*;
use azure_storage_blobs::prelude::*;

#[tokio::test]
async fn get_account_information() {
    let account =
        std::env::var("STORAGE_ACCOUNT").expect("Set env variable STORAGE_ACCOUNT first!");
    let access_key =
        std::env::var("STORAGE_ACCESS_KEY").expect("Set env variable STORAGE_ACCESS_KEY first!");

    let storage_credentials = StorageCredentials::access_key(account.clone(), access_key);
    let blob_service = BlobServiceClient::new(account, storage_credentials);

    blob_service.get_account_information().await.unwrap();
}

#![cfg(all(test, feature = "test_e2e"))]
use azure_storage::prelude::*;
use azure_storage_blobs::{container::PublicAccess, prelude::*};
use std::time::Duration;

#[tokio::test]
async fn lease() {
    let container_name: &'static str = "azuresdkrustetoets2";

    let storage = initialize();
    let container = storage.container_client(container_name);

    container
        .create()
        .public_access(PublicAccess::None)
        .await
        .unwrap();

    let res = container
        .acquire_lease(Duration::from_secs(30))
        .await
        .unwrap();
    let lease_id = res.lease_id;
    let lease = container.container_lease_client(lease_id);

    let _res = lease.renew().await.unwrap();
    let _res = lease.release().await.unwrap();

    container.delete().await.unwrap();
}

#[tokio::test]
async fn break_lease() {
    let container_name: &'static str = "azuresdkrustetoets3";

    let storage = initialize();
    let container = storage.container_client(container_name);

    container
        .create()
        .public_access(PublicAccess::None)
        .await
        .unwrap();

    let res = container
        .acquire_lease(Duration::from_secs(30))
        .await
        .unwrap();

    let lease = container.container_lease_client(res.lease_id);
    lease.renew().await.unwrap();

    let res = container
        .break_lease()
        .lease_break_period(Duration::from_secs(0))
        .await
        .unwrap();
    assert!(res.lease_time == 0);

    container.delete().await.unwrap();
}

fn initialize() -> BlobServiceClient {
    let account =
        std::env::var("STORAGE_ACCOUNT").expect("Set env variable STORAGE_ACCOUNT first!");
    let access_key =
        std::env::var("STORAGE_ACCESS_KEY").expect("Set env variable STORAGE_ACCESS_KEY first!");

    let storage_credentials = StorageCredentials::access_key(account.clone(), access_key);
    BlobServiceClient::new(account, storage_credentials)
}

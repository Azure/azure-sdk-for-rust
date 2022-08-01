#![cfg(all(test, feature = "test_e2e"))]
use azure_storage::core::prelude::*;
use azure_storage_blobs::{container::PublicAccess, prelude::*};
use time::Duration;

#[tokio::test]
async fn lease() {
    let container_name: &'static str = "azuresdkrustetoets2";

    let storage = initialize();
    let container = storage.container_client(container_name);

    container
        .create()
        .public_access(PublicAccess::None)
        .into_future()
        .await
        .unwrap();

    let res = container
        .acquire_lease(Duration::seconds(30))
        .into_future()
        .await
        .unwrap();
    let lease_id = res.lease_id;
    let lease = container.container_lease_client(lease_id);

    let _res = lease.renew().into_future().await.unwrap();
    let _res = lease.release().into_future().await.unwrap();

    container.delete().into_future().await.unwrap();
}

#[tokio::test]
async fn break_lease() {
    let container_name: &'static str = "azuresdkrustetoets3";

    let storage = initialize();
    let container = storage.container_client(container_name);

    container
        .create()
        .public_access(PublicAccess::None)
        .into_future()
        .await
        .unwrap();

    let res = container
        .acquire_lease(Duration::seconds(30))
        .into_future()
        .await
        .unwrap();

    let lease = container.container_lease_client(res.lease_id);
    lease.renew().into_future().await.unwrap();

    let res = container
        .break_lease()
        .lease_break_period(Duration::seconds(0))
        .into_future()
        .await
        .unwrap();
    assert!(res.lease_time == 0);

    container.delete().into_future().await.unwrap();
}

fn initialize() -> StorageClient {
    let account =
        std::env::var("STORAGE_ACCOUNT").expect("Set env variable STORAGE_ACCOUNT first!");
    let access_key =
        std::env::var("STORAGE_ACCESS_KEY").expect("Set env variable STORAGE_ACCESS_KEY first!");

    StorageClient::new_access_key(&account, &access_key)
}

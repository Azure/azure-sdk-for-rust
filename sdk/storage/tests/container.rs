#![cfg(all(test, feature = "test_e2e"))]
use azure_core::prelude::*;
use azure_storage::blob::container::PublicAccess;
use azure_storage::blob::prelude::*;
use azure_storage::core::prelude::*;
use std::sync::Arc;
use std::time::Duration;

#[tokio::test]
async fn lease() {
    let container_name: &'static str = "azuresdkrustetoets2";

    let storage = initialize().as_storage_client();
    let container = storage.as_container_client(container_name);

    container
        .create()
        .public_access(PublicAccess::None)
        .execute()
        .await
        .unwrap();

    let res = container
        .acquire_lease(Duration::from_secs(30))
        .execute()
        .await
        .unwrap();
    let lease_id = res.lease_id;
    let lease = container.as_container_lease_client(lease_id);

    let _res = lease.renew().execute().await.unwrap();
    let _res = lease.release().execute().await.unwrap();

    container.delete().execute().await.unwrap();
}

#[tokio::test]
async fn break_lease() {
    let container_name: &'static str = "azuresdkrustetoets3";

    let storage = initialize().as_storage_client();
    let container = storage.as_container_client(container_name);

    container
        .create()
        .public_access(PublicAccess::None)
        .execute()
        .await
        .unwrap();

    let res = container
        .acquire_lease(Duration::from_secs(30))
        .execute()
        .await
        .unwrap();

    let lease = container.as_container_lease_client(res.lease_id);
    lease.renew().execute().await.unwrap();

    let res = container
        .break_lease()
        .lease_break_period(Duration::from_secs(0))
        .execute()
        .await
        .unwrap();
    assert!(res.lease_time == 0);

    container.delete().execute().await.unwrap();
}

fn initialize() -> Arc<StorageAccountClient> {
    let account =
        std::env::var("STORAGE_ACCOUNT").expect("Set env variable STORAGE_ACCOUNT first!");
    let master_key =
        std::env::var("STORAGE_MASTER_KEY").expect("Set env variable STORAGE_MASTER_KEY first!");

    let http_client = new_http_client();

    StorageAccountClient::new_access_key(http_client.clone(), &account, &master_key)
}

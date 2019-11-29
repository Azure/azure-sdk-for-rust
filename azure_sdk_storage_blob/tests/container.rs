#![cfg(all(test, feature = "test_e2e"))]
use azure_sdk_core::errors::AzureError;
use azure_sdk_core::{
    ContainerNameSupport, LeaseBreakPeriodSupport, LeaseDurationSupport, LeaseIdSupport,
};
use azure_sdk_storage_blob::container::{PublicAccess, PublicAccessSupport};
use azure_sdk_storage_blob::prelude::*;
use azure_sdk_storage_core::prelude::*;

#[tokio::test]
async fn lease() {
    let container_name: &'static str = "azuresdkrustetoets2";

    let client = initialize().unwrap();
    client
        .create_container()
        .with_container_name(container_name)
        .with_public_access(PublicAccess::Container)
        .finalize()
        .await
        .unwrap();

    let res = client
        .acquire_container_lease()
        .with_container_name(container_name)
        .with_lease_duration(30)
        .finalize()
        .await
        .unwrap();
    let lease_id = res.lease_id;

    let _res = client
        .renew_container_lease()
        .with_container_name(container_name)
        .with_lease_id(&lease_id)
        .finalize()
        .await
        .unwrap();

    client
        .release_container_lease()
        .with_container_name(container_name)
        .with_lease_id(&lease_id)
        .finalize()
        .await
        .unwrap();

    client
        .delete_container()
        .with_container_name(container_name)
        .finalize()
        .await
        .unwrap();
}

#[tokio::test]
async fn break_lease() {
    let container_name: &'static str = "azuresdkrustetoets3";

    let client = initialize().unwrap();
    client
        .create_container()
        .with_container_name(container_name)
        .with_public_access(PublicAccess::Container)
        .finalize()
        .await
        .unwrap();

    let _res = client
        .acquire_container_lease()
        .with_container_name(container_name)
        .with_lease_duration(30)
        .finalize()
        .await
        .unwrap();

    let res = client
        .break_container_lease()
        .with_container_name(container_name)
        .with_lease_break_period(0)
        .finalize()
        .await
        .unwrap();
    assert!(res.lease_time == 0);

    client
        .delete_container()
        .with_container_name(container_name)
        .finalize()
        .await
        .unwrap();
}

fn initialize() -> Result<Client, AzureError> {
    let account =
        std::env::var("STORAGE_ACCOUNT").expect("Set env variable STORAGE_ACCOUNT first!");
    let master_key =
        std::env::var("STORAGE_MASTER_KEY").expect("Set env variable STORAGE_MASTER_KEY first!");

    Ok(Client::new(&account, &master_key)?)
}

#![cfg(all(test, feature = "test_e2e"))]
use azure_sdk_core::errors::AzureError;
use azure_sdk_core::{ContainerNameSupport, LeaseBreakPeriodSupport, LeaseDurationSupport, LeaseIdSupport};
use azure_sdk_storage_blob::container::{PublicAccess, PublicAccessSupport};
use azure_sdk_storage_blob::prelude::*;
use azure_sdk_storage_core::prelude::*;
use tokio_core::reactor::Core;

#[test]
fn lease() {
    let container_name: &'static str = "azuresdkrustetoets2";

    let (client, mut core) = initialize().unwrap();
    core.run(
        client
            .create_container()
            .with_container_name(container_name)
            .with_public_access(PublicAccess::Container)
            .finalize(),
    )
    .unwrap();

    let future = client
        .acquire_container_lease()
        .with_container_name(container_name)
        .with_lease_duration(30)
        .finalize();
    let res = core.run(future).unwrap();
    let lease_id = res.lease_id;

    let future = client
        .renew_container_lease()
        .with_container_name(container_name)
        .with_lease_id(&lease_id)
        .finalize();
    let _res = core.run(future).unwrap();

    let future = client
        .release_container_lease()
        .with_container_name(container_name)
        .with_lease_id(&lease_id)
        .finalize();
    core.run(future).unwrap();

    let cont_delete = client.delete_container().with_container_name(container_name).finalize();

    core.run(cont_delete).unwrap();
}

#[test]
fn break_lease() {
    let container_name: &'static str = "azuresdkrustetoets3";

    let (client, mut core) = initialize().unwrap();
    core.run(
        client
            .create_container()
            .with_container_name(container_name)
            .with_public_access(PublicAccess::Container)
            .finalize(),
    )
    .unwrap();

    let future = client
        .acquire_container_lease()
        .with_container_name(container_name)
        .with_lease_duration(30)
        .finalize();
    let _res = core.run(future).unwrap();

    let future = client
        .break_container_lease()
        .with_container_name(container_name)
        .with_lease_break_period(0)
        .finalize();
    let res = core.run(future).unwrap();
    assert!(res.lease_time == 0);

    let cont_delete = client.delete_container().with_container_name(container_name).finalize();

    core.run(cont_delete).unwrap();
}

fn initialize() -> Result<(Client, Core), AzureError> {
    let account = std::env::var("STORAGE_ACCOUNT").expect("Set env variable STORAGE_ACCOUNT first!");
    let master_key = std::env::var("STORAGE_MASTER_KEY").expect("Set env variable STORAGE_MASTER_KEY first!");
    let core = Core::new()?;

    Ok((Client::new(&account, &master_key)?, core))
}

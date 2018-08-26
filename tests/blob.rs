#![cfg(all(test, feature = "test_e2e"))]
extern crate azure_sdk_for_rust;
extern crate chrono;
extern crate env_logger;
extern crate futures;
extern crate hyper;
extern crate hyper_tls;
extern crate tokio_core;
#[macro_use]
extern crate log;
extern crate md5;
extern crate serde;
extern crate uuid;

use azure_sdk_for_rust::core::errors::AzureError;
use azure_sdk_for_rust::core::DeleteSnapshotsMethod;
use azure_sdk_for_rust::prelude::*;
use azure_sdk_for_rust::storage::{
    blob::BlockListType,
    client::Client,
    container::{Container, PublicAccess, PublicAccessSupport},
};
use chrono::{Duration, FixedOffset, Utc};
use futures::Future;
use std::ops::Add;
use std::ops::Deref;
use tokio_core::reactor::Core;
use uuid::Uuid;

#[test]
fn create_and_delete_container() {
    use azure_sdk_for_rust::storage::client::Container;

    let name: &'static str = "azuresdkrustetoets";

    let (client, mut core) = initialize().unwrap();
    core.run(
        client
            .create_container()
            .with_container_name(name)
            .with_public_access(PublicAccess::Container)
            .finalize(),
    ).unwrap();

    // get acl without stored access policy list
    let future = client.get_container_acl().with_container_name(name).finalize();
    let _result = core.run(future).unwrap();

    // set stored acess policy list
    let dt_start = Utc::now().with_timezone(&FixedOffset::east(0));
    let dt_end = dt_start.add(Duration::days(7));

    let mut sapl = StoredAccessPolicyList::default();
    sapl.stored_access.push(StoredAccessPolicy::new("pollo", dt_start, dt_end, "rwd"));

    let future = client
        .set_container_acl()
        .with_container_name(name)
        .with_public_access(PublicAccess::Blob)
        .with_stored_access_policy_list(&sapl)
        .finalize();

    let _result = core.run(future).unwrap();

    // now we get back the acess policy list and compare to the one created
    let future = client.get_container_acl().with_container_name(name).finalize();
    let result = core.run(future).unwrap();

    assert!(result.public_access == PublicAccess::Blob);
    // we cannot compare the returned result because Azure will
    // trim the milliseconds
    // assert!(sapl == result.stored_access_policy_list);
    assert!(sapl.stored_access.len() == result.stored_access_policy_list.stored_access.len());
    for (i1, i2) in sapl.stored_access.iter().zip(result.stored_access_policy_list.stored_access.iter()) {
        assert!(i1.id == i2.id);
        assert!(i1.permission == i2.permission);
    }

    let future = client.get_container_properties().with_container_name(name).finalize();
    let res = core.run(future).unwrap();
    assert!(res.container.public_access == PublicAccess::Blob);

    let list = core.run(client.list_containers().with_prefix(name).finalize()).unwrap();
    let cont_list: Vec<&azure_sdk_for_rust::storage::container::Container> =
        list.incomplete_vector.deref().into_iter().filter(|e| e.name == name).collect();

    if cont_list.len() != 1 {
        panic!("More than 1 container returned with the same name!");
    }

    let future = client
        .acquire_container_lease()
        .with_container_name(&cont_list[0].name)
        .with_lease_duration(30)
        .finalize();
    let res = core.run(future).unwrap();
    let lease_id = res.lease_id;

    let future = client
        .renew_container_lease()
        .with_container_name(&cont_list[0].name)
        .with_lease_id(&lease_id)
        .finalize();
    let _res = core.run(future).unwrap();

    let cont_delete = client
        .delete_container()
        .with_container_name(&cont_list[0].name)
        .with_lease_id(&lease_id) // must pass the lease here too
        .finalize();

    core.run(cont_delete).unwrap();
}

#[test]
fn put_and_get_block_list() {
    use azure_sdk_for_rust::storage::client::Container as ContainerTrait;

    let u = Uuid::new_v4();
    let container = Container::new(&format!("sdkrust{}", u));
    let name = "asdkrustputblock.txt";

    let (client, mut core) = initialize().unwrap();

    core.run(
        client
            .create_container()
            .with_container_name(&container.name)
            .with_public_access(PublicAccess::Container)
            .finalize(),
    ).expect("container already present");

    let contents1 = "AAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAA";
    let contents2 = "BBBBBBBBBBBBBBBBBBBBBBBBBBBBBBBBBBBBBBBBBBBBBBBBBBBBBBBBBBBBBB";
    let contents3 = "CCCCCCCCCCCCCCCCCCCCCCCCCCCCCCCCCCCCCCCCCCCCCCCCCCCCCCCCCCCCCC";

    let future = client
        .put_block()
        .with_container_name(&container.name)
        .with_blob_name(name)
        .with_body(&contents1.as_bytes())
        .with_block_id(b"block1")
        .finalize()
        .and_then(|_| {
            client
                .put_block()
                .with_container_name(&container.name)
                .with_blob_name(name)
                .with_body(&contents2.as_bytes())
                .with_block_id(b"block2")
                .finalize()
        }).and_then(|_| {
            client
                .put_block()
                .with_container_name(&container.name)
                .with_blob_name(name)
                .with_body(&contents3.as_bytes())
                .with_block_id(b"block3")
                .finalize()
        });

    core.run(future).unwrap();

    let future = client
        .get_block_list()
        .with_container_name(&container.name)
        .with_blob_name(name)
        .with_block_list_type(BlockListType::All)
        .finalize();

    let received_block_list = core.run(future).unwrap();

    let future = client
        .put_block_list()
        .with_container_name(&container.name)
        .with_blob_name(name)
        .with_block_list(&received_block_list.block_with_size_list.into())
        .finalize();
    core.run(future).unwrap();

    let future = client
        .acquire_blob_lease()
        .with_container_name(&container.name)
        .with_blob_name(name)
        .with_lease_duration(60)
        .finalize();
    let res = core.run(future).unwrap();
    println!("Acquire lease == {:?}", res);

    let lease_id = res.lease_id;

    let future = client
        .renew_blob_lease()
        .with_container_name(&container.name)
        .with_blob_name(name)
        .with_lease_id(&lease_id)
        .finalize();
    let res = core.run(future).unwrap();
    println!("Renew lease == {:?}", res);

    let future = client
        .break_blob_lease()
        .with_container_name(&container.name)
        .with_blob_name(name)
        .with_lease_break_period(15)
        .finalize();
    let res = core.run(future).unwrap();
    println!("Break lease == {:?}", res);

    let future = client
        .release_blob_lease()
        .with_container_name(&container.name)
        .with_blob_name(name)
        .with_lease_id(&lease_id)
        .finalize();
    let res = core.run(future).unwrap();
    println!("Release lease == {:?}", res);

    let future = client
        .delete_blob()
        .with_container_name(&container.name)
        .with_blob_name(name)
        .with_delete_snapshots_method(DeleteSnapshotsMethod::Include)
        .finalize();
    let res = core.run(future).unwrap();
    println!("Delete blob == {:?}", res);

    core.run(
        client
            .delete_container()
            .with_container_name(container.as_ref())
            .finalize()
            .map(|_| println!("container {} deleted!", container.name)),
    ).unwrap();
}

#[test]
fn list_containers() {
    use azure_sdk_for_rust::storage::client::Container as ContainerTrait;

    let (client, mut core) = initialize().unwrap();

    trace!("running list_containers");

    let mut next_marker: Option<String> = None;

    loop {
        let ret = {
            let builder = client.list_containers().with_max_results(2);
            if let Some(nm) = next_marker {
                core.run(builder.with_next_marker(&nm).finalize()).unwrap()
            } else {
                core.run(builder.finalize()).unwrap()
            }
        };

        trace!("ret {:?}\n\n", ret);
        if !ret.is_complete() {
            next_marker = Some(ret.incomplete_vector.token().unwrap().to_owned());
        } else {
            break;
        }
    }
}

#[test]
fn put_blob() {
    use azure_sdk_for_rust::storage::client::Blob as BlobTrait;
    use azure_sdk_for_rust::storage::client::Container as ContainerTrait;

    let (client, mut core) = initialize().unwrap();

    let blob_name: &'static str = "m1";
    let container_name: &'static str = "rust-upload-test";
    let data = b"abcdef";

    if core
        .run(client.list_containers().finalize())
        .unwrap()
        .incomplete_vector
        .iter()
        .find(|x| x.name == container_name)
        .is_none()
    {
        core.run(
            client
                .create_container()
                .with_container_name(container_name)
                .with_public_access(PublicAccess::Blob)
                .finalize(),
        ).unwrap();
    }

    // calculate md5 too!
    let digest = md5::compute(&data[..]);

    let future = client
        .put_block_blob()
        .with_container_name(&container_name)
        .with_blob_name(&blob_name)
        .with_content_type("text/plain")
        .with_body(&data[..])
        .with_content_md5(&digest[..])
        .finalize();

    core.run(future).unwrap();

    trace!("created {:?}", blob_name);
}

fn initialize() -> Result<(Client, Core), AzureError> {
    let account = std::env::var("STORAGE_ACCOUNT").expect("Set env variable STORAGE_ACCOUNT first!");
    let master_key = std::env::var("STORAGE_MASTER_KEY").expect("Set env variable STORAGE_MASTER_KEY first!");
    let core = Core::new()?;

    Ok((Client::new(&account, &master_key)?, core))
}

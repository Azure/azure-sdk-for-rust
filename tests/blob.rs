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
extern crate serde;
extern crate uuid;

use azure_sdk_for_rust::core::{
    errors::AzureError,
    lease::{LeaseState, LeaseStatus},
};
use azure_sdk_for_rust::core::{ContainerNameSupport, NextMarkerSupport, PrefixSupport, StoredAccessPolicy, StoredAccessPolicyList};
use azure_sdk_for_rust::storage::{
    blob::{get_block_list, put_block_list, Blob, BlobType, BlockListType, PUT_BLOCK_OPTIONS_DEFAULT, PUT_OPTIONS_DEFAULT},
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
            .create()
            .with_container_name(name)
            .with_public_access(PublicAccess::Container)
            .finalize(),
    ).unwrap();

    // get acl without stored access policy list
    let future = client.get_acl().with_container_name(name).finalize();
    let _result = core.run(future).unwrap();

    // set stored acess policy list
    let dt_start = Utc::now().with_timezone(&FixedOffset::east(0));
    let dt_end = dt_start.add(Duration::days(7));

    let mut sapl = StoredAccessPolicyList::default();
    sapl.stored_access.push(StoredAccessPolicy::new("pollo", dt_start, dt_end, "rwd"));

    let future = client
        .set_acl()
        .with_container_name(name)
        .with_public_access(PublicAccess::Blob)
        .with_stored_access_policy_list(&sapl)
        .finalize();

    let _result = core.run(future).unwrap();

    // now we get back the acess policy list and compare to the one created
    let future = client.get_acl().with_container_name(name).finalize();
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

    let list = core.run(client.list().with_prefix(name).finalize()).unwrap();
    let cont_list: Vec<&azure_sdk_for_rust::storage::container::Container> = list.deref().into_iter().filter(|e| e.name == name).collect();

    if cont_list.len() != 1 {
        panic!("More than 1 container returned with the same name!");
    }

    let cont_delete = client.delete().with_container_name(&cont_list[0].name).finalize();

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
            .create()
            .with_container_name(&container.name)
            .with_public_access(PublicAccess::Container)
            .finalize(),
    ).expect("container already present");

    let contents1 = "AAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAA";
    let contents2 = "BBBBBBBBBBBBBBBBBBBBBBBBBBBBBBBBBBBBBBBBBBBBBBBBBBBBBBBBBBBBBB";
    let contents3 = "CCCCCCCCCCCCCCCCCCCCCCCCCCCCCCCCCCCCCCCCCCCCCCCCCCCCCCCCCCCCCC";

    let new_blob = Blob {
        name: name.to_owned(),
        container_name: container.name.to_owned(),
        snapshot_time: None,
        last_modified: chrono::Utc::now(),
        etag: "".to_owned(),
        content_length: 0,
        content_type: Some("text/plain".to_owned()),
        content_encoding: None,
        content_language: None,
        content_md5: None,
        cache_control: None,
        x_ms_blob_sequence_number: None,
        blob_type: BlobType::BlockBlob,
        lease_status: LeaseStatus::Unlocked,
        lease_state: LeaseState::Available,
        lease_duration: None,
        copy_id: None,
        copy_status: None,
        copy_source: None,
        copy_progress: None,
        copy_completion: None,
        copy_status_description: None,
    };

    let future = new_blob
        .put_block(&client, "block1", &PUT_BLOCK_OPTIONS_DEFAULT, &contents1.as_bytes())
        .and_then(|_| new_blob.put_block(&client, "block2", &PUT_BLOCK_OPTIONS_DEFAULT, &contents2.as_bytes()))
        .and_then(|_| new_blob.put_block(&client, "block3", &PUT_BLOCK_OPTIONS_DEFAULT, &contents3.as_bytes()));

    core.run(future).unwrap();

    let container_name = container.name.clone();
    let future = get_block_list(
        &client,
        &(&container_name as &str, name),
        &BlockListType::All,
        None,
        None,
        None,
        None,
    );

    let received_block_list = core.run(future).unwrap();

    let future = put_block_list(
        &client,
        &(&container_name as &str, name),
        None,
        None,
        &received_block_list.block_list.into(),
    );
    core.run(future).unwrap();

    let future = Blob::delete(&client, &container.name, &name, None).map(|_| println!("Blob deleted!"));
    core.run(future).unwrap();

    core.run(
        client
            .delete()
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
            let builder = client.list().with_max_results(2);
            if let Some(nm) = next_marker {
                core.run(builder.with_next_marker(&nm).finalize()).unwrap()
            } else {
                core.run(builder.finalize()).unwrap()
            }
        };

        trace!("ret {:?}\n\n", ret);
        if !ret.is_complete() {
            next_marker = Some(ret.token().unwrap().to_owned());
        } else {
            break;
        }
    }
}

#[test]
fn put_blob() {
    use azure_sdk_for_rust::storage::client::Container as ContainerTrait;

    let (client, mut core) = initialize().unwrap();

    let blob_name: &'static str = "m1";
    let container_name: &'static str = "rust-upload-test";
    let value = "abcdef";

    if core
        .run(client.list().finalize())
        .unwrap()
        .iter()
        .find(|x| x.name == container_name)
        .is_none()
    {
        core.run(
            client
                .create()
                .with_container_name(container_name)
                .with_public_access(PublicAccess::Blob)
                .finalize(),
        ).unwrap();
    }

    let new_blob = Blob {
        name: blob_name.to_owned(),
        container_name: container_name.to_owned(),
        snapshot_time: None,
        last_modified: Utc::now(),
        etag: "".to_owned(),
        content_length: value.as_bytes().len() as u64,
        content_type: Some("application/octet-stream".to_owned()),
        content_encoding: None,
        content_language: None,
        content_md5: None,
        cache_control: None,
        x_ms_blob_sequence_number: None,
        blob_type: BlobType::BlockBlob,
        lease_status: LeaseStatus::Unlocked,
        lease_state: LeaseState::Available,
        lease_duration: None,
        copy_id: None,
        copy_status: None,
        copy_source: None,
        copy_progress: None,
        copy_completion: None,
        copy_status_description: None,
    };

    core.run(new_blob.put(&client, &PUT_OPTIONS_DEFAULT, Some(&value.as_bytes())))
        .unwrap();

    trace!("created {:?}", new_blob);
}

fn initialize() -> Result<(Client, Core), AzureError> {
    let account = std::env::var("STORAGE_ACCOUNT").expect("Set env variable STORAGE_ACCOUNT first!");
    let master_key = std::env::var("STORAGE_MASTER_KEY").expect("Set env variable STORAGE_MASTER_KEY first!");
    let core = Core::new()?;

    Ok((Client::new(&account, &master_key)?, core))
}

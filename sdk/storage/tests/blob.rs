#![cfg(all(test, feature = "test_e2e"))]
#[macro_use]
extern crate log;

use azure_core::prelude::*;
use azure_core::{Consistency, DeleteSnapshotsMethod};
use azure_storage::blob::{
    blob::BlockListType,
    container::{Container, PublicAccess, PublicAccessSupport},
    prelude::*,
};
use azure_storage::core::prelude::*;
use chrono::{Duration, FixedOffset, Utc};
use std::ops::Add;
use std::ops::Deref;
use uuid::Uuid;

#[tokio::test]
async fn create_and_delete_container() {
    let name: &'static str = "azuresdkrustetoets";

    let client = initialize();
    client
        .create_container()
        .with_container_name(name)
        .with_public_access(PublicAccess::Container)
        .finalize()
        .await
        .unwrap();

    // get acl without stored access policy list
    let _result = client
        .get_container_acl()
        .with_container_name(name)
        .finalize()
        .await
        .unwrap();

    // set stored acess policy list
    let dt_start = Utc::now().with_timezone(&FixedOffset::east(0));
    let dt_end = dt_start.add(Duration::days(7));

    let mut sapl = StoredAccessPolicyList::default();
    sapl.stored_access
        .push(StoredAccessPolicy::new("pollo", dt_start, dt_end, "rwd"));

    let _result = client
        .set_container_acl()
        .with_container_name(name)
        .with_public_access(PublicAccess::Blob)
        .with_stored_access_policy_list(&sapl)
        .finalize()
        .await
        .unwrap();

    // now we get back the acess policy list and compare to the one created
    let result = client
        .get_container_acl()
        .with_container_name(name)
        .finalize()
        .await
        .unwrap();

    assert!(result.public_access == PublicAccess::Blob);
    // we cannot compare the returned result because Azure will
    // trim the milliseconds
    // assert!(sapl == result.stored_access_policy_list);
    assert!(sapl.stored_access.len() == result.stored_access_policy_list.stored_access.len());
    for (i1, i2) in sapl
        .stored_access
        .iter()
        .zip(result.stored_access_policy_list.stored_access.iter())
    {
        assert!(i1.id == i2.id);
        assert!(i1.permission == i2.permission);
    }

    let res = client
        .get_container_properties()
        .with_container_name(name)
        .finalize()
        .await
        .unwrap();
    assert!(res.container.public_access == PublicAccess::Blob);

    let list = client
        .list_containers()
        .with_prefix(name)
        .finalize()
        .await
        .unwrap();
    let cont_list: Vec<&Container> = list
        .incomplete_vector
        .deref()
        .into_iter()
        .filter(|e| e.name == name)
        .collect();

    if cont_list.len() != 1 {
        panic!("More than 1 container returned with the same name!");
    }

    let res = client
        .acquire_container_lease()
        .with_container_name(&cont_list[0].name)
        .with_lease_duration(30)
        .finalize()
        .await
        .unwrap();
    let lease_id = res.lease_id;

    let _res = client
        .renew_container_lease()
        .with_container_name(&cont_list[0].name)
        .with_lease_id(&lease_id)
        .finalize()
        .await
        .unwrap();

    client
        .delete_container()
        .with_container_name(&cont_list[0].name)
        .with_lease_id(&lease_id) // must pass the lease here too
        .finalize()
        .await
        .unwrap();
}

#[tokio::test]
async fn put_and_get_block_list() {
    let u = Uuid::new_v4();
    let container = Container::new(&format!("sdkrust{}", u));
    let name = "asd - ()krustputblock.txt";

    let client = initialize();

    client
        .create_container()
        .with_container_name(&container.name)
        .with_public_access(PublicAccess::Container)
        .finalize()
        .await
        .expect("container already present");

    let contents1 = "AAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAA";
    let contents2 = "BBBBBBBBBBBBBBBBBBBBBBBBBBBBBBBBBBBBBBBBBBBBBBBBBBBBBBBBBBBBBB";
    let contents3 = "CCCCCCCCCCCCCCCCCCCCCCCCCCCCCCCCCCCCCCCCCCCCCCCCCCCCCCCCCCCCCC";

    let digest3 = md5::compute(contents3);

    let put_block_response = client
        .put_block()
        .with_container_name(&container.name)
        .with_blob_name(name)
        .with_body(&contents1.as_bytes())
        .with_block_id(b"block1")
        .finalize()
        .await
        .unwrap();

    match &put_block_response.consistency {
        Consistency::Crc64(_) => {}
        _ => panic!("must receive a content_crc64 header"),
    }

    client
        .put_block()
        .with_container_name(&container.name)
        .with_blob_name(name)
        .with_body(&contents2.as_bytes())
        .with_block_id(b"block2")
        .finalize()
        .await
        .unwrap();

    let put_block_response = client
        .put_block()
        .with_container_name(&container.name)
        .with_blob_name(name)
        .with_body(&contents3.as_bytes())
        .with_block_id(b"block3")
        .with_content_md5(&digest3[..])
        .finalize()
        .await
        .unwrap();

    match &put_block_response.consistency {
        Consistency::Md5(_) => {}
        _ => panic!("must receive a content_md5 header"),
    }

    let received_block_list = client
        .get_block_list()
        .with_container_name(&container.name)
        .with_blob_name(name)
        .with_block_list_type(BlockListType::All)
        .finalize()
        .await
        .unwrap();

    client
        .put_block_list()
        .with_container_name(&container.name)
        .with_blob_name(name)
        .with_block_list(&received_block_list.block_with_size_list.into())
        .finalize()
        .await
        .unwrap();

    let res = client
        .acquire_blob_lease()
        .with_container_name(&container.name)
        .with_blob_name(name)
        .with_lease_duration(60)
        .finalize()
        .await
        .unwrap();
    println!("Acquire lease == {:?}", res);

    let lease_id = res.lease_id;

    let res = client
        .renew_blob_lease()
        .with_container_name(&container.name)
        .with_blob_name(name)
        .with_lease_id(&lease_id)
        .finalize()
        .await
        .unwrap();
    println!("Renew lease == {:?}", res);

    let res = client
        .break_blob_lease()
        .with_container_name(&container.name)
        .with_blob_name(name)
        .with_lease_break_period(15)
        .finalize()
        .await
        .unwrap();
    println!("Break lease == {:?}", res);

    let res = client
        .release_blob_lease()
        .with_container_name(&container.name)
        .with_blob_name(name)
        .with_lease_id(&lease_id)
        .finalize()
        .await
        .unwrap();
    println!("Release lease == {:?}", res);

    let res = client
        .delete_blob()
        .with_container_name(&container.name)
        .with_blob_name(name)
        .with_delete_snapshots_method(DeleteSnapshotsMethod::Include)
        .finalize()
        .await
        .unwrap();
    println!("Delete blob == {:?}", res);

    client
        .delete_container()
        .with_container_name(container.as_ref())
        .finalize()
        .await
        .unwrap();

    println!("container {} deleted!", container.name);
}

#[tokio::test]
async fn list_containers() {
    let client = initialize();

    trace!("running list_containers");

    let mut next_marker: Option<String> = None;

    loop {
        let ret = {
            let builder = client.list_containers().with_max_results(2);
            if let Some(nm) = next_marker {
                builder.with_next_marker(&nm).finalize().await.unwrap()
            } else {
                builder.finalize().await.unwrap()
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

#[tokio::test]
async fn put_block_blob() {
    let client = initialize();

    let blob_name: &'static str = "m1";
    let container_name: &'static str = "rust-upload-test";
    let data = b"abcdef";

    if client
        .list_containers()
        .finalize()
        .await
        .unwrap()
        .incomplete_vector
        .iter()
        .find(|x| x.name == container_name)
        .is_none()
    {
        client
            .create_container()
            .with_container_name(container_name)
            .with_public_access(PublicAccess::Blob)
            .finalize()
            .await
            .unwrap();
    }

    // calculate md5 too!
    let digest = md5::compute(&data[..]);

    client
        .put_block_blob()
        .with_container_name(&container_name)
        .with_blob_name(&blob_name)
        .with_content_type("text/plain")
        .with_body(&data[..])
        .with_content_md5(&digest[..])
        .finalize()
        .await
        .unwrap();

    trace!("created {:?}", blob_name);
}

#[tokio::test]
async fn copy_blob() {
    let client = initialize();

    let blob_name: &'static str = "copysrc";
    let container_name: &'static str = "rust-upload-test";
    let data = b"abcdef";

    if client
        .list_containers()
        .finalize()
        .await
        .unwrap()
        .incomplete_vector
        .iter()
        .find(|x| x.name == container_name)
        .is_none()
    {
        client
            .create_container()
            .with_container_name(container_name)
            .with_public_access(PublicAccess::Blob)
            .finalize()
            .await
            .unwrap();
    }

    // calculate md5 too!
    let digest = md5::compute(&data[..]);

    client
        .put_block_blob()
        .with_container_name(&container_name)
        .with_blob_name(&blob_name)
        .with_content_type("text/plain")
        .with_body(&data[..])
        .with_content_md5(&digest[..])
        .finalize()
        .await
        .unwrap();

    trace!("created {:?}", blob_name);

    client
        .copy_blob()
        .with_source_url(&format!(
            "https://{}.blob.core.windows.net/{}/{}",
            &std::env::var("STORAGE_ACCOUNT").unwrap(),
            &container_name,
            &blob_name
        ))
        .with_container_name(&container_name)
        .with_blob_name("cloned_blob")
        .finalize()
        .await
        .unwrap();
}

async fn requires_send_future<F, O>(fut: F) -> O
where
    F: std::future::Future<Output = O> + Send,
{
    fut.await
}

#[tokio::test]
async fn put_block_blob_and_get_properties() {
    let client = initialize();

    let blob_name: &'static str = "properties";
    let container_name: &'static str = "rust-upload-test";
    let data = b"abcdef";

    if client
        .list_containers()
        .finalize()
        .await
        .unwrap()
        .incomplete_vector
        .iter()
        .find(|x| x.name == container_name)
        .is_none()
    {
        client
            .create_container()
            .with_container_name(container_name)
            .with_public_access(PublicAccess::Blob)
            .finalize()
            .await
            .unwrap();
    }

    // calculate md5 too!
    let digest = md5::compute(&data[..]);

    client
        .put_block_blob()
        .with_container_name(&container_name)
        .with_blob_name(&blob_name)
        .with_content_type("text/plain")
        .with_body(&data[..])
        .with_content_md5(&digest[..])
        .finalize()
        .await
        .unwrap();

    trace!("created {:?}", blob_name);

    let blob_properties = client
        .get_blob_properties()
        .with_container_name(&container_name)
        .with_blob_name(&blob_name)
        .finalize()
        .await
        .unwrap();

    assert_eq!(blob_properties.blob.content_length, 6);

    let _ = requires_send_future(
        client
            .get_blob_properties()
            .with_container_name(&container_name)
            .with_blob_name(&blob_name)
            .finalize(),
    );
}

fn initialize() -> Box<dyn Client> {
    let account =
        std::env::var("STORAGE_ACCOUNT").expect("Set env variable STORAGE_ACCOUNT first!");
    let master_key =
        std::env::var("STORAGE_MASTER_KEY").expect("Set env variable STORAGE_MASTER_KEY first!");

    Box::new(client::with_access_key(&account, &master_key))
}

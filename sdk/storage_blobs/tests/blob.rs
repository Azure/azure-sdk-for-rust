#![cfg(all(test, feature = "test_e2e"))]
#[macro_use]
extern crate log;

use azure_core::prelude::*;
use azure_storage::blob::{
    blob::BlockListType,
    container::{Container, PublicAccess},
    prelude::*,
};
use azure_storage::core::prelude::*;
use bytes::Bytes;
use chrono::{FixedOffset, Utc};
use std::ops::Add;
use std::ops::Deref;
use std::sync::Arc;
use std::time::Duration;
use url::Url;
use uuid::Uuid;

#[tokio::test]
async fn create_and_delete_container() {
    let name: &'static str = "azuresdkrustetoets";

    let storage_client = initialize().as_storage_client();
    let container = storage_client.as_container_client(name);

    container
        .create()
        .public_access(PublicAccess::None)
        .execute()
        .await
        .unwrap();

    // get acl without stored access policy list
    let _result = container.get_acl().execute().await.unwrap();

    // set stored acess policy list
    let dt_start = Utc::now().with_timezone(&FixedOffset::east(0));
    let dt_end = dt_start.add(chrono::Duration::days(7));

    let mut sapl = StoredAccessPolicyList::default();
    sapl.stored_access
        .push(StoredAccessPolicy::new("pollo", dt_start, dt_end, "rwd"));

    let _result = container
        .set_acl(PublicAccess::None)
        .stored_access_policy_list(&sapl)
        .execute()
        .await
        .unwrap();

    // now we get back the acess policy list and compare to the one created
    let result = container.get_acl().execute().await.unwrap();

    assert!(result.public_access == PublicAccess::None);
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

    let res = container.get_properties().execute().await.unwrap();
    assert!(res.container.public_access == PublicAccess::None);

    let list = storage_client
        .list_containers()
        .prefix(name)
        .execute()
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

    let res = container
        .acquire_lease(Duration::from_secs(30))
        .execute()
        .await
        .unwrap();
    let lease_id = res.lease_id;
    let lease = container.as_container_lease_client(res.lease_id);

    let _res = lease.renew().execute().await.unwrap();

    container
        .delete()
        .lease_id(&lease_id) // must pass the lease here too
        .execute()
        .await
        .unwrap();
}

#[tokio::test]
async fn put_and_get_block_list() {
    let u = Uuid::new_v4();
    let container_name = format!("sdkrust{}", u);
    let name = "asd - ()krustputblock.txt";

    let storage = initialize().as_storage_client();
    let container = storage.as_container_client(&container_name);
    let blob = container.as_blob_client(name);

    container
        .create()
        .public_access(PublicAccess::None)
        .execute()
        .await
        .expect("container already present");

    let contents1 = "AAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAA";
    let contents2 = "BBBBBBBBBBBBBBBBBBBBBBBBBBBBBBBBBBBBBBBBBBBBBBBBBBBBBBBBBBBBBB";
    let contents3 = "CCCCCCCCCCCCCCCCCCCCCCCCCCCCCCCCCCCCCCCCCCCCCCCCCCCCCCCCCCCCCC";

    let digest3 = md5::compute(contents3).into();

    let put_block_response = blob
        .put_block("block1", Bytes::from(contents1))
        .execute()
        .await
        .unwrap();

    assert!(put_block_response.content_crc64.is_some());

    blob.put_block("block2", Bytes::from(contents2))
        .execute()
        .await
        .unwrap();

    let put_block_response = blob
        .put_block("block3", Bytes::from(contents3))
        .hash(&digest3)
        .execute()
        .await
        .unwrap();

    assert!(put_block_response.content_crc64.is_some());

    let received_block_list = blob
        .get_block_list()
        .block_list_type(BlockListType::All)
        .execute()
        .await
        .unwrap();

    blob.put_block_list(&received_block_list.block_with_size_list.into())
        .execute()
        .await
        .unwrap();

    let res = blob
        .acquire_lease(Duration::from_secs(60))
        .execute()
        .await
        .unwrap();
    println!("Acquire lease == {:?}", res);

    let lease_id = res.lease_id;
    let lease = blob.as_blob_lease_client(lease_id);

    let res = lease.renew().execute().await.unwrap();
    println!("Renew lease == {:?}", res);

    let res = blob
        .break_lease()
        .lease_break_period(Duration::from_secs(15))
        .execute()
        .await
        .unwrap();
    println!("Break lease == {:?}", res);

    let res = lease.release().execute().await.unwrap();
    println!("Release lease == {:?}", res);

    let res = blob
        .delete()
        .delete_snapshots_method(DeleteSnapshotsMethod::Include)
        .execute()
        .await
        .unwrap();
    println!("Delete blob == {:?}", res);

    container.delete().execute().await.unwrap();

    println!("container {} deleted!", container_name);
}

#[tokio::test]
async fn list_containers() {
    let storage = initialize().as_storage_client();
    trace!("running list_containers");

    let mut next_marker = None;

    loop {
        let ret = {
            let builder = storage
                .list_containers()
                .max_results(std::num::NonZeroU32::new(2u32).unwrap());
            if let Some(nm) = next_marker {
                builder.next_marker(nm).execute().await.unwrap()
            } else {
                builder.execute().await.unwrap()
            }
        };

        trace!("ret {:?}\n\n", ret);
        if !ret.is_complete() {
            next_marker = Some(ret.incomplete_vector.next_marker().unwrap().to_owned());
        } else {
            break;
        }
    }
}

#[tokio::test]
async fn put_block_blob() {
    let blob_name: &'static str = "m1";
    let container_name: &'static str = "rust-upload-test";
    let data = Bytes::from_static(b"abcdef");

    let storage = initialize().as_storage_client();
    let container = storage.as_container_client(container_name);
    let blob = container.as_blob_client(blob_name);

    if storage
        .list_containers()
        .execute()
        .await
        .unwrap()
        .incomplete_vector
        .iter()
        .find(|x| x.name == container_name)
        .is_none()
    {
        container
            .create()
            .public_access(PublicAccess::None)
            .execute()
            .await
            .unwrap();
    }

    // calculate md5 too!
    let digest = md5::compute(&data[..]).into();

    blob.put_block_blob(data)
        .content_type("text/plain")
        .hash(&digest)
        .execute()
        .await
        .unwrap();

    trace!("created {:?}", blob_name);
}

#[tokio::test]
async fn copy_blob() {
    let blob_name: &'static str = "copysrc";
    let container_name: &'static str = "rust-upload-test";
    let data = Bytes::from_static(b"abcdef");

    let storage = initialize().as_storage_client();
    let container = storage.as_container_client(container_name);
    let blob = container.as_blob_client(blob_name);

    if storage
        .list_containers()
        .execute()
        .await
        .unwrap()
        .incomplete_vector
        .iter()
        .find(|x| x.name == container_name)
        .is_none()
    {
        container
            .create()
            .public_access(PublicAccess::None)
            .execute()
            .await
            .unwrap();
    }

    // calculate md5 too!
    let digest = md5::compute(&data[..]).into();

    blob.put_block_blob(data)
        .content_type("text/plain")
        .hash(&digest)
        .execute()
        .await
        .unwrap();

    trace!("created {:?}", blob_name);

    let cloned_blob = container.as_blob_client("cloned_blob");

    let url = Url::parse(&format!(
        "https://{}.blob.core.windows.net/{}/{}",
        &std::env::var("STORAGE_ACCOUNT").unwrap(),
        &container_name,
        &blob_name
    ))
    .unwrap();

    cloned_blob.copy(&url).execute().await.unwrap();
}

async fn requires_send_future<F, O>(fut: F) -> O
where
    F: std::future::Future<Output = O> + Send,
{
    fut.await
}

#[tokio::test]
async fn put_block_blob_and_get_properties() {
    let blob_name: &'static str = "properties";
    let container_name: &'static str = "rust-upload-test";
    let data = Bytes::from_static(b"abcdef");

    let storage = initialize().as_storage_client();
    let container = storage.as_container_client(container_name);
    let blob = container.as_blob_client(blob_name);

    if storage
        .list_containers()
        .execute()
        .await
        .unwrap()
        .incomplete_vector
        .iter()
        .find(|x| x.name == container_name)
        .is_none()
    {
        container
            .create()
            .public_access(PublicAccess::None)
            .execute()
            .await
            .unwrap();
    }

    // calculate md5 too!
    let digest = md5::compute(&data[..]).into();

    blob.put_block_blob(data)
        .content_type("text/plain")
        .hash(&digest)
        .execute()
        .await
        .unwrap();

    trace!("created {:?}", blob_name);

    let blob_properties = blob.get_properties().execute().await.unwrap();

    assert_eq!(blob_properties.blob.properties.content_length, 6);

    let _ = requires_send_future(blob.get_properties().execute());
}

#[allow(dead_code)]
fn send_check() {
    let client = initialize();
    let blob = client
        .as_storage_client()
        .as_container_client("a")
        .as_blob_client("b");

    let _ = requires_send_future(blob.acquire_lease(Duration::from_secs(10)).execute());
    let _ = requires_send_future(blob.clear_page(BA512Range::new(0, 1024).unwrap()).execute());
}

fn initialize() -> Arc<StorageAccountClient> {
    let account =
        std::env::var("STORAGE_ACCOUNT").expect("Set env variable STORAGE_ACCOUNT first!");
    let master_key =
        std::env::var("STORAGE_MASTER_KEY").expect("Set env variable STORAGE_MASTER_KEY first!");

    let http_client = azure_core::new_http_client();

    StorageAccountClient::new_access_key(http_client.clone(), &account, &master_key)
}

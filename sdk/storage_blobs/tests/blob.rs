#![cfg(all(test, feature = "test_e2e"))]
#[macro_use]
extern crate log;

use azure_core::date;
use azure_storage::core::prelude::*;
use azure_storage_blobs::{blob::BlockListType, container::PublicAccess, prelude::*};
use bytes::Bytes;
use futures::StreamExt;
use std::ops::{Add, Deref};
use std::time::Duration;
use time::OffsetDateTime;
use url::Url;
use uuid::Uuid;

#[tokio::test]
async fn create_and_delete_container() -> azure_core::Result<()> {
    let container_name = format!("create-{}", Uuid::new_v4());

    let storage_client = initialize();
    let blob_service = storage_client.blob_service_client();
    let container = storage_client.container_client(&container_name);

    container
        .create()
        .public_access(PublicAccess::None)
        .into_future()
        .await?;

    // get acl without stored access policy list
    let _result = container.get_acl().into_future().await?;

    // set stored acess policy list
    let dt_start = OffsetDateTime::now_utc();
    let dt_end = dt_start.add(date::duration_from_days(7));

    let mut sapl = StoredAccessPolicyList::default();
    sapl.stored_access
        .push(StoredAccessPolicy::new("pollo", dt_start, dt_end, "rwd"));

    let _result = container
        .set_acl(PublicAccess::None)
        .stored_access_policy_list(sapl.clone())
        .into_future()
        .await?;

    // now we get back the acess policy list and compare to the one created
    let result = container.get_acl().into_future().await?;

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

    let res = container.get_properties().into_future().await?;
    assert!(res.container.public_access == PublicAccess::None);

    let list = blob_service
        .list_containers()
        .prefix(container_name.clone())
        .into_stream()
        .next()
        .await
        .unwrap()?;
    let cont_list = list
        .containers
        .deref()
        .iter()
        .filter(|e| e.name == container_name);

    if cont_list.count() != 1 {
        panic!("More than 1 container returned with the same name!");
    }

    let res = container
        .acquire_lease(Duration::from_secs(30))
        .into_future()
        .await
        .unwrap();
    let lease_id = res.lease_id;
    let lease = container.container_lease_client(res.lease_id);

    let _res = lease.renew().into_future().await.unwrap();

    container
        .delete()
        .lease_id(lease_id) // must pass the lease here too
        .into_future()
        .await?;

    Ok(())
}

#[tokio::test]
async fn put_and_get_block_list() {
    let u = Uuid::new_v4();
    let container_name = format!("sdkrust{}", u);
    let name = "asd - ()krustputblock.txt";

    let storage = initialize();
    let container = storage.container_client(&container_name);
    let blob = container.blob_client(name);

    container
        .create()
        .public_access(PublicAccess::None)
        .into_future()
        .await
        .expect("container already present");

    let contents1 = "AAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAA";
    let contents2 = "BBBBBBBBBBBBBBBBBBBBBBBBBBBBBBBBBBBBBBBBBBBBBBBBBBBBBBBBBBBBBB";
    let contents3 = "CCCCCCCCCCCCCCCCCCCCCCCCCCCCCCCCCCCCCCCCCCCCCCCCCCCCCCCCCCCCCC";

    let digest3 = md5::compute(contents3);

    let put_block_response = blob
        .put_block("block1", Bytes::from(contents1))
        .into_future()
        .await
        .unwrap();

    assert!(put_block_response.content_crc64.is_some());

    blob.put_block("block2", Bytes::from(contents2))
        .into_future()
        .await
        .unwrap();

    let put_block_response = blob
        .put_block("block3", Bytes::from(contents3))
        .hash(digest3)
        .into_future()
        .await
        .unwrap();

    assert!(put_block_response.content_crc64.is_some());

    let received_block_list = blob
        .get_block_list()
        .block_list_type(BlockListType::All)
        .into_future()
        .await
        .unwrap();

    blob.put_block_list(received_block_list.block_with_size_list.into())
        .into_future()
        .await
        .unwrap();

    let res = blob
        .acquire_lease(Duration::from_secs(60))
        .into_future()
        .await
        .unwrap();
    println!("Acquire lease == {:?}", res);

    let lease_id = res.lease_id;
    let lease = blob.blob_lease_client(lease_id);

    let res = lease.renew().into_future().await.unwrap();
    println!("Renew lease == {:?}", res);

    let res = blob
        .break_lease()
        .lease_break_period(Duration::from_secs(15))
        .into_future()
        .await
        .unwrap();
    println!("Break lease == {:?}", res);

    let res = lease.release().into_future().await.unwrap();
    println!("Release lease == {:?}", res);

    let res = blob
        .delete()
        .delete_snapshots_method(DeleteSnapshotsMethod::Include)
        .into_future()
        .await
        .unwrap();
    println!("Delete blob == {:?}", res);

    container.delete().into_future().await.unwrap();

    println!("container {} deleted!", container_name);
}

#[tokio::test]
async fn list_containers() {
    let storage = initialize();
    let blob_service = storage.blob_service_client();
    trace!("running list_containers");

    let mut stream = blob_service
        .list_containers()
        .max_results(std::num::NonZeroU32::new(2u32).unwrap())
        .into_stream();

    while let Some(result) = stream.next().await {
        let ret = result.unwrap();
        trace!("ret {:?}\n\n", ret);
    }
}

#[tokio::test]
async fn put_block_blob() {
    let blob_name: &'static str = "m1";
    let container_name: &'static str = "rust-upload-test";
    let data = Bytes::from_static(b"abcdef");

    let storage = initialize();
    let blob_service = storage.blob_service_client();
    let container = storage.container_client(container_name);
    let blob = container.blob_client(blob_name);

    if !blob_service
        .list_containers()
        .into_stream()
        .next()
        .await
        .unwrap()
        .unwrap()
        .containers
        .iter()
        .any(|x| x.name == container_name)
    {
        container
            .create()
            .public_access(PublicAccess::None)
            .into_future()
            .await
            .unwrap();
    }

    // calculate md5 too!
    let digest = md5::compute(&data[..]);

    blob.put_block_blob(data)
        .content_type("text/plain")
        .hash(digest)
        .into_future()
        .await
        .unwrap();

    trace!("created {:?}", blob_name);
}

#[tokio::test]
async fn copy_blob() -> azure_core::Result<()> {
    let blob_name: &'static str = "copysrc";
    let container_name = format!("copy-blob-{}", Uuid::new_v4());
    let data = Bytes::from_static(b"abcdef");

    let storage = initialize();
    let blob_service = storage.blob_service_client();
    let container = storage.container_client(&container_name);
    let blob = container.blob_client(blob_name);

    if !blob_service
        .list_containers()
        .into_stream()
        .next()
        .await
        .unwrap()
        .unwrap()
        .containers
        .iter()
        .any(|x| x.name == container_name)
    {
        container
            .create()
            .public_access(PublicAccess::None)
            .into_future()
            .await?;
    }

    // calculate md5 too!
    let digest = md5::compute(&data[..]);

    blob.put_block_blob(data)
        .content_type("text/plain")
        .hash(digest)
        .into_future()
        .await?;

    trace!("created {:?}", blob_name);

    let cloned_blob = container.blob_client("cloned_blob");

    let url = Url::parse(&format!(
        "https://{}.blob.core.windows.net/{}/{}",
        &std::env::var("STORAGE_ACCOUNT").unwrap(),
        &container_name,
        &blob_name
    ))
    .unwrap();

    cloned_blob.copy(url).into_future().await?;

    container.delete().into_future().await?;
    Ok(())
}

async fn requires_send_future<F, O>(fut: F) -> O
where
    F: std::future::Future<Output = O> + Send,
{
    fut.await
}

#[tokio::test]
async fn put_block_blob_and_get_properties() -> azure_core::Result<()> {
    let blob_name: &'static str = "properties";
    let container_name = format!("properties-{}", Uuid::new_v4());
    let data = Bytes::from_static(b"abcdef");

    let storage = initialize();
    let blob_service = storage.blob_service_client();
    let container = storage.container_client(&container_name);
    let blob = container.blob_client(blob_name);

    if !blob_service
        .list_containers()
        .into_stream()
        .next()
        .await
        .unwrap()
        .unwrap()
        .containers
        .iter()
        .any(|x| x.name == container_name)
    {
        container
            .create()
            .public_access(PublicAccess::None)
            .into_future()
            .await
            .unwrap();
    }

    // calculate md5 too!
    let digest = md5::compute(&data[..]);

    blob.put_block_blob(data)
        .content_type("text/plain")
        .hash(digest)
        .into_future()
        .await
        .unwrap();

    trace!("created {:?}", blob_name);

    let blob_properties = blob.get_properties().into_future().await.unwrap();

    assert_eq!(blob_properties.blob.properties.content_length, 6);

    let _ = requires_send_future(blob.get_properties().into_future());
    container.delete().into_future().await?;
    Ok(())
}

#[tokio::test]
async fn put_block_blob_and_snapshot() {
    let blob_name: &'static str = "snapshot-blob.txt";
    let container_name: &'static str = "rust-snapshot-test";
    let data = Bytes::from_static(b"abcdef");

    let storage = initialize();
    let blob_service = storage.blob_service_client();
    let container = storage.container_client(container_name);
    let blob = container.blob_client(blob_name);

    if blob_service
        .list_containers()
        .into_stream()
        .next()
        .await
        .unwrap()
        .unwrap()
        .containers
        .iter()
        .find(|x| x.name == container_name)
        .is_none()
    {
        container
            .create()
            .public_access(PublicAccess::None)
            .into_future()
            .await
            .unwrap();
    }

    // calculate md5 too!
    let digest = md5::compute(&data[..]);

    blob.put_block_blob(data)
        .content_type("text/plain")
        .hash(digest)
        .into_future()
        .await
        .unwrap();

    trace!("created {:?}", blob_name);

    let snapshot = blob.snapshot().into_future().await.unwrap().snapshot;

    trace!("crated snapshot: {:?} of {:?}", snapshot, blob_name);

    // Clean-up test
    container.delete().into_future().await.unwrap();
    trace!("container {} deleted!", container_name);
}

#[tokio::test]
async fn set_blobtier() {
    let blob_name: &'static str = "m9";
    let container_name: &'static str = "rust-set-blobtier-test";
    let data = Bytes::from_static(b"abcdef");

    let storage = initialize();
    let blob_service = storage.blob_service_client();
    let container = storage.container_client(container_name);
    let blob = container.blob_client(blob_name);

    if !blob_service
        .list_containers()
        .into_stream()
        .next()
        .await
        .unwrap()
        .unwrap()
        .containers
        .iter()
        .any(|x| x.name == container_name)
    {
        container
            .create()
            .public_access(PublicAccess::None)
            .into_future()
            .await
            .unwrap();
    }

    // calculate md5 too!
    let digest = md5::compute(&data[..]);

    blob.put_block_blob(data)
        .content_type("text/plain")
        .hash(digest)
        .into_future()
        .await
        .unwrap();

    trace!("created {:?}", blob_name);

    //
    // Hot -> Cool
    //
    blob.set_blob_tier(AccessTier::Cool)
        .into_future()
        .await
        .unwrap();

    trace!("blob access tier set to {:?}", AccessTier::Cool);

    //
    // Cool -> Hot
    //
    blob.set_blob_tier(AccessTier::Hot)
        .into_future()
        .await
        .unwrap();

    trace!("blob access tier set to {:?}", AccessTier::Hot);

    //
    // Hot -> Archive
    //
    blob.set_blob_tier(AccessTier::Archive)
        .into_future()
        .await
        .unwrap();

    trace!("blob access tier set to {:?}", AccessTier::Archive);

    //
    // Archive -> Cool
    //
    blob.set_blob_tier(AccessTier::Cool)
        .into_future()
        .await
        .unwrap();

    trace!("blob access tier set to {:?}", AccessTier::Cool);

    //
    // Archive -> Cool (rehydrating)
    //
    blob.set_blob_tier(AccessTier::Cool)
        .into_future()
        .await
        .unwrap();

    trace!("blob access tier set to {:?}", AccessTier::Cool);

    // Clean-up test
    container.delete().into_future().await.unwrap();
    println!("container {} deleted!", container_name);
}

#[allow(dead_code)]
fn send_check() {
    let client = initialize();
    let blob = client.container_client("a").blob_client("b");

    let _ = requires_send_future(blob.acquire_lease(Duration::from_secs(10)).into_future());
    let _ = requires_send_future(
        blob.clear_page(BA512Range::new(0, 1024).unwrap())
            .into_future(),
    );
}

fn initialize() -> StorageClient {
    let account =
        std::env::var("STORAGE_ACCOUNT").expect("Set env variable STORAGE_ACCOUNT first!");
    let access_key =
        std::env::var("STORAGE_ACCESS_KEY").expect("Set env variable STORAGE_ACCESS_KEY first!");

    StorageClient::new_access_key(&account, &access_key)
}

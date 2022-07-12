#![cfg(all(test, feature = "test_e2e"))]
#[macro_use]
extern crate log;

use azure_core::prelude::*;
use azure_storage::core::prelude::*;
use azure_storage_blobs::{container::PublicAccess, prelude::*};
use bytes::Bytes;
use futures::StreamExt;

#[tokio::test]
async fn put_append_blob() {
    let account =
        std::env::var("STORAGE_ACCOUNT").expect("Set env variable STORAGE_ACCOUNT first!");
    let access_key =
        std::env::var("STORAGE_ACCESS_KEY").expect("Set env variable STORAGE_ACCESS_KEY first!");

    let blob_name: &'static str = "append_blob.txt";
    let container_name: &'static str = "rust-upload-test";
    let _data = b"abcdef";

    let storage = StorageClient::new_access_key(&account, &access_key, StorageOptions::default());
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

    let mut metadata = Metadata::new();
    metadata.insert("attrib", "value");
    metadata.insert("second", "something");

    blob.put_append_blob()
        .content_type("text/plain")
        .metadata(metadata)
        .into_future()
        .await
        .unwrap();

    trace!("created {:?}", blob_name);

    let resp = blob.get_metadata().into_future().await.unwrap();

    assert_eq!(resp.metadata.len(), 2);

    assert_eq!(resp.metadata.get("attrib"), Some(Bytes::from("value")));
    assert_eq!(resp.metadata.get("second"), Some(Bytes::from("something")));
    assert_eq!(resp.metadata.get("not_found"), None);
}

#![cfg(all(test, feature = "test_e2e"))]
#[macro_use]
extern crate log;

use azure_core::prelude::*;
use azure_storage::prelude::*;
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

    let storage_credentials = StorageCredentials::access_key(account.clone(), access_key);
    let blob_service = BlobServiceClient::new(account, storage_credentials);
    let container = blob_service.container_client(container_name);
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
            .await
            .unwrap();
    }

    let mut metadata = Metadata::new();
    metadata.insert("attrib", "value");
    metadata.insert("second", "something");

    blob.put_append_blob()
        .content_type("text/plain")
        .metadata(metadata)
        .await
        .unwrap();

    trace!("created {:?}", blob_name);

    let resp = blob.get_metadata().await.unwrap();

    assert_eq!(resp.metadata.len(), 2);

    assert_eq!(resp.metadata.get("attrib"), Some(Bytes::from("value")));
    assert_eq!(resp.metadata.get("second"), Some(Bytes::from("something")));
    assert_eq!(resp.metadata.get("not_found"), None);
}

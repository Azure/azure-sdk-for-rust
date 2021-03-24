#![cfg(all(test, feature = "test_e2e"))]
#[macro_use]
extern crate log;

use azure_core::prelude::*;
use azure_storage::blob::container::PublicAccess;
use azure_storage::blob::prelude::*;
use azure_storage::core::prelude::*;
use bytes::Bytes;
use std::sync::Arc;

#[tokio::test]
async fn put_append_blob() {
    let account =
        std::env::var("STORAGE_ACCOUNT").expect("Set env variable STORAGE_ACCOUNT first!");
    let master_key =
        std::env::var("STORAGE_MASTER_KEY").expect("Set env variable STORAGE_MASTER_KEY first!");

    let blob_name: &'static str = "append_blob.txt";
    let container_name: &'static str = "rust-upload-test";
    let _data = b"abcdef";

    let http_client: Arc<Box<dyn HttpClient>> = Arc::new(Box::new(reqwest::Client::new()));

    let storage = StorageAccountClient::new_access_key(http_client.clone(), &account, &master_key)
        .as_storage_client();
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
            .public_access(PublicAccess::Blob)
            .execute()
            .await
            .unwrap();
    }

    let mut metadata = Metadata::new();
    metadata.insert("attrib", "value");
    metadata.insert("second", "something");

    blob.put_append_blob()
        .content_type("text/plain")
        .metadata(&metadata)
        .execute()
        .await
        .unwrap();

    trace!("created {:?}", blob_name);

    let resp = blob.get_metadata().execute().await.unwrap();

    assert_eq!(resp.metadata.len(), 2);

    assert_eq!(resp.metadata.get("attrib"), Some(Bytes::from("value")));
    assert_eq!(resp.metadata.get("second"), Some(Bytes::from("something")));
    assert_eq!(resp.metadata.get("not_found"), None);
}

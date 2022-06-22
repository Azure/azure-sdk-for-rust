#![cfg(all(test, feature = "test_e2e"))]
#[macro_use]
extern crate log;
use azure_core::prelude::*;
use azure_storage::core::prelude::*;
use azure_storage_blobs::prelude::*;
use futures::StreamExt;
use std::sync::Arc;

#[tokio::test]
async fn put_page_blob() {
    let blob_name: &'static str = "page_blob.txt";
    let container_name: &'static str = "rust-upload-test";

    let storage = initialize().as_storage_client();
    let blob_service = storage.as_blob_service_client();
    let container = storage.as_container_client(container_name);
    let blob = container.as_blob_client(blob_name);

    if Box::pin(blob_service.list_containers().stream())
        .next()
        .await
        .unwrap()
        .unwrap()
        .incomplete_vector
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

    blob.put_page_blob(1024 * 64)
        .content_type("text/plain")
        .metadata(metadata)
        .into_future()
        .await
        .unwrap();

    trace!("created {:?}", blob_name);
}

fn initialize() -> Arc<StorageAccountClient> {
    let account =
        std::env::var("STORAGE_ACCOUNT").expect("Set env variable STORAGE_ACCOUNT first!");
    let master_key =
        std::env::var("STORAGE_MASTER_KEY").expect("Set env variable STORAGE_MASTER_KEY first!");

    let http_client = azure_core::new_http_client();

    StorageAccountClient::new_access_key(http_client.clone(), &account, &master_key)
}

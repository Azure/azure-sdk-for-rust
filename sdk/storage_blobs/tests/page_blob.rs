#![cfg(all(test, feature = "test_e2e"))]
#[macro_use]
extern crate log;
use azure_core::prelude::*;
use azure_storage::core::prelude::*;
use azure_storage_blobs::prelude::*;
use futures::StreamExt;

#[tokio::test]
async fn put_page_blob() {
    let blob_name: &'static str = "page_blob.txt";
    let container_name: &'static str = "rust-upload-test";

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

fn initialize() -> StorageClient {
    let account =
        std::env::var("STORAGE_ACCOUNT").expect("Set env variable STORAGE_ACCOUNT first!");
    let access_key =
        std::env::var("STORAGE_ACCESS_KEY").expect("Set env variable STORAGE_ACCESS_KEY first!");

    StorageClient::new_access_key(&account, &access_key, StorageOptions::default())
}

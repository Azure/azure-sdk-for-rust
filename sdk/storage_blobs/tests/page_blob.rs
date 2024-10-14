#![cfg(all(test, feature = "test_e2e"))]
use azure_core::prelude::*;
use azure_storage::prelude::*;
use azure_storage_blobs::prelude::*;
use futures::StreamExt;
use tracing::trace;

#[tokio::test]
async fn put_page_blob() {
    let blob_name: &'static str = "page_blob.txt";
    let container_name: &'static str = "rust-upload-test";

    let blob_service = initialize();
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

    blob.put_page_blob(1024 * 64)
        .content_type("text/plain")
        .metadata(metadata.clone())
        .await
        .unwrap();

    // CreateIfNotExists should fail if the page blob exists.
    // This behavior is controlled through IfMatch conditions.
    let res = blob
        .put_page_blob(1024 * 64)
        .content_type("text/plain")
        .metadata(metadata.clone())
        .if_match(IfMatchCondition::NotMatch(String::from("*")))
        .await;
    match res {
        Ok(_) => {
            assert!(false, "If-match condition is not being honored.");
        }
        Err(ref e) => {
            let http_error = e
                .as_http_error()
                .expect("Violating if-Match condition should return an HTTP error");
            assert_eq!(
                azure_core::StatusCode::Conflict,
                http_error.status(),
                "Violating if-Match condition should return 409, Conflict"
            );
        }
    }
    trace!("created {:?}", blob_name);

    // simulate Create a new page_blob with CreateIfNotExists.
    let ifmatch_blob_name = "page-blob-ifmatch.txt";
    let blob_client = container.blob_client(ifmatch_blob_name);
    blob_client
        .put_page_blob(1024 * 64)
        .content_type("text/plain")
        .metadata(metadata.clone())
        .if_match(IfMatchCondition::NotMatch(String::from("*")))
        .await
        .unwrap();
    trace!("created {:?}", ifmatch_blob_name);
}

fn initialize() -> BlobServiceClient {
    let account =
        std::env::var("STORAGE_ACCOUNT").expect("Set env variable STORAGE_ACCOUNT first!");
    let access_key =
        std::env::var("STORAGE_ACCESS_KEY").expect("Set env variable STORAGE_ACCESS_KEY first!");

    let storage_credentials = StorageCredentials::access_key(account.clone(), access_key);
    BlobServiceClient::new(account, storage_credentials)
}

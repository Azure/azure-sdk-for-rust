use azure_storage_blobs::container::PublicAccess;
use azure_storage_blobs::prelude::{AsBlobServiceClient, AsContainerClient};
use bytes::Bytes;
use futures::StreamExt;
use log::trace;

mod setup;

#[tokio::test]
async fn put_block_blob_and_snapshot() {
    const CONTAINER_NAME: &'static str = "test-container-put-block-blob-and-snapshot";
    const BLOB_NAME: &'static str = "test-container-put-block-blob-and-snapshot";
    let data = Bytes::from_static(b"abcdef");

    let storage = setup::initialize("put_block_blob_and_snapshot").unwrap();
    let blob_service = storage.blob_service_client();
    let container = storage.container_client(CONTAINER_NAME);
    let blob = container.blob_client(BLOB_NAME);

    if blob_service
        .list_containers()
        .into_stream()
        .next()
        .await
        .unwrap()
        .unwrap()
        .containers
        .iter()
        .find(|x| x.name == CONTAINER_NAME)
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

    trace!("created {:?}", BLOB_NAME);

    let snapshot = blob.snapshot().into_future().await.unwrap().snapshot;

    trace!("crated snapshot: {:?} of {:?}", snapshot, BLOB_NAME);

    // Clean-up test
    container.delete().into_future().await.unwrap();
    trace!("container {} deleted!", CONTAINER_NAME);
}

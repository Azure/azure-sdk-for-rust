use azure_storage_blobs::container::PublicAccess;
use bytes::Bytes;
use futures::StreamExt;
use log::trace;

mod setup;

#[tokio::test]
async fn put_block_blob_and_snapshot() {
    const CONTAINER_NAME: &str = "test-container-put-block-blob-and-snapshot";
    const BLOB_NAME: &str = "test-container-put-block-blob-and-snapshot";
    let data = Bytes::from_static(b"abcdef");

    let blob_service = setup::initialize("put_block_blob_and_snapshot").unwrap();
    let container = blob_service.container_client(CONTAINER_NAME);
    let blob = container.blob_client(BLOB_NAME);

    if !blob_service
        .list_containers()
        .into_stream()
        .next()
        .await
        .unwrap()
        .unwrap()
        .containers
        .iter()
        .any(|x| x.name == CONTAINER_NAME)
    {
        container
            .create()
            .public_access(PublicAccess::None)
            .await
            .unwrap();
    }

    // calculate md5 too!
    let digest = md5::compute(&data[..]).0;

    blob.put_block_blob(data)
        .content_type("text/plain")
        .hash(digest)
        .await
        .unwrap();

    trace!("created {:?}", BLOB_NAME);

    let snapshot = blob.snapshot().await.unwrap().snapshot;

    trace!("crated snapshot: {:?} of {:?}", snapshot, BLOB_NAME);

    // Clean-up test
    container.delete().await.unwrap();
    trace!("container {} deleted!", CONTAINER_NAME);
}

use azure_storage::core::prelude::*;
use azure_storage_blobs::prelude::*;
use futures::stream::StreamExt;
use uuid::Uuid;

#[tokio::main]
async fn main() -> azure_core::Result<()> {
    // First we retrieve the account name and access key from environment variables.
    let account =
        std::env::var("STORAGE_ACCOUNT").expect("Set env variable STORAGE_ACCOUNT first!");
    let access_key =
        std::env::var("STORAGE_ACCESS_KEY").expect("Set env variable STORAGE_ACCESS_KEY first!");

    let container_name = format!("range-example-{}", Uuid::new_v4());
    let blob_name = format!("blob-{}.txt", Uuid::new_v4());

    let container_client =
        StorageClient::new_access_key(&account, &access_key).container_client(&container_name);
    container_client.create().into_future().await?;

    let blob_client = container_client.blob_client(&blob_name);

    let buf = b"0123456789".repeat(1000);

    blob_client
        .put_block_blob(buf.clone())
        .into_future()
        .await?;

    // if we get the entire content, it should match
    let blob = blob_client.get_content().await?;
    assert_eq!(blob, buf);

    // if we only download specific ranges, this should act like we took a range slice
    // from the buffer
    for range in [0..1024, 10..100, 3..103] {
        let mut chunk: Vec<u8> = vec![];
        let mut stream = blob_client.get().range(range.clone()).into_stream();
        while let Some(value) = stream.next().await {
            let value = value?.data.collect().await?;
            chunk.extend(&value);
        }
        assert_eq!(chunk, &buf[range.clone()]);

        // if we download the range in large chunks, the result should still be the same
        let mut chunk: Vec<u8> = vec![];
        let mut stream = blob_client
            .get()
            .range(range.clone())
            .chunk_size(0xFFFF_FFFF_FFFFu64)
            .into_stream();
        while let Some(value) = stream.next().await {
            let value = value?.data.collect().await?;
            chunk.extend(&value);
        }
        assert_eq!(chunk, &buf[range.clone()]);

        // download a large blob streaming the individual page to keep memory usage low
        let mut result: Vec<u8> = vec![];
        let mut stream = blob_client
            .get()
            .chunk_size(0xFFFF_FFFF_FFFFu64)
            .into_stream();
        while let Some(value) = stream.next().await {
            let mut data_stream = value?.data;
            while let Some(value) = data_stream.next().await {
                let value = value?;
                result.extend(&value);
            }
        }
        assert_eq!(
            buf, result,
            "streamed blob content should match original buf"
        );

        // if we download the range in tiny chunks, the result should still be the same
        let mut chunk: Vec<u8> = vec![];
        let mut stream = blob_client
            .get()
            .range(range.clone())
            .chunk_size(17u64)
            .into_stream();
        while let Some(value) = stream.next().await {
            let value = value?.data.collect().await?;
            chunk.extend(&value);
        }
        assert_eq!(chunk, &buf[range.clone()]);
    }

    // download the whole blob in 100 byte chunks
    let mut result: Vec<u8> = vec![];
    let mut stream = blob_client.get().chunk_size(100u64).into_stream();
    while let Some(value) = stream.next().await {
        let value = value?.data.collect().await?;
        result.extend(&value);
    }
    assert_eq!(
        buf, result,
        "streamed blob content should match original buf"
    );

    container_client.delete().into_future().await?;

    Ok(())
}

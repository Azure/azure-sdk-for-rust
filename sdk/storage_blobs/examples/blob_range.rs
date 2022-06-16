use azure_core::{error::Result, prelude::*};
use azure_storage::core::prelude::*;
use azure_storage_blobs::prelude::*;
use futures::stream::StreamExt;

#[tokio::main]
async fn main() -> Result<()> {
    // First we retrieve the account name and master key from environment variables.
    let account =
        std::env::var("STORAGE_ACCOUNT").expect("Set env variable STORAGE_ACCOUNT first!");
    let master_key =
        std::env::var("STORAGE_MASTER_KEY").expect("Set env variable STORAGE_MASTER_KEY first!");

    let container = std::env::args()
        .nth(1)
        .expect("please specify container name as command line parameter");
    let blob = std::env::args()
        .nth(2)
        .expect("please specify blob name as command line parameter");

    let http_client = azure_core::new_http_client();
    let blob_client =
        StorageAccountClient::new_access_key(http_client.clone(), &account, &master_key)
            .as_container_client(&container)
            .as_blob_client(&blob);

    // 1024 G, 512 H and 2048 I
    let mut buf: Vec<u8> = Vec::with_capacity(1024 * 4);
    buf.extend([71; 1024]);
    buf.extend([72; 512]);
    buf.extend([73; 2048]);

    let content = std::str::from_utf8(&buf)?.to_owned();
    println!("content == {}", content);

    let _response = blob_client.put_block_blob(buf.clone()).execute().await?;

    let whole = blob_client.get().execute().await?;

    assert_eq!(whole.data.len(), buf.len());

    let chunk0 = blob_client
        .get()
        .range(Range::new(0, 1024))
        .execute()
        .await?;
    assert_eq!(chunk0.data.len(), 1024);
    for i in 0..1024 {
        assert_eq!(chunk0.data[i], 71);
    }

    let chunk1 = blob_client
        .get()
        .range(Range::new(1024, 1536))
        .execute()
        .await?;
    assert_eq!(chunk1.data.len(), 512);
    for i in 0..512 {
        assert_eq!(chunk1.data[i], 72);
    }

    let chunk2 = blob_client
        .get()
        .range(Range::new(1536, 3584))
        .execute()
        .await?;
    assert_eq!(chunk2.data.len(), 2048);
    for i in 0..2048 {
        assert_eq!(chunk2.data[i], 73);
    }

    let mut stream = Box::pin(blob_client.get().stream(512));

    println!("\nStreaming");
    let mut chunk: usize = 0;
    while let Some(value) = stream.next().await {
        let value = value?.data;
        println!("received {:?} bytes", value.len());
        println!("received {}", std::str::from_utf8(&value)?);

        for i in 0..512 {
            assert_eq!(
                value[i],
                match chunk {
                    0 | 1 => 71,
                    2 => 72,
                    _ => 73,
                }
            );
        }

        chunk += 1;
    }

    for dropped_suffix_len in &[3usize, 1] {
        println!("dropped_suffix_len == {}", dropped_suffix_len);
    }

    Ok(())
}

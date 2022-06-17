use azure_storage::core::prelude::*;
use azure_storage_blobs::prelude::*;
use futures::stream::StreamExt;
use std::{num::NonZeroU32, time::Duration};

#[tokio::main]
async fn main() -> azure_core::Result<()> {
    // First we retrieve the account name and master key from environment variables.
    let account =
        std::env::var("STORAGE_ACCOUNT").expect("Set env variable STORAGE_ACCOUNT first!");
    let master_key =
        std::env::var("STORAGE_MASTER_KEY").expect("Set env variable STORAGE_MASTER_KEY first!");

    let container_name = std::env::args()
        .nth(1)
        .expect("please specify container name as command line parameter");

    let http_client = azure_core::new_http_client();
    let storage_client =
        StorageAccountClient::new_access_key(http_client.clone(), &account, &master_key)
            .as_storage_client();
    let blob_service_client = storage_client.as_blob_service_client();
    let container_client = storage_client.as_container_client(&container_name);

    let iv = blob_service_client.list_containers().execute().await?;

    if iv
        .incomplete_vector
        .iter()
        .any(|item| item.name == container_name)
    {
        panic!("The specified container must not exists!");
    }

    // create the container
    container_client
        .create()
        .public_access(PublicAccess::None)
        .timeout(Duration::from_secs(100))
        .execute()
        .await?;
    println!("Container {} created", container_name);

    println!("Checking that container is empty");

    let iv = container_client
        .list_blobs()
        .max_results(NonZeroU32::new(100u32).unwrap())
        .delimiter("/")
        .execute()
        .await?;

    assert!(iv.blobs.blobs.is_empty());

    println!("Adding blobs");

    // create 4 root blobs
    for i in 0..4u8 {
        container_client
            .as_blob_client(format!("blob_at_root{}.txt", i))
            .put_block_blob("somedata")
            .content_type("text/plain")
            .execute()
            .await?;
    }

    // create 3 firstfolder/ blobs
    for i in 0..3u8 {
        container_client
            .as_blob_client(format!("firstfolder/blob_at_1stfolder{}.txt", i))
            .put_block_blob("somedata")
            .content_type("text/plain")
            .execute()
            .await?;
    }

    // create 3 secondroot/ blobs
    for i in 0..3u8 {
        container_client
            .as_blob_client(format!("secondroot/blobsd{}.txt", i))
            .put_block_blob("somedata")
            .content_type("text/plain")
            .execute()
            .await?;
    }

    // create 2 firstfolder/secondfolder blobs
    for i in 0..2u8 {
        container_client
            .as_blob_client(format!("firstfolder/secondfolder/blob{}.txt", i))
            .put_block_blob("somedata")
            .content_type("text/plain")
            .execute()
            .await?;
    }

    // create 4 firstfolder/thirdfolder blobs
    for i in 0..4u8 {
        container_client
            .as_blob_client(format!("firstfolder/thirdfolder/blob{}.txt", i))
            .put_block_blob("somedata")
            .content_type("text/plain")
            .execute()
            .await?;
    }

    // create 4 firstfolder/fourthfolder blobs
    for i in 0..5u8 {
        container_client
            .as_blob_client(format!(
                "firstfolder/thirdfolder/fourthfolder/blob{}.txt",
                i
            ))
            .put_block_blob("somedata")
            .content_type("text/plain")
            .execute()
            .await?;
    }

    let iv = container_client
        .list_blobs()
        .max_results(NonZeroU32::new(100u32).unwrap())
        .delimiter("/")
        .execute()
        .await?;

    println!(
        "List blob / returned {} blobs with blob_prefix == {:?}",
        iv.blobs.blobs.len(),
        iv.blobs.blob_prefix
    );
    iv.blobs.blobs.iter().for_each(|b| println!("\t{}", b.name));
    assert_eq!(iv.blobs.blobs.len(), 4);

    let iv = container_client
        .list_blobs()
        .max_results(NonZeroU32::new(100u32).unwrap())
        .prefix("firstfolder/")
        .delimiter("/")
        .execute()
        .await?;

    println!(
        "List blob firstfolder/ returned {} blobs with blob_prefix == {:?}",
        iv.blobs.blobs.len(),
        iv.blobs.blob_prefix
    );
    iv.blobs.blobs.iter().for_each(|b| println!("\t{}", b.name));
    assert_eq!(iv.blobs.blobs.len(), 3);

    let mut stream = Box::pin(
        container_client
            .list_blobs()
            .max_results(NonZeroU32::new(5u32).unwrap())
            .stream(),
    );

    println!("Streaming results without prefix");
    let mut cnt: i32 = 0;
    while let Some(value) = stream.next().await {
        let len = value?.blobs.blobs.len();
        println!("\treceived {} blobs", len);
        match cnt {
            // we added 21 blobs so 5x4 + 1
            0 | 1 | 2 | 3 => assert_eq!(len, 5),
            4 => assert_eq!(len, 1),
            _ => panic!("more than entries than expected!"),
        }
        cnt += 1;
    }

    container_client.delete().execute().await?;
    println!("Container {} deleted", container_name);

    Ok(())
}

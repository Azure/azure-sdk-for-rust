use azure_storage::prelude::*;
use azure_storage_blobs::prelude::*;
use futures::StreamExt;
use std::num::NonZeroU32;

#[tokio::main]
async fn main() -> azure_core::Result<()> {
    // First we retrieve the account name and access key from environment variables.
    let account =
        std::env::var("STORAGE_ACCOUNT").expect("Set env variable STORAGE_ACCOUNT first!");
    let access_key =
        std::env::var("STORAGE_ACCESS_KEY").expect("Set env variable STORAGE_ACCESS_KEY first!");

    let container_name = std::env::args()
        .nth(1)
        .expect("please specify container name as command line parameter");

    let storage_credentials = StorageCredentials::access_key(account.clone(), access_key);
    let blob_service = BlobServiceClient::new(account, storage_credentials);
    let container_client = blob_service.container_client(&container_name);

    let page = blob_service
        .list_containers()
        .into_stream()
        .next()
        .await
        .expect("stream failed")?;

    if page
        .containers
        .iter()
        .any(|item| item.name == container_name)
    {
        panic!("The specified container must not exists!");
    }

    // create the container
    container_client
        .create()
        .public_access(PublicAccess::None)
        .await?;
    println!("Container {container_name} created");

    println!("Checking that container is empty");

    let page = container_client
        .list_blobs()
        .max_results(NonZeroU32::new(100u32).unwrap())
        .delimiter("/")
        .into_stream()
        .next()
        .await
        .expect("stream failed")?;

    assert!(page.blobs.blobs().next().is_some());

    println!("Adding blobs");

    // create 4 root blobs
    for i in 0..4u8 {
        container_client
            .blob_client(format!("blob_at_root{i}.txt"))
            .put_block_blob("somedata")
            .content_type("text/plain")
            .await?;
    }

    // create 3 firstfolder/ blobs
    for i in 0..3u8 {
        container_client
            .blob_client(format!("firstfolder/blob_at_1stfolder{i}.txt"))
            .put_block_blob("somedata")
            .content_type("text/plain")
            .await?;
    }

    // create 3 secondroot/ blobs
    for i in 0..3u8 {
        container_client
            .blob_client(format!("secondroot/blobsd{i}.txt"))
            .put_block_blob("somedata")
            .content_type("text/plain")
            .await?;
    }

    // create 2 firstfolder/secondfolder blobs
    for i in 0..2u8 {
        container_client
            .blob_client(format!("firstfolder/secondfolder/blob{i}.txt"))
            .put_block_blob("somedata")
            .content_type("text/plain")
            .await?;
    }

    // create 4 firstfolder/thirdfolder blobs
    for i in 0..4u8 {
        container_client
            .blob_client(format!("firstfolder/thirdfolder/blob{i}.txt"))
            .put_block_blob("somedata")
            .content_type("text/plain")
            .await?;
    }

    // create 4 firstfolder/fourthfolder blobs
    for i in 0..5u8 {
        container_client
            .blob_client(format!("firstfolder/thirdfolder/fourthfolder/blob{i}.txt"))
            .put_block_blob("somedata")
            .content_type("text/plain")
            .await?;
    }

    let page = container_client
        .list_blobs()
        .max_results(NonZeroU32::new(100u32).unwrap())
        .delimiter("/")
        .into_stream()
        .next()
        .await
        .expect("stream failed")?;

    println!(
        "List blob / returned {} blobs with blob_prefix == {:?}",
        page.blobs.blobs().count(),
        page.blobs.prefixes().collect::<Vec<_>>()
    );
    page.blobs.blobs().for_each(|b| println!("\t{}", b.name));
    assert_eq!(page.blobs.blobs().count(), 4);

    let page = container_client
        .list_blobs()
        .max_results(NonZeroU32::new(100u32).unwrap())
        .prefix("firstfolder/")
        .delimiter("/")
        .into_stream()
        .next()
        .await
        .expect("stream failed")?;

    println!(
        "List blob firstfolder/ returned {} blobs with blob_prefix == {:?}",
        page.blobs.blobs().count(),
        page.blobs.prefixes().collect::<Vec<_>>()
    );
    page.blobs.blobs().for_each(|b| println!("\t{}", b.name));
    assert_eq!(page.blobs.blobs().count(), 3);

    let mut stream = container_client
        .list_blobs()
        .max_results(NonZeroU32::new(5u32).unwrap())
        .into_stream();

    println!("Streaming results without prefix");
    let mut cnt: i32 = 0;
    while let Some(value) = stream.next().await {
        let len = value?.blobs.blobs().count();
        println!("\treceived {len} blobs");
        match cnt {
            // we added 21 blobs so 5x4 + 1
            0..=3 => assert_eq!(len, 5),
            4 => assert_eq!(len, 1),
            _ => panic!("more than entries than expected!"),
        }
        cnt += 1;
    }

    container_client.delete().await?;
    println!("Container {container_name} deleted");

    Ok(())
}

use azure_core::{prelude::Timeout, Context};
use azure_storage::core::prelude::*;
use azure_storage_blobs::prelude::*;
use futures::StreamExt;
use std::num::NonZeroU32;
use std::time::Duration;

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

    let storage_client = StorageClient::new_access_key(&account, &access_key);
    let blob_service = storage_client.blob_service_client();
    let container_client = storage_client.container_client(&container_name);

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
    let mut context = Context::new();
    context.insert(Timeout::new(Duration::from_secs(100)));
    container_client
        .create()
        .public_access(PublicAccess::None)
        .context(context)
        .into_future()
        .await?;
    println!("Container {} created", container_name);

    // create 10 blobs
    for i in 0..10u8 {
        container_client
            .blob_client(format!("blob{}.txt", i))
            .put_block_blob("somedata")
            .content_type("text/plain")
            .into_future()
            .await?;
        println!("\tAdded blob {}", i);
    }

    let page = container_client
        .list_blobs()
        .max_results(NonZeroU32::new(3u32).unwrap())
        .into_stream()
        .next()
        .await
        .expect("stream failed")?;

    println!("List blob returned {} blobs.", page.blobs.blobs.len());
    for cont in page.blobs.blobs.iter() {
        println!("\t{}\t{} bytes", cont.name, cont.properties.content_length);
    }

    let mut stream = container_client
        .list_blobs()
        .max_results(NonZeroU32::new(3u32).unwrap())
        .into_stream();

    let mut cnt: i32 = 0;
    while let Some(value) = stream.next().await {
        let len = value?.blobs.blobs.len();
        println!("received {} blobs", len);
        match cnt {
            0 | 1 | 2 => assert_eq!(len, 3),
            3 => assert_eq!(len, 1),
            _ => panic!("more than 10 entries??"),
        }
        cnt += 1;
    }

    container_client.delete().into_future().await?;
    println!("Container {} deleted", container_name);

    Ok(())
}

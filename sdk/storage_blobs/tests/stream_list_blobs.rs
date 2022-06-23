#![cfg(all(test, feature = "test_e2e"))]
use azure_storage::core::prelude::*;
use azure_storage_blobs::prelude::*;
use futures::StreamExt;
use std::time::Duration;

#[tokio::test]
async fn stream_list_blobs() {
    // First we retrieve the account name and access key from environment variables.
    let account =
        std::env::var("STORAGE_ACCOUNT").expect("Set env variable STORAGE_ACCOUNT first!");
    let access_key =
        std::env::var("STORAGE_ACCESS_KEY").expect("Set env variable STORAGE_ACCESS_KEY first!");

    let container_name = "streamlistblobs235xx752zdve";

    let http_client = azure_core::new_http_client();

    let storage = StorageAccountClient::new_access_key(http_client.clone(), &account, &access_key)
        .storage_client();
    let blob_service = storage.blob_service_client();
    let container = storage.container_client(container_name);

    let page = blob_service
        .list_containers()
        .into_stream()
        .next()
        .await
        .unwrap()
        .unwrap();

    if page
        .containers
        .iter()
        .find(|item| item.name == container_name)
        .is_some()
    {
        panic!("The specified container must not exists!");
    }

    // create the container
    container
        .create()
        .public_access(PublicAccess::None)
        .timeout(Duration::from_secs(100))
        .into_future()
        .await
        .unwrap();

    // create 10 blobs
    for i in 0..10u8 {
        container
            .blob_client(format!("blob{}.txt", i))
            .put_block_blob("somedata")
            .content_type("text/plain")
            .into_future()
            .await
            .unwrap();
    }

    let mut stream = container
        .list_blobs()
        .max_results(std::num::NonZeroU32::new(3u32).unwrap())
        .into_stream();

    let mut cnt = 0u32;
    while let Some(value) = stream.next().await {
        let len = value.unwrap().blobs.blobs.len();
        println!("received {} blobs", len);
        match cnt {
            0 | 1 | 2 => assert_eq!(len, 3),
            3 => assert_eq!(len, 1),
            _ => panic!("more than 10 entries??"),
        }
        cnt += 1;
    }

    container.delete().into_future().await.unwrap();
}

#![cfg(all(test, feature = "test_e2e"))]
use azure_core::prelude::*;
use azure_storage::blob::prelude::*;
use azure_storage::core::prelude::*;
use futures::stream::StreamExt;
use std::time::Duration;

#[tokio::test]
async fn stream_list_blobs() {
    // First we retrieve the account name and master key from environment variables.
    let account =
        std::env::var("STORAGE_ACCOUNT").expect("Set env variable STORAGE_ACCOUNT first!");
    let master_key =
        std::env::var("STORAGE_MASTER_KEY").expect("Set env variable STORAGE_MASTER_KEY first!");

    let container_name = "streamlistblobs235xx752zdve";

    let http_client = azure_core::new_http_client();

    let storage = StorageAccountClient::new_access_key(http_client.clone(), &account, &master_key)
        .as_storage_client();
    let container = storage.as_container_client(container_name);

    let iv = storage.list_containers().execute().await.unwrap();

    if iv
        .incomplete_vector
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
        .execute()
        .await
        .unwrap();

    // create 10 blobs
    for i in 0..10u8 {
        container
            .as_blob_client(format!("blob{}.txt", i))
            .put_block_blob("somedata")
            .content_type("text/plain")
            .execute()
            .await
            .unwrap();
    }

    let mut stream = Box::pin(
        container
            .list_blobs()
            .max_results(std::num::NonZeroU32::new(3u32).unwrap())
            .stream(),
    );

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

    container.delete().execute().await.unwrap();
}

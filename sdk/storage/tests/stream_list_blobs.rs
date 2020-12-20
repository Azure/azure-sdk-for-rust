#![cfg(all(test, feature = "test_e2e"))]
use azure_core::prelude::*;
use azure_storage::blob::prelude::*;
use azure_storage::core::prelude::*;
use futures::stream::StreamExt;
use std::convert::TryInto;
use std::sync::Arc;
use std::time::Duration;

#[tokio::test]
async fn stream_list_blobs() {
    // First we retrieve the account name and master key from environment variables.
    let account =
        std::env::var("STORAGE_ACCOUNT").expect("Set env variable STORAGE_ACCOUNT first!");
    let master_key =
        std::env::var("STORAGE_MASTER_KEY").expect("Set env variable STORAGE_MASTER_KEY first!");

    let container_name = "streamlistblobs235xx752zdve";

    let http_client: Arc<Box<dyn HttpClient>> = Arc::new(Box::new(reqwest::Client::new()));

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
        .with_public_access(PublicAccess::None)
        .with_timeout(Duration::from_secs(100).into())
        .execute()
        .await
        .unwrap();

    // create 10 blobs
    for i in 0..10u8 {
        container
            .as_blob_client(format!("blob{}.txt", i))
            .put_block_blob("somedata".as_bytes())
            .with_content_type("text/plain".into())
            .execute()
            .await
            .unwrap();
    }

    let mut stream = Box::pin(
        container
            .list_blobs()
            .with_max_results(3u32.try_into().unwrap())
            .stream(),
    );

    let mut cnt = 0;
    while let Some(value) = stream.next().await {
        let len = value.unwrap().incomplete_vector.len();
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

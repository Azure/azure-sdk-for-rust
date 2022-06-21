use azure_storage::core::prelude::*;
use azure_storage_blobs::prelude::*;
use futures::stream::StreamExt;
use std::{num::NonZeroU32, time::Duration};

#[tokio::main]
async fn main() -> azure_core::Result<()> {
    // First we retrieve the account connection string from environment variables.
    let connection_string =
        std::env::var("CONNECTION_STRING").expect("Set env variable CONNECTION_STRING first!");

    let container_name = std::env::args()
        .nth(1)
        .expect("please specify container name as command line parameter");

    let http_client = azure_core::new_http_client();
    let storage_client =
        StorageAccountClient::new_connection_string(http_client.clone(), &connection_string)?
            .as_storage_client();
    let container_client = storage_client.as_container_client(&container_name);
    let blob_service = storage_client.as_blob_service_client();

    let mut stream = Box::pin(blob_service.list_containers().stream());
    while let Some(result) = stream.next().await {
        let container = result?;
        for container in container.incomplete_vector.as_ref() {
            if container.name == container_name {
                panic!("The specified container must not exists!");
            }
        }
    }

    // create the container
    container_client
        .create()
        .public_access(PublicAccess::None)
        .timeout(Duration::from_secs(100))
        .into_future()
        .await?;
    println!("Container {} created", container_name);

    // create 10 blobs
    for i in 0..10u8 {
        container_client
            .as_blob_client(format!("blob{}.txt", i))
            .put_block_blob("somedata")
            .content_type("text/plain")
            .into_future()
            .await?;
        println!("\tAdded blob {}", i);
    }

    let max_results = NonZeroU32::new(3).unwrap();

    let mut stream = Box::pin(
        container_client
            .list_blobs()
            .max_results(max_results)
            .stream(),
    );

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

use azure_storage::ConnectionString;
use azure_storage_blobs::prelude::*;
use futures::StreamExt;
use std::num::NonZeroU32;

#[tokio::main]
async fn main() -> azure_core::Result<()> {
    // First we retrieve the account connection string from environment variables.
    let connection_string =
        std::env::var("CONNECTION_STRING").expect("Set env variable CONNECTION_STRING first!");

    let container_name = std::env::args()
        .nth(1)
        .expect("please specify container name as command line parameter");

    let connection_string = ConnectionString::new(&connection_string)?;
    let blob_service = BlobServiceClient::new(
        connection_string.account_name.unwrap(),
        connection_string.storage_credentials()?,
    );
    let container_client = blob_service.container_client(&container_name);

    let mut stream = blob_service.list_containers().into_stream();
    while let Some(result) = stream.next().await {
        if !result?
            .containers
            .iter()
            .any(|container| container.name == container_name)
        {
            panic!("The specified container must not exists!");
        }
    }

    // create the container
    container_client
        .create()
        .public_access(PublicAccess::None)
        .await?;
    println!("Container {container_name} created");

    // create 10 blobs
    for i in 0..10u8 {
        container_client
            .blob_client(format!("blob{i}.txt"))
            .put_block_blob("somedata")
            .content_type("text/plain")
            .await?;
        println!("\tAdded blob {i}");
    }

    let max_results = NonZeroU32::new(3).unwrap();

    let mut stream = container_client
        .list_blobs()
        .max_results(max_results)
        .into_stream();

    let mut cnt: i32 = 0;
    while let Some(value) = stream.next().await {
        let len = value?.blobs.blobs().count();
        println!("received {len} blobs");
        match cnt {
            0..=2 => assert_eq!(len, 3),
            3 => assert_eq!(len, 1),
            _ => panic!("more than 10 entries??"),
        }
        cnt += 1;
    }

    container_client.delete().await?;
    println!("Container {container_name} deleted");

    Ok(())
}

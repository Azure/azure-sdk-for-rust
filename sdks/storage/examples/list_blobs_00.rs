use azure_sdk_core::prelude::*;
use azure_storage::blob::prelude::*;
use azure_storage::core::prelude::*;
use futures::stream::StreamExt;
use std::error::Error;

#[tokio::main]
async fn main() -> Result<(), Box<dyn Error>> {
    // First we retrieve the account name and master key from environment variables.
    let account =
        std::env::var("STORAGE_ACCOUNT").expect("Set env variable STORAGE_ACCOUNT first!");
    let master_key =
        std::env::var("STORAGE_MASTER_KEY").expect("Set env variable STORAGE_MASTER_KEY first!");

    let container_name = std::env::args()
        .nth(1)
        .expect("please specify container name as command line parameter");

    let client = client::with_access_key(&account, &master_key);

    let iv = client.list_containers().finalize().await?;

    if iv
        .incomplete_vector
        .iter()
        .find(|item| item.name == container_name)
        .is_some()
    {
        panic!("The specified container must not exists!");
    }

    // create the container
    client
        .create_container()
        .with_container_name(&container_name)
        .with_public_access(PublicAccess::None)
        .with_timeout(100)
        .finalize()
        .await?;
    println!("Container {} created", container_name);

    // create 10 blobs
    for i in 0..10u8 {
        client
            .put_block_blob()
            .with_container_name(&container_name)
            .with_blob_name(&format!("blob{}.txt", i))
            .with_content_type("text/plain")
            .with_body("somedata".as_bytes())
            .finalize()
            .await?;
        println!("\tAdded blob {}", i);
    }

    let iv = client
        .list_blobs()
        .with_container_name(&container_name)
        .with_max_results(3)
        .finalize()
        .await?;

    println!("List blob returned {} blobs.", iv.incomplete_vector.len());
    for cont in iv.incomplete_vector.iter() {
        println!("\t{}\t{} bytes", cont.name, cont.content_length);
    }

    let mut stream = Box::pin(
        client
            .list_blobs()
            .with_max_results(3)
            .with_container_name(&container_name)
            .stream(),
    );

    let mut cnt = 0;
    while let Some(value) = stream.next().await {
        let len = value?.incomplete_vector.len();
        println!("received {} blobs", len);
        match cnt {
            0 | 1 | 2 => assert_eq!(len, 3),
            3 => assert_eq!(len, 1),
            _ => panic!("more than 10 entries??"),
        }
        cnt += 1;
    }

    client
        .delete_container()
        .with_container_name(&container_name)
        .finalize()
        .await?;
    println!("Container {} deleted", container_name);

    Ok(())
}

use azure_core::prelude::*;
use azure_storage::blob::prelude::*;
use azure_storage::core::prelude::*;
use futures::stream::StreamExt;
use std::convert::TryInto;
use std::error::Error;
use std::sync::Arc;
use std::time::Duration;

#[tokio::main]
async fn main() -> Result<(), Box<dyn Error + Send + Sync>> {
    // First we retrieve the account name and master key from environment variables.
    let account =
        std::env::var("STORAGE_ACCOUNT").expect("Set env variable STORAGE_ACCOUNT first!");
    let master_key =
        std::env::var("STORAGE_MASTER_KEY").expect("Set env variable STORAGE_MASTER_KEY first!");

    let container_name = std::env::args()
        .nth(1)
        .expect("please specify container name as command line parameter");

    let client = client::with_access_key(&account, &master_key);

    let http_client: Arc<Box<dyn HttpClient>> = Arc::new(Box::new(reqwest::Client::new()));
    let storage_account =
        StorageAccountClient::new_access_key(http_client.clone(), &account, &master_key)
            .as_storage_client();
    let container = storage_account.as_container_client(&container_name);

    let iv = storage_account.list_containers().execute().await?;

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

    let iv = container
        .list_blobs()
        .with_max_results(3u32.try_into()?)
        .execute()
        .await?;

    println!("List blob returned {} blobs.", iv.incomplete_vector.len());
    for cont in iv.incomplete_vector.iter() {
        println!("\t{}\t{} bytes", cont.name, cont.content_length);
    }

    let mut stream = Box::pin(
        container
            .list_blobs()
            .with_max_results(3u32.try_into()?)
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

    container.delete().execute().await?;
    println!("Container {} deleted", container_name);

    Ok(())
}

use azure_core::prelude::*;
use azure_storage::blob::prelude::*;
use azure_storage::core::prelude::*;
use std::sync::Arc;

// This example shows how to stream data from a blob. We will create a simple blob first, the we
// ask it back using streaming features of the future crate. In this simple example we just
// concatenate the data received in order to make sure the retrieved blob is equals to the one
// created in the first place.
// We do not use leases here but you definitely want to do so otherwise the returned stream
// is not guaranteed to be consistent.
#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error + Send + Sync>> {
    // First we retrieve the account name and master key from environment variables.
    let account =
        std::env::var("STORAGE_ACCOUNT").expect("Set env variable STORAGE_ACCOUNT first!");
    let master_key =
        std::env::var("STORAGE_MASTER_KEY").expect("Set env variable STORAGE_MASTER_KEY first!");

    let container_name = std::env::args()
        .nth(1)
        .expect("please specify container name as first command line parameter");

    let file_name = std::env::args()
        .nth(2)
        .expect("please specify container name as second command line parameter");

    let https = hyper_rustls::HttpsConnector::with_native_roots();
    let client: hyper::Client<_, hyper::Body> = hyper::Client::builder().build(https);
    let http_client: Arc<Box<dyn HttpClient>> = Arc::new(Box::new(client));

    // uncomment below to test reqwest
    //let http_client: Arc<Box<dyn HttpClient>> = Arc::new(Box::new(reqwest::Client::new()));

    let storage_account_client =
        StorageAccountClient::new_access_key(http_client.clone(), &account, &master_key);
    let storage_client = storage_account_client.as_storage_client();
    let blob = storage_client
        .as_container_client(&container_name)
        .as_blob_client(file_name);

    let mut response = blob.get().stream().await?;

    let mut total: u64 = 0;

    while let Some(data) = response.data.chunk().await? {
        total += data.len() as u64;
        println!(
            "got {} bytes! (total {} MB)",
            data.len(),
            (total / (1024 * 1024))
        );
    }

    Ok(())
}

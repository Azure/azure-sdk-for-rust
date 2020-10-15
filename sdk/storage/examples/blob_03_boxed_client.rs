#[macro_use]
extern crate log;

use azure_core::errors::AzureError;
use azure_core::prelude::*;
use azure_storage::blob::prelude::*;
use azure_storage::core::prelude::*;
use std::error::Error;
use std::sync::Arc;

#[tokio::main]
async fn main() -> Result<(), Box<dyn Error>> {
    // First we retrieve the account name and master key from environment variables.
    let account =
        std::env::var("STORAGE_ACCOUNT").expect("Set env variable STORAGE_ACCOUNT first!");
    let master_key =
        std::env::var("STORAGE_MASTER_KEY").expect("Set env variable STORAGE_MASTER_KEY first!");

    let container = std::env::args()
        .nth(1)
        .expect("please specify container name as command line parameter");
    let blob = std::env::args()
        .nth(2)
        .expect("please specify blob name as command line parameter");

    let client: Box<dyn Client> = Box::new(client::with_access_key(&account, &master_key));
    let s_content = get_blob_box(&client, &container, &blob).await?;
    println!("blob == {:?}", blob);
    println!("s_content == {}", s_content);

    let client: Arc<dyn Client> = Arc::new(client::with_access_key(&account, &master_key));
    let s_content = get_blob_arc(client, &container, &blob).await?;
    println!("blob == {:?}", blob);
    println!("s_content == {}", s_content);

    Ok(())
}

async fn get_blob_box<'a>(
    client: &'a Box<dyn Client>,
    container: &'a str,
    blob: &'a str,
) -> Result<String, AzureError> {
    trace!("Requesting blob");

    let response = client
        .get_blob()
        .with_container_name(&container)
        .with_blob_name(&blob)
        .finalize()
        .await?;

    Ok(String::from_utf8(response.data)?)
}

async fn get_blob_arc<'a>(
    client: Arc<dyn Client>,
    container: &'a str,
    blob: &'a str,
) -> Result<String, AzureError> {
    trace!("Requesting blob");

    let response = client
        .get_blob()
        .with_container_name(&container)
        .with_blob_name(&blob)
        .finalize()
        .await?;

    Ok(String::from_utf8(response.data)?)
}

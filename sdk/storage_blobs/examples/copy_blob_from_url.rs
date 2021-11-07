use azure_storage::core::prelude::*;
use azure_storage_blobs::prelude::*;
use std::error::Error;

#[tokio::main]
async fn main() -> Result<(), Box<dyn Error + Send + Sync>> {
    // First we retrieve the account name and master key from environment variables.
    let account =
        std::env::var("STORAGE_ACCOUNT").expect("Set env variable STORAGE_ACCOUNT first!");
    let master_key =
        std::env::var("STORAGE_MASTER_KEY").expect("Set env variable STORAGE_MASTER_KEY first!");

    let source_container = std::env::args()
        .nth(1)
        .expect("please specify source container name as first command line parameter");
    let source_blob = std::env::args()
        .nth(2)
        .expect("please specify source blob name as second command line parameter");
    let destination_container = std::env::args()
        .nth(3)
        .expect("please specify destination container name as third command line parameter");
    let destination_blob = std::env::args()
        .nth(4)
        .expect("please specify destination blob name as fourth command line parameter");

    let http_client = azure_core::new_http_client();

    let storage_account_client =
        StorageAccountClient::new_access_key(http_client.clone(), &account, &master_key);
    let storage_client = storage_account_client.as_storage_client();
    let blob = storage_client
        .as_container_client(&destination_container)
        .as_blob_client(&destination_blob);

    let source_url = format!(
        "{}/{}/{}",
        storage_account_client.blob_storage_url().as_str(),
        source_container,
        source_blob
    );

    let response = blob
        .copy_from_url(&source_url)
        .is_synchronous(true)
        .execute()
        .await?;

    println!("response == {:?}", response);

    Ok(())
}

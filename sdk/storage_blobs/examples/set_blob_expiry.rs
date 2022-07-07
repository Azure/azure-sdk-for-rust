use azure_storage::core::prelude::*;
use azure_storage_blobs::prelude::*;

#[tokio::main]
async fn main() -> azure_core::Result<()> {
    // First we retrieve the account name and master key from environment variables.
    let account =
        std::env::var("STORAGE_ACCOUNT").expect("Set env variable STORAGE_ACCOUNT first!");
    let master_key =
        std::env::var("STORAGE_ACCESS_KEY").expect("Set env variable STORAGE_ACCESS_KEY first!");

    let container = std::env::args()
        .nth(1)
        .expect("please specify container name as command line parameter");
    let blob = std::env::args()
        .nth(2)
        .expect("please specify blob name as command line parameter");

    let storage_client = StorageClient::new_access_key(&account, &master_key);

    let blob_client = storage_client
        .container_client(&container)
        .blob_client(&blob);

    let response = blob_client
        .set_blob_expiry(BlobExpiry::RelativeToNow(1000 * 60 * 60 * 24 * 100))
        .into_future()
        .await?;

    println!("response: {:?}", response);

    Ok(())
}

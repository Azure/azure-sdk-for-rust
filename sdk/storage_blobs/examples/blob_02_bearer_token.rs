#[macro_use]
extern crate log;

use azure_storage::prelude::*;
use azure_storage_blobs::prelude::*;

#[tokio::main]
async fn main() -> azure_core::Result<()> {
    // First we retrieve the account name and access key from environment variables.

    let account = std::env::args()
        .nth(1)
        .expect("please specify the account name as first command line parameter");
    let container = std::env::args()
        .nth(2)
        .expect("please specify the container name as second command line parameter");
    let blob = std::env::args()
        .nth(3)
        .expect("please specify the blob name as third command line parameter");
    let bearer_token = std::env::args()
        .nth(4)
        .expect("please specify the bearer token as fourth command line parameter");

    let storage_credentials = StorageCredentials::bearer_token(bearer_token);
    let blob_client = BlobServiceClient::new(account, storage_credentials)
        .container_client(&container)
        .blob_client(&blob);

    trace!("Requesting blob");

    let blob = blob_client.get_content().await?;
    println!("response == {blob:?}");

    Ok(())
}

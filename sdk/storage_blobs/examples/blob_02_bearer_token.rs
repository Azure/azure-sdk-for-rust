#[macro_use]
extern crate log;

use azure_storage::core::prelude::*;
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

    let http_client = azure_core::new_http_client();
    let blob_client = StorageClient::new_bearer_token(http_client.clone(), &account, bearer_token)
        .container_client(&container)
        .blob_client(&blob);

    trace!("Requesting blob");

    let blob = blob_client.get_content().await?;
    println!("response == {:?}", blob);

    Ok(())
}

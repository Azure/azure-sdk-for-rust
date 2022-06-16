#[macro_use]
extern crate log;

use azure_core::error::{ErrorKind, Result, ResultExt};
use azure_storage::core::prelude::*;
use azure_storage_blobs::prelude::*;

#[tokio::main]
async fn main() -> Result<()> {
    // First we retrieve the account name and master key from environment variables.

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
    let blob_client =
        StorageAccountClient::new_bearer_token(http_client.clone(), &account, bearer_token)
            .as_container_client(&container)
            .as_blob_client(&blob);

    trace!("Requesting blob");

    let response = blob_client.get().execute().await?;

    let s_content =
        String::from_utf8(response.data.to_vec()).map_kind(ErrorKind::DataConversion)?;
    println!("blob == {:?}", blob);
    println!("s_content == {}", s_content);

    Ok(())
}

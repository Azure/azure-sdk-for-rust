#[macro_use]
extern crate log;

use azure_sdk_core::prelude::*;
use azure_sdk_storage_blob::prelude::*;
use azure_sdk_storage_core::prelude::*;
use std::error::Error;

#[tokio::main]
async fn main() -> Result<(), Box<dyn Error>> {
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

    let client = client::with_bearer_token(account, bearer_token);

    trace!("Requesting blob");

    let response = client
        .get_blob()
        .with_container_name(&container)
        .with_blob_name(&blob)
        .finalize()
        .await?;

    let s_content = String::from_utf8(response.data)?;
    println!("blob == {:?}", blob);
    println!("s_content == {}", s_content);

    Ok(())
}

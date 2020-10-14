#[macro_use]
extern crate log;

use azure_identity::{DefaultCredential, TokenCredential};
use azure_core::prelude::*;
use azure_storage::blob::prelude::*;
use azure_storage::core::prelude::*;
use std::error::Error;

#[tokio::main]
async fn main() -> Result<(), Box<dyn Error>> {
    env_logger::init();
    // First we retrieve the account name, container and blob name from command line args

    let account = std::env::args()
        .nth(1)
        .expect("please specify the account name as first command line parameter");
    let container = std::env::args()
        .nth(2)
        .expect("please specify the container name as second command line parameter");
    let blob = std::env::args()
        .nth(3)
        .expect("please specify the blob name as third command line parameter");

    let bearer_token = DefaultCredential::default()
        .get_token("https://storage.azure.com/")
        .await?;

    let client = client::with_bearer_token(account, bearer_token.token.secret());

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

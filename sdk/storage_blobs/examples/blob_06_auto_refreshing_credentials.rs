#[macro_use]
extern crate log;

use azure_core::error::{ErrorKind, ResultExt};
use azure_identity::{AutoRefreshingTokenCredential, DefaultAzureCredential};
use azure_storage::core::prelude::*;
use azure_storage_blobs::prelude::*;
use std::sync::Arc;

#[tokio::main]
async fn main() -> azure_core::Result<()> {
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

    let creds = Arc::new(DefaultAzureCredential::default());
    let auto_creds = Arc::new(AutoRefreshingTokenCredential::new(creds));

    let http_client = azure_core::new_http_client();
    let blob_client =
        StorageClient::new_token_credential(http_client.clone(), &account, auto_creds)
            .container_client(&container)
            .blob_client(&blob);

    trace!("Requesting blob");
    let content = blob_client.get_content().await?;
    println!("blob == {:?}", content);

    let s_content = String::from_utf8(content).map_kind(ErrorKind::DataConversion)?;
    println!("s_content == {}", s_content);

    Ok(())
}

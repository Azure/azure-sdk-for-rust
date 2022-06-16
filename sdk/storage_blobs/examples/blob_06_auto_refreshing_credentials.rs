#[macro_use]
extern crate log;

use azure_core::error::{ErrorKind, Result, ResultExt};
use azure_identity::{AutoRefreshingTokenCredential, DefaultAzureCredential};
use azure_storage::core::prelude::*;
use azure_storage_blobs::prelude::*;
use std::sync::Arc;

#[tokio::main]
async fn main() -> Result<()> {
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
        StorageAccountClient::new_token_credential(http_client.clone(), &account, auto_creds)
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

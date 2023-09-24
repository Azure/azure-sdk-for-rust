use azure_core::{
    auth::TokenCredential,
    error::{ErrorKind, ResultExt},
};
use azure_identity::DefaultAzureCredential;
use azure_storage::StorageCredentials;
use azure_storage_blobs::prelude::BlobServiceClient;
use log::trace;
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

    let default_creds: Arc<dyn TokenCredential> = Arc::new(DefaultAzureCredential::default());
    let credentials = StorageCredentials::token_credential(default_creds);
    let blob_client = BlobServiceClient::new(account, credentials)
        .container_client(&container)
        .blob_client(&blob);

    trace!("Requesting blob");

    let blob = blob_client.get_content().await?;
    println!("blob == {blob:?}");

    let s_content = String::from_utf8(blob).map_kind(ErrorKind::DataConversion)?;
    println!("s_content == {s_content}");

    Ok(())
}

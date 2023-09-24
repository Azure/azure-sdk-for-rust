use azure_core::auth::TokenCredential;
use azure_identity::DefaultAzureCredential;
use azure_storage::StorageCredentials;
use azure_storage_blobs::prelude::BlobServiceClient;
use clap::Parser;
use std::{sync::Arc, time::Duration};
use time::OffsetDateTime;

#[derive(Debug, Parser)]
struct Args {
    /// storage account name
    #[clap(env = "STORAGE_ACCOUNT")]
    account: String,
}

#[tokio::main]
async fn main() -> azure_core::Result<()> {
    env_logger::init();
    let args = Args::parse();

    let default_creds: Arc<dyn TokenCredential> = Arc::new(DefaultAzureCredential::default());
    let credentials = StorageCredentials::token_credential(default_creds);
    let client = BlobServiceClient::new(&args.account, credentials);

    let start = OffsetDateTime::now_utc();
    let expiry = start + Duration::from_secs(60 * 60);
    let response = client.get_user_deligation_key(start, expiry).await?;
    println!("{:#?}", response.user_deligation_key);
    Ok(())
}

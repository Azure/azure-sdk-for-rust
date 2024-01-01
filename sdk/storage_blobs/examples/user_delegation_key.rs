use azure_storage::{shared_access_signature::service_sas::BlobSasPermissions, StorageCredentials};
use azure_storage_blobs::prelude::BlobServiceClient;
use clap::Parser;
use std::time::Duration;
use time::OffsetDateTime;

#[derive(Debug, Parser)]
struct Args {
    /// storage account name
    #[clap(env = "STORAGE_ACCOUNT")]
    account: String,

    /// storage container name
    container: String,

    /// storage blob name
    #[clap(long)]
    blob: Option<String>,
}

#[tokio::main]
async fn main() -> azure_core::Result<()> {
    env_logger::init();
    let args = Args::parse();

    let default_creds = azure_identity::new_credential();
    let credentials = StorageCredentials::token_credential(default_creds);
    let client = BlobServiceClient::new(&args.account, credentials);

    let start = OffsetDateTime::now_utc();
    let expiry = start + Duration::from_secs(60 * 60);
    let response = client.get_user_deligation_key(start, expiry).await?;

    let container = client.container_client(&args.container);

    if !container.exists().await? {
        container.create().await?;
    }

    if let Some(blob) = args.blob {
        let blob = container.blob_client(blob);
        let sas = blob
            .user_delegation_shared_access_signature(
                BlobSasPermissions {
                    read: true,
                    ..Default::default()
                },
                &response.user_deligation_key,
            )
            .await?;
        let url = blob.generate_signed_blob_url(&sas)?;
        println!("blob url: {url}");
    } else {
        let sas = container
            .user_delegation_shared_access_signature(
                BlobSasPermissions {
                    read: true,
                    ..Default::default()
                },
                &response.user_deligation_key,
            )
            .await?;
        let url = container.generate_signed_container_url(&sas)?;
        println!("blob url: {url}");
    }

    Ok(())
}

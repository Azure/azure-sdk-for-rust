// Copyright (c) Microsoft Corporation. All rights reserved.
// Licensed under the MIT License.

use azure_identity::DeveloperToolsCredential;
use azure_storage_blob::BlobContainerClient;
use clap::Parser;
use futures::TryStreamExt;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let args = Args::parse();

    let credential = DeveloperToolsCredential::new(None)?;
    let endpoint = format!("https://{}.blob.core.windows.net/", args.account_name);

    // The reqwest/native-tls feature automatically configures native-tls as the TLS provider.
    let container_client =
        BlobContainerClient::new(&endpoint, &args.container_name, Some(credential), None)?;

    // Iterate through all pages of blobs in the container.
    let mut pager = container_client.list_blobs(None)?.into_pages();
    while let Some(page) = pager.try_next().await? {
        let response = page.into_model()?;
        for blob in &response.segment.blob_items {
            if let Some(name) = &blob.name {
                println!("{name}");
            }
        }
    }

    Ok(())
}

#[derive(Parser)]
struct Args {
    /// Azure Storage account name.
    #[arg(long, env = "AZURE_STORAGE_ACCOUNT_NAME")]
    account_name: String,

    /// Blob container name to list.
    #[arg(long, default_value = "samples")]
    container_name: String,
}

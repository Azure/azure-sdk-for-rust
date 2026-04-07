// Copyright (c) Microsoft Corporation. All rights reserved.
// Licensed under the MIT License.

use azure_core::http::{ClientOptions, Transport};
use azure_identity::DeveloperToolsCredential;
use azure_storage_blob::{BlobContainerClient, BlobContainerClientOptions};
use clap::Parser;
use futures::TryStreamExt;
use std::sync::Arc;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let args = Args::parse();

    // Build a reqwest client using rustls with the default aws-lc-rs provider.
    // Disable redirects to match the Azure SDK's default behavior.
    let http_client = reqwest::ClientBuilder::new()
        .redirect(reqwest::redirect::Policy::none())
        .build()?;

    let credential = DeveloperToolsCredential::new(None)?;
    let endpoint = format!("https://{}.blob.core.windows.net/", args.account_name);

    // Pass the custom reqwest client to the Azure SDK via Transport.
    let options = BlobContainerClientOptions {
        client_options: ClientOptions {
            transport: Some(Transport::new(Arc::new(http_client))),
            ..Default::default()
        },
        ..Default::default()
    };

    let container_client = BlobContainerClient::new(
        &endpoint,
        &args.container_name,
        Some(credential),
        Some(options),
    )?;

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

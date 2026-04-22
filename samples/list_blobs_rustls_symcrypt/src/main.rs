// Copyright (c) Microsoft Corporation. All rights reserved.
// Licensed under the MIT License.

use azure_core::http::{ClientOptions, Transport};
use azure_identity::DeveloperToolsCredential;
use azure_storage_blob::{BlobContainerClient, BlobContainerClientOptions};
use clap::Parser;
use futures::TryStreamExt;
use rustls::{ClientConfig, RootCertStore};
use std::sync::Arc;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let args = Args::parse();

    // Build a rustls ClientConfig using the SymCrypt crypto provider and the
    // webpki-roots certificate store. This ensures that aws-lc-rs is not
    // pulled in as a dependency and used for TLS when SymCrypt must be used.
    //
    // A simpler alternative is to install SymCrypt as the default provider:
    //
    //   rustls_symcrypt::default_symcrypt_provider()
    //       .install_default()
    //       .expect("failed to install SymCrypt crypto provider");
    //
    // However, that approach requires reqwest's `rustls` feature, which pulls
    // in aws-lc-rs. Using `rustls-no-provider` with an explicitly constructed
    // ClientConfig ensures only SymCrypt is used for TLS.
    let root_store = RootCertStore {
        roots: webpki_roots::TLS_SERVER_ROOTS.to_vec(),
    };
    let tls_config =
        ClientConfig::builder_with_provider(Arc::new(rustls_symcrypt::default_symcrypt_provider()))
            .with_safe_default_protocol_versions()?
            .with_root_certificates(root_store)
            .with_no_client_auth();

    let reqwest_client = reqwest::ClientBuilder::new()
        .redirect(reqwest::redirect::Policy::none())
        .use_preconfigured_tls(tls_config)
        .build()?;

    let options = BlobContainerClientOptions {
        client_options: ClientOptions {
            transport: Some(Transport::new(Arc::new(reqwest_client))),
            ..Default::default()
        },
        ..Default::default()
    };

    let credential = DeveloperToolsCredential::new(None)?;
    let endpoint = format!("https://{}.blob.core.windows.net/", args.account_name);

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

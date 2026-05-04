// Copyright (c) Microsoft Corporation. All rights reserved.
// Licensed under the MIT License.

use std::{
    sync::{
        atomic::{AtomicUsize, Ordering},
        Arc, LazyLock,
    },
    time::Duration,
};

use azure_core::{
    credentials::{AccessToken, TokenCredential, TokenRequestOptions},
    error::{ErrorKind, ResultExt},
    http::{ClientOptions, Transport, Url},
    Result,
};
use azure_identity::{DeveloperToolsCredential, ManagedIdentityCredential};
use azure_storage_blob::{BlobContainerClient, BlobContainerClientOptions};
use azure_storage_blob_test::fault_injection::{FaultInjectionPolicy, FaultInjectionProbabilities};
use rand::rngs::SmallRng;

pub fn get_container_client(
    fault_options: &FaultInjectionProbabilities,
) -> Result<BlobContainerClient> {
    let account_name = std::env::var("AZURE_STORAGE_ACCOUNT_NAME").with_context(
        ErrorKind::Other,
        "Configure `AZURE_STORAGE_ACCOUNT_NAME` environment variable.",
    )?;
    let container_name = uuid::Uuid::new_v4().to_string();

    let client_options = if fault_options.is_zero() {
        None
    } else {
        Some(BlobContainerClientOptions {
            client_options: ClientOptions {
                per_try_policies: vec![Arc::new(FaultInjectionPolicy::<SmallRng>::new(
                    Url::parse("https://127.0.0.1:7778").unwrap(),
                    fault_options.clone(),
                )?)],
                transport: Some(FAULT_HTTP_CLIENT.clone()),
                ..Default::default()
            },
            ..Default::default()
        })
    };

    BlobContainerClient::from_url(
        Url::parse(
            format!(
                "https://{}.blob.core.windows.net/{}",
                account_name, container_name
            )
            .as_str(),
        )?,
        Some(create_credential()?),
        client_options,
    )
}

/// Creates a chained credential that tries DeveloperToolsCredential first (for local dev),
/// then falls back to ManagedIdentityCredential (for Azure-hosted environments).
fn create_credential() -> Result<Arc<dyn TokenCredential>> {
    let sources: Vec<Arc<dyn TokenCredential>> = vec![
        DeveloperToolsCredential::new(None)?,
        ManagedIdentityCredential::new(None)?,
    ];
    Ok(Arc::new(ChainedCredential::new(sources)))
}

/// A credential that chains multiple TokenCredential implementations.
/// Tries each credential in order until one succeeds, then caches that credential for future use.
struct ChainedCredential {
    sources: Vec<Arc<dyn TokenCredential>>,
    cached_source_index: AtomicUsize,
}

impl ChainedCredential {
    fn new(sources: Vec<Arc<dyn TokenCredential>>) -> Self {
        Self {
            sources,
            cached_source_index: AtomicUsize::new(usize::MAX),
        }
    }
}

impl std::fmt::Debug for ChainedCredential {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.write_str("ChainedCredential")
    }
}

#[async_trait::async_trait]
impl TokenCredential for ChainedCredential {
    async fn get_token(
        &self,
        scopes: &[&str],
        options: Option<TokenRequestOptions<'_>>,
    ) -> Result<AccessToken> {
        // Check if we have a cached credential that worked before
        let cached_index = self.cached_source_index.load(Ordering::Relaxed);
        if cached_index != usize::MAX {
            if let Some(source) = self.sources.get(cached_index) {
                return source.get_token(scopes, options).await;
            }
        }

        // Try each credential in order
        let mut errors = Vec::new();
        for (index, source) in self.sources.iter().enumerate() {
            match source.get_token(scopes, options.clone()).await {
                Ok(token) => {
                    self.cached_source_index.store(index, Ordering::Relaxed);
                    return Ok(token);
                }
                Err(error) => errors.push(error),
            }
        }

        Err(azure_core::Error::with_message(
            azure_core::error::ErrorKind::Credential,
            format!(
                "All credentials failed. Errors: {}",
                errors
                    .iter()
                    .map(|e| e.to_string())
                    .collect::<Vec<_>>()
                    .join("; ")
            ),
        ))
    }
}

// Build a custom reqwest client that accepts the fault injector's
// self-signed TLS certificate and disables automatic decompression
// (required for partitioned downloads to work correctly).
static FAULT_HTTP_CLIENT: LazyLock<Transport> = LazyLock::new(|| {
    Transport::new(Arc::new(
        ::reqwest::ClientBuilder::new()
            .danger_accept_invalid_certs(true)
            .read_timeout(Duration::from_secs(10))
            .no_gzip()
            .no_brotli()
            .no_deflate()
            .no_zstd()
            .build()
            .expect("build reqwest client"),
    ))
});

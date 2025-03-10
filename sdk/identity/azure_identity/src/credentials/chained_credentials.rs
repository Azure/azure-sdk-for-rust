// Copyright (c) Microsoft Corporation. All rights reserved.
// Licensed under the MIT License.

use crate::credentials::cache::TokenCache;
use azure_core::{
    credentials::{AccessToken, TokenCredential},
    error::{Error, ErrorKind},
};
use std::sync::Arc;

/// Provides a mechanism of selectively adding credentials used for a `ChainedTokenCredential` instance
#[derive(Default)]
pub struct ChainedTokenCredentialBuilder {
    sources: Vec<Arc<dyn TokenCredential>>,
}

impl ChainedTokenCredentialBuilder {
    /// Create a new `ChainedTokenCredentialBuilder` with default options.
    pub fn new() -> Self {
        Self::default()
    }

    pub fn add_source(&mut self, credential: Arc<dyn TokenCredential>) {
        self.sources.push(credential);
    }

    /// Create a `ChainedTokenCredential` from this builder.
    pub fn build(self) -> Arc<ChainedTokenCredential> {
        ChainedTokenCredential::with_sources(self.sources)
    }
}

/// Provides a user-configurable `TokenCredential` authentication flow for applications that will be deployed to Azure.
///
/// The credential types are tried in the order specified by the user.
#[derive(Debug)]
pub struct ChainedTokenCredential {
    sources: Vec<Arc<dyn TokenCredential>>,
    cache: TokenCache,
}

impl ChainedTokenCredential {
    /// Create a [`ChainedTokenCredentialBuilder`] to create a `ChainedTokenCredential` with options.
    pub fn builder() -> ChainedTokenCredentialBuilder {
        ChainedTokenCredentialBuilder::new()
    }

    /// Creates a `ChainedTokenCredential` with specified sources.
    fn with_sources(sources: Vec<Arc<dyn TokenCredential>>) -> Arc<Self> {
        Arc::new(Self {
            sources,
            cache: TokenCache::new(),
        })
    }

    /// Try to fetch a token using each of the credential sources until one succeeds
    async fn get_token(&self, scopes: &[&str]) -> azure_core::Result<AccessToken> {
        let mut errors = Vec::new();
        for source in &self.sources {
            let token_res = source.get_token(scopes).await;

            match token_res {
                Ok(token) => return Ok(token),
                Err(error) => errors.push(error),
            }
        }
        Err(Error::with_message(ErrorKind::Credential, || {
            format!(
                "Multiple errors were encountered while attempting to authenticate:\n{}",
                format_aggregate_error(&errors)
            )
        }))
    }
}

#[cfg_attr(target_arch = "wasm32", async_trait::async_trait(?Send))]
#[cfg_attr(not(target_arch = "wasm32"), async_trait::async_trait)]
impl TokenCredential for ChainedTokenCredential {
    async fn get_token(&self, scopes: &[&str]) -> azure_core::Result<AccessToken> {
        self.cache.get_token(scopes, self.get_token(scopes)).await
    }

    /// Clear the credential's cache.
    async fn clear_cache(&self) -> azure_core::Result<()> {
        // clear the internal cache as well as each of the underlying providers
        self.cache.clear().await?;

        for source in &self.sources {
            source.clear_cache().await?;
        }

        Ok(())
    }
}

fn format_aggregate_error(errors: &[Error]) -> String {
    use std::error::Error;
    errors
        .iter()
        .map(|e| {
            let mut current: Option<&dyn Error> = Some(e);
            let mut stack = vec![];
            while let Some(err) = current.take() {
                stack.push(err.to_string());
                current = err.source();
            }
            stack.join(" - ")
        })
        .collect::<Vec<String>>()
        .join("\n")
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::credentials::AppServiceManagedIdentityCredential;
    use crate::credentials::AzureCliCredential;
    use crate::TokenCredentialOptions;

    #[test]
    fn test_builder_included_credential_flags() -> azure_core::Result<()> {
        let mut builder = ChainedTokenCredentialBuilder::new();
        #[cfg(not(target_arch = "wasm32"))]
        {
            builder.add_source(AzureCliCredential::new()?);
        }

        builder.add_source(AppServiceManagedIdentityCredential::new(
            TokenCredentialOptions::default(),
        )?);

        builder.build();
        Ok(())
    }
}

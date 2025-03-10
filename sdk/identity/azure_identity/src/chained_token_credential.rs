// Copyright (c) Microsoft Corporation. All rights reserved.
// Licensed under the MIT License.

use crate::credentials::cache::TokenCache;
use crate::TokenCredentialOptions;
use azure_core::{
    credentials::{AccessToken, TokenCredential},
    error::{Error, ErrorKind},
};
use std::sync::Arc;

#[derive(Debug, Default)]
pub struct ChainedTokenCredentialOptions {
    pub credential_options: TokenCredentialOptions,
}

// TODO: Should probably remove this once we consolidate and unify credentials.
impl From<TokenCredentialOptions> for ChainedTokenCredentialOptions {
    fn from(credential_options: TokenCredentialOptions) -> Self {
        Self {
            credential_options
        }
    }
}


/// Provides a user-configurable `TokenCredential` authentication flow for applications that will be deployed to Azure.
///
/// The credential types are tried in the order specified by the user.
#[derive(Debug)]
pub struct ChainedTokenCredential {
    #[allow(dead_code)]
    options: ChainedTokenCredentialOptions,
    sources: Vec<Arc<dyn TokenCredential>>,
    cache: TokenCache,
}

impl ChainedTokenCredential {
    /// Create a `ChainedTokenCredential` with options.
    pub fn new(options: Option<ChainedTokenCredentialOptions>) -> Self {
        Self {
            options: options.unwrap_or_default(),
            sources: Vec::new(),
            cache: TokenCache::new(),
        }
    }

    /// Add a credential source to the chain.
    pub fn add_source(&mut self, source: Arc<dyn TokenCredential>) {
        self.sources.push(source);
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
    use crate::credentials::AzureCliCredential;

    #[test]
    fn test_adding_azure_cli() -> azure_core::Result<()> {
        let mut credential = ChainedTokenCredential::new(None);
        #[cfg(not(target_arch = "wasm32"))]
        {
            credential.add_source(AzureCliCredential::new()?);
        }

        Ok(())
    }
}

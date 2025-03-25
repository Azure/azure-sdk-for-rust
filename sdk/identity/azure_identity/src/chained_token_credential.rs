// Copyright (c) Microsoft Corporation. All rights reserved.
// Licensed under the MIT License.

use crate::TokenCredentialOptions;
use async_lock::RwLock;
use azure_core::{
    credentials::{AccessToken, TokenCredential},
    error::{Error, ErrorKind},
};
use std::sync::Arc;

#[derive(Debug, Default)]
/// ChainedTokenCredentialOptions contains optional parameters for ChainedTokenCredential.
pub struct ChainedTokenCredentialOptions {
    pub retry_sources: bool,
    pub credential_options: TokenCredentialOptions,
}

// TODO: Should probably remove this once we consolidate and unify credentials.
impl From<TokenCredentialOptions> for ChainedTokenCredentialOptions {
    fn from(credential_options: TokenCredentialOptions) -> Self {
        Self {
            retry_sources: Default::default(),
            credential_options,
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
    successful_credential: RwLock<Option<Arc<dyn TokenCredential>>>,
}

impl ChainedTokenCredential {
    /// Create a `ChainedTokenCredential` with options.
    pub fn new(options: Option<ChainedTokenCredentialOptions>) -> Self {
        Self {
            options: options.unwrap_or_default(),
            sources: Vec::new(),
            successful_credential: RwLock::new(None),
        }
    }

    /// Add a credential source to the chain.
    pub fn add_source(&mut self, source: Arc<dyn TokenCredential>) {
        self.sources.push(source);
    }

    async fn get_token_impl(
        &self,
        scopes: &[&str],
    ) -> azure_core::Result<(Arc<dyn TokenCredential>, AccessToken)> {
        let mut errors = Vec::new();
        for source in &self.sources {
            let token_res = source.get_token(scopes).await;

            match token_res {
                Ok(token) => return Ok((source.clone(), token)),
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

    /// Try to fetch a token using each of the credential sources until one succeeds
    async fn get_token(&self, scopes: &[&str]) -> azure_core::Result<AccessToken> {
        if !self.options.retry_sources {
            if let Some(entry) = self.successful_credential.read().await.as_ref() {
                return entry.get_token(scopes).await;
            }
            let mut lock = self.successful_credential.write().await;
            // if after getting the write lock, we find that another thread has already found a credential, use that.
            if let Some(entry) = lock.as_ref() {
                return entry.get_token(scopes).await;
            }
            let (entry, token) = self.get_token_impl(scopes).await?;
            *lock = Some(entry);
            Ok(token)
        } else {
            // if we are retrying sources, we don't need to cache the successful credential
            Ok(self.get_token_impl(scopes).await?.1)
        }
    }
}

impl From<&[Arc<dyn TokenCredential>]> for ChainedTokenCredential {
    fn from(credential_options: &[Arc<dyn TokenCredential>]) -> Self {
        Self {
            options: ChainedTokenCredentialOptions::default(),
            sources: credential_options.to_vec(),
            successful_credential: RwLock::new(None),
        }
    }
}

#[cfg_attr(target_arch = "wasm32", async_trait::async_trait(?Send))]
#[cfg_attr(not(target_arch = "wasm32"), async_trait::async_trait)]
impl TokenCredential for ChainedTokenCredential {
    async fn get_token(&self, scopes: &[&str]) -> azure_core::Result<AccessToken> {
        self.get_token(scopes).await
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
    use async_lock::Mutex;
    use azure_core::credentials::{AccessToken, TokenCredential};
    use azure_core_test::credentials::MockCredential;

    /// `TokenFailure` is a mock credential that always fails to get a token.
    #[derive(Debug)]
    struct TokenFailure {
        counter: Mutex<u32>,
    }

    impl TokenFailure {
        fn new() -> Self {
            Self {
                counter: Mutex::new(0),
            }
        }

        async fn get_counter(&self) -> u32 {
            let count = self.counter.lock().await;
            *count
        }
    }

    #[cfg_attr(target_arch = "wasm32", async_trait::async_trait(?Send))]
    #[cfg_attr(not(target_arch = "wasm32"), async_trait::async_trait)]
    impl TokenCredential for TokenFailure {
        async fn get_token(&self, _scopes: &[&str]) -> azure_core::Result<AccessToken> {
            let mut count = self.counter.lock().await;
            *count += 1;
            Err(Error::message(ErrorKind::Credential, "failed to get token"))
        }
    }

    #[tokio::test]
    async fn test_basic() -> azure_core::Result<()> {
        let providers: Vec<Arc<dyn TokenCredential>> = vec![Arc::new(MockCredential {})];
        let credentials = ChainedTokenCredential::from(providers.as_slice());
        let scopes = ["https://management.azure.com/.default"];
        let token = credentials.get_token(&scopes).await?;
        assert_eq!(
            token.token.secret(),
            "TEST TOKEN https://management.azure.com/.default"
        );

        Ok(())
    }

    #[tokio::test]
    async fn test_with_retry() -> azure_core::Result<()> {
        let token_failure = Arc::new(TokenFailure::new());
        let mut chained_credential =
            ChainedTokenCredential::new(Some(ChainedTokenCredentialOptions {
                retry_sources: true,
                ..Default::default()
            }));
        chained_credential.add_source(token_failure.clone());
        chained_credential.add_source(Arc::new(MockCredential {}));

        let scopes = ["https://management.azure.com/.default"];
        let token = chained_credential.get_token(&scopes).await?;
        assert_eq!(
            token.token.secret(),
            "TEST TOKEN https://management.azure.com/.default"
        );
        let scopes = ["https://management.azure.com/.default"];
        let token = chained_credential.get_token(&scopes).await?;
        assert_eq!(
            token.token.secret(),
            "TEST TOKEN https://management.azure.com/.default"
        );

        assert_eq!(token_failure.get_counter().await, 2);
        Ok(())
    }

    #[tokio::test]
    async fn test_without_retry() -> azure_core::Result<()> {
        let token_failure = Arc::new(TokenFailure::new());
        let mut chained_credential = ChainedTokenCredential::new(None);
        chained_credential.add_source(token_failure.clone());
        chained_credential.add_source(Arc::new(MockCredential {}));

        let scopes = ["https://management.azure.com/.default"];
        let token = chained_credential.get_token(&scopes).await?;
        assert_eq!(
            token.token.secret(),
            "TEST TOKEN https://management.azure.com/.default"
        );
        let scopes = ["https://management.azure.com/.default"];
        let token = chained_credential.get_token(&scopes).await?;
        assert_eq!(
            token.token.secret(),
            "TEST TOKEN https://management.azure.com/.default"
        );

        assert_eq!(token_failure.get_counter().await, 1);
        Ok(())
    }
}

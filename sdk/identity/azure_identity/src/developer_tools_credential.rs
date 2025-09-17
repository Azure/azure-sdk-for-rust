// Copyright (c) Microsoft Corporation. All rights reserved.
// Licensed under the MIT License.

use crate::{
    AzureCliCredential, AzureCliCredentialOptions, AzureDeveloperCliCredential,
    AzureDeveloperCliCredentialOptions, Executor,
};
use azure_core::{
    credentials::{AccessToken, TokenCredential, TokenRequestOptions},
    error::{Error, ErrorKind},
};
use std::{
    fmt,
    sync::{
        atomic::{AtomicUsize, Ordering},
        Arc,
    },
};

/// Options for constructing a new [`DeveloperToolsCredential`]
#[derive(Clone, Debug, Default)]
pub struct DeveloperToolsCredentialOptions {
    /// An implementation of [`Executor`] to run commands asynchronously.
    pub executor: Option<Arc<dyn Executor>>,
}

/// Authenticates through developer tools such as the Azure CLI.
///
/// It tries the following credential types, in this order, stopping when one provides a token:
///
/// * [`AzureCliCredential`]
/// * [`AzureDeveloperCliCredential`]
///
/// `DeveloperToolsCredential` uses the first credential that provides a token for all subsequent token requests. It never tries the others again.
pub struct DeveloperToolsCredential {
    sources: Vec<Arc<dyn TokenCredential>>,
    // index of the source that first provided a token. usize::MAX indicates no source has provided a token.
    cached_source_index: AtomicUsize,
}

impl DeveloperToolsCredential {
    /// Creates a new instance of `DeveloperToolsCredential`.
    ///
    /// # Arguments
    /// * `options`: Options for configuring the credential. If `None` is provided, default options will be used.
    pub fn new(
        options: Option<DeveloperToolsCredentialOptions>,
    ) -> azure_core::Result<Arc<DeveloperToolsCredential>> {
        let options = options.unwrap_or_default();
        let sources: Vec<Arc<dyn TokenCredential>> = vec![
            AzureCliCredential::new(Some(AzureCliCredentialOptions {
                executor: options.executor.clone(),
                ..Default::default()
            }))?,
            AzureDeveloperCliCredential::new(Some(AzureDeveloperCliCredentialOptions {
                executor: options.executor,
                ..Default::default()
            }))?,
        ];
        Ok(Arc::new(Self {
            sources,
            cached_source_index: AtomicUsize::new(usize::MAX),
        }))
    }

    #[cfg(test)]
    pub(crate) fn new_with_sources(
        sources: Vec<Arc<dyn TokenCredential>>,
    ) -> azure_core::Result<Arc<DeveloperToolsCredential>> {
        Ok(Arc::new(Self {
            sources,
            cached_source_index: AtomicUsize::new(usize::MAX),
        }))
    }
}

impl fmt::Debug for DeveloperToolsCredential {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.write_str("DeveloperToolsCredential")
    }
}

#[async_trait::async_trait]
impl TokenCredential for DeveloperToolsCredential {
    async fn get_token(
        &self,
        scopes: &[&str],
        options: Option<TokenRequestOptions<'_>>,
    ) -> azure_core::Result<AccessToken> {
        let cached_index = self.cached_source_index.load(Ordering::Relaxed);
        if cached_index != usize::MAX {
            if let Some(source) = self.sources.get(cached_index) {
                return source.get_token(scopes, options).await;
            }
            // impossible because the vector's size never changes
            panic!("DeveloperToolsCredential source index {cached_index} is out of bounds")
        }

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
        Err(Error::with_message(ErrorKind::Credential, || {
            format!(
                "Multiple errors were encountered while attempting to authenticate:\n{}",
                format_aggregate_error(&errors)
            )
        }))
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
    use crate::tests::MockExecutor;
    use azure_core::credentials::AccessToken;
    use std::sync::atomic::{AtomicUsize, Ordering};
    use std::time::{Duration, SystemTime};

    #[derive(Debug)]
    struct MockCredential {
        call_count: AtomicUsize,
        id: String,
        succeed: bool,
    }

    impl MockCredential {
        fn new(id: &str, succeed: bool) -> Arc<Self> {
            Arc::new(Self {
                call_count: AtomicUsize::new(0),
                id: id.to_string(),
                succeed,
            })
        }

        fn call_count(&self) -> usize {
            self.call_count.load(Ordering::SeqCst)
        }
    }

    #[async_trait::async_trait]
    impl TokenCredential for MockCredential {
        async fn get_token(
            &self,
            _scopes: &[&str],
            _options: Option<TokenRequestOptions<'_>>,
        ) -> azure_core::Result<AccessToken> {
            self.call_count.fetch_add(1, Ordering::SeqCst);
            if self.succeed {
                Ok(AccessToken {
                    token: self.id.clone().into(),
                    expires_on: (SystemTime::now() + Duration::from_secs(3600)).into(),
                })
            } else {
                Err(Error::with_message(ErrorKind::Credential, || {
                    format!("{} failed", self.id)
                }))
            }
        }
    }

    #[tokio::test]
    async fn cached_credential() {
        let mock1 = MockCredential::new("mock1", false);
        let mock2 = MockCredential::new("mock2", false);
        let mock3 = MockCredential::new("mock3", true);
        let mock4 = MockCredential::new("mock4", true);
        let sources: Vec<Arc<dyn TokenCredential>> =
            vec![mock1.clone(), mock2.clone(), mock3.clone(), mock4.clone()];

        let credential = DeveloperToolsCredential::new_with_sources(sources).unwrap();

        for i in 1..=5 {
            let token = credential
                .get_token(&["scope"], None)
                .await
                .expect("authentication success");
            assert_eq!(token.token.secret(), "mock3");
            assert_eq!(mock1.call_count(), 1);
            assert_eq!(mock2.call_count(), 1);
            assert_eq!(mock3.call_count(), i);
            assert_eq!(mock4.call_count(), 0);
        }
    }

    #[tokio::test]
    async fn error_message() {
        let mock1 = MockCredential::new("mock1", false);
        let mock2 = MockCredential::new("mock2", false);
        let mock3 = MockCredential::new("mock3", false);
        let sources: Vec<Arc<dyn TokenCredential>> =
            vec![mock1.clone(), mock2.clone(), mock3.clone()];

        let credential = DeveloperToolsCredential::new_with_sources(sources).unwrap();

        let error_msg = credential
            .get_token(&["scope"], None)
            .await
            .expect_err("authentication error")
            .to_string();

        assert_eq!(mock1.call_count(), 1);
        assert_eq!(mock2.call_count(), 1);
        assert_eq!(mock3.call_count(), 1);
        assert!(error_msg.contains("mock1 failed"));
        assert!(error_msg.contains("mock2 failed"));
        assert!(error_msg.contains("mock3 failed"));
    }

    #[tokio::test]
    async fn executor() {
        let err = std::io::Error::other("something went wrong");
        let executor = MockExecutor::with_error(err);
        let options = DeveloperToolsCredentialOptions {
            executor: Some(executor.clone()),
        };
        let err = DeveloperToolsCredential::new(Some(options))
            .expect("valid credential")
            .get_token(&["scope"], None)
            .await
            .expect_err("expected error");
        assert!(err.to_string().contains("something went wrong"));
        assert_eq!(
            2,
            executor.call_count(),
            "Executor should have been called once for each inner credential"
        );
    }
}

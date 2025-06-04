// Copyright (c) Microsoft Corporation. All rights reserved.
// Licensed under the MIT License.

#[cfg(not(target_arch = "wasm32"))]
use crate::{AzureCliCredential, AzureDeveloperCliCredential};
use crate::{TokenCache, TokenCredentialOptions};
#[cfg(not(target_arch = "wasm32"))]
use azure_core::error::ResultExt;
use azure_core::{
    credentials::{AccessToken, TokenCredential, TokenRequestOptions},
    error::{Error, ErrorKind},
};
use std::sync::Arc;

/// Provides a mechanism of selectively disabling credentials used for a `DefaultAzureCredential` instance
pub struct DefaultAzureCredentialBuilder {
    options: TokenCredentialOptions,
    #[cfg(not(target_arch = "wasm32"))]
    include_azure_cli_credential: bool,
    #[cfg(not(target_arch = "wasm32"))]
    include_azure_developer_cli_credential: bool,
}

#[cfg_attr(target_arch = "wasm32", allow(clippy::derivable_impls))]
impl Default for DefaultAzureCredentialBuilder {
    fn default() -> Self {
        Self {
            options: TokenCredentialOptions::default(),
            #[cfg(not(target_arch = "wasm32"))]
            include_azure_cli_credential: true,
            #[cfg(not(target_arch = "wasm32"))]
            include_azure_developer_cli_credential: true,
        }
    }
}

impl DefaultAzureCredentialBuilder {
    /// Create a new `DefaultAzureCredentialBuilder`
    pub fn new() -> Self {
        Self::default()
    }

    pub fn with_options(&mut self, options: impl Into<TokenCredentialOptions>) -> &mut Self {
        self.options = options.into();
        self
    }

    /// Exclude authenticating using the Azure CLI (az).
    #[cfg(not(target_arch = "wasm32"))]
    pub fn exclude_azure_cli_credential(&mut self) -> &mut Self {
        self.include_azure_cli_credential = false;
        self
    }

    /// Exclude authenticating using the Azure Developer CLI (azd).
    #[cfg(not(target_arch = "wasm32"))]
    pub fn exclude_azure_developer_cli_credential(&mut self) -> &mut Self {
        self.include_azure_developer_cli_credential = false;
        self
    }

    /// Get a list of the credential types to include.
    fn included(&self) -> Vec<DefaultAzureCredentialType> {
        #[cfg_attr(target_arch = "wasm32", allow(unused_mut))]
        let mut sources = Vec::new();
        #[cfg(not(target_arch = "wasm32"))]
        if self.include_azure_cli_credential {
            sources.push(DefaultAzureCredentialType::AzureCli);
        }
        #[cfg(not(target_arch = "wasm32"))]
        if self.include_azure_developer_cli_credential {
            sources.push(DefaultAzureCredentialType::AzureDeveloperCli);
        }
        sources
    }

    /// Creates a list of `TokenCredential` instances from the included credential types.
    /// The credentials created successfully are used as sources for getting a token.
    fn create_sources(
        &self,
        included: &Vec<DefaultAzureCredentialType>,
    ) -> azure_core::Result<Vec<DefaultAzureCredentialKind>> {
        #[cfg_attr(target_arch = "wasm32", allow(unused_mut))]
        let mut sources = Vec::<DefaultAzureCredentialKind>::with_capacity(included.len());
        let errors = Vec::new();

        #[cfg_attr(target_arch = "wasm32", allow(clippy::never_loop))]
        for source in included {
            match source {
                #[cfg(not(target_arch = "wasm32"))]
                DefaultAzureCredentialType::AzureCli => {
                    if let Ok(credential) =
                        AzureCliCredential::new(Some(self.options.clone().into()))
                    {
                        sources.push(DefaultAzureCredentialKind::AzureCli(credential));
                    }
                }
                #[cfg(not(target_arch = "wasm32"))]
                DefaultAzureCredentialType::AzureDeveloperCli => {
                    if let Ok(credential) =
                        AzureDeveloperCliCredential::new(Some(self.options.clone().into()))
                    {
                        sources.push(DefaultAzureCredentialKind::AzureDeveloperCli(credential));
                    }
                }
                #[cfg(target_arch = "wasm32")]
                _ => {
                    return Err(Error::with_message(ErrorKind::Credential, || {
                        "No credential providers available"
                    }));
                }
            }
        }
        if sources.is_empty() {
            return Err(Error::with_message(ErrorKind::Credential, || {
                format!(
                    "No credential sources were available to be used for authentication.\n{}",
                    format_aggregate_error(&errors)
                )
            }));
        }
        Ok(sources)
    }

    /// Create a `DefaultAzureCredential` from this builder.
    pub fn build(&self) -> azure_core::Result<Arc<DefaultAzureCredential>> {
        let included = self.included();
        let sources = self.create_sources(&included)?;
        DefaultAzureCredential::with_sources(sources)
    }
}

/// Types that may be enabled for use by `DefaultAzureCredential`.
#[derive(Debug, PartialEq)]
enum DefaultAzureCredentialType {
    #[cfg(not(target_arch = "wasm32"))]
    AzureCli,
    #[cfg(not(target_arch = "wasm32"))]
    AzureDeveloperCli,
}

/// Types of `TokenCredential` supported by `DefaultAzureCredential`
#[derive(Debug)]
pub(crate) enum DefaultAzureCredentialKind {
    #[cfg(not(target_arch = "wasm32"))]
    /// `TokenCredential` from Azure CLI (az).
    AzureCli(Arc<AzureCliCredential>),
    #[cfg(not(target_arch = "wasm32"))]
    /// `TokenCredential` from Azure Developer CLI (azd).
    AzureDeveloperCli(Arc<AzureDeveloperCliCredential>),
}

#[cfg_attr(target_arch = "wasm32", async_trait::async_trait(?Send), allow(unused_variables))]
#[cfg_attr(not(target_arch = "wasm32"), async_trait::async_trait)]
impl TokenCredential for DefaultAzureCredentialKind {
    async fn get_token(
        &self,
        scopes: &[&str],
        _: Option<TokenRequestOptions>,
    ) -> azure_core::Result<AccessToken> {
        match self {
            #[cfg(not(target_arch = "wasm32"))]
            DefaultAzureCredentialKind::AzureCli(credential) => {
                credential.get_token(scopes, None).await.context(
                    ErrorKind::Credential,
                    "error getting token credential from Azure CLI",
                )
            }
            #[cfg(not(target_arch = "wasm32"))]
            DefaultAzureCredentialKind::AzureDeveloperCli(credential) => {
                credential.get_token(scopes, None).await.context(
                    ErrorKind::Credential,
                    "error getting token credential from Azure Developer CLI",
                )
            }
            #[cfg(target_arch = "wasm32")]
            _ => {
                return Err(Error::with_message(ErrorKind::Credential, || {
                    "No credential providers available"
                }));
            }
        }
    }
}

/// Provides a default `TokenCredential` authentication flow for applications that will be deployed to Azure.
///
/// The following credential types if enabled will be tried, in order:
///
/// * `ManagedIdentityCredential`
/// * `AzureCliCredential`
///
/// Consult the documentation of these credential types for more information on how they attempt authentication.
#[derive(Debug)]
pub struct DefaultAzureCredential {
    sources: Vec<DefaultAzureCredentialKind>,
    cache: TokenCache,
}

impl DefaultAzureCredential {
    /// Create a [`DefaultAzureCredentialBuilder`] to create a `DefaultAzureCredential` with options.
    pub fn builder() -> DefaultAzureCredentialBuilder {
        DefaultAzureCredentialBuilder::new()
    }

    /// Creates a `DefaultAzureCredential` with default options.
    pub fn new() -> azure_core::Result<Arc<DefaultAzureCredential>> {
        Self::with_options(TokenCredentialOptions::default())
    }

    /// Creates a `DefaultAzureCredential` with options.
    pub fn with_options(
        options: TokenCredentialOptions,
    ) -> azure_core::Result<Arc<DefaultAzureCredential>> {
        DefaultAzureCredentialBuilder::default()
            .with_options(options)
            .build()
    }

    /// Creates a `DefaultAzureCredential` with specified sources.
    fn with_sources(sources: Vec<DefaultAzureCredentialKind>) -> azure_core::Result<Arc<Self>> {
        Ok(Arc::new(DefaultAzureCredential {
            sources,
            cache: TokenCache::new(),
        }))
    }

    /// Try to fetch a token using each of the credential sources until one succeeds
    async fn get_token(
        &self,
        scopes: &[&str],
        options: Option<TokenRequestOptions>,
    ) -> azure_core::Result<AccessToken> {
        let mut errors = Vec::new();
        for source in &self.sources {
            let token_res = source.get_token(scopes, options.clone()).await;

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
impl TokenCredential for DefaultAzureCredential {
    async fn get_token(
        &self,
        scopes: &[&str],
        options: Option<TokenRequestOptions>,
    ) -> azure_core::Result<AccessToken> {
        self.cache
            .get_token(scopes, options, |s, o| self.get_token(s, o))
            .await
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

    #[test]
    fn test_builder_included_credential_flags() {
        let builder = DefaultAzureCredentialBuilder::new();
        #[cfg(not(target_arch = "wasm32"))]
        assert!(builder.include_azure_cli_credential);
        #[cfg(not(target_arch = "wasm32"))]
        assert!(builder.include_azure_developer_cli_credential);

        #[cfg(not(target_arch = "wasm32"))]
        {
            let mut builder = DefaultAzureCredentialBuilder::new();
            builder.exclude_azure_cli_credential();
            assert!(!builder.include_azure_cli_credential);
        }

        #[cfg(not(target_arch = "wasm32"))]
        {
            let mut builder = DefaultAzureCredentialBuilder::new();
            builder.exclude_azure_developer_cli_credential();
            assert!(!builder.include_azure_developer_cli_credential);
        }

        let builder = DefaultAzureCredentialBuilder::new();
        #[cfg(not(target_arch = "wasm32"))]
        assert!(builder.include_azure_cli_credential);
        #[cfg(not(target_arch = "wasm32"))]
        assert!(builder.include_azure_developer_cli_credential);
    }

    #[test]
    /// test default included credential types
    fn test_default_included_credential_types() {
        let builder = DefaultAzureCredentialBuilder::new();
        assert_eq!(
            builder.included(),
            vec![
                DefaultAzureCredentialType::AzureCli,
                DefaultAzureCredentialType::AzureDeveloperCli
            ]
        );
    }

    /// test excluding azure cli credential
    #[test]
    fn test_exclude_azure_cli_credential() {
        let mut builder = DefaultAzureCredentialBuilder::new();
        builder.exclude_azure_cli_credential();
        builder.exclude_azure_developer_cli_credential();
        assert!(builder.included().is_empty());
    }
}

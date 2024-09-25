// Copyright (c) Microsoft Corporation. All rights reserved.
// Licensed under the MIT License.

#[cfg(not(target_arch = "wasm32"))]
use crate::AzureCliCredential;
use crate::{
    credentials::cache::TokenCache, timeout::TimeoutExt, AppServiceManagedIdentityCredential,
    ImdsId, TokenCredentialOptions, VirtualMachineManagedIdentityCredential,
};
use azure_core::{
    credentials::{AccessToken, TokenCredential},
    error::{Error, ErrorKind, ResultExt},
};
use std::{sync::Arc, time::Duration};

/// Provides a mechanism of selectively disabling credentials used for a `DefaultAzureCredential` instance
pub struct DefaultAzureCredentialBuilder {
    options: TokenCredentialOptions,
    include_app_service_managed_identity_credential: bool,
    include_virtual_machine_managed_identity_credential: bool,
    #[cfg(not(target_arch = "wasm32"))]
    include_azure_cli_credential: bool,
}

impl Default for DefaultAzureCredentialBuilder {
    fn default() -> Self {
        Self {
            options: TokenCredentialOptions::default(),
            include_app_service_managed_identity_credential: true,
            include_virtual_machine_managed_identity_credential: true,
            #[cfg(not(target_arch = "wasm32"))]
            include_azure_cli_credential: true,
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

    /// Exclude using any managed identity credential
    pub fn exclude_managed_identity_credential(&mut self) -> &mut Self {
        self.include_app_service_managed_identity_credential = false;
        self.include_virtual_machine_managed_identity_credential = false;
        self
    }

    /// Exclude using virtual machine managed identity credential
    pub fn exclude_virtual_machine_managed_identity_credential(&mut self) -> &mut Self {
        self.include_virtual_machine_managed_identity_credential = false;
        self
    }

    /// Include using virtual machine managed identity credential
    pub fn include_virtual_machine_managed_identity_credential(&mut self) -> &mut Self {
        self.include_virtual_machine_managed_identity_credential = true;
        self
    }

    /// Include using app service managed identity credential
    pub fn include_app_service_managed_identity_credential(&mut self) -> &mut Self {
        self.include_app_service_managed_identity_credential = true;
        self
    }

    /// Exclude using credential from the cli
    #[cfg(not(target_arch = "wasm32"))]
    pub fn exclude_azure_cli_credential(&mut self) -> &mut Self {
        self.include_azure_cli_credential = false;
        self
    }

    /// Get a list of the credential types to include.
    fn included(&self) -> Vec<DefaultAzureCredentialType> {
        let mut sources = Vec::new();
        if self.include_app_service_managed_identity_credential {
            sources.push(DefaultAzureCredentialType::AppService);
        }
        if self.include_virtual_machine_managed_identity_credential {
            sources.push(DefaultAzureCredentialType::VirtualMachine);
        }
        #[cfg(not(target_arch = "wasm32"))]
        if self.include_azure_cli_credential {
            sources.push(DefaultAzureCredentialType::AzureCli);
        }
        sources
    }

    /// Creates a list of `TokenCredential` instances from the included credential types.
    /// The credentials created successfully are used as sources for getting a token.
    fn create_sources(
        &self,
        included: &Vec<DefaultAzureCredentialType>,
    ) -> azure_core::Result<Vec<DefaultAzureCredentialKind>> {
        let mut sources = Vec::<DefaultAzureCredentialKind>::with_capacity(included.len());
        let mut errors = Vec::new();
        for source in included {
            match source {
                DefaultAzureCredentialType::AppService => {
                    match AppServiceManagedIdentityCredential::create(self.options.clone()) {
                        Ok(credential) => {
                            sources.push(DefaultAzureCredentialKind::AppService(credential))
                        }
                        Err(error) => errors.push(error),
                    }
                }
                DefaultAzureCredentialType::VirtualMachine => {
                    sources.push(DefaultAzureCredentialKind::VirtualMachine(
                        VirtualMachineManagedIdentityCredential::new(
                            ImdsId::SystemAssigned,
                            self.options.clone(),
                        ),
                    ));
                }
                #[cfg(not(target_arch = "wasm32"))]
                DefaultAzureCredentialType::AzureCli => {
                    if let Ok(credential) = AzureCliCredential::create() {
                        sources.push(DefaultAzureCredentialKind::AzureCli(credential));
                    }
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
    pub fn build(&self) -> azure_core::Result<DefaultAzureCredential> {
        let included = self.included();
        let sources = self.create_sources(&included)?;
        Ok(DefaultAzureCredential::with_sources(sources))
    }
}

/// Types that may be enabled for use by `DefaultAzureCredential`.
#[derive(Debug, PartialEq)]
enum DefaultAzureCredentialType {
    AppService,
    VirtualMachine,
    #[cfg(not(target_arch = "wasm32"))]
    AzureCli,
}

/// Types of `TokenCredential` supported by `DefaultAzureCredential`
#[derive(Debug)]
pub(crate) enum DefaultAzureCredentialKind {
    /// `TokenCredential` from managed identity that has been assigned to an App Service.
    AppService(AppServiceManagedIdentityCredential),
    /// `TokenCredential` from managed identity that has been assigned to a virtual machine.
    VirtualMachine(VirtualMachineManagedIdentityCredential),
    #[cfg(not(target_arch = "wasm32"))]
    /// `TokenCredential` from Azure CLI.
    AzureCli(AzureCliCredential),
}

#[cfg_attr(target_arch = "wasm32", async_trait::async_trait(?Send))]
#[cfg_attr(not(target_arch = "wasm32"), async_trait::async_trait)]
impl TokenCredential for DefaultAzureCredentialKind {
    async fn get_token(&self, scopes: &[&str]) -> azure_core::Result<AccessToken> {
        match self {
            DefaultAzureCredentialKind::AppService(credential) => {
                credential.get_token(scopes).await.context(
                    ErrorKind::Credential,
                    "error getting managed identity credential for App Service",
                )
            }
            DefaultAzureCredentialKind::VirtualMachine(credential) => {
                // IMDS timeout is only limited to 1 second when used in DefaultAzureCredential
                credential
                    .get_token(scopes)
                    .timeout(Duration::from_secs(1))
                    .await
                    .context(
                        ErrorKind::Credential,
                        "getting virtual machine managed identity credential timed out",
                    )?
                    .context(
                        ErrorKind::Credential,
                        "error getting virtual machine managed identity credential",
                    )
            }
            #[cfg(not(target_arch = "wasm32"))]
            DefaultAzureCredentialKind::AzureCli(credential) => {
                credential.get_token(scopes).await.context(
                    ErrorKind::Credential,
                    "error getting token credential from Azure CLI",
                )
            }
        }
    }

    /// Clear the credential's cache.
    async fn clear_cache(&self) -> azure_core::Result<()> {
        match self {
            DefaultAzureCredentialKind::AppService(credential) => credential.clear_cache().await,
            DefaultAzureCredentialKind::VirtualMachine(credential) => {
                credential.clear_cache().await
            }
            #[cfg(not(target_arch = "wasm32"))]
            DefaultAzureCredentialKind::AzureCli(credential) => credential.clear_cache().await,
        }
    }
}

/// Provides a default `TokenCredential` authentication flow for applications that will be deployed to Azure.
///
/// The following credential types if enabled will be tried, in order:
/// - `ManagedIdentityCredential`
/// - `AzureCliCredential`
///
/// Consult the documentation of these credential types for more information on how they attempt authentication.
#[derive(Debug)]
pub struct DefaultAzureCredential {
    sources: Vec<DefaultAzureCredentialKind>,
    cache: TokenCache,
}

impl DefaultAzureCredential {
    pub fn create(options: TokenCredentialOptions) -> azure_core::Result<DefaultAzureCredential> {
        DefaultAzureCredentialBuilder::default()
            .with_options(options)
            .build()
    }

    /// Creates a `DefaultAzureCredential` with specified sources.
    fn with_sources(sources: Vec<DefaultAzureCredentialKind>) -> Self {
        DefaultAzureCredential {
            sources,
            cache: TokenCache::new(),
        }
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

/// Creates a new `DefaultAzureCredential` with the default options.
pub fn create_default_credential() -> azure_core::Result<Arc<dyn TokenCredential>> {
    DefaultAzureCredentialBuilder::default()
        .build()
        .map(|cred| Arc::new(cred) as Arc<dyn TokenCredential>)
}

#[cfg_attr(target_arch = "wasm32", async_trait::async_trait(?Send))]
#[cfg_attr(not(target_arch = "wasm32"), async_trait::async_trait)]
impl TokenCredential for DefaultAzureCredential {
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

    #[test]
    fn test_builder_included_credential_flags() {
        let builder = DefaultAzureCredentialBuilder::new();
        #[cfg(not(target_arch = "wasm32"))]
        assert!(builder.include_azure_cli_credential);
        assert!(builder.include_app_service_managed_identity_credential);
        assert!(builder.include_virtual_machine_managed_identity_credential);

        #[cfg(not(target_arch = "wasm32"))]
        {
            let mut builder = DefaultAzureCredentialBuilder::new();
            builder.exclude_azure_cli_credential();
            assert!(!builder.include_azure_cli_credential);
            assert!(builder.include_app_service_managed_identity_credential);
            assert!(builder.include_virtual_machine_managed_identity_credential);
        }

        let mut builder = DefaultAzureCredentialBuilder::new();
        builder.exclude_managed_identity_credential();
        #[cfg(not(target_arch = "wasm32"))]
        assert!(builder.include_azure_cli_credential);
        assert!(!builder.include_app_service_managed_identity_credential);
        assert!(!builder.include_virtual_machine_managed_identity_credential);
    }

    #[test]
    /// test default included credential types
    fn test_default_included_credential_types() {
        let builder = DefaultAzureCredentialBuilder::new();
        assert_eq!(
            builder.included(),
            vec![
                DefaultAzureCredentialType::AppService,
                DefaultAzureCredentialType::VirtualMachine,
                DefaultAzureCredentialType::AzureCli,
            ]
        );
    }

    /// test excluding virtual machine managed identity credential
    #[test]
    fn test_exclude_virtual_machine_managed_identity_credential() {
        let mut builder = DefaultAzureCredentialBuilder::new();
        builder.exclude_virtual_machine_managed_identity_credential();
        assert_eq!(
            builder.included(),
            vec![
                DefaultAzureCredentialType::AppService,
                DefaultAzureCredentialType::AzureCli,
            ]
        );
    }

    /// test excluding azure cli credential
    #[test]
    fn test_exclude_azure_cli_credential() {
        let mut builder = DefaultAzureCredentialBuilder::new();
        builder.exclude_azure_cli_credential();
        assert_eq!(
            builder.included(),
            vec![
                DefaultAzureCredentialType::AppService,
                DefaultAzureCredentialType::VirtualMachine,
            ]
        );
    }

    /// test excluding managed identity credential
    #[test]
    fn test_exclude_managed_identity_credential() {
        let mut builder = DefaultAzureCredentialBuilder::new();
        builder.exclude_managed_identity_credential();
        assert_eq!(
            builder.included(),
            vec![DefaultAzureCredentialType::AzureCli,]
        );
    }
}

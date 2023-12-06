use crate::{
    timeout::TimeoutExt,
    token_credentials::cache::TokenCache,
    {AzureCliCredential, ImdsManagedIdentityCredential},
};
use azure_core::{
    auth::{AccessToken, TokenCredential},
    error::{Error, ErrorKind, ResultExt},
};
use std::time::Duration;

#[derive(Debug)]
/// Provides a mechanism of selectively disabling credentials used for a `DefaultAzureCredential` instance
pub struct DefaultAzureCredentialBuilder {
    include_environment_credential: bool,
    include_managed_identity_credential: bool,
    include_azure_cli_credential: bool,
}

impl Default for DefaultAzureCredentialBuilder {
    fn default() -> Self {
        Self {
            include_environment_credential: true,
            include_managed_identity_credential: true,
            include_azure_cli_credential: true,
        }
    }
}

impl DefaultAzureCredentialBuilder {
    /// Create a new `DefaultAzureCredentialBuilder`
    pub fn new() -> Self {
        Self::default()
    }

    /// Exclude using credentials from the environment
    pub fn exclude_environment_credential(&mut self) -> &mut Self {
        self.include_environment_credential = false;
        self
    }

    /// Exclude using managed identity credentials
    pub fn exclude_managed_identity_credential(&mut self) -> &mut Self {
        self.include_managed_identity_credential = false;
        self
    }

    /// Exclude using credentials from the cli
    pub fn exclude_azure_cli_credential(&mut self) -> &mut Self {
        self.include_azure_cli_credential = false;
        self
    }

    /// Create a `DefaultAzureCredential` from this builder.
    pub fn build(&self) -> DefaultAzureCredential {
        let source_count = usize::from(self.include_azure_cli_credential)
            + usize::from(self.include_azure_cli_credential)
            + usize::from(self.include_managed_identity_credential);
        let mut sources = Vec::<DefaultAzureCredentialEnum>::with_capacity(source_count);
        if self.include_environment_credential {
            sources.push(DefaultAzureCredentialEnum::Environment(
                super::EnvironmentCredential::default(),
            ));
        }
        if self.include_managed_identity_credential {
            sources.push(DefaultAzureCredentialEnum::ManagedIdentity(
                ImdsManagedIdentityCredential::default(),
            ));
        }
        if self.include_azure_cli_credential {
            sources.push(DefaultAzureCredentialEnum::AzureCli(
                AzureCliCredential::new(),
            ));
        }
        DefaultAzureCredential::with_sources(sources)
    }
}

/// Types of `TokenCredential` supported by `DefaultAzureCredential`
#[derive(Debug)]
pub enum DefaultAzureCredentialEnum {
    /// `TokenCredential` from environment variable.
    Environment(super::EnvironmentCredential),
    /// `TokenCredential` from managed identity that has been assigned in this deployment environment.
    ManagedIdentity(ImdsManagedIdentityCredential),
    /// `TokenCredential` from Azure CLI.
    AzureCli(AzureCliCredential),
}

#[cfg_attr(target_arch = "wasm32", async_trait::async_trait(?Send))]
#[cfg_attr(not(target_arch = "wasm32"), async_trait::async_trait)]
impl TokenCredential for DefaultAzureCredentialEnum {
    async fn get_token(&self, scopes: &[&str]) -> azure_core::Result<AccessToken> {
        match self {
            DefaultAzureCredentialEnum::Environment(credential) => {
                credential.get_token(scopes).await.context(
                    ErrorKind::Credential,
                    "error getting environment credential",
                )
            }
            DefaultAzureCredentialEnum::ManagedIdentity(credential) => {
                // IMSD timeout is only limited to 1 second when used in DefaultAzureCredential
                credential
                    .get_token(scopes)
                    .timeout(Duration::from_secs(1))
                    .await
                    .context(
                        ErrorKind::Credential,
                        "getting managed identity credential timed out",
                    )?
                    .context(
                        ErrorKind::Credential,
                        "error getting managed identity credential",
                    )
            }
            DefaultAzureCredentialEnum::AzureCli(credential) => {
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
            DefaultAzureCredentialEnum::Environment(credential) => credential.clear_cache().await,
            DefaultAzureCredentialEnum::ManagedIdentity(credential) => {
                credential.clear_cache().await
            }
            DefaultAzureCredentialEnum::AzureCli(credential) => credential.clear_cache().await,
        }
    }
}

/// Provides a default `TokenCredential` authentication flow for applications that will be deployed to Azure.
///
/// The following credential types if enabled will be tried, in order:
/// - `EnvironmentCredential`
/// - `ManagedIdentityCredential`
/// - `AzureCliCredential`
/// Consult the documentation of these credential types for more information on how they attempt authentication.
#[derive(Debug)]
pub struct DefaultAzureCredential {
    sources: Vec<DefaultAzureCredentialEnum>,
    cache: TokenCache,
}

impl DefaultAzureCredential {
    /// Creates a `DefaultAzureCredential` with specified sources.
    ///
    /// These sources will be tried in the order provided in the `TokenCredential` authentication flow.
    pub fn with_sources(sources: Vec<DefaultAzureCredentialEnum>) -> Self {
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

impl Default for DefaultAzureCredential {
    fn default() -> Self {
        DefaultAzureCredentialBuilder::new().build()
    }
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
    errors
        .iter()
        .map(ToString::to_string)
        .collect::<Vec<String>>()
        .join("\n")
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::matches;

    #[test]
    fn test_builder_included_credential_flags() {
        let builder = DefaultAzureCredentialBuilder::new();
        assert!(builder.include_azure_cli_credential);
        assert!(builder.include_environment_credential);
        assert!(builder.include_managed_identity_credential);

        let mut builder = DefaultAzureCredentialBuilder::new();
        builder.exclude_azure_cli_credential();
        assert!(!builder.include_azure_cli_credential);
        assert!(builder.include_environment_credential);
        assert!(builder.include_managed_identity_credential);

        let mut builder = DefaultAzureCredentialBuilder::new();
        builder.exclude_environment_credential();
        assert!(builder.include_azure_cli_credential);
        assert!(!builder.include_environment_credential);
        assert!(builder.include_managed_identity_credential);

        let mut builder = DefaultAzureCredentialBuilder::new();
        builder.exclude_managed_identity_credential();
        assert!(builder.include_azure_cli_credential);
        assert!(builder.include_environment_credential);
        assert!(!builder.include_managed_identity_credential);
    }

    macro_rules! contains_credential {
        ($creds:expr, $p:pat) => {
            $creds.sources.iter().any(|x| matches!(x, $p))
        };
    }

    #[test]
    fn test_credential_sources() {
        let mut builder = DefaultAzureCredentialBuilder::new();

        // test with all sources

        let credential = builder.build();
        assert_eq!(credential.sources.len(), 3);

        assert!(contains_credential!(
            credential,
            DefaultAzureCredentialEnum::Environment(_)
        ));
        assert!(contains_credential!(
            credential,
            DefaultAzureCredentialEnum::AzureCli(_)
        ));
        assert!(contains_credential!(
            credential,
            DefaultAzureCredentialEnum::ManagedIdentity(_)
        ));

        // remove environment source

        builder.exclude_environment_credential();
        let credential = builder.build();

        assert_eq!(credential.sources.len(), 2);

        assert!(!contains_credential!(
            credential,
            DefaultAzureCredentialEnum::Environment(_)
        ));
        assert!(contains_credential!(
            credential,
            DefaultAzureCredentialEnum::AzureCli(_)
        ));
        assert!(contains_credential!(
            credential,
            DefaultAzureCredentialEnum::ManagedIdentity(_)
        ));

        // remove cli source

        builder.exclude_azure_cli_credential();
        let credential = builder.build();

        assert_eq!(credential.sources.len(), 1);

        assert!(!contains_credential!(
            credential,
            DefaultAzureCredentialEnum::Environment(_)
        ));
        assert!(!contains_credential!(
            credential,
            DefaultAzureCredentialEnum::AzureCli(_)
        ));
        assert!(contains_credential!(
            credential,
            DefaultAzureCredentialEnum::ManagedIdentity(_)
        ));

        // remove managed identity source

        builder.exclude_managed_identity_credential();
        let credential = builder.build();

        assert_eq!(credential.sources.len(), 0);
    }
}

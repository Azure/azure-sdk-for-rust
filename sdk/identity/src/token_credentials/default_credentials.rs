use super::{
    AzureCliCredential, EnvironmentCredential, ImdsManagedIdentityCredential, TokenCredential,
};
use azure_core::auth::TokenResponse;

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
        let source_count = self.include_azure_cli_credential as usize
            + self.include_azure_cli_credential as usize
            + self.include_managed_identity_credential as usize;
        let mut sources = Vec::<DefaultAzureCredentialEnum>::with_capacity(source_count);
        if self.include_environment_credential {
            sources.push(DefaultAzureCredentialEnum::Environment(
                EnvironmentCredential::default(),
            ));
        }
        if self.include_managed_identity_credential {
            sources.push(DefaultAzureCredentialEnum::ManagedIdentity(
                ImdsManagedIdentityCredential::default(),
            ))
        }
        if self.include_azure_cli_credential {
            sources.push(DefaultAzureCredentialEnum::AzureCli(AzureCliCredential {}));
        }
        DefaultAzureCredential::with_sources(sources)
    }
}

#[allow(missing_docs)]
#[non_exhaustive]
#[derive(Debug, thiserror::Error)]
pub enum DefaultAzureCredentialError {
    #[error("Error getting token credential from Azure CLI")]
    AzureCliCredential(#[source] super::AzureCliCredentialError),
    #[error("Error getting environment credential")]
    EnvironmentCredential(#[source] super::EnvironmentCredentialError),
    #[error("Error getting managed identity credential")]
    ManagedIdentityCredential(#[source] super::ManagedIdentityCredentialError),
    #[error(
        "Multiple errors were encountered while attempting to authenticate:\n{}",
        format_aggregate_error(.0)
    )]
    CredentialUnavailable(Vec<DefaultAzureCredentialError>),
}

/// Types of TokenCredential supported by DefaultAzureCredential
pub enum DefaultAzureCredentialEnum {
    /// `TokenCredential` from environment variable.
    Environment(EnvironmentCredential),
    /// `TokenCredential` from managed identity that has been assigned in this deployment environment.
    ManagedIdentity(ImdsManagedIdentityCredential),
    /// `TokenCredential` from Azure CLI.
    AzureCli(AzureCliCredential),
}

#[async_trait::async_trait]
impl TokenCredential for DefaultAzureCredentialEnum {
    type Error = DefaultAzureCredentialError;

    async fn get_token(&self, resource: &str) -> Result<TokenResponse, Self::Error> {
        match self {
            DefaultAzureCredentialEnum::Environment(credential) => credential
                .get_token(resource)
                .await
                .map_err(DefaultAzureCredentialError::EnvironmentCredential),
            DefaultAzureCredentialEnum::ManagedIdentity(credential) => credential
                .get_token(resource)
                .await
                .map_err(DefaultAzureCredentialError::ManagedIdentityCredential),
            DefaultAzureCredentialEnum::AzureCli(credential) => credential
                .get_token(resource)
                .await
                .map_err(DefaultAzureCredentialError::AzureCliCredential),
        }
    }
}

/// Provides a default `TokenCredential` authentication flow for applications that will be deployed to Azure.
///
/// The following credential types if enabled will be tried, in order:
/// - EnvironmentCredential
/// - ManagedIdentityCredential
/// - AzureCliCredential
/// Consult the documentation of these credential types for more information on how they attempt authentication.
pub struct DefaultAzureCredential {
    sources: Vec<DefaultAzureCredentialEnum>,
}

impl DefaultAzureCredential {
    /// Creates a `DefaultAzureCredential` with specified sources.
    ///
    /// These sources will be tried in the order provided in the `TokenCredential` authentication flow.
    pub fn with_sources(sources: Vec<DefaultAzureCredentialEnum>) -> Self {
        DefaultAzureCredential { sources }
    }
}

impl Default for DefaultAzureCredential {
    fn default() -> Self {
        DefaultAzureCredential {
            sources: vec![
                DefaultAzureCredentialEnum::Environment(EnvironmentCredential::default()),
                DefaultAzureCredentialEnum::ManagedIdentity(
                    ImdsManagedIdentityCredential::default(),
                ),
                DefaultAzureCredentialEnum::AzureCli(AzureCliCredential {}),
            ],
        }
    }
}

#[async_trait::async_trait]
impl TokenCredential for DefaultAzureCredential {
    type Error = DefaultAzureCredentialError;
    /// Try to fetch a token using each of the credential sources until one succeeds
    async fn get_token(&self, resource: &str) -> Result<TokenResponse, Self::Error> {
        let mut errors = Vec::new();
        for source in &self.sources {
            let token_res = source.get_token(resource).await;

            match token_res {
                Ok(token) => return Ok(token),
                Err(error) => errors.push(error),
            }
        }
        Err(DefaultAzureCredentialError::CredentialUnavailable(errors))
    }
}

#[async_trait::async_trait]
impl azure_core::auth::TokenCredential for DefaultAzureCredential {
    async fn get_token(
        &self,
        resource: &str,
    ) -> Result<azure_core::auth::TokenResponse, azure_core::Error> {
        TokenCredential::get_token(self, resource)
            .await
            .map_err(|error| azure_core::Error::GetToken(Box::new(error)))
    }
}

fn format_aggregate_error(errors: &[DefaultAzureCredentialError]) -> String {
    errors
        .iter()
        .map(|error| error.to_string())
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

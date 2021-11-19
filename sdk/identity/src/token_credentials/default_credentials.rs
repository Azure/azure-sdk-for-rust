use super::{
    AzureCliCredential, EnvironmentCredential, ImdsManagedIdentityCredential, TokenCredential,
};
use azure_core::TokenResponse;

#[derive(Debug)]
/// Provides a mechanism of selectively disabling credentials used for a `DefaultAzureCredential` instance
pub struct DefaultAzureCredentialBuilder {
    include_environment_credential: bool,
    include_managed_identity_credential: bool,
    include_cli_credential: bool,
}

impl Default for DefaultAzureCredentialBuilder {
    fn default() -> Self {
        Self {
            include_environment_credential: true,
            include_managed_identity_credential: true,
            include_cli_credential: true,
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

    /// Exclude using credentials from the cli
    pub fn exclude_cli_credential(&mut self) -> &mut Self {
        self.include_cli_credential = false;
        self
    }

    /// Exclude using managed identity credentials
    pub fn exclude_managed_identity_credential(&mut self) -> &mut Self {
        self.include_managed_identity_credential = false;
        self
    }

    pub fn build(&self) -> DefaultAzureCredential {
        let source_count = self.include_cli_credential as usize
            + self.include_cli_credential as usize
            + self.include_managed_identity_credential as usize;
        let mut sources = Vec::<DefaultAzureCredentialEnum>::with_capacity(source_count);
        if self.include_environment_credential {
            sources.push(DefaultAzureCredentialEnum::Environment(
                EnvironmentCredential::default(),
            ));
        }
        if self.include_managed_identity_credential {
            sources.push(DefaultAzureCredentialEnum::ManagedIdentity(
                ImdsManagedIdentityCredential {},
            ))
        }
        if self.include_cli_credential {
            sources.push(DefaultAzureCredentialEnum::AzureCli(AzureCliCredential {}));
        }
        DefaultAzureCredential::with_sources(sources)
    }
}

#[non_exhaustive]
#[derive(Debug, thiserror::Error)]
pub enum DefaultAzureCredentialError {
    #[error("Error getting token credential from Azure CLI: {0}")]
    AzureCliCredentialError(#[from] super::AzureCliCredentialError),
    #[error("Error getting environment credential: {0}")]
    EnvironmentCredentialError(#[from] super::EnvironmentCredentialError),
    #[error("Error getting managed identity credential: {0}")]
    ManagedIdentityCredentialError(#[from] super::ManagedIdentityCredentialError),
    #[error(
        "Multiple errors were encountered while attempting to authenticate:\n{}",
        format_aggregate_error(.0)
    )]
    CredentialUnavailable(Vec<DefaultAzureCredentialError>),
}

/// Types of TokenCredential supported by DefaultAzureCredential
pub enum DefaultAzureCredentialEnum {
    Environment(EnvironmentCredential),
    ManagedIdentity(ImdsManagedIdentityCredential),
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
                .map_err(DefaultAzureCredentialError::EnvironmentCredentialError),
            DefaultAzureCredentialEnum::ManagedIdentity(credential) => credential
                .get_token(resource)
                .await
                .map_err(DefaultAzureCredentialError::ManagedIdentityCredentialError),
            DefaultAzureCredentialEnum::AzureCli(credential) => credential
                .get_token(resource)
                .await
                .map_err(DefaultAzureCredentialError::AzureCliCredentialError),
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
    pub fn with_sources(sources: Vec<DefaultAzureCredentialEnum>) -> Self {
        DefaultAzureCredential { sources }
    }
}

impl Default for DefaultAzureCredential {
    fn default() -> Self {
        DefaultAzureCredential {
            sources: vec![
                DefaultAzureCredentialEnum::Environment(EnvironmentCredential::default()),
                DefaultAzureCredentialEnum::ManagedIdentity(ImdsManagedIdentityCredential {}),
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
impl azure_core::TokenCredential for DefaultAzureCredential {
    async fn get_token(
        &self,
        resource: &str,
    ) -> Result<azure_core::TokenResponse, azure_core::Error> {
        TokenCredential::get_token(self, resource)
            .await
            .map_err(|error| azure_core::Error::GetTokenError(Box::new(error)))
    }
}

fn format_aggregate_error(errors: &[DefaultAzureCredentialError]) -> String {
    errors
        .iter()
        .map(|error| error.to_string())
        .collect::<Vec<String>>()
        .join("\n")
}

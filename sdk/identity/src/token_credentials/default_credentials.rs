use super::{
    AzureCliCredential, EnvironmentCredential, ManagedIdentityCredential, TokenCredential,
};
use azure_core::TokenResponse;

#[derive(Debug, Default)]
/// Provides a mechanism of selectively disabling credentials used for a `DefaultCredential` instance
pub struct DefaultCredentialBuilder {
    include_environment_credential: bool,
    include_managed_identity_credential: bool,
    include_cli_credential: bool,
}

impl DefaultCredentialBuilder {
    /// Create a new `DefaultCredentialBuilder`
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

    pub fn build(&self) -> DefaultCredential {
        let source_count = self.include_cli_credential as usize
            + self.include_cli_credential as usize
            + self.include_managed_identity_credential as usize;
        let mut sources = Vec::<DefaultCredentialEnum>::with_capacity(source_count);
        if self.include_environment_credential {
            sources.push(DefaultCredentialEnum::Environment(
                EnvironmentCredential::default(),
            ));
        }
        if self.include_managed_identity_credential {
            sources.push(DefaultCredentialEnum::ManagedIdentity(
                ManagedIdentityCredential {},
            ))
        }
        if self.include_cli_credential {
            sources.push(DefaultCredentialEnum::AzureCli(AzureCliCredential {}));
        }
        DefaultCredential::with_sources(sources)
    }
}

#[non_exhaustive]
#[derive(Debug, thiserror::Error)]
pub enum DefaultCredentialError {
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
    EndOfDefaultList(Vec<DefaultCredentialError>),
}

/// Types of TokenCredential supported by DefaultCredential
pub enum DefaultCredentialEnum {
    Environment(EnvironmentCredential),
    ManagedIdentity(ManagedIdentityCredential),
    AzureCli(AzureCliCredential),
}

#[async_trait::async_trait]
impl TokenCredential for DefaultCredentialEnum {
    type Error = DefaultCredentialError;

    async fn get_token(&self, resource: &str) -> Result<TokenResponse, Self::Error> {
        match self {
            DefaultCredentialEnum::Environment(credential) => credential
                .get_token(resource)
                .await
                .map_err(DefaultCredentialError::EnvironmentCredentialError),
            DefaultCredentialEnum::ManagedIdentity(credential) => credential
                .get_token(resource)
                .await
                .map_err(DefaultCredentialError::ManagedIdentityCredentialError),
            DefaultCredentialEnum::AzureCli(credential) => credential
                .get_token(resource)
                .await
                .map_err(DefaultCredentialError::AzureCliCredentialError),
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
pub struct DefaultCredential {
    sources: Vec<DefaultCredentialEnum>,
}

impl DefaultCredential {
    pub fn with_sources(sources: Vec<DefaultCredentialEnum>) -> Self {
        DefaultCredential { sources }
    }
}

impl Default for DefaultCredential {
    fn default() -> Self {
        DefaultCredential {
            sources: vec![
                DefaultCredentialEnum::Environment(EnvironmentCredential::default()),
                DefaultCredentialEnum::ManagedIdentity(ManagedIdentityCredential {}),
                DefaultCredentialEnum::AzureCli(AzureCliCredential {}),
            ],
        }
    }
}

#[async_trait::async_trait]
impl TokenCredential for DefaultCredential {
    type Error = DefaultCredentialError;
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
        Err(DefaultCredentialError::EndOfDefaultList(errors))
    }
}

#[async_trait::async_trait]
impl azure_core::TokenCredential for DefaultCredential {
    async fn get_token(
        &self,
        resource: &str,
    ) -> Result<azure_core::TokenResponse, azure_core::Error> {
        TokenCredential::get_token(self, resource)
            .await
            .map_err(|error| azure_core::Error::GetTokenError(Box::new(error)))
    }
}

fn format_aggregate_error(errors: &[DefaultCredentialError]) -> String {
    errors
        .iter()
        .map(|error| error.to_string())
        .collect::<Vec<String>>()
        .join("\n")
}

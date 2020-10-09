use crate::token_credentials::AzureCliCredential;
use crate::{
    token_credentials::{EnvironmentCredential, ManagedIdentityCredential, TokenCredential},
    TokenResponse,
};
use azure_sdk_core::errors::AzureError;
use log::debug;

/// Provides a mechanism of selectively disabling credentials used for a `DefaultCredential` instance
pub struct DefaultCredentialBuilder {
    include_environment_credential: bool,
    include_managed_identity_credential: bool,
    include_cli_credential: bool,
}

impl DefaultCredentialBuilder {
    pub fn new() -> Self {
        DefaultCredentialBuilder {
            include_cli_credential: true,
            include_managed_identity_credential: true,
            include_environment_credential: true,
        }
    }

    pub fn exclude_environment_credential(&mut self) -> &mut Self {
        self.include_environment_credential = false;
        self
    }
    pub fn exclude_cli_credential(&mut self) -> &mut Self {
        self.include_cli_credential = false;
        self
    }
    pub fn exclude_managed_identity_credential(&mut self) -> &mut Self {
        self.include_managed_identity_credential = false;
        self
    }
    fn source_count(&self) -> usize {
        self.include_cli_credential as usize
            + self.include_cli_credential as usize
            + self.include_managed_identity_credential as usize
    }
    pub fn build(&self) -> DefaultCredential {
        let mut sources =
            Vec::<Box<dyn TokenCredential + Send + Sync>>::with_capacity(self.source_count());
        if self.include_environment_credential {
            sources.push(Box::new(EnvironmentCredential {}));
        }
        if self.include_managed_identity_credential {
            sources.push(Box::new(ManagedIdentityCredential {}))
        }
        if self.include_cli_credential {
            sources.push(Box::new(AzureCliCredential {}));
        }
        DefaultCredential::with_sources(sources)
    }
}

/// Provides a default `TokenCredential` authentication flow for applications that will be deployed to Azure.  The following credential
/// types if enabled will be tried, in order:
/// - EnvironmentCredential
/// - ManagedIdentityCredential
/// - AzureCliCredential
/// Consult the documentation of these credential types for more information on how they attempt authentication.
pub struct DefaultCredential {
    sources: Vec<Box<dyn TokenCredential + Send + Sync>>,
}

impl DefaultCredential {
    pub fn with_sources(sources: Vec<Box<dyn TokenCredential + Send + Sync>>) -> Self {
        DefaultCredential { sources }
    }
}

impl Default for DefaultCredential {
    fn default() -> Self {
        DefaultCredential {
            sources: vec![
                Box::new(EnvironmentCredential {}),
                Box::new(ManagedIdentityCredential {}),
                Box::new(AzureCliCredential {}),
            ],
        }
    }
}

#[async_trait::async_trait]
impl TokenCredential for DefaultCredential {
    async fn get_token(&self, resource: &str) -> Result<TokenResponse, AzureError> {
        for source in &self.sources {
            let token_res = source.get_token(resource).await;

            if let Ok(token) = token_res {
                return Ok(token);
            } else {
                debug!("Failed to get credentials: {:?}", token_res.err().unwrap());
            }
        }

        Err(AzureError::GenericErrorWithText(
            "End of default list".to_owned(),
        ))
    }
}

use super::{ClientSecretCredential, TokenCredential, TokenCredentialOptions};
use azure_core::auth::TokenResponse;

const AZURE_TENANT_ID_ENV_KEY: &str = "AZURE_TENANT_ID";
const AZURE_CLIENT_ID_ENV_KEY: &str = "AZURE_CLIENT_ID";
const AZURE_CLIENT_SECRET_ENV_KEY: &str = "AZURE_CLIENT_SECRET";
const AZURE_USERNAME_ENV_KEY: &str = "AZURE_USERNAME";
const AZURE_PASSWORD_ENV_KEY: &str = "AZURE_PASSWORD";
const AZURE_CLIENT_CERTIFICATE_PATH_ENV_KEY: &str = "AZURE_CLIENT_CERTIFICATE_PATH";

/// Enables authentication to Azure Active Directory using client secret, or a username and password.
///
/// Details configured in the following environment variables:
///
/// | Variable                            | Description                                      |
/// |-------------------------------------|--------------------------------------------------|
/// | `AZURE_TENANT_ID`                   | The Azure Active Directory tenant(directory) ID. |
/// | `AZURE_CLIENT_ID`                   | The client(application) ID of an App Registration in the tenant. |
/// | `AZURE_CLIENT_SECRET`               | A client secret that was generated for the App Registration. |
///
/// This credential ultimately uses a `ClientSecretCredential` to perform the authentication using
/// these details.
/// Please consult the documentation of that class for more details.
#[derive(Clone, Debug, Default)]
pub struct EnvironmentCredential {
    options: TokenCredentialOptions,
}

impl EnvironmentCredential {
    /// Creates a new `EnvironmentCredential` with the given `TokenCredentialOptions`.
    pub fn new(options: TokenCredentialOptions) -> Self {
        Self { options }
    }
}

#[allow(missing_docs)]
#[non_exhaustive]
#[derive(Debug, thiserror::Error)]
pub enum EnvironmentCredentialError {
    #[error(
        "Missing tenant id set in {} environment variable",
        AZURE_TENANT_ID_ENV_KEY
    )]
    MissingTenantId(#[source] std::env::VarError),
    #[error(
        "Missing client id set in {} environment variable",
        AZURE_CLIENT_ID_ENV_KEY
    )]
    MissingClientId(#[source] std::env::VarError),
    #[error("No valid environment credential providers")]
    NoValid,
    #[error(transparent)]
    ClientSecretCredential(super::ClientSecretCredentialError),
}

#[async_trait::async_trait]
impl TokenCredential for EnvironmentCredential {
    type Error = EnvironmentCredentialError;

    async fn get_token(&self, resource: &str) -> Result<TokenResponse, Self::Error> {
        let tenant_id = std::env::var(AZURE_TENANT_ID_ENV_KEY)
            .map_err(EnvironmentCredentialError::MissingTenantId)?;
        let client_id = std::env::var(AZURE_CLIENT_ID_ENV_KEY)
            .map_err(EnvironmentCredentialError::MissingClientId)?;

        let client_secret = std::env::var(AZURE_CLIENT_SECRET_ENV_KEY);
        let username = std::env::var(AZURE_USERNAME_ENV_KEY);
        let password = std::env::var(AZURE_PASSWORD_ENV_KEY);
        let client_certificate_path = std::env::var(AZURE_CLIENT_CERTIFICATE_PATH_ENV_KEY);

        if let Ok(client_secret) = client_secret {
            let credential = ClientSecretCredential::new(
                tenant_id,
                client_id,
                client_secret,
                self.options.clone(),
            );
            return credential
                .get_token(resource)
                .await
                .map_err(EnvironmentCredentialError::ClientSecretCredential);
        } else if username.is_ok() && password.is_ok() {
            // Could use multiple if-let with #![feature(let_chains)] once stabilised - see https://github.com/rust-lang/rust/issues/53667
            // TODO: username & password credential
        } else if let Ok(_path) = client_certificate_path {
            // TODO: client certificate credential
            todo!()
        }

        Err(EnvironmentCredentialError::NoValid)
    }
}

#[async_trait::async_trait]
impl azure_core::auth::TokenCredential for EnvironmentCredential {
    async fn get_token(
        &self,
        resource: &str,
    ) -> Result<azure_core::auth::TokenResponse, azure_core::Error> {
        TokenCredential::get_token(self, resource)
            .await
            .map_err(|error| azure_core::Error::GetToken(Box::new(error)))
    }
}

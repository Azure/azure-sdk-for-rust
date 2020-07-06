use crate::{ClientSecretCredential, TokenCredential};
use azure_sdk_core::errors::AzureError;

const AZURE_TENANT_ID_ENV_KEY: &str = "AZURE_TENANT_ID";
const AZURE_CLIENT_ID_ENV_KEY: &str = "AZURE_CLIENT_ID";
const AZURE_CLIENT_SECRET_ENV_KEY: &str = "AZURE_CLIENT_SECRET";
const AZURE_USERNAME_ENV_KEY: &str = "AZURE_USERNAME";
const AZURE_PASSWORD_ENV_KEY: &str = "AZURE_PASSWORD";
const AZURE_CLIENT_CERTIFICATE_PATH_ENV_KEY: &str = "AZURE_CLIENT_CERTIFICATE_PATH";

/// Enables authentication to Azure Active Directory using client secret, or username and password,
/// details configured in the following environment variables:
///
/// | Variable                            | Description                                      |
/// |-------------------------------------|--------------------------------------------------|
/// | `AZURE_TENANT_ID`                   | The Azure Active Directory tenant(directory) ID. |
/// | `AZURE_CLIENT_ID`                   | The client(application) ID of an App Registration in the tenant. |
/// | `AZURE_CLIENT_SECRET`               | A client secret that was generated for the App Registration. |
///
/// This credential ultimately uses a `ClientSecretCredential` to
/// perform the authentication using these details. Please consult the
/// documentation of that class for more details.
pub struct EnvironmentCredential;

#[async_trait::async_trait]
impl TokenCredential for EnvironmentCredential {
    async fn get_token(&self, resource: &str) -> Result<crate::TokenResponse, AzureError> {
        let tenant_id = std::env::var(AZURE_TENANT_ID_ENV_KEY).map_err(|_| {
            AzureError::GenericErrorWithText(format!(
                "Missing tenant id set in {} environment variable",
                AZURE_TENANT_ID_ENV_KEY
            ))
        })?;
        let client_id = std::env::var(AZURE_CLIENT_ID_ENV_KEY).map_err(|_| {
            AzureError::GenericErrorWithText(format!(
                "Missing client id set in {} environment variable",
                AZURE_CLIENT_ID_ENV_KEY
            ))
        })?;

        let client_secret = std::env::var(AZURE_CLIENT_SECRET_ENV_KEY);
        let username = std::env::var(AZURE_USERNAME_ENV_KEY);
        let password = std::env::var(AZURE_PASSWORD_ENV_KEY);
        let client_certificate_path = std::env::var(AZURE_CLIENT_CERTIFICATE_PATH_ENV_KEY);

        if let Ok(client_secret) = client_secret {
            let credential = ClientSecretCredential::new(tenant_id, client_id, client_secret);
            return credential.get_token(resource).await;
        } else if username.is_ok() && password.is_ok() {
            // Could use multiple if-let with #![feature(let_chains)] once stabilised - see https://github.com/rust-lang/rust/issues/53667
            // TODO: username & password credential
        } else if let Ok(_) = client_certificate_path {
            // TODO: client certificate credential
        }

        Err(AzureError::GenericErrorWithText(
            "No valid environment credential providers".to_string(),
        ))
    }
}

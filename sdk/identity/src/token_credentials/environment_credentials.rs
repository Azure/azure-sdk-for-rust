use super::{ClientSecretCredential, TokenCredentialOptions};
use azure_core::auth::{TokenCredential, TokenResponse};
use azure_core::error::{Error, ErrorKind, ResultExt};
use azure_core::HttpClient;
use std::sync::Arc;

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
#[derive(Clone, Debug)]
pub struct EnvironmentCredential {
    http_client: Arc<dyn HttpClient>,
    options: TokenCredentialOptions,
}

impl Default for EnvironmentCredential {
    /// Creates an instance of the `EnvironmentCredential` using the default `HttpClient`.
    fn default() -> Self {
        Self::new(
            azure_core::new_http_client(),
            TokenCredentialOptions::default(),
        )
    }
}

impl EnvironmentCredential {
    /// Creates a new `EnvironmentCredential` with the given `TokenCredentialOptions`.
    pub fn new(http_client: Arc<dyn HttpClient>, options: TokenCredentialOptions) -> Self {
        Self {
            http_client,
            options,
        }
    }
}

#[cfg_attr(target_arch = "wasm32", async_trait::async_trait(?Send))]
#[cfg_attr(not(target_arch = "wasm32"), async_trait::async_trait)]
impl TokenCredential for EnvironmentCredential {
    async fn get_token(&self, resource: &str) -> azure_core::Result<TokenResponse> {
        let tenant_id =
            std::env::var(AZURE_TENANT_ID_ENV_KEY).with_context(ErrorKind::Credential, || {
                format!(
                    "missing tenant id set in {} environment variable",
                    AZURE_TENANT_ID_ENV_KEY
                )
            })?;
        let client_id =
            std::env::var(AZURE_CLIENT_ID_ENV_KEY).with_context(ErrorKind::Credential, || {
                format!(
                    "missing client id set in {} environment variable",
                    AZURE_CLIENT_ID_ENV_KEY
                )
            })?;

        let client_secret = std::env::var(AZURE_CLIENT_SECRET_ENV_KEY);
        let username = std::env::var(AZURE_USERNAME_ENV_KEY);
        let password = std::env::var(AZURE_PASSWORD_ENV_KEY);
        let client_certificate_path = std::env::var(AZURE_CLIENT_CERTIFICATE_PATH_ENV_KEY);

        if let Ok(client_secret) = client_secret {
            let credential = ClientSecretCredential::new(
                self.http_client.clone(),
                tenant_id,
                client_id,
                client_secret,
                self.options.clone(),
            );
            return credential.get_token(resource).await;
        } else if username.is_ok() && password.is_ok() {
            // Could use multiple if-let with #![feature(let_chains)] once stabilised - see https://github.com/rust-lang/rust/issues/53667
            // TODO: username & password credential
        } else if let Ok(_path) = client_certificate_path {
            // TODO: client certificate credential
            todo!()
        }

        Err(Error::message(
            ErrorKind::Credential,
            "no valid environment credential providers",
        ))
    }
}

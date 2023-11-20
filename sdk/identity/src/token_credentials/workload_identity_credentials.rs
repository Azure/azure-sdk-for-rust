use azure_core::auth::{AccessToken, TokenCredential, TokenResponse};
use azure_core::error::{ErrorKind, ResultExt};
use azure_core::HttpClient;
use std::str;
use std::sync::Arc;
use std::time::Duration;
use time::OffsetDateTime;

use crate::{federated_credentials_flow, TokenCredentialOptions};

/// Enables authentication to Azure Active Directory using a client secret that was generated for an App Registration.
///
/// More information on how to configure a client secret can be found here:
/// <https://docs.microsoft.com/azure/active-directory/develop/quickstart-configure-app-access-web-apis#add-credentials-to-your-web-application>
pub struct WorkloadIdentityCredential {
    http_client: Arc<dyn HttpClient>,
    tenant_id: String,
    client_id: String,
    token: String,
    options: TokenCredentialOptions,
}

impl WorkloadIdentityCredential {
    /// Create a new `WorkloadIdentityCredential`
    pub fn new(
        http_client: Arc<dyn HttpClient>,
        tenant_id: String,
        client_id: String,
        token: String,
    ) -> Self {
        Self {
            http_client,
            tenant_id,
            client_id,
            token,
            options: TokenCredentialOptions::default(),
        }
    }

    /// set `TokenCredentialOptions`
    pub fn set_options(&mut self, options: TokenCredentialOptions) {
        self.options = options;
    }
}

#[cfg_attr(target_arch = "wasm32", async_trait::async_trait(?Send))]
#[cfg_attr(not(target_arch = "wasm32"), async_trait::async_trait)]
impl TokenCredential for WorkloadIdentityCredential {
    async fn get_token(&self, resource: &str) -> azure_core::Result<TokenResponse> {
        let res: TokenResponse = federated_credentials_flow::perform(
            self.http_client.clone(),
            &self.client_id,
            &self.token,
            &[&format!("{resource}/.default")],
            &self.tenant_id,
            self.options.authority_host(),
        )
        .await
        .map(|r| {
            TokenResponse::new(
                AccessToken::new(r.access_token().secret().to_owned()),
                OffsetDateTime::now_utc() + Duration::from_secs(r.expires_in),
            )
        })
        .context(ErrorKind::Credential, "request token error")?;
        Ok(res)
    }
}

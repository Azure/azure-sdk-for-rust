use super::internal_server::*;
use crate::authorization_code_flow;
use azure_core::{error::ErrorKind, http::new_http_client, http::Url, Error};
use oauth2::{
    basic::BasicTokenType, AuthorizationCode, ClientId, EmptyExtraTokenFields,
    StandardTokenResponse,
};
use std::{str::FromStr, sync::Arc};

/// Default OAuth scopes used when none are provided.
#[allow(dead_code)]
const DEFAULT_SCOPE_ARR: [&str; 3] = ["openid", "offline_access", "profile"];
/// Default client ID for interactive browser authentication.
#[allow(dead_code)]
const DEFAULT_DEVELOPER_SIGNON_CLIENT_ID: &str = "04b07795-8ddb-461a-bbee-02f9e1bf7b46";
/// Default tenant ID used when none is specified.
#[allow(dead_code)]
const DEFAULT_ORGANIZATIONS_TENANT_ID: &str = "organizations";

/// Provides interactive browser-based authentication.
#[derive(Clone)]
pub struct InteractiveBrowserCredential {
    /// Client ID of the application.
    pub(crate) client_id: ClientId,
    /// Tenant ID for the authentication request.
    pub(crate) tenant_id: String,
    /// Redirect URI where the authentication response is sent.
    pub(crate) redirect_url: Url,
}

impl InteractiveBrowserCredential {
    /// Creates a new `InteractiveBrowserCredential` instance with optional parameters.
    pub fn new(
        client_id: Option<ClientId>,
        tenant_id: Option<String>,
        redirect_url: Option<Url>,
    ) -> azure_core::Result<Arc<Self>> {
        let client_id = client_id
            .unwrap_or_else(|| ClientId::new(DEFAULT_DEVELOPER_SIGNON_CLIENT_ID.to_owned()));

        let tenant_id = tenant_id.unwrap_or_else(|| DEFAULT_ORGANIZATIONS_TENANT_ID.to_owned());

        let redirect_url = redirect_url.unwrap_or_else(|| {
            Url::from_str(&format!("http://localhost:{}", LOCAL_SERVER_PORT))
                .expect("Failed to parse redirect URL")
        });

        Ok(Arc::new(Self {
            client_id,
            tenant_id,
            redirect_url,
        }))
    }

    /// Starts the interactive browser authentication flow and returns an access token.
    ///
    /// If no scopes are provided, default scopes will be used.
    #[allow(dead_code)]
    pub async fn get_token(
        &self,
        scopes: Option<&[&str]>,
    ) -> azure_core::Result<StandardTokenResponse<EmptyExtraTokenFields, BasicTokenType>> {
        let scopes = scopes.unwrap_or(&DEFAULT_SCOPE_ARR);

        let authorization_code_flow = authorization_code_flow::authorize(
            self.client_id.clone(),
            None,
            &self.tenant_id,
            self.redirect_url.clone(),
            scopes,
        );

        let auth_code = open_url(authorization_code_flow.authorize_url.as_ref()).await;

        match auth_code {
            Some(code) => {
                authorization_code_flow
                    .exchange(new_http_client(), AuthorizationCode::new(code))
                    .await
            }
            None => Err(Error::message(
                ErrorKind::Other,
                "Failed to retrieve authorization code.",
            )),
        }
    }
}
#[cfg(test)]
mod tests {
    use super::*;
    use tracing::debug;
    use tracing::Level;
    use tracing_subscriber;
    static INIT: std::sync::Once = std::sync::Once::new();

    fn init_tracing() {
        INIT.call_once(|| {
            tracing_subscriber::fmt()
                .with_max_level(Level::DEBUG)
                .init();
        });
    }

    #[tokio::test]
    async fn interactive_auth_flow_should_return_token() {
        init_tracing();
        debug!("Starting interactive authentication test");

        let credential = InteractiveBrowserCredential::new(None, None, None)
            .expect("Failed to create credential");

        let token_response = credential.get_token(None).await;
        debug!("Authentication result: {:#?}", token_response);
        assert!(token_response.is_ok());
    }
}

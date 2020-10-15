//! This crate provides mechanisms for several ways to authenticate against Azure

#[cfg(feature = "development")]
pub mod development;
mod device_code_flow;
mod device_code_responses;
pub mod errors;
mod login_response;
mod refresh_token;
mod responses;
pub mod token_credentials;
mod traits;

pub use device_code_flow::*;
pub use device_code_responses::*;
pub use login_response::*;
pub use refresh_token::*;

use azure_core::errors::AzureError;
use futures::TryFutureExt;
use log::debug;
use oauth2::basic::BasicClient;
use oauth2::reqwest::async_http_client;
use oauth2::{ClientId, ClientSecret};
use url::{form_urlencoded, Url};

/// An object representing an OAuth2 authorization code flow.
///
/// To learn more about the OAuth2 authentication flow, read the guide [here](https://docs.microsoft.com/en-us/azure/active-directory/develop/v2-oauth2-auth-code-flow).
#[derive(Debug)]
pub struct AuthorizeCodeFlow {
    /// An HTTP client configured for OAuth2 authentication
    pub client: BasicClient,
    /// The authentication HTTP endpoint
    pub authorize_url: Url,
    /// The CSRF token
    pub csrf_state: oauth2::CsrfToken,
    /// The PKCE code verifier
    pub pkce_code_verifier: oauth2::PkceCodeVerifier,
}

impl AuthorizeCodeFlow {
    /// Create a new `AuthorizeCodeFlow`.
    ///
    /// The values for `client_id`, `client_secret`, `tenant_id`, and `redirect_url` can all be found
    /// inside of the Azure portal.
    ///
    /// This object contains information for performing oauth authentication.
    pub fn new(
        client_id: ClientId,
        client_secret: Option<ClientSecret>,
        tenant_id: &str,
        redirect_url: Url,
        resource: &str,
    ) -> Self {
        let auth_url = oauth2::AuthUrl::from_url(
            Url::parse(&format!(
                "https://login.microsoftonline.com/{}/oauth2/v2.0/authorize",
                tenant_id
            ))
            .expect("Invalid authorization endpoint URL"),
        );
        let token_url = oauth2::TokenUrl::from_url(
            Url::parse(&format!(
                "https://login.microsoftonline.com/{}/oauth2/v2.0/token",
                tenant_id
            ))
            .expect("Invalid token endpoint URL"),
        );

        // Set up the config for the Microsoft Graph OAuth2 process.
        let client = BasicClient::new(client_id, client_secret, auth_url, Some(token_url))
            // Microsoft Graph requires client_id and client_secret in URL rather than
            // using Basic authentication.
            .set_auth_type(oauth2::AuthType::RequestBody)
            .set_redirect_url(oauth2::RedirectUrl::from_url(redirect_url));

        // Microsoft Graph supports Proof Key for Code Exchange (PKCE - https://oauth.net/2/pkce/).
        // Create a PKCE code verifier and SHA-256 encode it as a code challenge.
        let (pkce_code_challenge, pkce_code_verifier) =
            oauth2::PkceCodeChallenge::new_random_sha256();

        // Generate the authorization URL to which we'll redirect the user.
        let (authorize_url, csrf_state) = client
            .authorize_url(oauth2::CsrfToken::new_random)
            .add_extra_param("scope", resource)
            .set_pkce_challenge(pkce_code_challenge)
            .url();

        Self {
            client,
            authorize_url,
            csrf_state,
            pkce_code_verifier,
        }
    }

    /// Exchange an authorization code for a token.
    pub async fn exchange(
        self,
        code: oauth2::AuthorizationCode,
    ) -> Result<
        oauth2::StandardTokenResponse<oauth2::EmptyExtraTokenFields, oauth2::basic::BasicTokenType>,
        oauth2::RequestTokenError<
            oauth2::reqwest::Error<reqwest::Error>,
            oauth2::StandardErrorResponse<oauth2::basic::BasicErrorResponseType>,
        >,
    > {
        let token = self
            .client
            .exchange_code(code)
            // Send the PKCE code verifier in the token request
            .set_pkce_verifier(self.pkce_code_verifier)
            .request_async(async_http_client)
            .await?;

        debug!("\nMS Graph returned the following token:\n{:?}\n", token);
        Ok(token)
    }
}

/// Authorize using the client credentials flow
///
/// You can read more about this here: https://docs.microsoft.com/en-us/azure/active-directory/develop/v2-oauth2-client-creds-grant-flow
pub async fn authorize_client_credentials_flow(
    client: reqwest::Client,
    //  grant_type: &str, fixed on "client_credentials",
    client_id: &oauth2::ClientId,
    client_secret: &oauth2::ClientSecret,
    scope: &str,
    tenant_id: &str,
) -> Result<LoginResponse, AzureError> {
    let encoded: String = form_urlencoded::Serializer::new(String::new())
        .append_pair("client_id", client_id.as_str())
        .append_pair("scope", scope)
        .append_pair("client_secret", client_secret.secret())
        .append_pair("grant_type", "client_credentials")
        .finish();

    let url = url::Url::parse(&format!(
        "https://login.microsoftonline.com/{}/oauth2/v2.0/token",
        tenant_id
    ))
    .map_err(|error| AzureError::GenericErrorWithText(error.to_string()))?;

    client
        .post(url)
        .header("ContentType", "Application / WwwFormUrlEncoded")
        .body(encoded)
        .send()
        .await
        .map_err(|e| AzureError::GenericErrorWithText(e.to_string()))?
        .text()
        .map_err(|e| AzureError::GenericErrorWithText(e.to_string()))
        .await
        .and_then(|s| {
            serde_json::from_str::<LoginResponse>(&s).map_err(|e| {
                serde_json::from_str::<errors::ErrorResponse>(&s)
                    .map(|er| AzureError::GenericErrorWithText(er.to_string()))
                    .unwrap_or_else(|_| {
                        AzureError::GenericErrorWithText(format!(
                            "Failed to parse Azure response: {}",
                            e.to_string()
                        ))
                    })
            })
        })
}

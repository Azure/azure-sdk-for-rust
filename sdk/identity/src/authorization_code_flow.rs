//! Authorize using the authorization code flow
//!
//! You can learn more about the OAuth2 authorization code flow [here](https://docs.microsoft.com/azure/active-directory/develop/v2-oauth2-auth-code-flow).

use oauth2::basic::BasicClient;
use oauth2::reqwest::async_http_client;
use oauth2::{ClientId, ClientSecret};
use url::Url;

/// Start an authorization code flow.
///
/// The values for `client_id`, `client_secret`, `tenant_id`, and `redirect_url` can all be found
/// inside of the Azure portal.
pub fn start(
    client_id: ClientId,
    client_secret: Option<ClientSecret>,
    tenant_id: &str,
    redirect_url: Url,
    resource: &str,
) -> AuthorizationCodeFlow {
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
        .set_redirect_uri(oauth2::RedirectUrl::from_url(redirect_url));

    // Microsoft Graph supports Proof Key for Code Exchange (PKCE - https://oauth.net/2/pkce/).
    // Create a PKCE code verifier and SHA-256 encode it as a code challenge.
    let (pkce_code_challenge, pkce_code_verifier) = oauth2::PkceCodeChallenge::new_random_sha256();

    // Generate the authorization URL to which we'll redirect the user.
    let (authorize_url, csrf_state) = client
        .authorize_url(oauth2::CsrfToken::new_random)
        .add_extra_param("scope", resource)
        .set_pkce_challenge(pkce_code_challenge)
        .url();

    AuthorizationCodeFlow {
        client,
        authorize_url,
        csrf_state,
        pkce_code_verifier,
    }
}

/// An object representing an OAuth 2.0 authorization code flow.
#[derive(Debug)]
pub struct AuthorizationCodeFlow {
    /// An HTTP client configured for OAuth2 authentication
    pub client: BasicClient,
    /// The authentication HTTP endpoint
    pub authorize_url: Url,
    /// The CSRF token
    pub csrf_state: oauth2::CsrfToken,
    /// The PKCE code verifier
    pub pkce_code_verifier: oauth2::PkceCodeVerifier,
}

impl AuthorizationCodeFlow {
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

        Ok(token)
    }
}

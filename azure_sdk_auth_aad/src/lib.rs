#![feature(async_closure)]
#[macro_use]
extern crate serde_derive;
extern crate log;
use azure_sdk_core::errors::AzureError;
use log::debug;
use oauth2::basic::BasicClient;
use oauth2::reqwest::async_http_client;
use oauth2::AsyncCodeTokenRequest;
use oauth2::{
    AuthType, AuthUrl, AuthorizationCode, CsrfToken, PkceCodeChallenge, PkceCodeVerifier,
    RedirectUrl, TokenUrl,
};
pub use oauth2::{ClientId, ClientSecret};
use url::form_urlencoded;
use url::Url;
mod login_response;
pub use login_response::*;
mod device_code_responses;
use std::sync::Arc;
mod device_code_flow;
pub mod errors;
mod refresh_token;
pub use refresh_token::*;
mod naive_server;
mod traits;
pub use crate::device_code_flow::*;
pub use crate::device_code_responses::*;
use futures::TryFutureExt;
mod responses;
pub use naive_server::naive_server;
mod prelude;
mod token_credentials;
pub use token_credentials::*;

#[derive(Debug)]
pub struct AuthObj {
    pub client: BasicClient,
    pub authorize_url: Url,
    pub csrf_state: CsrfToken,
    pub pkce_code_verifier: PkceCodeVerifier,
}

pub fn authorize_code_flow(
    client_id: ClientId,
    client_secret: Option<ClientSecret>,
    tenant_id: &str,
    redirect_url: Url,
    resource: &str,
) -> AuthObj {
    let auth_url = AuthUrl::from_url(
        Url::parse(&format!(
            "https://login.microsoftonline.com/{}/oauth2/v2.0/authorize",
            tenant_id
        ))
        .expect("Invalid authorization endpoint URL"),
    );
    let token_url = TokenUrl::from_url(
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
        .set_auth_type(AuthType::RequestBody)
        .set_redirect_url(RedirectUrl::from_url(redirect_url));

    // Microsoft Graph supports Proof Key for Code Exchange (PKCE - https://oauth.net/2/pkce/).
    // Create a PKCE code verifier and SHA-256 encode it as a code challenge.
    let (pkce_code_challenge, pkce_code_verifier) = PkceCodeChallenge::new_random_sha256();

    // Generate the authorization URL to which we'll redirect the user.
    let (authorize_url, csrf_state) = client
        .authorize_url(CsrfToken::new_random)
        .add_extra_param("scope", resource)
        .set_pkce_challenge(pkce_code_challenge)
        .url();

    AuthObj {
        client,
        authorize_url,
        csrf_state,
        pkce_code_verifier,
    }
}

pub async fn exchange(
    auth_obj: AuthObj,
    code: AuthorizationCode,
) -> Result<
    oauth2::StandardTokenResponse<oauth2::EmptyExtraTokenFields, oauth2::basic::BasicTokenType>,
    oauth2::RequestTokenError<
        oauth2::reqwest::Error<reqwest::Error>,
        oauth2::StandardErrorResponse<oauth2::basic::BasicErrorResponseType>,
    >,
> {
    // Exchange the code with a token.
    let token = auth_obj
        .client
        .exchange_code(code)
        // Send the PKCE code verifier in the token request
        .set_pkce_verifier(auth_obj.pkce_code_verifier)
        .request_async(async_http_client)
        .await?;

    debug!("\nMS Graph returned the following token:\n{:?}\n", token);
    Ok(token)
}

pub async fn authorize_client_credentials_flow(
    client: Arc<reqwest::Client>,
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

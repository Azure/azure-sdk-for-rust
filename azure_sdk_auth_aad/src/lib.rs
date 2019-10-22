#[macro_use]
extern crate failure;
#[macro_use]
extern crate serde_derive;
extern crate log;
use azure_sdk_core::errors::AzureError;
use futures::future::{done, ok, Future};
use log::debug;
use oauth2::basic::BasicClient;
use oauth2::reqwest::http_client;
use oauth2::{
    AuthType, AuthUrl, AuthorizationCode, ClientId, ClientSecret, CsrfToken, PkceCodeChallenge, PkceCodeVerifier, RedirectUrl, TokenUrl,
};
use std::str::FromStr;
use url::form_urlencoded;
use url::Url;
mod login_response;
use azure_sdk_core::perform_http_request;
use http::status::StatusCode;
use hyper::{Body, Client, Request};
pub use login_response::*;
use std::sync::Arc;
pub mod errors;
mod naive_server;
pub use naive_server::naive_server;

#[derive(Debug)]
pub struct AuthObj {
    pub client: BasicClient,
    pub authorize_url: Url,
    pub csrf_state: CsrfToken,
    pub pkce_code_verifier: PkceCodeVerifier,
}

pub fn authorize_delegate(client_id: ClientId, client_secret: ClientSecret, tenant_id: &str, redirect_url: Url, resource: &str) -> AuthObj {
    let auth_url = AuthUrl::new(
        Url::parse(&format!("https://login.microsoftonline.com/{}/oauth2/authorize", tenant_id))
            .expect("Invalid authorization endpoint URL"),
    );
    let token_url = TokenUrl::new(
        Url::parse(&format!("https://login.microsoftonline.com/{}/oauth2/v2.0/token", tenant_id)).expect("Invalid token endpoint URL"),
    );

    // Set up the config for the Microsoft Graph OAuth2 process.
    let client = BasicClient::new(client_id, Some(client_secret), auth_url, Some(token_url))
        // Microsoft Graph requires client_id and client_secret in URL rather than
        // using Basic authentication.
        .set_auth_type(AuthType::RequestBody)
        .set_redirect_url(RedirectUrl::new(redirect_url));

    // Microsoft Graph supports Proof Key for Code Exchange (PKCE - https://oauth.net/2/pkce/).
    // Create a PKCE code verifier and SHA-256 encode it as a code challenge.
    let (pkce_code_challenge, pkce_code_verifier) = PkceCodeChallenge::new_random_sha256();

    // Generate the authorization URL to which we'll redirect the user.
    let (authorize_url, csrf_state) = client
        .authorize_url(CsrfToken::new_random)
        .add_extra_param("resource", resource) //"https://management.azure.com/".to_owned())
        .set_pkce_challenge(pkce_code_challenge)
        .url();

    AuthObj {
        client,
        authorize_url,
        csrf_state,
        pkce_code_verifier,
    }
}

pub fn exchange(
    auth_obj: AuthObj,
    code: AuthorizationCode,
) -> Result<
    oauth2::StandardTokenResponse<oauth2::EmptyExtraTokenFields, oauth2::basic::BasicTokenType>,
    oauth2::RequestTokenError<oauth2::reqwest::Error, oauth2::StandardErrorResponse<oauth2::basic::BasicErrorResponseType>>,
> {
    // Exchange the code with a token.
    let token = auth_obj
        .client
        .exchange_code(code)
        // Send the PKCE code verifier in the token request
        .set_pkce_verifier(auth_obj.pkce_code_verifier)
        .request(http_client);

    debug!("MS Graph returned the following token:\n{:?}\n", token);
    token
}

pub fn authorize_non_interactive(
    client: Arc<Client<hyper_rustls::HttpsConnector<hyper::client::HttpConnector>>>,
    //  grant_type: &str, fixed on "client_credentials",
    client_id: &oauth2::ClientId,
    client_secret: &oauth2::ClientSecret,
    resource: &str,
    tenant_id: &str,
) -> impl Future<Item = LoginResponse, Error = AzureError> {
    let encoded: String = form_urlencoded::Serializer::new(String::new())
        .append_pair("grant_type", "client_credentials")
        .append_pair("client_id", client_id.as_str())
        .append_pair("client_secret", client_secret.secret())
        .append_pair("resource", resource)
        .finish();

    let uri = format!("https://login.microsoftonline.com/{}/oauth2/token", tenant_id);

    done(
        Request::builder()
            .method("POST")
            .header("ContentType", "Application / WwwFormUrlEncoded")
            .uri(uri)
            .body(Body::from(encoded)),
    )
    .from_err()
    .and_then(move |request| {
        perform_http_request(&client, request, StatusCode::OK).and_then(|resp| {
            done(LoginResponse::from_str(&resp)).from_err().and_then(|r| {
                debug!("{:?}", r);
                ok(r)
            })
        })
    })
}

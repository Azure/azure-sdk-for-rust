//! Authorize using the OAuth 2.0 client credentials flow
//!
//! For example:
//!
//! ```no_run
//! use azure_identity::client_credentials_flow;
//! use oauth2::{ClientId, ClientSecret};
//! use url::Url;
//!
//! use std::env;
//! use std::error::Error;
//!
//! #[tokio::main]
//! async fn main() -> Result<(), Box<dyn Error>> {
//!     let client_id =
//!         ClientId::new(env::var("CLIENT_ID").expect("Missing CLIENT_ID environment variable."));
//!     let client_secret = ClientSecret::new(
//!         env::var("CLIENT_SECRET").expect("Missing CLIENT_SECRET environment variable."),
//!     );
//!     let tenant_id = env::var("TENANT_ID").expect("Missing TENANT_ID environment variable.");
//!     let subscription_id =
//!         env::var("SUBSCRIPTION_ID").expect("Missing SUBSCRIPTION_ID environment variable.");
//!
//!     let client = reqwest::Client::new();
//!     // This will give you the final token to use in authorization.
//!     let token = client_credentials_flow::perform(
//!         client,
//!         &client_id,
//!         &client_secret,
//!         &["https://management.azure.com/"],
//!         &tenant_id,
//!     )
//!     .await?;
//!     Ok(())
//! }
//! ```
//!
//! You can learn more about this athorization flow [here](https://docs.microsoft.com/azure/active-directory/develop/v2-oauth2-client-creds-grant-flow).

mod login_response;

use login_response::LoginResponse;
use url::form_urlencoded;

/// Perform the client credentials flow
pub async fn perform(
    client: reqwest::Client,
    client_id: &oauth2::ClientId,
    client_secret: &oauth2::ClientSecret,
    scopes: &[&str],
    tenant_id: &str,
) -> Result<LoginResponse, ClientCredentialError> {
    let encoded: String = form_urlencoded::Serializer::new(String::new())
        .append_pair("client_id", client_id.as_str())
        .append_pair("scope", &scopes.join(" "))
        .append_pair("client_secret", client_secret.secret())
        .append_pair("grant_type", "client_credentials")
        .finish();

    let url = url::Url::parse(&format!(
        "https://login.microsoftonline.com/{}/oauth2/v2.0/token",
        tenant_id
    ))
    .map_err(|_| ClientCredentialError::InvalidTenantId(tenant_id.to_owned()))?;

    let response = client
        .post(url)
        .header("ContentType", "Application / WwwFormUrlEncoded")
        .body(encoded)
        .send()
        .await
        .map_err(|e| ClientCredentialError::Request(Box::new(e)))?;

    if !response.status().is_success() {
        return Err(ClientCredentialError::UnsuccessfulResponse(
            response.status().as_u16(),
            response.text().await.ok(),
        ));
    }

    let b = response
        .text()
        .await
        .map_err(|e| ClientCredentialError::Request(Box::new(e)))?;

    serde_json::from_str::<LoginResponse>(&b)
        .map_err(|_| ClientCredentialError::InvalidResponseBody(b))
}

/// Errors when performing the client credential flow
#[derive(thiserror::Error, Debug)]
pub enum ClientCredentialError {
    /// The http response was unsuccessful
    #[error("The http response was unsuccessful with status {0}: {}", .1.as_deref().unwrap_or("<NO UTF-8 BODY>"))]
    UnsuccessfulResponse(u16, Option<String>),
    /// The http response body was could not be turned into a client credential response
    #[error("The http response body could not be turned into a client credential response: {0}")]
    InvalidResponseBody(String),
    /// The tenant id could not be url encoded
    #[error("The supplied tenant id could not be url encoded: {0}")]
    InvalidTenantId(String),
    /// An error occurred when trying to make a request
    #[error("An error occurred when trying to make a request")]
    Request(#[source] Box<dyn std::error::Error + Send + Sync>),
}

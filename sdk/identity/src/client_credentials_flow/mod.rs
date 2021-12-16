//! Authorize using the client credentials flow
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

use crate::Error;
use login_response::LoginResponse;
use url::form_urlencoded;

/// Perform the client credentials flow
pub async fn perform(
    client: reqwest::Client,
    client_id: &oauth2::ClientId,
    client_secret: &oauth2::ClientSecret,
    scopes: &[&str],
    tenant_id: &str,
) -> Result<LoginResponse, Error> {
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

    let s = client
        .post(url)
        .header("ContentType", "Application / WwwFormUrlEncoded")
        .body(encoded)
        .send()
        .await
        .map_err(|e| ClientCredentialError::ReqwestError(e))?
        .text()
        .await
        .map_err(|e| ClientCredentialError::ReqwestError(e))?;
    // TODO The HTTP status code should be checked to deserialize an error response.

    Ok(serde_json::from_str::<LoginResponse>(&s)
        .map_err(|_| ClientCredentialError::InvalidResponse(s))?)
}

#[derive(thiserror::Error, Debug)]
pub enum ClientCredentialError {
    #[error("Invalid client credential response: {0}")]
    InvalidResponse(String),
    #[error("Invalid tenant id: {0}")]
    InvalidTenantId(String),
    #[error("Reqwest error: {0}")]
    ReqwestError(reqwest::Error),
}

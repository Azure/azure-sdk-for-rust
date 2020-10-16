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
//!         "https://management.azure.com/",
//!         &tenant_id,
//!     )
//!     .await?;
//!     Ok(())
//! }
//! ```
//!
//! You can learn more about this athorization flow [here](https://docs.microsoft.com/en-us/azure/active-directory/develop/v2-oauth2-client-creds-grant-flow).

mod login_response;

use super::errors;
use login_response::LoginResponse;

use azure_core::errors::AzureError;
use futures::TryFutureExt;
use url::form_urlencoded;

/// Perform the client credentials flow
pub async fn perform(
    client: reqwest::Client,
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

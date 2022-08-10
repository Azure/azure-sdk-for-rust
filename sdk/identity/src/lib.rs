//! Azure Identity crate for the unofficial Microsoft Azure SDK for Rust. This crate is part of a collection of crates: for more information please refer to [https://github.com/azure/azure-sdk-for-rust](https://github.com/azure/azure-sdk-for-rust).
//! This crate provides mechanisms for several ways to authenticate against Azure
//!
//! For example, to authenticate using the recommended DefaultAzureCredential, you can do the following:
//!
//! ```no_run
//! use azure_core::auth::TokenCredential;
//! use azure_identity::{DefaultAzureCredential};
//! use url::Url;
//!
//! use std::env;
//! use std::error::Error;
//!
//! #[tokio::main]
//! async fn main() -> Result<(), Box<dyn Error>> {
//!     let credential = DefaultAzureCredential::default();
//!     let response = credential
//!         .get_token("https://management.azure.com")
//!         .await?;
//!
//!     let subscription_id = env::var("AZURE_SUBSCRIPTION_ID")?;
//!     let url = Url::parse(&format!(
//!         "https://management.azure.com/subscriptions/{}/providers/Microsoft.Storage/storageAccounts?api-version=2019-06-01",
//!         subscription_id))?;
//!     let response = reqwest::Client::new()
//!         .get(url)
//!         .header("Authorization", format!("Bearer {}", response.token.secret()))
//!         .send()
//!         .await?
//!         .text()
//!         .await?;
//!
//!     println!("{:?}", response);
//!     Ok(())
//! }
//! ```
//!
//! The supported authentication flows are:
//! * [Authorization code flow](https://docs.microsoft.com/azure/active-directory/develop/v2-oauth2-auth-code-flow).
//! * [Client credentials flow](https://docs.microsoft.com/azure/active-directory/develop/v2-oauth2-client-creds-grant-flow).
//! * [Device code flow](https://docs.microsoft.com/azure/active-directory/develop/v2-oauth2-device-code).
//!
//! This crate also includes utilities for handling refresh tokens and accessing token credentials from many different sources.

pub mod authorization_code_flow;
pub mod client_credentials_flow;
#[cfg(feature = "development")]
pub mod development;
pub mod device_code_flow;
pub mod refresh_token;
mod token_credentials;

pub use crate::token_credentials::*;

pub(crate) async fn oauth2_async_http_client(
    _request: oauth2::HttpRequest,
) -> Result<oauth2::HttpResponse, azure_core::error::Error> {
    // let client = {
    //     let builder = reqwest::Client::builder();

    //     // Following redirects opens the client up to SSRF vulnerabilities.
    //     // but this is not possible to prevent on wasm targets
    //     #[cfg(not(target_arch = "wasm32"))]
    //     let builder = builder.redirect(reqwest::redirect::Policy::none());

    //     builder.build().map_err(Error::Reqwest)?
    // };

    // let mut request_builder = client
    //     .request(request.method, request.url.as_str())
    //     .body(request.body);
    // for (name, value) in &request.headers {
    //     request_builder = request_builder.header(name.as_str(), value.as_bytes());
    // }
    // let request = request_builder.build().map_err(Error::Reqwest)?;

    // let response = client.execute(request).await.map_err(Error::Reqwest)?;

    // let status_code = response.status();
    // let headers = response.headers().to_owned();
    // let chunks = response.bytes().await.map_err(Error::Reqwest)?;

    let status_code = http::StatusCode::CREATED;
    let headers = http::HeaderMap::new();
    let body = vec![];

    Ok(oauth2::HttpResponse {
        status_code,
        headers,
        body,
    })
}

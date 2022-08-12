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

use std::sync::Arc;

use azure_core::{
    error::{Error, ErrorKind, ResultExt},
    HttpClient, Request,
};

pub use crate::token_credentials::*;

pub(crate) struct Oauth2HttpClient {
    http_client: Arc<dyn HttpClient>,
}

impl Oauth2HttpClient {
    /// Create a new Oauth2HttpClient
    pub fn new(http_client: Arc<dyn HttpClient>) -> Self {
        Self { http_client }
    }

    pub(crate) async fn request_async(
        &self,
        oauth2_request: oauth2::HttpRequest,
    ) -> Result<oauth2::HttpResponse, azure_core::error::Error> {
        let method = try_from_method(&oauth2_request.method)?;
        let request = Request::new(oauth2_request.url, method);
        let response = self.http_client.execute_request(&request).await?;
        let status_code = try_from_status(response.status())?;
        let headers = try_from_headers(response.headers())?;
        let body = response.into_body().collect().await?.to_vec();
        Ok(oauth2::HttpResponse {
            status_code,
            headers,
            body,
        })
    }
}

fn try_from_method(method: &oauth2::http::Method) -> azure_core::Result<azure_core::Method> {
    if method == oauth2::http::Method::GET {
        Ok(azure_core::Method::Get)
    } else if method == oauth2::http::Method::POST {
        Ok(azure_core::Method::Post)
    } else if method == oauth2::http::Method::PUT {
        Ok(azure_core::Method::Put)
    } else if method == oauth2::http::Method::DELETE {
        Ok(azure_core::Method::Delete)
    } else if method == oauth2::http::Method::HEAD {
        Ok(azure_core::Method::Head)
    } else if method == oauth2::http::Method::OPTIONS {
        Ok(azure_core::Method::Options)
    } else if method == oauth2::http::Method::CONNECT {
        Ok(azure_core::Method::Connect)
    } else if method == oauth2::http::Method::PATCH {
        Ok(azure_core::Method::Patch)
    } else if method == oauth2::http::Method::TRACE {
        Ok(azure_core::Method::Trace)
    } else {
        Err(Error::with_message(ErrorKind::DataConversion, || {
            format!("unsupported oauth2::http::Method {}", method)
        }))
    }
}

fn try_from_headers(
    _headers: &azure_core::headers::Headers,
) -> azure_core::Result<oauth2::http::HeaderMap> {
    let header_map = oauth2::http::HeaderMap::new();
    // TODO
    // for (name, value) in headers.iter() {
    //     let name = name.as_str();
    //     let value = value.as_str();
    //     header_map.append(
    //         name,
    //         http::HeaderValue::from_str(&value).with_context(ErrorKind::DataConversion, || {
    //             format!("unable to convert http header '{}'", name)
    //         })?,
    //     );
    // }
    Ok(header_map)
}

fn try_from_status(status: azure_core::StatusCode) -> azure_core::Result<oauth2::http::StatusCode> {
    oauth2::http::StatusCode::from_u16(status as u16).map_kind(ErrorKind::DataConversion)
}

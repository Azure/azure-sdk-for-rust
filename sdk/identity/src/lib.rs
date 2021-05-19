//! This crate provides mechanisms for several ways to authenticate against Azure
//!
//! For example, to authenticate using the client credential flow, you can do the following:
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
//! The supported authentication flows are:
//! * [Authorization code flow](https://docs.microsoft.com/en-us/azure/active-directory/develop/v2-oauth2-auth-code-flow).
//! * [Client credentials flow](https://docs.microsoft.com/en-us/azure/active-directory/develop/v2-oauth2-client-creds-grant-flow).
//! * [Device code flow](https://docs.microsoft.com/en-us/azure/active-directory/develop/v2-oauth2-device-code).
//!
//! This crate also includes utilities for handling refresh tokens and accessing token credentials from many different sources.

pub mod authorization_code_flow;
// pub mod client_credentials_flow;
#[cfg(feature = "development")]
pub mod development;
// pub mod device_code_flow;
mod errors;
pub use errors::Error;
pub mod refresh_token;
pub mod token_credentials;
mod traits;

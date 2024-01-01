//! Azure Identity crate for the unofficial Microsoft Azure SDK for Rust. This crate is part of a collection of crates: for more information please refer to [https://github.com/azure/azure-sdk-for-rust](https://github.com/azure/azure-sdk-for-rust).
//!
//! This crate provides several implementations of the [azure_core::auth::TokenCredential](https://docs.rs/azure_core/latest/azure_core/auth/trait.TokenCredential.html) trait.
//! It is recommended to use `azure_identity::new_credential()`, which will create an instance of `DefaultAzureCredential`. It will in turn attempt to create and use several credential types. If you want to use a specific credential type, the `AZURE_CREDENTIAL_TYPE` environment variable may be set to a value from `azure_credential_types`, such as `azurecli` or `virtualmachine`.
//!
//! ```no_run
//!#[tokio::main]
//!async fn main() -> Result<(), Box<dyn std::error::Error>> {
//!    let subscription_id =
//!        std::env::var("AZURE_SUBSCRIPTION_ID").expect("AZURE_SUBSCRIPTION_ID required");
//!
//!    let credential = azure_identity::new_credential();
//!
//!    // Let's enumerate the Azure storage accounts in the subscription using the REST API directly.
//!    // This is just an example. It is easier to use the Azure SDK for Rust crates.
//!    let url = url::Url::parse(&format!("https://management.azure.com/subscriptions/{subscription_id}/providers/Microsoft.Storage/storageAccounts?api-version=2019-06-01"))?;
//!
//!    let access_token = credential
//!        .get_token(&["https://management.azure.com/.default"])
//!        .await?;
//!
//!    let response = reqwest::Client::new()
//!        .get(url)
//!        .header(
//!            "Authorization",
//!            format!("Bearer {}", access_token.token.secret()),
//!        )
//!        .send()
//!        .await?
//!        .text()
//!        .await?;
//!
//!    println!("{response}");
//!    Ok(())
//!}
//! ```
//!
//! The supported authentication flows are:
//! * [Authorization code flow](https://docs.microsoft.com/azure/active-directory/develop/v2-oauth2-auth-code-flow).
//! * [Client credentials flow](https://docs.microsoft.com/azure/active-directory/develop/v2-oauth2-client-creds-grant-flow).
//! * [Device code flow](https://docs.microsoft.com/azure/active-directory/develop/v2-oauth2-device-code).

pub mod authorization_code_flow;
pub mod client_credentials_flow;
#[cfg(feature = "development")]
pub mod development;
pub mod device_code_flow;
pub mod env;
pub mod federated_credentials_flow;
mod oauth2_http_client;
pub mod refresh_token;
mod timeout;
mod token_credentials;

pub use crate::token_credentials::*;

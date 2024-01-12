//! Access to token credentials through various means
//!
//! Supported means currently include:
//! * The environment
//! * Azure CLI credentials cache
//! * Managed identity
//! * Client secret
#[cfg(not(target_arch = "wasm32"))]
mod azure_cli_credentials;
#[cfg(feature = "azureauth-cli")]
#[cfg(not(target_arch = "wasm32"))]
mod azureauth_cli_credentials;
mod cache;
#[cfg(feature = "client_certificate")]
mod client_certificate_credentials;
mod client_secret_credentials;
mod default_credentials;
mod empty_credential;
mod environment_credentials;
mod imds_managed_identity_credentials;
mod workload_identity_credentials;

#[cfg(not(target_arch = "wasm32"))]
pub use azure_cli_credentials::*;
#[cfg(feature = "azureauth-cli")]
#[cfg(not(target_arch = "wasm32"))]
pub use azureauth_cli_credentials::*;
#[cfg(feature = "client_certificate")]
pub use client_certificate_credentials::*;
pub use client_secret_credentials::*;
pub use default_credentials::*;
pub use empty_credential::*;
pub use environment_credentials::*;
pub use imds_managed_identity_credentials::*;
pub use workload_identity_credentials::*;

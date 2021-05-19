//! Access to token credentials through various means
//!
//! Supported means currently include:
//! * The environment
//! * Azure CLI credentials cache
//! * Managed identity
//! * Client secret
mod cli_credentials;
// mod client_secret_credentials;
// mod default_credentials;
// mod environment_credentials;
// mod managed_identity_credentials;

pub use cli_credentials::*;
// pub use client_secret_credentials::*;
// pub use default_credentials::*;
// pub use environment_credentials::*;
// pub use managed_identity_credentials::*;

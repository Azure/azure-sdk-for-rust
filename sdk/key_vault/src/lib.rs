mod client;
pub mod secret;
pub use client::KeyVaultClient;
pub use secret::RecoveryLevel;

use thiserror::Error;

#[derive(Error, Debug)]
pub enum KeyVaultError {
    #[error("Key Vault does not exist, or is unreachable at '{keyvault_name:?}.vault.azure.net'")]
    KeyVaultDoesNotExist { keyvault_name: String },

    #[error("Azure Active Directory authorization error")]
    AuthorizationError(#[from] anyhow::Error),

    #[error("General error: {0}")]
    GeneralError(String),
}

mod client;
pub mod key;
pub mod secret;

pub use client::KeyClient;
pub use secret::RecoveryLevel;

use thiserror::Error;

#[derive(Error, Debug)]
pub enum KeyVaultError {
    #[error("Key Vault does not exist, or is unreachable at '{keyvault_name:?}.vault.azure.net'")]
    KeyVaultDoesNotExist { keyvault_name: String },

    #[error("Azure Active Directory authorization error")]
    Authorization,

    #[error("Received an error accessing the Key Vault, which could not be parsed as expected.")]
    UnparsableError,

    #[error(transparent)]
    Reqwest(#[from] reqwest::Error),

    #[error("Key Vault Error: {0}")]
    General(String),

    #[error("Failed to parse response from Key Vault: {0}")]
    SerdeParse(#[from] serde_json::Error),

    #[error(transparent)]
    Error(#[from] anyhow::Error),
}

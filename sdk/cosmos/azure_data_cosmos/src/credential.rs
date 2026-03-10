// Copyright (c) Microsoft Corporation. All rights reserved.
// Licensed under the MIT License.

//! Credential types for authenticating with Azure Cosmos DB.

use azure_core::credentials::TokenCredential;
use std::sync::Arc;

#[cfg(feature = "key_auth")]
use azure_core::credentials::Secret;

/// Authentication credential for connecting to a Cosmos DB account.
///
/// Either key-based authentication using a master key, or token-based
/// authentication using an Azure credential (e.g., managed identity, service principal).
///
/// # Examples
///
/// Using Entra ID (Azure AD) authentication:
///
/// ```rust,no_run
/// use azure_data_cosmos::CosmosCredential;
/// use std::sync::Arc;
///
/// let credential: Arc<dyn azure_core::credentials::TokenCredential> =
///     azure_identity::DeveloperToolsCredential::new(None).unwrap();
/// let credential: CosmosCredential = credential.into();
/// ```
///
/// Using key authentication (requires `key_auth` feature):
///
/// ```rust,no_run,ignore
/// use azure_data_cosmos::CosmosCredential;
/// use azure_core::credentials::Secret;
///
/// let credential: CosmosCredential = Secret::from("my_account_key").into();
/// ```
#[derive(Clone)]
#[non_exhaustive]
pub enum CosmosCredential {
    /// Entra ID (Azure AD) token credential.
    TokenCredential(Arc<dyn TokenCredential>),
    /// Primary or secondary account key.
    #[cfg(feature = "key_auth")]
    MasterKey(Secret),
}

impl std::fmt::Debug for CosmosCredential {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::TokenCredential(_) => f.debug_tuple("TokenCredential").field(&"...").finish(),
            #[cfg(feature = "key_auth")]
            Self::MasterKey(_) => f.debug_tuple("MasterKey").field(&"***").finish(),
        }
    }
}

impl From<Arc<dyn TokenCredential>> for CosmosCredential {
    fn from(credential: Arc<dyn TokenCredential>) -> Self {
        Self::TokenCredential(credential)
    }
}

#[cfg(feature = "key_auth")]
impl From<Secret> for CosmosCredential {
    fn from(key: Secret) -> Self {
        Self::MasterKey(key)
    }
}

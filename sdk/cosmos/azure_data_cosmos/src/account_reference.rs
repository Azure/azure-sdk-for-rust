// Copyright (c) Microsoft Corporation. All rights reserved.
// Licensed under the MIT License.

//! Account reference types for Azure Cosmos DB.

use crate::{CosmosAccountEndpoint, CosmosCredential};

#[cfg(feature = "key_auth")]
use azure_core::credentials::Secret;
use azure_core::credentials::TokenCredential;
use azure_core::http::Url;
use std::sync::Arc;

/// A reference to a Cosmos DB account, combining an endpoint with a credential.
///
/// This type bundles together the account endpoint and the credential needed to
/// authenticate with it. Use convenience constructors [`with_credential()`](Self::with_credential)
/// or [`with_master_key()`](Self::with_master_key) to create instances.
///
/// # Examples
///
/// Using Entra ID authentication:
///
/// ```rust,no_run
/// use azure_data_cosmos::{CosmosAccountReference, CosmosAccountEndpoint};
/// use std::sync::Arc;
///
/// let credential: Arc<dyn azure_core::credentials::TokenCredential> =
///     azure_identity::DeveloperToolsCredential::new(None).unwrap();
/// let endpoint: CosmosAccountEndpoint = "https://myaccount.documents.azure.com/".parse().unwrap();
/// let account = CosmosAccountReference::with_credential(endpoint, credential);
/// ```
///
/// Using key authentication (requires `key_auth` feature):
///
/// ```rust,ignore
/// use azure_data_cosmos::{CosmosAccountReference, CosmosAccountEndpoint};
/// use azure_core::credentials::Secret;
///
/// let endpoint: CosmosAccountEndpoint = "https://myaccount.documents.azure.com/".parse().unwrap();
/// let account = CosmosAccountReference::with_master_key(endpoint, Secret::from("my_account_key"));
/// ```
#[derive(Clone, Debug)]
#[non_exhaustive]
pub struct CosmosAccountReference {
    endpoint: CosmosAccountEndpoint,
    credential: CosmosCredential,
}

impl CosmosAccountReference {
    /// Creates a new account reference with an Entra ID (Azure AD) token credential.
    ///
    /// # Arguments
    ///
    /// * `endpoint` - The Cosmos DB account endpoint.
    /// * `credential` - An Entra ID token credential.
    pub fn with_credential(
        endpoint: CosmosAccountEndpoint,
        credential: Arc<dyn TokenCredential>,
    ) -> Self {
        Self {
            endpoint,
            credential: CosmosCredential::from(credential),
        }
    }

    /// Creates a new account reference with a master key.
    ///
    /// # Arguments
    ///
    /// * `endpoint` - The Cosmos DB account endpoint.
    /// * `key` - The primary or secondary account key.
    #[cfg(feature = "key_auth")]
    pub fn with_master_key(endpoint: CosmosAccountEndpoint, key: Secret) -> Self {
        Self {
            endpoint,
            credential: CosmosCredential::from(key),
        }
    }

    /// Returns the endpoint and credential as a tuple.
    ///
    /// This is used internally by the builder to extract the components.
    pub(crate) fn into_parts(self) -> (CosmosAccountEndpoint, CosmosCredential) {
        (self.endpoint, self.credential)
    }
}

// Conversion from (endpoint, credential) tuples for ergonomic use.

impl<C: Into<CosmosCredential>> From<(CosmosAccountEndpoint, C)> for CosmosAccountReference {
    fn from((endpoint, credential): (CosmosAccountEndpoint, C)) -> Self {
        Self {
            endpoint,
            credential: credential.into(),
        }
    }
}

impl<C: Into<CosmosCredential>> From<(Url, C)> for CosmosAccountReference {
    fn from((url, credential): (Url, C)) -> Self {
        Self {
            endpoint: CosmosAccountEndpoint::from(url),
            credential: credential.into(),
        }
    }
}

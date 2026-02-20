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
/// or [`with_master_key()`](Self::with_master_key) to create instances, or build one manually
/// with [`CosmosAccountReferenceBuilder`].
///
/// # Examples
///
/// Using Entra ID authentication:
///
/// ```rust,no_run
/// use azure_data_cosmos::CosmosAccountReference;
/// use std::sync::Arc;
///
/// let credential: Arc<dyn azure_core::credentials::TokenCredential> =
///     azure_identity::DeveloperToolsCredential::new(None).unwrap();
/// let account = CosmosAccountReference::with_credential(
///     "https://myaccount.documents.azure.com/",
///     credential,
/// ).unwrap();
/// ```
///
/// Using key authentication (requires `key_auth` feature):
///
/// ```rust,no_run,ignore
/// use azure_data_cosmos::CosmosAccountReference;
/// use azure_core::credentials::Secret;
///
/// let account = CosmosAccountReference::with_master_key(
///     "https://myaccount.documents.azure.com/",
///     Secret::from("my_account_key"),
/// ).unwrap();
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
    /// * `endpoint` - The Cosmos DB account endpoint URL (e.g. `"https://myaccount.documents.azure.com/"`).
    /// * `credential` - An Entra ID token credential.
    ///
    /// # Errors
    ///
    /// Returns an error if the endpoint URL cannot be parsed.
    pub fn with_credential(
        endpoint: impl AsRef<str>,
        credential: Arc<dyn TokenCredential>,
    ) -> azure_core::Result<Self> {
        let endpoint: CosmosAccountEndpoint = endpoint.as_ref().parse()?;
        Ok(Self {
            endpoint,
            credential: CosmosCredential::from(credential),
        })
    }

    /// Creates a new account reference with a master key.
    ///
    /// # Arguments
    ///
    /// * `endpoint` - The Cosmos DB account endpoint URL (e.g. `"https://myaccount.documents.azure.com/"`).
    /// * `key` - The primary or secondary account key.
    ///
    /// # Errors
    ///
    /// Returns an error if the endpoint URL cannot be parsed.
    #[cfg(feature = "key_auth")]
    pub fn with_master_key(endpoint: impl AsRef<str>, key: Secret) -> azure_core::Result<Self> {
        let endpoint: CosmosAccountEndpoint = endpoint.as_ref().parse()?;
        Ok(Self {
            endpoint,
            credential: CosmosCredential::from(key),
        })
    }

    /// Returns the endpoint and credential as a tuple.
    ///
    /// This is used internally by the builder to extract the components.
    pub(crate) fn into_parts(self) -> (CosmosAccountEndpoint, CosmosCredential) {
        (self.endpoint, self.credential)
    }
}

/// Builder for creating [`CosmosAccountReference`] instances.
///
/// # Examples
///
/// ```rust,no_run
/// use azure_data_cosmos::CosmosAccountReferenceBuilder;
/// use azure_data_cosmos::CosmosCredential;
/// use std::sync::Arc;
///
/// let credential: Arc<dyn azure_core::credentials::TokenCredential> =
///     azure_identity::DeveloperToolsCredential::new(None).unwrap();
/// let account = CosmosAccountReferenceBuilder::new(
///     "https://myaccount.documents.azure.com/",
///     credential,
/// ).unwrap()
///     .build();
/// ```
#[derive(Clone, Debug)]
#[non_exhaustive]
pub struct CosmosAccountReferenceBuilder {
    endpoint: CosmosAccountEndpoint,
    credential: CosmosCredential,
}

impl CosmosAccountReferenceBuilder {
    /// Creates a new builder with the given endpoint and credential.
    ///
    /// # Arguments
    ///
    /// * `endpoint` - The Cosmos DB account endpoint URL.
    /// * `credential` - The authentication credential.
    ///
    /// # Errors
    ///
    /// Returns an error if the endpoint URL cannot be parsed.
    pub fn new(
        endpoint: impl AsRef<str>,
        credential: impl Into<CosmosCredential>,
    ) -> azure_core::Result<Self> {
        let endpoint: CosmosAccountEndpoint = endpoint.as_ref().parse()?;
        Ok(Self {
            endpoint,
            credential: credential.into(),
        })
    }

    /// Builds the [`CosmosAccountReference`].
    pub fn build(self) -> CosmosAccountReference {
        CosmosAccountReference {
            endpoint: self.endpoint,
            credential: self.credential,
        }
    }
}

// Conversion from (endpoint_str, credential) tuples for ergonomic use.

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

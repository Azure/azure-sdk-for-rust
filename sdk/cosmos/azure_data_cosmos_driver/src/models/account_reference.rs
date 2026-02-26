// Copyright (c) Microsoft Corporation. All rights reserved.
// Licensed under the MIT License.

//! Account reference and authentication types.

use azure_core::credentials::{Secret, TokenCredential};
use std::{hash::Hash, sync::Arc};
use url::Url;

/// An account endpoint URL used as a cache key.
///
/// This is a newtype wrapper around `Url` that implements `Hash` and `Eq`
/// based on the URL only (ignoring authentication). Used as a key in
/// account-scoped caches.
#[derive(Clone, Debug)]
pub(crate) struct AccountEndpoint(Url);

impl AccountEndpoint {
    /// Creates a new account endpoint from a URL.
    pub(crate) fn new(url: Url) -> Self {
        Self(url)
    }

    /// Returns the endpoint URL.
    pub(crate) fn url(&self) -> &Url {
        &self.0
    }

    /// Returns the host portion of the endpoint URL.
    ///
    /// Returns an empty string if the URL has no host (which shouldn't
    /// happen for valid Cosmos DB endpoints).
    pub(crate) fn host(&self) -> &str {
        self.0.host_str().unwrap_or("")
    }

    /// Joins a resource path to this endpoint to create a full request URL.
    ///
    /// The path should be the resource path (e.g., "/dbs/mydb/colls/mycoll").
    /// Leading slashes in the path are handled correctly.
    pub(crate) fn join_path(&self, path: &str) -> Url {
        let mut url = self.0.clone();
        // Set the path, handling leading slash
        let normalized_path = if path.starts_with('/') {
            path.to_string()
        } else if path.is_empty() {
            String::new()
        } else {
            format!("/{}", path)
        };
        url.set_path(&normalized_path);
        url
    }
}

impl PartialEq for AccountEndpoint {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}

impl Eq for AccountEndpoint {}

impl std::fmt::Display for AccountEndpoint {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.write_str(self.0.as_str())
    }
}

impl Hash for AccountEndpoint {
    fn hash<H: std::hash::Hasher>(&self, state: &mut H) {
        self.0.hash(state);
    }
}

impl From<Url> for AccountEndpoint {
    fn from(url: Url) -> Self {
        Self::new(url)
    }
}

impl TryFrom<&str> for AccountEndpoint {
    type Error = url::ParseError;

    fn try_from(value: &str) -> Result<Self, Self::Error> {
        Ok(Self::new(Url::parse(value)?))
    }
}

impl From<&AccountReference> for AccountEndpoint {
    fn from(account: &AccountReference) -> Self {
        account.endpoint.clone()
    }
}

/// Authentication options for connecting to a Cosmos DB account.
///
/// Either key-based authentication using a master key, or token-based
/// authentication using an Azure credential (e.g., managed identity, service principal).
#[derive(Clone)]
pub enum AuthOptions {
    /// Key-based authentication using the account's primary or secondary master key.
    MasterKey(Secret),
    /// Token-based authentication using an Azure credential.
    TokenCredential(Arc<dyn TokenCredential>),
}

impl std::fmt::Debug for AuthOptions {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::MasterKey(_) => f.debug_tuple("MasterKey").field(&"***").finish(),
            Self::TokenCredential(_) => f.debug_tuple("TokenCredential").field(&"...").finish(),
        }
    }
}

impl From<Secret> for AuthOptions {
    fn from(key: Secret) -> Self {
        Self::MasterKey(key)
    }
}

impl From<Arc<dyn TokenCredential>> for AuthOptions {
    fn from(credential: Arc<dyn TokenCredential>) -> Self {
        Self::TokenCredential(credential)
    }
}

/// A reference to a Cosmos DB account.
///
/// Contains the service endpoint and authentication credentials. Authentication
/// is required - use [`AccountReferenceBuilder`] to construct an instance.
///
/// # Examples
///
/// ```
/// use azure_data_cosmos_driver::models::AccountReference;
/// use url::Url;
///
/// // With master key authentication
/// let account = AccountReference::builder(
///     Url::parse("https://myaccount.documents.azure.com:443/").unwrap(),
/// )
/// .master_key("my-master-key")
/// .build()
/// .unwrap();
///
/// // Using the shorthand constructor
/// let account = AccountReference::with_master_key(
///     Url::parse("https://myaccount.documents.azure.com:443/").unwrap(),
///     "my-master-key",
/// );
/// ```
#[derive(Clone, Debug)]
#[non_exhaustive]
pub struct AccountReference {
    /// The service endpoint URL (required).
    endpoint: AccountEndpoint,
    /// Authentication credentials (required).
    auth: AuthOptions,
}

// Manual PartialEq implementation because AuthOptions contains Arc<dyn TokenCredential>
// which doesn't implement PartialEq. We compare by endpoint only.
impl PartialEq for AccountReference {
    fn eq(&self, other: &Self) -> bool {
        self.endpoint == other.endpoint
    }
}

impl Eq for AccountReference {}

// Manual Hash implementation to match PartialEq (compares by endpoint only).
impl std::hash::Hash for AccountReference {
    fn hash<H: std::hash::Hasher>(&self, state: &mut H) {
        self.endpoint.hash(state);
    }
}

impl AccountReference {
    /// Creates a new builder for an account reference.
    ///
    /// Use this to construct an `AccountReference` with the required authentication.
    pub fn builder(endpoint: Url) -> AccountReferenceBuilder {
        AccountReferenceBuilder::new(endpoint)
    }

    /// Creates a new account reference with master key authentication.
    ///
    /// This is a convenience method for the common case of key-based auth.
    pub fn with_master_key(endpoint: Url, key: impl Into<Secret>) -> Self {
        Self {
            endpoint: AccountEndpoint::from(endpoint),
            auth: AuthOptions::MasterKey(key.into()),
        }
    }

    /// Creates a new account reference with token credential authentication.
    ///
    /// This is a convenience method for token-based auth (e.g., managed identity).
    pub fn with_credential(endpoint: Url, credential: Arc<dyn TokenCredential>) -> Self {
        Self {
            endpoint: AccountEndpoint::from(endpoint),
            auth: AuthOptions::TokenCredential(credential),
        }
    }

    /// Returns the service endpoint URL.
    pub fn endpoint(&self) -> &Url {
        self.endpoint.url()
    }

    /// Returns the authentication options.
    ///
    /// Authentication is always present - it's required during construction.
    pub fn auth(&self) -> &AuthOptions {
        &self.auth
    }
}

/// Builder for constructing an [`AccountReference`].
///
/// Authentication must be configured before calling `build()`.
///
/// # Example
///
/// ```
/// use azure_data_cosmos_driver::models::AccountReference;
/// use url::Url;
///
/// let account = AccountReference::builder(
///     Url::parse("https://myaccount.documents.azure.com:443/").unwrap(),
/// )
/// .master_key("my-master-key")
/// .build()
/// .unwrap();
/// ```
#[derive(Debug)]
#[non_exhaustive]
pub struct AccountReferenceBuilder {
    endpoint: AccountEndpoint,
    auth: Option<AuthOptions>,
}

impl AccountReferenceBuilder {
    /// Creates a new builder with the specified endpoint.
    pub fn new(endpoint: Url) -> Self {
        Self {
            endpoint: AccountEndpoint::from(endpoint),
            auth: None,
        }
    }

    /// Sets the service endpoint URL.
    pub fn endpoint(mut self, endpoint: Url) -> Self {
        self.endpoint = AccountEndpoint::from(endpoint);
        self
    }

    /// Sets master key authentication.
    pub fn master_key(mut self, key: impl Into<Secret>) -> Self {
        self.auth = Some(AuthOptions::MasterKey(key.into()));
        self
    }

    /// Sets token credential authentication.
    pub fn credential(mut self, credential: Arc<dyn TokenCredential>) -> Self {
        self.auth = Some(AuthOptions::TokenCredential(credential));
        self
    }

    /// Sets authentication options directly.
    pub fn auth(mut self, auth: AuthOptions) -> Self {
        self.auth = Some(auth);
        self
    }

    /// Builds the account reference.
    ///
    /// # Errors
    ///
    /// Returns an error if authentication has not been configured.
    pub fn build(self) -> azure_core::Result<AccountReference> {
        let auth = self.auth.ok_or_else(|| {
            azure_core::Error::with_message(
                azure_core::error::ErrorKind::Credential,
                "Authentication is required. Use master_key() or credential() to set credentials.",
            )
        })?;

        Ok(AccountReference {
            endpoint: self.endpoint,
            auth,
        })
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn account_endpoint_join_path_with_leading_slash() {
        let endpoint =
            AccountEndpoint::try_from("https://myaccount.documents.azure.com:443/").unwrap();
        let url = endpoint.join_path("/dbs/mydb/colls/mycoll");
        assert_eq!(url.path(), "/dbs/mydb/colls/mycoll");
        assert_eq!(url.host_str(), Some("myaccount.documents.azure.com"));
    }

    #[test]
    fn account_endpoint_join_path_without_leading_slash() {
        let endpoint =
            AccountEndpoint::try_from("https://myaccount.documents.azure.com:443/").unwrap();
        let url = endpoint.join_path("dbs/mydb/colls/mycoll");
        assert_eq!(url.path(), "/dbs/mydb/colls/mycoll");
    }

    #[test]
    fn account_endpoint_join_path_empty() {
        let endpoint =
            AccountEndpoint::try_from("https://myaccount.documents.azure.com:443/").unwrap();
        let url = endpoint.join_path("");
        // Empty path is normalized to "/" by the URL library
        assert_eq!(url.path(), "/");
    }

    #[test]
    fn account_endpoint_host() {
        let endpoint =
            AccountEndpoint::try_from("https://myaccount.documents.azure.com:443/").unwrap();
        assert_eq!(endpoint.host(), "myaccount.documents.azure.com");
    }

    #[test]
    fn builder_with_master_key() {
        let account =
            AccountReference::builder(Url::parse("https://test.documents.azure.com:443/").unwrap())
                .master_key("my-secret-key")
                .build()
                .unwrap();

        match account.auth() {
            AuthOptions::MasterKey(key) => assert_eq!(key.secret(), "my-secret-key"),
            _ => panic!("Expected MasterKey auth"),
        }
    }

    #[test]
    fn builder_requires_auth() {
        let result =
            AccountReference::builder(Url::parse("https://test.documents.azure.com:443/").unwrap())
                .build();

        assert!(result.is_err());
    }

    #[test]
    fn builder_endpoint_setter_uses_url() {
        let account = AccountReference::builder(
            Url::parse("https://initial.documents.azure.com:443/").unwrap(),
        )
        .endpoint(Url::parse("https://override.documents.azure.com:443/").unwrap())
        .master_key("my-secret-key")
        .build()
        .unwrap();

        assert_eq!(
            account.endpoint().as_str(),
            "https://override.documents.azure.com/"
        );
    }

    #[test]
    fn shorthand_with_master_key() {
        let account = AccountReference::with_master_key(
            Url::parse("https://test.documents.azure.com:443/").unwrap(),
            "my-secret-key",
        );

        match account.auth() {
            AuthOptions::MasterKey(key) => assert_eq!(key.secret(), "my-secret-key"),
            _ => panic!("Expected MasterKey auth"),
        }
    }

    #[test]
    fn account_reference_equality_ignores_auth() {
        let account1 = AccountReference::with_master_key(
            Url::parse("https://test.documents.azure.com:443/").unwrap(),
            "key1",
        );

        let account2 = AccountReference::with_master_key(
            Url::parse("https://test.documents.azure.com:443/").unwrap(),
            "key2",
        );

        // Same endpoint, different keys - should be equal
        assert_eq!(account1, account2);
    }
}

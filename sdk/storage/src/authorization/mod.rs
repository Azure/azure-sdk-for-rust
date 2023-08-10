mod authorization_policy;

use azure_core::{
    auth::TokenCredential,
    error::{ErrorKind, ResultExt},
};
use std::sync::Arc;

pub(crate) use authorization_policy::AuthorizationPolicy;

/// Credentials for accessing a storage account.
///
/// # Example
///
/// The best way to create `StorageCredentials` is through use of one of the helper functions.
///
/// For example, to use an account name and access key:
/// ```rust
/// azure_storage::StorageCredentials::access_key("my_account", "SOMEACCESSKEY");
/// ```
#[derive(Clone)]
pub enum StorageCredentials {
    Key(String, String),
    SASToken(Vec<(String, String)>),
    BearerToken(String),
    TokenCredential(Arc<dyn TokenCredential>),
    Anonymous,
}

impl StorageCredentials {
    /// Create an Access Key based credential
    ///
    /// When you create a storage account, Azure generates two 512-bit storage
    /// account access keys for that account. These keys can be used to
    /// authorize access to data in your storage account via Shared Key
    /// authorization.
    ///
    /// ref: <https://docs.microsoft.com/azure/storage/common/storage-account-keys-manage>
    pub fn access_key<A, K>(account: A, key: K) -> Self
    where
        A: Into<String>,
        K: Into<String>,
    {
        Self::Key(account.into(), key.into())
    }

    /// Create a Shared Access Signature (SAS) token based credential
    ///
    /// SAS tokens are HTTP query strings that provide delegated access to
    /// resources in a storage account with granular control over how the client
    /// can access data in the account.
    ///
    /// * ref: [Grant limited access to Azure Storage resources using shared access signatures (SAS)](https://docs.microsoft.com/azure/storage/common/storage-sas-overview)
    /// * ref: [Create SAS tokens for storage containers](https://docs.microsoft.com/azure/applied-ai-services/form-recognizer/create-sas-tokens)
    pub fn sas_token<S>(token: S) -> azure_core::Result<Self>
    where
        S: AsRef<str>,
    {
        let params = get_sas_token_parms(token.as_ref())?;
        Ok(Self::SASToken(params))
    }

    /// Create an Bearer Token based credential
    ///
    /// Azure Storage accepts OAuth 2.0 access tokens from the Azure AD tenant
    /// associated with the subscription that contains the storage account.
    ///
    /// While `StorageCredentials::TokenCredential` is the preferred way to
    /// manage access tokens, this method is provided for manual management of
    /// Oauth2 tokens.
    ///
    /// ref: <https://docs.microsoft.com/rest/api/storageservices/authorize-with-azure-active-directory>
    pub fn bearer_token<T>(token: T) -> Self
    where
        T: Into<String>,
    {
        Self::BearerToken(token.into())
    }

    /// Create a `TokenCredential` based credential
    ///
    /// Azure Storage accepts OAuth 2.0 access tokens from the Azure AD tenant
    /// associated with the subscription that contains the storage account.
    ///
    /// Token Credentials can be created and automatically updated using
    /// `azure_identity`.
    ///
    /// ```
    /// use azure_identity::DefaultAzureCredential;
    /// use azure_storage::prelude::*;
    /// use std::sync::Arc;
    /// let token_credential = Arc::new(DefaultAzureCredential::default());
    /// let storage_credentials = StorageCredentials::token_credential(token_credential);
    /// ```
    ///
    /// ref: <https://docs.microsoft.com/rest/api/storageservices/authorize-with-azure-active-directory>
    pub fn token_credential(credential: Arc<dyn TokenCredential>) -> Self {
        Self::TokenCredential(credential)
    }

    /// Create an anonymous credential
    ///
    /// Azure Storage supports optional anonymous public read access for
    /// containers and blobs. By default, anonymous access to data in a storage
    /// account data is not permitted. Unless anonymous access is explicitly
    /// enabled, all requests to a container and its blobs must be authorized.
    /// When a container's public access level setting is configured to permit
    /// anonymous access, clients can read data in that container without
    /// authorizing the request.
    ///
    /// ref: <https://docs.microsoft.com/azure/storage/blobs/anonymous-read-access-configure>
    pub fn anonymous() -> Self {
        Self::Anonymous
    }
}

impl std::fmt::Debug for StorageCredentials {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match &self {
            StorageCredentials::Key(_, _) => f
                .debug_struct("StorageCredentials")
                .field("credential", &"Key")
                .finish(),
            StorageCredentials::SASToken(_) => f
                .debug_struct("StorageCredentials")
                .field("credential", &"SASToken")
                .finish(),
            StorageCredentials::BearerToken(_) => f
                .debug_struct("StorageCredentials")
                .field("credential", &"BearerToken")
                .finish(),
            StorageCredentials::TokenCredential(_) => f
                .debug_struct("StorageCredentials")
                .field("credential", &"TokenCredential")
                .finish(),
            StorageCredentials::Anonymous => f
                .debug_struct("StorageCredentials")
                .field("credential", &"Anonymous")
                .finish(),
        }
    }
}

impl From<Arc<dyn TokenCredential>> for StorageCredentials {
    fn from(cred: Arc<dyn TokenCredential>) -> Self {
        Self::TokenCredential(cred)
    }
}

fn get_sas_token_parms(sas_token: &str) -> azure_core::Result<Vec<(String, String)>> {
    // Any base url will do: we just need to parse the SAS token
    // to get its query pairs.
    let base_url = url::Url::parse("https://blob.core.windows.net").unwrap();

    let url = url::Url::options().base_url(Some(&base_url));

    // this code handles the leading ?
    // we support both with or without
    let url = if sas_token.starts_with('?') {
        url.parse(sas_token)
    } else {
        url.parse(&format!("?{sas_token}"))
    }
    .with_context(ErrorKind::DataConversion, || {
        format!("failed to parse SAS token: {sas_token}")
    })?;

    Ok(url
        .query_pairs()
        .map(|p| (String::from(p.0), String::from(p.1)))
        .collect())
}

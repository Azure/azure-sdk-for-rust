mod authorization_policy;

pub(crate) use self::authorization_policy::AuthorizationPolicy;
use crate::clients::{EMULATOR_ACCOUNT, EMULATOR_ACCOUNT_KEY};
use azure_core::{
    auth::TokenCredential,
    error::{ErrorKind, ResultExt},
};
use futures::lock::Mutex;
use std::{
    mem::replace,
    ops::{Deref, DerefMut},
    sync::Arc,
};
use url::Url;

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
pub struct StorageCredentials(pub Arc<Mutex<StorageCredentialsInner>>);

#[derive(Clone)]
pub enum StorageCredentialsInner {
    Key(String, String),
    SASToken(Vec<(String, String)>),
    BearerToken(String),
    TokenCredential(Arc<dyn TokenCredential>),
    Anonymous,
}

impl StorageCredentials {
    /// Create a new `StorageCredentials` from a `StorageCredentialsInner`
    fn wrap(inner: StorageCredentialsInner) -> Self {
        Self(Arc::new(Mutex::new(inner)))
    }

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
        Self::wrap(StorageCredentialsInner::Key(account.into(), key.into()))
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
        Ok(Self::wrap(StorageCredentialsInner::SASToken(params)))
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
        Self::wrap(StorageCredentialsInner::BearerToken(token.into()))
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
        Self::wrap(StorageCredentialsInner::TokenCredential(credential))
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
        Self::wrap(StorageCredentialsInner::Anonymous)
    }

    /// Create an Access Key credential for use with the Azure Storage emulator
    pub fn emulator() -> Self {
        Self::access_key(EMULATOR_ACCOUNT, EMULATOR_ACCOUNT_KEY)
    }

    /// Replace the current credentials with new credentials
    ///
    /// This method is useful for updating credentials that are used by multiple
    /// clients at once.
    pub async fn replace(&self, other: Self) -> azure_core::Result<()> {
        if Arc::ptr_eq(&self.0, &other.0) {
            return Ok(());
        }

        let mut creds = self.0.lock().await;
        let other = other.0.lock().await;
        let creds = creds.deref_mut();
        let other = other.deref().clone();
        let _old_creds = replace(creds, other);

        Ok(())
    }
}

impl std::fmt::Debug for StorageCredentials {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let creds = self.0.try_lock();

        match creds.as_deref() {
            None => f
                .debug_struct("StorageCredentials")
                .field("credential", &"locked")
                .finish(),
            Some(inner) => match &inner {
                StorageCredentialsInner::Key(_, _) => f
                    .debug_struct("StorageCredentials")
                    .field("credential", &"Key")
                    .finish(),
                StorageCredentialsInner::SASToken(_) => f
                    .debug_struct("StorageCredentials")
                    .field("credential", &"SASToken")
                    .finish(),
                StorageCredentialsInner::BearerToken(_) => f
                    .debug_struct("StorageCredentials")
                    .field("credential", &"BearerToken")
                    .finish(),
                StorageCredentialsInner::TokenCredential(_) => f
                    .debug_struct("StorageCredentials")
                    .field("credential", &"TokenCredential")
                    .finish(),
                StorageCredentialsInner::Anonymous => f
                    .debug_struct("StorageCredentials")
                    .field("credential", &"Anonymous")
                    .finish(),
            },
        }
    }
}

impl From<Arc<dyn TokenCredential>> for StorageCredentials {
    fn from(cred: Arc<dyn TokenCredential>) -> Self {
        Self::token_credential(cred)
    }
}

impl TryFrom<&Url> for StorageCredentials {
    type Error = azure_core::Error;
    fn try_from(value: &Url) -> Result<Self, Self::Error> {
        match value.query() {
            Some(query) => Self::sas_token(query),
            None => Ok(Self::anonymous()),
        }
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

#[cfg(test)]
mod tests {
    use super::*;

    #[tokio::test]
    async fn test_replacement() -> azure_core::Result<()> {
        let base = StorageCredentials::anonymous();
        let other = StorageCredentials::bearer_token("foo");

        base.replace(other).await?;

        // check that the value was updated
        {
            let inner = base.0.lock().await;
            let inner_locked = inner.deref();
            assert!(
                matches!(&inner_locked, &StorageCredentialsInner::BearerToken(value) if value == "foo")
            );
        }

        // updating with the same StorageCredentials shouldn't deadlock
        base.replace(base.clone()).await?;

        Ok(())
    }
}

use std::sync::Arc;

use azure_core::credentials::{TokenCredential, TokenRequestOptions};
use base64::{engine::general_purpose::STANDARD, Engine};
use hmac::{Hmac, Mac};
use serde::Deserialize;
use sha2::Sha256;
use time::{macros::format_description, Duration, OffsetDateTime};
use url::Url;
use uuid::Uuid;

use crate::error::SasError;

pub(crate) fn format_sas_time(dt: OffsetDateTime) -> Result<String, time::error::Format> {
    dt.format(format_description!(
        "[year]-[month]-[day]T[hour]:[minute]:[second]Z"
    ))
}

/// The user delegation key returned by Azure Storage.
///
/// Keys are valid for up to [`crate::UserDelegationSasBuilder::MAX_KEY_EXPIRY`] and can be reused to sign multiple SAS tokens within
/// that window. Obtain one via [`UserDelegationKeyFetcher::fetch`] or [`crate::UserDelegationSasBuilder::fetch_key`] and pass it to
/// subsequent builders with [`crate::UserDelegationSasBuilder::with_key`].
///
/// <https://learn.microsoft.com/en-us/rest/api/storageservices/create-user-delegation-sas#request-the-user-delegation-key>
/// <https://learn.microsoft.com/en-us/rest/api/storageservices/get-user-delegation-key>
#[derive(Clone, Debug, Deserialize)]
#[serde(rename_all = "PascalCase")]
pub struct UserDelegationKey {
    /// The immutable identifier for an object in the Microsoft identity system.
    pub signed_oid: String,
    /// A GUID that represents the Microsoft Entra tenant that the user is from.
    pub signed_tid: String,
    /// The start time of the user delegation key, in ISO date format.
    pub signed_start: String,
    /// The expiration time of the user delegation key, in ISO date format.
    pub signed_expiry: String,
    /// The service that the user delegation key can be used for, where b represents Blob Storage.
    pub signed_service: String,
    /// The REST API version that's used to get the user delegation key.
    pub signed_version: String,
    /// The user delegation key.
    pub value: String,
}

impl UserDelegationKey {
    /// Fetches a user delegation key from the given storage service endpoint.
    ///
    /// `delegated_user_tid` is the tenant ID of the end user for cross-tenant scenarios.
    /// When provided it is sent as `<DelegatedUserTid>` in the request body.
    pub(crate) async fn fetch(
        endpoint: &Url,
        signed_version: &str,
        credential: &Arc<dyn TokenCredential>,
        client: &reqwest::Client,
        start_str: &str,
        expiry_str: &str,
        delegated_user_tid: Option<&str>,
    ) -> Result<Self, SasError> {
        let token = credential
            .get_token(
                &["https://storage.azure.com/.default"],
                Some(TokenRequestOptions::default()),
            )
            .await?;

        let mut key_url = endpoint.clone();
        key_url.set_query(Some("restype=service&comp=userdelegationkey"));
        let dutid_elem = delegated_user_tid
            .map(|t| format!("<DelegatedUserTid>{t}</DelegatedUserTid>"))
            .unwrap_or_default();
        let xml_body = format!(
            r#"<?xml version="1.0" encoding="utf-8"?>
                <KeyInfo>
                    <Start>{start_str}</Start>
                    <Expiry>{expiry_str}</Expiry>
                    {dutid_elem}
                </KeyInfo>
            "#
        );

        let response = client
            .post(key_url)
            .header("Authorization", format!("Bearer {}", token.token.secret()))
            .header("x-ms-version", signed_version)
            .header("Content-Type", "application/xml")
            .body(xml_body)
            .send()
            .await?;

        if !response.status().is_success() {
            return Err(SasError::DelegationKeyError {
                status: response.status().as_u16(),
                message: response.text().await?,
            });
        }

        let xml = response.text().await?;
        Ok(quick_xml::de::from_str(&xml)?)
    }

    /// Signs `string_to_sign` with HMAC-SHA256 using this key's value.
    ///
    /// <https://learn.microsoft.com/en-us/rest/api/storageservices/create-user-delegation-sas#specify-the-signature>
    pub(crate) fn compute_signature(&self, string_to_sign: &str) -> Result<String, SasError> {
        let key_bytes = STANDARD.decode(&self.value)?;
        let mut mac =
            Hmac::<Sha256>::new_from_slice(&key_bytes).map_err(|_| SasError::HmacError)?;
        mac.update(string_to_sign.as_bytes());
        Ok(STANDARD.encode(mac.finalize().into_bytes()))
    }
}

/// Fetches a [`UserDelegationKey`] independently of any [`crate::UserDelegationSasBuilder`].
///
/// Use this when you want to obtain a key once and share it across multiple builders
/// via [`crate::UserDelegationSasBuilder::with_key`], without tying the fetch to any
/// particular resource or permission set.
///
/// `GetUserDelegationKey` is a Blob Storage service operation, so the key is always fetched
/// from the blob endpoint regardless of which storage service you plan to use the SAS for.
///
/// # Example
///
/// ```ignore
/// use std::sync::Arc;
/// use azure_identity::DefaultAzureCredential;
/// use azure_storage_sas::{UserDelegationKeyFetcher, UserDelegationSasBuilder};
/// use azure_storage_sas::blob::{BlobResource, BlobSasPermissions};
///
/// let credential = Arc::new(DefaultAzureCredential::default());
/// let key = UserDelegationKeyFetcher::new("myaccount", credential)
///     .fetch()
///     .await?;
///
/// for blob in blobs {
///     let url = UserDelegationSasBuilder::new("myaccount", blob, permissions, expiry)
///         .with_key(key.clone())
///         .build()?;
/// }
/// ```
pub struct UserDelegationKeyFetcher {
    account: String,
    credential: Arc<dyn TokenCredential>,
    key_expiry: Duration,
    endpoint: Option<Url>,
    http_client: Option<reqwest::Client>,
    version: Option<String>,
    delegated_user_tenant_id: Option<Uuid>,
}

impl UserDelegationKeyFetcher {
    /// Azure-enforced upper limit on user delegation key validity.
    pub const MAX_KEY_EXPIRY: Duration = Duration::days(7);

    /// Creates a new fetcher for the given storage account.
    pub fn new(account: impl Into<String>, credential: Arc<dyn TokenCredential>) -> Self {
        Self {
            account: account.into(),
            credential,
            key_expiry: Self::MAX_KEY_EXPIRY,
            endpoint: None,
            http_client: None,
            version: None,
            delegated_user_tenant_id: None,
        }
    }

    /// Sets the validity period of the delegation key.
    ///
    /// Defaults to [`Self::MAX_KEY_EXPIRY`].
    pub fn key_expiry(mut self, duration: Duration) -> Self {
        self.key_expiry = duration;
        self
    }

    /// Overrides the blob service endpoint used to call `GetUserDelegationKey`.
    ///
    /// Defaults to `https://{account}.blob.core.windows.net`. Override for local emulators,
    /// sovereign clouds, or custom domains.
    pub fn endpoint(mut self, endpoint: Url) -> Self {
        self.endpoint = Some(endpoint);
        self
    }

    /// Provides a shared [`reqwest::Client`] to reuse an existing connection pool.
    pub fn http_client(mut self, client: reqwest::Client) -> Self {
        self.http_client = Some(client);
        self
    }

    /// Overrides the API version sent in `x-ms-version`.
    ///
    /// Defaults to [`crate::blob::BLOB_DEFAULT_VERSION`].
    pub fn api_version(mut self, version: impl Into<String>) -> Self {
        self.version = Some(version.into());
        self
    }

    /// Sets the tenant ID of the delegated user for cross-tenant scenarios.
    ///
    /// When provided, it is sent as `<DelegatedUserTid>` in the `GetUserDelegationKey`
    /// request body. Pass the same value as `delegated_user_tenant_id` on the SAS builder
    /// to include it as `skdutid` in the signed token.
    pub fn delegated_user_tenant_id(mut self, tid: Uuid) -> Self {
        self.delegated_user_tenant_id = Some(tid);
        self
    }

    /// Fetches the user delegation key from Azure Storage.
    ///
    /// # Errors
    ///
    /// - [`SasError::KeyExpiryTooLong`] — `key_expiry` exceeds [`Self::MAX_KEY_EXPIRY`].
    /// - [`SasError::TokenError`] — credential failed to produce a token.
    /// - [`SasError::DelegationKeyError`] — the storage service rejected the key request.
    /// - [`SasError::HttpError`] — a network error occurred.
    pub async fn fetch(mut self) -> Result<UserDelegationKey, SasError> {
        if self.key_expiry > Self::MAX_KEY_EXPIRY {
            return Err(SasError::KeyExpiryTooLong);
        }

        let now = OffsetDateTime::now_utc();
        let start_str = format_sas_time(now)?;
        let expiry_str = format_sas_time(now + self.key_expiry)?;

        let endpoint = self.endpoint.unwrap_or_else(|| {
            Url::parse(&format!("https://{}.blob.core.windows.net", self.account))
                .expect("valid blob endpoint URL")
        });
        let version = self
            .version
            .unwrap_or_else(|| crate::resource::blob::BLOB_DEFAULT_VERSION.to_owned());

        let delegated_user_tid_str = self.delegated_user_tenant_id.map(|u| u.to_string());
        let client = self
            .http_client
            .get_or_insert_with(reqwest::Client::default);

        UserDelegationKey::fetch(
            &endpoint,
            &version,
            &self.credential,
            client,
            &start_str,
            &expiry_str,
            delegated_user_tid_str.as_deref(),
        )
        .await
    }
}

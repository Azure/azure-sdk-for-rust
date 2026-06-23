// Copyright (c) Microsoft Corporation. All rights reserved.
// Licensed under the MIT License.

pub use crate::generated::clients::{QueueClient, QueueClientOptions};

use crate::logging::apply_storage_logging_defaults;
use azure_core::{
    credentials::TokenCredential,
    http::{
        policies::{auth::BearerTokenAuthorizationPolicy, Policy},
        Pipeline, StatusCode, Url,
    },
    tracing, Result,
};
use std::sync::Arc;

impl QueueClient {
    /// Creates a new `QueueClient` from a queue URL.
    ///
    /// # Arguments
    ///
    /// * `queue_url` - The full URL of the queue, for example `https://myaccount.queue.core.windows.net/myqueue`.
    ///   The caller is responsible for percent-encoding the URL correctly; it will be used as-is.
    /// * `credential` - An optional implementation of [`TokenCredential`] that can provide an Entra ID token to use when authenticating.
    /// * `options` - Optional configuration for the client.
    #[tracing::new("Storage.Queues.Queue")]
    pub fn new(
        queue_url: Url,
        credential: Option<Arc<dyn TokenCredential>>,
        options: Option<QueueClientOptions>,
    ) -> Result<Self> {
        // Storage endpoints must be base URLs.
        if queue_url.cannot_be_a_base() {
            return Err(azure_core::Error::with_message(
                azure_core::error::ErrorKind::Other,
                format!("{queue_url} is not a valid base URL"),
            ));
        }

        let mut options = options.unwrap_or_default();
        apply_storage_logging_defaults(&mut options.client_options);

        let mut per_retry_policies: Vec<Arc<dyn Policy>> = Vec::default();
        if let Some(token_credential) = credential {
            if !queue_url.scheme().starts_with("https") {
                return Err(azure_core::Error::with_message(
                    azure_core::error::ErrorKind::Other,
                    format!("{queue_url} must use https"),
                ));
            }
            per_retry_policies.push(Arc::new(BearerTokenAuthorizationPolicy::new(
                token_credential,
                vec!["https://storage.azure.com/.default"],
            )));
        }

        let pipeline = Pipeline::new(
            option_env!("CARGO_PKG_NAME"),
            option_env!("CARGO_PKG_VERSION"),
            options.client_options.clone(),
            Vec::default(),
            per_retry_policies,
            None,
        );

        Ok(Self {
            endpoint: queue_url,
            version: options.version,
            pipeline,
        })
    }

    /// Gets the URL of the resource this client is configured for.
    pub fn url(&self) -> &Url {
        &self.endpoint
    }

    /// Checks if the queue exists.
    ///
    /// Returns `true` if the queue exists, `false` if the queue does not exist, and propagates all other errors.
    pub async fn exists(&self) -> Result<bool> {
        match self.get_properties(None).await {
            Ok(_) => Ok(true),
            Err(e) if e.http_status() == Some(StatusCode::NotFound) => Ok(false),
            Err(e) => Err(e),
        }
    }

    /// Begins building a user delegation SAS URL for this queue.
    ///
    /// Returns a [`QueueSasBuilder`](crate::models::sas::QueueSasBuilder) pre-initialized with
    /// this queue's resource info and the requested permissions. Chain optional
    /// setters, then call [`url()`](crate::models::sas::QueueSasBuilder::url) to produce the
    /// signed URL.
    ///
    /// The returned URL can be passed directly to [`QueueClient::new`] with
    /// `None` for the credential to construct a SAS-authenticated client.
    ///
    /// # Errors
    ///
    /// Returns an error if `key` is missing any required field.
    ///
    /// # Examples
    ///
    /// Generate a SAS URL for reading and processing messages:
    ///
    /// ```no_run
    /// # use azure_storage_queue::QueueClient;
    /// # use azure_storage_queue::models::sas::{QueuePermissions, UserDelegationKey};
    /// # use time::OffsetDateTime;
    /// # fn example(client: &QueueClient, udk: UserDelegationKey) -> azure_core::Result<()> {
    /// let url = client
    ///     .user_delegation_sas(
    ///         "myaccount",
    ///         &udk,
    ///         QueuePermissions::new().read().process(),
    ///         OffsetDateTime::now_utc() + time::Duration::hours(1),
    ///     )?
    ///     .url();
    /// # Ok(()) }
    /// ```
    ///
    /// Restrict to HTTPS-only:
    ///
    /// ```no_run
    /// # use azure_storage_queue::QueueClient;
    /// # use azure_storage_queue::models::sas::{QueuePermissions, UserDelegationKey};
    /// # use azure_storage_sas::SasProtocol;
    /// # use time::OffsetDateTime;
    /// # fn example(client: &QueueClient, udk: UserDelegationKey) -> azure_core::Result<()> {
    /// let url = client
    ///     .user_delegation_sas(
    ///         "myaccount",
    ///         &udk,
    ///         QueuePermissions::new().read().process().add(),
    ///         OffsetDateTime::now_utc() + time::Duration::hours(8),
    ///     )?
    ///     .protocol(SasProtocol::Https)
    ///     .url();
    /// # Ok(()) }
    /// ```
    #[cfg(feature = "sas_builder")]
    #[cfg_attr(docsrs, doc(cfg(feature = "sas_builder")))]
    pub fn user_delegation_sas<'a>(
        &self,
        account_name: &str,
        key: &'a azure_storage_common::models::UserDelegationKey,
        permissions: azure_storage_sas::resource::QueuePermissions,
        expiry: time::OffsetDateTime,
    ) -> Result<crate::sas::QueueSasBuilder<'a>> {
        let segments: Vec<String> = self
            .endpoint
            .path_segments()
            .map(|p| {
                p.filter(|s| !s.is_empty())
                    .map(|s| {
                        percent_encoding::percent_decode_str(s)
                            .decode_utf8_lossy()
                            .into_owned()
                    })
                    .collect()
            })
            .unwrap_or_default();

        // Emulator and path-style endpoints (e.g. Azurite) encode the account
        // name as the first path segment rather than as a subdomain of the host.
        // Detect this layout and strip the leading segment so that the remainder
        // is always [queue].
        let is_path_prefix = self
            .endpoint
            .host_str()
            .map_or(false, |h| !h.starts_with(account_name))
            && segments.first().map(String::as_str) == Some(account_name);
        let segments: &[String] = if is_path_prefix {
            &segments[1..]
        } else {
            &segments
        };

        if segments.len() != 1 {
            return Err(azure_core::Error::with_message(
                azure_core::error::ErrorKind::Other,
                "queue endpoint URL must include exactly one path segment",
            ));
        }

        let queue = azure_storage_sas::resource::QueueResource::new(&segments[0]);
        let inner = azure_storage_sas::SasBuilder::new(account_name, key, expiry)?
            .queue(queue, permissions);
        Ok(crate::sas::QueueSasBuilder::new(
            self.endpoint.clone(),
            inner,
        ))
    }
}

#[cfg(test)]
mod tests {
    use super::{QueueClient, QueueClientOptions, Url};
    use azure_core_test::credentials::MockCredential;

    #[test]
    fn new_requires_https_with_credential() {
        let cred = MockCredential::new().unwrap();
        let url = Url::parse("http://myaccount.queue.core.windows.net/myqueue").unwrap();
        let err = QueueClient::new(url, Some(cred), None).err().unwrap();
        assert!(
            err.to_string().contains("must use https"),
            "Expected error message to contain 'must use https', got: {err}"
        );
    }

    #[test]
    fn new_rejects_non_base_url() {
        let url = Url::parse("data:text/plain,hello").unwrap();
        assert!(QueueClient::new(url, None, None).is_err());
    }

    #[test]
    fn new_allows_http_without_credential() {
        // HTTP is allowed when no credential is provided (e.g., SAS token in URL)
        let url = Url::parse("http://myaccount.queue.core.windows.net/myqueue").unwrap();
        assert!(QueueClient::new(url, None, None).is_ok());
    }

    #[test]
    fn new_allows_https_with_credential() {
        let cred = MockCredential::new().unwrap();
        let url = Url::parse("https://myaccount.queue.core.windows.net/myqueue").unwrap();
        let result = QueueClient::new(url, Some(cred), Some(QueueClientOptions::default()));
        assert!(result.is_ok());
    }

    #[cfg(feature = "sas_builder")]
    #[test]
    fn user_delegation_sas_attaches_sas_query() {
        use crate::models::sas::{QueuePermissions, UserDelegationKey};
        use time::macros::datetime;

        let url = Url::parse("https://acct.queue.core.windows.net/q1").unwrap();
        let client = QueueClient::new(url, None, None).unwrap();
        let udk = UserDelegationKey {
            signed_delegated_user_tid: None,
            signed_oid: Some("oid".into()),
            signed_tid: Some("tid".into()),
            signed_start: Some(datetime!(2025-01-15 00:00:00 UTC)),
            signed_expiry: Some(datetime!(2025-01-16 00:00:00 UTC)),
            signed_service: Some("b".into()),
            signed_version: Some("2025-11-05".into()),
            value: Some(b"testkey".to_vec()),
        };

        let sas_url = client
            .user_delegation_sas(
                "acct",
                &udk,
                QueuePermissions::new().read().add(),
                datetime!(2025-06-01 12:00:00 UTC),
            )
            .unwrap()
            .url();

        let query = sas_url.query().unwrap();
        assert!(query.contains("sp=ra"), "got: {query}");
        assert!(query.contains("sig="), "got: {query}");
    }

    #[cfg(feature = "sas_builder")]
    #[test]
    fn sas_url_path_prefix_endpoint_strips_account_segment() {
        use crate::models::sas::{QueuePermissions, UserDelegationKey};
        use time::macros::datetime;

        let url = Url::parse("http://127.0.0.1:10001/devstoreaccount1/myqueue").unwrap();
        let client = QueueClient::new(url, None, None).unwrap();
        let udk = UserDelegationKey {
            signed_delegated_user_tid: None,
            signed_oid: Some("oid".into()),
            signed_tid: Some("tid".into()),
            signed_start: Some(datetime!(2025-01-15 00:00:00 UTC)),
            signed_expiry: Some(datetime!(2025-01-16 00:00:00 UTC)),
            signed_service: Some("b".into()),
            signed_version: Some("2025-11-05".into()),
            value: Some(b"testkey".to_vec()),
        };

        let sas_url = client
            .user_delegation_sas(
                "devstoreaccount1",
                &udk,
                QueuePermissions::new().read().process(),
                datetime!(2025-06-01 12:00:00 UTC),
            )
            .unwrap()
            .url();

        assert_eq!(sas_url.path(), "/devstoreaccount1/myqueue");
        let query = sas_url.query().unwrap();
        assert!(query.contains("sig="), "got: {query}");
    }

    #[cfg(feature = "sas_builder")]
    #[test]
    fn sas_url_queue_named_same_as_account_no_false_skip() {
        use crate::models::sas::{QueuePermissions, UserDelegationKey};
        use time::macros::datetime;

        // Queue has the same name as the account — must NOT be skipped.
        let url = Url::parse("https://acct.queue.core.windows.net/acct").unwrap();
        let client = QueueClient::new(url, None, None).unwrap();
        let udk = UserDelegationKey {
            signed_delegated_user_tid: None,
            signed_oid: Some("oid".into()),
            signed_tid: Some("tid".into()),
            signed_start: Some(datetime!(2025-01-15 00:00:00 UTC)),
            signed_expiry: Some(datetime!(2025-01-16 00:00:00 UTC)),
            signed_service: Some("b".into()),
            signed_version: Some("2025-11-05".into()),
            value: Some(b"testkey".to_vec()),
        };

        let sas_url = client
            .user_delegation_sas(
                "acct",
                &udk,
                QueuePermissions::new().read(),
                datetime!(2025-06-01 12:00:00 UTC),
            )
            .unwrap()
            .url();

        let query = sas_url.query().unwrap();
        assert!(query.contains("sig="), "got: {query}");
    }
}

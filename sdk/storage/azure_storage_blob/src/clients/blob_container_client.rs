// Copyright (c) Microsoft Corporation. All rights reserved.
// Licensed under the MIT License.

pub use crate::generated::clients::{BlobContainerClient, BlobContainerClientOptions};

use crate::{models::StorageErrorCode, BlobClient};
use azure_core::{
    credentials::TokenCredential,
    error::ErrorKind,
    http::{
        policies::{auth::BearerTokenAuthorizationPolicy, Policy},
        Pipeline, StatusCode, Url,
    },
    tracing, Result,
};
use std::sync::Arc;

impl BlobContainerClient {
    /// Creates a new BlobContainerClient from a container URL.
    ///
    /// # Arguments
    ///
    /// * `container_url` - The full URL of the container, for example `https://myaccount.blob.core.windows.net/mycontainer`.
    ///   The caller is responsible for percent-encoding the URL correctly; it will be used as-is.
    /// * `credential` - An optional implementation of [`TokenCredential`] that can provide an Entra ID token to use when authenticating.
    /// * `options` - Optional configuration for the client.
    #[tracing::new("Storage.Blob.Container")]
    pub fn new(
        container_url: Url,
        credential: Option<Arc<dyn TokenCredential>>,
        options: Option<BlobContainerClientOptions>,
    ) -> Result<Self> {
        // Storage endpoints must be base URLs.
        if container_url.cannot_be_a_base() {
            return Err(azure_core::Error::with_message(
                azure_core::error::ErrorKind::Other,
                format!("{container_url} is not a valid base URL"),
            ));
        }

        let mut options = options.unwrap_or_default();
        super::apply_client_defaults(&mut options.client_options);

        let mut per_retry_policies: Vec<Arc<dyn Policy>> = Vec::default();
        if let Some(token_credential) = credential {
            if !container_url.scheme().starts_with("https") {
                return Err(azure_core::Error::with_message(
                    azure_core::error::ErrorKind::Other,
                    format!("{container_url} must use https"),
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
            endpoint: container_url,
            version: options.version,
            pipeline,
        })
    }

    /// Returns a new instance of BlobClient.
    ///
    /// # Arguments
    ///
    /// * `blob_name` - The name of the blob.
    pub fn blob_client(&self, blob_name: &str) -> BlobClient {
        let mut blob_url = self.url().clone();
        blob_url
            .path_segments_mut()
            // This should not fail as container URL has already been validated on client construction.
            .expect("Invalid endpoint URL: Cannot append blob_name to the blob endpoint.")
            .extend([blob_name]);

        BlobClient {
            endpoint: blob_url,
            pipeline: self.pipeline.clone(),
            version: self.version.clone(),
            tracer: self.tracer.clone(),
        }
    }

    /// Gets the URL of the container.
    pub fn url(&self) -> &Url {
        &self.endpoint
    }

    /// Checks if the container exists.
    ///
    /// Returns `true` if the container exists, `false` if the container does not exist, and propagates all other errors.
    pub async fn exists(&self) -> Result<bool> {
        match self.get_properties(None).await {
            Ok(_) => Ok(true),
            Err(e) if e.http_status() == Some(StatusCode::NotFound) => match e.kind() {
                ErrorKind::HttpResponse {
                    error_code: Some(error_code),
                    ..
                } if error_code == StorageErrorCode::ContainerNotFound.as_ref() => Ok(false),
                // Propagate all other error types.
                _ => Err(e),
            },
            Err(e) => Err(e),
        }
    }

    /// Generates a user delegation SAS URL for this container.
    ///
    /// The `configure` closure receives a [`SasBuilder`](azure_storage_sas::SasBuilder)
    /// pre-initialized with this container's resource info and the requested
    /// permissions.
    ///
    /// The returned URL can be passed directly to [`BlobContainerClient::new`]
    /// with `None` for the credential to construct a SAS-authenticated client.
    ///
    /// To generate a SAS for a single blob in the container, use
    /// [`BlobContainerClient::blob_client`] to get a [`BlobClient`] and call
    /// [`BlobClient::generate_user_delegation_sas_url`](crate::BlobClient::generate_user_delegation_sas_url)
    /// on that.
    ///
    /// # Errors
    ///
    /// Returns an error if `key` is missing any required field.
    ///
    /// # Examples
    ///
    /// Generate a read + list SAS URL with default settings:
    ///
    /// ```no_run
    /// # use azure_storage_blob::BlobContainerClient;
    /// # use azure_storage_blob::models::sas::{ContainerPermissions, UserDelegationKey};
    /// # use time::OffsetDateTime;
    /// # fn example(client: &BlobContainerClient, udk: UserDelegationKey) -> azure_core::Result<()> {
    /// let url = client.generate_user_delegation_sas_url(
    ///     "myaccount",
    ///     &udk,
    ///     ContainerPermissions::new().read().list(),
    ///     OffsetDateTime::now_utc() + time::Duration::hours(1),
    ///     |sas| sas, // no customization needed
    /// )?;
    /// # Ok(()) }
    /// ```
    ///
    /// Restrict to HTTPS and add an IP range:
    ///
    /// ```no_run
    /// # use azure_storage_blob::BlobContainerClient;
    /// # use azure_storage_blob::models::sas::{ContainerPermissions, UserDelegationKey};
    /// # use azure_storage_sas::{SasIpRange, SasProtocol};
    /// # use std::net::Ipv4Addr;
    /// # use time::OffsetDateTime;
    /// # fn example(client: &BlobContainerClient, udk: UserDelegationKey) -> azure_core::Result<()> {
    /// let url = client.generate_user_delegation_sas_url(
    ///     "myaccount",
    ///     &udk,
    ///     ContainerPermissions::new().read().list().write(),
    ///     OffsetDateTime::now_utc() + time::Duration::hours(8),
    ///     |sas| {
    ///         sas.protocol(SasProtocol::Https)
    ///             .ip_range(SasIpRange::Range {
    ///                 start: Ipv4Addr::new(10, 0, 0, 1).into(),
    ///                 end: Ipv4Addr::new(10, 0, 0, 255).into(),
    ///             })
    ///     },
    /// )?;
    /// # Ok(()) }
    /// ```
    #[cfg(feature = "sas_builder")]
    #[cfg_attr(docsrs, doc(cfg(feature = "sas_builder")))]
    pub fn generate_user_delegation_sas_url<F>(
        &self,
        account_name: &str,
        key: &azure_storage_common::models::UserDelegationKey,
        permissions: azure_storage_sas::resource::blob::ContainerPermissions,
        expiry: time::OffsetDateTime,
        configure: F,
    ) -> Result<Url>
    where
        F: FnOnce(
            azure_storage_sas::SasBuilder<'_, azure_storage_sas::state::ContainerState>,
        )
            -> azure_storage_sas::SasBuilder<'_, azure_storage_sas::state::ContainerState>,
    {
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
        // is always [container].
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
                "container endpoint URL must include exactly one path segment",
            ));
        }

        let container = azure_storage_sas::resource::blob::Container::new(&segments[0]);
        let builder = azure_storage_sas::SasBuilder::new(account_name, key, expiry)?
            .container(container, permissions);
        let token = configure(builder).build();
        Ok(crate::sas_helpers::append_query(&self.endpoint, &token))
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn from_url_rejects_cannot_be_a_base_url() {
        let url = Url::parse("data:text/plain,hello").unwrap();
        assert!(BlobContainerClient::new(url, None, None).is_err());
    }

    #[test]
    fn from_url_accepts_http_without_credential() {
        let url = Url::parse("http://127.0.0.1:10000/devstoreaccount1/container").unwrap();
        let container = BlobContainerClient::new(url, None, None).unwrap();
        assert_eq!(
            container.blob_client("blob").url().path(),
            "/devstoreaccount1/container/blob"
        );
    }

    #[test]
    fn from_url_accepts_https_custom_hostname() {
        // CDN / Front Door / private endpoint hostnames are still https URLs.
        let url = Url::parse("https://cdn.contoso.com/container").unwrap();
        assert!(BlobContainerClient::new(url, None, None).is_ok());
    }

    #[cfg(feature = "sas_builder")]
    #[test]
    fn generate_user_delegation_sas_url_attaches_sas_query() {
        use crate::models::sas::{ContainerPermissions, UserDelegationKey};
        use time::macros::datetime;

        let url = Url::parse("https://acct.blob.core.windows.net/c1").unwrap();
        let client = BlobContainerClient::new(url, None, None).unwrap();
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
            .generate_user_delegation_sas_url(
                "acct",
                &udk,
                ContainerPermissions::new().read(),
                datetime!(2025-06-01 12:00:00 UTC),
                |sas| sas,
            )
            .unwrap();

        let query = sas_url.query().unwrap();
        assert!(query.contains("sr=c"), "got: {query}");
        assert!(query.contains("sig="), "got: {query}");
    }

    #[cfg(feature = "sas_builder")]
    #[test]
    fn sas_url_path_prefix_endpoint_strips_account_segment() {
        use crate::models::sas::{ContainerPermissions, UserDelegationKey};
        use time::macros::datetime;

        let url = Url::parse("http://127.0.0.1:10000/devstoreaccount1/mycontainer").unwrap();
        let client = BlobContainerClient::new(url, None, None).unwrap();
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
            .generate_user_delegation_sas_url(
                "devstoreaccount1",
                &udk,
                ContainerPermissions::new().read().list(),
                datetime!(2025-06-01 12:00:00 UTC),
                |sas| sas,
            )
            .unwrap();

        assert_eq!(sas_url.path(), "/devstoreaccount1/mycontainer");
        let query = sas_url.query().unwrap();
        assert!(query.contains("sr=c"), "got: {query}");
    }

    #[cfg(feature = "sas_builder")]
    #[test]
    fn sas_url_container_named_same_as_account_no_false_skip() {
        use crate::models::sas::{ContainerPermissions, UserDelegationKey};
        use time::macros::datetime;

        // Container has the same name as the account — must NOT be skipped.
        let url = Url::parse("https://acct.blob.core.windows.net/acct").unwrap();
        let client = BlobContainerClient::new(url, None, None).unwrap();
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
            .generate_user_delegation_sas_url(
                "acct",
                &udk,
                ContainerPermissions::new().read(),
                datetime!(2025-06-01 12:00:00 UTC),
                |sas| sas,
            )
            .unwrap();

        let query = sas_url.query().unwrap();
        assert!(query.contains("sr=c"), "got: {query}");
    }
}

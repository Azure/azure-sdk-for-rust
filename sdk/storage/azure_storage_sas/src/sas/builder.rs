use std::sync::Arc;

use azure_core::credentials::TokenCredential;
use time::{Duration, OffsetDateTime};
use url::Url;
use uuid::Uuid;

use crate::{
    error::SasError,
    key::{format_sas_time, UserDelegationKey},
    resource::sealed,
    sas::{SasSigningContext, SasUrlParams, SignedProtocol},
    Resource,
};

/// Generates a user delegation SAS URL for any Azure Storage resource.
///
/// The type parameter `R` is the resource struct ([`crate::blob::BlobResource`], [`crate::blob::ContainerResource`], etc.)
/// which determines the valid permission type at compile time via [`Resource::Permissions`].
/// The type parameter `K` tracks whether a delegation key is present:
///
/// # Simple usage — fetch key and build in one chain
///
/// ```ignore
/// use std::sync::Arc;
/// use time::{OffsetDateTime, Duration};
/// use azure_identity::DefaultAzureCredential;
/// use azure_storage_sas::{BlobResource, BlobSasPermissions, UserDelegationSasBuilder};
///
/// let credential = Arc::new(DefaultAzureCredential::default());
/// let expiry = OffsetDateTime::now_utc() + Duration::hours(1);
/// let url = UserDelegationSasBuilder::new(
///     "myaccount",
///     BlobResource {
///         container: "mycontainer".into(),
///         blob: "path/to/blob.txt".into(),
///         options: None,
///     },
///     BlobSasPermissions { write: true, create: true, ..Default::default() },
///     expiry,
/// )
/// .fetch_key(credential)
/// .await?
/// .build()?;
/// ```
///
/// # Reusing a key across multiple SAS tokens
///
/// ```ignore
/// // Fetch once — the key is valid for up to [`UserDelegationSasBuilder::MAX_KEY_EXPIRY`]
/// let builder = UserDelegationSasBuilder::new("myaccount", resource, permissions, expiry)
///     .fetch_key(credential)
///     .await?;
/// let key = builder.key().clone();
///
/// for blob_resource in blobs {
///     let url = UserDelegationSasBuilder::new("myaccount", blob_resource, permissions, expiry)
///         .with_key(key.clone())
///         .build()?;
/// }
/// ```
///
/// <https://learn.microsoft.com/en-us/rest/api/storageservices/create-user-delegation-sas>
pub struct UserDelegationSasBuilder<R: Resource, K = ()> {
    account: String,
    resource: R,
    permissions: R::Permissions,
    start: Option<OffsetDateTime>,
    expiry: OffsetDateTime,
    key: K,
    version: Option<String>,
    key_expiry: Duration,
    http_client: Option<reqwest::Client>,
    endpoint: Option<Url>,
    ip: Option<String>,
    protocol: SignedProtocol,
    authorized_user_object_id: Option<Uuid>,
    unauthorized_user_object_id: Option<Uuid>,
    delegated_user_object_id: Option<Uuid>,
    delegated_user_tenant_id: Option<Uuid>,
}

impl<R: Resource, K> UserDelegationSasBuilder<R, K> {
    /// Azure-enforced upper limit on user delegation key validity.
    pub const MAX_KEY_EXPIRY: Duration = Duration::days(7);

    /// Overrides the API version sent in the `sv` parameter and `x-ms-version` header.
    /// <https://learn.microsoft.com/en-us/rest/api/storageservices/create-user-delegation-sas#specify-the-signed-version-field>
    ///
    /// Defaults to the minimum required version for the chosen resource type
    /// - [`crate::blob::BLOB_DEFAULT_VERSION`]
    /// - [`crate::queue::QUEUE_MIN_VERSION`]
    /// - [`crate::table::TABLE_MIN_VERSION`]
    /// - [`crate::file::FILE_MIN_VERSION`]
    pub fn signed_version(mut self, version: impl Into<String>) -> Self {
        self.version = Some(version.into());
        self
    }

    /// Overrides the validity period of the user delegation key.
    ///
    /// Defaults to [`Self::MAX_KEY_EXPIRY`].
    pub fn key_expiry_duration(mut self, duration: Duration) -> Self {
        self.key_expiry = duration;
        self
    }

    /// Provides a shared [`reqwest::Client`] to reuse an existing connection pool.
    ///
    /// If not set, a new client is created per [`UserDelegationSasBuilder::fetch_key`] call.
    pub fn http_client(mut self, client: reqwest::Client) -> Self {
        self.http_client = Some(client);
        self
    }

    /// Overrides the storage account endpoint base URL.
    ///
    /// Defaults to the standard public endpoint for the selected service
    /// (e.g. `https://{account}.blob.core.windows.net`). Override for local emulators,
    /// sovereign clouds, or custom domains.
    pub fn endpoint(mut self, endpoint: Url) -> Self {
        self.endpoint = Some(endpoint);
        self
    }

    /// Sets the start time for the SAS token (`st`).
    ///
    /// When not set, `st` is omitted from the URL and the token is valid immediately.
    /// Set this to a time slightly in the past (e.g. `OffsetDateTime::now_utc() - Duration::minutes(5)`)
    /// to tolerate clock skew between the issuer and Azure.
    pub fn start(mut self, start: OffsetDateTime) -> Self {
        self.start = Some(start);
        self
    }

    /// Restricts the SAS token to requests from the given IP address or range.
    ///
    /// Emits `sip` in the URL and signs it at the appropriate position in the string-to-sign.
    pub fn ip(mut self, ip: impl Into<String>) -> Self {
        self.ip = Some(ip.into());
        self
    }

    /// Controls which protocols are permitted (`spr`).
    ///
    /// Defaults to [`SignedProtocol::HttpsAndHttp`].
    pub fn protocol(mut self, protocol: SignedProtocol) -> Self {
        self.protocol = protocol;
        self
    }
}

#[allow(private_bounds)]
impl<R: Resource + sealed::BlobService, K> UserDelegationSasBuilder<R, K> {
    /// Restricts the SAS token to a specific authorized user by Azure AD object ID.
    ///
    /// Emits `saoid` in the URL.
    /// Used for POSIX ACL authorization on hierarchical-namespace accounts.
    ///
    /// Only available for Blob Storage and Data Lake resources.
    ///
    /// <https://learn.microsoft.com/en-us/rest/api/storageservices/create-user-delegation-sas#specify-query-parameters-to-override-response-headers-blob-storage-and-azure-files-only>
    pub fn authorized_user_object_id(mut self, oid: Uuid) -> Self {
        self.authorized_user_object_id = Some(oid);
        self
    }

    /// Sets the unauthorized user object ID for subrequest delegation.
    ///
    /// Emits `suoid` in the URL.
    ///
    /// Only available for Blob Storage and Data Lake resources.
    ///
    /// <https://learn.microsoft.com/en-us/rest/api/storageservices/create-user-delegation-sas#specify-query-parameters-to-override-response-headers-blob-storage-and-azure-files-only>
    pub fn unauthorized_user_object_id(mut self, oid: Uuid) -> Self {
        self.unauthorized_user_object_id = Some(oid);
        self
    }
}

#[allow(private_bounds)]
impl<R: Resource + sealed::DelegatedUserService, K> UserDelegationSasBuilder<R, K> {
    /// Binds the SAS to a specific end-user by their Azure AD object ID.
    ///
    /// Emits `sduoid` in the URL. The value should be the `"oid"` claim of the end user who will
    /// present the SAS.
    ///
    /// Only available for Queue, Table, and File Storage resources.
    pub fn delegated_user_object_id(mut self, oid: Uuid) -> Self {
        self.delegated_user_object_id = Some(oid);
        self
    }

    /// Sets the tenant ID of the delegated user for cross-tenant scenarios.
    ///
    /// Emits `skdutid` in the URL. Only required when the delegated user is in a different tenant
    /// than the storage account.
    ///
    /// Only available for Queue, Table, and File Storage resources.
    pub fn delegated_user_tenant_id(mut self, tid: Uuid) -> Self {
        self.delegated_user_tenant_id = Some(tid);
        self
    }
}

impl<R: Resource> UserDelegationSasBuilder<R> {
    /// Creates a new builder for the given resource.
    ///
    /// The `permissions` argument must match the resource's associated permission type
    /// (e.g. [`crate::blob::BlobSasPermissions`] for [`crate::blob::BlobResource`]). This is enforced at compile time.
    ///
    /// Call [`Self::fetch_key`] or [`Self::with_key`] to unlock [`UserDelegationSasBuilder::build`].
    pub fn new(
        account: impl Into<String>,
        resource: R,
        permissions: R::Permissions,
        expiry: OffsetDateTime,
    ) -> Self {
        Self {
            account: account.into(),
            resource,
            permissions,
            start: None,
            expiry,
            key: (),
            version: None,
            key_expiry: Self::MAX_KEY_EXPIRY,
            http_client: None,
            endpoint: None,
            ip: None,
            protocol: SignedProtocol::default(),
            authorized_user_object_id: None,
            unauthorized_user_object_id: None,
            delegated_user_object_id: None,
            delegated_user_tenant_id: None,
        }
    }

    /// Fetches a user delegation key from Azure Storage and returns a ready-to-build builder.
    ///
    /// The key is valid for the duration set via [`Self::key_expiry_duration`] (default: [`Self::MAX_KEY_EXPIRY`]).
    /// Extract the key via [`UserDelegationSasBuilder::key`] to reuse it
    /// across multiple builders with [`Self::with_key`].
    ///
    /// # Errors
    ///
    /// - [`SasError::KeyExpiryTooLong`] — `key_expiry_duration` exceeds [`Self::MAX_KEY_EXPIRY`].
    /// - [`SasError::TokenError`] — credential failed to produce a token.
    /// - [`SasError::DelegationKeyError`] — the storage service rejected the key request.
    /// - [`SasError::HttpError`] — a network error occurred.
    pub async fn fetch_key(
        mut self,
        credential: Arc<dyn TokenCredential>,
    ) -> Result<UserDelegationSasBuilder<R, UserDelegationKey>, SasError> {
        if self.key_expiry > Self::MAX_KEY_EXPIRY {
            return Err(SasError::KeyExpiryTooLong);
        }

        let now = OffsetDateTime::now_utc();
        let key_start = format_sas_time(now)?;
        let key_expiry_str = format_sas_time(now + self.key_expiry)?;

        // Each storage service exposes its own GetUserDelegationKey endpoint.
        let key_endpoint = if let Some(ref ep) = self.endpoint {
            ep.clone()
        } else {
            self.resource.default_endpoint(&self.account)
        };
        let version = self
            .version
            .clone()
            .unwrap_or_else(|| self.resource.default_api_version().to_owned());

        let delegated_user_tid_str = self.delegated_user_tenant_id.map(|u| u.to_string());
        let client = self
            .http_client
            .get_or_insert_with(reqwest::Client::default);
        let key = UserDelegationKey::fetch(
            &key_endpoint,
            &version,
            &credential,
            client,
            &key_start,
            &key_expiry_str,
            delegated_user_tid_str.as_deref(),
        )
        .await?;
        Ok(self.with_key(key))
    }

    /// Sets a pre-fetched user delegation key and returns a ready-to-build builder.
    ///
    /// Use this to reuse an existing key.
    pub fn with_key(
        self,
        key: UserDelegationKey,
    ) -> UserDelegationSasBuilder<R, UserDelegationKey> {
        UserDelegationSasBuilder {
            account: self.account,
            resource: self.resource,
            permissions: self.permissions,
            start: self.start,
            expiry: self.expiry,
            key,
            version: self.version,
            key_expiry: self.key_expiry,
            http_client: self.http_client,
            endpoint: self.endpoint,
            ip: self.ip,
            protocol: self.protocol,
            authorized_user_object_id: self.authorized_user_object_id,
            unauthorized_user_object_id: self.unauthorized_user_object_id,
            delegated_user_object_id: self.delegated_user_object_id,
            delegated_user_tenant_id: self.delegated_user_tenant_id,
        }
    }
}

impl<R: Resource> UserDelegationSasBuilder<R, UserDelegationKey> {
    /// Returns the stored user delegation key.
    ///
    /// Clone and pass to [`UserDelegationSasBuilder::with_key`] on subsequent
    /// builders to avoid extra network round-trips.
    pub fn key(&self) -> &UserDelegationKey {
        &self.key
    }

    /// Signs the resource and returns a SAS URL.
    ///
    /// # Errors
    ///
    /// - [`SasError::TimeError`] — internal time formatting failure.
    /// - [`SasError::HmacError`] — the stored key's bytes are invalid.
    /// - [`SasError::UrlError`] — the constructed URL could not be parsed.
    pub fn build(self) -> Result<Url, SasError> {
        let key = self.key;
        let permissions_str = self.permissions.to_string();

        let start_str = self.start.map(format_sas_time).transpose()?;
        let sas_expiry_str = format_sas_time(self.expiry)?;

        let account_endpoint = if let Some(ep) = self.endpoint {
            ep
        } else {
            self.resource.default_endpoint(&self.account)
        };
        let version = self
            .version
            .unwrap_or_else(|| self.resource.default_api_version().to_owned());

        let saoid = self.authorized_user_object_id.map(|id| id.to_string());
        let suoid = self.unauthorized_user_object_id.map(|id| id.to_string());
        let sduoid = self.delegated_user_object_id.map(|id| id.to_string());
        let skdutid = self.delegated_user_tenant_id.map(|id| id.to_string());

        let canon = self.resource.canonicalized_resource(&self.account);
        let ctx = SasSigningContext {
            permissions: &permissions_str,
            start: start_str.as_deref().unwrap_or(""),
            expiry: &sas_expiry_str,
            canon: &canon,
            key: &key,
            version: &version,
            ip: self.ip.as_deref(),
            protocol: self.protocol,
            authorized_user_object_id: saoid.as_deref(),
            unauthorized_user_object_id: suoid.as_deref(),
            delegated_user_object_id: sduoid.as_deref(),
            delegated_user_tenant_id: skdutid.as_deref(),
        };
        let s2s = self.resource.string_to_sign(&ctx);
        let sig = key.compute_signature(&s2s)?;

        self.resource.sas_url(
            &account_endpoint,
            &SasUrlParams {
                permissions: &permissions_str,
                start: start_str.as_deref(),
                expiry: &sas_expiry_str,
                key: &key,
                signature: &sig,
                version: &version,
                ip: self.ip.as_deref(),
                protocol: self.protocol,
                authorized_user_object_id: saoid.as_deref(),
                unauthorized_user_object_id: suoid.as_deref(),
                delegated_user_object_id: sduoid.as_deref(),
                delegated_user_tenant_id: skdutid.as_deref(),
            },
        )
    }
}

#[cfg(test)]
mod tests {
    use time::macros::datetime;
    use url::Url;
    use uuid::Uuid;

    use super::UserDelegationSasBuilder;
    use crate::{
        blob::{
            container::{ContainerResource, ContainerSasPermissions},
            directory::{DirectoryResource, DirectorySasPermissions},
            snapshot::BlobSnapshotResource,
            version::BlobVersionResource,
            BlobResource, BlobResourceOptions, BlobSasPermissions,
        },
        file::{
            share::{ShareResource, ShareSasPermissions},
            FileResource, FileSasPermissions,
        },
        queue::{QueueResource, QueueSasPermissions},
        table::{TableResource, TableResourceOptions, TableSasPermissions},
        test_utils::{make_key, url_params},
    };

    // Far-future expiry so `build()` never rejects it as in the past.
    const EXPIRY: time::OffsetDateTime = datetime!(2099-12-31 23:59:59 UTC);

    #[test]
    fn blob_resource() {
        let url = UserDelegationSasBuilder::new(
            "myaccount",
            BlobResource {
                container: "mycontainer".into(),
                blob: "path/to/blob.txt".into(),
                options: None,
            },
            BlobSasPermissions {
                read: true,
                ..Default::default()
            },
            EXPIRY,
        )
        .with_key(make_key())
        .build()
        .unwrap();

        assert_eq!(url.host_str(), Some("myaccount.blob.core.windows.net"));
        assert!(url.path().ends_with("/mycontainer/path/to/blob.txt"));
        let p = url_params(&url);
        assert_eq!(p["sr"], "b");
        assert_eq!(p["sp"], "r");
        assert_eq!(p["spr"], "https,http");
        assert!(!p["sig"].is_empty());
    }

    #[test]
    fn blob_resource_with_options() {
        let oid = Uuid::new_v4();
        let url = UserDelegationSasBuilder::new(
            "myaccount",
            BlobResource {
                container: "mycontainer".into(),
                blob: "data.csv".into(),
                options: Some(BlobResourceOptions {
                    content_type: Some("text/csv".into()),
                    cache_control: Some("no-cache".into()),
                    ..Default::default()
                }),
            },
            BlobSasPermissions {
                read: true,
                write: true,
                ..Default::default()
            },
            EXPIRY,
        )
        .ip("10.0.0.1")
        .authorized_user_object_id(oid)
        .with_key(make_key())
        .build()
        .unwrap();

        let p = url_params(&url);
        assert_eq!(p["sr"], "b");
        assert_eq!(p["sp"], "rw");
        assert_eq!(p["sip"], "10.0.0.1");
        assert_eq!(p["saoid"], oid.to_string());
        assert_eq!(p["rsct"], "text/csv");
        assert_eq!(p["rscc"], "no-cache");
    }

    #[test]
    fn blob_resource_unauthorized_user() {
        // suoid is independently settable from saoid.
        let suoid = Uuid::new_v4();
        let url = UserDelegationSasBuilder::new(
            "myaccount",
            BlobResource {
                container: "mycontainer".into(),
                blob: "blob.txt".into(),
                options: None,
            },
            BlobSasPermissions {
                read: true,
                ..Default::default()
            },
            EXPIRY,
        )
        .unauthorized_user_object_id(suoid)
        .with_key(make_key())
        .build()
        .unwrap();

        let p = url_params(&url);
        assert_eq!(p["suoid"], suoid.to_string());
        assert!(!p.contains_key("saoid"));
    }

    #[test]
    fn container_resource() {
        let url = UserDelegationSasBuilder::new(
            "myaccount",
            ContainerResource {
                container: "mycontainer".into(),
                options: None,
            },
            ContainerSasPermissions {
                read: true,
                list: true,
                ..Default::default()
            },
            EXPIRY,
        )
        .with_key(make_key())
        .build()
        .unwrap();

        assert_eq!(url.host_str(), Some("myaccount.blob.core.windows.net"));
        assert!(url.path().ends_with("/mycontainer"));
        let p = url_params(&url);
        assert_eq!(p["sr"], "c");
        assert_eq!(p["sp"], "rl");
        assert!(!p["sig"].is_empty());
    }

    #[test]
    fn directory_resource() {
        let url = UserDelegationSasBuilder::new(
            "myaccount",
            DirectoryResource {
                container: "mycontainer".into(),
                path: "logs/2024/01".into(),
                options: None,
            },
            DirectorySasPermissions {
                read: true,
                list: true,
                ..Default::default()
            },
            EXPIRY,
        )
        .with_key(make_key())
        .build()
        .unwrap();

        assert_eq!(url.host_str(), Some("myaccount.blob.core.windows.net"));
        let p = url_params(&url);
        assert_eq!(p["sr"], "d");
        assert_eq!(p["sdd"], "3", "logs/2024/01 has 3 path segments");
        assert!(!p["sig"].is_empty());
    }

    #[test]
    fn blob_snapshot_resource() {
        let snapshot_time = datetime!(2024-06-15 12:30:45.1234567 UTC);
        let url = UserDelegationSasBuilder::new(
            "myaccount",
            BlobSnapshotResource {
                container: "mycontainer".into(),
                blob: "archive.zip".into(),
                snapshot_time,
                options: None,
            },
            BlobSasPermissions {
                read: true,
                ..Default::default()
            },
            EXPIRY,
        )
        .with_key(make_key())
        .build()
        .unwrap();

        assert_eq!(url.host_str(), Some("myaccount.blob.core.windows.net"));
        let p = url_params(&url);
        assert_eq!(p["sr"], "bs");
        assert_eq!(p["sst"], "2024-06-15T12:30:45.1234567Z");
        assert!(!p["sig"].is_empty());
    }

    #[test]
    fn blob_version_resource() {
        let version_id = datetime!(2024-06-15 09:00:00.0000000 UTC);
        let url = UserDelegationSasBuilder::new(
            "myaccount",
            BlobVersionResource {
                container: "mycontainer".into(),
                blob: "report.pdf".into(),
                version_id,
                options: None,
            },
            BlobSasPermissions {
                read: true,
                ..Default::default()
            },
            EXPIRY,
        )
        .with_key(make_key())
        .build()
        .unwrap();

        assert_eq!(url.host_str(), Some("myaccount.blob.core.windows.net"));
        let p = url_params(&url);
        assert_eq!(p["sr"], "bv");
        assert_eq!(p["svid"], "2024-06-15T09:00:00.0000000Z");
        assert!(!p["sig"].is_empty());
    }

    #[test]
    fn queue_resource() {
        let url = UserDelegationSasBuilder::new(
            "myaccount",
            QueueResource {
                queue: "myqueue".into(),
            },
            QueueSasPermissions {
                read: true,
                process: true,
                ..Default::default()
            },
            EXPIRY,
        )
        .with_key(make_key())
        .build()
        .unwrap();

        assert_eq!(url.host_str(), Some("myaccount.queue.core.windows.net"));
        assert!(url.path().ends_with("/myqueue"));
        let p = url_params(&url);
        assert!(!p.contains_key("sr"), "Queue SAS must not include sr");
        assert_eq!(p["sp"], "rp");
        assert_eq!(p["spr"], "https,http");
        assert!(!p["sig"].is_empty());
    }

    #[test]
    fn queue_resource_with_delegated_user() {
        let oid = Uuid::new_v4();
        let tid = Uuid::new_v4();
        let url = UserDelegationSasBuilder::new(
            "myaccount",
            QueueResource {
                queue: "myqueue".into(),
            },
            QueueSasPermissions {
                add: true,
                ..Default::default()
            },
            EXPIRY,
        )
        .delegated_user_object_id(oid)
        .delegated_user_tenant_id(tid)
        .with_key(make_key())
        .build()
        .unwrap();

        let p = url_params(&url);
        assert_eq!(p["sduoid"], oid.to_string());
        assert_eq!(p["skdutid"], tid.to_string());
    }

    #[test]
    fn queue_resource_with_delegated_user_oid_only() {
        // skdutid is cross-tenant only — sduoid must be settable independently.
        let oid = Uuid::new_v4();
        let url = UserDelegationSasBuilder::new(
            "myaccount",
            QueueResource {
                queue: "myqueue".into(),
            },
            QueueSasPermissions {
                read: true,
                ..Default::default()
            },
            EXPIRY,
        )
        .delegated_user_object_id(oid)
        .with_key(make_key())
        .build()
        .unwrap();

        let p = url_params(&url);
        assert_eq!(p["sduoid"], oid.to_string());
        assert!(!p.contains_key("skdutid"));
    }

    #[test]
    fn table_resource() {
        let url = UserDelegationSasBuilder::new(
            "myaccount",
            TableResource {
                table: "mytable".into(),
                options: None,
            },
            TableSasPermissions {
                read: true,
                ..Default::default()
            },
            EXPIRY,
        )
        .with_key(make_key())
        .build()
        .unwrap();

        assert_eq!(url.host_str(), Some("myaccount.table.core.windows.net"));
        let p = url_params(&url);
        assert!(!p.contains_key("sr"), "Table SAS must not include sr");
        assert_eq!(p["tn"], "mytable");
        assert_eq!(p["sp"], "r");
        assert_eq!(p["spr"], "https,http");
        assert!(!p["sig"].is_empty());
    }

    #[test]
    fn table_resource_with_key_range() {
        let url = UserDelegationSasBuilder::new(
            "myaccount",
            TableResource {
                table: "mytable".into(),
                options: Some(TableResourceOptions {
                    start_key: Some(("partA".into(), "row001".into())),
                    end_key: Some(("partZ".into(), "row999".into())),
                }),
            },
            TableSasPermissions {
                read: true,
                ..Default::default()
            },
            EXPIRY,
        )
        .with_key(make_key())
        .build()
        .unwrap();

        let p = url_params(&url);
        assert_eq!(p["spk"], "partA");
        assert_eq!(p["srk"], "row001");
        assert_eq!(p["epk"], "partZ");
        assert_eq!(p["erk"], "row999");
    }

    #[test]
    fn table_resource_with_delegated_user() {
        let oid = Uuid::new_v4();
        let tid = Uuid::new_v4();
        let url = UserDelegationSasBuilder::new(
            "myaccount",
            TableResource {
                table: "mytable".into(),
                options: None,
            },
            TableSasPermissions {
                read: true,
                ..Default::default()
            },
            EXPIRY,
        )
        .delegated_user_object_id(oid)
        .delegated_user_tenant_id(tid)
        .with_key(make_key())
        .build()
        .unwrap();

        let p = url_params(&url);
        assert_eq!(p["sduoid"], oid.to_string());
        assert_eq!(p["skdutid"], tid.to_string());
    }

    #[test]
    fn file_resource() {
        let url = UserDelegationSasBuilder::new(
            "myaccount",
            FileResource {
                share: "myshare".into(),
                path: "docs/readme.txt".into(),
                options: None,
            },
            FileSasPermissions {
                read: true,
                ..Default::default()
            },
            EXPIRY,
        )
        .with_key(make_key())
        .build()
        .unwrap();

        assert_eq!(url.host_str(), Some("myaccount.file.core.windows.net"));
        assert!(url.path().ends_with("/myshare/docs/readme.txt"));
        let p = url_params(&url);
        assert_eq!(p["sr"], "f");
        assert_eq!(p["sp"], "r");
        assert_eq!(p["spr"], "https,http");
        assert!(!p["sig"].is_empty());
    }

    #[test]
    fn file_resource_with_delegated_user() {
        let oid = Uuid::new_v4();
        let url = UserDelegationSasBuilder::new(
            "myaccount",
            FileResource {
                share: "myshare".into(),
                path: "docs/readme.txt".into(),
                options: None,
            },
            FileSasPermissions {
                read: true,
                ..Default::default()
            },
            EXPIRY,
        )
        .delegated_user_object_id(oid)
        .with_key(make_key())
        .build()
        .unwrap();

        let p = url_params(&url);
        assert_eq!(p["sduoid"], oid.to_string());
        assert!(!p.contains_key("skdutid"));
    }

    #[test]
    fn share_resource() {
        let url = UserDelegationSasBuilder::new(
            "myaccount",
            ShareResource {
                share: "myshare".into(),
                options: None,
            },
            ShareSasPermissions {
                read: true,
                list: true,
                ..Default::default()
            },
            EXPIRY,
        )
        .with_key(make_key())
        .build()
        .unwrap();

        assert_eq!(url.host_str(), Some("myaccount.file.core.windows.net"));
        assert!(url.path().ends_with("/myshare"));
        let p = url_params(&url);
        assert_eq!(p["sr"], "s");
        assert_eq!(p["sp"], "rl");
        assert!(!p["sig"].is_empty());
    }

    #[test]
    fn share_resource_with_delegated_user() {
        let oid = Uuid::new_v4();
        let tid = Uuid::new_v4();
        let url = UserDelegationSasBuilder::new(
            "myaccount",
            ShareResource {
                share: "myshare".into(),
                options: None,
            },
            ShareSasPermissions {
                read: true,
                ..Default::default()
            },
            EXPIRY,
        )
        .delegated_user_object_id(oid)
        .delegated_user_tenant_id(tid)
        .with_key(make_key())
        .build()
        .unwrap();

        let p = url_params(&url);
        assert_eq!(p["sduoid"], oid.to_string());
        assert_eq!(p["skdutid"], tid.to_string());
    }

    #[test]
    fn custom_endpoint_is_used() {
        let endpoint = Url::parse("http://127.0.0.1:10000/devstoreaccount1").unwrap();
        let url = UserDelegationSasBuilder::new(
            "devstoreaccount1",
            BlobResource {
                container: "mycontainer".into(),
                blob: "blob.txt".into(),
                options: None,
            },
            BlobSasPermissions {
                read: true,
                ..Default::default()
            },
            EXPIRY,
        )
        .endpoint(endpoint.clone())
        .with_key(make_key())
        .build()
        .unwrap();

        assert_eq!(url.host_str(), Some("127.0.0.1"));
        assert!(url.as_str().starts_with("http://127.0.0.1:10000/"));
    }

    #[test]
    fn https_only_protocol() {
        use crate::SignedProtocol;

        let url = UserDelegationSasBuilder::new(
            "myaccount",
            BlobResource {
                container: "mycontainer".into(),
                blob: "blob.txt".into(),
                options: None,
            },
            BlobSasPermissions {
                read: true,
                ..Default::default()
            },
            EXPIRY,
        )
        .protocol(SignedProtocol::Https)
        .with_key(make_key())
        .build()
        .unwrap();

        assert_eq!(url_params(&url)["spr"], "https");
    }
}

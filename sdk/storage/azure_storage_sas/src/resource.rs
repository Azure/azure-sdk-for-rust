pub mod blob;
pub mod file;
pub mod queue;
pub mod table;

/// Links an Azure Storage resource struct to the permission type valid for it.
///
/// This trait is sealed: only resource types defined in this crate can implement it.
/// Use the concrete resource structs ([`crate::blob::BlobResource`], [`crate::blob::ContainerResource`], etc.)
/// rather than this trait directly.
///
/// The associated [`Resource::Permissions`] type ensures only the correct
/// permission helper is accepted by [`crate::UserDelegationSasBuilder::new`] at compile time.
#[allow(private_bounds)]
pub trait Resource: sealed::Resource {
    /// The permission type for this resource.
    type Permissions: std::fmt::Display;
}

pub(crate) mod sealed {
    use crate::sas::{SasSigningContext, SasUrlParams};

    /// Seals [`crate::Resource`] so only types in this crate can implement it.
    pub(crate) trait Resource {
        fn default_endpoint(&self, account: &str) -> url::Url;
        fn default_api_version(&self) -> &'static str;
        fn canonicalized_resource(&self, account: &str) -> String;
        fn string_to_sign(&self, ctx: &SasSigningContext<'_>) -> String;
        fn sas_url(
            &self,
            account_endpoint: &url::Url,
            params: &SasUrlParams<'_>,
        ) -> azure_core::Result<url::Url>;
    }

    /// Marker for Blob Storage and Data Lake resource types.
    ///
    /// Unlocks [`crate::UserDelegationSasBuilder::authorized_user_object_id`] and
    /// [`crate::UserDelegationSasBuilder::unauthorized_user_object_id`], which are
    /// Blob-specific and absent from Queue, Table, and File formats.
    ///
    /// <https://learn.microsoft.com/rest/api/storageservices/create-user-delegation-sas#specify-query-parameters-to-override-response-headers-blob-storage-and-azure-files-only>
    pub(crate) trait BlobService {}

    /// Marker for Queue, Table, and File resource types.
    ///
    /// Unlocks [`crate::UserDelegationSasBuilder::delegated_user_object_id`] and
    /// [`crate::UserDelegationSasBuilder::delegated_user_tenant_id`], which are
    /// absent from the Blob format.
    pub(crate) trait DelegatedUserService {}
}

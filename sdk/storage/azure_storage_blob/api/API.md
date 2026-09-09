# azure_storage_blob

- **Description**: Microsoft Azure Blob Storage client library for Rust
- **Edition**: 2021
- **Rust version**: 1.88

## Features

- `default`
  - `azure_core/default`
  - `tokio`
- `tokio`

```rust
#![allow(dead_code)]
#![cfg_attr(docsrs, feature(doc_cfg))]
pub use azure_storage_blob::clients::*;
pub fn format_filter_expression(tags: &std::collections::HashMap<String, String>) -> Result<String, std::io::Error>;
#[derive(Clone, Debug)]
pub struct StorageError {
    pub status_code: azure_core::http::StatusCode,
    pub error_code: Option<crate::generated::models::StorageErrorCode>,
    pub message: Option<String>,
    pub request_id: Option<String>,
    pub reason: Option<String>,
    pub authentication_error_detail: Option<String>,
    pub copy_source_status_code: Option<azure_core::http::StatusCode>,
    pub copy_source_error_code: Option<String>,
    pub copy_source_error_message: Option<String>,
    pub additional_error_info: std::collections::HashMap<String, String>,
}
impl Display for StorageError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result;
}
impl Error for StorageError {
}
impl TryFrom<Error> for StorageError {
    type Error = Error;
    fn try_from(error: azure_core::Error) -> std::result::Result<Self, <Self as >::Error>;
}
pub type Result<T> = std::result::Result<T, StorageError>;
pub mod clients {
    pub struct AppendBlobClient {
    }
    impl AppendBlobClient {
        fn new(blob_url: Url, credential: Option<Arc<dyn TokenCredential>>, options: Option<AppendBlobClientOptions>) -> Result<Self>;
        fn url(&self) -> &Url;
    }
    impl AppendBlobClient {
        async fn append_block(&self, body: RequestContent<Bytes, NoFormat>, content_length: u64, options: Option<AppendBlobClientAppendBlockOptions<'_>>) -> Result<Response<AppendBlobClientAppendBlockResult, NoFormat>>;
        async fn append_block_from_url(&self, source_url: String, content_length: u64, options: Option<AppendBlobClientAppendBlockFromUrlOptions<'_>>) -> Result<Response<AppendBlobClientAppendBlockFromUrlResult, NoFormat>>;
        async fn create(&self, options: Option<AppendBlobClientCreateOptions<'_>>) -> Result<Response<AppendBlobClientCreateResult, NoFormat>>;
        async fn seal(&self, options: Option<AppendBlobClientSealOptions<'_>>) -> Result<Response<AppendBlobClientSealResult, NoFormat>>;
    }
    #[derive(Clone, Debug)]
    pub struct AppendBlobClientOptions {
        pub client_options: azure_core::http::ClientOptions,
        pub version: String,
    }
    impl Default for AppendBlobClientOptions {
        fn default() -> Self;
    }
    pub struct BlobClient {
    }
    impl BlobClient {
        fn append_blob_client(&self) -> AppendBlobClient;
        fn block_blob_client(&self) -> BlockBlobClient;
        async fn download(&self, options: Option<BlobClientDownloadOptions<'_>>) -> Result<BlobClientDownloadResult>;
        async fn download_into(&self, buffer: &mut [u8], options: Option<BlobClientDownloadOptions<'_>>) -> Result<BlobClientDownloadIntoResult>;
        async fn exists(&self) -> Result<bool>;
        fn new(blob_url: Url, credential: Option<Arc<dyn TokenCredential>>, options: Option<BlobClientOptions>) -> Result<Self>;
        fn page_blob_client(&self) -> PageBlobClient;
        async fn upload(&self, content: RequestContent<Bytes, NoFormat>, options: Option<BlobClientUploadOptions<'_>>) -> Result<BlobClientUploadResult>;
        fn url(&self) -> &Url;
        fn with_snapshot(&self, snapshot: &str) -> Result<Self>;
        fn with_version(&self, version_id: &str) -> Result<Self>;
    }
    impl BlobClient {
        async fn abort_copy(&self, copy_id: &str, options: Option<BlobClientAbortCopyOptions<'_>>) -> Result<Response<(), NoFormat>>;
        async fn acquire_lease(&self, duration: i32, options: Option<BlobClientAcquireLeaseOptions<'_>>) -> Result<Response<BlobClientAcquireLeaseResult, NoFormat>>;
        async fn break_lease(&self, options: Option<BlobClientBreakLeaseOptions<'_>>) -> Result<Response<BlobClientBreakLeaseResult, NoFormat>>;
        async fn change_lease(&self, lease_id: String, proposed_lease_id: String, options: Option<BlobClientChangeLeaseOptions<'_>>) -> Result<Response<BlobClientChangeLeaseResult, NoFormat>>;
        async fn create_snapshot(&self, options: Option<BlobClientCreateSnapshotOptions<'_>>) -> Result<Response<BlobClientCreateSnapshotResult, NoFormat>>;
        async fn delete(&self, options: Option<BlobClientDeleteOptions<'_>>) -> Result<Response<(), NoFormat>>;
        async fn delete_immutability_policy(&self, options: Option<BlobClientDeleteImmutabilityPolicyOptions<'_>>) -> Result<Response<(), NoFormat>>;
        async fn get_account_info(&self, options: Option<BlobClientGetAccountInfoOptions<'_>>) -> Result<Response<BlobClientGetAccountInfoResult, NoFormat>>;
        async fn get_properties(&self, options: Option<BlobClientGetPropertiesOptions<'_>>) -> Result<Response<BlobClientGetPropertiesResult, NoFormat>>;
        async fn get_tags(&self, options: Option<BlobClientGetTagsOptions<'_>>) -> Result<Response<BlobTags, XmlFormat>>;
        async fn release_lease(&self, lease_id: String, options: Option<BlobClientReleaseLeaseOptions<'_>>) -> Result<Response<BlobClientReleaseLeaseResult, NoFormat>>;
        async fn renew_lease(&self, lease_id: String, options: Option<BlobClientRenewLeaseOptions<'_>>) -> Result<Response<BlobClientRenewLeaseResult, NoFormat>>;
        async fn set_immutability_policy(&self, expiry: &OffsetDateTime, options: Option<BlobClientSetImmutabilityPolicyOptions<'_>>) -> Result<Response<(), NoFormat>>;
        async fn set_legal_hold(&self, legal_hold: bool, options: Option<BlobClientSetLegalHoldOptions<'_>>) -> Result<Response<(), NoFormat>>;
        async fn set_metadata(&self, metadata: &HashMap<String, String>, options: Option<BlobClientSetMetadataOptions<'_>>) -> Result<Response<(), NoFormat>>;
        async fn set_properties(&self, options: Option<BlobClientSetPropertiesOptions<'_>>) -> Result<Response<(), NoFormat>>;
        async fn set_tags(&self, tags: RequestContent<BlobTags, XmlFormat>, options: Option<BlobClientSetTagsOptions<'_>>) -> Result<Response<(), NoFormat>>;
        async fn set_tier(&self, tier: AccessTier, options: Option<BlobClientSetTierOptions<'_>>) -> Result<Response<(), NoFormat>>;
        async fn start_copy_from_url(&self, copy_source: String, options: Option<BlobClientStartCopyFromUrlOptions<'_>>) -> Result<Response<BlobClientStartCopyFromUrlResult, NoFormat>>;
        async fn undelete(&self, options: Option<BlobClientUndeleteOptions<'_>>) -> Result<Response<(), NoFormat>>;
    }
    #[derive(Clone, Debug)]
    pub struct BlobClientOptions {
        pub client_options: azure_core::http::ClientOptions,
        pub version: String,
    }
    impl Default for BlobClientOptions {
        fn default() -> Self;
    }
    pub struct BlobContainerClient {
    }
    impl BlobContainerClient {
        fn blob_client(&self, blob_name: &str) -> BlobClient;
        async fn exists(&self) -> Result<bool>;
        fn new(container_url: Url, credential: Option<Arc<dyn TokenCredential>>, options: Option<BlobContainerClientOptions>) -> Result<Self>;
        fn url(&self) -> &Url;
    }
    impl BlobContainerClient {
        async fn acquire_lease(&self, duration: i32, options: Option<BlobContainerClientAcquireLeaseOptions<'_>>) -> Result<Response<BlobContainerClientAcquireLeaseResult, NoFormat>>;
        async fn break_lease(&self, options: Option<BlobContainerClientBreakLeaseOptions<'_>>) -> Result<Response<BlobContainerClientBreakLeaseResult, NoFormat>>;
        async fn change_lease(&self, lease_id: String, proposed_lease_id: String, options: Option<BlobContainerClientChangeLeaseOptions<'_>>) -> Result<Response<BlobContainerClientChangeLeaseResult, NoFormat>>;
        async fn create(&self, options: Option<BlobContainerClientCreateOptions<'_>>) -> Result<Response<(), NoFormat>>;
        async fn delete(&self, options: Option<BlobContainerClientDeleteOptions<'_>>) -> Result<Response<(), NoFormat>>;
        fn find_blobs_by_tags(&self, filter_expression: &str, options: Option<BlobContainerClientFindBlobsByTagsOptions<'_>>) -> Result<Pager<FilteredBlobResponse, XmlFormat>>;
        async fn get_access_policy(&self, options: Option<BlobContainerClientGetAccessPolicyOptions<'_>>) -> Result<Response<SignedIdentifiers, XmlFormat>>;
        async fn get_account_info(&self, options: Option<BlobContainerClientGetAccountInfoOptions<'_>>) -> Result<Response<BlobContainerClientGetAccountInfoResult, NoFormat>>;
        async fn get_properties(&self, options: Option<BlobContainerClientGetPropertiesOptions<'_>>) -> Result<Response<BlobContainerClientGetPropertiesResult, NoFormat>>;
        fn list_blobs(&self, options: Option<BlobContainerClientListBlobsOptions<'_>>) -> Result<Pager<ListBlobsResponse, XmlFormat>>;
        async fn release_lease(&self, lease_id: String, options: Option<BlobContainerClientReleaseLeaseOptions<'_>>) -> Result<Response<BlobContainerClientReleaseLeaseResult, NoFormat>>;
        async fn renew_lease(&self, lease_id: String, options: Option<BlobContainerClientRenewLeaseOptions<'_>>) -> Result<Response<BlobContainerClientRenewLeaseResult, NoFormat>>;
        async fn set_access_policy(&self, container_acl: RequestContent<SignedIdentifiers, XmlFormat>, options: Option<BlobContainerClientSetAccessPolicyOptions<'_>>) -> Result<Response<(), NoFormat>>;
        async fn set_metadata(&self, metadata: &HashMap<String, String>, options: Option<BlobContainerClientSetMetadataOptions<'_>>) -> Result<Response<(), NoFormat>>;
    }
    #[derive(Clone, Debug)]
    pub struct BlobContainerClientOptions {
        pub client_options: azure_core::http::ClientOptions,
        pub version: String,
    }
    impl Default for BlobContainerClientOptions {
        fn default() -> Self;
    }
    pub struct BlobServiceClient {
    }
    impl BlobServiceClient {
        fn blob_client(&self, container_name: &str, blob_name: &str) -> BlobClient;
        fn blob_container_client(&self, container_name: &str) -> BlobContainerClient;
        fn new(service_url: Url, credential: Option<Arc<dyn TokenCredential>>, options: Option<BlobServiceClientOptions>) -> Result<Self>;
        fn url(&self) -> &Url;
    }
    impl BlobServiceClient {
        fn find_blobs_by_tags(&self, filter_expression: &str, options: Option<BlobServiceClientFindBlobsByTagsOptions<'_>>) -> Result<Pager<FilteredBlobResponse, XmlFormat>>;
        async fn get_account_info(&self, options: Option<BlobServiceClientGetAccountInfoOptions<'_>>) -> Result<Response<BlobServiceClientGetAccountInfoResult, NoFormat>>;
        async fn get_properties(&self, options: Option<BlobServiceClientGetPropertiesOptions<'_>>) -> Result<Response<BlobServiceProperties, XmlFormat>>;
        async fn get_statistics(&self, options: Option<BlobServiceClientGetStatisticsOptions<'_>>) -> Result<Response<StorageServiceStats, XmlFormat>>;
        async fn get_user_delegation_key(&self, key_info: RequestContent<KeyInfo, XmlFormat>, options: Option<BlobServiceClientGetUserDelegationKeyOptions<'_>>) -> Result<Response<UserDelegationKey, XmlFormat>>;
        fn list_containers(&self, options: Option<BlobServiceClientListContainersOptions<'_>>) -> Result<Pager<ListContainersResponse, XmlFormat>>;
        async fn set_properties(&self, storage_service_properties: RequestContent<BlobServiceProperties, XmlFormat>, options: Option<BlobServiceClientSetPropertiesOptions<'_>>) -> Result<Response<(), NoFormat>>;
    }
    #[derive(Clone, Debug)]
    pub struct BlobServiceClientOptions {
        pub client_options: azure_core::http::ClientOptions,
        pub version: String,
    }
    impl Default for BlobServiceClientOptions {
        fn default() -> Self;
    }
    pub struct BlockBlobClient {
    }
    impl BlockBlobClient {
        fn new(blob_url: Url, credential: Option<Arc<dyn TokenCredential>>, options: Option<BlockBlobClientOptions>) -> Result<Self>;
        async fn upload(&self, content: RequestContent<Bytes, NoFormat>, options: Option<BlockBlobClientUploadOptions<'_>>) -> Result<BlockBlobClientUploadResult>;
        fn url(&self) -> &Url;
    }
    impl BlockBlobClient {
        async fn commit_block_list(&self, blocks: RequestContent<BlockLookupList, XmlFormat>, options: Option<BlockBlobClientCommitBlockListOptions<'_>>) -> Result<Response<BlockBlobClientCommitBlockListResult, NoFormat>>;
        async fn get_block_list(&self, list_type: BlockListType, options: Option<BlockBlobClientGetBlockListOptions<'_>>) -> Result<Response<BlockList, XmlFormat>>;
        async fn stage_block(&self, block_id: &[u8], content_length: u64, body: RequestContent<Bytes, NoFormat>, options: Option<BlockBlobClientStageBlockOptions<'_>>) -> Result<Response<BlockBlobClientStageBlockResult, NoFormat>>;
        async fn stage_block_from_url(&self, block_id: &[u8], content_length: u64, source_url: String, options: Option<BlockBlobClientStageBlockFromUrlOptions<'_>>) -> Result<Response<BlockBlobClientStageBlockFromUrlResult, NoFormat>>;
        async fn upload_blob_from_url(&self, copy_source: String, options: Option<BlockBlobClientUploadBlobFromUrlOptions<'_>>) -> Result<Response<BlockBlobClientUploadBlobFromUrlResult, NoFormat>>;
    }
    #[derive(Clone, Debug)]
    pub struct BlockBlobClientOptions {
        pub client_options: azure_core::http::ClientOptions,
        pub version: String,
    }
    impl Default for BlockBlobClientOptions {
        fn default() -> Self;
    }
    pub struct PageBlobClient {
    }
    impl PageBlobClient {
        fn new(blob_url: Url, credential: Option<Arc<dyn TokenCredential>>, options: Option<PageBlobClientOptions>) -> Result<Self>;
        fn url(&self) -> &Url;
    }
    impl PageBlobClient {
        async fn clear_pages(&self, range: HttpRange, options: Option<PageBlobClientClearPagesOptions<'_>>) -> Result<Response<PageBlobClientClearPagesResult, NoFormat>>;
        async fn create(&self, size: u64, options: Option<PageBlobClientCreateOptions<'_>>) -> Result<Response<PageBlobClientCreateResult, NoFormat>>;
        fn list_page_ranges(&self, options: Option<PageBlobClientListPageRangesOptions<'_>>) -> Result<PageIterator<Response<PageList, XmlFormat>>>;
        async fn resize(&self, size: u64, options: Option<PageBlobClientResizeOptions<'_>>) -> Result<Response<PageBlobClientResizeResult, NoFormat>>;
        async fn set_sequence_number(&self, sequence_number_action: SequenceNumberActionType, options: Option<PageBlobClientSetSequenceNumberOptions<'_>>) -> Result<Response<PageBlobClientSetSequenceNumberResult, NoFormat>>;
        async fn upload_pages(&self, body: RequestContent<Bytes, NoFormat>, content_length: u64, range: HttpRange, options: Option<PageBlobClientUploadPagesOptions<'_>>) -> Result<Response<PageBlobClientUploadPagesResult, NoFormat>>;
        async fn upload_pages_from_url(&self, source_url: String, source_range: HttpRange, content_length: u64, range: HttpRange, options: Option<PageBlobClientUploadPagesFromUrlOptions<'_>>) -> Result<Response<PageBlobClientUploadPagesFromUrlResult, NoFormat>>;
    }
    #[derive(Clone, Debug)]
    pub struct PageBlobClientOptions {
        pub client_options: azure_core::http::ClientOptions,
        pub version: String,
    }
    impl Default for PageBlobClientOptions {
        fn default() -> Self;
    }
}
pub mod models {
    pub use azure_storage_blob::models::method_options::BlockBlobClientUploadOptions as BlobClientUploadOptions;
    pub use azure_storage_blob::models::upload_result::BlockBlobClientUploadResult as BlobClientUploadResult;
    #[derive(Clone, Debug, Default, serde::Deserialize, serde::Serialize)]
    pub struct AccessPolicy {
        #[serde(default, rename = "Expiry", skip_serializing_if = "Option::is_none", with = "models_serde::option_offset_date_time_rfc3339_fixed_width")]
        pub expiry: Option<azure_core::time::OffsetDateTime>,
        #[serde(rename = "Permission", skip_serializing_if = "Option::is_none")]
        pub permission: Option<String>,
        #[serde(default, rename = "Start", skip_serializing_if = "Option::is_none", with = "models_serde::option_offset_date_time_rfc3339_fixed_width")]
        pub start: Option<azure_core::time::OffsetDateTime>,
    }
    #[derive(Clone, Debug, Default)]
    pub struct AppendBlobClientAppendBlockFromUrlOptions<'a> {
        pub append_position: Option<i64>,
        pub copy_source_authorization: Option<String>,
        pub encryption_algorithm: Option<super::EncryptionAlgorithmType>,
        pub encryption_key: Option<String>,
        pub encryption_key_sha256: Option<String>,
        pub encryption_scope: Option<String>,
        pub file_request_intent: Option<super::FileShareTokenIntent>,
        pub if_match: Option<azure_core::http::Etag>,
        pub if_modified_since: Option<azure_core::time::OffsetDateTime>,
        pub if_none_match: Option<azure_core::http::Etag>,
        pub if_tags: Option<String>,
        pub if_unmodified_since: Option<azure_core::time::OffsetDateTime>,
        pub lease_id: Option<String>,
        pub max_size: Option<i64>,
        pub method_options: azure_core::http::ClientMethodOptions<'a>,
        pub source_content_crc64: Option<Vec<u8>>,
        pub source_content_md5: Option<Vec<u8>>,
        pub source_encryption_algorithm: Option<super::EncryptionAlgorithmType>,
        pub source_encryption_key: Option<String>,
        pub source_encryption_key_sha256: Option<String>,
        pub source_if_match: Option<azure_core::http::Etag>,
        pub source_if_modified_since: Option<azure_core::time::OffsetDateTime>,
        pub source_if_none_match: Option<azure_core::http::Etag>,
        pub source_if_unmodified_since: Option<azure_core::time::OffsetDateTime>,
        pub source_range: Option<crate::models::HttpRange>,
        pub timeout: Option<i32>,
        pub transactional_content_md5: Option<Vec<u8>>,
    }
    #[derive(Debug)]
    pub struct AppendBlobClientAppendBlockFromUrlResult;
    #[derive(Clone, Debug, Default)]
    pub struct AppendBlobClientAppendBlockOptions<'a> {
        pub append_position: Option<i64>,
        pub encryption_algorithm: Option<super::EncryptionAlgorithmType>,
        pub encryption_key: Option<String>,
        pub encryption_key_sha256: Option<String>,
        pub encryption_scope: Option<String>,
        pub if_match: Option<azure_core::http::Etag>,
        pub if_modified_since: Option<azure_core::time::OffsetDateTime>,
        pub if_none_match: Option<azure_core::http::Etag>,
        pub if_tags: Option<String>,
        pub if_unmodified_since: Option<azure_core::time::OffsetDateTime>,
        pub lease_id: Option<String>,
        pub max_size: Option<i64>,
        pub method_options: azure_core::http::ClientMethodOptions<'a>,
        pub timeout: Option<i32>,
        pub transactional_content_crc64: Option<Vec<u8>>,
        pub transactional_content_md5: Option<Vec<u8>>,
    }
    #[derive(Debug)]
    pub struct AppendBlobClientAppendBlockResult;
    #[derive(Clone, Debug, Default)]
    pub struct AppendBlobClientCreateOptions<'a> {
        pub blob_cache_control: Option<String>,
        pub blob_content_disposition: Option<String>,
        pub blob_content_encoding: Option<String>,
        pub blob_content_language: Option<String>,
        pub blob_content_md5: Option<Vec<u8>>,
        pub blob_content_type: Option<String>,
        pub blob_tags_string: Option<String>,
        pub encryption_algorithm: Option<super::EncryptionAlgorithmType>,
        pub encryption_key: Option<String>,
        pub encryption_key_sha256: Option<String>,
        pub encryption_scope: Option<String>,
        pub if_match: Option<azure_core::http::Etag>,
        pub if_modified_since: Option<azure_core::time::OffsetDateTime>,
        pub if_none_match: Option<azure_core::http::Etag>,
        pub if_tags: Option<String>,
        pub if_unmodified_since: Option<azure_core::time::OffsetDateTime>,
        pub immutability_policy_expiry: Option<azure_core::time::OffsetDateTime>,
        pub immutability_policy_mode: Option<super::ImmutabilityPolicyMode>,
        pub lease_id: Option<String>,
        pub legal_hold: Option<bool>,
        pub metadata: Option<std::collections::HashMap<String, String>>,
        pub method_options: azure_core::http::ClientMethodOptions<'a>,
        pub timeout: Option<i32>,
    }
    impl crate::models::AppendBlobClientCreateOptions<'_> {
        fn if_not_exists(self) -> Self;
    }
    impl crate::models::AppendBlobClientCreateOptions<'_> {
        fn with_tags<impl Into<BlobTags>: Into<BlobTags>>(self, tags: impl Into<BlobTags>) -> Self;
    }
    #[derive(Debug)]
    pub struct AppendBlobClientCreateResult;
    #[derive(Clone, Debug, Default)]
    pub struct AppendBlobClientSealOptions<'a> {
        pub append_position: Option<i64>,
        pub if_match: Option<azure_core::http::Etag>,
        pub if_modified_since: Option<azure_core::time::OffsetDateTime>,
        pub if_none_match: Option<azure_core::http::Etag>,
        pub if_unmodified_since: Option<azure_core::time::OffsetDateTime>,
        pub lease_id: Option<String>,
        pub method_options: azure_core::http::ClientMethodOptions<'a>,
        pub timeout: Option<i32>,
    }
    #[derive(Debug)]
    pub struct AppendBlobClientSealResult;
    #[derive(Clone, Debug, Default)]
    pub struct BlobClientAbortCopyOptions<'a> {
        pub lease_id: Option<String>,
        pub method_options: azure_core::http::ClientMethodOptions<'a>,
        pub timeout: Option<i32>,
    }
    #[derive(Clone, Debug, Default)]
    pub struct BlobClientAcquireLeaseOptions<'a> {
        pub if_match: Option<azure_core::http::Etag>,
        pub if_modified_since: Option<azure_core::time::OffsetDateTime>,
        pub if_none_match: Option<azure_core::http::Etag>,
        pub if_tags: Option<String>,
        pub if_unmodified_since: Option<azure_core::time::OffsetDateTime>,
        pub method_options: azure_core::http::ClientMethodOptions<'a>,
        pub proposed_lease_id: Option<String>,
        pub timeout: Option<i32>,
    }
    #[derive(Debug)]
    pub struct BlobClientAcquireLeaseResult;
    #[derive(Clone, Debug, Default)]
    pub struct BlobClientBreakLeaseOptions<'a> {
        pub break_period: Option<i32>,
        pub if_match: Option<azure_core::http::Etag>,
        pub if_modified_since: Option<azure_core::time::OffsetDateTime>,
        pub if_none_match: Option<azure_core::http::Etag>,
        pub if_tags: Option<String>,
        pub if_unmodified_since: Option<azure_core::time::OffsetDateTime>,
        pub method_options: azure_core::http::ClientMethodOptions<'a>,
        pub timeout: Option<i32>,
    }
    #[derive(Debug)]
    pub struct BlobClientBreakLeaseResult;
    #[derive(Clone, Debug, Default)]
    pub struct BlobClientChangeLeaseOptions<'a> {
        pub if_match: Option<azure_core::http::Etag>,
        pub if_modified_since: Option<azure_core::time::OffsetDateTime>,
        pub if_none_match: Option<azure_core::http::Etag>,
        pub if_tags: Option<String>,
        pub if_unmodified_since: Option<azure_core::time::OffsetDateTime>,
        pub method_options: azure_core::http::ClientMethodOptions<'a>,
        pub timeout: Option<i32>,
    }
    #[derive(Debug)]
    pub struct BlobClientChangeLeaseResult;
    #[derive(Clone, Debug, Default)]
    pub struct BlobClientCreateSnapshotOptions<'a> {
        pub encryption_algorithm: Option<super::EncryptionAlgorithmType>,
        pub encryption_key: Option<String>,
        pub encryption_key_sha256: Option<String>,
        pub encryption_scope: Option<String>,
        pub if_match: Option<azure_core::http::Etag>,
        pub if_modified_since: Option<azure_core::time::OffsetDateTime>,
        pub if_none_match: Option<azure_core::http::Etag>,
        pub if_tags: Option<String>,
        pub if_unmodified_since: Option<azure_core::time::OffsetDateTime>,
        pub lease_id: Option<String>,
        pub metadata: Option<std::collections::HashMap<String, String>>,
        pub method_options: azure_core::http::ClientMethodOptions<'a>,
        pub timeout: Option<i32>,
    }
    #[derive(Debug)]
    pub struct BlobClientCreateSnapshotResult;
    #[derive(Clone, Debug, Default)]
    pub struct BlobClientDeleteImmutabilityPolicyOptions<'a> {
        pub method_options: azure_core::http::ClientMethodOptions<'a>,
        pub snapshot: Option<String>,
        pub timeout: Option<i32>,
        pub version_id: Option<String>,
    }
    #[derive(Clone, Debug, Default)]
    pub struct BlobClientDeleteOptions<'a> {
        pub access_tier_if_modified_since: Option<azure_core::time::OffsetDateTime>,
        pub access_tier_if_unmodified_since: Option<azure_core::time::OffsetDateTime>,
        pub blob_delete_type: Option<super::BlobDeleteType>,
        pub delete_snapshots: Option<super::DeleteSnapshotsOptionType>,
        pub if_match: Option<azure_core::http::Etag>,
        pub if_modified_since: Option<azure_core::time::OffsetDateTime>,
        pub if_none_match: Option<azure_core::http::Etag>,
        pub if_tags: Option<String>,
        pub if_unmodified_since: Option<azure_core::time::OffsetDateTime>,
        pub lease_id: Option<String>,
        pub method_options: azure_core::http::ClientMethodOptions<'a>,
        pub snapshot: Option<String>,
        pub timeout: Option<i32>,
        pub version_id: Option<String>,
    }
    #[derive(Debug)]
    pub struct BlobClientDownloadIntoResult {
        pub len: usize,
        pub properties: BlobDownloadProperties,
        pub headers: azure_core::http::headers::Headers,
    }
    #[derive(Clone, Debug, Default)]
    pub struct BlobClientDownloadOptions<'a> {
        pub encryption_algorithm: Option<crate::models::EncryptionAlgorithmType>,
        pub encryption_key: Option<String>,
        pub encryption_key_sha256: Option<String>,
        pub if_match: Option<azure_core::http::Etag>,
        pub if_modified_since: Option<time::OffsetDateTime>,
        pub if_none_match: Option<azure_core::http::Etag>,
        pub if_tags: Option<String>,
        pub if_unmodified_since: Option<time::OffsetDateTime>,
        pub lease_id: Option<String>,
        pub method_options: azure_core::http::ClientMethodOptions<'a>,
        pub parallel: Option<std::num::NonZero<usize>>,
        pub partition_size: Option<std::num::NonZero<usize>>,
        pub range: Option<crate::models::HttpRange>,
        pub range_get_content_crc64: Option<bool>,
        pub range_get_content_md5: Option<bool>,
        pub snapshot: Option<String>,
        pub timeout: Option<i32>,
        pub version_id: Option<String>,
    }
    #[derive(Debug)]
    pub struct BlobClientDownloadResult {
        pub body: azure_core::http::response::AsyncResponseBody,
        pub properties: BlobDownloadProperties,
        pub headers: azure_core::http::headers::Headers,
    }
    #[derive(Clone, Debug, Default)]
    pub struct BlobClientGetAccountInfoOptions<'a> {
        pub method_options: azure_core::http::ClientMethodOptions<'a>,
        pub timeout: Option<i32>,
    }
    #[derive(Debug)]
    pub struct BlobClientGetAccountInfoResult;
    #[derive(Clone, Debug, Default)]
    pub struct BlobClientGetPropertiesOptions<'a> {
        pub encryption_algorithm: Option<super::EncryptionAlgorithmType>,
        pub encryption_key: Option<String>,
        pub encryption_key_sha256: Option<String>,
        pub if_match: Option<azure_core::http::Etag>,
        pub if_modified_since: Option<azure_core::time::OffsetDateTime>,
        pub if_none_match: Option<azure_core::http::Etag>,
        pub if_tags: Option<String>,
        pub if_unmodified_since: Option<azure_core::time::OffsetDateTime>,
        pub lease_id: Option<String>,
        pub method_options: azure_core::http::ClientMethodOptions<'a>,
        pub snapshot: Option<String>,
        pub timeout: Option<i32>,
        pub version_id: Option<String>,
    }
    #[derive(Debug)]
    pub struct BlobClientGetPropertiesResult;
    #[derive(Clone, Debug, Default)]
    pub struct BlobClientGetTagsOptions<'a> {
        pub if_match: Option<azure_core::http::Etag>,
        pub if_modified_since: Option<azure_core::time::OffsetDateTime>,
        pub if_none_match: Option<azure_core::http::Etag>,
        pub if_tags: Option<String>,
        pub if_unmodified_since: Option<azure_core::time::OffsetDateTime>,
        pub lease_id: Option<String>,
        pub method_options: azure_core::http::ClientMethodOptions<'a>,
        pub snapshot: Option<String>,
        pub timeout: Option<i32>,
        pub version_id: Option<String>,
    }
    #[derive(Clone, Debug, Default)]
    pub struct BlobClientReleaseLeaseOptions<'a> {
        pub if_match: Option<azure_core::http::Etag>,
        pub if_modified_since: Option<azure_core::time::OffsetDateTime>,
        pub if_none_match: Option<azure_core::http::Etag>,
        pub if_tags: Option<String>,
        pub if_unmodified_since: Option<azure_core::time::OffsetDateTime>,
        pub method_options: azure_core::http::ClientMethodOptions<'a>,
        pub timeout: Option<i32>,
    }
    #[derive(Debug)]
    pub struct BlobClientReleaseLeaseResult;
    #[derive(Clone, Debug, Default)]
    pub struct BlobClientRenewLeaseOptions<'a> {
        pub if_match: Option<azure_core::http::Etag>,
        pub if_modified_since: Option<azure_core::time::OffsetDateTime>,
        pub if_none_match: Option<azure_core::http::Etag>,
        pub if_tags: Option<String>,
        pub if_unmodified_since: Option<azure_core::time::OffsetDateTime>,
        pub method_options: azure_core::http::ClientMethodOptions<'a>,
        pub timeout: Option<i32>,
    }
    #[derive(Debug)]
    pub struct BlobClientRenewLeaseResult;
    #[derive(Clone, Debug, Default)]
    pub struct BlobClientSetImmutabilityPolicyOptions<'a> {
        pub if_unmodified_since: Option<azure_core::time::OffsetDateTime>,
        pub immutability_policy_mode: Option<super::ImmutabilityPolicyMode>,
        pub method_options: azure_core::http::ClientMethodOptions<'a>,
        pub snapshot: Option<String>,
        pub timeout: Option<i32>,
        pub version_id: Option<String>,
    }
    #[derive(Clone, Debug, Default)]
    pub struct BlobClientSetLegalHoldOptions<'a> {
        pub method_options: azure_core::http::ClientMethodOptions<'a>,
        pub snapshot: Option<String>,
        pub timeout: Option<i32>,
        pub version_id: Option<String>,
    }
    #[derive(Clone, Debug, Default)]
    pub struct BlobClientSetMetadataOptions<'a> {
        pub encryption_algorithm: Option<super::EncryptionAlgorithmType>,
        pub encryption_key: Option<String>,
        pub encryption_key_sha256: Option<String>,
        pub encryption_scope: Option<String>,
        pub if_match: Option<azure_core::http::Etag>,
        pub if_modified_since: Option<azure_core::time::OffsetDateTime>,
        pub if_none_match: Option<azure_core::http::Etag>,
        pub if_tags: Option<String>,
        pub if_unmodified_since: Option<azure_core::time::OffsetDateTime>,
        pub lease_id: Option<String>,
        pub method_options: azure_core::http::ClientMethodOptions<'a>,
        pub timeout: Option<i32>,
    }
    #[derive(Clone, Debug, Default)]
    pub struct BlobClientSetPropertiesOptions<'a> {
        pub blob_cache_control: Option<String>,
        pub blob_content_disposition: Option<String>,
        pub blob_content_encoding: Option<String>,
        pub blob_content_language: Option<String>,
        pub blob_content_md5: Option<Vec<u8>>,
        pub blob_content_type: Option<String>,
        pub if_match: Option<azure_core::http::Etag>,
        pub if_modified_since: Option<azure_core::time::OffsetDateTime>,
        pub if_none_match: Option<azure_core::http::Etag>,
        pub if_tags: Option<String>,
        pub if_unmodified_since: Option<azure_core::time::OffsetDateTime>,
        pub lease_id: Option<String>,
        pub method_options: azure_core::http::ClientMethodOptions<'a>,
        pub timeout: Option<i32>,
    }
    #[derive(Clone, Debug, Default)]
    pub struct BlobClientSetTagsOptions<'a> {
        pub if_match: Option<azure_core::http::Etag>,
        pub if_modified_since: Option<azure_core::time::OffsetDateTime>,
        pub if_none_match: Option<azure_core::http::Etag>,
        pub if_tags: Option<String>,
        pub if_unmodified_since: Option<azure_core::time::OffsetDateTime>,
        pub lease_id: Option<String>,
        pub method_options: azure_core::http::ClientMethodOptions<'a>,
        pub timeout: Option<i32>,
        pub transactional_content_crc64: Option<Vec<u8>>,
        pub transactional_content_md5: Option<Vec<u8>>,
        pub version_id: Option<String>,
    }
    #[derive(Clone, Debug, Default)]
    pub struct BlobClientSetTierOptions<'a> {
        pub if_tags: Option<String>,
        pub lease_id: Option<String>,
        pub method_options: azure_core::http::ClientMethodOptions<'a>,
        pub rehydrate_priority: Option<super::RehydratePriority>,
        pub snapshot: Option<String>,
        pub timeout: Option<i32>,
        pub version_id: Option<String>,
    }
    #[derive(Clone, Debug, Default)]
    pub struct BlobClientStartCopyFromUrlOptions<'a> {
        pub blob_tags_string: Option<String>,
        pub if_match: Option<azure_core::http::Etag>,
        pub if_modified_since: Option<azure_core::time::OffsetDateTime>,
        pub if_none_match: Option<azure_core::http::Etag>,
        pub if_tags: Option<String>,
        pub if_unmodified_since: Option<azure_core::time::OffsetDateTime>,
        pub immutability_policy_expiry: Option<azure_core::time::OffsetDateTime>,
        pub immutability_policy_mode: Option<super::ImmutabilityPolicyMode>,
        pub lease_id: Option<String>,
        pub legal_hold: Option<bool>,
        pub metadata: Option<std::collections::HashMap<String, String>>,
        pub method_options: azure_core::http::ClientMethodOptions<'a>,
        pub rehydrate_priority: Option<super::RehydratePriority>,
        pub seal_blob: Option<bool>,
        pub source_if_match: Option<azure_core::http::Etag>,
        pub source_if_modified_since: Option<azure_core::time::OffsetDateTime>,
        pub source_if_none_match: Option<azure_core::http::Etag>,
        pub source_if_tags: Option<String>,
        pub source_if_unmodified_since: Option<azure_core::time::OffsetDateTime>,
        pub tier: Option<super::AccessTier>,
        pub timeout: Option<i32>,
    }
    #[derive(Debug)]
    pub struct BlobClientStartCopyFromUrlResult;
    #[derive(Clone, Debug, Default)]
    pub struct BlobClientUndeleteOptions<'a> {
        pub method_options: azure_core::http::ClientMethodOptions<'a>,
        pub timeout: Option<i32>,
    }
    #[derive(Clone, Debug, Default)]
    pub struct BlobContainerClientAcquireLeaseOptions<'a> {
        pub if_modified_since: Option<azure_core::time::OffsetDateTime>,
        pub if_unmodified_since: Option<azure_core::time::OffsetDateTime>,
        pub method_options: azure_core::http::ClientMethodOptions<'a>,
        pub proposed_lease_id: Option<String>,
        pub timeout: Option<i32>,
    }
    #[derive(Debug)]
    pub struct BlobContainerClientAcquireLeaseResult;
    #[derive(Clone, Debug, Default)]
    pub struct BlobContainerClientBreakLeaseOptions<'a> {
        pub break_period: Option<i32>,
        pub if_modified_since: Option<azure_core::time::OffsetDateTime>,
        pub if_unmodified_since: Option<azure_core::time::OffsetDateTime>,
        pub method_options: azure_core::http::ClientMethodOptions<'a>,
        pub timeout: Option<i32>,
    }
    #[derive(Debug)]
    pub struct BlobContainerClientBreakLeaseResult;
    #[derive(Clone, Debug, Default)]
    pub struct BlobContainerClientChangeLeaseOptions<'a> {
        pub if_modified_since: Option<azure_core::time::OffsetDateTime>,
        pub if_unmodified_since: Option<azure_core::time::OffsetDateTime>,
        pub method_options: azure_core::http::ClientMethodOptions<'a>,
        pub timeout: Option<i32>,
    }
    #[derive(Debug)]
    pub struct BlobContainerClientChangeLeaseResult;
    #[derive(Clone, Debug, Default)]
    pub struct BlobContainerClientCreateOptions<'a> {
        pub access: Option<super::PublicAccessType>,
        pub default_encryption_scope: Option<String>,
        pub metadata: Option<std::collections::HashMap<String, String>>,
        pub method_options: azure_core::http::ClientMethodOptions<'a>,
        pub prevent_encryption_scope_override: Option<bool>,
        pub timeout: Option<i32>,
    }
    #[derive(Clone, Debug, Default)]
    pub struct BlobContainerClientDeleteOptions<'a> {
        pub if_modified_since: Option<azure_core::time::OffsetDateTime>,
        pub if_unmodified_since: Option<azure_core::time::OffsetDateTime>,
        pub lease_id: Option<String>,
        pub method_options: azure_core::http::ClientMethodOptions<'a>,
        pub timeout: Option<i32>,
    }
    #[derive(Clone, Debug, Default)]
    pub struct BlobContainerClientFindBlobsByTagsOptions<'a> {
        pub include: Option<Vec<super::FilterBlobsIncludeItem>>,
        pub marker: Option<String>,
        pub maxresults: Option<i32>,
        pub method_options: azure_core::http::pager::PagerOptions<'a>,
        pub timeout: Option<i32>,
    }
    impl BlobContainerClientFindBlobsByTagsOptions<'_> {
        fn into_owned(self) -> BlobContainerClientFindBlobsByTagsOptions<'static>;
    }
    #[derive(Clone, Debug, Default)]
    pub struct BlobContainerClientGetAccessPolicyOptions<'a> {
        pub lease_id: Option<String>,
        pub method_options: azure_core::http::ClientMethodOptions<'a>,
        pub timeout: Option<i32>,
    }
    #[derive(Clone, Debug, Default)]
    pub struct BlobContainerClientGetAccountInfoOptions<'a> {
        pub method_options: azure_core::http::ClientMethodOptions<'a>,
        pub timeout: Option<i32>,
    }
    #[derive(Debug)]
    pub struct BlobContainerClientGetAccountInfoResult;
    #[derive(Clone, Debug, Default)]
    pub struct BlobContainerClientGetPropertiesOptions<'a> {
        pub lease_id: Option<String>,
        pub method_options: azure_core::http::ClientMethodOptions<'a>,
        pub timeout: Option<i32>,
    }
    #[derive(Debug)]
    pub struct BlobContainerClientGetPropertiesResult;
    #[derive(Clone, Debug, Default)]
    pub struct BlobContainerClientListBlobsOptions<'a> {
        pub include: Option<Vec<super::ListBlobsIncludeItem>>,
        pub marker: Option<String>,
        pub maxresults: Option<i32>,
        pub method_options: azure_core::http::pager::PagerOptions<'a>,
        pub prefix: Option<String>,
        pub start_from: Option<String>,
        pub timeout: Option<i32>,
    }
    impl BlobContainerClientListBlobsOptions<'_> {
        fn into_owned(self) -> BlobContainerClientListBlobsOptions<'static>;
    }
    #[derive(Clone, Debug, Default)]
    pub struct BlobContainerClientReleaseLeaseOptions<'a> {
        pub if_modified_since: Option<azure_core::time::OffsetDateTime>,
        pub if_unmodified_since: Option<azure_core::time::OffsetDateTime>,
        pub method_options: azure_core::http::ClientMethodOptions<'a>,
        pub timeout: Option<i32>,
    }
    #[derive(Debug)]
    pub struct BlobContainerClientReleaseLeaseResult;
    #[derive(Clone, Debug, Default)]
    pub struct BlobContainerClientRenewLeaseOptions<'a> {
        pub if_modified_since: Option<azure_core::time::OffsetDateTime>,
        pub if_unmodified_since: Option<azure_core::time::OffsetDateTime>,
        pub method_options: azure_core::http::ClientMethodOptions<'a>,
        pub timeout: Option<i32>,
    }
    #[derive(Debug)]
    pub struct BlobContainerClientRenewLeaseResult;
    #[derive(Clone, Debug, Default)]
    pub struct BlobContainerClientSetAccessPolicyOptions<'a> {
        pub access: Option<super::PublicAccessType>,
        pub if_modified_since: Option<azure_core::time::OffsetDateTime>,
        pub if_unmodified_since: Option<azure_core::time::OffsetDateTime>,
        pub lease_id: Option<String>,
        pub method_options: azure_core::http::ClientMethodOptions<'a>,
        pub timeout: Option<i32>,
    }
    #[derive(Clone, Debug, Default)]
    pub struct BlobContainerClientSetMetadataOptions<'a> {
        pub if_modified_since: Option<azure_core::time::OffsetDateTime>,
        pub lease_id: Option<String>,
        pub method_options: azure_core::http::ClientMethodOptions<'a>,
        pub timeout: Option<i32>,
    }
    #[derive(Debug)]
    pub struct BlobDownloadProperties {
        pub etag: Option<azure_core::http::Etag>,
        pub last_modified: Option<time::OffsetDateTime>,
        pub created_on: Option<time::OffsetDateTime>,
        pub last_accessed: Option<time::OffsetDateTime>,
        pub content_length: Option<u64>,
        pub content_type: Option<String>,
        pub cache_control: Option<String>,
        pub content_disposition: Option<String>,
        pub content_encoding: Option<String>,
        pub content_language: Option<String>,
        pub content_md5: Option<Vec<u8>>,
        pub content_crc64: Option<Vec<u8>>,
        pub blob_content_md5: Option<Vec<u8>>,
        pub blob_type: Option<crate::generated::models::BlobType>,
        pub blob_sequence_number: Option<i64>,
        pub blob_committed_block_count: Option<i32>,
        pub is_sealed: Option<bool>,
        pub metadata: std::collections::HashMap<String, String>,
        pub version_id: Option<String>,
        pub lease_state: Option<crate::generated::models::LeaseState>,
        pub lease_status: Option<crate::generated::models::LeaseStatus>,
        pub lease_duration: Option<crate::generated::models::LeaseDuration>,
        pub legal_hold: Option<bool>,
        pub immutability_policy_mode: Option<crate::generated::models::ImmutabilityPolicyMode>,
        pub immutability_policy_expires_on: Option<time::OffsetDateTime>,
        pub copy_completed_on: Option<time::OffsetDateTime>,
        pub copy_id: Option<String>,
        pub copy_progress: Option<String>,
        pub copy_source: Option<String>,
        pub copy_status: Option<crate::generated::models::CopyStatus>,
        pub copy_status_description: Option<String>,
        pub object_replication_policy_id: Option<String>,
        pub object_replication_rules: std::collections::HashMap<String, String>,
        pub tag_count: Option<i64>,
        pub encryption_scope: Option<String>,
        pub encryption_key_sha256: Option<String>,
    }
    #[derive(Clone, Debug, Default, serde::Deserialize, serde::Serialize)]
    #[non_exhaustive]
    #[serde(rename = "Blob")]
    pub struct BlobItem {
        #[serde(rename = "Tags", skip_serializing_if = "Option::is_none")]
        pub blob_tags: Option<BlobTags>,
        #[serde(rename = "Deleted", skip_serializing_if = "Option::is_none")]
        pub deleted: Option<bool>,
        #[serde(rename = "HasVersionsOnly", skip_serializing_if = "Option::is_none")]
        pub has_versions_only: Option<bool>,
        #[serde(rename = "IsCurrentVersion", skip_serializing_if = "Option::is_none")]
        pub is_current_version: Option<bool>,
        #[serde(rename = "Metadata", skip_serializing_if = "Option::is_none")]
        pub metadata: Option<crate::models::BlobMetadata>,
        #[serde(deserialize_with = "crate::models::blob_name::option::deserialize", rename = "Name", skip_serializing_if = "Option::is_none")]
        pub name: Option<String>,
        #[serde(rename = "OrMetadata", skip_serializing_if = "Option::is_none")]
        pub object_replication_metadata: Option<ObjectReplicationMetadata>,
        #[serde(rename = "Properties", skip_serializing_if = "Option::is_none")]
        pub properties: Option<BlobProperties>,
        #[serde(rename = "Snapshot", skip_serializing_if = "Option::is_none")]
        pub snapshot: Option<String>,
        #[serde(rename = "VersionId", skip_serializing_if = "Option::is_none")]
        pub version_id: Option<String>,
    }
    #[derive(Clone, Debug, Default)]
    #[non_exhaustive]
    pub struct BlobMetadata {
        pub values: Option<std::collections::HashMap<String, String>>,
        pub encrypted: Option<String>,
    }
    impl Serialize for BlobMetadata {
        fn serialize<S>(&self, serializer: S) -> Result<<S as >::Ok, <S as >::Error> where S: serde::Serializer;
    }
    impl<'de> Deserialize<'de> for BlobMetadata {
        fn deserialize<D>(deserializer: D) -> Result<Self, <D as >::Error> where D: serde::Deserializer<'de>;
    }
    #[derive(Clone, Debug, Default, serde::Deserialize, serde::Serialize)]
    #[non_exhaustive]
    pub struct BlobName {
        #[serde(rename = "$text", skip_serializing_if = "Option::is_none")]
        pub content: Option<String>,
        #[serde(rename = "@Encoded", skip_serializing_if = "Option::is_none")]
        pub encoded: Option<bool>,
    }
    #[derive(Clone, Debug, Default, serde::Deserialize, serde::Serialize)]
    #[non_exhaustive]
    #[serde(rename = "Properties")]
    pub struct BlobProperties {
        #[serde(rename = "AccessTier", skip_serializing_if = "Option::is_none")]
        pub access_tier: Option<super::AccessTier>,
        #[serde(default, rename = "AccessTierChangeTime", skip_serializing_if = "Option::is_none", with = "azure_core::time::rfc7231::option")]
        pub access_tier_change_time: Option<azure_core::time::OffsetDateTime>,
        #[serde(rename = "AccessTierInferred", skip_serializing_if = "Option::is_none")]
        pub access_tier_inferred: Option<bool>,
        #[serde(rename = "ArchiveStatus", skip_serializing_if = "Option::is_none")]
        pub archive_status: Option<super::ArchiveStatus>,
        #[serde(rename = "x-ms-blob-sequence-number", skip_serializing_if = "Option::is_none")]
        pub blob_sequence_number: Option<i64>,
        #[serde(rename = "BlobType", skip_serializing_if = "Option::is_none")]
        pub blob_type: Option<super::BlobType>,
        #[serde(rename = "Cache-Control", skip_serializing_if = "Option::is_none")]
        pub cache_control: Option<String>,
        #[serde(rename = "Content-Disposition", skip_serializing_if = "Option::is_none")]
        pub content_disposition: Option<String>,
        #[serde(rename = "Content-Encoding", skip_serializing_if = "Option::is_none")]
        pub content_encoding: Option<String>,
        #[serde(rename = "Content-Language", skip_serializing_if = "Option::is_none")]
        pub content_language: Option<String>,
        #[serde(rename = "Content-Length", skip_serializing_if = "Option::is_none")]
        pub content_length: Option<u64>,
        #[serde(default, deserialize_with = "base64::option::deserialize", rename = "Content-MD5", serialize_with = "base64::option::serialize", skip_serializing_if = "Option::is_none")]
        pub content_md5: Option<Vec<u8>>,
        #[serde(rename = "Content-Type", skip_serializing_if = "Option::is_none")]
        pub content_type: Option<String>,
        #[serde(default, rename = "CopyCompletionTime", skip_serializing_if = "Option::is_none", with = "azure_core::time::rfc7231::option")]
        pub copy_completion_time: Option<azure_core::time::OffsetDateTime>,
        #[serde(rename = "CopyId", skip_serializing_if = "Option::is_none")]
        pub copy_id: Option<String>,
        #[serde(rename = "CopyProgress", skip_serializing_if = "Option::is_none")]
        pub copy_progress: Option<String>,
        #[serde(rename = "CopySource", skip_serializing_if = "Option::is_none")]
        pub copy_source: Option<String>,
        #[serde(rename = "CopyStatus", skip_serializing_if = "Option::is_none")]
        pub copy_status: Option<super::CopyStatus>,
        #[serde(rename = "CopyStatusDescription", skip_serializing_if = "Option::is_none")]
        pub copy_status_description: Option<String>,
        #[serde(default, rename = "Creation-Time", skip_serializing_if = "Option::is_none", with = "azure_core::time::rfc7231::option")]
        pub creation_time: Option<azure_core::time::OffsetDateTime>,
        #[serde(default, rename = "DeletedTime", skip_serializing_if = "Option::is_none", with = "azure_core::time::rfc7231::option")]
        pub deleted_time: Option<azure_core::time::OffsetDateTime>,
        #[serde(rename = "DestinationSnapshot", skip_serializing_if = "Option::is_none")]
        pub destination_snapshot: Option<String>,
        #[serde(rename = "CustomerProvidedKeySha256", skip_serializing_if = "Option::is_none")]
        pub encryption_key_sha256: Option<String>,
        #[serde(rename = "EncryptionScope", skip_serializing_if = "Option::is_none")]
        pub encryption_scope: Option<String>,
        #[serde(rename = "Etag", skip_serializing_if = "Option::is_none")]
        pub etag: Option<azure_core::http::Etag>,
        #[serde(default, rename = "Expiry-Time", skip_serializing_if = "Option::is_none", with = "azure_core::time::rfc7231::option")]
        pub expires_on: Option<azure_core::time::OffsetDateTime>,
        #[serde(default, rename = "ImmutabilityPolicyUntilDate", skip_serializing_if = "Option::is_none", with = "azure_core::time::rfc7231::option")]
        pub immutability_policy_expires_on: Option<azure_core::time::OffsetDateTime>,
        #[serde(rename = "ImmutabilityPolicyMode", skip_serializing_if = "Option::is_none")]
        pub immutability_policy_mode: Option<super::ImmutabilityPolicyMode>,
        #[serde(rename = "IncrementalCopy", skip_serializing_if = "Option::is_none")]
        pub incremental_copy: Option<bool>,
        #[serde(rename = "Sealed", skip_serializing_if = "Option::is_none")]
        pub is_sealed: Option<bool>,
        #[serde(default, rename = "LastAccessTime", skip_serializing_if = "Option::is_none", with = "azure_core::time::rfc7231::option")]
        pub last_accessed_on: Option<azure_core::time::OffsetDateTime>,
        #[serde(default, rename = "Last-Modified", skip_serializing_if = "Option::is_none", with = "azure_core::time::rfc7231::option")]
        pub last_modified: Option<azure_core::time::OffsetDateTime>,
        #[serde(rename = "LeaseDuration", skip_serializing_if = "Option::is_none")]
        pub lease_duration: Option<super::LeaseDuration>,
        #[serde(rename = "LeaseState", skip_serializing_if = "Option::is_none")]
        pub lease_state: Option<super::LeaseState>,
        #[serde(rename = "LeaseStatus", skip_serializing_if = "Option::is_none")]
        pub lease_status: Option<super::LeaseStatus>,
        #[serde(rename = "LegalHold", skip_serializing_if = "Option::is_none")]
        pub legal_hold: Option<bool>,
        #[serde(rename = "RehydratePriority", skip_serializing_if = "Option::is_none")]
        pub rehydrate_priority: Option<super::RehydratePriority>,
        #[serde(rename = "RemainingRetentionDays", skip_serializing_if = "Option::is_none")]
        pub remaining_retention_days: Option<i32>,
        #[serde(rename = "ServerEncrypted", skip_serializing_if = "Option::is_none")]
        pub server_encrypted: Option<bool>,
        #[serde(rename = "TagCount", skip_serializing_if = "Option::is_none")]
        pub tag_count: Option<i32>,
    }
    #[derive(Clone, Debug, Default)]
    pub struct BlobServiceClientFindBlobsByTagsOptions<'a> {
        pub include: Option<Vec<super::FilterBlobsIncludeItem>>,
        pub marker: Option<String>,
        pub maxresults: Option<i32>,
        pub method_options: azure_core::http::pager::PagerOptions<'a>,
        pub timeout: Option<i32>,
    }
    impl BlobServiceClientFindBlobsByTagsOptions<'_> {
        fn into_owned(self) -> BlobServiceClientFindBlobsByTagsOptions<'static>;
    }
    #[derive(Clone, Debug, Default)]
    pub struct BlobServiceClientGetAccountInfoOptions<'a> {
        pub method_options: azure_core::http::ClientMethodOptions<'a>,
        pub timeout: Option<i32>,
    }
    #[derive(Debug)]
    pub struct BlobServiceClientGetAccountInfoResult;
    #[derive(Clone, Debug, Default)]
    pub struct BlobServiceClientGetPropertiesOptions<'a> {
        pub method_options: azure_core::http::ClientMethodOptions<'a>,
        pub timeout: Option<i32>,
    }
    #[derive(Clone, Debug, Default)]
    pub struct BlobServiceClientGetStatisticsOptions<'a> {
        pub method_options: azure_core::http::ClientMethodOptions<'a>,
        pub timeout: Option<i32>,
    }
    #[derive(Clone, Debug, Default)]
    pub struct BlobServiceClientGetUserDelegationKeyOptions<'a> {
        pub method_options: azure_core::http::ClientMethodOptions<'a>,
        pub timeout: Option<i32>,
    }
    #[derive(Clone, Debug, Default)]
    pub struct BlobServiceClientListContainersOptions<'a> {
        pub include: Option<Vec<super::ListContainersIncludeType>>,
        pub marker: Option<String>,
        pub maxresults: Option<i32>,
        pub method_options: azure_core::http::pager::PagerOptions<'a>,
        pub prefix: Option<String>,
        pub timeout: Option<i32>,
    }
    impl BlobServiceClientListContainersOptions<'_> {
        fn into_owned(self) -> BlobServiceClientListContainersOptions<'static>;
    }
    #[derive(Clone, Debug, Default)]
    pub struct BlobServiceClientSetPropertiesOptions<'a> {
        pub method_options: azure_core::http::ClientMethodOptions<'a>,
        pub timeout: Option<i32>,
    }
    #[derive(Clone, Debug, Default, serde::Deserialize, serde::Serialize)]
    #[serde(rename = "StorageServiceProperties")]
    pub struct BlobServiceProperties {
        #[serde(default, deserialize_with = "CorsCorsRule::unwrap", rename = "Cors", serialize_with = "CorsCorsRule::wrap", skip_serializing_if = "Option::is_none")]
        pub cors: Option<Vec<CorsRule>>,
        #[serde(rename = "DefaultServiceVersion", skip_serializing_if = "Option::is_none")]
        pub default_service_version: Option<String>,
        #[serde(rename = "DeleteRetentionPolicy", skip_serializing_if = "Option::is_none")]
        pub delete_retention_policy: Option<RetentionPolicy>,
        #[serde(rename = "HourMetrics", skip_serializing_if = "Option::is_none")]
        pub hour_metrics: Option<Metrics>,
        #[serde(rename = "Logging", skip_serializing_if = "Option::is_none")]
        pub logging: Option<Logging>,
        #[serde(rename = "MinuteMetrics", skip_serializing_if = "Option::is_none")]
        pub minute_metrics: Option<Metrics>,
        #[serde(rename = "StaticWebsite", skip_serializing_if = "Option::is_none")]
        pub static_website: Option<StaticWebsite>,
    }
    impl TryFrom<BlobServiceProperties> for azure_core::http::RequestContent<super::BlobServiceProperties, azure_core::http::XmlFormat> {
        type Error = Error;
        fn try_from(value: BlobServiceProperties) -> Result<Self>;
    }
    #[derive(Clone, Debug, Default, serde::Deserialize, serde::Serialize)]
    #[serde(rename = "Tag")]
    pub struct BlobTag {
        #[serde(rename = "Key", skip_serializing_if = "Option::is_none")]
        pub key: Option<String>,
        #[serde(rename = "Value", skip_serializing_if = "Option::is_none")]
        pub value: Option<String>,
    }
    #[derive(Clone, Debug, Default, serde::Deserialize, serde::Serialize)]
    #[serde(rename = "Tags")]
    pub struct BlobTags {
        #[serde(default, deserialize_with = "Blob_tag_setBlobTag::unwrap", rename = "TagSet", serialize_with = "Blob_tag_setBlobTag::wrap", skip_serializing_if = "Option::is_none")]
        pub blob_tag_set: Option<Vec<BlobTag>>,
    }
    impl From<BlobTags> for std::collections::HashMap<String, String> {
        fn from(blob_tags: BlobTags) -> Self;
    }
    impl From<HashMap<String, String>> for crate::models::BlobTags {
        fn from(tags: HashMap<String, String>) -> Self;
    }
    impl TryFrom<BlobTags> for azure_core::http::RequestContent<super::BlobTags, azure_core::http::XmlFormat> {
        type Error = Error;
        fn try_from(value: BlobTags) -> Result<Self>;
    }
    #[derive(Clone, Debug, Default, serde::Deserialize, serde::Serialize)]
    #[non_exhaustive]
    pub struct Block {
        #[serde(default, deserialize_with = "base64::option::deserialize", rename = "Name", serialize_with = "base64::option::serialize", skip_serializing_if = "Option::is_none")]
        pub name: Option<Vec<u8>>,
        #[serde(rename = "Size", skip_serializing_if = "Option::is_none")]
        pub size: Option<i64>,
    }
    #[derive(Clone, Debug, Default)]
    pub struct BlockBlobClientCommitBlockListOptions<'a> {
        pub blob_cache_control: Option<String>,
        pub blob_content_disposition: Option<String>,
        pub blob_content_encoding: Option<String>,
        pub blob_content_language: Option<String>,
        pub blob_content_md5: Option<Vec<u8>>,
        pub blob_content_type: Option<String>,
        pub blob_tags_string: Option<String>,
        pub encryption_algorithm: Option<super::EncryptionAlgorithmType>,
        pub encryption_key: Option<String>,
        pub encryption_key_sha256: Option<String>,
        pub encryption_scope: Option<String>,
        pub if_match: Option<azure_core::http::Etag>,
        pub if_modified_since: Option<azure_core::time::OffsetDateTime>,
        pub if_none_match: Option<azure_core::http::Etag>,
        pub if_tags: Option<String>,
        pub if_unmodified_since: Option<azure_core::time::OffsetDateTime>,
        pub immutability_policy_expiry: Option<azure_core::time::OffsetDateTime>,
        pub immutability_policy_mode: Option<super::ImmutabilityPolicyMode>,
        pub lease_id: Option<String>,
        pub legal_hold: Option<bool>,
        pub metadata: Option<std::collections::HashMap<String, String>>,
        pub method_options: azure_core::http::ClientMethodOptions<'a>,
        pub tier: Option<super::AccessTier>,
        pub timeout: Option<i32>,
        pub transactional_content_crc64: Option<Vec<u8>>,
        pub transactional_content_md5: Option<Vec<u8>>,
    }
    impl crate::models::BlockBlobClientCommitBlockListOptions<'_> {
        fn with_tags<impl Into<BlobTags>: Into<BlobTags>>(self, tags: impl Into<BlobTags>) -> Self;
    }
    #[derive(Debug)]
    pub struct BlockBlobClientCommitBlockListResult;
    #[derive(Clone, Debug, Default)]
    pub struct BlockBlobClientGetBlockListOptions<'a> {
        pub if_tags: Option<String>,
        pub lease_id: Option<String>,
        pub method_options: azure_core::http::ClientMethodOptions<'a>,
        pub snapshot: Option<String>,
        pub timeout: Option<i32>,
    }
    #[derive(Clone, Debug, Default)]
    pub struct BlockBlobClientStageBlockFromUrlOptions<'a> {
        pub copy_source_authorization: Option<String>,
        pub encryption_algorithm: Option<super::EncryptionAlgorithmType>,
        pub encryption_key: Option<String>,
        pub encryption_key_sha256: Option<String>,
        pub encryption_scope: Option<String>,
        pub file_request_intent: Option<super::FileShareTokenIntent>,
        pub lease_id: Option<String>,
        pub method_options: azure_core::http::ClientMethodOptions<'a>,
        pub source_content_crc64: Option<Vec<u8>>,
        pub source_content_md5: Option<Vec<u8>>,
        pub source_encryption_algorithm: Option<super::EncryptionAlgorithmType>,
        pub source_encryption_key: Option<String>,
        pub source_encryption_key_sha256: Option<String>,
        pub source_if_match: Option<azure_core::http::Etag>,
        pub source_if_modified_since: Option<azure_core::time::OffsetDateTime>,
        pub source_if_none_match: Option<azure_core::http::Etag>,
        pub source_if_unmodified_since: Option<azure_core::time::OffsetDateTime>,
        pub source_range: Option<crate::models::HttpRange>,
        pub timeout: Option<i32>,
    }
    #[derive(Debug)]
    pub struct BlockBlobClientStageBlockFromUrlResult;
    #[derive(Clone, Debug, Default)]
    pub struct BlockBlobClientStageBlockOptions<'a> {
        pub encryption_algorithm: Option<super::EncryptionAlgorithmType>,
        pub encryption_key: Option<String>,
        pub encryption_key_sha256: Option<String>,
        pub encryption_scope: Option<String>,
        pub lease_id: Option<String>,
        pub method_options: azure_core::http::ClientMethodOptions<'a>,
        pub timeout: Option<i32>,
        pub transactional_content_crc64: Option<Vec<u8>>,
        pub transactional_content_md5: Option<Vec<u8>>,
    }
    #[derive(Debug)]
    pub struct BlockBlobClientStageBlockResult;
    #[derive(Clone, Debug, Default)]
    pub struct BlockBlobClientUploadBlobFromUrlOptions<'a> {
        pub blob_cache_control: Option<String>,
        pub blob_content_disposition: Option<String>,
        pub blob_content_encoding: Option<String>,
        pub blob_content_language: Option<String>,
        pub blob_content_md5: Option<Vec<u8>>,
        pub blob_content_type: Option<String>,
        pub blob_tags_string: Option<String>,
        pub copy_source_authorization: Option<String>,
        pub copy_source_blob_properties: Option<bool>,
        pub copy_source_tags: Option<super::BlobCopySourceTags>,
        pub encryption_algorithm: Option<super::EncryptionAlgorithmType>,
        pub encryption_key: Option<String>,
        pub encryption_key_sha256: Option<String>,
        pub encryption_scope: Option<String>,
        pub file_request_intent: Option<super::FileShareTokenIntent>,
        pub if_match: Option<azure_core::http::Etag>,
        pub if_modified_since: Option<azure_core::time::OffsetDateTime>,
        pub if_none_match: Option<azure_core::http::Etag>,
        pub if_tags: Option<String>,
        pub if_unmodified_since: Option<azure_core::time::OffsetDateTime>,
        pub lease_id: Option<String>,
        pub metadata: Option<std::collections::HashMap<String, String>>,
        pub method_options: azure_core::http::ClientMethodOptions<'a>,
        pub source_content_md5: Option<Vec<u8>>,
        pub source_encryption_algorithm: Option<super::EncryptionAlgorithmType>,
        pub source_encryption_key: Option<String>,
        pub source_encryption_key_sha256: Option<String>,
        pub source_if_match: Option<azure_core::http::Etag>,
        pub source_if_modified_since: Option<azure_core::time::OffsetDateTime>,
        pub source_if_none_match: Option<azure_core::http::Etag>,
        pub source_if_tags: Option<String>,
        pub source_if_unmodified_since: Option<azure_core::time::OffsetDateTime>,
        pub tier: Option<super::AccessTier>,
        pub timeout: Option<i32>,
        pub transactional_content_md5: Option<Vec<u8>>,
    }
    impl crate::models::BlockBlobClientUploadBlobFromUrlOptions<'_> {
        fn if_not_exists(self) -> Self;
    }
    impl crate::models::BlockBlobClientUploadBlobFromUrlOptions<'_> {
        fn with_tags<impl Into<BlobTags>: Into<BlobTags>>(self, tags: impl Into<BlobTags>) -> Self;
    }
    #[derive(Debug)]
    pub struct BlockBlobClientUploadBlobFromUrlResult;
    #[derive(Clone, Debug, Default)]
    pub struct BlockBlobClientUploadOptions<'a> {
        pub blob_cache_control: Option<String>,
        pub blob_content_disposition: Option<String>,
        pub blob_content_encoding: Option<String>,
        pub blob_content_language: Option<String>,
        pub blob_content_md5: Option<Vec<u8>>,
        pub blob_content_type: Option<String>,
        pub blob_tags_string: Option<String>,
        pub encryption_algorithm: Option<crate::models::EncryptionAlgorithmType>,
        pub encryption_key: Option<String>,
        pub encryption_key_sha256: Option<String>,
        pub encryption_scope: Option<String>,
        pub immutability_policy_expiry: Option<time::OffsetDateTime>,
        pub immutability_policy_mode: Option<crate::models::ImmutabilityPolicyMode>,
        pub if_match: Option<azure_core::http::Etag>,
        pub if_modified_since: Option<time::OffsetDateTime>,
        pub if_none_match: Option<azure_core::http::Etag>,
        pub if_tags: Option<String>,
        pub if_unmodified_since: Option<time::OffsetDateTime>,
        pub lease_id: Option<String>,
        pub legal_hold: Option<bool>,
        pub metadata: Option<std::collections::HashMap<String, String>>,
        pub method_options: azure_core::http::ClientMethodOptions<'a>,
        pub parallel: Option<std::num::NonZero<usize>>,
        pub partition_size: Option<std::num::NonZero<u64>>,
        pub per_request_timeout: Option<i32>,
        pub tier: Option<crate::models::AccessTier>,
    }
    impl crate::models::BlockBlobClientUploadOptions<'_> {
        fn if_not_exists(self) -> Self;
    }
    impl crate::models::BlockBlobClientUploadOptions<'_> {
        fn with_tags<impl Into<BlobTags>: Into<BlobTags>>(self, tags: impl Into<BlobTags>) -> Self;
    }
    #[derive(Debug)]
    pub struct BlockBlobClientUploadResult {
        pub etag: Option<azure_core::http::Etag>,
        pub last_modified: Option<time::OffsetDateTime>,
        pub content_md5: Option<Vec<u8>>,
        pub content_crc64: Option<Vec<u8>>,
        pub encryption_key_sha256: Option<String>,
        pub encryption_scope: Option<String>,
        pub is_server_encrypted: Option<bool>,
        pub version_id: Option<String>,
        pub raw_response: azure_core::http::RawResponse,
    }
    #[derive(Clone, Debug, Default, serde::Deserialize, serde::Serialize)]
    #[non_exhaustive]
    pub struct BlockList {
        #[serde(default, deserialize_with = "Committed_blocksBlock::unwrap", rename = "CommittedBlocks", serialize_with = "Committed_blocksBlock::wrap", skip_serializing_if = "Option::is_none")]
        pub committed_blocks: Option<Vec<Block>>,
        #[serde(default, deserialize_with = "Uncommitted_blocksBlock::unwrap", rename = "UncommittedBlocks", serialize_with = "Uncommitted_blocksBlock::wrap", skip_serializing_if = "Option::is_none")]
        pub uncommitted_blocks: Option<Vec<Block>>,
    }
    #[derive(Clone, Debug, Default, serde::Deserialize, serde::Serialize)]
    #[serde(rename = "BlockList")]
    pub struct BlockLookupList {
        #[serde(default, rename = "Committed", skip_serializing_if = "Option::is_none", with = "models_serde::option_vec_encoded_bytes_std")]
        pub committed: Option<Vec<Vec<u8>>>,
        #[serde(default, rename = "Latest", skip_serializing_if = "Option::is_none", with = "models_serde::option_vec_encoded_bytes_std")]
        pub latest: Option<Vec<Vec<u8>>>,
        #[serde(default, rename = "Uncommitted", skip_serializing_if = "Option::is_none", with = "models_serde::option_vec_encoded_bytes_std")]
        pub uncommitted: Option<Vec<Vec<u8>>>,
    }
    impl TryFrom<BlockLookupList> for azure_core::http::RequestContent<super::BlockLookupList, azure_core::http::XmlFormat> {
        type Error = Error;
        fn try_from(value: BlockLookupList) -> Result<Self>;
    }
    #[derive(Clone, Debug, Default, serde::Deserialize, serde::Serialize)]
    #[non_exhaustive]
    pub struct ClearRange {
        #[serde(rename = "End", skip_serializing_if = "Option::is_none")]
        pub end: Option<i64>,
        #[serde(rename = "Start", skip_serializing_if = "Option::is_none")]
        pub start: Option<i64>,
    }
    #[derive(Clone, Debug, Default, serde::Deserialize, serde::Serialize)]
    #[non_exhaustive]
    #[serde(rename = "Container")]
    pub struct ContainerItem {
        #[serde(rename = "Deleted", skip_serializing_if = "Option::is_none")]
        pub deleted: Option<bool>,
        #[serde(rename = "Metadata", skip_serializing_if = "Option::is_none")]
        pub metadata: Option<std::collections::HashMap<String, String>>,
        #[serde(rename = "Name", skip_serializing_if = "Option::is_none")]
        pub name: Option<String>,
        #[serde(rename = "Properties", skip_serializing_if = "Option::is_none")]
        pub properties: Option<ContainerProperties>,
        #[serde(rename = "Version", skip_serializing_if = "Option::is_none")]
        pub version: Option<String>,
    }
    #[derive(Clone, Debug, Default, serde::Deserialize, serde::Serialize)]
    #[non_exhaustive]
    pub struct ContainerProperties {
        #[serde(rename = "DefaultEncryptionScope", skip_serializing_if = "Option::is_none")]
        pub default_encryption_scope: Option<String>,
        #[serde(default, rename = "DeletedTime", skip_serializing_if = "Option::is_none", with = "azure_core::time::rfc7231::option")]
        pub deleted_time: Option<azure_core::time::OffsetDateTime>,
        #[serde(rename = "Etag", skip_serializing_if = "Option::is_none")]
        pub etag: Option<azure_core::http::Etag>,
        #[serde(rename = "HasImmutabilityPolicy", skip_serializing_if = "Option::is_none")]
        pub has_immutability_policy: Option<bool>,
        #[serde(rename = "HasLegalHold", skip_serializing_if = "Option::is_none")]
        pub has_legal_hold: Option<bool>,
        #[serde(rename = "ImmutableStorageWithVersioningEnabled", skip_serializing_if = "Option::is_none")]
        pub is_immutable_storage_with_versioning_enabled: Option<bool>,
        #[serde(default, rename = "Last-Modified", skip_serializing_if = "Option::is_none", with = "azure_core::time::rfc7231::option")]
        pub last_modified: Option<azure_core::time::OffsetDateTime>,
        #[serde(rename = "LeaseDuration", skip_serializing_if = "Option::is_none")]
        pub lease_duration: Option<super::LeaseDuration>,
        #[serde(rename = "LeaseState", skip_serializing_if = "Option::is_none")]
        pub lease_state: Option<super::LeaseState>,
        #[serde(rename = "LeaseStatus", skip_serializing_if = "Option::is_none")]
        pub lease_status: Option<super::LeaseStatus>,
        #[serde(rename = "DenyEncryptionScopeOverride", skip_serializing_if = "Option::is_none")]
        pub prevent_encryption_scope_override: Option<bool>,
        #[serde(rename = "PublicAccess", skip_serializing_if = "Option::is_none")]
        pub public_access: Option<super::PublicAccessType>,
        #[serde(rename = "RemainingRetentionDays", skip_serializing_if = "Option::is_none")]
        pub remaining_retention_days: Option<i32>,
    }
    #[derive(Clone, Debug, Default, serde::Deserialize, serde::Serialize)]
    #[serde(rename = "CorsRule")]
    pub struct CorsRule {
        #[serde(rename = "AllowedHeaders", skip_serializing_if = "Option::is_none")]
        pub allowed_headers: Option<String>,
        #[serde(rename = "AllowedMethods", skip_serializing_if = "Option::is_none")]
        pub allowed_methods: Option<String>,
        #[serde(rename = "AllowedOrigins", skip_serializing_if = "Option::is_none")]
        pub allowed_origins: Option<String>,
        #[serde(rename = "ExposedHeaders", skip_serializing_if = "Option::is_none")]
        pub exposed_headers: Option<String>,
        #[serde(rename = "MaxAgeInSeconds", skip_serializing_if = "Option::is_none")]
        pub max_age_in_seconds: Option<i32>,
    }
    #[derive(Clone, Debug, serde::Deserialize, serde::Serialize)]
    pub struct Error {
        #[serde(rename = "Code", skip_serializing_if = "Option::is_none")]
        pub code: Option<super::StorageErrorCode>,
        #[serde(rename = "CopySourceErrorCode", skip_serializing_if = "Option::is_none")]
        pub copy_source_error_code: Option<String>,
        #[serde(rename = "CopySourceErrorMessage", skip_serializing_if = "Option::is_none")]
        pub copy_source_error_message: Option<String>,
        #[serde(rename = "CopySourceStatusCode", skip_serializing_if = "Option::is_none")]
        pub copy_source_status_code: Option<i32>,
        #[serde(rename = "errorCode", skip_serializing_if = "Option::is_none")]
        pub error_code: Option<String>,
        #[serde(rename = "Message", skip_serializing_if = "Option::is_none")]
        pub message: Option<String>,
        #[serde(rename = "xMsCopySourceErrorCode", skip_serializing_if = "Option::is_none")]
        pub x_ms_copy_source_error_code: Option<String>,
        #[serde(rename = "xMsCopySourceStatusCode", skip_serializing_if = "Option::is_none")]
        pub x_ms_copy_source_status_code: Option<i32>,
    }
    #[derive(Clone, Debug, Default, serde::Deserialize, serde::Serialize)]
    #[non_exhaustive]
    #[serde(rename = "Blob")]
    pub struct FilterBlobItem {
        #[serde(rename = "ContainerName", skip_serializing_if = "Option::is_none")]
        pub container_name: Option<String>,
        #[serde(rename = "IsCurrentVersion", skip_serializing_if = "Option::is_none")]
        pub is_current_version: Option<bool>,
        #[serde(rename = "Name", skip_serializing_if = "Option::is_none")]
        pub name: Option<String>,
        #[serde(rename = "Tags", skip_serializing_if = "Option::is_none")]
        pub tags: Option<BlobTags>,
        #[serde(rename = "VersionId", skip_serializing_if = "Option::is_none")]
        pub version_id: Option<String>,
    }
    #[derive(Clone, Debug, Default, serde::Deserialize, serde::Serialize)]
    #[non_exhaustive]
    #[serde(rename = "EnumerationResults")]
    pub struct FilteredBlobResponse {
        #[serde(default, deserialize_with = "Blob_itemsFilterBlobItem::unwrap", rename = "Blobs", serialize_with = "Blob_itemsFilterBlobItem::wrap")]
        pub blob_items: Vec<FilterBlobItem>,
        #[serde(rename = "NextMarker", skip_serializing_if = "Option::is_none")]
        pub next_marker: Option<String>,
        #[serde(rename = "@ServiceEndpoint", skip_serializing_if = "Option::is_none")]
        pub service_endpoint: Option<String>,
        #[serde(rename = "Where", skip_serializing_if = "Option::is_none")]
        pub where_prop: Option<String>,
    }
    impl Page for super::FilteredBlobResponse {
        type IntoIter = <Vec<FilterBlobItem> as IntoIterator>::IntoIter;
        type Item = FilterBlobItem;
        #[allow(elided_named_lifetimes, clippy::async_yields_async, clippy::diverging_sub_expression, clippy::let_unit_value, clippy::needless_arbitrary_self_type, clippy::no_effect_underscore_binding, clippy::shadow_same, clippy::type_complexity, clippy::type_repetition_in_bounds, clippy::used_underscore_binding)]
        fn into_items(self) -> ::core::pin::Pin<Box<dyn ::core::future::Future<Output = Result<<Self as >::IntoIter>> + ::core::marker::Send>>;
    }
    #[derive(Clone, Debug, Default, serde::Deserialize, serde::Serialize)]
    #[non_exhaustive]
    pub struct GeoReplication {
        #[serde(default, rename = "LastSyncTime", skip_serializing_if = "Option::is_none", with = "azure_core::time::rfc7231::option")]
        pub last_sync_time: Option<azure_core::time::OffsetDateTime>,
        #[serde(rename = "Status", skip_serializing_if = "Option::is_none")]
        pub status: Option<super::GeoReplicationStatusType>,
    }
    #[derive(Clone, Debug, Eq, PartialEq)]
    pub struct HttpRange {
    }
    impl HttpRange {
        fn from_offset(offset: u64) -> Self;
        fn new(offset: u64, length: u64) -> Self;
    }
    impl Display for HttpRange {
        fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result;
    }
    impl From<HttpRange> for azure_core::http::headers::HeaderValue {
        fn from(range: HttpRange) -> Self;
    }
    impl From<Range<u64>> for HttpRange {
        fn from(range: Range<u64>) -> Self;
    }
    impl From<Range<usize>> for HttpRange {
        fn from(range: Range<usize>) -> Self;
    }
    impl From<RangeFrom<u64>> for HttpRange {
        fn from(range: RangeFrom<u64>) -> Self;
    }
    impl From<RangeFrom<usize>> for HttpRange {
        fn from(range: RangeFrom<usize>) -> Self;
    }
    impl From<RangeInclusive<u64>> for HttpRange {
        fn from(range: RangeInclusive<u64>) -> Self;
    }
    impl From<RangeInclusive<usize>> for HttpRange {
        fn from(range: RangeInclusive<usize>) -> Self;
    }
    impl From<RangeTo<u64>> for HttpRange {
        fn from(range: RangeTo<u64>) -> Self;
    }
    impl From<RangeTo<usize>> for HttpRange {
        fn from(range: RangeTo<usize>) -> Self;
    }
    impl From<RangeToInclusive<u64>> for HttpRange {
        fn from(range: RangeToInclusive<u64>) -> Self;
    }
    impl From<RangeToInclusive<usize>> for HttpRange {
        fn from(range: RangeToInclusive<usize>) -> Self;
    }
    #[derive(Clone, Debug, Default, serde::Deserialize, serde::Serialize)]
    pub struct KeyInfo {
        #[serde(rename = "DelegatedUserTid", skip_serializing_if = "Option::is_none")]
        pub delegated_user_tid: Option<String>,
        #[serde(default, deserialize_with = "models_serde::option_offset_date_time_rfc3339::deserialize", rename = "Expiry", serialize_with = "azure_storage_common::rfc3339::seconds_only::option::serialize", skip_serializing_if = "Option::is_none")]
        pub expiry: Option<azure_core::time::OffsetDateTime>,
        #[serde(default, deserialize_with = "models_serde::option_offset_date_time_rfc3339::deserialize", rename = "Start", serialize_with = "azure_storage_common::rfc3339::seconds_only::option::serialize", skip_serializing_if = "Option::is_none")]
        pub start: Option<azure_core::time::OffsetDateTime>,
    }
    impl TryFrom<KeyInfo> for azure_core::http::RequestContent<super::KeyInfo, azure_core::http::XmlFormat> {
        type Error = Error;
        fn try_from(value: KeyInfo) -> Result<Self>;
    }
    #[derive(Clone, Debug, Default, serde::Deserialize, serde::Serialize)]
    #[non_exhaustive]
    #[serde(rename = "EnumerationResults")]
    pub struct ListBlobsResponse {
        #[serde(default, deserialize_with = "Blob_itemsBlobItem::unwrap", rename = "Blobs", serialize_with = "Blob_itemsBlobItem::wrap")]
        pub blob_items: Vec<BlobItem>,
        #[serde(rename = "@ContainerName", skip_serializing_if = "Option::is_none")]
        pub container_name: Option<String>,
        #[serde(rename = "Marker", skip_serializing_if = "Option::is_none")]
        pub marker: Option<String>,
        #[serde(rename = "MaxResults", skip_serializing_if = "Option::is_none")]
        pub max_results: Option<i32>,
        #[serde(rename = "NextMarker", skip_serializing_if = "Option::is_none")]
        pub next_marker: Option<String>,
        #[serde(rename = "Prefix", skip_serializing_if = "Option::is_none")]
        pub prefix: Option<String>,
        #[serde(rename = "@ServiceEndpoint", skip_serializing_if = "Option::is_none")]
        pub service_endpoint: Option<String>,
    }
    impl Page for super::ListBlobsResponse {
        type IntoIter = <Vec<BlobItem> as IntoIterator>::IntoIter;
        type Item = BlobItem;
        #[allow(elided_named_lifetimes, clippy::async_yields_async, clippy::diverging_sub_expression, clippy::let_unit_value, clippy::needless_arbitrary_self_type, clippy::no_effect_underscore_binding, clippy::shadow_same, clippy::type_complexity, clippy::type_repetition_in_bounds, clippy::used_underscore_binding)]
        fn into_items(self) -> ::core::pin::Pin<Box<dyn ::core::future::Future<Output = Result<<Self as >::IntoIter>> + ::core::marker::Send>>;
    }
    #[derive(Clone, Debug, Default, serde::Deserialize, serde::Serialize)]
    #[non_exhaustive]
    #[serde(rename = "EnumerationResults")]
    pub struct ListContainersResponse {
        #[serde(default, deserialize_with = "Container_itemsContainerItem::unwrap", rename = "Containers", serialize_with = "Container_itemsContainerItem::wrap")]
        pub container_items: Vec<ContainerItem>,
        #[serde(rename = "Marker", skip_serializing_if = "Option::is_none")]
        pub marker: Option<String>,
        #[serde(rename = "MaxResults", skip_serializing_if = "Option::is_none")]
        pub max_results: Option<i32>,
        #[serde(rename = "NextMarker", skip_serializing_if = "Option::is_none")]
        pub next_marker: Option<String>,
        #[serde(rename = "Prefix", skip_serializing_if = "Option::is_none")]
        pub prefix: Option<String>,
        #[serde(rename = "@ServiceEndpoint", skip_serializing_if = "Option::is_none")]
        pub service_endpoint: Option<String>,
    }
    impl Page for super::ListContainersResponse {
        type IntoIter = <Vec<ContainerItem> as IntoIterator>::IntoIter;
        type Item = ContainerItem;
        #[allow(elided_named_lifetimes, clippy::async_yields_async, clippy::diverging_sub_expression, clippy::let_unit_value, clippy::needless_arbitrary_self_type, clippy::no_effect_underscore_binding, clippy::shadow_same, clippy::type_complexity, clippy::type_repetition_in_bounds, clippy::used_underscore_binding)]
        fn into_items(self) -> ::core::pin::Pin<Box<dyn ::core::future::Future<Output = Result<<Self as >::IntoIter>> + ::core::marker::Send>>;
    }
    #[derive(Clone, Debug, Default, serde::Deserialize, serde::Serialize)]
    pub struct Logging {
        #[serde(rename = "Delete", skip_serializing_if = "Option::is_none")]
        pub delete: Option<bool>,
        #[serde(rename = "Read", skip_serializing_if = "Option::is_none")]
        pub read: Option<bool>,
        #[serde(rename = "RetentionPolicy", skip_serializing_if = "Option::is_none")]
        pub retention_policy: Option<RetentionPolicy>,
        #[serde(rename = "Version", skip_serializing_if = "Option::is_none")]
        pub version: Option<String>,
        #[serde(rename = "Write", skip_serializing_if = "Option::is_none")]
        pub write: Option<bool>,
    }
    #[derive(Clone, Debug, Default, serde::Deserialize, serde::Serialize)]
    pub struct Metrics {
        #[serde(rename = "Enabled", skip_serializing_if = "Option::is_none")]
        pub enabled: Option<bool>,
        #[serde(rename = "IncludeAPIs", skip_serializing_if = "Option::is_none")]
        pub include_apis: Option<bool>,
        #[serde(rename = "RetentionPolicy", skip_serializing_if = "Option::is_none")]
        pub retention_policy: Option<RetentionPolicy>,
        #[serde(rename = "Version", skip_serializing_if = "Option::is_none")]
        pub version: Option<String>,
    }
    #[derive(Clone, Debug, Default)]
    #[non_exhaustive]
    pub struct ObjectReplicationMetadata {
        pub additional_properties: Option<std::collections::HashMap<String, String>>,
    }
    impl Serialize for super::ObjectReplicationMetadata {
        fn serialize<S>(&self, serializer: S) -> Result<<S as >::Ok, <S as >::Error> where S: serde::Serializer;
    }
    impl<'de> Deserialize<'de> for super::ObjectReplicationMetadata {
        fn deserialize<D>(deserializer: D) -> Result<Self, <D as >::Error> where D: serde::Deserializer<'de>;
    }
    #[derive(Clone, Debug, Default)]
    pub struct PageBlobClientClearPagesOptions<'a> {
        pub encryption_algorithm: Option<super::EncryptionAlgorithmType>,
        pub encryption_key: Option<String>,
        pub encryption_key_sha256: Option<String>,
        pub encryption_scope: Option<String>,
        pub if_match: Option<azure_core::http::Etag>,
        pub if_modified_since: Option<azure_core::time::OffsetDateTime>,
        pub if_none_match: Option<azure_core::http::Etag>,
        pub if_sequence_number_equal_to: Option<i64>,
        pub if_sequence_number_less_than: Option<i64>,
        pub if_sequence_number_less_than_or_equal_to: Option<i64>,
        pub if_tags: Option<String>,
        pub if_unmodified_since: Option<azure_core::time::OffsetDateTime>,
        pub lease_id: Option<String>,
        pub method_options: azure_core::http::ClientMethodOptions<'a>,
        pub timeout: Option<i32>,
    }
    #[derive(Debug)]
    pub struct PageBlobClientClearPagesResult;
    #[derive(Clone, Debug, Default)]
    pub struct PageBlobClientCreateOptions<'a> {
        pub blob_cache_control: Option<String>,
        pub blob_content_disposition: Option<String>,
        pub blob_content_encoding: Option<String>,
        pub blob_content_language: Option<String>,
        pub blob_content_md5: Option<Vec<u8>>,
        pub blob_content_type: Option<String>,
        pub blob_sequence_number: Option<i64>,
        pub blob_tags_string: Option<String>,
        pub encryption_algorithm: Option<super::EncryptionAlgorithmType>,
        pub encryption_key: Option<String>,
        pub encryption_key_sha256: Option<String>,
        pub encryption_scope: Option<String>,
        pub if_match: Option<azure_core::http::Etag>,
        pub if_modified_since: Option<azure_core::time::OffsetDateTime>,
        pub if_none_match: Option<azure_core::http::Etag>,
        pub if_tags: Option<String>,
        pub if_unmodified_since: Option<azure_core::time::OffsetDateTime>,
        pub immutability_policy_expiry: Option<azure_core::time::OffsetDateTime>,
        pub immutability_policy_mode: Option<super::ImmutabilityPolicyMode>,
        pub lease_id: Option<String>,
        pub legal_hold: Option<bool>,
        pub metadata: Option<std::collections::HashMap<String, String>>,
        pub method_options: azure_core::http::ClientMethodOptions<'a>,
        pub tier: Option<super::PremiumPageBlobAccessTier>,
        pub timeout: Option<i32>,
    }
    impl crate::models::PageBlobClientCreateOptions<'_> {
        fn if_not_exists(self) -> Self;
    }
    impl crate::models::PageBlobClientCreateOptions<'_> {
        fn with_tags<impl Into<BlobTags>: Into<BlobTags>>(self, tags: impl Into<BlobTags>) -> Self;
    }
    #[derive(Debug)]
    pub struct PageBlobClientCreateResult;
    #[derive(Clone, Debug, Default)]
    pub struct PageBlobClientListPageRangesOptions<'a> {
        pub if_match: Option<azure_core::http::Etag>,
        pub if_modified_since: Option<azure_core::time::OffsetDateTime>,
        pub if_none_match: Option<azure_core::http::Etag>,
        pub if_tags: Option<String>,
        pub if_unmodified_since: Option<azure_core::time::OffsetDateTime>,
        pub lease_id: Option<String>,
        pub marker: Option<String>,
        pub maxresults: Option<i32>,
        pub method_options: azure_core::http::pager::PagerOptions<'a>,
        pub range: Option<crate::models::HttpRange>,
        pub snapshot: Option<String>,
        pub timeout: Option<i32>,
    }
    impl PageBlobClientListPageRangesOptions<'_> {
        fn into_owned(self) -> PageBlobClientListPageRangesOptions<'static>;
    }
    #[derive(Clone, Debug, Default)]
    pub struct PageBlobClientResizeOptions<'a> {
        pub encryption_algorithm: Option<super::EncryptionAlgorithmType>,
        pub encryption_key: Option<String>,
        pub encryption_key_sha256: Option<String>,
        pub encryption_scope: Option<String>,
        pub if_match: Option<azure_core::http::Etag>,
        pub if_modified_since: Option<azure_core::time::OffsetDateTime>,
        pub if_none_match: Option<azure_core::http::Etag>,
        pub if_tags: Option<String>,
        pub if_unmodified_since: Option<azure_core::time::OffsetDateTime>,
        pub lease_id: Option<String>,
        pub method_options: azure_core::http::ClientMethodOptions<'a>,
        pub timeout: Option<i32>,
    }
    #[derive(Debug)]
    pub struct PageBlobClientResizeResult;
    #[derive(Clone, Debug, Default)]
    pub struct PageBlobClientSetSequenceNumberOptions<'a> {
        pub blob_sequence_number: Option<i64>,
        pub if_match: Option<azure_core::http::Etag>,
        pub if_modified_since: Option<azure_core::time::OffsetDateTime>,
        pub if_none_match: Option<azure_core::http::Etag>,
        pub if_tags: Option<String>,
        pub if_unmodified_since: Option<azure_core::time::OffsetDateTime>,
        pub lease_id: Option<String>,
        pub method_options: azure_core::http::ClientMethodOptions<'a>,
        pub timeout: Option<i32>,
    }
    #[derive(Debug)]
    pub struct PageBlobClientSetSequenceNumberResult;
    #[derive(Clone, Debug, Default)]
    pub struct PageBlobClientUploadPagesFromUrlOptions<'a> {
        pub copy_source_authorization: Option<String>,
        pub encryption_algorithm: Option<super::EncryptionAlgorithmType>,
        pub encryption_key: Option<String>,
        pub encryption_key_sha256: Option<String>,
        pub encryption_scope: Option<String>,
        pub file_request_intent: Option<super::FileShareTokenIntent>,
        pub if_match: Option<azure_core::http::Etag>,
        pub if_modified_since: Option<azure_core::time::OffsetDateTime>,
        pub if_none_match: Option<azure_core::http::Etag>,
        pub if_sequence_number_equal_to: Option<i64>,
        pub if_sequence_number_less_than: Option<i64>,
        pub if_sequence_number_less_than_or_equal_to: Option<i64>,
        pub if_tags: Option<String>,
        pub if_unmodified_since: Option<azure_core::time::OffsetDateTime>,
        pub lease_id: Option<String>,
        pub method_options: azure_core::http::ClientMethodOptions<'a>,
        pub source_content_crc64: Option<Vec<u8>>,
        pub source_content_md5: Option<Vec<u8>>,
        pub source_encryption_algorithm: Option<super::EncryptionAlgorithmType>,
        pub source_encryption_key: Option<String>,
        pub source_encryption_key_sha256: Option<String>,
        pub source_if_match: Option<azure_core::http::Etag>,
        pub source_if_modified_since: Option<azure_core::time::OffsetDateTime>,
        pub source_if_none_match: Option<azure_core::http::Etag>,
        pub source_if_unmodified_since: Option<azure_core::time::OffsetDateTime>,
        pub timeout: Option<i32>,
    }
    #[derive(Debug)]
    pub struct PageBlobClientUploadPagesFromUrlResult;
    #[derive(Clone, Debug, Default)]
    pub struct PageBlobClientUploadPagesOptions<'a> {
        pub encryption_algorithm: Option<super::EncryptionAlgorithmType>,
        pub encryption_key: Option<String>,
        pub encryption_key_sha256: Option<String>,
        pub encryption_scope: Option<String>,
        pub if_match: Option<azure_core::http::Etag>,
        pub if_modified_since: Option<azure_core::time::OffsetDateTime>,
        pub if_none_match: Option<azure_core::http::Etag>,
        pub if_sequence_number_equal_to: Option<i64>,
        pub if_sequence_number_less_than: Option<i64>,
        pub if_sequence_number_less_than_or_equal_to: Option<i64>,
        pub if_tags: Option<String>,
        pub if_unmodified_since: Option<azure_core::time::OffsetDateTime>,
        pub lease_id: Option<String>,
        pub method_options: azure_core::http::ClientMethodOptions<'a>,
        pub timeout: Option<i32>,
        pub transactional_content_crc64: Option<Vec<u8>>,
        pub transactional_content_md5: Option<Vec<u8>>,
    }
    #[derive(Debug)]
    pub struct PageBlobClientUploadPagesResult;
    #[derive(Clone, Debug, Default, serde::Deserialize, serde::Serialize)]
    #[non_exhaustive]
    pub struct PageList {
        #[serde(rename = "ClearRange", skip_serializing_if = "Option::is_none")]
        pub clear_ranges: Option<Vec<ClearRange>>,
        #[serde(rename = "NextMarker", skip_serializing_if = "Option::is_none")]
        pub next_marker: Option<String>,
        #[serde(default, rename = "PageRange")]
        pub page_ranges: Vec<PageRange>,
    }
    #[derive(Clone, Debug, Default, serde::Deserialize, serde::Serialize)]
    #[non_exhaustive]
    pub struct PageRange {
        #[serde(rename = "End", skip_serializing_if = "Option::is_none")]
        pub end: Option<i64>,
        #[serde(rename = "Start", skip_serializing_if = "Option::is_none")]
        pub start: Option<i64>,
    }
    #[derive(Clone, Debug, Default, serde::Deserialize, serde::Serialize)]
    pub struct RetentionPolicy {
        #[serde(rename = "AllowPermanentDelete", skip_serializing_if = "Option::is_none")]
        pub allow_permanent_delete: Option<bool>,
        #[serde(rename = "Days", skip_serializing_if = "Option::is_none")]
        pub days: Option<i32>,
        #[serde(rename = "Enabled", skip_serializing_if = "Option::is_none")]
        pub enabled: Option<bool>,
    }
    #[derive(Clone, Debug, Default, serde::Deserialize, serde::Serialize)]
    #[serde(rename = "SignedIdentifier")]
    pub struct SignedIdentifier {
        #[serde(rename = "AccessPolicy", skip_serializing_if = "Option::is_none")]
        pub access_policy: Option<AccessPolicy>,
        #[serde(rename = "Id", skip_serializing_if = "Option::is_none")]
        pub id: Option<String>,
    }
    #[derive(Clone, Debug, Default, serde::Deserialize, serde::Serialize)]
    pub struct SignedIdentifiers {
        #[serde(rename = "SignedIdentifier", skip_serializing_if = "Option::is_none")]
        pub items: Option<Vec<SignedIdentifier>>,
    }
    impl From<HashMap<String, AccessPolicy>> for crate::models::SignedIdentifiers {
        fn from(policies: HashMap<String, AccessPolicy>) -> Self;
    }
    impl TryFrom<SignedIdentifiers> for azure_core::http::RequestContent<super::SignedIdentifiers, azure_core::http::XmlFormat> {
        type Error = Error;
        fn try_from(value: SignedIdentifiers) -> Result<Self>;
    }
    #[derive(Clone, Debug, Default, serde::Deserialize, serde::Serialize)]
    pub struct StaticWebsite {
        #[serde(rename = "DefaultIndexDocumentPath", skip_serializing_if = "Option::is_none")]
        pub default_index_document_path: Option<String>,
        #[serde(rename = "Enabled", skip_serializing_if = "Option::is_none")]
        pub enabled: Option<bool>,
        #[serde(rename = "ErrorDocument404Path", skip_serializing_if = "Option::is_none")]
        pub error_document404_path: Option<String>,
        #[serde(rename = "IndexDocument", skip_serializing_if = "Option::is_none")]
        pub index_document: Option<String>,
    }
    #[derive(Clone, Debug, Default, serde::Deserialize, serde::Serialize)]
    #[non_exhaustive]
    pub struct StorageServiceStats {
        #[serde(rename = "GeoReplication", skip_serializing_if = "Option::is_none")]
        pub geo_replication: Option<GeoReplication>,
    }
    #[derive(Clone, Debug, Eq, PartialEq)]
    pub enum AccessTier {
        Archive,
        Cold,
        Cool,
        Hot,
        P10,
        P15,
        P20,
        P30,
        P4,
        P40,
        P50,
        P6,
        P60,
        P70,
        P80,
        Premium,
        UnknownValue(String),
    }
    impl AsRef<str> for super::AccessTier {
        fn as_ref(&self) -> &str;
    }
    impl Display for super::AccessTier {
        fn fmt(&self, f: &mut Formatter<'_>) -> ::std::fmt::Result;
    }
    impl FromStr for super::AccessTier {
        type Err = Infallible;
        fn from_str(s: &str) -> ::core::result::Result<Self, <Self as FromStr>::Err>;
    }
    impl Serialize for super::AccessTier {
        fn serialize<S>(&self, s: S) -> ::core::result::Result<<S as >::Ok, <S as >::Error> where S: Serializer;
    }
    impl<'a> From<&'a AccessTier> for &'a str {
        fn from(e: &'a AccessTier) -> Self;
    }
    impl<'de> Deserialize<'de> for super::AccessTier {
        fn deserialize<D>(deserializer: D) -> ::core::result::Result<Self, <D as >::Error> where D: Deserializer<'de>;
    }
    #[derive(Clone, Copy, Debug, Eq, PartialEq)]
    pub enum AccountKind {
        BlobStorage,
        BlockBlobStorage,
        FileStorage,
        Storage,
        StorageV2,
    }
    impl AsRef<str> for super::AccountKind {
        fn as_ref(&self) -> &str;
    }
    impl Display for super::AccountKind {
        fn fmt(&self, f: &mut Formatter<'_>) -> ::std::fmt::Result;
    }
    impl FromStr for super::AccountKind {
        type Err = Error;
        fn from_str(s: &str) -> ::core::result::Result<Self, <Self as FromStr>::Err>;
    }
    impl Serialize for super::AccountKind {
        fn serialize<S>(&self, s: S) -> ::core::result::Result<<S as >::Ok, <S as >::Error> where S: Serializer;
    }
    impl<'de> Deserialize<'de> for super::AccountKind {
        fn deserialize<D>(deserializer: D) -> ::core::result::Result<Self, <D as >::Error> where D: Deserializer<'de>;
    }
    #[derive(Clone, Debug, Eq, PartialEq)]
    pub enum ArchiveStatus {
        RehydratePendingToCold,
        RehydratePendingToCool,
        RehydratePendingToHot,
        UnknownValue(String),
    }
    impl AsRef<str> for super::ArchiveStatus {
        fn as_ref(&self) -> &str;
    }
    impl Display for super::ArchiveStatus {
        fn fmt(&self, f: &mut Formatter<'_>) -> ::std::fmt::Result;
    }
    impl FromStr for super::ArchiveStatus {
        type Err = Infallible;
        fn from_str(s: &str) -> ::core::result::Result<Self, <Self as FromStr>::Err>;
    }
    impl Serialize for super::ArchiveStatus {
        fn serialize<S>(&self, s: S) -> ::core::result::Result<<S as >::Ok, <S as >::Error> where S: Serializer;
    }
    impl<'a> From<&'a ArchiveStatus> for &'a str {
        fn from(e: &'a ArchiveStatus) -> Self;
    }
    impl<'de> Deserialize<'de> for super::ArchiveStatus {
        fn deserialize<D>(deserializer: D) -> ::core::result::Result<Self, <D as >::Error> where D: Deserializer<'de>;
    }
    #[derive(Clone, Copy, Debug, Eq, PartialEq)]
    pub enum BlobCopySourceTags {
        Copy,
        Replace,
    }
    impl AsRef<str> for super::BlobCopySourceTags {
        fn as_ref(&self) -> &str;
    }
    impl Display for super::BlobCopySourceTags {
        fn fmt(&self, f: &mut Formatter<'_>) -> ::std::fmt::Result;
    }
    impl FromStr for super::BlobCopySourceTags {
        type Err = Error;
        fn from_str(s: &str) -> ::core::result::Result<Self, <Self as FromStr>::Err>;
    }
    impl Serialize for super::BlobCopySourceTags {
        fn serialize<S>(&self, s: S) -> ::core::result::Result<<S as >::Ok, <S as >::Error> where S: Serializer;
    }
    impl<'de> Deserialize<'de> for super::BlobCopySourceTags {
        fn deserialize<D>(deserializer: D) -> ::core::result::Result<Self, <D as >::Error> where D: Deserializer<'de>;
    }
    #[derive(Clone, Copy, Debug, Eq, PartialEq)]
    pub enum BlobDeleteType {
        Permanent,
    }
    impl AsRef<str> for super::BlobDeleteType {
        fn as_ref(&self) -> &str;
    }
    impl Display for super::BlobDeleteType {
        fn fmt(&self, f: &mut Formatter<'_>) -> ::std::fmt::Result;
    }
    impl FromStr for super::BlobDeleteType {
        type Err = Error;
        fn from_str(s: &str) -> ::core::result::Result<Self, <Self as FromStr>::Err>;
    }
    impl Serialize for super::BlobDeleteType {
        fn serialize<S>(&self, s: S) -> ::core::result::Result<<S as >::Ok, <S as >::Error> where S: Serializer;
    }
    impl<'de> Deserialize<'de> for super::BlobDeleteType {
        fn deserialize<D>(deserializer: D) -> ::core::result::Result<Self, <D as >::Error> where D: Deserializer<'de>;
    }
    #[derive(Clone, Copy, Debug, Eq, PartialEq)]
    pub enum BlobType {
        AppendBlob,
        BlockBlob,
        PageBlob,
    }
    impl AsRef<str> for super::BlobType {
        fn as_ref(&self) -> &str;
    }
    impl Display for super::BlobType {
        fn fmt(&self, f: &mut Formatter<'_>) -> ::std::fmt::Result;
    }
    impl FromStr for super::BlobType {
        type Err = Error;
        fn from_str(s: &str) -> ::core::result::Result<Self, <Self as FromStr>::Err>;
    }
    impl Serialize for super::BlobType {
        fn serialize<S>(&self, s: S) -> ::core::result::Result<<S as >::Ok, <S as >::Error> where S: Serializer;
    }
    impl<'de> Deserialize<'de> for super::BlobType {
        fn deserialize<D>(deserializer: D) -> ::core::result::Result<Self, <D as >::Error> where D: Deserializer<'de>;
    }
    #[derive(Clone, Copy, Debug, Eq, PartialEq)]
    pub enum BlockListType {
        All,
        Committed,
        Uncommitted,
    }
    impl AsRef<str> for super::BlockListType {
        fn as_ref(&self) -> &str;
    }
    impl Display for super::BlockListType {
        fn fmt(&self, f: &mut Formatter<'_>) -> ::std::fmt::Result;
    }
    impl FromStr for super::BlockListType {
        type Err = Error;
        fn from_str(s: &str) -> ::core::result::Result<Self, <Self as FromStr>::Err>;
    }
    impl Serialize for super::BlockListType {
        fn serialize<S>(&self, s: S) -> ::core::result::Result<<S as >::Ok, <S as >::Error> where S: Serializer;
    }
    impl<'de> Deserialize<'de> for super::BlockListType {
        fn deserialize<D>(deserializer: D) -> ::core::result::Result<Self, <D as >::Error> where D: Deserializer<'de>;
    }
    #[derive(Clone, Copy, Debug, Eq, PartialEq)]
    pub enum CopyStatus {
        Aborted,
        Failed,
        Pending,
        Success,
    }
    impl AsRef<str> for super::CopyStatus {
        fn as_ref(&self) -> &str;
    }
    impl Display for super::CopyStatus {
        fn fmt(&self, f: &mut Formatter<'_>) -> ::std::fmt::Result;
    }
    impl FromStr for super::CopyStatus {
        type Err = Error;
        fn from_str(s: &str) -> ::core::result::Result<Self, <Self as FromStr>::Err>;
    }
    impl Serialize for super::CopyStatus {
        fn serialize<S>(&self, s: S) -> ::core::result::Result<<S as >::Ok, <S as >::Error> where S: Serializer;
    }
    impl<'de> Deserialize<'de> for super::CopyStatus {
        fn deserialize<D>(deserializer: D) -> ::core::result::Result<Self, <D as >::Error> where D: Deserializer<'de>;
    }
    #[derive(Clone, Copy, Debug, Eq, PartialEq)]
    pub enum DeleteSnapshotsOptionType {
        Include,
        Only,
    }
    impl AsRef<str> for super::DeleteSnapshotsOptionType {
        fn as_ref(&self) -> &str;
    }
    impl Display for super::DeleteSnapshotsOptionType {
        fn fmt(&self, f: &mut Formatter<'_>) -> ::std::fmt::Result;
    }
    impl FromStr for super::DeleteSnapshotsOptionType {
        type Err = Error;
        fn from_str(s: &str) -> ::core::result::Result<Self, <Self as FromStr>::Err>;
    }
    impl Serialize for super::DeleteSnapshotsOptionType {
        fn serialize<S>(&self, s: S) -> ::core::result::Result<<S as >::Ok, <S as >::Error> where S: Serializer;
    }
    impl<'de> Deserialize<'de> for super::DeleteSnapshotsOptionType {
        fn deserialize<D>(deserializer: D) -> ::core::result::Result<Self, <D as >::Error> where D: Deserializer<'de>;
    }
    #[derive(Clone, Copy, Debug, Eq, PartialEq)]
    pub enum EncryptionAlgorithmType {
        Aes256,
    }
    impl AsRef<str> for super::EncryptionAlgorithmType {
        fn as_ref(&self) -> &str;
    }
    impl Display for super::EncryptionAlgorithmType {
        fn fmt(&self, f: &mut Formatter<'_>) -> ::std::fmt::Result;
    }
    impl FromStr for super::EncryptionAlgorithmType {
        type Err = Error;
        fn from_str(s: &str) -> ::core::result::Result<Self, <Self as FromStr>::Err>;
    }
    impl Serialize for super::EncryptionAlgorithmType {
        fn serialize<S>(&self, s: S) -> ::core::result::Result<<S as >::Ok, <S as >::Error> where S: Serializer;
    }
    impl<'de> Deserialize<'de> for super::EncryptionAlgorithmType {
        fn deserialize<D>(deserializer: D) -> ::core::result::Result<Self, <D as >::Error> where D: Deserializer<'de>;
    }
    #[derive(Clone, Debug, Eq, PartialEq)]
    pub enum FileShareTokenIntent {
        Backup,
        UnknownValue(String),
    }
    impl AsRef<str> for super::FileShareTokenIntent {
        fn as_ref(&self) -> &str;
    }
    impl Display for super::FileShareTokenIntent {
        fn fmt(&self, f: &mut Formatter<'_>) -> ::std::fmt::Result;
    }
    impl FromStr for super::FileShareTokenIntent {
        type Err = Infallible;
        fn from_str(s: &str) -> ::core::result::Result<Self, <Self as FromStr>::Err>;
    }
    impl Serialize for super::FileShareTokenIntent {
        fn serialize<S>(&self, s: S) -> ::core::result::Result<<S as >::Ok, <S as >::Error> where S: Serializer;
    }
    impl<'a> From<&'a FileShareTokenIntent> for &'a str {
        fn from(e: &'a FileShareTokenIntent) -> Self;
    }
    impl<'de> Deserialize<'de> for super::FileShareTokenIntent {
        fn deserialize<D>(deserializer: D) -> ::core::result::Result<Self, <D as >::Error> where D: Deserializer<'de>;
    }
    #[derive(Clone, Copy, Debug, Eq, PartialEq)]
    pub enum FilterBlobsIncludeItem {
        None,
        Versions,
    }
    impl AsRef<str> for super::FilterBlobsIncludeItem {
        fn as_ref(&self) -> &str;
    }
    impl Display for super::FilterBlobsIncludeItem {
        fn fmt(&self, f: &mut Formatter<'_>) -> ::std::fmt::Result;
    }
    impl FromStr for super::FilterBlobsIncludeItem {
        type Err = Error;
        fn from_str(s: &str) -> ::core::result::Result<Self, <Self as FromStr>::Err>;
    }
    impl Serialize for super::FilterBlobsIncludeItem {
        fn serialize<S>(&self, s: S) -> ::core::result::Result<<S as >::Ok, <S as >::Error> where S: Serializer;
    }
    impl<'de> Deserialize<'de> for super::FilterBlobsIncludeItem {
        fn deserialize<D>(deserializer: D) -> ::core::result::Result<Self, <D as >::Error> where D: Deserializer<'de>;
    }
    #[derive(Clone, Debug, Eq, PartialEq)]
    pub enum GeoReplicationStatusType {
        Bootstrap,
        Live,
        Unavailable,
        UnknownValue(String),
    }
    impl AsRef<str> for super::GeoReplicationStatusType {
        fn as_ref(&self) -> &str;
    }
    impl Display for super::GeoReplicationStatusType {
        fn fmt(&self, f: &mut Formatter<'_>) -> ::std::fmt::Result;
    }
    impl FromStr for super::GeoReplicationStatusType {
        type Err = Infallible;
        fn from_str(s: &str) -> ::core::result::Result<Self, <Self as FromStr>::Err>;
    }
    impl Serialize for super::GeoReplicationStatusType {
        fn serialize<S>(&self, s: S) -> ::core::result::Result<<S as >::Ok, <S as >::Error> where S: Serializer;
    }
    impl<'a> From<&'a GeoReplicationStatusType> for &'a str {
        fn from(e: &'a GeoReplicationStatusType) -> Self;
    }
    impl<'de> Deserialize<'de> for super::GeoReplicationStatusType {
        fn deserialize<D>(deserializer: D) -> ::core::result::Result<Self, <D as >::Error> where D: Deserializer<'de>;
    }
    #[derive(Clone, Copy, Debug, Eq, PartialEq)]
    pub enum ImmutabilityPolicyMode {
        Locked,
        Mutable,
        Unlocked,
    }
    impl AsRef<str> for super::ImmutabilityPolicyMode {
        fn as_ref(&self) -> &str;
    }
    impl Display for super::ImmutabilityPolicyMode {
        fn fmt(&self, f: &mut Formatter<'_>) -> ::std::fmt::Result;
    }
    impl FromStr for super::ImmutabilityPolicyMode {
        type Err = Error;
        fn from_str(s: &str) -> ::core::result::Result<Self, <Self as FromStr>::Err>;
    }
    impl Serialize for super::ImmutabilityPolicyMode {
        fn serialize<S>(&self, s: S) -> ::core::result::Result<<S as >::Ok, <S as >::Error> where S: Serializer;
    }
    impl<'de> Deserialize<'de> for super::ImmutabilityPolicyMode {
        fn deserialize<D>(deserializer: D) -> ::core::result::Result<Self, <D as >::Error> where D: Deserializer<'de>;
    }
    #[derive(Clone, Copy, Debug, Eq, PartialEq)]
    pub enum LeaseDuration {
        Fixed,
        Infinite,
    }
    impl AsRef<str> for super::LeaseDuration {
        fn as_ref(&self) -> &str;
    }
    impl Display for super::LeaseDuration {
        fn fmt(&self, f: &mut Formatter<'_>) -> ::std::fmt::Result;
    }
    impl FromStr for super::LeaseDuration {
        type Err = Error;
        fn from_str(s: &str) -> ::core::result::Result<Self, <Self as FromStr>::Err>;
    }
    impl Serialize for super::LeaseDuration {
        fn serialize<S>(&self, s: S) -> ::core::result::Result<<S as >::Ok, <S as >::Error> where S: Serializer;
    }
    impl<'de> Deserialize<'de> for super::LeaseDuration {
        fn deserialize<D>(deserializer: D) -> ::core::result::Result<Self, <D as >::Error> where D: Deserializer<'de>;
    }
    #[derive(Clone, Copy, Debug, Eq, PartialEq)]
    pub enum LeaseState {
        Available,
        Breaking,
        Broken,
        Expired,
        Leased,
    }
    impl AsRef<str> for super::LeaseState {
        fn as_ref(&self) -> &str;
    }
    impl Display for super::LeaseState {
        fn fmt(&self, f: &mut Formatter<'_>) -> ::std::fmt::Result;
    }
    impl FromStr for super::LeaseState {
        type Err = Error;
        fn from_str(s: &str) -> ::core::result::Result<Self, <Self as FromStr>::Err>;
    }
    impl Serialize for super::LeaseState {
        fn serialize<S>(&self, s: S) -> ::core::result::Result<<S as >::Ok, <S as >::Error> where S: Serializer;
    }
    impl<'de> Deserialize<'de> for super::LeaseState {
        fn deserialize<D>(deserializer: D) -> ::core::result::Result<Self, <D as >::Error> where D: Deserializer<'de>;
    }
    #[derive(Clone, Copy, Debug, Eq, PartialEq)]
    pub enum LeaseStatus {
        Locked,
        Unlocked,
    }
    impl AsRef<str> for super::LeaseStatus {
        fn as_ref(&self) -> &str;
    }
    impl Display for super::LeaseStatus {
        fn fmt(&self, f: &mut Formatter<'_>) -> ::std::fmt::Result;
    }
    impl FromStr for super::LeaseStatus {
        type Err = Error;
        fn from_str(s: &str) -> ::core::result::Result<Self, <Self as FromStr>::Err>;
    }
    impl Serialize for super::LeaseStatus {
        fn serialize<S>(&self, s: S) -> ::core::result::Result<<S as >::Ok, <S as >::Error> where S: Serializer;
    }
    impl<'de> Deserialize<'de> for super::LeaseStatus {
        fn deserialize<D>(deserializer: D) -> ::core::result::Result<Self, <D as >::Error> where D: Deserializer<'de>;
    }
    #[derive(Clone, Copy, Debug, Eq, PartialEq)]
    pub enum ListBlobsIncludeItem {
        Copy,
        Deleted,
        DeletedWithVersions,
        ImmutabilityPolicy,
        LegalHold,
        Metadata,
        Snapshots,
        Tags,
        UncommittedBlobs,
        Versions,
    }
    impl AsRef<str> for super::ListBlobsIncludeItem {
        fn as_ref(&self) -> &str;
    }
    impl Display for super::ListBlobsIncludeItem {
        fn fmt(&self, f: &mut Formatter<'_>) -> ::std::fmt::Result;
    }
    impl FromStr for super::ListBlobsIncludeItem {
        type Err = Error;
        fn from_str(s: &str) -> ::core::result::Result<Self, <Self as FromStr>::Err>;
    }
    impl Serialize for super::ListBlobsIncludeItem {
        fn serialize<S>(&self, s: S) -> ::core::result::Result<<S as >::Ok, <S as >::Error> where S: Serializer;
    }
    impl<'de> Deserialize<'de> for super::ListBlobsIncludeItem {
        fn deserialize<D>(deserializer: D) -> ::core::result::Result<Self, <D as >::Error> where D: Deserializer<'de>;
    }
    #[derive(Clone, Copy, Debug, Eq, PartialEq)]
    pub enum ListContainersIncludeType {
        Deleted,
        Metadata,
        System,
    }
    impl AsRef<str> for super::ListContainersIncludeType {
        fn as_ref(&self) -> &str;
    }
    impl Display for super::ListContainersIncludeType {
        fn fmt(&self, f: &mut Formatter<'_>) -> ::std::fmt::Result;
    }
    impl FromStr for super::ListContainersIncludeType {
        type Err = Error;
        fn from_str(s: &str) -> ::core::result::Result<Self, <Self as FromStr>::Err>;
    }
    impl Serialize for super::ListContainersIncludeType {
        fn serialize<S>(&self, s: S) -> ::core::result::Result<<S as >::Ok, <S as >::Error> where S: Serializer;
    }
    impl<'de> Deserialize<'de> for super::ListContainersIncludeType {
        fn deserialize<D>(deserializer: D) -> ::core::result::Result<Self, <D as >::Error> where D: Deserializer<'de>;
    }
    #[derive(Clone, Debug, Eq, PartialEq)]
    pub enum PremiumPageBlobAccessTier {
        P10,
        P15,
        P20,
        P30,
        P4,
        P40,
        P50,
        P6,
        P60,
        P70,
        P80,
        UnknownValue(String),
    }
    impl AsRef<str> for super::PremiumPageBlobAccessTier {
        fn as_ref(&self) -> &str;
    }
    impl Display for super::PremiumPageBlobAccessTier {
        fn fmt(&self, f: &mut Formatter<'_>) -> ::std::fmt::Result;
    }
    impl FromStr for super::PremiumPageBlobAccessTier {
        type Err = Infallible;
        fn from_str(s: &str) -> ::core::result::Result<Self, <Self as FromStr>::Err>;
    }
    impl Serialize for super::PremiumPageBlobAccessTier {
        fn serialize<S>(&self, s: S) -> ::core::result::Result<<S as >::Ok, <S as >::Error> where S: Serializer;
    }
    impl<'a> From<&'a PremiumPageBlobAccessTier> for &'a str {
        fn from(e: &'a PremiumPageBlobAccessTier) -> Self;
    }
    impl<'de> Deserialize<'de> for super::PremiumPageBlobAccessTier {
        fn deserialize<D>(deserializer: D) -> ::core::result::Result<Self, <D as >::Error> where D: Deserializer<'de>;
    }
    #[derive(Clone, Debug, Eq, PartialEq)]
    pub enum PublicAccessType {
        Blob,
        Container,
        UnknownValue(String),
    }
    impl AsRef<str> for super::PublicAccessType {
        fn as_ref(&self) -> &str;
    }
    impl Display for super::PublicAccessType {
        fn fmt(&self, f: &mut Formatter<'_>) -> ::std::fmt::Result;
    }
    impl FromStr for super::PublicAccessType {
        type Err = Infallible;
        fn from_str(s: &str) -> ::core::result::Result<Self, <Self as FromStr>::Err>;
    }
    impl Serialize for super::PublicAccessType {
        fn serialize<S>(&self, s: S) -> ::core::result::Result<<S as >::Ok, <S as >::Error> where S: Serializer;
    }
    impl<'a> From<&'a PublicAccessType> for &'a str {
        fn from(e: &'a PublicAccessType) -> Self;
    }
    impl<'de> Deserialize<'de> for super::PublicAccessType {
        fn deserialize<D>(deserializer: D) -> ::core::result::Result<Self, <D as >::Error> where D: Deserializer<'de>;
    }
    #[derive(Clone, Debug, Eq, PartialEq)]
    pub enum RehydratePriority {
        High,
        Standard,
        UnknownValue(String),
    }
    impl AsRef<str> for super::RehydratePriority {
        fn as_ref(&self) -> &str;
    }
    impl Display for super::RehydratePriority {
        fn fmt(&self, f: &mut Formatter<'_>) -> ::std::fmt::Result;
    }
    impl FromStr for super::RehydratePriority {
        type Err = Infallible;
        fn from_str(s: &str) -> ::core::result::Result<Self, <Self as FromStr>::Err>;
    }
    impl Serialize for super::RehydratePriority {
        fn serialize<S>(&self, s: S) -> ::core::result::Result<<S as >::Ok, <S as >::Error> where S: Serializer;
    }
    impl<'a> From<&'a RehydratePriority> for &'a str {
        fn from(e: &'a RehydratePriority) -> Self;
    }
    impl<'de> Deserialize<'de> for super::RehydratePriority {
        fn deserialize<D>(deserializer: D) -> ::core::result::Result<Self, <D as >::Error> where D: Deserializer<'de>;
    }
    #[derive(Clone, Copy, Debug, Eq, PartialEq)]
    pub enum SequenceNumberActionType {
        Increment,
        Max,
        Update,
    }
    impl AsRef<str> for super::SequenceNumberActionType {
        fn as_ref(&self) -> &str;
    }
    impl Display for super::SequenceNumberActionType {
        fn fmt(&self, f: &mut Formatter<'_>) -> ::std::fmt::Result;
    }
    impl FromStr for super::SequenceNumberActionType {
        type Err = Error;
        fn from_str(s: &str) -> ::core::result::Result<Self, <Self as FromStr>::Err>;
    }
    impl Serialize for super::SequenceNumberActionType {
        fn serialize<S>(&self, s: S) -> ::core::result::Result<<S as >::Ok, <S as >::Error> where S: Serializer;
    }
    impl<'de> Deserialize<'de> for super::SequenceNumberActionType {
        fn deserialize<D>(deserializer: D) -> ::core::result::Result<Self, <D as >::Error> where D: Deserializer<'de>;
    }
    #[derive(Clone, Copy, Debug, Eq, PartialEq)]
    pub enum SkuName {
        PremiumLrs,
        PremiumZrs,
        StandardGrs,
        StandardGzrs,
        StandardLrs,
        StandardRagrs,
        StandardRagzrs,
        StandardZrs,
    }
    impl AsRef<str> for super::SkuName {
        fn as_ref(&self) -> &str;
    }
    impl Display for super::SkuName {
        fn fmt(&self, f: &mut Formatter<'_>) -> ::std::fmt::Result;
    }
    impl FromStr for super::SkuName {
        type Err = Error;
        fn from_str(s: &str) -> ::core::result::Result<Self, <Self as FromStr>::Err>;
    }
    impl Serialize for super::SkuName {
        fn serialize<S>(&self, s: S) -> ::core::result::Result<<S as >::Ok, <S as >::Error> where S: Serializer;
    }
    impl<'de> Deserialize<'de> for super::SkuName {
        fn deserialize<D>(deserializer: D) -> ::core::result::Result<Self, <D as >::Error> where D: Deserializer<'de>;
    }
    #[derive(Clone, Debug, Eq, PartialEq)]
    pub enum StorageErrorCode {
        AccountAlreadyExists,
        AccountBeingCreated,
        AccountIsDisabled,
        AppendPositionConditionNotMet,
        AuthenticationFailed,
        AuthorizationFailure,
        AuthorizationPermissionMismatch,
        AuthorizationProtocolMismatch,
        AuthorizationResourceTypeMismatch,
        AuthorizationServiceMismatch,
        AuthorizationSourceIPMismatch,
        BlobAccessTierNotSupportedForAccountType,
        BlobAlreadyExists,
        BlobArchived,
        BlobBeingRehydrated,
        BlobImmutableDueToPolicy,
        BlobNotArchived,
        BlobNotFound,
        BlobOverwritten,
        BlobTierInadequateForContentLength,
        BlobUsesCustomerSpecifiedEncryption,
        BlockCountExceedsLimit,
        BlockListTooLong,
        CannotChangeToLowerTier,
        CannotVerifyCopySource,
        ConditionHeadersNotSupported,
        ConditionNotMet,
        ContainerAlreadyExists,
        ContainerBeingDeleted,
        ContainerDisabled,
        ContainerNotFound,
        ContentLengthLargerThanTierLimit,
        CopyAcrossAccountsNotSupported,
        CopyIdMismatch,
        EmptyMetadataKey,
        FeatureVersionMismatch,
        IncrementalCopyBlobMismatch,
        IncrementalCopyOfEarlierSnapshotNotAllowed,
        IncrementalCopyOfEarlierVersionSnapshotNotAllowed,
        IncrementalCopySourceMustBeSnapshot,
        InfiniteLeaseDurationRequired,
        InsufficientAccountPermissions,
        InternalError,
        InvalidAuthenticationInfo,
        InvalidBlobOrBlock,
        InvalidBlobTier,
        InvalidBlobType,
        InvalidBlockId,
        InvalidBlockList,
        InvalidHeaderValue,
        InvalidHttpVerb,
        InvalidInput,
        InvalidMd5,
        InvalidMetadata,
        InvalidOperation,
        InvalidPageRange,
        InvalidQueryParameterValue,
        InvalidRange,
        InvalidRequestUrl,
        InvalidResourceName,
        InvalidSourceBlobType,
        InvalidSourceBlobUrl,
        InvalidUri,
        InvalidVersionForPageBlobOperation,
        InvalidXmlDocument,
        InvalidXmlNodeValue,
        LeaseAlreadyBroken,
        LeaseAlreadyPresent,
        LeaseIdMismatchWithBlobOperation,
        LeaseIdMismatchWithContainerOperation,
        LeaseIdMismatchWithLeaseOperation,
        LeaseIdMissing,
        LeaseIsBreakingAndCannotBeAcquired,
        LeaseIsBreakingAndCannotBeChanged,
        LeaseIsBrokenAndCannotBeRenewed,
        LeaseLost,
        LeaseNotPresentWithBlobOperation,
        LeaseNotPresentWithContainerOperation,
        LeaseNotPresentWithLeaseOperation,
        MaxBlobSizeConditionNotMet,
        Md5Mismatch,
        MetadataTooLarge,
        MissingContentLengthHeader,
        MissingRequiredHeader,
        MissingRequiredQueryParameter,
        MissingRequiredXmlNode,
        MultipleConditionHeadersNotSupported,
        NoAuthenticationInformation,
        NoPendingCopyOperation,
        OperationNotAllowedOnIncrementalCopyBlob,
        OperationTimedOut,
        OutOfRangeInput,
        OutOfRangeQueryParameterValue,
        PendingCopyOperation,
        PreviousSnapshotCannotBeNewer,
        PreviousSnapshotNotFound,
        PreviousSnapshotOperationNotSupported,
        RequestBodyTooLarge,
        RequestUrlFailedToParse,
        ResourceAlreadyExists,
        ResourceNotFound,
        ResourceTypeMismatch,
        SequenceNumberConditionNotMet,
        SequenceNumberIncrementTooLarge,
        ServerBusy,
        SnapshotCountExceeded,
        SnapshotOperationRateExceeded,
        SnapshotsPresent,
        SourceConditionNotMet,
        SystemInUse,
        TargetConditionNotMet,
        UnauthorizedBlobOverwrite,
        UnsupportedHeader,
        UnsupportedHttpVerb,
        UnsupportedQueryParameter,
        UnsupportedXmlNode,
        UnknownValue(String),
    }
    impl AsRef<str> for super::StorageErrorCode {
        fn as_ref(&self) -> &str;
    }
    impl Display for super::StorageErrorCode {
        fn fmt(&self, f: &mut Formatter<'_>) -> ::std::fmt::Result;
    }
    impl FromStr for super::StorageErrorCode {
        type Err = Infallible;
        fn from_str(s: &str) -> ::core::result::Result<Self, <Self as FromStr>::Err>;
    }
    impl Serialize for super::StorageErrorCode {
        fn serialize<S>(&self, s: S) -> ::core::result::Result<<S as >::Ok, <S as >::Error> where S: Serializer;
    }
    impl<'a> From<&'a StorageErrorCode> for &'a str {
        fn from(e: &'a StorageErrorCode) -> Self;
    }
    impl<'de> Deserialize<'de> for super::StorageErrorCode {
        fn deserialize<D>(deserializer: D) -> ::core::result::Result<Self, <D as >::Error> where D: Deserializer<'de>;
    }
    pub trait AppendBlobClientAppendBlockFromUrlResultHeaders: private::Sealed {
        fn blob_append_offset(&self) -> Result<Option<String>>;
        fn blob_committed_block_count(&self) -> Result<Option<i32>>;
        fn content_crc64(&self) -> Result<Option<Vec<u8>>>;
        fn content_md5(&self) -> Result<Option<Vec<u8>>>;
        fn encryption_key_sha256(&self) -> Result<Option<String>>;
        fn encryption_scope(&self) -> Result<Option<String>>;
        fn etag(&self) -> Result<Option<Etag>>;
        fn is_server_encrypted(&self) -> Result<Option<bool>>;
        fn last_modified(&self) -> Result<Option<OffsetDateTime>>;
    }
    pub trait AppendBlobClientAppendBlockResultHeaders: private::Sealed {
        fn blob_append_offset(&self) -> Result<Option<String>>;
        fn blob_committed_block_count(&self) -> Result<Option<i32>>;
        fn content_crc64(&self) -> Result<Option<Vec<u8>>>;
        fn content_md5(&self) -> Result<Option<Vec<u8>>>;
        fn encryption_key_sha256(&self) -> Result<Option<String>>;
        fn encryption_scope(&self) -> Result<Option<String>>;
        fn etag(&self) -> Result<Option<Etag>>;
        fn is_server_encrypted(&self) -> Result<Option<bool>>;
        fn last_modified(&self) -> Result<Option<OffsetDateTime>>;
    }
    pub trait AppendBlobClientCreateResultHeaders: private::Sealed {
        fn content_md5(&self) -> Result<Option<Vec<u8>>>;
        fn encryption_key_sha256(&self) -> Result<Option<String>>;
        fn encryption_scope(&self) -> Result<Option<String>>;
        fn etag(&self) -> Result<Option<Etag>>;
        fn is_server_encrypted(&self) -> Result<Option<bool>>;
        fn last_modified(&self) -> Result<Option<OffsetDateTime>>;
        fn version_id(&self) -> Result<Option<String>>;
    }
    pub trait AppendBlobClientSealResultHeaders: private::Sealed {
        fn etag(&self) -> Result<Option<Etag>>;
        fn is_sealed(&self) -> Result<Option<bool>>;
        fn last_modified(&self) -> Result<Option<OffsetDateTime>>;
    }
    pub trait BlobClientAcquireLeaseResultHeaders: private::Sealed {
        fn etag(&self) -> Result<Option<Etag>>;
        fn last_modified(&self) -> Result<Option<OffsetDateTime>>;
        fn lease_id(&self) -> Result<Option<String>>;
    }
    pub trait BlobClientBreakLeaseResultHeaders: private::Sealed {
        fn etag(&self) -> Result<Option<Etag>>;
        fn last_modified(&self) -> Result<Option<OffsetDateTime>>;
        fn lease_time(&self) -> Result<Option<i32>>;
    }
    pub trait BlobClientChangeLeaseResultHeaders: private::Sealed {
        fn etag(&self) -> Result<Option<Etag>>;
        fn last_modified(&self) -> Result<Option<OffsetDateTime>>;
        fn lease_id(&self) -> Result<Option<String>>;
    }
    pub trait BlobClientCreateSnapshotResultHeaders: private::Sealed {
        fn etag(&self) -> Result<Option<Etag>>;
        fn is_server_encrypted(&self) -> Result<Option<bool>>;
        fn last_modified(&self) -> Result<Option<OffsetDateTime>>;
        fn snapshot(&self) -> Result<Option<String>>;
        fn version_id(&self) -> Result<Option<String>>;
    }
    pub trait BlobClientGetAccountInfoResultHeaders: private::Sealed {
        fn account_kind(&self) -> Result<Option<AccountKind>>;
        fn is_hierarchical_namespace_enabled(&self) -> Result<Option<bool>>;
        fn sku_name(&self) -> Result<Option<SkuName>>;
    }
    pub trait BlobClientGetPropertiesResultHeaders: private::Sealed {
        fn access_tier(&self) -> Result<Option<String>>;
        fn access_tier_change_time(&self) -> Result<Option<OffsetDateTime>>;
        fn access_tier_inferred(&self) -> Result<Option<bool>>;
        fn archive_status(&self) -> Result<Option<ArchiveStatus>>;
        fn blob_committed_block_count(&self) -> Result<Option<i32>>;
        fn blob_sequence_number(&self) -> Result<Option<i64>>;
        fn blob_type(&self) -> Result<Option<BlobType>>;
        fn cache_control(&self) -> Result<Option<String>>;
        fn content_disposition(&self) -> Result<Option<String>>;
        fn content_encoding(&self) -> Result<Option<String>>;
        fn content_language(&self) -> Result<Option<String>>;
        fn content_length(&self) -> Result<Option<u64>>;
        fn content_md5(&self) -> Result<Option<Vec<u8>>>;
        fn content_type(&self) -> Result<Option<String>>;
        fn copy_completion_time(&self) -> Result<Option<OffsetDateTime>>;
        fn copy_id(&self) -> Result<Option<String>>;
        fn copy_progress(&self) -> Result<Option<String>>;
        fn copy_source(&self) -> Result<Option<String>>;
        fn copy_status(&self) -> Result<Option<CopyStatus>>;
        fn copy_status_description(&self) -> Result<Option<String>>;
        fn creation_time(&self) -> Result<Option<OffsetDateTime>>;
        fn destination_snapshot(&self) -> Result<Option<String>>;
        fn duration(&self) -> Result<Option<LeaseDuration>>;
        fn encryption_key_sha256(&self) -> Result<Option<String>>;
        fn encryption_scope(&self) -> Result<Option<String>>;
        fn etag(&self) -> Result<Option<Etag>>;
        fn expires_on(&self) -> Result<Option<OffsetDateTime>>;
        fn immutability_policy_expires_on(&self) -> Result<Option<OffsetDateTime>>;
        fn immutability_policy_mode(&self) -> Result<Option<ImmutabilityPolicyMode>>;
        fn is_current_version(&self) -> Result<Option<bool>>;
        fn is_incremental_copy(&self) -> Result<Option<bool>>;
        fn is_sealed(&self) -> Result<Option<bool>>;
        fn is_server_encrypted(&self) -> Result<Option<bool>>;
        fn last_accessed(&self) -> Result<Option<OffsetDateTime>>;
        fn last_modified(&self) -> Result<Option<OffsetDateTime>>;
        fn lease_state(&self) -> Result<Option<LeaseState>>;
        fn lease_status(&self) -> Result<Option<LeaseStatus>>;
        fn legal_hold(&self) -> Result<Option<bool>>;
        fn metadata(&self) -> Result<HashMap<String, String>>;
        fn object_replication_policy_id(&self) -> Result<Option<String>>;
        fn object_replication_rules(&self) -> Result<HashMap<String, String>>;
        fn rehydrate_priority(&self) -> Result<Option<RehydratePriority>>;
        fn tag_count(&self) -> Result<Option<i64>>;
        fn version_id(&self) -> Result<Option<String>>;
    }
    pub trait BlobClientReleaseLeaseResultHeaders: private::Sealed {
        fn etag(&self) -> Result<Option<Etag>>;
        fn last_modified(&self) -> Result<Option<OffsetDateTime>>;
    }
    pub trait BlobClientRenewLeaseResultHeaders: private::Sealed {
        fn etag(&self) -> Result<Option<Etag>>;
        fn last_modified(&self) -> Result<Option<OffsetDateTime>>;
        fn lease_id(&self) -> Result<Option<String>>;
    }
    pub trait BlobClientStartCopyFromUrlResultHeaders: private::Sealed {
        fn copy_id(&self) -> Result<Option<String>>;
        fn copy_status(&self) -> Result<Option<CopyStatus>>;
        fn etag(&self) -> Result<Option<Etag>>;
        fn last_modified(&self) -> Result<Option<OffsetDateTime>>;
        fn version_id(&self) -> Result<Option<String>>;
    }
    pub trait BlobContainerClientAcquireLeaseResultHeaders: private::Sealed {
        fn etag(&self) -> Result<Option<Etag>>;
        fn last_modified(&self) -> Result<Option<OffsetDateTime>>;
        fn lease_id(&self) -> Result<Option<String>>;
    }
    pub trait BlobContainerClientBreakLeaseResultHeaders: private::Sealed {
        fn etag(&self) -> Result<Option<Etag>>;
        fn last_modified(&self) -> Result<Option<OffsetDateTime>>;
        fn lease_time(&self) -> Result<Option<i32>>;
    }
    pub trait BlobContainerClientChangeLeaseResultHeaders: private::Sealed {
        fn etag(&self) -> Result<Option<Etag>>;
        fn last_modified(&self) -> Result<Option<OffsetDateTime>>;
        fn lease_id(&self) -> Result<Option<String>>;
    }
    pub trait BlobContainerClientGetAccountInfoResultHeaders: private::Sealed {
        fn account_kind(&self) -> Result<Option<AccountKind>>;
        fn is_hierarchical_namespace_enabled(&self) -> Result<Option<bool>>;
        fn sku_name(&self) -> Result<Option<SkuName>>;
    }
    pub trait BlobContainerClientGetPropertiesResultHeaders: private::Sealed {
        fn access(&self) -> Result<Option<PublicAccessType>>;
        fn default_encryption_scope(&self) -> Result<Option<String>>;
        fn duration(&self) -> Result<Option<LeaseDuration>>;
        fn etag(&self) -> Result<Option<Etag>>;
        fn has_immutability_policy(&self) -> Result<Option<bool>>;
        fn has_legal_hold(&self) -> Result<Option<bool>>;
        fn is_immutable_storage_with_versioning_enabled(&self) -> Result<Option<bool>>;
        fn last_modified(&self) -> Result<Option<OffsetDateTime>>;
        fn lease_state(&self) -> Result<Option<LeaseState>>;
        fn lease_status(&self) -> Result<Option<LeaseStatus>>;
        fn metadata(&self) -> Result<HashMap<String, String>>;
        fn prevent_encryption_scope_override(&self) -> Result<Option<bool>>;
    }
    pub trait BlobContainerClientReleaseLeaseResultHeaders: private::Sealed {
        fn etag(&self) -> Result<Option<Etag>>;
        fn last_modified(&self) -> Result<Option<OffsetDateTime>>;
    }
    pub trait BlobContainerClientRenewLeaseResultHeaders: private::Sealed {
        fn etag(&self) -> Result<Option<Etag>>;
        fn last_modified(&self) -> Result<Option<OffsetDateTime>>;
        fn lease_id(&self) -> Result<Option<String>>;
    }
    pub trait BlobServiceClientGetAccountInfoResultHeaders: private::Sealed {
        fn account_kind(&self) -> Result<Option<AccountKind>>;
        fn is_hierarchical_namespace_enabled(&self) -> Result<Option<bool>>;
        fn sku_name(&self) -> Result<Option<SkuName>>;
    }
    pub trait BlockBlobClientCommitBlockListResultHeaders: private::Sealed {
        fn content_crc64(&self) -> Result<Option<Vec<u8>>>;
        fn content_md5(&self) -> Result<Option<Vec<u8>>>;
        fn encryption_key_sha256(&self) -> Result<Option<String>>;
        fn encryption_scope(&self) -> Result<Option<String>>;
        fn etag(&self) -> Result<Option<Etag>>;
        fn is_server_encrypted(&self) -> Result<Option<bool>>;
        fn last_modified(&self) -> Result<Option<OffsetDateTime>>;
        fn version_id(&self) -> Result<Option<String>>;
    }
    pub trait BlockBlobClientStageBlockFromUrlResultHeaders: private::Sealed {
        fn content_crc64(&self) -> Result<Option<Vec<u8>>>;
        fn content_md5(&self) -> Result<Option<Vec<u8>>>;
        fn encryption_key_sha256(&self) -> Result<Option<String>>;
        fn encryption_scope(&self) -> Result<Option<String>>;
        fn is_server_encrypted(&self) -> Result<Option<bool>>;
    }
    pub trait BlockBlobClientStageBlockResultHeaders: private::Sealed {
        fn content_crc64(&self) -> Result<Option<Vec<u8>>>;
        fn content_md5(&self) -> Result<Option<Vec<u8>>>;
        fn encryption_key_sha256(&self) -> Result<Option<String>>;
        fn encryption_scope(&self) -> Result<Option<String>>;
        fn is_server_encrypted(&self) -> Result<Option<bool>>;
    }
    pub trait BlockBlobClientUploadBlobFromUrlResultHeaders: private::Sealed {
        fn content_md5(&self) -> Result<Option<Vec<u8>>>;
        fn encryption_key_sha256(&self) -> Result<Option<String>>;
        fn encryption_scope(&self) -> Result<Option<String>>;
        fn etag(&self) -> Result<Option<Etag>>;
        fn is_server_encrypted(&self) -> Result<Option<bool>>;
        fn last_modified(&self) -> Result<Option<OffsetDateTime>>;
        fn version_id(&self) -> Result<Option<String>>;
    }
    pub trait BlockListHeaders: private::Sealed {
        fn blob_content_length(&self) -> Result<Option<i64>>;
        fn etag(&self) -> Result<Option<Etag>>;
        fn last_modified(&self) -> Result<Option<OffsetDateTime>>;
    }
    pub trait PageBlobClientClearPagesResultHeaders: private::Sealed {
        fn blob_sequence_number(&self) -> Result<Option<i64>>;
        fn content_crc64(&self) -> Result<Option<Vec<u8>>>;
        fn content_md5(&self) -> Result<Option<Vec<u8>>>;
        fn etag(&self) -> Result<Option<Etag>>;
        fn last_modified(&self) -> Result<Option<OffsetDateTime>>;
    }
    pub trait PageBlobClientCreateResultHeaders: private::Sealed {
        fn content_md5(&self) -> Result<Option<Vec<u8>>>;
        fn encryption_key_sha256(&self) -> Result<Option<String>>;
        fn encryption_scope(&self) -> Result<Option<String>>;
        fn etag(&self) -> Result<Option<Etag>>;
        fn is_server_encrypted(&self) -> Result<Option<bool>>;
        fn last_modified(&self) -> Result<Option<OffsetDateTime>>;
        fn version_id(&self) -> Result<Option<String>>;
    }
    pub trait PageBlobClientResizeResultHeaders: private::Sealed {
        fn blob_sequence_number(&self) -> Result<Option<i64>>;
        fn etag(&self) -> Result<Option<Etag>>;
        fn last_modified(&self) -> Result<Option<OffsetDateTime>>;
    }
    pub trait PageBlobClientSetSequenceNumberResultHeaders: private::Sealed {
        fn blob_sequence_number(&self) -> Result<Option<i64>>;
        fn etag(&self) -> Result<Option<Etag>>;
        fn last_modified(&self) -> Result<Option<OffsetDateTime>>;
    }
    pub trait PageBlobClientUploadPagesFromUrlResultHeaders: private::Sealed {
        fn blob_sequence_number(&self) -> Result<Option<i64>>;
        fn content_crc64(&self) -> Result<Option<Vec<u8>>>;
        fn content_md5(&self) -> Result<Option<Vec<u8>>>;
        fn encryption_key_sha256(&self) -> Result<Option<String>>;
        fn encryption_scope(&self) -> Result<Option<String>>;
        fn etag(&self) -> Result<Option<Etag>>;
        fn is_server_encrypted(&self) -> Result<Option<bool>>;
        fn last_modified(&self) -> Result<Option<OffsetDateTime>>;
    }
    pub trait PageBlobClientUploadPagesResultHeaders: private::Sealed {
        fn blob_sequence_number(&self) -> Result<Option<i64>>;
        fn content_crc64(&self) -> Result<Option<Vec<u8>>>;
        fn content_md5(&self) -> Result<Option<Vec<u8>>>;
        fn encryption_key_sha256(&self) -> Result<Option<String>>;
        fn encryption_scope(&self) -> Result<Option<String>>;
        fn etag(&self) -> Result<Option<Etag>>;
        fn is_server_encrypted(&self) -> Result<Option<bool>>;
        fn last_modified(&self) -> Result<Option<OffsetDateTime>>;
    }
    pub trait PageListHeaders: private::Sealed {
        fn blob_content_length(&self) -> Result<Option<i64>>;
        fn etag(&self) -> Result<Option<Etag>>;
        fn last_modified(&self) -> Result<Option<OffsetDateTime>>;
    }
    pub trait SignedIdentifiersHeaders: private::Sealed {
        fn access(&self) -> Result<Option<PublicAccessType>>;
        fn etag(&self) -> Result<Option<Etag>>;
        fn last_modified(&self) -> Result<Option<OffsetDateTime>>;
    }
}
pub mod stream {
    #[cfg(feature = "tokio")]
    #[cfg_attr(docsrs, doc(cfg(feature = "tokio")))]
    pub mod tokio {
        #[derive(Clone, Debug)]
        pub struct FileStream {
        }
        impl FileStream {
            fn builder(file: File) -> FileStreamBuilder;
        }
        impl AsyncRead for FileStream {
            fn poll_read(self: Pin<&mut Self>, cx: &mut Context<'_>, buf: &mut [u8]) -> Poll<std::io::Result<usize>>;
        }
        impl From<FileStream> for azure_core::http::Body {
            fn from(stream: FileStream) -> Self;
        }
        impl SeekableStream for FileStream {
            fn buffer_size(&self) -> usize;
            fn len(&self) -> Option<u64>;
            #[allow(elided_named_lifetimes, clippy::async_yields_async, clippy::diverging_sub_expression, clippy::let_unit_value, clippy::needless_arbitrary_self_type, clippy::no_effect_underscore_binding, clippy::shadow_same, clippy::type_complexity, clippy::type_repetition_in_bounds, clippy::used_underscore_binding)]
            fn reset(&mut self) -> ::core::pin::Pin<Box<dyn ::core::future::Future<Output = azure_core::Result<()>> + ::core::marker::Send>>;
        }
        #[derive(Debug)]
        pub struct FileStreamBuilder {
        }
        impl FileStreamBuilder {
            async fn build(self) -> azure_core::Result<FileStream>;
            fn with_buffer_size(self, buffer_size: usize) -> Self;
        }
    }
}
```

# azure_storage_sas

- **Description**: User delegation Shared Access Signature (SAS) builder for Microsoft Azure Storage services
- **Edition**: 2021
- **Rust version**: 1.88

## Features

- `default`
  - `azure_core/default`

```rust
#![crate_name = "azure_storage_sas"]
#![crate_type = "lib"]
#[derive(Clone, Debug)]
pub struct SasToken(/* private fields */);
impl Deref for SasToken {
    type Target = str;
    fn deref(&self) -> &str;
}
impl Display for SasToken {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result;
}
impl From<SasToken> for String {
    fn from(t: SasToken) -> String;
}
pub struct SasTokenBuilder<'a, S = Untyped> {
}
#[allow(private_bounds)]
impl<S: SasResource> SasTokenBuilder<'_, S> {
    pub fn build(&self) -> SasToken;
}
impl<S> SasTokenBuilder<'_, S> {
    pub fn delegated_user_object_id<impl Into<String>: Into<String>>(self, value: impl Into<String>) -> Self;
    pub fn ip_range(self, ip: SasIpRange) -> Self;
    pub fn protocol(self, protocol: SasProtocol) -> Self;
    pub fn start(self, start: OffsetDateTime) -> Self;
}
#[allow(private_bounds)]
impl<S: BlobServiceState + BlobOptions> crate::builder::SasTokenBuilder<'_, S> {
    pub fn authorized_object_id<impl Into<String>: Into<String>>(self, value: impl Into<String>) -> Self;
    pub fn cache_control<impl Into<String>: Into<String>>(self, value: impl Into<String>) -> Self;
    pub fn content_disposition<impl Into<String>: Into<String>>(self, value: impl Into<String>) -> Self;
    pub fn content_encoding<impl Into<String>: Into<String>>(self, value: impl Into<String>) -> Self;
    pub fn content_language<impl Into<String>: Into<String>>(self, value: impl Into<String>) -> Self;
    pub fn content_type<impl Into<String>: Into<String>>(self, value: impl Into<String>) -> Self;
    pub fn correlation_id<impl Into<String>: Into<String>>(self, value: impl Into<String>) -> Self;
    pub fn encryption_scope<impl Into<String>: Into<String>>(self, scope: impl Into<String>) -> Self;
    pub fn signed_request_header<impl Into<String>: Into<String>, impl Into<String>: Into<String>>(self, key: impl Into<String>, value: impl Into<String>) -> Self;
    pub fn signed_request_query_parameter<impl Into<String>: Into<String>, impl Into<String>: Into<String>>(self, key: impl Into<String>, value: impl Into<String>) -> Self;
    pub fn unauthorized_object_id<impl Into<String>: Into<String>>(self, value: impl Into<String>) -> Self;
}
#[allow(private_bounds)]
impl<S: ContainerPermsAccess> crate::builder::SasTokenBuilder<'_, S> {
    pub fn add(self) -> Self;
    pub fn create(self) -> Self;
    pub fn delete(self) -> Self;
    pub fn delete_version(self) -> Self;
    pub fn execute(self) -> Self;
    pub fn list(self) -> Self;
    pub fn move_blob(self) -> Self;
    pub fn ownership(self) -> Self;
    pub fn permanent_delete(self) -> Self;
    pub fn permissions(self) -> Self;
    pub fn read(self) -> Self;
    pub fn set_immutability_policy(self) -> Self;
    pub fn tags(self) -> Self;
    pub fn write(self) -> Self;
}
impl<'a> SasTokenBuilder<'a, Untyped> {
    pub fn blob<impl Into<String>: Into<String>, impl Into<String>: Into<String>>(self, container: impl Into<String>, blob: impl Into<String>) -> SasTokenBuilder<'a, BlobState>;
    pub fn container<impl Into<String>: Into<String>>(self, container: impl Into<String>) -> SasTokenBuilder<'a, ContainerState>;
    pub fn directory<impl Into<String>: Into<String>, impl Into<String>: Into<String>>(self, container: impl Into<String>, directory: impl Into<String>) -> SasTokenBuilder<'a, DirectoryState>;
    pub fn file<impl Into<String>: Into<String>, impl Into<String>: Into<String>>(self, share: impl Into<String>, path: impl Into<String>) -> SasTokenBuilder<'a, FileState>;
    pub fn new<impl Into<String>: Into<String>>(account: impl Into<String>, key: &'a UserDelegationKey, expiry: OffsetDateTime) -> azure_core::Result<Self>;
    pub fn queue<impl Into<String>: Into<String>>(self, queue: impl Into<String>) -> SasTokenBuilder<'a, QueueState>;
    pub fn share<impl Into<String>: Into<String>>(self, share: impl Into<String>) -> SasTokenBuilder<'a, ShareState>;
    pub fn table<impl Into<String>: Into<String>>(self, table: impl Into<String>) -> SasTokenBuilder<'a, TableState>;
}
impl crate::builder::SasTokenBuilder<'_, BlobState> {
    pub fn add(self) -> Self;
    pub fn create(self) -> Self;
    pub fn delete(self) -> Self;
    pub fn delete_version(self) -> Self;
    pub fn execute(self) -> Self;
    pub fn move_blob(self) -> Self;
    pub fn ownership(self) -> Self;
    pub fn permanent_delete(self) -> Self;
    pub fn permissions(self) -> Self;
    pub fn read(self) -> Self;
    pub fn set_immutability_policy(self) -> Self;
    pub fn snapshot<impl Into<String>: Into<String>>(self, snapshot: impl Into<String>) -> Self;
    pub fn tags(self) -> Self;
    pub fn version<impl Into<String>: Into<String>>(self, version_id: impl Into<String>) -> Self;
    pub fn write(self) -> Self;
}
impl crate::builder::SasTokenBuilder<'_, FileState> {
    pub fn cache_control<impl Into<String>: Into<String>>(self, value: impl Into<String>) -> Self;
    pub fn content_disposition<impl Into<String>: Into<String>>(self, value: impl Into<String>) -> Self;
    pub fn content_encoding<impl Into<String>: Into<String>>(self, value: impl Into<String>) -> Self;
    pub fn content_language<impl Into<String>: Into<String>>(self, value: impl Into<String>) -> Self;
    pub fn content_type<impl Into<String>: Into<String>>(self, value: impl Into<String>) -> Self;
    pub fn delete(self) -> Self;
    pub fn read(self) -> Self;
    pub fn write(self) -> Self;
}
impl crate::builder::SasTokenBuilder<'_, QueueState> {
    pub fn add(self) -> Self;
    pub fn process(self) -> Self;
    pub fn read(self) -> Self;
    pub fn update(self) -> Self;
}
impl crate::builder::SasTokenBuilder<'_, ShareState> {
    pub fn cache_control<impl Into<String>: Into<String>>(self, value: impl Into<String>) -> Self;
    pub fn content_disposition<impl Into<String>: Into<String>>(self, value: impl Into<String>) -> Self;
    pub fn content_encoding<impl Into<String>: Into<String>>(self, value: impl Into<String>) -> Self;
    pub fn content_language<impl Into<String>: Into<String>>(self, value: impl Into<String>) -> Self;
    pub fn content_type<impl Into<String>: Into<String>>(self, value: impl Into<String>) -> Self;
    pub fn create(self) -> Self;
    pub fn delete(self) -> Self;
    pub fn list(self) -> Self;
    pub fn read(self) -> Self;
    pub fn write(self) -> Self;
}
impl crate::builder::SasTokenBuilder<'_, TableState> {
    pub fn add(self) -> Self;
    pub fn delete(self) -> Self;
    pub fn end_key<impl Into<String>: Into<String>, impl Into<String>: Into<String>>(self, partition_key: impl Into<String>, row_key: impl Into<String>) -> Self;
    pub fn read(self) -> Self;
    pub fn start_key<impl Into<String>: Into<String>, impl Into<String>: Into<String>>(self, partition_key: impl Into<String>, row_key: impl Into<String>) -> Self;
    pub fn update(self) -> Self;
}
#[derive(Clone, Debug)]
pub struct SasUrl(/* private fields */);
impl Deref for SasUrl {
    type Target = Url;
    fn deref(&self) -> &url::Url;
}
impl Display for SasUrl {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result;
}
impl From<SasUrl> for url::Url {
    fn from(u: SasUrl) -> url::Url;
}
pub struct SasUrlBuilder<'a, S = Untyped> {
}
#[allow(private_bounds)]
impl<S: SasResource> SasUrlBuilder<'_, S> {
    pub fn build(&self) -> azure_core::Result<SasUrl>;
}
impl<'a, S> SasUrlBuilder<'a, S> {
    pub fn delegated_user_object_id<impl Into<String>: Into<String>>(self, value: impl Into<String>) -> Self;
    pub fn endpoint(self, endpoint: url::Url) -> Self;
    pub fn ip_range(self, ip: SasIpRange) -> Self;
    pub fn protocol(self, protocol: SasProtocol) -> Self;
    pub fn start(self, start: OffsetDateTime) -> Self;
}
#[allow(private_bounds)]
impl<S: BlobServiceState + BlobOptions> crate::builder::SasUrlBuilder<'_, S> {
    pub fn authorized_object_id<impl Into<String>: Into<String>>(self, value: impl Into<String>) -> Self;
    pub fn cache_control<impl Into<String>: Into<String>>(self, value: impl Into<String>) -> Self;
    pub fn content_disposition<impl Into<String>: Into<String>>(self, value: impl Into<String>) -> Self;
    pub fn content_encoding<impl Into<String>: Into<String>>(self, value: impl Into<String>) -> Self;
    pub fn content_language<impl Into<String>: Into<String>>(self, value: impl Into<String>) -> Self;
    pub fn content_type<impl Into<String>: Into<String>>(self, value: impl Into<String>) -> Self;
    pub fn correlation_id<impl Into<String>: Into<String>>(self, value: impl Into<String>) -> Self;
    pub fn encryption_scope<impl Into<String>: Into<String>>(self, scope: impl Into<String>) -> Self;
    pub fn signed_request_header<impl Into<String>: Into<String>, impl Into<String>: Into<String>>(self, key: impl Into<String>, value: impl Into<String>) -> Self;
    pub fn signed_request_query_parameter<impl Into<String>: Into<String>, impl Into<String>: Into<String>>(self, key: impl Into<String>, value: impl Into<String>) -> Self;
    pub fn unauthorized_object_id<impl Into<String>: Into<String>>(self, value: impl Into<String>) -> Self;
}
#[allow(private_bounds)]
impl<S: ContainerPermsAccess> crate::builder::SasUrlBuilder<'_, S> {
    pub fn add(self) -> Self;
    pub fn create(self) -> Self;
    pub fn delete(self) -> Self;
    pub fn delete_version(self) -> Self;
    pub fn execute(self) -> Self;
    pub fn list(self) -> Self;
    pub fn move_blob(self) -> Self;
    pub fn ownership(self) -> Self;
    pub fn permanent_delete(self) -> Self;
    pub fn permissions(self) -> Self;
    pub fn read(self) -> Self;
    pub fn set_immutability_policy(self) -> Self;
    pub fn tags(self) -> Self;
    pub fn write(self) -> Self;
}
impl<'a> SasUrlBuilder<'a, Untyped> {
    pub fn blob<impl Into<String>: Into<String>, impl Into<String>: Into<String>>(self, container: impl Into<String>, blob: impl Into<String>) -> SasUrlBuilder<'a, BlobState>;
    pub fn container<impl Into<String>: Into<String>>(self, container: impl Into<String>) -> SasUrlBuilder<'a, ContainerState>;
    pub fn directory<impl Into<String>: Into<String>, impl Into<String>: Into<String>>(self, container: impl Into<String>, directory: impl Into<String>) -> SasUrlBuilder<'a, DirectoryState>;
    pub fn file<impl Into<String>: Into<String>, impl Into<String>: Into<String>>(self, share: impl Into<String>, path: impl Into<String>) -> SasUrlBuilder<'a, FileState>;
    pub fn new<impl Into<String>: Into<String>>(account: impl Into<String>, key: &'a UserDelegationKey, expiry: OffsetDateTime) -> azure_core::Result<Self>;
    pub fn queue<impl Into<String>: Into<String>>(self, queue: impl Into<String>) -> SasUrlBuilder<'a, QueueState>;
    pub fn share<impl Into<String>: Into<String>>(self, share: impl Into<String>) -> SasUrlBuilder<'a, ShareState>;
    pub fn table<impl Into<String>: Into<String>>(self, table: impl Into<String>) -> SasUrlBuilder<'a, TableState>;
}
impl crate::builder::SasUrlBuilder<'_, BlobState> {
    pub fn add(self) -> Self;
    pub fn create(self) -> Self;
    pub fn delete(self) -> Self;
    pub fn delete_version(self) -> Self;
    pub fn execute(self) -> Self;
    pub fn move_blob(self) -> Self;
    pub fn ownership(self) -> Self;
    pub fn permanent_delete(self) -> Self;
    pub fn permissions(self) -> Self;
    pub fn read(self) -> Self;
    pub fn set_immutability_policy(self) -> Self;
    pub fn snapshot<impl Into<String>: Into<String>>(self, snapshot: impl Into<String>) -> Self;
    pub fn tags(self) -> Self;
    pub fn version<impl Into<String>: Into<String>>(self, version_id: impl Into<String>) -> Self;
    pub fn write(self) -> Self;
}
impl crate::builder::SasUrlBuilder<'_, FileState> {
    pub fn cache_control<impl Into<String>: Into<String>>(self, value: impl Into<String>) -> Self;
    pub fn content_disposition<impl Into<String>: Into<String>>(self, value: impl Into<String>) -> Self;
    pub fn content_encoding<impl Into<String>: Into<String>>(self, value: impl Into<String>) -> Self;
    pub fn content_language<impl Into<String>: Into<String>>(self, value: impl Into<String>) -> Self;
    pub fn content_type<impl Into<String>: Into<String>>(self, value: impl Into<String>) -> Self;
    pub fn delete(self) -> Self;
    pub fn read(self) -> Self;
    pub fn write(self) -> Self;
}
impl crate::builder::SasUrlBuilder<'_, QueueState> {
    pub fn add(self) -> Self;
    pub fn process(self) -> Self;
    pub fn read(self) -> Self;
    pub fn update(self) -> Self;
}
impl crate::builder::SasUrlBuilder<'_, ShareState> {
    pub fn cache_control<impl Into<String>: Into<String>>(self, value: impl Into<String>) -> Self;
    pub fn content_disposition<impl Into<String>: Into<String>>(self, value: impl Into<String>) -> Self;
    pub fn content_encoding<impl Into<String>: Into<String>>(self, value: impl Into<String>) -> Self;
    pub fn content_language<impl Into<String>: Into<String>>(self, value: impl Into<String>) -> Self;
    pub fn content_type<impl Into<String>: Into<String>>(self, value: impl Into<String>) -> Self;
    pub fn create(self) -> Self;
    pub fn delete(self) -> Self;
    pub fn list(self) -> Self;
    pub fn read(self) -> Self;
    pub fn write(self) -> Self;
}
impl crate::builder::SasUrlBuilder<'_, TableState> {
    pub fn add(self) -> Self;
    pub fn delete(self) -> Self;
    pub fn end_key<impl Into<String>: Into<String>, impl Into<String>: Into<String>>(self, partition_key: impl Into<String>, row_key: impl Into<String>) -> Self;
    pub fn read(self) -> Self;
    pub fn start_key<impl Into<String>: Into<String>, impl Into<String>: Into<String>>(self, partition_key: impl Into<String>, row_key: impl Into<String>) -> Self;
    pub fn update(self) -> Self;
}
#[derive(Clone, Debug, Eq, PartialEq)]
pub enum SasIpRange {
    Address(std::net::Ipv4Addr),
    InclusiveRange { start: std::net::Ipv4Addr, end: std::net::Ipv4Addr },
}
impl Display for SasIpRange {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result;
}
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum SasProtocol {
    Https,
    HttpsAndHttp,
}
impl Display for SasProtocol {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result;
}
pub mod blob {
    pub struct BlobState {
    }
    impl BlobServiceState for BlobState {
    }
    pub struct ContainerState {
    }
    impl BlobServiceState for ContainerState {
    }
    pub struct DirectoryState {
    }
    impl BlobServiceState for DirectoryState {
    }
    pub trait BlobServiceState: Sealed {
    }
}
pub mod file {
    pub struct FileState {
    }
    pub struct ShareState {
    }
    pub const FILE_MIN_VERSION: &str = "2025-07-05";
}
pub mod queue {
    pub struct QueueState {
    }
}
pub mod table {
    pub struct TableState {
    }
    pub const TABLE_MIN_VERSION: &str = "2025-07-05";
}
```

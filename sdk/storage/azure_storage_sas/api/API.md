# azure_storage_sas

- **Description**: User delegation Shared Access Signature (SAS) builder for Microsoft Azure Storage services
- **Edition**: 2021
- **Rust version**: 1.88

## Features

- `default`

```rust
pub struct SasBuilder<'a, S = Untyped> {
}
#[allow(private_bounds)]
impl<S: SasResource> SasBuilder<'_, S> {
    pub fn build(&self) -> String;
}
impl<S> SasBuilder<'_, S> {
    pub fn delegated_user_object_id<impl Into<String>: Into<String>>(self, value: impl Into<String>) -> Self;
    pub fn ip_range(self, ip: SasIpRange) -> Self;
    pub fn protocol(self, protocol: SasProtocol) -> Self;
    pub fn start(self, start: OffsetDateTime) -> Self;
}
#[allow(private_bounds)]
impl<S: BlobServiceState + BlobOptions> crate::builder::SasBuilder<'_, S> {
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
impl<S: ContainerPermsAccess> crate::builder::SasBuilder<'_, S> {
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
impl<'a> SasBuilder<'a, Untyped> {
    pub fn blob<impl Into<String>: Into<String>, impl Into<String>: Into<String>>(self, container: impl Into<String>, blob: impl Into<String>) -> SasBuilder<'a, BlobState>;
    pub fn container<impl Into<String>: Into<String>>(self, container: impl Into<String>) -> SasBuilder<'a, ContainerState>;
    pub fn directory<impl Into<String>: Into<String>, impl Into<String>: Into<String>>(self, container: impl Into<String>, directory: impl Into<String>) -> SasBuilder<'a, DirectoryState>;
    pub fn new<impl Into<String>: Into<String>>(account: impl Into<String>, key: &'a UserDelegationKey, expiry: OffsetDateTime) -> azure_core::Result<Self>;
    pub fn queue<impl Into<String>: Into<String>>(self, queue: impl Into<String>) -> SasBuilder<'a, QueueState>;
}
impl crate::builder::SasBuilder<'_, BlobState> {
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
impl crate::builder::SasBuilder<'_, QueueState> {
    pub fn add(self) -> Self;
    pub fn process(self) -> Self;
    pub fn read(self) -> Self;
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
pub mod queue {
    pub struct QueueState {
    }
}
```

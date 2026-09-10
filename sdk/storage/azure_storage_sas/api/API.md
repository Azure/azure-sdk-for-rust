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
    fn build(&self) -> String;
}
impl<S> SasBuilder<'_, S> {
    fn delegated_user_object_id<impl Into<String>: Into<String>>(self, value: impl Into<String>) -> Self;
    fn ip_range(self, ip: SasIpRange) -> Self;
    fn protocol(self, protocol: SasProtocol) -> Self;
    fn start(self, start: OffsetDateTime) -> Self;
}
#[allow(private_bounds)]
impl<S: BlobServiceState + BlobOptions> crate::builder::SasBuilder<'_, S> {
    fn authorized_object_id<impl Into<String>: Into<String>>(self, value: impl Into<String>) -> Self;
    fn cache_control<impl Into<String>: Into<String>>(self, value: impl Into<String>) -> Self;
    fn content_disposition<impl Into<String>: Into<String>>(self, value: impl Into<String>) -> Self;
    fn content_encoding<impl Into<String>: Into<String>>(self, value: impl Into<String>) -> Self;
    fn content_language<impl Into<String>: Into<String>>(self, value: impl Into<String>) -> Self;
    fn content_type<impl Into<String>: Into<String>>(self, value: impl Into<String>) -> Self;
    fn correlation_id<impl Into<String>: Into<String>>(self, value: impl Into<String>) -> Self;
    fn encryption_scope<impl Into<String>: Into<String>>(self, scope: impl Into<String>) -> Self;
    fn signed_request_header<impl Into<String>: Into<String>, impl Into<String>: Into<String>>(self, key: impl Into<String>, value: impl Into<String>) -> Self;
    fn signed_request_query_parameter<impl Into<String>: Into<String>, impl Into<String>: Into<String>>(self, key: impl Into<String>, value: impl Into<String>) -> Self;
    fn unauthorized_object_id<impl Into<String>: Into<String>>(self, value: impl Into<String>) -> Self;
}
#[allow(private_bounds)]
impl<S: ContainerPermsAccess> crate::builder::SasBuilder<'_, S> {
    fn add(self) -> Self;
    fn create(self) -> Self;
    fn delete(self) -> Self;
    fn delete_version(self) -> Self;
    fn execute(self) -> Self;
    fn list(self) -> Self;
    fn move_blob(self) -> Self;
    fn ownership(self) -> Self;
    fn permanent_delete(self) -> Self;
    fn permissions(self) -> Self;
    fn read(self) -> Self;
    fn set_immutability_policy(self) -> Self;
    fn tags(self) -> Self;
    fn write(self) -> Self;
}
impl<'a> SasBuilder<'a, Untyped> {
    fn blob<impl Into<String>: Into<String>, impl Into<String>: Into<String>>(self, container: impl Into<String>, blob: impl Into<String>) -> SasBuilder<'a, BlobState>;
    fn container<impl Into<String>: Into<String>>(self, container: impl Into<String>) -> SasBuilder<'a, ContainerState>;
    fn directory<impl Into<String>: Into<String>, impl Into<String>: Into<String>>(self, container: impl Into<String>, directory: impl Into<String>) -> SasBuilder<'a, DirectoryState>;
    fn new<impl Into<String>: Into<String>>(account: impl Into<String>, key: &'a UserDelegationKey, expiry: OffsetDateTime) -> azure_core::Result<Self>;
    fn queue<impl Into<String>: Into<String>>(self, queue: impl Into<String>) -> SasBuilder<'a, QueueState>;
}
impl crate::builder::SasBuilder<'_, BlobState> {
    fn add(self) -> Self;
    fn create(self) -> Self;
    fn delete(self) -> Self;
    fn delete_version(self) -> Self;
    fn execute(self) -> Self;
    fn move_blob(self) -> Self;
    fn ownership(self) -> Self;
    fn permanent_delete(self) -> Self;
    fn permissions(self) -> Self;
    fn read(self) -> Self;
    fn set_immutability_policy(self) -> Self;
    fn snapshot<impl Into<String>: Into<String>>(self, snapshot: impl Into<String>) -> Self;
    fn tags(self) -> Self;
    fn version<impl Into<String>: Into<String>>(self, version_id: impl Into<String>) -> Self;
    fn write(self) -> Self;
}
impl crate::builder::SasBuilder<'_, QueueState> {
    fn add(self) -> Self;
    fn process(self) -> Self;
    fn read(self) -> Self;
    fn update(self) -> Self;
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

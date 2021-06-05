pub mod clients;
mod connection_string;
mod connection_string_builder;
mod copy_id;
mod copy_progress;
mod errors;
mod into_azure_path;
pub mod prelude;
pub mod shared_access_signature;
pub use self::connection_string::{ConnectionString, EndpointProtocol};
pub use self::connection_string_builder::ConnectionStringBuilder;
pub use self::into_azure_path::IntoAzurePath;
pub(crate) mod headers;
pub use copy_id::{copy_id_from_headers, CopyId};
pub use copy_progress::CopyProgress;
pub(crate) mod parsing_xml;
mod stored_access_policy;
pub use errors::AzureStorageError;

#[derive(Debug, Clone, Eq, PartialEq, Copy, Serialize, Deserialize)]
pub struct Yes;
#[derive(Debug, Clone, Eq, PartialEq, Copy, Serialize, Deserialize)]
pub struct No;

pub trait ToAssign: std::fmt::Debug {}
pub trait Assigned: ToAssign {}
pub trait NotAssigned: ToAssign {}

impl ToAssign for Yes {}
impl ToAssign for No {}

impl Assigned for Yes {}
impl NotAssigned for No {}

#[derive(Debug, Clone, PartialEq)]
pub struct IPRange {
    pub start: std::net::IpAddr,
    pub end: std::net::IpAddr,
}

pub use stored_access_policy::{StoredAccessPolicy, StoredAccessPolicyList};

#[derive(Debug, Clone, PartialEq)]
pub struct ConsistencyCRC64(pub [u8; 8]);

#[derive(Debug, Clone, PartialEq)]
pub struct ConsistencyMD5(pub [u8; 16]);

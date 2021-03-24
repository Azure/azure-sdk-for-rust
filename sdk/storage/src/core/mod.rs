pub mod clients;
mod connection_string;
mod connection_string_builder;
mod copy_id;
mod copy_progress;
mod into_azure_path;
pub mod prelude;
pub mod shared_access_signature;
pub use self::connection_string::{ConnectionString, EndpointProtocol};
pub use self::connection_string_builder::ConnectionStringBuilder;
pub use self::into_azure_path::IntoAzurePath;
use azure_core::errors::AzureError;
use azure_core::headers::*;
pub use copy_id::{copy_id_from_headers, CopyId};
pub use copy_progress::CopyProgress;

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

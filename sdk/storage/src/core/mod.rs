mod bearer_token_client;
mod blob_sas_builder;
pub mod client;
mod client_endpoint;
mod connection_string;
mod connection_string_builder;
mod container_sas_builder;
mod copy_id;
mod copy_progress;
mod hyper_client_endpoint;
mod into_azure_path;
pub mod key_client;
mod perform_request_response;
pub mod prelude;
mod rest_client;
pub mod shared_access_signature;
pub use self::connection_string::{ConnectionString, EndpointProtocol};
pub use self::connection_string_builder::ConnectionStringBuilder;
pub use self::into_azure_path::IntoAzurePath;
pub use self::rest_client::{
    get_default_json_mime, get_json_mime_fullmetadata, get_json_mime_nometadata, perform_request,
    ServiceType,
};
use crate::key_client::KeyClient;
use azure_core::errors::AzureError;
use azure_core::headers::*;
pub use client::Client;
pub use client_endpoint::ClientEndpoint;
pub use copy_id::{copy_id_from_headers, CopyId};
pub use copy_progress::CopyProgress;
pub use hyper_client_endpoint::HyperClientEndpoint;
pub use perform_request_response::PerformRequestResponse;

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

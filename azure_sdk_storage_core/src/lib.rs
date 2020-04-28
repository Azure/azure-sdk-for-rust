#![recursion_limit = "128"]

#[macro_use]
extern crate log;
pub mod client;
mod rest_client;
pub use self::rest_client::{
    get_default_json_mime, get_json_mime_fullmetadata, get_json_mime_nometadata, perform_request,
    ServiceType,
};
use crate::client::Client;
use azure_sdk_core::errors::AzureError;
use azure_sdk_core::headers::COPY_ID;
use azure_sdk_core::util::HeaderMapExt;
mod into_azure_path;
pub mod prelude;
pub use self::into_azure_path::IntoAzurePath;
mod blob_sas_builder;
use http::HeaderMap;
mod client_endpoint;
mod container_sas_builder;
mod hyper_client_endpoint;
pub mod shared_access_signature;
pub use client_endpoint::ClientEndpoint;
pub use hyper_client_endpoint::HyperClientEndpoint;

pub trait ClientRequired<'a> {
    fn client(&self) -> &'a Client;
}

pub trait SharedAccessSignatureSupport<'a> {
    type O;
    fn with_shared_access_signature(
        self,
        signature: &'a shared_access_signature::SharedAccessSignature,
    ) -> Self::O;
}

pub trait SharedAccessSignatureRequired<'a> {
    fn shared_access_signature(&self) -> &'a shared_access_signature::SharedAccessSignature;
}

#[derive(Debug, Clone, PartialEq)]
pub struct IPRange {
    pub start: std::net::IpAddr,
    pub end: std::net::IpAddr,
}

pub type CopyId = uuid::Uuid;

pub fn copy_id_from_headers(headers: &HeaderMap) -> Result<CopyId, AzureError> {
    let copy_id = headers
        .get_as_str(COPY_ID)
        .ok_or_else(|| AzureError::HeaderNotFound(COPY_ID.to_owned()))?;
    Ok(uuid::Uuid::parse_str(copy_id)?)
}

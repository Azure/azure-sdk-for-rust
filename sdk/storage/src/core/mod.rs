mod bearer_token_client;
mod blob_sas_builder;
pub mod client;
mod client_endpoint;
mod connection_string;
mod connection_string_builder;
mod container_sas_builder;
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
use azure_core::headers::COPY_ID;
use azure_core::util::HeaderMapExt;
pub use client::Client;
pub use client_endpoint::ClientEndpoint;
use http::HeaderMap;
pub use hyper_client_endpoint::HyperClientEndpoint;
pub use perform_request_response::PerformRequestResponse;

pub trait ClientRequired<'a, C>
where
    C: Client,
{
    fn client(&self) -> &'a C;
}

pub trait KeyClientRequired<'a> {
    fn key_client(&self) -> &'a KeyClient;
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

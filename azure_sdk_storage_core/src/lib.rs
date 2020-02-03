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
mod into_azure_path;
pub mod prelude;
pub use self::into_azure_path::IntoAzurePath;
mod blob_sas_builder;
mod container_sas_builder;
pub mod shared_access_signature;

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

#![recursion_limit = "128"]

extern crate base64;
extern crate chrono;
extern crate futures;
extern crate http;
extern crate hyper;
extern crate hyper_rustls;
extern crate md5;
extern crate ring;
extern crate time;
extern crate url;
extern crate uuid;
extern crate xml;
#[macro_use]
extern crate log;
extern crate azure_sdk_core;
extern crate bytes;
extern crate quick_error;
extern crate serde;
extern crate serde_derive;
extern crate serde_json;
extern crate serde_xml_rs;
extern crate smallvec;
pub mod client;
mod rest_client;
pub use self::rest_client::{get_default_json_mime, get_json_mime_nometadata, perform_request, ServiceType};
use crate::client::Client;

pub trait ClientRequired<'a> {
    fn client(&self) -> &'a Client;
}
mod into_azure_path;
pub mod prelude;
pub use self::into_azure_path::IntoAzurePath;
mod blob_sas_builder;
mod container_sas_builder;
pub mod shared_access_signature;

pub trait SharedAccessSignatureSupport<'a> {
    type O;
    fn with_shared_access_signature(self, signature: &'a shared_access_signature::SharedAccessSignature) -> Self::O;
}

pub trait SharedAccessSignatureRequired<'a> {
    fn shared_access_signature(&self) -> &'a shared_access_signature::SharedAccessSignature;
}

#[derive(Debug, Clone, PartialEq)]
pub struct IPRange {
    pub start: std::net::IpAddr,
    pub end: std::net::IpAddr,
}

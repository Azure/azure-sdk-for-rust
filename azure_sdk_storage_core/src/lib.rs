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

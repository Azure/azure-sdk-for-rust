extern crate hyper;
extern crate ring;
extern crate url;

#[macro_use]
pub mod errors;
pub mod parsing;
#[macro_use]
pub mod enumerations;
pub mod incompletevector;
pub mod lease;
use azure::storage::client::Client;
use std::fmt::Debug;
pub mod ba512_range;
pub mod range;
use url::percent_encoding;
define_encode_set! {
    pub COMPLETE_ENCODE_SET = [percent_encoding::USERINFO_ENCODE_SET] | {
        '+', '-', '&'
    }
}
pub mod headers;
use self::headers::{CLIENT_REQUEST_ID, LEASE_ID};
use uuid::Uuid;
pub type RequestId = Uuid;
use azure::core::lease::LeaseId;
use http::request::Builder;
mod stored_access_policy;
pub(crate) mod util;
pub use self::stored_access_policy::{StoredAccessPolicy, StoredAccessPolicyList};

#[derive(Debug)]
pub struct Yes;
#[derive(Debug)]
pub struct No;

pub trait ToAssign: Debug {}
pub trait Assigned: ToAssign {}
pub trait NotAssigned: ToAssign {}

impl ToAssign for Yes {}
impl ToAssign for No {}

impl Assigned for Yes {}
impl NotAssigned for No {}

pub trait ClientRequired<'a> {
    fn client(&self) -> &'a Client;
}

pub trait TimeoutSupport {
    type O;
    fn with_timeout(self, timeout: u64) -> Self::O;
}

pub trait TimeoutOption {
    fn timeout(&self) -> Option<u64>;

    fn to_uri_parameter(&self) -> Option<String> {
        if let Some(nm) = self.timeout() {
            Some(format!("timeout={}", nm))
        } else {
            None
        }
    }
}

pub trait ClientRequestIdSupport<'a> {
    type O;
    fn with_client_request_id(self, client_request_id: &'a str) -> Self::O;
}

pub trait ClientRequestIdOption<'a> {
    fn client_request_id(&self) -> Option<&'a str>;

    fn add_header(&self, builder: &mut Builder) {
        if let Some(client_request_id) = self.client_request_id() {
            builder.header(CLIENT_REQUEST_ID, client_request_id);
        }
    }
}

pub trait NextMarkerSupport<'a> {
    type O;
    fn with_next_marker(self, next_marker: &'a str) -> Self;
}

pub trait NextMarkerOption<'a> {
    fn next_marker(&self) -> Option<&'a str>;

    fn to_uri_parameter(&self) -> Option<String> {
        if let Some(ref nm) = self.next_marker() {
            Some(format!("marker={}", nm))
        } else {
            None
        }
    }
}

pub trait PrefixSupport<'a> {
    type O;
    fn with_prefix(self, prefix: &'a str) -> Self::O;
}

pub trait PrefixOption<'a> {
    fn prefix(&self) -> Option<&'a str>;

    fn to_uri_parameter(&self) -> Option<String> {
        if let Some(ref nm) = self.prefix() {
            Some(format!("prefix={}", nm))
        } else {
            None
        }
    }
}

pub trait LeaseIdSupport<'a> {
    type O;
    fn with_lease_id(self, &'a LeaseId) -> Self::O;
}

pub trait LeaseIdOption<'a> {
    fn lease_id(&self) -> Option<&'a LeaseId>;

    fn add_header(&self, builder: &mut Builder) {
        if let Some(lease_id) = self.lease_id() {
            builder.header(LEASE_ID, &lease_id.to_string() as &str);
        }
    }
}

pub trait ContainerNameSupport<'a> {
    type O;
    fn with_container_name(self, container_name: &'a str) -> Self::O;
}

pub trait ContainerNameRequired<'a> {
    fn container_name(&self) -> &'a str;
}

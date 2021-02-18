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
use http::request::Builder;
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

pub trait TimeoutSupport {
    type O;
    fn with_timeout(self, timeout: u64) -> Self::O;
}

pub trait TimeoutOption {
    fn timeout(&self) -> Option<u64>;

    fn to_uri_parameter(&self) -> Option<String> {
        self.timeout().map(|nm| format!("timeout={}", nm))
    }

    fn append_to_url(&self, url: &mut url::Url) {
        if let Some(timeout) = self.timeout() {
            url.query_pairs_mut()
                .append_pair("timeout", &format!("{}", timeout));
        }
    }
}
pub trait ContinuationOption<'a> {
    fn continuation(&self) -> Option<&'a str>;

    #[must_use]
    fn add_optional_header(&self, builder: Builder) -> Builder {
        if let Some(continuation) = self.continuation() {
            builder.header(CONTINUATION, continuation)
        } else {
            builder
        }
    }
}

pub trait ContinuationSupport<'a> {
    type O;
    fn with_continuation(self, continuation: &'a str) -> Self::O;
}

pub trait MaxResultsSupport {
    type O;
    fn with_max_results(self, max_results: u32) -> Self::O;
}

pub trait MaxResultsOption {
    fn max_results(&self) -> Option<u32>;

    fn to_uri_parameter(&self) -> Option<String> {
        self.max_results()
            .map(|ref nm| format!("maxresults={}", nm))
    }

    fn append_to_url(&self, url: &mut url::Url) {
        if let Some(max_results) = self.max_results() {
            url.query_pairs_mut()
                .append_pair("maxresults", &format!("{}", max_results));
        }
    }
}

pub trait ClientRequestIdSupport<'a> {
    type O;
    fn with_client_request_id(self, client_request_id: &'a str) -> Self::O;
}

pub trait ClientRequestIdOption<'a> {
    fn client_request_id(&self) -> Option<&'a str>;

    #[must_use]
    fn add_optional_header(&self, mut builder: Builder) -> Builder {
        if let Some(client_request_id) = self.client_request_id() {
            builder = builder.header(CLIENT_REQUEST_ID, client_request_id);
        }
        builder
    }
}

pub trait PrefixSupport<'a> {
    type O;
    fn with_prefix(self, prefix: &'a str) -> Self::O;
}

pub trait IncludeMetadataSupport {
    type O;
    fn with_include_metadata(self) -> Self::O;
}

pub trait NextMarkerSupport<'a> {
    type O;
    fn with_next_marker(self, next_marker: &'a str) -> Self::O;
}

pub trait IncludeMetadataOption {
    fn include_metadata(&self) -> bool;

    fn to_uri_parameter(&self) -> Option<&'static str> {
        if self.include_metadata() {
            Some("include=metadata")
        } else {
            None
        }
    }

    fn append_to_url(&self, url: &mut url::Url) {
        if self.include_metadata() {
            url.query_pairs_mut().append_pair("include", "metadata");
        }
    }
}

pub trait NextMarkerOption<'a> {
    fn next_marker(&self) -> Option<&'a str>;

    fn to_uri_parameter(&self) -> Option<String> {
        self.next_marker().map(|ref nm| format!("marker={}", nm))
    }

    fn append_to_url(&self, url: &mut url::Url) {
        if let Some(next_marker) = self.next_marker() {
            url.query_pairs_mut().append_pair("marker", next_marker);
        }
    }
}
pub trait PrefixOption<'a> {
    fn prefix(&self) -> Option<&'a str>;

    fn to_uri_parameter(&self) -> Option<String> {
        self.prefix().map(|ref nm| format!("prefix={}", nm))
    }

    fn append_to_url(&self, url: &mut url::Url) {
        if let Some(prefix) = self.prefix() {
            url.query_pairs_mut().append_pair("prefix", prefix);
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

pub trait BlobNameSupport<'a> {
    type O;
    fn with_blob_name(self, blob_name: &'a str) -> Self::O;
}

pub trait BlobNameRequired<'a> {
    fn blob_name(&self) -> &'a str;
}
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

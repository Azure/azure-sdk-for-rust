use crate::{
    client::headers::*,
    document::{DocumentAttributes, IndexingDirective},
    partition_key::PartitionKey,
    request_response::*,
    ConsistencyLevel,
};
#[allow(unused_imports)]
use azure_sdk_core::{
    errors::{
        check_status_extract_body, check_status_extract_headers_and_body,
        extract_status_headers_and_body, AzureError, UnexpectedHTTPResult,
    },
    incompletevector::ContinuationToken,
    util::RequestBuilderExt,
};
use http::request::Builder as RequestBuilder;
use hyper::{
    self,
    header::{self, HeaderMap, HeaderValue},
    StatusCode,
};
use hyper_rustls::HttpsConnector;
use serde::de::DeserializeOwned;
use serde_json;
use std::sync::Arc;
use std::{marker::PhantomData, str};

type HyperClient = Arc<hyper::Client<HttpsConnector<hyper::client::HttpConnector>>>;

macro_rules! request_bytes_ref {
    ($name:ident, $h:path) => {
        pub fn $name<V:Into<bytes::Bytes>>(mut self, value: V) -> Self {
            self.request.header_bytes($h, value);
            self
        }
    };
}

macro_rules! request_option {
    ($name:ident, bool, $h:path) => {
        pub fn $name<V: Into<bool>>(mut self, value: V) -> Self {
            self.request.header($h, ::http::header::HeaderValue::from_static(
                if value.into() { "true" } else { "false" }));
            self
        }
    };
    ($name:ident, $ty:ty, $h:path) => {
        pub fn $name<V: Into<$ty>>(mut self, value: V) -> Self {
            self.request.header_formatted($h, value.into());
            self
        }
    };
}

mod document_requests;
mod sproc_requests;

pub use self::document_requests::*;
pub use self::sproc_requests::*;

// Copyright (c) Microsoft Corporation. All rights reserved.
// Licensed under the MIT License.

//! Types and functions for building HTTP clients.

#[cfg(all(feature = "test", feature = "unstable_fake"))]
#[cfg_attr(docsrs, doc(cfg(all(feature = "test", feature = "unstable_fake"))))]
pub mod fake;
pub mod headers;
mod models;
mod options;
pub mod pager;
mod pipeline;
pub mod policies;
pub mod poller;
pub mod request;

pub use models::*;
pub use options::*;
pub use pager::{ItemIterator, PageIterator, Pager};
pub use pipeline::*;
pub use poller::Poller;
pub use request::{Body, Request, RequestContent};
pub use response::{AsyncRawResponse, AsyncResponse, AsyncResponseBody, RawResponse, Response};

pub use typespec_client_core::http::response;
pub use typespec_client_core::http::{
    new_http_client, AppendToUrlQuery, Context, DeserializeWith, Format, HttpClient,
    HttpClientOptions, JsonFormat, Method, NoFormat, QueryBuilder, Sanitizer, StatusCode, Url,
    UrlExt, DEFAULT_ALLOWED_HEADER_NAMES, DEFAULT_ALLOWED_QUERY_PARAMETERS, REDACTED_PATTERN,
};

pub use crate::error::check_success;
#[cfg(feature = "xml")]
pub use typespec_client_core::http::XmlFormat;

/// Adds a static operation name to method options for fake server routing.
///
/// Pass the operation name and a mutable reference to method options with a
/// `context` field, such as [`ClientMethodOptions`] or [`pager::PagerOptions`].
/// Does nothing, without evaluating either argument, unless both the `test` and
/// `unstable_fake` features are enabled in `azure_core`.
#[cfg(all(feature = "test", feature = "unstable_fake"))]
#[macro_export]
macro_rules! operation_name {
    ($name:expr, $options:expr $(,)?) => {
        $crate::http::fake::operation_name($name, &mut ($options).context)
    };
}

/// Adds a static operation name to method options for fake server routing.
///
/// Does nothing, without evaluating either argument, unless both the `test` and
/// `unstable_fake` features are enabled in `azure_core`.
#[cfg(not(all(feature = "test", feature = "unstable_fake")))]
#[macro_export]
macro_rules! operation_name {
    ($name:expr, $options:expr $(,)?) => {};
}

pub use crate::operation_name;

#[cfg(test)]
mod tests {
    #[cfg(not(all(feature = "test", feature = "unstable_fake")))]
    #[test]
    fn operation_name_is_a_noop() {
        super::operation_name!(undefined_name, undefined_options);
        super::operation_name!(panic!("must not evaluate"), panic!("must not evaluate"));
    }
}

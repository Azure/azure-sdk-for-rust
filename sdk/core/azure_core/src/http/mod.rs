// Copyright (c) Microsoft Corporation. All rights reserved.
// Licensed under the MIT License.

//! Types and functions for building HTTP clients.

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
pub use response::{AsyncResponse, BufResponse, RawResponse, Response};

pub use typespec_client_core::http::response;
pub use typespec_client_core::http::{
    new_http_client, AppendToUrlQuery, Context, DeserializeWith, Format, HttpClient, JsonFormat,
    Method, NoFormat, StatusCode, Url, UrlExt,
};

pub use crate::error::check_success;
#[cfg(feature = "xml")]
pub use typespec_client_core::http::XmlFormat;

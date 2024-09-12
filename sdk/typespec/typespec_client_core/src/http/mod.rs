// Copyright (c) Microsoft Corporation. All rights reserved.
// Licensed under the MIT License.

//! Types and functions for building HTTP clients.

mod clients;
mod context;
pub mod headers;
mod models;
mod options;
mod pageable;
mod pipeline;
pub mod policies;
pub mod request;
pub mod response;

pub use clients::*;
pub use context::*;
pub use headers::Header;
pub use models::*;
pub use options::*;
pub use pageable::*;
pub use pipeline::*;
pub use request::{Body, Request, RequestContent};
pub use response::{Model, Response};

// Re-export important types.
pub use http_types::{Method, StatusCode};
pub use url::Url;

/// Add a new query pair into the target [`Url`]'s query string.
pub trait AppendToUrlQuery {
    fn append_to_url_query(&self, url: &mut Url);
}

impl<T> AppendToUrlQuery for &T
where
    T: AppendToUrlQuery,
{
    fn append_to_url_query(&self, url: &mut Url) {
        (*self).append_to_url_query(url);
    }
}

impl<T> AppendToUrlQuery for Option<T>
where
    T: AppendToUrlQuery,
{
    fn append_to_url_query(&self, url: &mut Url) {
        if let Some(i) = self {
            i.append_to_url_query(url);
        }
    }
}

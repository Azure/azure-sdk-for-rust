// Copyright (c) Microsoft Corporation. All rights reserved.
// Licensed under the MIT License.

//! Types and functions for building HTTP clients.

mod clients;
mod context;
pub mod headers;
mod models;
mod options;
mod pager;
mod pipeline;
pub mod policies;
pub mod request;
pub mod response;

pub use clients::*;
pub use context::*;
pub use headers::Header;
pub use models::*;
pub use options::*;
pub use pager::*;
pub use pipeline::*;
pub use request::{Body, Request, RequestContent};
pub use response::{Model, Response};

// Re-export important types.
pub use http_types::{Method, StatusCode};
pub use url::Url;
use url::{form_urlencoded::Serializer, UrlQuery};

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

/// Extension methods for [`Url`].
pub trait UrlExt: private::Sealed {
    /// Removes a named query parameter(s).
    ///
    /// # Examples
    ///
    /// ```
    /// # use typespec_client_core::http::{Url, UrlExt as _};
    /// let mut url: Url = "https://azure.net/api?api-version=2024-06-01".parse().unwrap();
    /// url.remove_query_param("api-version");
    /// assert_eq!(url.as_str(), "https://azure.net/api?");
    /// ```
    fn remove_query_param(&mut self, name: &str) -> Serializer<'_, UrlQuery<'_>>;

    /// Sets or replaces a named query parameter(s).
    ///
    /// # Examples
    ///
    /// ```
    /// # use typespec_client_core::http::{Url, UrlExt as _};
    /// let mut url: Url = "https://azure.net/api?api-version=2024-06-01".parse().unwrap();
    /// url.replace_query_pair("api-version", "2025-01-01");
    /// assert_eq!(url.as_str(), "https://azure.net/api?api-version=2025-01-01");
    /// ```
    fn replace_query_pair(&mut self, name: &str, value: &str) -> Serializer<'_, UrlQuery<'_>>;
}

mod private {
    pub trait Sealed {}

    impl Sealed for url::Url {}
}

impl UrlExt for Url {
    fn remove_query_param(&mut self, name: &str) -> Serializer<'_, UrlQuery<'_>> {
        let url = self.to_owned();
        let filtered = url.query_pairs().filter(|(n, _)| n.ne(name));

        let mut qp = self.query_pairs_mut();
        qp.clear().extend_pairs(filtered);

        qp
    }

    fn replace_query_pair(&mut self, name: &str, value: &str) -> Serializer<'_, UrlQuery<'_>> {
        let mut qp = self.remove_query_param(name);
        qp.append_pair(name, value);

        qp
    }
}

// Copyright (c) Microsoft Corporation. All rights reserved.
// Licensed under the MIT License.

//! Types and functions for building HTTP clients.

mod clients;
mod context;
mod format;
pub mod headers;
mod method;
mod models;
mod options;
mod pipeline;
pub mod policies;
pub mod request;
pub mod response;
mod sanitizer;

pub use clients::*;
pub use context::*;
pub use format::*;
pub use method::Method;
pub use models::*;
pub use options::*;
pub use pipeline::*;
pub use request::{Body, Request, RequestContent};
pub use response::{BufResponse, RawResponse, Response};
pub use sanitizer::*;

// Re-export important types.
pub use typespec::http::StatusCode;
pub use url::Url;

/// Add a new query pair into the target [`Url`]'s query string.
pub trait AppendToUrlQuery {
    /// Append the query pair represented by `self` to the given `url`.
    ///
    /// # Arguments
    /// * `url` - The mutable reference to the `Url` to which the query pair will be appended.
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

/// Extension trait for [`Url`] to provide additional URL manipulation methods.
pub trait UrlExt: crate::private::Sealed {
    /// Appends a path segment to the URL's path, handling slashes appropriately and preserving query parameters.
    ///
    /// This always assumes the existing URL terminates with a directory, and the `path` you pass in is a separate directory or file segment.
    ///
    /// # Examples
    ///
    /// ```
    /// use typespec_client_core::http::{Url, UrlExt as _};
    ///
    /// let mut url: Url = "https://contoso.com/foo?a=1".parse().unwrap();
    /// url.append_path("bar");
    /// assert_eq!(url.as_str(), "https://contoso.com/foo/bar?a=1");
    /// ```
    fn append_path(&mut self, path: impl AsRef<str>);
}

impl UrlExt for Url {
    fn append_path(&mut self, p: impl AsRef<str>) {
        let path = p.as_ref().trim_start_matches('/');
        if self.path() == "/" {
            self.set_path(path);
            return;
        }
        if path.is_empty() {
            return;
        }
        let needs_separator = !self.path().ends_with('/');
        let mut new_len = self.path().len() + path.len();
        if needs_separator {
            new_len += 1;
        }
        let mut new_path = String::with_capacity(new_len);
        debug_assert_eq!(new_path.capacity(), new_len);
        new_path.push_str(self.path());
        if needs_separator {
            new_path.push('/');
        }
        new_path.push_str(path);
        debug_assert_eq!(new_path.capacity(), new_len);

        self.set_path(&new_path);
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn url_append_path() {
        let mut url = Url::parse("https://www.microsoft.com?q=q").unwrap();
        url.append_path("foo");
        assert_eq!(url.as_str(), "https://www.microsoft.com/foo?q=q");

        url = Url::parse("https://www.microsoft.com/?q=q").unwrap();
        url.append_path("foo");
        assert_eq!(url.as_str(), "https://www.microsoft.com/foo?q=q");

        url = Url::parse("https://www.microsoft.com?q=q").unwrap();
        url.append_path("/foo");
        assert_eq!(url.as_str(), "https://www.microsoft.com/foo?q=q");

        url = Url::parse("https://www.microsoft.com/?q=q").unwrap();
        url.append_path("/foo");
        assert_eq!(url.as_str(), "https://www.microsoft.com/foo?q=q");

        url = Url::parse("https://www.microsoft.com?q=q").unwrap();
        url.append_path("foo/");
        assert_eq!(url.as_str(), "https://www.microsoft.com/foo/?q=q");

        url = Url::parse("https://www.microsoft.com/?q=q").unwrap();
        url.append_path("foo/");
        assert_eq!(url.as_str(), "https://www.microsoft.com/foo/?q=q");

        url = Url::parse("https://www.microsoft.com?q=q").unwrap();
        url.append_path("/foo/");
        assert_eq!(url.as_str(), "https://www.microsoft.com/foo/?q=q");

        url = Url::parse("https://www.microsoft.com/?q=q").unwrap();
        url.append_path("/foo/");
        assert_eq!(url.as_str(), "https://www.microsoft.com/foo/?q=q");

        url = Url::parse("https://www.microsoft.com/foo?q=q").unwrap();
        url.append_path("bar");
        assert_eq!(url.as_str(), "https://www.microsoft.com/foo/bar?q=q");

        url = Url::parse("https://www.microsoft.com/foo/?q=q").unwrap();
        url.append_path("bar");
        assert_eq!(url.as_str(), "https://www.microsoft.com/foo/bar?q=q");

        url = Url::parse("https://www.microsoft.com/foo?q=q").unwrap();
        url.append_path("/bar");
        assert_eq!(url.as_str(), "https://www.microsoft.com/foo/bar?q=q");

        url = Url::parse("https://www.microsoft.com/foo/?q=q").unwrap();
        url.append_path("/bar");
        assert_eq!(url.as_str(), "https://www.microsoft.com/foo/bar?q=q");

        url = Url::parse("https://www.microsoft.com/foo?q=q").unwrap();
        url.append_path("bar/");
        assert_eq!(url.as_str(), "https://www.microsoft.com/foo/bar/?q=q");

        url = Url::parse("https://www.microsoft.com/foo/?q=q").unwrap();
        url.append_path("bar/");
        assert_eq!(url.as_str(), "https://www.microsoft.com/foo/bar/?q=q");

        url = Url::parse("https://www.microsoft.com/foo?q=q").unwrap();
        url.append_path("/bar/");
        assert_eq!(url.as_str(), "https://www.microsoft.com/foo/bar/?q=q");

        url = Url::parse("https://www.microsoft.com/foo/?q=q").unwrap();
        url.append_path("/bar/");
        assert_eq!(url.as_str(), "https://www.microsoft.com/foo/bar/?q=q");

        url = Url::parse("https://www.microsoft.com?q=q").unwrap();
        url.append_path("/");
        assert_eq!(url.as_str(), "https://www.microsoft.com/?q=q");

        url = Url::parse("https://www.microsoft.com/?q=q").unwrap();
        url.append_path("/");
        assert_eq!(url.as_str(), "https://www.microsoft.com/?q=q");

        url = Url::parse("https://www.microsoft.com?q=q").unwrap();
        url.append_path("");
        assert_eq!(url.as_str(), "https://www.microsoft.com/?q=q");

        url = Url::parse("https://www.microsoft.com?q=q").unwrap();
        url.append_path("");
        assert_eq!(url.as_str(), "https://www.microsoft.com/?q=q");

        url = Url::parse("https://www.microsoft.com/foo?q=q").unwrap();
        url.append_path("/");
        assert_eq!(url.as_str(), "https://www.microsoft.com/foo?q=q");

        url = Url::parse("https://www.microsoft.com/foo/?q=q").unwrap();
        url.append_path("/");
        assert_eq!(url.as_str(), "https://www.microsoft.com/foo/?q=q");

        url = Url::parse("https://www.microsoft.com/foo?q=q").unwrap();
        url.append_path("");
        assert_eq!(url.as_str(), "https://www.microsoft.com/foo?q=q");

        url = Url::parse("https://www.microsoft.com/foo/?q=q").unwrap();
        url.append_path("");
        assert_eq!(url.as_str(), "https://www.microsoft.com/foo/?q=q");
    }
}

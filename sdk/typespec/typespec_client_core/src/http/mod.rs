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

/// Extension trait for `Url` to provide additional URL manipulation methods.
pub trait UrlExt {
    /// Appends a path segment to the URL's path, handling slashes appropriately and preserving query parameters.
    fn append_path(&mut self, p: &str);
}

impl UrlExt for Url {
    fn append_path(&mut self, p: &str) {
        if self.path().len() == 1 {
            self.set_path(p);
        } else if !p.is_empty() && p != "/" {
            match if self.path().ends_with('/') { 1 } else { 0 }
                + if p.starts_with('/') { 1 } else { 0 }
            {
                0 => self.set_path(&format!("{}/{}", self.path(), p).to_string()),
                1 => self.set_path(&(self.path().to_owned() + p)),
                _ => self.set_path(&(self.path()[..self.path().len() - 1].to_owned() + p)),
            }
        }
    }
}

#[test]
fn test_url_append_path() {
    {
        let mut url = Url::parse("https://www.microsoft.com?q=q").unwrap();
        url.append_path("alpha");
        assert_eq!(url.as_str(), "https://www.microsoft.com/alpha?q=q");
    }
    {
        let mut url = Url::parse("https://www.microsoft.com/?q=q").unwrap();
        url.append_path("alpha");
        assert_eq!(url.as_str(), "https://www.microsoft.com/alpha?q=q");
    }
    {
        let mut url = Url::parse("https://www.microsoft.com?q=q").unwrap();
        url.append_path("/alpha");
        assert_eq!(url.as_str(), "https://www.microsoft.com/alpha?q=q");
    }
    {
        let mut url = Url::parse("https://www.microsoft.com/?q=q").unwrap();
        url.append_path("/alpha");
        assert_eq!(url.as_str(), "https://www.microsoft.com/alpha?q=q");
    }
    {
        let mut url = Url::parse("https://www.microsoft.com?q=q").unwrap();
        url.append_path("alpha/");
        assert_eq!(url.as_str(), "https://www.microsoft.com/alpha/?q=q");
    }
    {
        let mut url = Url::parse("https://www.microsoft.com/?q=q").unwrap();
        url.append_path("alpha/");
        assert_eq!(url.as_str(), "https://www.microsoft.com/alpha/?q=q");
    }
    {
        let mut url = Url::parse("https://www.microsoft.com?q=q").unwrap();
        url.append_path("/alpha/");
        assert_eq!(url.as_str(), "https://www.microsoft.com/alpha/?q=q");
    }
    {
        let mut url = Url::parse("https://www.microsoft.com/?q=q").unwrap();
        url.append_path("/alpha/");
        assert_eq!(url.as_str(), "https://www.microsoft.com/alpha/?q=q");
    }
    {
        let mut url = Url::parse("https://www.microsoft.com/alpha?q=q").unwrap();
        url.append_path("beta");
        assert_eq!(url.as_str(), "https://www.microsoft.com/alpha/beta?q=q");
    }
    {
        let mut url = Url::parse("https://www.microsoft.com/alpha/?q=q").unwrap();
        url.append_path("beta");
        assert_eq!(url.as_str(), "https://www.microsoft.com/alpha/beta?q=q");
    }
    {
        let mut url = Url::parse("https://www.microsoft.com/alpha?q=q").unwrap();
        url.append_path("/beta");
        assert_eq!(url.as_str(), "https://www.microsoft.com/alpha/beta?q=q");
    }
    {
        let mut url = Url::parse("https://www.microsoft.com/alpha/?q=q").unwrap();
        url.append_path("/beta");
        assert_eq!(url.as_str(), "https://www.microsoft.com/alpha/beta?q=q");
    }
    {
        let mut url = Url::parse("https://www.microsoft.com/alpha?q=q").unwrap();
        url.append_path("beta/");
        assert_eq!(url.as_str(), "https://www.microsoft.com/alpha/beta/?q=q");
    }
    {
        let mut url = Url::parse("https://www.microsoft.com/alpha/?q=q").unwrap();
        url.append_path("beta/");
        assert_eq!(url.as_str(), "https://www.microsoft.com/alpha/beta/?q=q");
    }
    {
        let mut url = Url::parse("https://www.microsoft.com/alpha?q=q").unwrap();
        url.append_path("/beta/");
        assert_eq!(url.as_str(), "https://www.microsoft.com/alpha/beta/?q=q");
    }
    {
        let mut url = Url::parse("https://www.microsoft.com/alpha/?q=q").unwrap();
        url.append_path("/beta/");
        assert_eq!(url.as_str(), "https://www.microsoft.com/alpha/beta/?q=q");
    }
    {
        let mut url = Url::parse("https://www.microsoft.com?q=q").unwrap();
        url.append_path("/");
        assert_eq!(url.as_str(), "https://www.microsoft.com/?q=q");
    }
    {
        let mut url = Url::parse("https://www.microsoft.com/?q=q").unwrap();
        url.append_path("/");
        assert_eq!(url.as_str(), "https://www.microsoft.com/?q=q");
    }
    {
        let mut url = Url::parse("https://www.microsoft.com?q=q").unwrap();
        url.append_path("");
        assert_eq!(url.as_str(), "https://www.microsoft.com/?q=q");
    }
    {
        let mut url = Url::parse("https://www.microsoft.com?q=q").unwrap();
        url.append_path("");
        assert_eq!(url.as_str(), "https://www.microsoft.com/?q=q");
    }
    {
        let mut url = Url::parse("https://www.microsoft.com/alpha?q=q").unwrap();
        url.append_path("/");
        assert_eq!(url.as_str(), "https://www.microsoft.com/alpha?q=q");
    }
    {
        let mut url = Url::parse("https://www.microsoft.com/alpha/?q=q").unwrap();
        url.append_path("/");
        assert_eq!(url.as_str(), "https://www.microsoft.com/alpha/?q=q");
    }
    {
        let mut url = Url::parse("https://www.microsoft.com/alpha?q=q").unwrap();
        url.append_path("");
        assert_eq!(url.as_str(), "https://www.microsoft.com/alpha?q=q");
    }
    {
        let mut url = Url::parse("https://www.microsoft.com/alpha/?q=q").unwrap();
        url.append_path("");
        assert_eq!(url.as_str(), "https://www.microsoft.com/alpha/?q=q");
    }
}

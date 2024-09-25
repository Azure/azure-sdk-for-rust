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

/// Appends new path segments to the target [`Url`].
///
/// # Examples
/// ```rust
/// use url::Url;
///
/// let mut url: Url = "https://example.com/foo".parse().unwrap();
/// url.append_to_path(&["bar", "baz"]);
/// assert_eq!("https://example.com/foo/bar/baz", url.to_string());
/// ```
pub trait AppendPathSegments {
    fn append_path_segments<'a, T: AsRef<str>>(
        &mut self,

        // We have to use T:AsRef<str> here instead of just &str.
        // We want to be able to pass a slice of strings, which is &[&str].
        // Slices of T implement IntoIterator by producing an iterator that returns &T, which would be &&str in our case.
        // An iterator that returns &&str is NOT compatible with IntoIterator<Item = &str>, but IS compatible with IntoIterator<Item = T> (where T: AsRef<str>)
        segments: impl IntoIterator<Item = T>,
    ) -> ();
}

impl AppendPathSegments for Url {
    fn append_path_segments<'a, T: AsRef<str>>(
        &mut self,
        segments: impl IntoIterator<Item = T>,
    ) -> () {
        let mut path_segments = self
            .path_segments_mut()
            .expect("the URL must not be a 'cannot-be-a-base' URL");
        for segment in segments {
            path_segments.push(segment.as_ref());
        }
    }
}

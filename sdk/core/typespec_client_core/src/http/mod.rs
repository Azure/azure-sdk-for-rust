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
pub use response::{AsyncRawResponse, RawResponse, Response};
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

    /// Returns a query builder for setting query parameters.
    ///
    /// The builder allows setting multiple query parameters, overwriting any existing values
    /// with the same key. Call `build()` to apply the changes to the URL.
    ///
    /// # Examples
    ///
    /// ```
    /// use typespec_client_core::http::{Url, UrlExt as _};
    ///
    /// let mut url: Url = "https://contoso.com?a=1&b=2".parse().unwrap();
    /// url.query_builder()
    ///     .set_pair("a", "new_value")
    ///     .set_pair("c", "3")
    ///     .build();
    /// let params: Vec<_> = url.query_pairs().collect();
    /// assert!(params.contains(&("a".into(), "new_value".into())));
    /// assert!(params.contains(&("b".into(), "2".into())));
    /// assert!(params.contains(&("c".into(), "3".into())));
    /// ```
    fn query_builder(&mut self) -> QueryBuilder<'_>;
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

    fn query_builder(&mut self) -> QueryBuilder<'_> {
        QueryBuilder::new(self)
    }
}

/// A builder for setting query parameters on a URL.
///
/// This builder allows you to set multiple query parameters, overwriting any existing
/// values with the same key. Call [`build()`](QueryBuilder::build) to apply the changes.
pub struct QueryBuilder<'a> {
    url: &'a mut Url,
    values: std::collections::HashMap<std::borrow::Cow<'a, str>, Vec<std::borrow::Cow<'a, str>>>,
    dirty: bool,
}

impl<'a> QueryBuilder<'a> {
    fn new(url: &'a mut Url) -> Self {
        let mut values = std::collections::HashMap::new();

        // Parse existing query params into values
        for (key, value) in url.query_pairs() {
            values
                .entry(key.into_owned().into())
                .or_insert_with(Vec::new)
                .push(value.into_owned().into());
        }

        Self {
            url,
            values,
            dirty: false,
        }
    }

    /// Appends a key without a value to the URL query string.
    ///
    /// This is useful for boolean flags or markers in query strings that don't require a value.
    ///
    /// Returns `&mut Self` to allow chaining multiple calls.
    ///
    /// # Examples
    ///
    /// ```
    /// use typespec_client_core::http::{Url, UrlExt as _};
    ///
    /// let mut url: Url = "https://contoso.com".parse().unwrap();
    /// url.query_builder()
    ///     .append_key_only("debug")
    ///     .append_pair("a", "1")
    ///     .build();
    /// let params: Vec<_> = url.query_pairs().collect();
    /// assert!(params.contains(&("debug".into(), "".into())));
    /// assert!(params.contains(&("a".into(), "1".into())));
    /// ```
    pub fn append_key_only(&mut self, key: impl Into<std::borrow::Cow<'a, str>>) -> &mut Self {
        let key = key.into();

        if let Some(vals) = self.values.get_mut(&key) {
            vals.push("".into());
        } else {
            self.values.insert(key, vec!["".into()]);
        }

        self.dirty = true;
        self
    }

    /// Appends a query parameter to the URL.
    ///
    /// If the key already exists, this adds an additional value for that key (allowing duplicates).
    /// Use [`set_pair`](Self::set_pair) to overwrite existing values instead.
    ///
    /// Both key and value accept types that can be converted to `Cow<'a, str>`, which includes
    /// `&'a str`, `String`, and `Cow<'a, str>`. This avoids unnecessary allocations when using
    /// string literals.
    ///
    /// Returns `&mut Self` to allow chaining multiple calls.
    ///
    /// # Examples
    ///
    /// ```
    /// use typespec_client_core::http::{Url, UrlExt as _};
    ///
    /// let mut url: Url = "https://contoso.com?a=1".parse().unwrap();
    /// url.query_builder()
    ///     .append_pair("a", "2")
    ///     .append_pair("b", "3")
    ///     .build();
    /// let params: Vec<_> = url.query_pairs().collect();
    /// assert!(params.contains(&("a".into(), "1".into())));
    /// assert!(params.contains(&("a".into(), "2".into())));
    /// assert!(params.contains(&("b".into(), "3".into())));
    /// ```
    pub fn append_pair(
        &mut self,
        key: impl Into<std::borrow::Cow<'a, str>>,
        value: impl Into<std::borrow::Cow<'a, str>>,
    ) -> &mut Self {
        let key = key.into();
        let value = value.into();

        if let Some(vals) = self.values.get_mut(&key) {
            vals.push(value);
        } else {
            self.values.insert(key, vec![value]);
        }

        self.dirty = true;
        self
    }

    /// Sets a query parameter, overwriting any existing value with the same key.
    ///
    /// If the key already exists, all existing values for that key are replaced with the new value.
    /// If the key doesn't exist, it's added to the end of the query parameters.
    /// Use [`append_pair`](Self::append_pair) to add additional values without replacing existing ones.
    ///
    /// Both key and value accept types that can be converted to `Cow<'a, str>`, which includes
    /// `&'a str`, `String`, and `Cow<'a, str>`. This avoids unnecessary allocations when using
    /// string literals.
    ///
    /// Returns `&mut Self` to allow chaining multiple calls.
    ///
    /// # Examples
    ///
    /// ```
    /// use typespec_client_core::http::{Url, UrlExt as _};
    ///
    /// let mut url: Url = "https://contoso.com?a=1&b=2".parse().unwrap();
    /// url.query_builder()
    ///     .set_pair("a", "new_value")
    ///     .set_pair("c", "3")
    ///     .build();
    /// let params: Vec<_> = url.query_pairs().collect();
    /// assert!(params.contains(&("a".into(), "new_value".into())));
    /// assert!(params.contains(&("b".into(), "2".into())));
    /// assert!(params.contains(&("c".into(), "3".into())));
    /// ```
    pub fn set_pair(
        &mut self,
        key: impl Into<std::borrow::Cow<'a, str>>,
        value: impl Into<std::borrow::Cow<'a, str>>,
    ) -> &mut Self {
        let key = key.into();
        let value = value.into();

        // Replace existing values (taking ownership)
        self.values.insert(key, vec![value]);

        self.dirty = true;
        self
    }

    /// Applies all the query parameter changes to the URL.
    pub fn build(&mut self) {
        if !self.dirty {
            return;
        }

        // Rebuild the query string with all values from the HashMap
        self.url.query_pairs_mut().clear();

        let mut serializer = self.url.query_pairs_mut();
        for (key, values) in &self.values {
            for value in values {
                if value.is_empty() {
                    serializer.append_key_only(key);
                } else {
                    serializer.append_pair(key, value);
                }
            }
        }

        self.dirty = false;
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

    #[test]
    fn test_query_builder_empty_query() {
        let mut url = Url::parse("https://contoso.com").unwrap();
        let mut builder = url.query_builder();
        builder.set_pair("a", "1");
        builder.build();
        assert_eq!(url.as_str(), "https://contoso.com/?a=1");
    }

    #[test]
    fn test_query_builder_new_parameter() {
        let mut url = Url::parse("https://contoso.com?b=2").unwrap();
        let mut builder = url.query_builder();
        builder.set_pair("a", "1");
        builder.build();
        assert_eq!(url.as_str(), "https://contoso.com/?a=1&b=2");
    }

    #[test]
    fn test_query_builder_overwrite_existing() {
        let mut url = Url::parse("https://contoso.com?a=1&b=2").unwrap();
        let mut builder = url.query_builder();
        builder.set_pair("a", "new_value");
        builder.build();
        let params: Vec<_> = url.query_pairs().collect();
        assert!(params.contains(&("a".into(), "new_value".into())));
        assert!(params.contains(&("b".into(), "2".into())));
        assert_eq!(params.len(), 2);
    }

    #[test]
    fn test_query_builder_overwrite_duplicate() {
        let mut url = Url::parse("https://contoso.com?a=1&b=2&a=3").unwrap();
        let mut builder = url.query_builder();
        builder.set_pair("a", "new_value");
        builder.build();

        let params: Vec<_> = url.query_pairs().collect();
        assert!(params.contains(&("a".into(), "new_value".into())));
        assert!(params.contains(&("b".into(), "2".into())));
        assert_eq!(params.len(), 2);
    }

    #[test]
    fn test_query_builder_with_hashmap() {
        let mut url = Url::parse("https://contoso.com?x=1&a=old&y=2&z=3").unwrap();
        let mut builder = url.query_builder();
        builder.set_pair("a", "new");
        builder.build();

        let params: Vec<_> = url.query_pairs().collect();
        assert!(params.contains(&("a".into(), "new".into())));
        assert!(params.contains(&("x".into(), "1".into())));
        assert!(params.contains(&("y".into(), "2".into())));
        assert!(params.contains(&("z".into(), "3".into())));
        assert_eq!(params.len(), 4);
    }

    #[test]
    fn test_query_builder_with_special_chars() {
        let mut url = Url::parse("https://contoso.com?a=old").unwrap();
        let mut builder = url.query_builder();
        builder.set_pair("a", "hello world");
        builder.build();
        assert_eq!(url.as_str(), "https://contoso.com/?a=hello+world");
    }

    #[test]
    fn test_query_builder_multiple_sets() {
        let mut url = Url::parse("https://contoso.com?a=1&b=2").unwrap();
        let mut builder = url.query_builder();
        builder
            .set_pair("a", "new")
            .set_pair("c", "3")
            .set_pair("b", "updated");
        builder.build();
        let params: Vec<_> = url.query_pairs().collect();
        assert!(params.contains(&("a".into(), "new".into())));
        assert!(params.contains(&("b".into(), "updated".into())));
        assert!(params.contains(&("c".into(), "3".into())));
        assert_eq!(params.len(), 3);
    }

    #[test]
    fn test_query_builder_with_numeric_value() {
        let mut url = Url::parse("https://contoso.com").unwrap();
        let mut builder = url.query_builder();
        builder.set_pair("foo", 1.to_string()).set_pair("bar", "2");
        builder.build();
        let params: Vec<_> = url.query_pairs().collect();
        assert!(params.contains(&("bar".into(), "2".into())));
        assert!(params.contains(&("foo".into(), "1".into())));
        assert_eq!(params.len(), 2);
    }

    #[test]
    fn test_query_builder_append_key_only() {
        let mut url = Url::parse("https://contoso.com").unwrap();
        let mut builder = url.query_builder();
        builder
            .append_key_only("debug")
            .append_pair("a", "1")
            .append_key_only("verbose");
        builder.build();

        let params: Vec<_> = url.query_pairs().collect();
        assert!(params.contains(&("a".into(), "1".into())));
        assert!(params.contains(&("debug".into(), "".into())));
        assert!(params.contains(&("verbose".into(), "".into())));
        assert_eq!(params.len(), 3);
    }

    #[test]
    fn test_query_builder_multiple_builds() {
        let mut url = Url::parse("https://contoso.com").unwrap();

        let mut builder = url.query_builder();
        builder.set_pair("a", "1");
        builder.build();

        builder.set_pair("b", "2");
        builder.build();

        // Calling build() again without changes does nothing
        builder.build();
        let params: Vec<_> = url.query_pairs().collect();
        assert!(params.contains(&("a".into(), "1".into())));
        assert!(params.contains(&("b".into(), "2".into())));
        assert_eq!(params.len(), 2);
    }
}

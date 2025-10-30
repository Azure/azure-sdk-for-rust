use async_trait::async_trait;
use azure_core::http::{
    headers::Headers,
    pager::{Page, PagerResult},
    ItemIterator, RawResponse,
};
use serde::{de::DeserializeOwned, Deserialize};

use crate::constants;

/// Represents a single page of results from a Cosmos DB feed.
///
/// A feed could be a list of items, databases, containers, etc.
/// The feed may represent a single-partition or cross-partition query.
///
/// Cosmos DB queries can be executed using non-HTTP transports, depending on the circumstances.
/// They may also produce results that don't directly correlate to specific HTTP responses (as in the case of cross-partition queries).
/// Because of this, Cosmos DB query responses use `FeedPage` to represent the results, rather than a more generic type like [`Response`](azure_core::http::Response).
#[derive(Debug)]
pub struct FeedPage<T> {
    /// The items in the response.
    items: Vec<T>,

    /// The continuation token for the next page of results.
    continuation: Option<String>,

    /// Response headers from the server for this page of results.
    /// In a cross-partition query, these headers may be missing on some pages.
    headers: Headers,
}

impl<T> FeedPage<T> {
    /// Creates a new `FeedPage` instance.
    #[cfg_attr(not(feature = "preview_query_engine"), allow(dead_code))]
    pub(crate) fn new(items: Vec<T>, continuation: Option<String>, headers: Headers) -> Self {
        Self {
            items,
            continuation,
            headers,
        }
    }

    /// Gets the items in this page of results.
    pub fn items(&self) -> &[T] {
        &self.items
    }

    /// Consumes the page and returns a vector of the items.
    ///
    /// This is essentially shorthand for `self.deconstruct().0`.
    pub fn into_items(self) -> Vec<T> {
        self.items
    }

    /// Deconstructs the page into its components.
    pub fn deconstruct(self) -> (Vec<T>, Option<String>, Headers) {
        (self.items, self.continuation, self.headers)
    }

    /// Gets the continuation token for the next page of results, if any.
    pub fn continuation(&self) -> Option<&str> {
        self.continuation.as_deref()
    }

    /// Gets any headers returned by the server for this page of results.
    pub fn headers(&self) -> &Headers {
        &self.headers
    }
}

impl<T> From<FeedPage<T>> for PagerResult<FeedPage<T>, String> {
    fn from(value: FeedPage<T>) -> Self {
        let continuation = value.continuation.clone();
        match continuation {
            Some(continuation) => PagerResult::More {
                response: value,
                continuation,
            },
            None => PagerResult::Done { response: value },
        }
    }
}

#[derive(Deserialize)]
struct FeedBody<T> {
    #[serde(alias = "Documents")]
    #[serde(alias = "DocumentCollections")]
    #[serde(alias = "Databases")]
    #[serde(alias = "Offers")]
    items: Vec<T>,
}

impl<T: DeserializeOwned> FeedPage<T> {
    pub(crate) async fn from_response(response: RawResponse) -> azure_core::Result<Self> {
        let headers = response.headers().clone();
        let continuation = headers.get_optional_string(&constants::CONTINUATION);
        let body: FeedBody<T> = response.into_body().json()?;

        Ok(Self {
            items: body.items,
            continuation,
            headers,
        })
    }
}

#[cfg_attr(not(target_arch = "wasm32"), async_trait)]
#[cfg_attr(target_arch = "wasm32", async_trait(?Send))]
impl<T: DeserializeOwned + Send> Page for FeedPage<T> {
    type Item = T;
    type IntoIter = <Vec<T> as IntoIterator>::IntoIter;
    async fn into_items(self) -> azure_core::Result<Self::IntoIter> {
        Ok(self.items.into_iter())
    }
}

/// Represents a stream of pages from a Cosmos DB feed.
///
/// See [`FeedPage`] for more details on Cosmos DB feeds.
pub type FeedPager<T> = ItemIterator<FeedPage<T>>;

// Copyright (c) Microsoft Corporation. All rights reserved.
// Licensed under the MIT License.

use std::{pin::Pin, task};

use azure_core::http::{headers::Headers, pager::PagerResult};
#[cfg(not(target_arch = "wasm32"))]
use futures::stream::BoxStream;
use futures::Stream;
use serde::{de::DeserializeOwned, Deserialize};

use crate::{conditional_send::ConditionalSend, constants, models::CosmosResponse};

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
    #[allow(dead_code)]
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

impl<T> From<FeedPage<T>> for PagerResult<FeedPage<T>> {
    fn from(value: FeedPage<T>) -> Self {
        let continuation = value.continuation.clone();
        match continuation {
            Some(continuation) => PagerResult::More {
                response: value,
                continuation: continuation.into(),
            },
            None => PagerResult::Done { response: value },
        }
    }
}

#[derive(Deserialize)]
pub(crate) struct FeedBody<T> {
    #[serde(alias = "Documents")]
    #[serde(alias = "DocumentCollections")]
    #[serde(alias = "Databases")]
    #[serde(alias = "Offers")]
    pub(crate) items: Vec<T>,
}

impl<T: DeserializeOwned> FeedPage<T> {
    pub(crate) async fn from_response(
        response: CosmosResponse<FeedBody<T>>,
    ) -> azure_core::Result<Self> {
        let headers = response.headers().clone();
        let continuation = headers.get_optional_string(&constants::CONTINUATION);
        let body: FeedBody<T> = response.into_model()?;

        Ok(Self {
            items: body.items,
            continuation,
            headers,
        })
    }
}

/// Represents a stream of pages from a Cosmos DB feed.
///
/// See [`FeedPage`] for more details on Cosmos DB feeds.
#[pin_project::pin_project]
pub struct FeedItemIterator<T: ConditionalSend> {
    #[pin]
    #[cfg(not(target_arch = "wasm32"))]
    pages: BoxStream<'static, azure_core::Result<FeedPage<T>>>,
    #[pin]
    #[cfg(target_arch = "wasm32")]
    pages: Pin<Box<dyn Stream<Item = azure_core::Result<FeedPage<T>>> + 'static>>,
    current: Option<std::vec::IntoIter<T>>,
}

impl<T: ConditionalSend> FeedItemIterator<T> {
    /// Creates a new `FeedItemIterator` from a stream of pages.
    pub(crate) fn new(
        stream: impl Stream<Item = azure_core::Result<FeedPage<T>>> + ConditionalSend + 'static,
    ) -> Self {
        Self {
            pages: Box::pin(stream),
            current: None,
        }
    }

    pub fn into_pages(self) -> FeedPageIterator<T> {
        FeedPageIterator(self.pages)
    }
}

impl<T: ConditionalSend> Stream for FeedItemIterator<T> {
    type Item = azure_core::Result<T>;

    fn poll_next(
        self: Pin<&mut Self>,
        cx: &mut task::Context<'_>,
    ) -> task::Poll<Option<Self::Item>> {
        let mut this = self.project();
        loop {
            if let Some(current) = this.current.as_mut() {
                if let Some(item) = current.next() {
                    return task::Poll::Ready(Some(Ok(item)));
                }

                // Reset the iterator and poll for the next page.
                *this.current = None;
            }

            match this.pages.as_mut().poll_next(cx) {
                task::Poll::Ready(page) => match page {
                    Some(Ok(page)) => {
                        *this.current = Some(page.items.into_iter());
                        continue;
                    }
                    Some(Err(err)) => return task::Poll::Ready(Some(Err(err))),
                    None => return task::Poll::Ready(None),
                },
                task::Poll::Pending => return task::Poll::Pending,
            }
        }
    }
}

pub struct FeedPageIterator<T: ConditionalSend>(
    #[cfg(not(target_arch = "wasm32"))] BoxStream<'static, azure_core::Result<FeedPage<T>>>,
    #[cfg(target_arch = "wasm32")]
    Pin<Box<dyn Stream<Item = azure_core::Result<FeedPage<T>>> + 'static>>,
);

impl<T: ConditionalSend> Stream for FeedPageIterator<T> {
    type Item = azure_core::Result<FeedPage<T>>;

    fn poll_next(
        mut self: Pin<&mut Self>,
        cx: &mut task::Context<'_>,
    ) -> task::Poll<Option<Self::Item>> {
        self.0.as_mut().poll_next(cx)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use futures::StreamExt;

    fn create_test_page<T>(items: Vec<T>, continuation: Option<String>) -> FeedPage<T> {
        FeedPage::new(items, continuation, Headers::new())
    }

    #[tokio::test]
    async fn item_iterator_yields_all_items_from_multiple_pages() {
        let pages = vec![
            Ok(create_test_page(vec![1, 2, 3], Some("token1".to_string()))),
            Ok(create_test_page(vec![4, 5], Some("token2".to_string()))),
            Ok(create_test_page(vec![6], None)),
        ];

        let stream = futures::stream::iter(pages);
        let item_iter = FeedItemIterator::new(stream);

        let items: Vec<_> = item_iter
            .collect::<Vec<_>>()
            .await
            .into_iter()
            .map(|r| r.unwrap())
            .collect();
        assert_eq!(items, vec![1, 2, 3, 4, 5, 6]);
    }

    #[tokio::test]
    async fn page_iterator_yields_all_pages() {
        let pages = vec![
            Ok(create_test_page(vec![1, 2], Some("token1".to_string()))),
            Ok(create_test_page(vec![3], None)),
        ];

        let stream = futures::stream::iter(pages);
        let page_iter = FeedItemIterator::new(stream).into_pages();

        let page_items: Vec<_> = page_iter
            .collect::<Vec<_>>()
            .await
            .into_iter()
            .map(|r| r.unwrap().into_items())
            .collect();
        assert_eq!(page_items, vec![vec![1, 2], vec![3]]);
    }

    #[tokio::test]
    async fn item_iterator_propagates_errors() {
        let pages = vec![
            Ok(create_test_page(vec![1, 2], Some("token".to_string()))),
            Err(azure_core::Error::new(
                azure_core::error::ErrorKind::Other,
                "test error",
            )),
        ];

        let stream = futures::stream::iter(pages);
        let mut item_iter = FeedItemIterator::new(stream);

        // First two items should succeed
        assert_eq!(item_iter.next().await.unwrap().unwrap(), 1);
        assert_eq!(item_iter.next().await.unwrap().unwrap(), 2);

        // Third item should be an error
        assert!(item_iter.next().await.unwrap().is_err());
    }

    #[tokio::test]
    async fn item_iterator_handles_empty_pages() {
        let pages = vec![
            Ok(create_test_page(vec![1], Some("token1".to_string()))),
            Ok(create_test_page(vec![], Some("token2".to_string()))),
            Ok(create_test_page(vec![2], None)),
        ];

        let stream = futures::stream::iter(pages);
        let item_iter = FeedItemIterator::new(stream);

        let items: Vec<_> = item_iter
            .collect::<Vec<_>>()
            .await
            .into_iter()
            .map(|r| r.unwrap())
            .collect();

        assert_eq!(items, vec![1, 2]);
    }
}

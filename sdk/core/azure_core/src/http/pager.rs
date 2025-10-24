// Copyright (c) Microsoft Corporation. All rights reserved.
// Licensed under the MIT License.

//! Types and methods for pageable responses.

use crate::http::{headers::HeaderName, response::Response, DeserializeWith, Format, JsonFormat};
use async_trait::async_trait;
use futures::{stream::unfold, FutureExt, Stream};
use std::{
    fmt,
    future::Future,
    ops::Deref,
    pin::Pin,
    str::FromStr,
    sync::{Arc, Mutex},
    task,
};

/// Represents the state of a [`Pager`] or [`PageIterator`].
#[derive(Debug, Default, PartialEq, Eq)]
pub enum PagerState<C: AsRef<str>> {
    /// The pager should fetch the initial page.
    #[default]
    Initial,
    /// The pager should fetch a subsequent page using the next link/continuation token `C`.
    More(C),
}

impl<C: AsRef<str>> PagerState<C> {
    /// Maps a `PagerState<C>` to a `PagerState<U>` by applying a function to a next link/continuation token `C` (if `PagerState::More`) or returns `PagerState::Initial` (if `PagerState::Initial`).
    #[inline]
    pub fn map<U, F>(self, f: F) -> PagerState<U>
    where
        U: AsRef<str>,
        F: FnOnce(C) -> U,
    {
        match self {
            PagerState::Initial => PagerState::Initial,
            PagerState::More(c) => PagerState::More(f(c)),
        }
    }

    /// Converts from `&PagerState<C>` to `PagerState<&C>`.
    #[inline]
    pub const fn as_ref(&self) -> PagerState<&C> {
        match *self {
            PagerState::Initial => PagerState::Initial,
            PagerState::More(ref x) => PagerState::More(x),
        }
    }

    /// Converts from `PagerState<C>` (or `&PagerState<C>`) to `PagerState<&C::Target>`.
    ///
    /// Leaves the original `PagerState` in-place, creating a new one with a reference
    /// to the original one, additionally coercing the contents via [`Deref`].
    #[inline]
    pub fn as_deref(&self) -> PagerState<&C::Target>
    where
        C: Deref,
        C::Target: AsRef<str>,
    {
        self.as_ref().map(|t| t.deref())
    }
}

impl<C: Clone + AsRef<str>> Clone for PagerState<C> {
    #[inline]
    fn clone(&self) -> Self {
        match self {
            PagerState::Initial => PagerState::Initial,
            PagerState::More(c) => PagerState::More(c.clone()),
        }
    }
}

/// The result of fetching a single page from a [`Pager`], whether there are more pages or paging is done.
pub enum PagerResult<P, C: AsRef<str>> {
    /// There are more pages the [`Pager`] may fetch using the `continuation` token.
    More {
        /// The response for the current page.
        response: P,
        /// The continuation token for the next page.
        continuation: C,
    },
    /// The [`Pager`] is done and there are no additional pages to fetch.
    Done {
        /// The response for the current page.
        response: P,
    },
}

impl<P, F> PagerResult<Response<P, F>, String> {
    /// Creates a [`PagerResult<P, C>`] from the provided response, extracting the continuation value from the provided header.
    ///
    /// If the provided response has a header with the matching name, this returns [`PagerResult::More`], using the value from the header as the continuation.
    /// If the provided response does not have a header with the matching name, this returns [`PagerResult::Done`].
    pub fn from_response_header(response: Response<P, F>, header_name: &HeaderName) -> Self {
        match response.headers().get_optional_string(header_name) {
            Some(continuation) => PagerResult::More {
                response,
                continuation,
            },
            None => PagerResult::Done { response },
        }
    }
}

impl<P, C: fmt::Debug + AsRef<str>> fmt::Debug for PagerResult<P, C> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::More { continuation, .. } => f
                .debug_struct("More")
                .field("continuation", &continuation)
                .finish_non_exhaustive(),
            Self::Done { .. } => f.debug_struct("Done").finish_non_exhaustive(),
        }
    }
}

/// Represents a single page of items returned by a collection request to a service.
#[cfg_attr(not(target_arch = "wasm32"), async_trait)]
#[cfg_attr(target_arch = "wasm32", async_trait(?Send))]
pub trait Page {
    /// The type of items in the collection.
    type Item;
    /// The type containing items in the collection e.g., [`Vec<Self::Item>`](Vec).
    type IntoIter: Iterator<Item = Self::Item>;

    /// Gets a single page of items returned by a collection request to a service.
    async fn into_items(self) -> crate::Result<Self::IntoIter>;
}

#[cfg_attr(not(target_arch = "wasm32"), async_trait)]
#[cfg_attr(target_arch = "wasm32", async_trait(?Send))]
impl<P, F> Page for Response<P, F>
where
    P: DeserializeWith<F> + Page + Send,
    F: Format + Send,
{
    type Item = P::Item;
    type IntoIter = P::IntoIter;
    async fn into_items(self) -> crate::Result<Self::IntoIter> {
        let page: P = self.into_body()?;
        page.into_items().await
    }
}

/// Represents a paginated stream of items returned by a collection request to a service.
///
/// Specifically, this is a [`ItemIterator`] that yields [`Response<T>`] items.
///
/// # Examples
///
/// For clients that return a `Pager`, you can iterate over items across one or more pages:
///
/// ```no_run
/// # use azure_core::{credentials::TokenCredential, http::Transport};
/// # use azure_security_keyvault_secrets::{ResourceExt, SecretClient, SecretClientOptions};
/// # use futures::TryStreamExt;
/// # async fn example() -> Result<(), Box<dyn std::error::Error>> {
/// # let credential: std::sync::Arc<dyn TokenCredential> = unimplemented!();
/// let client = SecretClient::new(
///     "https://my-vault.vault.azure.net",
///     credential.clone(),
///     None,
/// )?;
///
/// // List secret properties using a Pager.
/// let mut pager = client.list_secret_properties(None)?;
/// while let Some(secret) = pager.try_next().await? {
///     println!("{}", secret.resource_id()?.name);
/// }
/// # Ok(()) }
/// ```
///
/// If you want to iterate each page of items, you can call [`Pager::into_pages`] to get a [`PageIterator`]:
///
/// ```no_run
/// # use azure_core::{credentials::TokenCredential, http::Transport};
/// # use azure_security_keyvault_secrets::{ResourceExt, SecretClient, SecretClientOptions};
/// # use futures::TryStreamExt;
/// # async fn example() -> Result<(), Box<dyn std::error::Error>> {
/// # let credential: std::sync::Arc<dyn TokenCredential> = unimplemented!();
/// let client = SecretClient::new(
///     "https://my-vault.vault.azure.net",
///     credential.clone(),
///     None,
/// )?;
///
/// // Iterate each page of secrets using a PageIterator.
/// let mut pager = client.list_secret_properties(None)?.into_pages();
/// while let Some(page) = pager.try_next().await? {
///     let page = page.into_body()?;
///     for secret in page.value {
///         println!("{}", secret.resource_id()?.name);
///     }
/// }
/// # Ok(()) }
/// ```
pub type Pager<P, F = JsonFormat> = ItemIterator<Response<P, F>>;

#[cfg(not(target_arch = "wasm32"))]
type BoxedStream<P> = Box<dyn Stream<Item = crate::Result<P>> + Send>;

#[cfg(target_arch = "wasm32")]
type BoxedStream<P> = Box<dyn Stream<Item = crate::Result<P>>>;

/// Iterates over a collection of items or individual pages of items from a service.
///
/// You can asynchronously iterate over items returned by a collection request to a service,
/// or asynchronously fetch pages of items if preferred.
///
/// # Examples
///
/// For clients that return a `Pager`, you can iterate over items across one or more pages:
///
/// ```no_run
/// # use azure_core::{credentials::TokenCredential, http::Transport};
/// # use azure_security_keyvault_secrets::{ResourceExt, SecretClient, SecretClientOptions};
/// # use futures::TryStreamExt;
/// # async fn example() -> Result<(), Box<dyn std::error::Error>> {
/// # let credential: std::sync::Arc<dyn TokenCredential> = unimplemented!();
/// let client = SecretClient::new(
///     "https://my-vault.vault.azure.net",
///     credential.clone(),
///     None,
/// )?;
///
/// // List secret properties using a Pager.
/// let mut pager = client.list_secret_properties(None)?;
/// while let Some(secret) = pager.try_next().await? {
///     println!("{}", secret.resource_id()?.name);
/// }
/// # Ok(()) }
/// ```
///
/// If you want to iterate each page of items, you can call [`Pager::into_pages`] to get a [`PageIterator`]:
///
/// ```no_run
/// # use azure_core::{credentials::TokenCredential, http::Transport};
/// # use azure_security_keyvault_secrets::{ResourceExt, SecretClient, SecretClientOptions};
/// # use futures::TryStreamExt;
/// # async fn example() -> Result<(), Box<dyn std::error::Error>> {
/// # let credential: std::sync::Arc<dyn TokenCredential> = unimplemented!();
/// let client = SecretClient::new(
///     "https://my-vault.vault.azure.net",
///     credential.clone(),
///     None,
/// )?;
///
/// // Iterate each page of secrets using a PageIterator.
/// let mut pager = client.list_secret_properties(None)?.into_pages();
/// while let Some(page) = pager.try_next().await? {
///     let page = page.into_body()?;
///     for secret in page.value {
///         println!("{}", secret.resource_id()?.name);
///     }
/// }
/// # Ok(()) }
/// ```
#[pin_project::pin_project]
pub struct ItemIterator<P: Page> {
    #[pin]
    stream: Pin<BoxedStream<P>>,
    current: Option<P::IntoIter>,
}

impl<P: Page> ItemIterator<P> {
    /// Creates a [`ItemIterator<P>`] from a callback that will be called repeatedly to request each page.
    ///
    /// This method expect a callback that accepts a single [`PagerState<C>`] parameter, and returns a [`PagerResult<T, C>`] value asynchronously.
    /// The `C` type parameter is the type of the next link/continuation token. It may be any [`Send`]able type.
    /// The result will be an asynchronous stream of [`Result<T>`](typespec::Result<T>) values.
    ///
    /// The first time your callback is called, it will be called with [`Option::None`], indicating no next link/continuation token is present.
    ///
    /// Your callback must return one of:
    /// * `Ok(result)` - The request succeeded, and the provided [`PagerResult`] indicates the value to return and if there are more pages.
    /// * `Err(..)` - The request failed. The error will be yielded to the stream, the stream will end, and the callback will not be called again.
    ///
    /// ## Examples
    ///
    /// To page results using a next link:
    ///
    /// ```rust,no_run
    /// # use azure_core::{Result, http::{RawResponse, Context, ItemIterator, pager::{Page, PagerResult, PagerState}, Pipeline, Request, Response, Method, Url}, json};
    /// # let api_version = "2025-06-04".to_string();
    /// # let pipeline: Pipeline = panic!("Not a runnable example");
    /// #[derive(serde::Deserialize)]
    /// struct ListItemsResult {
    ///     items: Vec<String>,
    ///     next_link: Option<String>,
    /// }
    /// #[cfg_attr(not(target_arch = "wasm32"), async_trait::async_trait)]
    /// #[cfg_attr(target_arch = "wasm32", async_trait::async_trait(?Send))]
    /// impl Page for ListItemsResult {
    ///     type Item = String;
    ///     type IntoIter = <Vec<String> as IntoIterator>::IntoIter;
    ///     async fn into_items(self) -> Result<Self::IntoIter> {
    ///         Ok(self.items.into_iter())
    ///     }
    /// }
    /// let url = "https://example.com/my_paginated_api".parse().unwrap();
    /// let mut base_req = Request::new(url, Method::Get);
    /// let pager = ItemIterator::from_callback(move |next_link: PagerState<Url>| {
    ///     // The callback must be 'static, so you have to clone and move any values you want to use.
    ///     let pipeline = pipeline.clone();
    ///     let api_version = api_version.clone();
    ///     let mut req = base_req.clone();
    ///     async move {
    ///         if let PagerState::More(next_link) = next_link {
    ///             // Ensure the api-version from the client is appended.
    ///             let qp = next_link
    ///                 .query_pairs()
    ///                 .filter(|(name, _)| name.ne("api-version"));
    ///             req
    ///                 .url_mut()
    ///                 .query_pairs_mut()
    ///                 .clear()
    ///                 .extend_pairs(qp)
    ///                 .append_pair("api-version", &api_version);
    ///         }
    ///         let resp = pipeline
    ///           .send(&Context::new(), &mut req, None)
    ///           .await?;
    ///         let (status, headers, body) = resp.deconstruct();
    ///         let result: ListItemsResult = json::from_json(&body)?;
    ///         let resp: Response<ListItemsResult> = RawResponse::from_bytes(status, headers, body).into();
    ///         Ok(match result.next_link {
    ///             Some(next_link) => PagerResult::More {
    ///                 response: resp,
    ///                 continuation: next_link.parse()?,
    ///             },
    ///             None => PagerResult::Done { response: resp }
    ///         })
    ///     }
    /// });
    /// ```
    ///
    /// To page results using headers:
    ///
    /// ```rust,no_run
    /// # use azure_core::{Result, http::{Context, ItemIterator, pager::{Page, PagerResult, PagerState}, Pipeline, Request, Response, Method, headers::HeaderName}};
    /// # let pipeline: Pipeline = panic!("Not a runnable example");
    /// #[derive(serde::Deserialize)]
    /// struct ListItemsResult {
    ///     items: Vec<String>,
    /// }
    /// #[cfg_attr(not(target_arch = "wasm32"), async_trait::async_trait)]
    /// #[cfg_attr(target_arch = "wasm32", async_trait::async_trait(?Send))]
    /// impl Page for ListItemsResult {
    ///     type Item = String;
    ///     type IntoIter = <Vec<String> as IntoIterator>::IntoIter;
    ///     async fn into_items(self) -> Result<Self::IntoIter> {
    ///         Ok(self.items.into_iter())
    ///     }
    /// }
    /// let url = "https://example.com/my_paginated_api".parse().unwrap();
    /// let mut base_req = Request::new(url, Method::Get);
    /// let pager = ItemIterator::from_callback(move |continuation| {
    ///     // The callback must be 'static, so you have to clone and move any values you want to use.
    ///     let pipeline = pipeline.clone();
    ///     let mut req = base_req.clone();
    ///     async move {
    ///         if let PagerState::More(continuation) = continuation {
    ///             req.insert_header("x-ms-continuation", continuation);
    ///         }
    ///         let resp: Response<ListItemsResult> = pipeline
    ///           .send(&Context::new(), &mut req, None)
    ///           .await?
    ///           .into();
    ///         Ok(PagerResult::from_response_header(resp, &HeaderName::from_static("x-next-continuation")))
    ///     }
    /// });
    /// ```
    pub fn from_callback<
        // This is a bit gnarly, but the only thing that differs between the WASM/non-WASM configs is the presence of Send bounds.
        #[cfg(not(target_arch = "wasm32"))] C: AsRef<str> + Send + 'static,
        #[cfg(not(target_arch = "wasm32"))] F: Fn(PagerState<C>) -> Fut + Send + 'static,
        #[cfg(not(target_arch = "wasm32"))] Fut: Future<Output = crate::Result<PagerResult<P, C>>> + Send + 'static,
        #[cfg(target_arch = "wasm32")] C: AsRef<str> + 'static,
        #[cfg(target_arch = "wasm32")] F: Fn(PagerState<C>) -> Fut + 'static,
        #[cfg(target_arch = "wasm32")] Fut: Future<Output = crate::Result<PagerResult<P, C>>> + 'static,
    >(
        make_request: F,
    ) -> Self {
        Self::from_stream(iter_from_callback(make_request, || None, |_| {}))
    }

    /// Creates a [`ItemIterator<P>`] from a raw stream of [`Result<P>`](crate::Result<P>) values.
    ///
    /// This constructor is used when you are implementing a completely custom stream and want to use it as a pager.
    pub fn from_stream<
        // This is a bit gnarly, but the only thing that differs between the WASM/non-WASM configs is the presence of Send bounds.
        #[cfg(not(target_arch = "wasm32"))] S: Stream<Item = crate::Result<P>> + Send + 'static,
        #[cfg(target_arch = "wasm32")] S: Stream<Item = crate::Result<P>> + 'static,
    >(
        stream: S,
    ) -> Self {
        Self {
            stream: Box::pin(stream),
            current: None,
        }
    }

    /// Gets a [`PageIterator<P>`] to iterate over a collection of pages from a service.
    ///
    /// You can use this to asynchronously iterate pages returned by a collection request to a service.
    /// This allows you to get the individual pages' [`Response<P>`], from which you can iterate items in each page
    /// or deserialize the raw response as appropriate.
    pub fn into_pages(self) -> PageIterator<P> {
        PageIterator {
            stream: self.stream,
            continuation_token: Default::default(),
        }
    }
}

impl<P: Page> futures::Stream for ItemIterator<P> {
    type Item = crate::Result<P::Item>;

    fn poll_next(
        self: Pin<&mut Self>,
        cx: &mut task::Context<'_>,
    ) -> task::Poll<Option<Self::Item>> {
        let mut projected_self = self.project();
        loop {
            if let Some(current) = projected_self.current.as_mut() {
                if let Some(item) = current.next() {
                    return task::Poll::Ready(Some(Ok(item)));
                }
                // Reset the iterator and poll for the next page.
                *projected_self.current = None;
            }

            match projected_self.stream.as_mut().poll_next(cx) {
                task::Poll::Ready(page) => match page {
                    Some(Ok(page)) => match page.into_items().poll_unpin(cx) {
                        task::Poll::Ready(Ok(iter)) => {
                            *projected_self.current = Some(iter);
                            continue;
                        }
                        task::Poll::Ready(Err(err)) => return task::Poll::Ready(Some(Err(err))),
                        task::Poll::Pending => return task::Poll::Pending,
                    },
                    Some(Err(err)) => return task::Poll::Ready(Some(Err(err))),
                    None => return task::Poll::Ready(None),
                },
                task::Poll::Pending => return task::Poll::Pending,
            }
        }
    }
}

impl<P: Page> fmt::Debug for ItemIterator<P> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.debug_struct("ItemIterator").finish_non_exhaustive()
    }
}

/// Iterates over a collection pages of items from a service.
///
/// # Examples
///
/// Some clients may return a `PageIterator` if there are no items to iterate or multiple items to iterate.
/// The following example shows how you can also get a `PageIterator` from a [`Pager`] to iterate over pages instead of items.
/// The pattern for iterating pages is otherwise the same:
///
/// ```no_run
/// # use azure_core::{credentials::TokenCredential, http::Transport};
/// # use azure_security_keyvault_secrets::{ResourceExt, SecretClient, SecretClientOptions};
/// # use futures::TryStreamExt;
/// # async fn example() -> Result<(), Box<dyn std::error::Error>> {
/// # let credential: std::sync::Arc<dyn TokenCredential> = unimplemented!();
/// let client = SecretClient::new(
///     "https://my-vault.vault.azure.net",
///     credential.clone(),
///     None,
/// )?;
///
/// // Iterate each page of secrets using a PageIterator.
/// let mut pager = client.list_secret_properties(None)?.into_pages();
/// while let Some(page) = pager.try_next().await? {
///     let page = page.into_body()?;
///     for secret in page.value {
///         println!("{}", secret.resource_id()?.name);
///     }
/// }
/// # Ok(()) }
/// ```
#[pin_project::pin_project]
pub struct PageIterator<P> {
    #[pin]
    stream: Pin<BoxedStream<P>>,
    continuation_token: Arc<Mutex<Option<String>>>,
}

impl<P> PageIterator<P> {
    /// Creates a [`PageIterator<P>`] from a callback that will be called repeatedly to request each page.
    ///
    /// This method expect a callback that accepts a single [`PagerState<C>`] parameter, and returns a [`PagerResult<T, C>`] value asynchronously.
    /// The `C` type parameter is the type of the next link/continuation token. It may be any [`Send`]able type.
    /// The result will be an asynchronous stream of [`Result<T>`](typespec::Result<T>) values.
    ///
    /// The first time your callback is called, it will be called with [`PagerState::Initial`], indicating no next link/continuation token is present.
    ///
    /// Your callback must return one of:
    /// * `Ok(result)` - The request succeeded, and the provided [`PagerResult`] indicates the value to return and if there are more pages.
    /// * `Err(..)` - The request failed. The error will be yielded to the stream, the stream will end, and the callback will not be called again.
    ///
    /// ## Examples
    ///
    /// To page results using a next link:
    ///
    /// ```rust,no_run
    /// # use azure_core::{Result, http::{RawResponse, Context, pager::{PageIterator, PagerResult, PagerState}, Pipeline, Request, Response, Method, Url}, json};
    /// # let api_version = "2025-06-04".to_string();
    /// # let pipeline: Pipeline = panic!("Not a runnable example");
    /// #[derive(serde::Deserialize)]
    /// struct ListItemsResult {
    ///     items: Vec<String>,
    ///     next_link: Option<String>,
    /// }
    /// let url = "https://example.com/my_paginated_api".parse().unwrap();
    /// let mut base_req = Request::new(url, Method::Get);
    /// let pager = PageIterator::from_callback(move |next_link: PagerState<Url>| {
    ///     // The callback must be 'static, so you have to clone and move any values you want to use.
    ///     let pipeline = pipeline.clone();
    ///     let api_version = api_version.clone();
    ///     let mut req = base_req.clone();
    ///     async move {
    ///         if let PagerState::More(next_link) = next_link {
    ///             // Ensure the api-version from the client is appended.
    ///             let qp = next_link
    ///                 .query_pairs()
    ///                 .filter(|(name, _)| name.ne("api-version"));
    ///             req
    ///                 .url_mut()
    ///                 .query_pairs_mut()
    ///                 .clear()
    ///                 .extend_pairs(qp)
    ///                 .append_pair("api-version", &api_version);
    ///         }
    ///         let resp = pipeline
    ///           .send(&Context::new(), &mut req, None)
    ///           .await?;
    ///         let (status, headers, body) = resp.deconstruct();
    ///         let result: ListItemsResult = json::from_json(&body)?;
    ///         let resp: Response<ListItemsResult> = RawResponse::from_bytes(status, headers, body).into();
    ///         Ok(match result.next_link {
    ///             Some(next_link) => PagerResult::More {
    ///                 response: resp,
    ///                 continuation: next_link.parse()?,
    ///             },
    ///             None => PagerResult::Done { response: resp }
    ///         })
    ///     }
    /// });
    /// ```
    ///
    /// To page results using headers:
    ///
    /// ```rust,no_run
    /// # use azure_core::{Result, http::{Context, pager::{PageIterator, PagerResult, PagerState}, Pipeline, Request, Response, Method, headers::HeaderName}};
    /// # let pipeline: Pipeline = panic!("Not a runnable example");
    /// #[derive(serde::Deserialize)]
    /// struct ListItemsResult {
    ///     items: Vec<String>,
    /// }
    /// let url = "https://example.com/my_paginated_api".parse().unwrap();
    /// let mut base_req = Request::new(url, Method::Get);
    /// let pager = PageIterator::from_callback(move |continuation| {
    ///     // The callback must be 'static, so you have to clone and move any values you want to use.
    ///     let pipeline = pipeline.clone();
    ///     let mut req = base_req.clone();
    ///     async move {
    ///         if let PagerState::More(continuation) = continuation {
    ///             req.insert_header("x-ms-continuation", continuation);
    ///         }
    ///         let resp: Response<ListItemsResult> = pipeline
    ///           .send(&Context::new(), &mut req, None)
    ///           .await?
    ///           .into();
    ///         Ok(PagerResult::from_response_header(resp, &HeaderName::from_static("x-ms-continuation")))
    ///     }
    /// });
    /// ```
    pub fn from_callback<
        // This is a bit gnarly, but the only thing that differs between the WASM/non-WASM configs is the presence of Send bounds.
        #[cfg(not(target_arch = "wasm32"))] C: AsRef<str> + FromStr + Send + 'static,
        #[cfg(not(target_arch = "wasm32"))] F: Fn(PagerState<C>) -> Fut + Send + 'static,
        #[cfg(not(target_arch = "wasm32"))] Fut: Future<Output = crate::Result<PagerResult<P, C>>> + Send + 'static,
        #[cfg(target_arch = "wasm32")] C: AsRef<str> + FromStr + 'static,
        #[cfg(target_arch = "wasm32")] F: Fn(PagerState<C>) -> Fut + 'static,
        #[cfg(target_arch = "wasm32")] Fut: Future<Output = crate::Result<PagerResult<P, C>>> + 'static,
    >(
        make_request: F,
    ) -> Self
    where
        <C as FromStr>::Err: fmt::Debug,
    {
        let continuation_token = Arc::new(Mutex::new(None::<String>));

        let get_clone = continuation_token.clone();
        let set_clone = continuation_token.clone();
        let stream = iter_from_callback(
            make_request,
            move || {
                if let Ok(token_guard) = get_clone.lock() {
                    return token_guard
                        .clone()
                        .map(|n| n.parse().expect("valid continuation_token"));
                }

                None
            },
            move |next_token| {
                if let Ok(mut token_guard) = set_clone.lock() {
                    *token_guard = next_token.map(Into::into);
                }
            },
        );

        Self {
            stream: Box::pin(stream),
            continuation_token,
        }
    }

    /// Creates a [`PageIterator<P>`] from a raw stream of [`Result<P>`](typespec::Result<P>) values.
    ///
    /// This constructor is used when you are implementing a completely custom stream and want to use it as a pager.
    pub fn from_stream<
        // This is a bit gnarly, but the only thing that differs between the WASM/non-WASM configs is the presence of Send bounds.
        #[cfg(not(target_arch = "wasm32"))] S: Stream<Item = crate::Result<P>> + Send + 'static,
        #[cfg(target_arch = "wasm32")] S: Stream<Item = crate::Result<P>> + 'static,
    >(
        stream: S,
    ) -> Self {
        Self {
            stream: Box::pin(stream),
            continuation_token: Default::default(),
        }
    }

    /// Advance the `PageIterator` to the page referenced by `continuation_token`.
    ///
    /// You should call this before iterating the [`Stream`] or results may be unpredictable.
    ///
    /// # Examples
    ///
    /// Using a result of a call to [`PageIterator::continuation_token`] in another process, you can create a new `PageIterator`
    /// that, when first iterated, will get the next page of results.
    ///
    /// ``` no_run
    /// use azure_identity::DeveloperToolsCredential;
    /// use azure_security_keyvault_secrets::SecretClient;
    /// use futures::stream::TryStreamExt as _;
    ///
    /// # #[tokio::main]
    /// # async fn main() -> azure_core::Result<()> {
    /// let client = SecretClient::new("https://my-vault.vault.azure.net", DeveloperToolsCredential::new(None)?, None)?;
    ///
    /// // Advance first pager to first page.
    /// let mut pager = client.list_secret_properties(None)?
    ///     .into_pages();
    ///
    /// let mut pager = client.list_secret_properties(None)?
    ///     .into_pages()
    ///     .with_continuation_token("continuation_token_from_another_pager".to_string());
    ///
    /// while let Some(secrets) = pager.try_next().await? {
    ///     let secrets = secrets.into_body()?;
    ///     for secret in secrets.value {
    ///         println!("{:?}", secret.id);
    ///     }
    /// }
    /// # Ok(())
    /// # }
    /// ```
    pub fn with_continuation_token(self, continuation_token: String) -> Self {
        if let Ok(mut token_guard) = self.continuation_token.lock() {
            *token_guard = Some(continuation_token);
        }

        self
    }

    /// Gets the continuation token for the current page.
    ///
    /// Pass this to [`PageIterator::with_continuation_token`] to create a `PageIterator` that, when first iterated,
    /// will return the next page. You can use this to page results across separate processes.
    pub fn continuation_token(&self) -> Option<String> {
        if let Ok(token) = self.continuation_token.lock() {
            return token.clone();
        }

        None
    }
}

impl<P> futures::Stream for PageIterator<P> {
    type Item = crate::Result<P>;

    fn poll_next(
        self: Pin<&mut Self>,
        cx: &mut std::task::Context<'_>,
    ) -> std::task::Poll<Option<Self::Item>> {
        self.project().stream.poll_next(cx)
    }
}

impl<P> fmt::Debug for PageIterator<P> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.debug_struct("PageIterator").finish_non_exhaustive()
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
enum State<T> {
    Init,
    More(T),
    Done,
}

fn iter_from_callback<
    P,
    // This is a bit gnarly, but the only thing that differs between the WASM/non-WASM configs is the presence of Send bounds.
    #[cfg(not(target_arch = "wasm32"))] C: AsRef<str> + Send + 'static,
    #[cfg(not(target_arch = "wasm32"))] F: Fn(PagerState<C>) -> Fut + Send + 'static,
    #[cfg(not(target_arch = "wasm32"))] Fut: Future<Output = crate::Result<PagerResult<P, C>>> + Send + 'static,
    #[cfg(not(target_arch = "wasm32"))] G: Fn() -> Option<C> + Send + 'static,
    #[cfg(not(target_arch = "wasm32"))] S: Fn(Option<&str>) + Send + 'static,
    #[cfg(target_arch = "wasm32")] C: AsRef<str> + 'static,
    #[cfg(target_arch = "wasm32")] F: Fn(PagerState<C>) -> Fut + 'static,
    #[cfg(target_arch = "wasm32")] Fut: Future<Output = crate::Result<PagerResult<P, C>>> + 'static,
    #[cfg(target_arch = "wasm32")] G: Fn() -> Option<C> + 'static,
    #[cfg(target_arch = "wasm32")] S: Fn(Option<&str>) + 'static,
>(
    make_request: F,
    get_next: G,
    set_next: S,
) -> impl Stream<Item = crate::Result<P>> + 'static {
    unfold(
        // We flow the `make_request` callback, 'get_next', and `set_next` through the state value so that we can avoid cloning.
        (State::Init, make_request, get_next, set_next),
        |(mut state, make_request, get_next, set_next)| async move {
            if let Some(next_token) = get_next() {
                state = State::More(next_token);
            }
            let result = match state {
                State::Init => make_request(PagerState::Initial).await,
                State::More(n) => make_request(PagerState::More(n)).await,
                State::Done => {
                    set_next(None);
                    return None;
                }
            };
            let (item, next_state) = match result {
                Err(e) => return Some((Err(e), (State::Done, make_request, get_next, set_next))),
                Ok(PagerResult::More {
                    response,
                    continuation: next_token,
                }) => {
                    set_next(Some(next_token.as_ref()));
                    (Ok(response), State::More(next_token))
                }
                Ok(PagerResult::Done { response }) => {
                    set_next(None);
                    (Ok(response), State::Done)
                }
            };

            // Flow 'make_request', 'get_next', and 'set_next' through to avoid cloning
            Some((item, (next_state, make_request, get_next, set_next)))
        },
    )
}

#[cfg(test)]
mod tests {
    use crate::http::{
        headers::{HeaderName, HeaderValue},
        pager::{PageIterator, Pager, PagerResult, PagerState},
        RawResponse, Response, StatusCode,
    };
    use async_trait::async_trait;
    use futures::{StreamExt as _, TryStreamExt as _};
    use serde::Deserialize;
    use std::collections::HashMap;

    #[derive(Deserialize, Debug, PartialEq, Eq)]
    struct Page {
        pub items: Vec<i32>,
        pub page: Option<i32>,
    }

    #[cfg_attr(not(target_arch = "wasm32"), async_trait)]
    #[cfg_attr(target_arch = "wasm32", async_trait(?Send))]
    impl super::Page for Page {
        type Item = i32;
        type IntoIter = <Vec<i32> as IntoIterator>::IntoIter;

        async fn into_items(self) -> crate::Result<Self::IntoIter> {
            Ok(self.items.into_iter())
        }
    }

    #[tokio::test]
    async fn callback_item_pagination() {
        let pager: Pager<Page> = Pager::from_callback(|continuation| async move {
            match continuation {
                PagerState::Initial => Ok(PagerResult::More {
                    response: RawResponse::from_bytes(
                        StatusCode::Ok,
                        HashMap::from([(
                            HeaderName::from_static("x-test-header"),
                            HeaderValue::from_static("page-1"),
                        )])
                        .into(),
                        r#"{"items":[1],"page":1}"#,
                    )
                    .into(),
                    continuation: "1",
                }),
                PagerState::More("1") => Ok(PagerResult::More {
                    response: RawResponse::from_bytes(
                        StatusCode::Ok,
                        HashMap::from([(
                            HeaderName::from_static("x-test-header"),
                            HeaderValue::from_static("page-2"),
                        )])
                        .into(),
                        r#"{"items":[2],"page":2}"#,
                    )
                    .into(),
                    continuation: "2",
                }),
                PagerState::More("2") => Ok(PagerResult::Done {
                    response: RawResponse::from_bytes(
                        StatusCode::Ok,
                        HashMap::from([(
                            HeaderName::from_static("x-test-header"),
                            HeaderValue::from_static("page-3"),
                        )])
                        .into(),
                        r#"{"items":[3],"page":3}"#,
                    )
                    .into(),
                }),
                _ => {
                    panic!("Unexpected continuation value")
                }
            }
        });
        let items: Vec<i32> = pager.try_collect().await.unwrap();
        assert_eq!(vec![1, 2, 3], items.as_slice())
    }

    #[tokio::test]
    async fn callback_item_pagination_error() {
        let pager: Pager<Page> = Pager::from_callback(|continuation| async move {
            match continuation {
                PagerState::Initial => Ok(PagerResult::More {
                    response: RawResponse::from_bytes(
                        StatusCode::Ok,
                        HashMap::from([(
                            HeaderName::from_static("x-test-header"),
                            HeaderValue::from_static("page-1"),
                        )])
                        .into(),
                        r#"{"items":[1],"page":1}"#,
                    )
                    .into(),
                    continuation: "1",
                }),
                PagerState::More("1") => Err(typespec::Error::with_message(
                    typespec::error::ErrorKind::Other,
                    "yon request didst fail",
                )),
                _ => {
                    panic!("Unexpected continuation value")
                }
            }
        });
        let pages: Vec<Result<(String, Page), typespec::Error>> = pager
            .into_pages()
            .then(|r| async move {
                let r = r?;
                let header = r
                    .headers()
                    .get_optional_string(&HeaderName::from_static("x-test-header"))
                    .unwrap();
                let body = r.into_body()?;
                Ok((header, body))
            })
            .collect()
            .await;
        assert_eq!(2, pages.len());
        assert_eq!(
            &(
                "page-1".to_string(),
                Page {
                    items: vec![1],
                    page: Some(1)
                }
            ),
            pages[0].as_ref().unwrap()
        );

        let err = pages[1].as_ref().unwrap_err();
        assert_eq!(&typespec::error::ErrorKind::Other, err.kind());
        assert_eq!("yon request didst fail", format!("{}", err));
    }

    #[tokio::test]
    async fn page_iterator_with_continuation_token() {
        let make_callback = || {
            |continuation: PagerState<String>| async move {
                match continuation.as_deref() {
                    PagerState::Initial => Ok(PagerResult::More {
                        response: RawResponse::from_bytes(
                            StatusCode::Ok,
                            Default::default(),
                            r#"{"items":[1],"page":1}"#,
                        )
                        .into(),
                        continuation: "next-token-1".to_string(),
                    }),
                    PagerState::More("next-token-1") => Ok(PagerResult::More {
                        response: RawResponse::from_bytes(
                            StatusCode::Ok,
                            HashMap::from([(
                                HeaderName::from_static("x-test-header"),
                                HeaderValue::from_static("page-2"),
                            )])
                            .into(),
                            r#"{"items":[2],"page":2}"#,
                        )
                        .into(),
                        continuation: "next-token-2".to_string(),
                    }),
                    PagerState::More("next-token-2") => Ok(PagerResult::Done {
                        response: RawResponse::from_bytes(
                            StatusCode::Ok,
                            HashMap::from([(
                                HeaderName::from_static("x-test-header"),
                                HeaderValue::from_static("page-3"),
                            )])
                            .into(),
                            r#"{"items":[3]}"#,
                        )
                        .into(),
                    }),
                    _ => {
                        panic!("Unexpected continuation value: {:?}", continuation)
                    }
                }
            }
        };

        // Create the first PageIterator.
        let mut first_pager: PageIterator<Response<Page>> =
            PageIterator::from_callback(make_callback());

        // Should start with no continuation_token.
        assert_eq!(first_pager.continuation_token(), None);

        // Advance to the first page.
        let first_page = first_pager
            .next()
            .await
            .expect("expected first page")
            .expect("expected successful first page")
            .into_body()
            .expect("expected page");
        assert_eq!(first_page.page, Some(1));
        assert_eq!(first_page.items, vec![1]);

        // continuation_token should point to second page.
        let continuation_token = first_pager
            .continuation_token()
            .expect("expected continuation_token from first page");
        assert_eq!(continuation_token, "next-token-1");

        // Create the second PageIterator.
        let mut second_pager: PageIterator<Response<Page>> =
            PageIterator::from_callback(make_callback())
                .with_continuation_token(continuation_token);

        // Should start with link to second page.
        assert_eq!(
            second_pager.continuation_token(),
            Some("next-token-1".into())
        );

        // Advance to second page.
        let second_page = second_pager
            .next()
            .await
            .expect("expected second page")
            .expect("expected successful second page")
            .into_body()
            .expect("expected page");
        assert_eq!(second_page.page, Some(2));
        assert_eq!(second_page.items, vec![2]);
        assert_eq!(
            second_pager.continuation_token(),
            Some("next-token-2".into())
        );

        // Advance to last page.
        let last_page = second_pager
            .next()
            .await
            .expect("expected last page")
            .expect("expected successful last page")
            .into_body()
            .expect("expected page");
        assert_eq!(last_page.page, None);
        assert_eq!(last_page.items, vec![3]);
        assert_eq!(second_pager.continuation_token(), None);
    }
}

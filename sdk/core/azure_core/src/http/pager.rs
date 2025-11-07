// Copyright (c) Microsoft Corporation. All rights reserved.
// Licensed under the MIT License.

//! Types and methods for pageable responses.

use crate::{
    error::ErrorKind,
    http::{
        headers::HeaderName, policies::create_public_api_span, response::Response, Context,
        DeserializeWith, Format, JsonFormat,
    },
    tracing::{Span, SpanStatus},
};
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
        let page: P = self.into_model()?;
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
///     let page = page.into_model()?;
///     for secret in page.value {
///         println!("{}", secret.resource_id()?.name);
///     }
/// }
/// # Ok(()) }
/// ```
pub type Pager<P, F = JsonFormat> = ItemIterator<Response<P, F>>;

/// Options for configuring the behavior of a [`Pager`].
#[derive(Clone, Debug, Default)]
pub struct PagerOptions<'a> {
    /// Context for HTTP requests made by the [`Pager`].
    pub context: Context<'a>,
}

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
///     let page = page.into_model()?;
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
    continuation_token: Option<String>,
    next_token: Arc<Mutex<Option<String>>>,
    current: Option<P::IntoIter>,
}

impl<P: Page> ItemIterator<P> {
    /// Creates a [`ItemIterator<P>`] from a callback that will be called repeatedly to request each page.
    ///
    /// This method expect a callback that accepts a single [`PagerState<C>`] parameter, and returns a [`PagerResult<T, C>`] value asynchronously.
    /// The `C` type parameter is the type of the next link/continuation token. It may be any [`Send`]able type.
    /// The result will be an asynchronous stream of [`Result<T>`](crate::Result<T>) values.
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
    /// let pager = ItemIterator::from_callback(move |next_link: PagerState<Url>, ctx: Context| {
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
    ///           .send(&ctx, &mut req, None)
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
    /// }, None);
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
    /// let pager = ItemIterator::from_callback(move |continuation, ctx| {
    ///     // The callback must be 'static, so you have to clone and move any values you want to use.
    ///     let pipeline = pipeline.clone();
    ///     let mut req = base_req.clone();
    ///     async move {
    ///         if let PagerState::More(continuation) = continuation {
    ///             req.insert_header("x-ms-continuation", continuation);
    ///         }
    ///         let resp: Response<ListItemsResult> = pipeline
    ///           .send(&ctx, &mut req, None)
    ///           .await?
    ///           .into();
    ///         Ok(PagerResult::from_response_header(resp, &HeaderName::from_static("x-next-continuation")))
    ///     }
    /// }, None);
    /// ```
    pub fn from_callback<
        // This is a bit gnarly, but the only thing that differs between the WASM/non-WASM configs is the presence of Send bounds.
        #[cfg(not(target_arch = "wasm32"))] C: AsRef<str> + FromStr + Send + 'static,
        #[cfg(not(target_arch = "wasm32"))] F: Fn(PagerState<C>, Context<'static>) -> Fut + Send + 'static,
        #[cfg(not(target_arch = "wasm32"))] Fut: Future<Output = crate::Result<PagerResult<P, C>>> + Send + 'static,
        #[cfg(target_arch = "wasm32")] C: AsRef<str> + FromStr + 'static,
        #[cfg(target_arch = "wasm32")] F: Fn(PagerState<C>, Context<'static>) -> Fut + 'static,
        #[cfg(target_arch = "wasm32")] Fut: Future<Output = crate::Result<PagerResult<P, C>>> + 'static,
    >(
        make_request: F,
        options: Option<PagerOptions<'static>>,
    ) -> Self
    where
        <C as FromStr>::Err: std::error::Error,
    {
        let options = options.unwrap_or_default();
        let next_token = Arc::new(Mutex::new(None::<String>));
        let stream = iter_from_callback(make_request, options.context.clone(), next_token.clone());

        Self {
            stream: Box::pin(stream),
            continuation_token: None,
            next_token,
            current: None,
        }
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
            continuation_token: None,
            next_token: Default::default(),
            current: None,
        }
    }

    /// Gets a [`PageIterator<P>`] to iterate over a collection of pages from a service.
    ///
    /// You can use this to asynchronously iterate pages returned by a collection request to a service.
    /// This allows you to get the individual pages' [`Response<P>`], from which you can iterate items in each page
    /// or deserialize the raw response as appropriate.
    ///
    /// The returned `PageIterator` resumes from the current page until _after_ all items are processed.
    /// It does not continue on the next page until you call `next()` after the last item in the current page
    /// because of how iterators are implemented. This may yield duplicates but will reduce the likelihood of skipping items instead.
    pub fn into_pages(self) -> PageIterator<P> {
        // Attempt to start paging from the current page so that we don't skip items,
        // assuming the service collection hasn't changed (most services don't create ephemeral snapshots).
        if let Ok(mut token) = self.next_token.lock() {
            *token = self.continuation_token;
        }

        PageIterator {
            stream: self.stream,
            continuation_token: self.next_token,
        }
    }

    /// Start the `ItemIterator` at the page referenced by `continuation_token`.
    ///
    /// You should call this before iterating the [`Stream`] or results may be unpredictable.
    /// Iteration of items will start from the beginning on the current page until _after_ all items are iterated.
    /// It does not continue on the next page until you call `next()` after the last item in the current page
    /// because of how iterators are implemented. This may yield duplicates but will reduce the likelihood of skipping items instead.
    ///
    /// # Examples
    ///
    /// Using a result of a call to [`ItemIterator::continuation_token`] in another process, you can create a new `ItemIterator`
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
    /// // Start the first pager at the first page.
    /// let mut pager = client.list_secret_properties(None)?;
    ///
    /// // Continue the second pager from where the first pager left off,
    /// // which is the first page in this example.
    /// let mut pager = client.list_secret_properties(None)?
    ///     .with_continuation_token(Some("continuation_token_from_another_pager".into()));
    ///
    /// while let Some(secret) = pager.try_next().await? {
    ///     println!("{:?}", secret.id);
    /// }
    /// # Ok(())
    /// # }
    /// ```
    pub fn with_continuation_token(self, continuation_token: Option<String>) -> Self {
        // Set the next_token because that's what is passed to iter_from_callback to get the initial page.
        if let Ok(mut token) = self.next_token.lock() {
            *token = continuation_token;
        }
        self
    }

    /// Gets the continuation token for the current page.
    ///
    /// Pass this to [`ItemIterator::with_continuation_token`] to create a `ItemIterator` that, when first iterated,
    /// will return the current page until _after_ all items are iterated.
    /// It does not continue on the next page until you call `next()` after the last item in the current page
    /// because of how iterators are implemented. This may yield duplicates but will reduce the likelihood of skipping items instead.
    pub fn continuation_token(&self) -> Option<String> {
        // Get the continuation_token because that will be used to start over with the current page.
        self.continuation_token.clone()
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

            // Set the current_token to the next page only after iterating through all items.
            if let Ok(token) = projected_self.next_token.lock() {
                tracing::trace!(
                    "updating continuation_token from {:?} to {:?}",
                    projected_self.continuation_token,
                    token
                );
                *projected_self.continuation_token = token.clone();
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
///     let page = page.into_model()?;
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
    /// The result will be an asynchronous stream of [`Result<T>`](crate::Result<T>) values.
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
    /// let pager = PageIterator::from_callback(move |next_link: PagerState<Url>, ctx| {
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
    ///           .send(&ctx, &mut req, None)
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
    /// }, None);
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
    /// let pager = PageIterator::from_callback(move |continuation, ctx| {
    ///     // The callback must be 'static, so you have to clone and move any values you want to use.
    ///     let pipeline = pipeline.clone();
    ///     let mut req = base_req.clone();
    ///     async move {
    ///         if let PagerState::More(continuation) = continuation {
    ///             req.insert_header("x-ms-continuation", continuation);
    ///         }
    ///         let resp: Response<ListItemsResult> = pipeline
    ///           .send(&ctx, &mut req, None)
    ///           .await?
    ///           .into();
    ///         Ok(PagerResult::from_response_header(resp, &HeaderName::from_static("x-ms-continuation")))
    ///     }
    /// }, None);
    /// ```
    pub fn from_callback<
        // This is a bit gnarly, but the only thing that differs between the WASM/non-WASM configs is the presence of Send bounds.
        #[cfg(not(target_arch = "wasm32"))] C: AsRef<str> + FromStr + Send + 'static,
        #[cfg(not(target_arch = "wasm32"))] F: Fn(PagerState<C>, Context<'static>) -> Fut + Send + 'static,
        #[cfg(not(target_arch = "wasm32"))] Fut: Future<Output = crate::Result<PagerResult<P, C>>> + Send + 'static,
        #[cfg(target_arch = "wasm32")] C: AsRef<str> + FromStr + 'static,
        #[cfg(target_arch = "wasm32")] F: Fn(PagerState<C>, Context<'static>) -> Fut + 'static,
        #[cfg(target_arch = "wasm32")] Fut: Future<Output = crate::Result<PagerResult<P, C>>> + 'static,
    >(
        make_request: F,
        options: Option<PagerOptions<'static>>,
    ) -> Self
    where
        <C as FromStr>::Err: std::error::Error,
    {
        let options = options.unwrap_or_default();
        let continuation_token = Arc::new(Mutex::new(None::<String>));
        let stream = iter_from_callback(
            make_request,
            options.context.clone(),
            continuation_token.clone(),
        );

        Self {
            stream: Box::pin(stream),
            continuation_token,
        }
    }

    /// Creates a [`PageIterator<P>`] from a raw stream of [`Result<P>`](crate::Result<P>) values.
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

    /// Start the `PageIterator` at the page referenced by `continuation_token`.
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
    /// // Start the first pager at the first page.
    /// let mut pager = client.list_secret_properties(None)?
    ///     .into_pages();
    ///
    /// // Continue the second pager from where the first pager left off,
    /// // which is the first page in this example.
    /// let mut pager = client.list_secret_properties(None)?
    ///     .into_pages()
    ///     .with_continuation_token("continuation_token_from_another_pager".to_string());
    ///
    /// while let Some(secrets) = pager.try_next().await? {
    ///     let secrets = secrets.into_model()?;
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

#[derive(Debug, Clone, Eq)]
enum State<C> {
    Init,
    More(C),
    Done,
}

impl<C> PartialEq for State<C> {
    fn eq(&self, other: &Self) -> bool {
        // Only needs to compare if both states are Init or Done; internally, we don't care about any other states.
        matches!(
            (self, other),
            (State::Init, State::Init) | (State::Done, State::Done)
        )
    }
}

#[derive(Debug)]
struct StreamState<'a, C, F> {
    state: State<C>,
    make_request: F,
    continuation_token: Arc<Mutex<Option<String>>>,
    ctx: Context<'a>,
    added_span: bool,
}

fn iter_from_callback<
    P,
    // This is a bit gnarly, but the only thing that differs between the WASM/non-WASM configs is the presence of Send bounds.
    #[cfg(not(target_arch = "wasm32"))] C: AsRef<str> + FromStr + Send + 'static,
    #[cfg(not(target_arch = "wasm32"))] F: Fn(PagerState<C>, Context<'static>) -> Fut + Send + 'static,
    #[cfg(not(target_arch = "wasm32"))] Fut: Future<Output = crate::Result<PagerResult<P, C>>> + Send + 'static,
    #[cfg(target_arch = "wasm32")] C: AsRef<str> + FromStr + 'static,
    #[cfg(target_arch = "wasm32")] F: Fn(PagerState<C>, Context<'static>) -> Fut + 'static,
    #[cfg(target_arch = "wasm32")] Fut: Future<Output = crate::Result<PagerResult<P, C>>> + 'static,
>(
    make_request: F,
    ctx: Context<'static>,
    continuation_token: Arc<Mutex<Option<String>>>,
) -> impl Stream<Item = crate::Result<P>> + 'static
where
    <C as FromStr>::Err: std::error::Error,
{
    unfold(
        StreamState {
            state: State::Init,
            make_request,
            continuation_token,
            ctx,
            added_span: false,
        },
        |mut stream_state| async move {
            // Get the `continuation_token` to pick up where we left off, or None for the initial page,
            // but don't override the terminal `State::Done`.

            if stream_state.state != State::Done {
                let result = match stream_state.continuation_token.lock() {
                    Ok(next_token) => match next_token.as_deref() {
                        Some(n) => match n.parse() {
                            Ok(s) => Ok(State::More(s)),
                            Err(err) => Err(crate::Error::with_message_fn(
                                ErrorKind::DataConversion,
                                || format!("invalid continuation token: {err}"),
                            )),
                        },
                        // Restart the pager if `next_token` is None indicating we resumed from before or within the first page.
                        None => Ok(State::Init),
                    },
                    Err(err) => Err(crate::Error::with_message_fn(ErrorKind::Other, || {
                        format!("continuation token lock: {err}")
                    })),
                };

                match result {
                    Ok(state) => stream_state.state = state,
                    Err(err) => {
                        stream_state.state = State::Done;
                        return Some((Err(err), stream_state));
                    }
                }
            }
            let result = match stream_state.state {
                State::Init => {
                    tracing::debug!("initial page request");
                    // At the very start of polling, create a span for the entire request, and attach it to the context
                    let span = create_public_api_span(&stream_state.ctx, None, None);
                    if let Some(ref s) = span {
                        stream_state.added_span = true;
                        stream_state.ctx = stream_state.ctx.with_value(s.clone());
                    }
                    (stream_state.make_request)(PagerState::Initial, stream_state.ctx.clone()).await
                }
                State::More(n) => {
                    tracing::debug!("subsequent page request to {:?}", AsRef::<str>::as_ref(&n));
                    (stream_state.make_request)(PagerState::More(n), stream_state.ctx.clone()).await
                }
                State::Done => {
                    tracing::debug!("done");
                    // Set the `continuation_token` to None now that we are done.
                    if let Ok(mut token) = stream_state.continuation_token.lock() {
                        *token = None;
                    }
                    return None;
                }
            };
            let (item, next_state) = match result {
                Err(e) => {
                    if stream_state.added_span {
                        if let Some(span) = stream_state.ctx.value::<Arc<dyn Span>>() {
                            // Mark the span as an error with an appropriate description.
                            span.set_status(SpanStatus::Error {
                                description: e.to_string(),
                            });
                            span.set_attribute("error.type", e.kind().to_string().into());
                            span.end();
                        }
                    }

                    stream_state.state = State::Done;
                    return Some((Err(e), stream_state));
                }
                Ok(PagerResult::More {
                    response,
                    continuation: next_token,
                }) => {
                    // Set the `continuation_token` to the next page.
                    if let Ok(mut token) = stream_state.continuation_token.lock() {
                        *token = Some(next_token.as_ref().into());
                    }
                    (Ok(response), State::More(next_token))
                }
                Ok(PagerResult::Done { response }) => {
                    // Set the `continuation_token` to None now that we are done.
                    if let Ok(mut token) = stream_state.continuation_token.lock() {
                        *token = None;
                    }
                    // When the result is done, finalize the span. Note that we only do that if we created the span in the first place,
                    // otherwise it is the responsibility of the caller to end their span.
                    if stream_state.added_span {
                        if let Some(span) = stream_state.ctx.value::<Arc<dyn Span>>() {
                            // P is unconstrained, so it's not possible to retrieve the status code for now.

                            span.end();
                        }
                    }
                    (Ok(response), State::Done)
                }
            };

            stream_state.state = next_state;
            Some((item, stream_state))
        },
    )
}

#[cfg(test)]
mod tests {
    use crate::{
        error::ErrorKind,
        http::{
            headers::{HeaderName, HeaderValue},
            pager::{PageIterator, Pager, PagerResult, PagerState},
            Context, RawResponse, Response, StatusCode,
        },
    };
    use async_trait::async_trait;
    use futures::{StreamExt as _, TryStreamExt as _};
    use serde::Deserialize;
    use std::{collections::HashMap, future::Future, pin::Pin};

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
        let pager: Pager<Page> = Pager::from_callback(
            |continuation: PagerState<String>, _ctx| async move {
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
                        continuation: "1".into(),
                    }),
                    PagerState::More(ref i) if i == "1" => Ok(PagerResult::More {
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
                        continuation: "2".into(),
                    }),
                    PagerState::More(ref i) if i == "2" => Ok(PagerResult::Done {
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
            },
            None,
        );
        let items: Vec<i32> = pager.try_collect().await.unwrap();
        assert_eq!(vec![1, 2, 3], items.as_slice())
    }

    #[tokio::test]
    async fn callback_item_pagination_error() {
        let pager: Pager<Page> = Pager::from_callback(
            |continuation: PagerState<String>, _ctx| async move {
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
                        continuation: "1".into(),
                    }),
                    PagerState::More(ref i) if i == "1" => Err(crate::Error::with_message(
                        crate::error::ErrorKind::Other,
                        "yon request didst fail",
                    )),
                    _ => {
                        panic!("Unexpected continuation value")
                    }
                }
            },
            None,
        );
        let pages: Vec<Result<(String, Page), crate::Error>> = pager
            .into_pages()
            .then(|r| async move {
                let r = r?;
                let header = r
                    .headers()
                    .get_optional_string(&HeaderName::from_static("x-test-header"))
                    .unwrap();
                let body = r.into_model()?;
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
        assert_eq!(&crate::error::ErrorKind::Other, err.kind());
        assert_eq!("yon request didst fail", format!("{}", err));
    }

    #[tokio::test]
    async fn page_iterator_with_continuation_token() {
        // Create the first PageIterator.
        let mut first_pager: PageIterator<Response<Page>> =
            PageIterator::from_callback(make_three_page_callback(), None);

        // Should start with no continuation_token.
        assert_eq!(first_pager.continuation_token(), None);

        // Advance to the first page.
        let first_page = first_pager
            .next()
            .await
            .expect("expected first page")
            .expect("expected successful first page")
            .into_model()
            .expect("expected page");
        assert_eq!(first_page.page, Some(1));
        assert_eq!(first_page.items, vec![1, 2, 3]);

        // continuation_token should point to second page.
        let continuation_token = first_pager
            .continuation_token()
            .expect("expected continuation_token from first page");
        assert_eq!(continuation_token, "next-token-1");

        // Create the second PageIterator.
        let mut second_pager: PageIterator<Response<Page>> =
            PageIterator::from_callback(make_three_page_callback(), None)
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
            .into_model()
            .expect("expected page");
        assert_eq!(second_page.page, Some(2));
        assert_eq!(second_page.items, vec![4, 5, 6]);
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
            .into_model()
            .expect("expected page");
        assert_eq!(last_page.page, None);
        assert_eq!(last_page.items, vec![7, 8, 9]);
        assert_eq!(second_pager.continuation_token(), None);
    }

    #[tokio::test]
    async fn page_iterator_from_item_iterator_after_first_page() {
        // Create an ItemIterator and consume all items from first page.
        let mut item_pager: Pager<Page> = Pager::from_callback(make_three_page_callback(), None);

        // Should start with no continuation_token.
        assert_eq!(item_pager.continuation_token(), None);

        // Consume all three items from the first page.
        let first_item = item_pager
            .next()
            .await
            .expect("expected first item")
            .expect("expected successful first item");
        assert_eq!(first_item, 1);

        let second_item = item_pager
            .next()
            .await
            .expect("expected second item")
            .expect("expected successful second item");
        assert_eq!(second_item, 2);

        let third_item = item_pager
            .next()
            .await
            .expect("expected third item")
            .expect("expected successful third item");
        assert_eq!(third_item, 3);

        // Convert to PageIterator after consuming first page.
        let mut page_pager = item_pager.into_pages();

        // Should start with None initially.
        assert_eq!(page_pager.continuation_token(), None);

        // Verify we start over with the first page again (ItemIterator.continuation_token() was None).
        let first_page = page_pager
            .next()
            .await
            .expect("expected first page")
            .expect("expected successful first page")
            .into_model()
            .expect("expected page");
        assert_eq!(first_page.page, Some(1));
        assert_eq!(first_page.items, vec![1, 2, 3]);

        // continuation_token should now point to second page.
        let continuation_token = page_pager
            .continuation_token()
            .expect("expected continuation_token from first page");
        assert_eq!(continuation_token, "next-token-1");
    }

    #[tokio::test]
    async fn page_iterator_from_item_iterator_second_page_first_item() {
        // Create an ItemIterator and consume items up to first item of second page.
        let mut item_pager: Pager<Page> = Pager::from_callback(make_three_page_callback(), None);

        // Should start with no continuation_token.
        assert_eq!(item_pager.continuation_token(), None);

        // Consume all three items from the first page.
        let first_item = item_pager
            .next()
            .await
            .expect("expected first item")
            .expect("expected successful first item");
        assert_eq!(first_item, 1);

        let second_item = item_pager
            .next()
            .await
            .expect("expected second item")
            .expect("expected successful second item");
        assert_eq!(second_item, 2);

        let third_item = item_pager
            .next()
            .await
            .expect("expected third item")
            .expect("expected successful third item");
        assert_eq!(third_item, 3);

        // Get first item from second page.
        let fourth_item = item_pager
            .next()
            .await
            .expect("expected fourth item")
            .expect("expected successful fourth item");
        assert_eq!(fourth_item, 4);

        // Convert to PageIterator after consuming first item of second page.
        let mut page_pager = item_pager.into_pages();

        // Should start with second page since that's where we were.
        assert_eq!(page_pager.continuation_token(), Some("next-token-1".into()));

        // Get second page - should be the second page.
        let second_page = page_pager
            .next()
            .await
            .expect("expected second page")
            .expect("expected successful second page")
            .into_model()
            .expect("expected page");
        assert_eq!(second_page.page, Some(2));
        assert_eq!(second_page.items, vec![4, 5, 6]);

        // continuation_token should now point to third page.
        let continuation_token = page_pager
            .continuation_token()
            .expect("expected continuation_token from second page");
        assert_eq!(continuation_token, "next-token-2");
    }

    #[tokio::test]
    async fn item_iterator_with_continuation_token() {
        // Create the first ItemIterator.
        let mut first_pager: Pager<Page> = Pager::from_callback(make_three_page_callback(), None);

        // Should start with no continuation_token.
        assert_eq!(first_pager.continuation_token(), None);

        // Get first item from first page.
        let first_item = first_pager
            .next()
            .await
            .expect("expected first item")
            .expect("expected successful first item");
        assert_eq!(first_item, 1);

        // Get second item from first page.
        let second_item = first_pager
            .next()
            .await
            .expect("expected second item")
            .expect("expected successful second item");
        assert_eq!(second_item, 2);

        // continuation_token should point to current page after processing some, but not all, items.
        let continuation_token = first_pager.continuation_token();
        assert_eq!(continuation_token, None);

        // Create the second ItemIterator with continuation token.
        let mut second_pager: Pager<Page> = Pager::from_callback(make_three_page_callback(), None)
            .with_continuation_token(continuation_token);

        // Should start with link to first page.
        assert_eq!(second_pager.continuation_token(), None);

        // When continuing with a continuation token, we should start over from the
        // beginning of the page, not where we left off in the item stream.
        // This means we should get the first item of the first page (1), not the
        // third item of the first page (3).
        let first_item_second_pager = second_pager
            .next()
            .await
            .expect("expected first item from second pager")
            .expect("expected successful first item from second pager");
        assert_eq!(first_item_second_pager, 1);

        // Get remaining items.
        let items: Vec<i32> = second_pager.try_collect().await.unwrap();
        assert_eq!(items.as_slice(), vec![2, 3, 4, 5, 6, 7, 8, 9]);
    }

    #[tokio::test]
    async fn item_iterator_continuation_second_page_second_item() {
        // Create the first ItemIterator.
        let mut first_pager: Pager<Page> = Pager::from_callback(make_three_page_callback(), None);

        // Should start with no continuation_token.
        assert_eq!(first_pager.continuation_token(), None);

        // Iterate to the second item of the second page.
        // First page: items 1, 2, 3
        let first_item = first_pager
            .next()
            .await
            .expect("expected first item")
            .expect("expected successful first item");
        assert_eq!(first_item, 1);

        let second_item = first_pager
            .next()
            .await
            .expect("expected second item")
            .expect("expected successful second item");
        assert_eq!(second_item, 2);

        let third_item = first_pager
            .next()
            .await
            .expect("expected third item")
            .expect("expected successful third item");
        assert_eq!(third_item, 3);

        // Second page: item 4 (first of second page)
        let fourth_item = first_pager
            .next()
            .await
            .expect("expected fourth item")
            .expect("expected successful fourth item");
        assert_eq!(fourth_item, 4);

        // Second page: item 5 (second of second page)
        let fifth_item = first_pager
            .next()
            .await
            .expect("expected fifth item")
            .expect("expected successful fifth item");
        assert_eq!(fifth_item, 5);

        // Get continuation token - should point to current page (second page).
        let continuation_token = first_pager.continuation_token();
        assert_eq!(continuation_token.as_deref(), Some("next-token-1"));

        // Create the second ItemIterator with continuation token.
        let mut second_pager: Pager<Page> = Pager::from_callback(make_three_page_callback(), None)
            .with_continuation_token(continuation_token);

        // When continuing with a continuation token, we should start over from the
        // beginning of the current page (second page), not where we left off.
        // This means we should get the first item of the second page (4).
        let first_item_second_pager = second_pager
            .next()
            .await
            .expect("expected first item from second pager")
            .expect("expected successful first item from second pager");
        assert_eq!(first_item_second_pager, 4);

        // Get remaining items.
        let items: Vec<i32> = second_pager.try_collect().await.unwrap();
        assert_eq!(items.as_slice(), vec![5, 6, 7, 8, 9]);
    }

    #[tokio::test]
    async fn item_iterator_continuation_after_first_page() {
        // Create the first ItemIterator.
        let mut first_pager: Pager<Page> = Pager::from_callback(make_three_page_callback(), None);

        // Should start with no continuation_token.
        assert_eq!(first_pager.continuation_token(), None);

        // Iterate past the third item of the first page (all items of first page).
        let first_item = first_pager
            .next()
            .await
            .expect("expected first item")
            .expect("expected successful first item");
        assert_eq!(first_item, 1);

        let second_item = first_pager
            .next()
            .await
            .expect("expected second item")
            .expect("expected successful second item");
        assert_eq!(second_item, 2);

        let third_item = first_pager
            .next()
            .await
            .expect("expected third item")
            .expect("expected successful third item");
        assert_eq!(third_item, 3);

        // Get continuation token after finishing the first page - should still point to current page (first page).
        let continuation_token = first_pager.continuation_token();
        assert_eq!(continuation_token, None);

        // Create the second ItemIterator with continuation token.
        let mut second_pager: Pager<Page> = Pager::from_callback(make_three_page_callback(), None)
            .with_continuation_token(continuation_token);

        // When continuing with a continuation token after finishing a page, we should
        // start from the beginning of the current page.
        // This means we should get the first item of the second page (4).
        let first_item_second_pager = second_pager
            .next()
            .await
            .expect("expected first item from first pager")
            .expect("expected successful first item from first pager");
        assert_eq!(first_item_second_pager, 1);

        // Get remaining items.
        let items: Vec<i32> = second_pager.try_collect().await.unwrap();
        assert_eq!(items.as_slice(), vec![2, 3, 4, 5, 6, 7, 8, 9]);
    }

    /// A continuation token type that always fails to parse, used to test FromStr constraint.
    #[derive(Debug, Clone, PartialEq, Eq)]
    struct ContinuationToken(String);

    impl AsRef<str> for ContinuationToken {
        fn as_ref(&self) -> &str {
            &self.0
        }
    }

    impl std::str::FromStr for ContinuationToken {
        type Err = std::io::Error;

        fn from_str(_s: &str) -> Result<Self, Self::Err> {
            // Always fail to parse
            Err(std::io::Error::new(
                std::io::ErrorKind::InvalidData,
                "ContinuationToken parsing always fails",
            ))
        }
    }

    #[tokio::test]
    async fn callback_item_pagination_from_str_error() {
        let mut pager: Pager<Page> = Pager::from_callback(
            |continuation: PagerState<ContinuationToken>, _ctx| async move {
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
                        // cspell:ignore unparseable
                        continuation: ContinuationToken("unparseable-token".to_string()),
                    }),
                    _ => {
                        panic!("Unexpected continuation value: {:?}", continuation)
                    }
                }
            },
            None,
        );

        // Get the first item from the first page.
        let first_item = pager.try_next().await.expect("expected first page");
        assert_eq!(first_item, Some(1));

        // Attempt to get the second page, which will attempt to parse the continuation token that should fail.
        assert!(
            matches!(pager.try_next().await, Err(err) if err.kind() == &ErrorKind::DataConversion)
        );
    }

    #[allow(clippy::type_complexity)]
    fn make_three_page_callback() -> impl Fn(
        PagerState<String>,
        Context<'_>,
    ) -> Pin<
        Box<dyn Future<Output = crate::Result<PagerResult<Response<Page>, String>>> + Send>,
    > + Send
           + 'static {
        |continuation: PagerState<String>, _ctx| {
            Box::pin(async move {
                match continuation.as_deref() {
                    PagerState::Initial => Ok(PagerResult::More {
                        response: RawResponse::from_bytes(
                            StatusCode::Ok,
                            Default::default(),
                            r#"{"items":[1,2,3],"page":1}"#,
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
                            r#"{"items":[4,5,6],"page":2}"#,
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
                            r#"{"items":[7,8,9]}"#,
                        )
                        .into(),
                    }),
                    _ => {
                        panic!("Unexpected continuation value: {:?}", continuation)
                    }
                }
            })
        }
    }
}

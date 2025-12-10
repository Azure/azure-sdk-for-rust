// Copyright (c) Microsoft Corporation. All rights reserved.
// Licensed under the MIT License.

//! Types and methods for pageable responses.

use crate::{
    conditional_send::ConditionalSend,
    http::{
        headers::HeaderName, policies::create_public_api_span, response::Response, Context,
        DeserializeWith, Format, JsonFormat, Url,
    },
    tracing::{Span, SpanStatus},
};
use async_trait::async_trait;
use futures::{stream::FusedStream, FutureExt, Stream};
use pin_project::pin_project;
use std::{fmt, future::Future, ops::Deref, pin::Pin, sync::Arc, task};

/// Represents the state of a [`Pager`] or [`PageIterator`].
#[derive(Debug, Default, PartialEq, Eq)]
pub enum PagerState<C: AsRef<str> = Url> {
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

impl<C: AsRef<str> + Clone> Clone for PagerState<C> {
    #[inline]
    fn clone(&self) -> Self {
        match self {
            PagerState::Initial => PagerState::Initial,
            PagerState::More(c) => PagerState::More(c.clone()),
        }
    }
}

/// The result of fetching a single page from a [`Pager`], whether there are more pages or paging is done.
pub enum PagerResult<P, C: AsRef<str> = Url> {
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
    /// Creates a [`PagerResult`] from the provided response, extracting the continuation value from the provided header.
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

impl<P, C: AsRef<str>> fmt::Debug for PagerResult<P, C> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::More { continuation, .. } => f
                .debug_struct("More")
                .field("continuation", &continuation.as_ref())
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
/// Specifically, this is a [`ItemIterator`] that yields [`Response`] items.
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
pub type Pager<P, F = JsonFormat, C = Url> = ItemIterator<Response<P, F>, C>;

/// A pinned boxed [`Future`] that can be stored and called dynamically.
///
/// Intended only for [`ItemIterator`] and [`PageIterator`].
#[cfg(not(target_arch = "wasm32"))]
pub type BoxedFuture<P, C> =
    Pin<Box<dyn Future<Output = crate::Result<PagerResult<P, C>>> + Send + 'static>>;

/// A pinned boxed [`Future`] that can be stored and called dynamically.
///
/// Intended only for [`ItemIterator`] and [`PageIterator`].
#[cfg(target_arch = "wasm32")]
pub type BoxedFuture<P, C> =
    Pin<Box<dyn Future<Output = crate::Result<PagerResult<P, C>>> + 'static>>;

type PagerFn<P, C> = Box<dyn Fn(PagerState<C>, PagerOptions<'static, C>) -> BoxedFuture<P, C>>;

/// Options for configuring the behavior of a [`Pager`].
#[derive(Clone)]
pub struct PagerOptions<'a, C: AsRef<str> = Url> {
    /// Context for HTTP requests made by the [`Pager`].
    pub context: Context<'a>,

    /// Optional continuation token or next link to resume paging.
    ///
    /// # Examples
    ///
    /// ``` no_run
    /// use azure_core::http::pager::PagerOptions;
    /// use azure_identity::DeveloperToolsCredential;
    /// use azure_security_keyvault_secrets::{
    ///     models::SecretClientListSecretPropertiesOptions,
    ///     SecretClient,
    /// };
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
    /// let options = SecretClientListSecretPropertiesOptions {
    ///     method_options: PagerOptions {
    ///         continuation_token: pager.into_continuation_token(),
    ///         ..Default::default()
    ///     },
    ///     ..Default::default()
    /// };
    /// let mut pager = client.list_secret_properties(Some(options))?;
    /// while let Some(secret) = pager.try_next().await? {
    ///     println!("{:?}", secret.id);
    /// }
    /// # Ok(())
    /// # }
    /// ```
    pub continuation_token: Option<C>,
}

impl<'a, C: AsRef<str>> fmt::Debug for PagerOptions<'a, C> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.debug_struct("PagerOptions")
            .field("context", &self.context)
            .field(
                "continuation_token",
                &self.continuation_token.as_ref().map(AsRef::as_ref),
            )
            .finish()
    }
}

impl<'a, C: AsRef<str>> Default for PagerOptions<'a, C> {
    fn default() -> Self {
        PagerOptions {
            context: Context::new(),
            continuation_token: None,
        }
    }
}

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
#[pin_project(project = ItemIteratorProjection, project_replace = ItemIteratorProjectionOwned)]
pub struct ItemIterator<P, C = Url>
where
    P: Page + ConditionalSend,
    C: AsRef<str> + ConditionalSend,
{
    #[pin]
    iter: PageIterator<P, C>,
    /// The continuation token or next link for the current page.
    ///
    /// Unlike the inner [`PageIterator::continuation_token`], this continuation token might be a page behind.
    /// To help avoid skipping items (barring underlying changes in the remote collection), we don't continue with the next page
    /// until after we've iterated all items on the current page.
    continuation_token: Option<C>,
    current: Option<P::IntoIter>,
}

impl<P, C> ItemIterator<P, C>
where
    P: Page + ConditionalSend,
    C: AsRef<str> + Clone + ConditionalSend + 'static,
{
    /// Creates a [`ItemIterator`] from a callback that will be called repeatedly to request each page.
    ///
    /// This method expect a callback that accepts a single [`PagerState`] parameter, and returns a [`PagerResult`] value asynchronously.
    /// The `C` type parameter is the type of the next link/continuation token. It may be any [`Send`]able type.
    /// The result will be an asynchronous stream of [`Result`](crate::Result) values.
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
    /// # use azure_core::{Result, http::{RawResponse, ItemIterator, pager::{Page, PagerOptions, PagerResult, PagerState}, Pipeline, Request, Response, Method, Url}, json};
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
    /// let pager = ItemIterator::new(move |next_link: PagerState<Url>, options: PagerOptions<'static, Url>| {
    ///     // The callback must be 'static, so you have to clone and move any values you want to use.
    ///     let pipeline = pipeline.clone();
    ///     let api_version = api_version.clone();
    ///     let mut req = base_req.clone();
    ///     Box::pin(async move {
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
    ///           .send(&options.context, &mut req, None)
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
    ///     })
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
    /// let pager = ItemIterator::new(move |continuation, options| {
    ///     // The callback must be 'static, so you have to clone and move any values you want to use.
    ///     let pipeline = pipeline.clone();
    ///     let mut req = base_req.clone();
    ///     Box::pin(async move {
    ///         if let PagerState::More(continuation) = continuation {
    ///             req.insert_header("x-ms-continuation", continuation);
    ///         }
    ///         let resp: Response<ListItemsResult> = pipeline
    ///           .send(&options.context, &mut req, None)
    ///           .await?
    ///           .into();
    ///         Ok(PagerResult::from_response_header(resp, &HeaderName::from_static("x-next-continuation")))
    ///     })
    /// }, None);
    /// ```
    pub fn new<
        F: Fn(PagerState<C>, PagerOptions<'static, C>) -> BoxedFuture<P, C>
            + ConditionalSend
            + 'static,
    >(
        make_request: F,
        options: Option<PagerOptions<'static, C>>,
    ) -> Self {
        let options = options.unwrap_or_default();

        // Start from the optional `PagerOptions::continuation_token`.
        let continuation_token = options.continuation_token.clone();

        Self {
            iter: PageIterator::new(make_request, Some(options)),
            continuation_token,
            current: None,
        }
    }

    /// Gets the continuation token to pass to [`PagerOptions`] to resume paging in another iterator.
    pub fn continuation_token(&self) -> Option<&C> {
        self.continuation_token.as_ref()
    }

    /// Gets the continuation token to pass to [`PagerOptions`] to resume paging in another iterator.
    ///
    /// # Examples
    ///
    /// This takes ownership of the iterator and can be useful when constructing a new iterator.
    ///
    /// ```no_run
    /// # use azure_core::http::pager::PagerOptions;
    /// # use azure_security_keyvault_secrets::{SecretClient, models::SecretClientListSecretPropertiesOptions};
    /// # use futures::stream::TryStreamExt;
    /// # #[tokio::main] async fn main() -> azure_core::Result<()> {
    /// # let client: SecretClient = unimplemented!();
    /// let pager1 = client.list_secret_properties(None)?;
    /// assert!(pager1.try_next().await?.is_some());
    /// let options = SecretClientListSecretPropertiesOptions {
    ///     method_options: PagerOptions {
    ///         continuation_token: pager1.into_continuation_token(),
    ///         ..Default::default()
    ///     },
    ///     ..Default::default()
    /// };
    /// let pager2 = client.list_secret_properties(Some(options))?;
    /// assert!(pager2.try_next().await?.is_some());
    /// # Ok(()) }
    /// ```
    pub fn into_continuation_token(self) -> Option<C> {
        self.continuation_token
    }

    /// Gets a [`PageIterator`] to iterate over pages instead of items.
    ///
    /// Resumes from the current page of items until after all items in the current page have been iterated
    /// to avoid skipping items in the current page.
    pub fn into_pages(self) -> PageIterator<P, C> {
        let mut iter = self.iter;

        // Start with the current page until after all items are iterated.
        iter.options.continuation_token = self.continuation_token;
        iter.state = iter
            .options
            .continuation_token
            .as_ref()
            .map_or_else(|| State::Init, |_| State::More);

        iter
    }
}

impl<P, C> Stream for ItemIterator<P, C>
where
    P: Page + ConditionalSend,
    C: AsRef<str> + Clone + ConditionalSend + 'static,
{
    type Item = crate::Result<P::Item>;

    fn poll_next(
        self: Pin<&mut Self>,
        cx: &mut std::task::Context<'_>,
    ) -> std::task::Poll<Option<Self::Item>> {
        let mut this = self.project();
        let mut iter = this.iter.as_mut();
        loop {
            if let Some(current) = this.current.as_mut() {
                if let Some(item) = current.next() {
                    return task::Poll::Ready(Some(Ok(item)));
                }

                // Reset the iterator and poll for the next page.
                *this.current = None;
            }

            // Set the current_token to the next page only after iterating through all items.
            tracing::trace!(
                "updating continuation_token from {:?} to {:?}",
                this.continuation_token.as_ref().map(AsRef::as_ref),
                iter.continuation_token().map(AsRef::as_ref),
            );
            *this.continuation_token = iter.options.continuation_token.clone();

            match iter.as_mut().poll_next(cx) {
                task::Poll::Ready(page) => match page {
                    Some(Ok(page)) => match page.into_items().poll_unpin(cx) {
                        task::Poll::Ready(Ok(iter)) => {
                            *this.current = Some(iter);
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

impl<P, C> fmt::Debug for ItemIterator<P, C>
where
    P: Page + ConditionalSend + 'static,
    C: AsRef<str> + Clone + ConditionalSend + 'static,
{
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.debug_struct("ItemIterator")
            .field("iter", &self.iter)
            .field(
                "continuation_token",
                &self.continuation_token.as_ref().map(AsRef::as_ref),
            )
            .finish_non_exhaustive()
    }
}

impl<P, C> FusedStream for ItemIterator<P, C>
where
    P: Page + ConditionalSend + 'static,
    C: AsRef<str> + Clone + ConditionalSend + 'static,
{
    fn is_terminated(&self) -> bool {
        self.iter.is_terminated()
    }
}

#[allow(
    unsafe_code,
    reason = "`P` and `C` are `Send` so the iterator can be as well"
)]
unsafe impl<P, C> Send for ItemIterator<P, C>
where
    P: Page + ConditionalSend + 'static,
    C: AsRef<str> + Clone + ConditionalSend + 'static,
{
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
#[must_use = "streams do nothing unless polled"]
#[pin_project(project = PageIteratorProjection, project_replace = PageIteratorProjectionOwned)]
pub struct PageIterator<P, C = Url>
where
    P: ConditionalSend,
    C: AsRef<str> + ConditionalSend,
{
    #[pin]
    make_request: PagerFn<P, C>,
    options: PagerOptions<'static, C>,
    state: State<P, C>,
    added_span: bool,
}

impl<P, C> PageIterator<P, C>
where
    P: ConditionalSend,
    C: AsRef<str> + Clone + ConditionalSend + 'static,
{
    /// Creates a [`PageIterator`] from a callback that will be called repeatedly to request each page.
    ///
    /// This method expect a callback that accepts a single [`PagerState`] parameter, and returns a [`PagerResult`] value asynchronously.
    /// The `C` type parameter is the type of the next link/continuation token. It may be any [`Send`]able type.
    /// The result will be an asynchronous stream of [`Result`](crate::Result) values.
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
    /// # use azure_core::{Result, http::{RawResponse, pager::{PageIterator, PagerOptions, PagerResult, PagerState}, Pipeline, Request, Response, Method, Url}, json};
    /// # let api_version = "2025-06-04".to_string();
    /// # let pipeline: Pipeline = panic!("Not a runnable example");
    /// #[derive(serde::Deserialize)]
    /// struct ListItemsResult {
    ///     items: Vec<String>,
    ///     next_link: Option<String>,
    /// }
    /// let url = "https://example.com/my_paginated_api".parse().unwrap();
    /// let mut base_req = Request::new(url, Method::Get);
    /// let pager = PageIterator::new(move |next_link: PagerState<Url>, options: PagerOptions<'static, Url>| {
    ///     // The callback must be 'static, so you have to clone and move any values you want to use.
    ///     let pipeline = pipeline.clone();
    ///     let api_version = api_version.clone();
    ///     let mut req = base_req.clone();
    ///     Box::pin(async move {
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
    ///           .send(&options.context, &mut req, None)
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
    ///     })
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
    /// let pager = PageIterator::new(move |continuation, options| {
    ///     // The callback must be 'static, so you have to clone and move any values you want to use.
    ///     let pipeline = pipeline.clone();
    ///     let mut req = base_req.clone();
    ///     Box::pin(async move {
    ///         if let PagerState::More(continuation) = continuation {
    ///             req.insert_header("x-ms-continuation", continuation);
    ///         }
    ///         let resp: Response<ListItemsResult> = pipeline
    ///           .send(&options.context, &mut req, None)
    ///           .await?
    ///           .into();
    ///         Ok(PagerResult::from_response_header(resp, &HeaderName::from_static("x-ms-continuation")))
    ///     })
    /// }, None);
    /// ```
    pub fn new<
        F: Fn(PagerState<C>, PagerOptions<'static, C>) -> BoxedFuture<P, C>
            + ConditionalSend
            + 'static,
    >(
        make_request: F,
        options: Option<PagerOptions<'static, C>>,
    ) -> Self {
        let options = options.unwrap_or_default();
        let state = options
            .continuation_token
            .as_ref()
            .map_or_else(|| State::Init, |_| State::More);

        Self {
            make_request: Box::new(make_request),
            options,
            state,
            added_span: false,
        }
    }

    /// Gets the continuation token to pass to [`PagerOptions`] to resume paging in another iterator.
    pub fn continuation_token(&self) -> Option<&C> {
        self.options.continuation_token.as_ref()
    }

    /// Gets the continuation token to pass to [`PagerOptions`] to resume paging in another iterator.
    ///
    /// # Examples
    ///
    /// This takes ownership of the iterator and can be useful when constructing a new iterator.
    ///
    /// ```no_run
    /// # use azure_core::http::pager::PagerOptions;
    /// # use azure_security_keyvault_secrets::{SecretClient, models::SecretClientListSecretPropertiesOptions};
    /// # use futures::stream::TryStreamExt;
    /// # #[tokio::main] async fn main() -> azure_core::Result<()> {
    /// # let client: SecretClient = unimplemented!();
    /// let pager1 = client.list_secret_properties(None)?.into_pages();
    /// assert!(pager1.try_next().await?.is_some());
    /// let options = SecretClientListSecretPropertiesOptions {
    ///     method_options: PagerOptions {
    ///         continuation_token: pager1.into_continuation_token(),
    ///         ..Default::default()
    ///     },
    ///     ..Default::default()
    /// };
    /// let pager2 = client.list_secret_properties(Some(options))?.into_pages();
    /// assert!(pager2.try_next().await?.is_some());
    /// # Ok(()) }
    /// ```
    pub fn into_continuation_token(self) -> Option<C> {
        self.options.continuation_token
    }
}

impl<P, C> Stream for PageIterator<P, C>
where
    P: ConditionalSend,
    C: AsRef<str> + Clone + ConditionalSend + 'static,
{
    type Item = crate::Result<P>;

    fn poll_next(
        self: Pin<&mut Self>,
        cx: &mut std::task::Context<'_>,
    ) -> std::task::Poll<Option<Self::Item>> {
        let this = self.project();

        // When in the initial state or resuming from a continuation token,
        // attach a span to the context for the entire paging operation.
        if *this.state == State::Init || this.options.continuation_token.is_some() {
            tracing::debug!("establish a public API span for new pager.");

            // At the very start of polling, create a span for the entire request, and attach it to the context
            let span = create_public_api_span(&this.options.context, None, None);
            if let Some(ref s) = span {
                *this.added_span = true;
                this.options.context.insert(s.clone());
            }
        }

        let result = match *this.state {
            State::Init => {
                tracing::debug!("initial page request");
                let options = this.options.clone();
                let mut fut = (this.make_request)(PagerState::Initial, options);

                match fut.poll_unpin(cx) {
                    task::Poll::Ready(result) => result,
                    task::Poll::Pending => {
                        *this.state = State::Pending(fut);
                        return task::Poll::Pending;
                    }
                }
            }
            State::Pending(ref mut fut) => task::ready!(fut.poll_unpin(cx)),
            State::More => {
                let options = this.options.clone();
                let continuation_token = options
                    .continuation_token
                    .clone()
                    // We should always have a continuation_token with `State::More`.
                    .expect("expected continuation_token");
                tracing::debug!(
                    "subsequent page request to {:?}",
                    &continuation_token.as_ref(),
                );

                let mut fut = (this.make_request)(PagerState::More(continuation_token), options);

                match fut.poll_unpin(cx) {
                    task::Poll::Ready(result) => result,
                    task::Poll::Pending => {
                        *this.state = State::Pending(fut);
                        return task::Poll::Pending;
                    }
                }
            }
            State::Done => {
                tracing::debug!("done");
                // Set the `continuation_token` to None now that we are done.
                this.options.continuation_token = None;
                return task::Poll::Ready(None);
            }
        };

        // Update continuation token and instrumentation.
        match result {
            Err(e) => {
                if *this.added_span {
                    if let Some(span) = this.options.context.value::<Arc<dyn Span>>() {
                        // Mark the span as an error with an appropriate description.
                        span.set_status(SpanStatus::Error {
                            description: e.to_string(),
                        });
                        span.set_attribute("error.type", e.kind().to_string().into());
                        span.end();
                    }
                }

                *this.state = State::Done;
                task::Poll::Ready(Some(Err(e)))
            }

            Ok(PagerResult::More {
                response,
                continuation: continuation_token,
            }) => {
                // Set the `continuation_token` to the next page.
                this.options.continuation_token = Some(continuation_token);
                *this.state = State::More;
                task::Poll::Ready(Some(Ok(response)))
            }

            Ok(PagerResult::Done { response }) => {
                // Set the `continuation_token` to None now that we are done.
                this.options.continuation_token = None;
                *this.state = State::Done;

                // When the result is done, finalize the span. Note that we only do that if we created the span in the first place;
                // otherwise, it is the responsibility of the caller to end their span.
                if *this.added_span {
                    if let Some(span) = this.options.context.value::<Arc<dyn Span>>() {
                        span.end();
                    }
                }

                task::Poll::Ready(Some(Ok(response)))
            }
        }
    }
}

impl<P, C> fmt::Debug for PageIterator<P, C>
where
    P: ConditionalSend,
    C: AsRef<str> + ConditionalSend,
{
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.debug_struct("PageIterator")
            .field(
                "continuation_token",
                &self.options.continuation_token.as_ref().map(AsRef::as_ref),
            )
            .field("options", &self.options)
            .field("state", &self.state)
            .field("added_span", &self.added_span)
            .finish_non_exhaustive()
    }
}

impl<P, C> FusedStream for PageIterator<P, C>
where
    P: ConditionalSend,
    C: AsRef<str> + Clone + ConditionalSend + 'static,
{
    fn is_terminated(&self) -> bool {
        self.state == State::Done
    }
}

#[allow(
    unsafe_code,
    reason = "`P` and `C` are `Send` so the iterator can be as well"
)]
unsafe impl<P, C> Send for PageIterator<P, C>
where
    P: ConditionalSend,
    C: AsRef<str> + Clone + ConditionalSend + 'static,
{
}

enum State<P, C: AsRef<str>> {
    Init,
    Pending(BoxedFuture<P, C>),
    More,
    Done,
}

impl<P, C: AsRef<str>> fmt::Debug for State<P, C> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            State::Init => f.write_str("Init"),
            State::Pending(..) => f.debug_tuple("Pending").finish_non_exhaustive(),
            State::More => f.write_str("More"),
            State::Done => f.write_str("Done"),
        }
    }
}

impl<P, C: AsRef<str>> PartialEq for State<P, C> {
    fn eq(&self, other: &Self) -> bool {
        // Only needs to compare if both states are Init or Done; internally, we don't care about any other states.
        matches!(
            (self, other),
            (State::Init, State::Init) | (State::Done, State::Done)
        )
    }
}

#[cfg(test)]
mod tests {
    use super::{ItemIterator, PageIterator, Pager, PagerOptions, PagerResult, PagerState};
    use crate::http::{
        headers::{HeaderName, HeaderValue},
        pager::BoxedFuture,
        JsonFormat, RawResponse, Response, StatusCode,
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
        let pager: Pager<Page, JsonFormat, String> = Pager::new(
            |continuation: PagerState<String>, _ctx| {
                Box::pin(async move {
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
                })
            },
            None,
        );
        let items: Vec<i32> = pager.try_collect().await.unwrap();
        assert_eq!(vec![1, 2, 3], items.as_slice())
    }

    #[tokio::test]
    async fn callback_item_pagination_error() {
        let pager: Pager<Page, JsonFormat, String> = ItemIterator::new(
            |continuation: PagerState<String>, _options| {
                Box::pin(async move {
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
                })
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
    async fn page_iterator_iterate_all_pages() {
        // Create a PageIterator and iterate through all three pages.
        let mut pager = PageIterator::new(make_three_page_callback(), None);

        // Should start with no continuation_token.
        assert_eq!(pager.continuation_token(), None);

        // Get first page.
        let first_page = pager
            .next()
            .await
            .expect("expected first page")
            .expect("expected successful first page")
            .into_model()
            .expect("expected page");
        assert_eq!(first_page.page, Some(1));
        assert_eq!(first_page.items, vec![1, 2, 3]);

        // continuation_token should now point to second page.
        assert_eq!(
            pager.continuation_token().map(AsRef::as_ref),
            Some("next-token-1")
        );

        // Get second page.
        let second_page = pager
            .next()
            .await
            .expect("expected second page")
            .expect("expected successful second page")
            .into_model()
            .expect("expected page");
        assert_eq!(second_page.page, Some(2));
        assert_eq!(second_page.items, vec![4, 5, 6]);

        // continuation_token should now point to third page.
        assert_eq!(
            pager.continuation_token().map(AsRef::as_ref),
            Some("next-token-2")
        );

        // Get third page.
        let third_page = pager
            .next()
            .await
            .expect("expected third page")
            .expect("expected successful third page")
            .into_model()
            .expect("expected page");
        assert_eq!(third_page.page, None);
        assert_eq!(third_page.items, vec![7, 8, 9]);

        // continuation_token should now be None (done).
        assert_eq!(pager.continuation_token(), None);

        // Verify stream is exhausted.
        assert!(pager.next().await.is_none());
    }

    #[tokio::test]
    async fn page_iterator_with_continuation_token() {
        // Create the first PageIterator.
        let mut first_pager = PageIterator::new(make_three_page_callback(), None);

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
        let mut second_pager = PageIterator::new(
            make_three_page_callback(),
            Some(PagerOptions {
                continuation_token: Some(continuation_token.into()),
                ..Default::default()
            }),
        );

        // Should start with link to second page.
        assert_eq!(
            second_pager.continuation_token().map(AsRef::as_ref),
            Some("next-token-1"),
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
            second_pager.continuation_token().map(AsRef::as_ref),
            Some("next-token-2")
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
        let mut item_pager = ItemIterator::new(make_three_page_callback(), None);

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
        let mut item_pager = ItemIterator::new(make_three_page_callback(), None);

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
        assert_eq!(
            page_pager.continuation_token().map(AsRef::as_ref),
            Some("next-token-1")
        );

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
        let mut first_pager = ItemIterator::new(make_three_page_callback(), None);

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
        let mut second_pager = ItemIterator::new(
            make_three_page_callback(),
            Some(PagerOptions {
                continuation_token: continuation_token.map(Into::into),
                ..Default::default()
            }),
        );

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
        let mut first_pager = ItemIterator::new(make_three_page_callback(), None);

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
        let continuation_token = first_pager.into_continuation_token();
        assert_eq!(continuation_token.as_deref(), Some("next-token-1"));

        // Create the second ItemIterator with continuation token.
        let mut second_pager = ItemIterator::new(
            make_three_page_callback(),
            Some(PagerOptions {
                continuation_token,
                ..Default::default()
            }),
        );

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
        let mut first_pager = ItemIterator::new(make_three_page_callback(), None);

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
        let mut second_pager = ItemIterator::new(
            make_three_page_callback(),
            Some(PagerOptions {
                continuation_token: continuation_token.map(Into::into),
                ..Default::default()
            }),
        );

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

    #[allow(clippy::type_complexity)]
    fn make_three_page_callback(
    ) -> impl Fn(PagerState<String>, PagerOptions<'_, String>) -> BoxedFuture<Response<Page>, String>
    {
        |continuation: PagerState<String>, _options| {
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

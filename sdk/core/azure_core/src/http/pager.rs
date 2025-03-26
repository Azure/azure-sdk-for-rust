// Copyright (c) Microsoft Corporation. All rights reserved.
// Licensed under the MIT License.

use crate::http::{headers::HeaderName, response::Response};
use futures::{stream::unfold, Stream};
use std::{future::Future, pin::Pin};
use typespec::Error;

/// The result of fetching a single page from a [`Pager`], whether the `Pager` should continue or is complete.
#[derive(Debug)]
pub enum PagerResult<T, C> {
    /// The [`Pager`] may fetch additional pages with the included `continuation` token.
    Continue { value: T, continuation: C },
    /// The [`Pager`] is complete and there are no additional pages to fetch.
    Complete { value: T },
}

impl<T> PagerResult<Response<T>, String> {
    /// Creates a [`PagerResult<T, C>`] from the provided response, extracting the continuation value from the provided header.
    ///
    /// If the provided response has a header with the matching name, this returns [`PagerResult::Continue`], using the value from the header as the continuation.
    /// If the provided response does not have a header with the matching name, this returns [`PagerResult::Complete`].
    pub fn from_response_header(response: Response<T>, header_name: &HeaderName) -> Self {
        match response.headers().get_optional_string(header_name) {
            Some(continuation) => PagerResult::Continue {
                value: response,
                continuation,
            },
            None => PagerResult::Complete { value: response },
        }
    }
}

/// Represents a paginated stream of results generated through HTTP requests to a service.
///
/// Specifically, this is a [`PageStream`] that yields [`Response<T>`] values.
pub type Pager<T> = PageStream<Response<T>>;

/// Represents a paginated stream of results from a service.
#[pin_project::pin_project]
pub struct PageStream<T> {
    #[pin]
    #[cfg(not(target_arch = "wasm32"))]
    stream: Pin<Box<dyn Stream<Item = Result<T, Error>> + Send>>,

    #[pin]
    #[cfg(target_arch = "wasm32")]
    stream: Pin<Box<dyn Stream<Item = Result<T, Error>>>>,
}

impl<T> PageStream<T> {
    /// Creates a [`Pager<T>`] from a callback that will be called repeatedly to request each page.
    ///
    /// This method expect a callback that accepts a single `Option<C>` parameter, and returns a [`PagerResult<T, C>`] value, asynchronously.
    /// The `C` type parameter is the type of the continuation/state. It may be any [`Send`]able type.
    /// The result will be an asynchronous stream of [`Result<T>`](typespec::Result<T>) values.
    ///
    /// The first time your callback is called, it will be called with [`Option::None`], indicating no continuation value is present.
    /// Your callback must return one of:
    /// * `Ok(result)` - The request succeeded, and the provided [`PagerResult`] indicates the value to return and if there are more pages.
    /// * `Err(..)` - The request failed. The error will be yielded to the stream, the stream will end, and the callback will not be called again.
    ///
    /// ## Examples
    ///
    /// ```rust,no_run
    /// # use azure_core::http::{Context, Pager, PagerResult, Pipeline, Request, Response, Method, headers::HeaderName};
    /// # let pipeline: Pipeline = panic!("Not a runnable example");
    /// # struct MyModel;
    /// let url = "https://example.com/my_paginated_api".parse().unwrap();
    /// let mut base_req = Request::new(url, Method::Get);
    /// let pager = Pager::from_callback(move |continuation| {
    ///     // The callback must be 'static, so you have to clone and move any values you want to use.
    ///     let pipeline = pipeline.clone();
    ///     let mut req = base_req.clone();
    ///     async move {
    ///         if let Some(continuation) = continuation {
    ///             req.insert_header("x-continuation", continuation);
    ///         }
    ///         let resp: Response<MyModel> = pipeline
    ///           .send(&Context::new(), &mut req)
    ///           .await?;
    ///         Ok(PagerResult::from_response_header(resp, &HeaderName::from_static("x-next-continuation")))
    ///     }
    /// });
    /// ```
    pub fn from_callback<
        // This is a bit gnarly, but the only thing that differs between the WASM/non-WASM configs is the presence of Send bounds.
        #[cfg(not(target_arch = "wasm32"))] C: Send + 'static,
        #[cfg(not(target_arch = "wasm32"))] F: Fn(Option<C>) -> Fut + Send + 'static,
        #[cfg(not(target_arch = "wasm32"))] Fut: Future<Output = Result<PagerResult<T, C>, typespec::Error>> + Send + 'static,
        #[cfg(target_arch = "wasm32")] C: 'static,
        #[cfg(target_arch = "wasm32")] F: Fn(Option<C>) -> Fut + 'static,
        #[cfg(target_arch = "wasm32")] Fut: Future<Output = Result<PagerResult<T, C>, typespec::Error>> + 'static,
    >(
        make_request: F,
    ) -> Self {
        let stream = unfold(
            // We flow the `make_request` callback through the state value so that we can avoid cloning.
            (State::Init, make_request),
            |(state, make_request)| async move {
                let result = match state {
                    State::Init => make_request(None).await,
                    State::Continuation(c) => make_request(Some(c)).await,
                    State::Done => return None,
                };
                let (item, next_state) = match result {
                    Err(e) => return Some((Err(e), (State::Done, make_request))),
                    Ok(PagerResult::Continue {
                        value: response,
                        continuation,
                    }) => (Ok(response), State::Continuation(continuation)),
                    Ok(PagerResult::Complete { value: response }) => (Ok(response), State::Done),
                };

                // Flow 'make_request' through to avoid cloning
                Some((item, (next_state, make_request)))
            },
        );
        Self {
            stream: Box::pin(stream),
        }
    }
}

impl<T> futures::Stream for PageStream<T> {
    type Item = Result<T, Error>;

    fn poll_next(
        self: Pin<&mut Self>,
        cx: &mut std::task::Context<'_>,
    ) -> std::task::Poll<Option<Self::Item>> {
        self.project().stream.poll_next(cx)
    }
}

impl<T> std::fmt::Debug for PageStream<T> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("Pager").finish_non_exhaustive()
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
enum State<T> {
    Init,
    Continuation(T),
    Done,
}

#[cfg(test)]
mod tests {
    use std::collections::HashMap;

    use crate::http::Model;
    use futures::StreamExt;
    use serde::Deserialize;

    use crate::http::{
        headers::{HeaderName, HeaderValue},
        Pager, PagerResult, Response, StatusCode,
    };

    #[tokio::test]
    pub async fn standard_pagination() {
        #[derive(Model, Deserialize, Debug, PartialEq, Eq)]
        #[typespec(crate = "crate")]
        struct Page {
            pub page: usize,
        }

        let pager: Pager<Page> = Pager::from_callback(|continuation| async move {
            match continuation {
                None => Ok(PagerResult::Continue {
                    value: Response::from_bytes(
                        StatusCode::Ok,
                        HashMap::from([(
                            HeaderName::from_static("x-test-header"),
                            HeaderValue::from_static("page-1"),
                        )])
                        .into(),
                        r#"{"page":1}"#,
                    ),
                    continuation: "1",
                }),
                Some("1") => Ok(PagerResult::Continue {
                    value: Response::from_bytes(
                        StatusCode::Ok,
                        HashMap::from([(
                            HeaderName::from_static("x-test-header"),
                            HeaderValue::from_static("page-2"),
                        )])
                        .into(),
                        r#"{"page":2}"#,
                    ),
                    continuation: "2",
                }),
                Some("2") => Ok(PagerResult::Complete {
                    value: Response::from_bytes(
                        StatusCode::Ok,
                        HashMap::from([(
                            HeaderName::from_static("x-test-header"),
                            HeaderValue::from_static("page-3"),
                        )])
                        .into(),
                        r#"{"page":3}"#,
                    ),
                }),
                _ => {
                    panic!("Unexpected continuation value")
                }
            }
        });
        let pages: Vec<(String, Page)> = pager
            .then(|r| async move {
                let r = r.unwrap();
                let header = r
                    .headers()
                    .get_optional_string(&HeaderName::from_static("x-test-header"))
                    .unwrap();
                let body = r.into_body().await.unwrap();
                (header, body)
            })
            .collect()
            .await;
        assert_eq!(
            &[
                ("page-1".to_string(), Page { page: 1 }),
                ("page-2".to_string(), Page { page: 2 }),
                ("page-3".to_string(), Page { page: 3 }),
            ],
            pages.as_slice()
        )
    }

    #[tokio::test]
    pub async fn error_stops_pagination() {
        #[derive(Model, Deserialize, Debug, PartialEq, Eq)]
        #[typespec(crate = "crate")]
        struct Page {
            pub page: usize,
        }

        let pager: Pager<Page> = Pager::from_callback(|continuation| async move {
            match continuation {
                None => Ok(PagerResult::Continue {
                    value: Response::from_bytes(
                        StatusCode::Ok,
                        HashMap::from([(
                            HeaderName::from_static("x-test-header"),
                            HeaderValue::from_static("page-1"),
                        )])
                        .into(),
                        r#"{"page":1}"#,
                    ),
                    continuation: "1",
                }),
                Some("1") => Err(typespec::Error::message(
                    typespec::error::ErrorKind::Other,
                    "yon request didst fail",
                )),
                _ => {
                    panic!("Unexpected continuation value")
                }
            }
        });
        let pages: Vec<Result<(String, Page), typespec::Error>> = pager
            .then(|r| async move {
                let r = r?;
                let header = r
                    .headers()
                    .get_optional_string(&HeaderName::from_static("x-test-header"))
                    .unwrap();
                let body = r.into_body().await?;
                Ok((header, body))
            })
            .collect()
            .await;
        assert_eq!(2, pages.len());
        assert_eq!(
            &("page-1".to_string(), Page { page: 1 }),
            pages[0].as_ref().unwrap()
        );

        let err = pages[1].as_ref().unwrap_err();
        assert_eq!(&typespec::error::ErrorKind::Other, err.kind());
        assert_eq!("yon request didst fail", format!("{}", err));
    }
}

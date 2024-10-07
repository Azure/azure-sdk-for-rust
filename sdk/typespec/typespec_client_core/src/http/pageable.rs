// Copyright (c) Microsoft Corporation. All rights reserved.
// Licensed under the MIT License.

use std::{future::Future, pin::Pin};

use futures::{stream::unfold, Stream};
use typespec::Error;

use crate::http::Response;

#[pin_project::pin_project]
pub struct Pager<T> {
    #[pin]
    #[cfg(not(target_arch = "wasm32"))]
    stream: Pin<Box<dyn Stream<Item = Result<Response<T>, Error>> + Send>>,

    #[pin]
    #[cfg(target_arch = "wasm32")]
    stream: Pin<Box<dyn Stream<Item = Result<Response<T>, Error>>>>,
}

impl<T> Pager<T> {
    pub fn from_fn<
        // This is a bit gnarly, but the only thing that differs between the WASM/non-WASM configs is the presence of Send bounds.
        #[cfg(not(target_arch = "wasm32"))] C: Send + 'static,
        #[cfg(target_arch = "wasm32")] C: 'static,
        E: Into<typespec::Error>,
        #[cfg(not(target_arch = "wasm32"))] F: Fn(Option<C>) -> Fut + Send + 'static,
        #[cfg(target_arch = "wasm32")] F: Fn(Option<C>) -> Fut + 'static,
        #[cfg(not(target_arch = "wasm32"))] Fut: Future<Output = Result<(Response<T>, Option<C>), E>> + Send + 'static,
        #[cfg(target_arch = "wasm32")] Fut: Future<Output = Result<(Response<T>, Option<C>), E>> + 'static,
    >(
        make_request: F,
    ) -> Self {
        let stream = unfold(
            (State::Init, make_request),
            |(state, make_request)| async move {
                let result = match state {
                    State::Init => make_request(None).await,
                    State::Continuation(c) => make_request(Some(c)).await,
                    State::Done => return None,
                };
                let (response, continuation) = match result {
                    Err(e) => return Some((Err(e.into()), (State::Done, make_request))),
                    Ok(r) => r,
                };
                let next_state = continuation.map_or(State::Done, State::Continuation);
                Some((Ok(response), (next_state, make_request)))
            },
        );
        Self {
            stream: Box::pin(stream),
        }
    }
}

impl<T> std::fmt::Debug for Pager<T> {
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

// Copyright (c) Microsoft Corporation. All rights reserved.
// Licensed under the MIT License.

use azure_core::tracing::{Span, WithSpan};
use core::task::{Context, Poll};
use std::future::Future;
use std::pin::Pin;

/// A wrapper that implements WithSpan for futures
pub struct SpanFuture<F> {
    future: F,
    span: Box<dyn Span + Send + Sync>,
}

impl<F> Future for SpanFuture<F>
where
    F: Future + Send,
{
    type Output = F::Output;

    fn poll(self: Pin<&mut Self>, cx: &mut Context<'_>) -> Poll<Self::Output> {
        // Safety: We never move the future
        let this = unsafe { self.get_unchecked_mut() };
        let future = unsafe { Pin::new_unchecked(&mut this.future) };

        todo!();

        //     // Set the span as active for the duration of this poll
        //     let _guard = this.span.inner.as_ref().span_context();

        //     future.poll(cx)
    }
}

impl<F> WithSpan for F
where
    F: Future + Send + Sync + 'static,
{
    fn with_span(
        self,
        span: Box<dyn Span + Send + Sync>,
    ) -> Box<dyn Future<Output = F::Output> + Send + Sync> {
        Box::new(SpanFuture { future: self, span })
    }
}

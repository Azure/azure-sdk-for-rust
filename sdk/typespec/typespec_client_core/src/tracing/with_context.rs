// Copyright (C) Microsoft Corporation. All rights reserved.
// Licensed under the MIT License.

// cspell: ignore cloneable

use super::Span;
use crate::http::Context;
use pin_project::pin_project;
use std::{
    future::Future,
    pin::Pin,
    sync::Arc,
    task::{Context as TaskContext, Poll},
};

impl<T: Sized + Future> FutureExt for T {}

impl<T: std::future::Future> std::future::Future for WithContext<'_, T> {
    type Output = T::Output;

    fn poll(self: Pin<&mut Self>, task_cx: &mut TaskContext<'_>) -> Poll<Self::Output> {
        let this = self.project();
        if let Some(span) = this.context.value::<Arc<dyn Span>>() {
            let _guard = span.set_current(this.context);

            this.inner.poll(task_cx)
        } else {
            this.inner.poll(task_cx)
        }
    }
}

#[pin_project]
/// A future, that has an associated span.
pub struct WithContext<'a, T> {
    #[pin]
    inner: T,
    context: Context<'a>,
}

/// Extension trait allowing futures, streams, and sinks to be traced with a span.
pub trait FutureExt: Sized {
    /// Attaches the provided [`Context`] to this type, returning a `WithContext`
    /// wrapper.
    ///
    /// When the wrapped type is a future, stream, or sink, the attached context
    /// will be set as current while it is being polled.
    ///
    /// [`Context`]: Context
    fn with_context(self, context: Context) -> WithContext<Self> {
        WithContext {
            inner: self,
            context,
        }
    }
}

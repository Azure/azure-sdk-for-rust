// Copyright (c) Microsoft Corporation. All rights reserved.
// Licensed under the MIT License.

use std::{future::Future, pin::Pin, task::Poll, time::Duration};

use futures::{
    future::{self, Either, Select},
    ready,
};
use pin_project::pin_project;
use tokio::time::{sleep, Sleep};

pub trait OptionalTimeoutFutureExt: Sized {
    fn timeout(self, timeout: Option<Duration>) -> Timeout<Self>;
}

impl<F: Future> OptionalTimeoutFutureExt for F {
    fn timeout(self, timeout: Option<Duration>) -> Timeout<Self> {
        let timeout = timeout.unwrap_or(Duration::MAX);
        Timeout {
            fut: future::select(Box::pin(self), Box::pin(sleep(timeout))),
            timeout_duration: timeout,
        }
    }
}

#[pin_project]
pub struct Timeout<F> {
    #[pin]
    fut: Select<Pin<Box<F>>, Pin<Box<Sleep>>>,
    timeout_duration: Duration,
}

impl<F: Future> Future for Timeout<F> {
    type Output = std::result::Result<F::Output, TimeoutError>;

    fn poll(self: Pin<&mut Self>, cx: &mut std::task::Context<'_>) -> Poll<Self::Output> {
        let this = self.project();
        match ready!(this.fut.poll(cx)) {
            future::Either::Left((output, _)) => Poll::Ready(Ok(output)),
            future::Either::Right((_timeout, _)) => {
                Poll::Ready(Err(TimeoutError(*this.timeout_duration)))
            }
        }
    }
}

pub trait OptionalFlatTimeoutFutureExt: Sized {
    fn flat_timeout(self, timeout: Option<Duration>) -> FlatTimeout<Self>;
}

impl<F, T, E> OptionalFlatTimeoutFutureExt for F
where
    F: Future<Output = Result<T, E>>,
{
    fn flat_timeout(self, timeout: Option<Duration>) -> FlatTimeout<Self> {
        let timeout = timeout.unwrap_or(Duration::MAX);
        FlatTimeout {
            fut: future::select(Box::pin(self), Box::pin(sleep(timeout))),
            timeout_duration: timeout,
        }
    }
}

#[pin_project]
pub struct FlatTimeout<F> {
    #[pin]
    fut: Select<Pin<Box<F>>, Pin<Box<Sleep>>>,
    timeout_duration: Duration,
}

impl<F, T, E> Future for FlatTimeout<F>
where
    F: Future<Output = Result<T, E>>,
{
    type Output = std::result::Result<T, Either<E, TimeoutError>>;

    fn poll(self: Pin<&mut Self>, cx: &mut std::task::Context<'_>) -> Poll<Self::Output> {
        let this = self.project();
        match ready!(this.fut.poll(cx)) {
            future::Either::Left((Ok(ok), _)) => Poll::Ready(Ok(ok)),
            future::Either::Left((Err(err), _)) => Poll::Ready(Err(Either::Left(err))),
            future::Either::Right((_timeout, _)) => {
                Poll::Ready(Err(Either::Right(TimeoutError(*this.timeout_duration))))
            }
        }
    }
}

pub struct TimeoutError(pub Duration);

impl From<TimeoutError> for azure_core::Error {
    fn from(value: TimeoutError) -> Self {
        azure_core::Error::with_message(
            azure_core::error::ErrorKind::Other,
            format!(
                "The operation timed out after {:.2} seconds",
                value.0.as_secs_f32()
            ),
        )
    }
}

impl std::fmt::Display for TimeoutError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "The operation timed out after {:.2} seconds",
            self.0.as_secs_f32()
        )
    }
}

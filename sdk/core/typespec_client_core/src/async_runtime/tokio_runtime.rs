// Copyright (c) Microsoft Corporation. All rights reserved.
// Licensed under the MIT License.

use super::{AsyncRuntime, SpawnedTask, TaskFuture};
use crate::time::Duration;
use std::{
    error::Error,
    pin::Pin,
    task::{Context, Poll},
};
use tokio::{task, time};

/// An [`AsyncRuntime`] using `tokio` based APIs.
pub(crate) struct TokioRuntime;

impl AsyncRuntime for TokioRuntime {
    fn spawn(&self, f: TaskFuture) -> Pin<Box<dyn SpawnedTask>> {
        let handle = ::tokio::spawn(f);
        Box::pin(JoinHandle {
            handle: Some(handle),
        })
    }

    fn sleep(&self, duration: Duration) -> Pin<Box<dyn std::future::Future<Output = ()> + Send>> {
        Box::pin(time::sleep(
            duration
                .try_into()
                .expect("Failed to convert duration to tokio format"),
        ))
    }

    fn yield_now(&self) -> TaskFuture {
        Box::pin(async {
            tokio::task::yield_now().await;
        })
    }
}

struct JoinHandle {
    handle: Option<task::JoinHandle<()>>,
}

impl std::future::Future for JoinHandle {
    type Output = Result<(), Box<dyn Error + Send>>;

    fn poll(mut self: Pin<&mut Self>, cx: &mut Context<'_>) -> Poll<Self::Output> {
        if let Some(handle) = &mut self.handle {
            match Pin::new(handle).poll(cx) {
                Poll::Ready(_) => {
                    self.handle = None;
                    Poll::Ready(Ok(()))
                }
                Poll::Pending => Poll::Pending,
            }
        } else {
            Poll::Ready(Ok(()))
        }
    }
}

impl SpawnedTask for JoinHandle {
    fn abort(&self) {
        if let Some(handle) = &self.handle {
            handle.abort();
        }
    }
}

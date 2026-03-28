// Copyright (c) Microsoft Corporation. All rights reserved.
// Licensed under the MIT License.

use super::{AsyncRuntime, SpawnedTask, TaskFuture};
use crate::{async_runtime::AbortableTask, time::Duration};
use pin_project::pin_project;
use std::{
    error::Error,
    pin::Pin,
    task::{Context, Poll},
};
use tokio::{task, time};

/// An [`AsyncRuntime`] using `tokio` based APIs.
pub(crate) struct TokioRuntime;

impl AsyncRuntime for TokioRuntime {
    fn spawn(&self, f: TaskFuture) -> SpawnedTask {
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

#[pin_project]
struct JoinHandle {
    #[pin]
    handle: Option<task::JoinHandle<()>>,
}

impl std::future::Future for JoinHandle {
    type Output = Result<(), Box<dyn Error + Send>>;

    fn poll(self: Pin<&mut Self>, cx: &mut Context<'_>) -> Poll<Self::Output> {
        let mut this = self.project();
        if let Some(handle) = this.handle.as_mut().as_pin_mut() {
            match handle.poll(cx) {
                Poll::Ready(_) => {
                    this.handle.set(None);
                    Poll::Ready(Ok(()))
                }
                Poll::Pending => Poll::Pending,
            }
        } else {
            Poll::Ready(Ok(()))
        }
    }
}

impl AbortableTask for JoinHandle {
    fn abort(&self) {
        if let Some(handle) = &self.handle {
            handle.abort();
        }
    }
}

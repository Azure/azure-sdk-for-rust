// Copyright (c) Microsoft Corporation. All rights reserved.
// Licensed under the MIT License.

use super::{AsyncRuntime, SpawnedTask, TaskFuture};
use crate::time::Duration;
use std::pin::Pin;

/// An [`AsyncRuntime`] using `tokio` based APIs.
pub(crate) struct TokioRuntime;

impl AsyncRuntime for TokioRuntime {
    fn spawn(&self, f: TaskFuture) -> SpawnedTask {
        let handle = ::tokio::spawn(f);
        Box::pin(async move {
            handle
                .await
                .map_err(|e| Box::new(e) as Box<dyn std::error::Error + Send>)
        })
    }

    fn sleep(&self, duration: Duration) -> Pin<Box<dyn std::future::Future<Output = ()> + Send>> {
        Box::pin(::tokio::time::sleep(
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

// Copyright (c) Microsoft Corporation. All rights reserved.
// Licensed under the MIT License.

use super::{AsyncRuntime, SpawnedTask, TaskFuture};
use std::fmt::Debug;

/// A [`TaskSpawner`] using [`tokio::spawn`].
#[derive(Debug)]
pub struct TokioRuntime;

impl AsyncRuntime for TokioRuntime {
    fn spawn(&self, f: TaskFuture) -> SpawnedTask {
        let handle = ::tokio::spawn(f);
        Box::pin(async move {
            handle
                .await
                .map_err(|e| Box::new(e) as Box<dyn std::error::Error + Send>)
        })
    }

    fn sleep(&self, duration: std::time::Duration) -> TaskFuture {
        Box::pin(::tokio::time::sleep(duration))
    }
}

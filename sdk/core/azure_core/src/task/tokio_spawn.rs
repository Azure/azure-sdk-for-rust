// Copyright (c) Microsoft Corporation. All rights reserved.
// Licensed under the MIT License.

use super::{SpawnedTask, TaskFuture, TaskSpawner};
use std::fmt::Debug;

/// A [`TaskSpawner`] using [`tokio::spawn`].
#[derive(Debug)]
pub struct TokioSpawner;

impl TaskSpawner for TokioSpawner {
    fn spawn(&self, f: TaskFuture) -> SpawnedTask {
        let handle = ::tokio::spawn(f);
        Box::pin(async move {
            handle
                .await
                .map_err(|e| Box::new(e) as Box<dyn std::error::Error + Send>)
        })
    }
}

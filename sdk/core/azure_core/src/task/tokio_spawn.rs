// Copyright (c) Microsoft Corporation. All rights reserved.
// Licensed under the MIT License.

use super::{SpawnHandleMethods, SpawnHandleT, TaskFuture, TaskSpawner};
use async_trait::async_trait;
use std::fmt::Debug;

/// A [`TaskSpawner`] using [`tokio::spawn`].
#[derive(Debug)]
pub struct TokioSpawner;

impl TaskSpawner for TokioSpawner {
    fn spawn(&self, f: TaskFuture) -> SpawnHandleT<TokioSpawnHandle> {
        let handle = ::tokio::spawn(f);
        SpawnHandleT {
            inner: TokioSpawnHandle(handle),
        }
    }
}

#[derive(Debug)]
pub struct TokioSpawnHandle(tokio::task::JoinHandle<()>);

#[async_trait]
impl SpawnHandleMethods for TokioSpawnHandle {
    /// Wait for the task to complete and return the result.
    async fn wait(self) -> crate::Result<()> {
        self.0.await.map_err(|e| {
            crate::Error::message(
                crate::error::ErrorKind::Other,
                format!("Task was cancelled before completion: {}", e),
            )
        })
    }
}

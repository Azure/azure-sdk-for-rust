// Copyright (c) Microsoft Corporation. All rights reserved.
// Licensed under the MIT License.

use super::{TaskFuture, TaskSpawner};

/// A [`TaskSpawner`] using [`tokio::spawn`].
#[derive(Debug)]
pub struct TokioSpawner;

impl TaskSpawner for TokioSpawner {
    fn spawn(&self, f: TaskFuture) -> SpawnHandle {
        let handle = ::tokio::spawn(f);
        SpawnHandle(handle)
    }
}
#[derive(Debug)]
pub struct SpawnHandle(tokio::task::JoinHandle<()>);

impl SpawnHandle {
    /// Wait for the task to complete and return the result.
    pub async fn await_result(self) -> crate::Result<()> {
        self.0.await.map_err(|e| {
            crate::Error::message(
                crate::error::ErrorKind::Other,
                format!("Task was cancelled before completion: {}", e),
            )
        })
    }
}

// Copyright (c) Microsoft Corporation. All rights reserved.
// Licensed under the MIT License.

use super::{SpawnHandle, TaskFuture, TaskSpawner};
use std::fmt::Debug;

/// A [`TaskSpawner`] using [`tokio::spawn`].
#[derive(Debug)]
pub struct TokioSpawner;

impl TaskSpawner for TokioSpawner {
    fn spawn(&self, f: TaskFuture) -> SpawnHandle {
        let handle = ::tokio::spawn(f);
        SpawnHandle::Tokio(TokioSpawnHandle(handle))
    }
}

#[derive(Debug)]
pub struct TokioSpawnHandle(tokio::task::JoinHandle<()>);

impl TokioSpawnHandle {
    /// Wait for the task to complete and return the result.
    pub(crate) async fn wait(self) -> crate::Result<()> {
        self.0.await.map_err(|e| {
            crate::Error::message(
                crate::error::ErrorKind::Other,
                format!("Task was cancelled before completion: {}", e),
            )
        })
    }
}

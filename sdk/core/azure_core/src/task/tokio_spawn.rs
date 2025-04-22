// Copyright (c) Microsoft Corporation. All rights reserved.
// Licensed under the MIT License.

use super::{SpawnHandle, TaskFuture, TaskSpawner};
use async_trait::async_trait;
use std::fmt::Debug;

/// A [`TaskSpawner`] using [`tokio::spawn`].
#[derive(Debug)]
pub struct TokioSpawner;

impl TaskSpawner for TokioSpawner {
    fn spawn(&self, f: TaskFuture) -> Box<dyn SpawnHandle> {
        let handle = ::tokio::spawn(f);
        Box::new(TokioSpawnHandle(handle))
    }
}

#[derive(Debug)]
pub struct TokioSpawnHandle(tokio::task::JoinHandle<()>);

#[async_trait]
impl SpawnHandle for TokioSpawnHandle {
    /// Wait for the task to complete and return the result.
    async fn wait(self: Box<Self>) -> crate::Result<()> {
        self.0.await.map_err(|e| {
            crate::Error::message(
                crate::error::ErrorKind::Other,
                format!("Task was cancelled before completion: {}", e),
            )
        })
    }
}

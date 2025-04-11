// Copyright (c) Microsoft Corporation. All rights reserved.
// Licensed under the MIT License.

//! Asynchronous task execution utilities.

use std::{fmt, future::Future, pin::Pin, sync::Arc};

#[cfg(not(feature = "tokio"))]
mod standard;

#[cfg(feature = "tokio")]
mod tokio_spawn;

#[cfg(test)]
mod tests;

#[cfg(not(feature = "tokio"))]
pub use standard::StdSpawner;

#[cfg(feature = "tokio")]
pub use tokio_spawn::TokioSpawner;

/// Creates a new [`TaskSpawner`].
///
/// This returns a [`TaskSpawner`] that spawns a [`std::thread`] to run the task unless `tokio` was enabled,
/// in which case an executor is returned that calls [`tokio::task::spawn`].
pub fn new_task_spawner() -> Arc<dyn TaskSpawner> {
    #[cfg(not(feature = "tokio"))]
    {
        Arc::new(StdSpawner)
    }
    #[cfg(feature = "tokio")]
    {
        Arc::new(TokioSpawner)
    }
}

#[cfg(feature = "tokio")]
#[derive(Debug)]
pub struct SpawnHandle(tokio::task::JoinHandle<()>);

#[cfg(feature = "tokio")]
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

#[cfg(not(feature = "tokio"))]
#[derive(Debug)]
pub struct SpawnHandle(std::thread::JoinHandle<()>);

#[cfg(not(feature = "tokio"))]
impl SpawnHandle {
    /// Wait for the task to complete and return the result.
    pub async fn await_result(self) -> crate::Result<()> {
        self.0.join().map_err(|_| {
            crate::Error::message(
                crate::error::ErrorKind::Other,
                "Task was cancelled before completion.",
            )
        })
    }
}

/// An async command runner.
pub trait TaskSpawner: Send + Sync + fmt::Debug {
    /// Spawn a task that executes a given future and returns the output.
    fn spawn(&self, f: Pin<Box<dyn Future<Output = ()> + Send + 'static>>) -> SpawnHandle;
}

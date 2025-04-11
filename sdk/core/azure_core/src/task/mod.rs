// Copyright (c) Microsoft Corporation. All rights reserved.
// Licensed under the MIT License.

//! Asynchronous task execution utilities.

use std::{fmt, future::Future, pin::Pin, sync::Arc};

#[cfg(all(not(feature = "tokio"), not(target_arch = "wasm32")))]
mod standard_spawn;

#[cfg(all(not(feature = "tokio"), target_arch = "wasm32"))]
mod wasm_spawn;

#[cfg(feature = "tokio")]
mod tokio_spawn;

#[cfg(test)]
mod tests;

#[cfg(all(not(feature = "tokio"), not(target_arch = "wasm32")))]
pub use standard_spawn::StdSpawner;

#[cfg(all(not(feature = "tokio"), target_arch = "wasm32"))]
pub use wasm_spawn::WasmSpawner;

#[cfg(feature = "tokio")]
pub use tokio_spawn::TokioSpawner;

#[cfg(not(target_arch = "wasm32"))]
pub(crate) type TaskFuture = Pin<Box<dyn Future<Output = ()> + Send + 'static>>;

#[cfg(target_arch = "wasm32")]
pub(crate) type TaskFuture = Pin<Box<dyn Future<Output = ()> + 'static>>;

/// Creates a new [`TaskSpawner`].
///
/// This returns a [`TaskSpawner`] that spawns a [`std::thread`] to run the task unless `tokio` was enabled,
/// in which case an executor is returned that calls [`tokio::task::spawn`].
pub fn new_task_spawner() -> Arc<dyn TaskSpawner> {
    #[cfg(feature = "tokio")]
    {
        Arc::new(TokioSpawner)
    }
    #[cfg(all(not(feature = "tokio"), not(target_arch = "wasm32")))]
    {
        Arc::new(StdSpawner)
    }
    #[cfg(all(not(feature = "tokio"), target_arch = "wasm32"))]
    {
        Arc::new(WasmSpawner)
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

#[cfg(all(not(feature = "tokio"), not(target_arch = "wasm32")))]
#[derive(Debug)]
pub struct SpawnHandle(std::thread::JoinHandle<()>);

#[cfg(all(not(feature = "tokio"), not(target_arch = "wasm32")))]
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

#[cfg(all(not(feature = "tokio"), target_arch = "wasm32"))]
#[derive(Debug)]
pub struct SpawnHandle();

#[cfg(all(not(feature = "tokio"), target_arch = "wasm32"))]
impl SpawnHandle {
    /// Wait for the task to complete and return the result.
    pub async fn await_result(self) -> crate::Result<()> {
        unimplemented!()
    }
}

/// An async command runner.
pub trait TaskSpawner: Send + Sync + fmt::Debug {
    /// Spawn a task that executes a given future and returns the output.
    fn spawn(&self, f: TaskFuture) -> SpawnHandle;
}

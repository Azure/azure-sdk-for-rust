// Copyright (c) Microsoft Corporation. All rights reserved.
// Licensed under the MIT License.

//! Asynchronous task execution utilities.

use async_trait::async_trait;
use std::{
    fmt::{self, Debug},
    future::Future,
    pin::Pin,
    sync::Arc,
};

#[cfg(all(not(feature = "tokio"), not(target_arch = "wasm32")))]
mod standard_spawn;

#[cfg(all(not(feature = "tokio"), target_arch = "wasm32"))]
mod wasm_spawn;

#[cfg(feature = "tokio")]
mod tokio_spawn;

#[cfg(test)]
mod tests;

#[cfg_attr(not(target_arch = "wasm32"), async_trait)]
#[cfg_attr(target_arch = "wasm32", async_trait(?Send))]
pub trait SpawnHandleMethods: Send + fmt::Debug {
    /// Wait for the task to complete and return the result.
    async fn wait(self) -> crate::Result<()>;
}

#[derive(Debug)]
pub struct SpawnHandleT<T>
where
    T: SpawnHandleMethods + 'static,
{
    pub(crate) inner: T,
}

#[cfg_attr(not(target_arch = "wasm32"), async_trait)]
#[cfg_attr(target_arch = "wasm32", async_trait(?Send))]
impl<T> SpawnHandleMethods for SpawnHandleT<T>
where
    T: SpawnHandleMethods + 'static,
{
    /// Wait for the task to complete and return the result.
    async fn wait(self) -> crate::Result<()> {
        self.inner.wait().await
    }
}

// A type alias for the spawn handle.
// This is used to abstract over the different spawn handle types used in different implementations.
#[cfg(all(not(feature = "tokio"), not(target_arch = "wasm32")))]
pub type SpawnHandle = SpawnHandleT<standard_spawn::StdSpawnHandle>;

#[cfg(all(not(feature = "tokio"), target_arch = "wasm32"))]
pub type SpawnHandle = SpawnHandleT<wasm_spawn::WasmSpawnHandle>;

#[cfg(feature = "tokio")]
pub type SpawnHandle = SpawnHandleT<tokio_spawn::TokioSpawnHandle>;

#[cfg(not(target_arch = "wasm32"))]
pub(crate) type TaskFuture = Pin<Box<dyn Future<Output = ()> + Send + 'static>>;

// WASM32 does not support `Send` futures, so we use a non-Send future type.
#[cfg(target_arch = "wasm32")]
pub(crate) type TaskFuture = Pin<Box<dyn Future<Output = ()> + 'static>>;

/// Creates a new [`TaskSpawner`].
///
/// This returns a [`TaskSpawner`] that runs a task asynchronously.
///
/// The implementation depends on the target architecture and the features enabled:
/// - If the `tokio` feature is enabled, it uses a tokio based spawner.
/// - If the `tokio` feature is not enabled and the target architecture is not `wasm32`, it uses a std::thread based spawner.
/// - If the `tokio` feature is not enabled and the target architecture is `wasm32`, it uses a wasm specific spawner.
///
pub fn new_task_spawner() -> Arc<dyn TaskSpawner> {
    #[cfg(feature = "tokio")]
    {
        Arc::new(tokio_spawn::TokioSpawner)
    }
    #[cfg(all(not(feature = "tokio"), not(target_arch = "wasm32")))]
    {
        Arc::new(standard_spawn::StdSpawner)
    }
    #[cfg(all(not(feature = "tokio"), target_arch = "wasm32"))]
    {
        Arc::new(wasm_spawn::WasmSpawner)
    }
}

/// An async command runner.
pub trait TaskSpawner: Send + Sync + fmt::Debug {
    /// Spawn a task that executes a given future and returns the output.
    fn spawn(&self, f: TaskFuture) -> SpawnHandle;
}

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
pub use standard_spawn::{SpawnHandle, StdSpawner};

#[cfg(all(not(feature = "tokio"), target_arch = "wasm32"))]
pub use wasm_spawn::{SpawnHandle, WasmSpawnHandle};

#[cfg(feature = "tokio")]
pub use tokio_spawn::{SpawnHandle, TokioSpawner};

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
/// - If the `tokio` feature is enabled, it uses [`TokioSpawner`].
/// - If the `tokio` feature is not enabled and the target architecture is not `wasm32`, it uses [`StdSpawner`].
/// - If the `tokio` feature is not enabled and the target architecture is `wasm32`, it uses [`WasmSpawner`].
///
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

/// An async command runner.
pub trait TaskSpawner: Send + Sync + fmt::Debug {
    /// Spawn a task that executes a given future and returns the output.
    fn spawn(&self, f: TaskFuture) -> SpawnHandle;
}

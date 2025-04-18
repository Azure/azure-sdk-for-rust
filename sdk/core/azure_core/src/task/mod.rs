// Copyright (c) Microsoft Corporation. All rights reserved.
// Licensed under the MIT License.

//! Asynchronous task execution utilities.
//!
//! This module provides a mechanism to spawn tasks asynchronously and wait for their completion.
//!
//! It abstracts away the underlying implementation details, allowing for different task execution strategies based on the target architecture and features enabled.
//!
//!
//! Example usage:
//!
//! ```
//! use azure_core::task::{new_task_spawner, TaskSpawner, SpawnHandleMethods};
//! use futures::FutureExt;
//!
//! #[tokio::main]
//! async fn main() {
//!     let spawner = new_task_spawner();
//!     let handle = spawner.spawn(async {
//!         // Simulate some work
//!         std::thread::sleep(std::time::Duration::from_secs(1));
//!     }.boxed());
//!
//!     handle.wait().await.expect("Task should complete successfully");
//!
//!     println!("Task completed");
//! }
//! ```
//!
//!
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

/// Common methods for all spawn handles.
///
/// This trait defines the common interface for all spawn handles.
/// It is implemented for different types of spawn handles depending on the target architecture and features.
///
#[cfg_attr(not(target_arch = "wasm32"), async_trait)]
#[cfg_attr(target_arch = "wasm32", async_trait(?Send))]
pub trait SpawnHandleMethods: Send + fmt::Debug {
    /// Wait for the task to complete and return the result.
    ///
    /// # Returns
    /// A `Result` indicating the success or failure of the task.
    ///
    async fn wait(self) -> crate::Result<()>;
}

/// A spawn handle that can be used to wait for a task to complete.
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

/// A type alias for the spawn handle when using Tokio.
///
/// A `SpawnHandle` is a handle to a spawned task, allowing you to wait for its completion.
#[cfg(all(not(feature = "tokio"), not(target_arch = "wasm32")))]
pub type SpawnHandle = SpawnHandleT<standard_spawn::StdSpawnHandle>;

/// A type alias for the spawn handle when using Tokio.
///
/// A `SpawnHandle` is a handle to a spawned task, allowing you to wait for its completion.
#[cfg(all(not(feature = "tokio"), target_arch = "wasm32"))]
pub type SpawnHandle = SpawnHandleT<wasm_spawn::WasmSpawnHandle>;

/// A type alias for the spawn handle when using Tokio.
///
/// A `SpawnHandle` is a handle to a spawned task, allowing you to wait for its completion.
#[cfg(feature = "tokio")]
pub type SpawnHandle = SpawnHandleT<tokio_spawn::TokioSpawnHandle>;

#[cfg(not(target_arch = "wasm32"))]
pub(crate) type TaskFuture = Pin<Box<dyn Future<Output = ()> + Send + 'static>>;

// WASM32 does not support `Send` futures, so we use a non-Send future type.
#[cfg(target_arch = "wasm32")]
pub(crate) type TaskFuture = Pin<Box<dyn Future<Output = ()> + 'static>>;

/// An async command runner.
pub trait TaskSpawner: Send + Sync + fmt::Debug {
    /// Spawn a task that executes a given future and returns the output.
    ///
    /// # Arguments
    ///
    /// * `f` - A future representing the task to be spawned. This future cannot capture any variables
    ///  from its environment by reference, as it will be executed in a different thread or context.
    fn spawn(&self, f: TaskFuture) -> SpawnHandle;
}

/// Creates a new [`TaskSpawner`] to enable running tasks asynchronously.
///
///
/// The implementation depends on the target architecture and the features enabled:
/// - If the `tokio` feature is enabled, it uses a tokio based spawner.
/// - If the `tokio` feature is not enabled and the target architecture is not `wasm32`, it uses a std::thread based spawner.
/// - If the `tokio` feature is not enabled and the target architecture is `wasm32`, it uses a wasm specific spawner.
///
/// # Returns
///  A new instance of a [`TaskSpawner`] which can be used to spawn background tasks.
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

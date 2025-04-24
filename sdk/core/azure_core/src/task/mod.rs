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
//! use azure_core::task::{new_task_spawner, TaskSpawner};
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
//!     handle.await.expect("Task should complete successfully");
//!
//!     println!("Task completed");
//! }
//! ```
//!
//!
use std::{fmt::Debug, future::Future, pin::Pin, sync::Arc};

mod standard_spawn;

#[cfg(feature = "tokio")]
mod tokio_spawn;

#[cfg(test)]
mod tests;

#[cfg(not(target_arch = "wasm32"))]
pub(crate) type TaskFuture = Pin<Box<dyn Future<Output = ()> + Send + 'static>>;

// WASM32 does not support `Send` futures, so we use a non-Send future type.
#[cfg(target_arch = "wasm32")]
pub(crate) type TaskFuture = Pin<Box<dyn Future<Output = ()> + 'static>>;

/// A `SpawnedTask` is a future that represents a running task.
/// It can be awaited to block until the task has completed.
#[cfg(not(target_arch = "wasm32"))]
pub type SpawnedTask = Pin<
    Box<
        dyn Future<Output = std::result::Result<(), Box<dyn std::error::Error + Send>>>
            + Send
            + 'static,
    >,
>;

#[cfg(target_arch = "wasm32")]
pub type SpawnedTask =
    Pin<Box<dyn Future<Output = std::result::Result<(), Box<dyn std::error::Error>>> + 'static>>;

/// An async command runner.
///
// Note that this trait cannot use *`async_trait`* because method implementations in an async_trait never directly return futures, and we want the `spawn` method to return a future that can be awaited.
pub trait TaskSpawner: Send + Sync + Debug {
    /// Spawn a task that executes a given future and returns the output.
    ///
    /// # Arguments
    ///
    /// * `f` - A future representing the task to be spawned. This future cannot capture any variables
    ///   from its environment by reference, as it will be executed in a different thread or context.
    ///
    /// # Returns
    /// A future which can be awaited to block until the task has completed.
    ///
    /// # Example
    /// ```
    /// use azure_core::task::{new_task_spawner, TaskSpawner};
    /// use futures::FutureExt;
    ///
    /// #[tokio::main]
    /// async fn main() {
    ///   let spawner = new_task_spawner();
    ///   let future = spawner.spawn(async {
    ///     // Simulate some work
    ///     std::thread::sleep(std::time::Duration::from_secs(1));
    ///   }.boxed());
    ///   future.await.expect("Task should complete successfully");
    /// }
    /// ```
    ///
    fn spawn(&self, f: TaskFuture) -> SpawnedTask;
}

/// Creates a new [`TaskSpawner`] to enable running tasks asynchronously.
///
///
/// The implementation depends on the target architecture and the features enabled:
/// - If the `tokio` feature is enabled, it uses a tokio based spawner.
/// - If the `tokio` feature is not enabled and the target architecture is not `wasm32`, it uses a std::thread based spawner.
///
/// # Returns
///  A new instance of a [`TaskSpawner`] which can be used to spawn background tasks.
///
/// # Example
///
/// ```
/// use azure_core::task::{new_task_spawner, TaskSpawner};
/// use futures::FutureExt;
///
/// #[tokio::main]
/// async fn main() {
///   let spawner = new_task_spawner();
///   let handle = spawner.spawn(async {
///     // Simulate some work
///     std::thread::sleep(std::time::Duration::from_secs(1));
///   }.boxed());
/// }
/// ```
///
pub fn new_task_spawner() -> Arc<dyn TaskSpawner> {
    #[cfg(not(feature = "tokio"))]
    {
        Arc::new(standard_spawn::StdSpawner)
    }
    #[cfg(feature = "tokio")]
    {
        Arc::new(tokio_spawn::TokioSpawner) as Arc<dyn TaskSpawner>
    }
}

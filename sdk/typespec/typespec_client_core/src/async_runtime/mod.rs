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
use std::{
    fmt::Debug,
    future::Future,
    pin::Pin,
    sync::{Arc, OnceLock},
};

mod standard_runtime;

#[cfg(feature = "tokio")]
mod tokio_runtime;

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

/// An Asynchronous Runtime.
///
/// This trait defines the various
///
pub trait AsyncRuntime: Send + Sync + Debug {
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
    /// use azure_core::async_runtime::{get_async_runtime, TaskSpawner};
    /// use futures::FutureExt;
    ///
    /// #[tokio::main]
    /// async fn main() {
    ///   let async_runtime = get_async_runtime();
    ///   let future = async_runtime.spawn(async {
    ///     // Simulate some work
    ///     std::thread::sleep(std::time::Duration::from_secs(1));
    ///   }.boxed());
    ///   future.await.expect("Task should complete successfully");
    /// }
    /// ```
    ///
    /// # Note
    ///
    /// This trait intentionally does not use the *`async_trait`* macro because when the
    /// `async_trait` attribute is applied to a trait  implementation, the rewritten
    /// method cannot directly return a future, instead they wrap the return value
    /// in a future, and we want the `spawn` method to directly return a future
    /// that can be awaited.
    ///
    fn spawn(&self, f: TaskFuture) -> SpawnedTask;

    fn sleep(&self, duration: std::time::Duration) -> TaskFuture;
}

static ASYNC_RUNTIME_IMPLEMENTATION: OnceLock<Arc<dyn AsyncRuntime>> = OnceLock::new();

/// Returns an [`AsyncRuntime`] to enable running operations which need to interact with an
/// asynchronous runtime.
///
///
/// The implementation depends on the target architecture and the features enabled:
/// - If the `tokio` feature is enabled, it uses a tokio based spawner and timer.
/// - If the `tokio` feature is not enabled and the target architecture is not `wasm32`, it uses a std::thread based spawner and timer.
///
/// # Returns
///  A new instance of a [`AsyncRuntime`] which can be used to spawn background tasks.
///
/// # Example
///
/// ```
/// use azure_core::get_async_runtime;
/// use futures::FutureExt;
///
/// #[tokio::main]
/// async fn main() {
///   let async_runtime = get_async_runtime();
///   let handle = async_runtime.spawn(async {
///     // Simulate some work
///     std::thread::sleep(std::time::Duration::from_secs(1));
///   }.boxed());
/// }
/// ```
///
pub fn get_async_runtime() -> Arc<dyn AsyncRuntime> {
    ASYNC_RUNTIME_IMPLEMENTATION
        .get_or_init(|| create_async_runtime())
        .clone()
}

/// Sets the current [`AsyncRuntime`] to enable running operations which need to interact with an
/// asynchronous runtime.
///
///
/// # Returns
///  Ok if the async runtime was set successfully, or an error if it has already been set.
///
/// # Example
///
/// ```
/// use azure_core::async_runtime::{get_async_runtime, AsyncRuntime};
/// use futures::FutureExt;
///
/// async fn main() {
///   let async_runtime = set_async_runtime();
///   let handle = async_runtime.spawn(async {
///     // Simulate some work
///     std::thread::sleep(std::time::Duration::from_secs(1));
///   }.boxed());
/// }
/// ```
///
pub fn set_async_runtime(runtime: Arc<dyn AsyncRuntime>) -> crate::Result<()> {
    let result = ASYNC_RUNTIME_IMPLEMENTATION.set(runtime);
    if result.is_err() {
        Err(crate::Error::message(
            crate::error::ErrorKind::Other,
            "Async runtime has already been set.",
        ))
    } else {
        Ok(())
    }
}

fn create_async_runtime() -> Arc<dyn AsyncRuntime> {
    #[cfg(not(feature = "tokio"))]
    {
        Arc::new(standard_runtime::StdRuntime)
    }
    #[cfg(feature = "tokio")]
    {
        Arc::new(tokio_runtime::TokioRuntime) as Arc<dyn AsyncRuntime>
    }
}

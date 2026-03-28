// Copyright (c) Microsoft Corporation. All rights reserved.
// Licensed under the MIT License.

//! Asynchronous task execution utilities.
//!
//! This module provides a mechanism to spawn tasks asynchronously and wait for their completion.
//!
//! It abstracts away the underlying implementation details, allowing for different task execution strategies based on the target architecture and features enabled.
//!
//! # Examples
//!
//! ```
//! use typespec_client_core::async_runtime::get_async_runtime;
//!
//! # #[tokio::main]
//! # async fn main() {
//! let async_runtime = get_async_runtime();
//! let handle = async_runtime.spawn(Box::pin(async {
//!     // Simulate some work
//!     std::thread::sleep(std::time::Duration::from_secs(1));
//! }));
//! handle.await.expect("Task should complete successfully");
//! println!("Task completed");
//! # }
//! ```
use crate::time::Duration;
use std::{
    future::Future,
    pin::Pin,
    sync::{Arc, OnceLock},
};

#[cfg_attr(feature = "tokio", allow(dead_code))]
mod standard_runtime;

#[cfg(feature = "tokio")]
mod tokio_runtime;

#[cfg(test)]
mod tests;

/// A `TaskFuture` is a boxed future that represents a task that can be spawned and executed asynchronously.
pub type TaskFuture = Pin<Box<dyn Future<Output = ()> + Send + 'static>>;

/// A pinned, boxed [`AbortableTask`].
///
/// Returned by [`AsyncRuntime::spawn`]. Await it to wait for the task to
/// complete, or call [`AbortableTask::abort`] to request cancellation.
pub type SpawnedTask = Pin<Box<dyn AbortableTask>>;

/// A future that represents a running task which can be cancelled.
///
/// Awaiting an `AbortableTask` blocks until the task completes and yields
/// `Ok(())` on success or an error if the task panicked or otherwise failed.
///
/// Call [`abort`](AbortableTask::abort) to request cancellation. The exact
/// semantics are runtime-dependent:
///
/// * **Tokio** — calls [`JoinHandle::abort`](tokio::task::JoinHandle::abort),
///   which cancels the task at the next `.await` point.
/// * **std thread** — drops the join handle and marks the task as finished so
///   that awaiting the future resolves immediately. The underlying thread may
///   continue running, but the caller is no longer blocked on it.
pub trait AbortableTask:
    Future<Output = std::result::Result<(), Box<dyn std::error::Error + Send>>> + Send
{
    /// Requests cancellation of the task.
    ///
    /// After calling `abort`, awaiting this future will resolve without waiting
    /// for the spawned work to finish. Calling `abort` on an already-completed
    /// task is a no-op.
    fn abort(&self);
}

/// An asynchronous runtime.
///
/// This trait abstracts task spawning, sleeping, and yielding so that library
/// code can remain runtime-agnostic.
pub trait AsyncRuntime: Send + Sync {
    /// Spawn a task that executes a given future and returns the output.
    ///
    /// # Arguments
    ///
    /// * `f` - A future representing the task to be spawned. This future cannot capture any variables
    ///   from its environment by reference, as it will be executed in a different thread or context.
    ///
    /// # Returns
    ///
    /// A future which can be awaited to block until the task has completed.
    ///
    /// # Examples
    ///
    /// ```
    /// use typespec_client_core::async_runtime::get_async_runtime;
    ///
    /// # #[tokio::main]
    /// # async fn main() {
    /// let async_runtime = get_async_runtime();
    /// let handle = async_runtime.spawn(Box::pin(async {
    ///   // Simulate some work
    ///   std::thread::sleep(std::time::Duration::from_secs(1));
    /// }));
    /// handle.await.expect("Task should complete successfully");
    /// # }
    /// ```
    ///
    /// # Notes
    ///
    /// This trait intentionally does not use the *`async_trait`* macro because when the
    /// `async_trait` attribute is applied to a trait  implementation, the rewritten
    /// method cannot directly return a future, instead they wrap the return value
    /// in a future, and we want the `spawn` method to directly return a future
    /// that can be awaited.
    fn spawn(&self, f: TaskFuture) -> SpawnedTask;

    /// Sleep for the specified duration asynchronously.
    ///
    /// # Arguments
    /// * `duration` - The duration to sleep for.
    /// # Returns
    /// A future that resolves after the specified duration has elapsed.
    fn sleep(&self, duration: Duration) -> TaskFuture;

    /// Yield the current task back to the runtime scheduler.
    fn yield_now(&self) -> TaskFuture;
}

static ASYNC_RUNTIME_IMPLEMENTATION: OnceLock<Arc<dyn AsyncRuntime>> = OnceLock::new();

/// Returns an [`AsyncRuntime`] to enable running operations which need to interact with an
/// asynchronous runtime.
///
///
/// The implementation depends on the target architecture and the features enabled:
/// - If the `tokio` feature is enabled, it uses a tokio based spawner and timer.
/// - If the `tokio` feature is not enabled, it uses a std::thread based spawner and timer.
///
/// # Returns
///  An instance of a [`AsyncRuntime`] which can be used to spawn background tasks or perform other asynchronous operations.
///
/// # Examples
///
/// ```
/// use typespec_client_core::async_runtime::get_async_runtime;
///
/// # #[tokio::main]
/// # async fn main() {
/// let async_runtime = get_async_runtime();
/// let handle = async_runtime.spawn(Box::pin(async {
///   // Simulate some work
///   std::thread::sleep(std::time::Duration::from_secs(1));
/// }));
/// # }
/// ```
pub fn get_async_runtime() -> Arc<dyn AsyncRuntime> {
    ASYNC_RUNTIME_IMPLEMENTATION
        .get_or_init(|| create_async_runtime())
        .clone()
}

/// Sets the current [`AsyncRuntime`] to enable running operations which need to interact with an
/// asynchronous runtime.
///
/// # Arguments
/// * `runtime` - An instance of a type that implements the [`AsyncRuntime`] trait.
///
/// # Returns
///  Ok if the async runtime was set successfully, or an error if it has already been set.
///
/// # Examples
///
/// ```
/// use typespec_client_core::async_runtime::{
///     set_async_runtime, AbortableTask, AsyncRuntime, TaskFuture, SpawnedTask};
/// use std::sync::Arc;
///
/// struct CustomRuntime;
///
/// impl AsyncRuntime for CustomRuntime {
///    fn spawn(&self, _f: TaskFuture) -> SpawnedTask {
///      unimplemented!("Custom spawn not implemented");
///    }
///    fn sleep(&self, _duration: typespec_client_core::time::Duration) -> TaskFuture {
///      unimplemented!("Custom sleep not implemented");
///    }
///    fn yield_now(&self) -> TaskFuture {
///      unimplemented!("Custom yield not implemented");
///    }
///  }
///
///  set_async_runtime(Arc::new(CustomRuntime)).expect("Failed to set async runtime");
/// ```
///
pub fn set_async_runtime(runtime: Arc<dyn AsyncRuntime>) -> crate::Result<()> {
    let result = ASYNC_RUNTIME_IMPLEMENTATION.set(runtime);
    if result.is_err() {
        Err(crate::Error::with_message(
            crate::error::ErrorKind::Other,
            "Async runtime has already been set.",
        ))
    } else {
        Ok(())
    }
}

fn create_async_runtime() -> Arc<dyn AsyncRuntime> {
    #[cfg(feature = "tokio")]
    {
        Arc::new(tokio_runtime::TokioRuntime) as Arc<dyn AsyncRuntime>
    }
    #[cfg(not(feature = "tokio"))]
    {
        Arc::new(standard_runtime::StdRuntime) as Arc<dyn AsyncRuntime>
    }
}

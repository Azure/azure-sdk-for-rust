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
use cfg_if::cfg_if;
use std::{
    future::Future,
    pin::Pin,
    sync::{Arc, OnceLock},
};

#[cfg_attr(any(feature = "tokio", feature = "wasm_bindgen", feature = "spin"), allow(dead_code))]
mod standard_runtime;

#[cfg(feature = "tokio")]
mod tokio_runtime;

#[cfg(all(target_arch = "wasm32", feature = "wasm_bindgen"))]
mod web_runtime;

#[cfg(test)]
mod tests;

#[cfg(not(target_arch = "wasm32"))]
pub type TaskFuture = Pin<Box<dyn Future<Output = ()> + Send + 'static>>;

// WASM32 does not support `Send` futures, so we use a non-Send future type.
#[cfg(target_arch = "wasm32")]
pub type TaskFuture = Pin<Box<dyn Future<Output = ()> + 'static>>;

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

    fn sleep(&self, duration: Duration) -> TaskFuture;
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
///     set_async_runtime, AsyncRuntime, TaskFuture, SpawnedTask};
/// use std::sync::Arc;
/// use futures::FutureExt;
///
/// struct CustomRuntime;
///
/// impl AsyncRuntime for CustomRuntime {
///    fn spawn(&self, f: TaskFuture) -> SpawnedTask {
///      unimplemented!("Custom spawn not implemented");
///    }
///    fn sleep(&self, duration: typespec_client_core::time::Duration) -> TaskFuture {
///      unimplemented!("Custom sleep not implemented");
///    }
///  }
///
///  set_async_runtime(Arc::new(CustomRuntime)).expect("Failed to set async runtime");
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
    cfg_if! {
        if #[cfg(all(target_arch = "wasm32", target_os = "wasi", feature = "spin"))] {
            Arc::new(spin_runtime::SpinRuntime) as Arc<dyn AsyncRuntime>
        } else if #[cfg(all(target_arch = "wasm32", feature = "wasm_bindgen"))] {
            Arc::new(web_runtime::WasmBindgenRuntime) as Arc<dyn AsyncRuntime>
        } else if #[cfg(feature = "tokio")] {
            Arc::new(tokio_runtime::TokioRuntime) as Arc<dyn AsyncRuntime>
        } else {
            Arc::new(standard_runtime::StdRuntime) as Arc<dyn AsyncRuntime>
        }
    }
}

#[cfg(all(target_arch = "wasm32", target_os = "wasi", feature = "spin"))]
mod spin_runtime {
    use super::{AsyncRuntime, SpawnedTask, TaskFuture};
    use crate::time::Duration;
    // Only reference spin_executor when the feature is on and we're in WASI.

    pub(crate) struct SpinRuntime;

    impl AsyncRuntime for SpinRuntime {
        fn spawn(&self, f: TaskFuture) -> SpawnedTask {
            // Spin executor runs the future to completion; wrap in a future to fit the trait.
            // In wasm, SpawnedTask is non-Send, which matches spin environments.
            Box::pin(async move {
                spin_executor::run(async move { f.await });
                Ok(())
            })
        }

        fn sleep(&self, duration: Duration) -> TaskFuture {
            // No direct sleep in spin-executor; use gloo-timers if available isn't guaranteed here.
            // Implement a simple timer via spin_executor by awaiting a future that yields after duration using web timers is not available.
            // Fallback: busy-waiting is not acceptable; instead, use a single-shot spin sleep via spin's timer once available.
            // For now, provide a no-op sleep to keep compatibility.
            Box::pin(async move {
                // TODO: consider integrating a WASI clock once stabilized.
                let _ = duration;
            })
        }
    }
}

// Copyright (c) Microsoft Corporation. All rights reserved.
// Licensed under the MIT License.

//! Cooperative cancellation manager for background tasks.
//!
//! The core SDK's [`AsyncRuntime::spawn`] returns a [`SpawnedTask`] (a boxed
//! future) rather than a join handle with an `abort()` method. This means we
//! cannot forcibly cancel a spawned task from the outside. Instead,
//! [`BackgroundTaskManager`] uses a shared [`AtomicBool`] shutdown flag that
//! background tasks should check periodically and exit when it becomes `true`.
//!
//! When the `BackgroundTaskManager` is dropped (e.g. when the owning client is
//! disposed), the flag is set automatically, causing all cooperating tasks to
//! exit on their next check.
//!
//! # Future improvement
//!
//! Once the core SDK exposes an abort-capable handle type (analogous to
//! `tokio::task::JoinHandle` / `tokio::task::JoinSet`), this module should be
//! updated to use that mechanism for immediate cancellation.

use azure_core::async_runtime::{get_async_runtime, TaskFuture};
use std::sync::atomic::{AtomicBool, Ordering};
use std::sync::Arc;
use tracing::debug;

/// Manages the lifecycle of background tasks spawned by a client.
///
/// Tasks spawned through this manager receive a shared cancellation token
/// ([`AtomicBool`]) that they should check periodically. When the manager is
/// dropped, the token is set to `true`, signaling all tasks to exit gracefully.
#[derive(Debug)]
pub(crate) struct BackgroundTaskManager {
    /// Shared shutdown flag. `true` means "please stop".
    shutdown: Arc<AtomicBool>,
}

impl BackgroundTaskManager {
    /// Creates a new [`BackgroundTaskManager`] with the shutdown flag initially
    /// set to `false`.
    pub fn new() -> Self {
        Self {
            shutdown: Arc::new(AtomicBool::new(false)),
        }
    }

    /// Returns a clone of the shared shutdown token.
    ///
    /// Background tasks should call `token.load(Ordering::SeqCst)` at the top
    /// of every loop iteration and return early when it is `true`.
    pub fn cancellation_token(&self) -> Arc<AtomicBool> {
        Arc::clone(&self.shutdown)
    }

    /// Returns `true` if the shutdown signal has been set.
    #[allow(dead_code)]
    pub fn is_shutdown(&self) -> bool {
        self.shutdown.load(Ordering::SeqCst)
    }

    /// Spawns a fire-and-forget background task on the async runtime.
    ///
    /// The returned [`SpawnedTask`] future is intentionally dropped — the task
    /// runs independently until it completes or observes the shutdown signal.
    pub fn spawn(&self, future: TaskFuture) {
        drop(get_async_runtime().spawn(future));
    }
}

impl Drop for BackgroundTaskManager {
    fn drop(&mut self) {
        debug!("BackgroundTaskManager: signaling shutdown to all background tasks.");
        self.shutdown.store(true, Ordering::SeqCst);
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn new_manager_is_not_shutdown() {
        let manager = BackgroundTaskManager::new();
        assert!(!manager.is_shutdown());
    }

    #[test]
    fn cancellation_token_reflects_shutdown() {
        let manager = BackgroundTaskManager::new();
        let token = manager.cancellation_token();
        assert!(!token.load(Ordering::SeqCst));

        // Simulate drop
        drop(manager);

        assert!(token.load(Ordering::SeqCst));
    }

    #[test]
    fn drop_signals_shutdown() {
        let token;
        {
            let manager = BackgroundTaskManager::new();
            token = manager.cancellation_token();
            assert!(!token.load(Ordering::SeqCst));
        }
        // Manager has been dropped
        assert!(token.load(Ordering::SeqCst));
    }

    #[test]
    fn multiple_tokens_all_observe_shutdown() {
        let manager = BackgroundTaskManager::new();
        let t1 = manager.cancellation_token();
        let t2 = manager.cancellation_token();
        let t3 = manager.cancellation_token();

        assert!(!t1.load(Ordering::SeqCst));
        assert!(!t2.load(Ordering::SeqCst));
        assert!(!t3.load(Ordering::SeqCst));

        drop(manager);

        assert!(t1.load(Ordering::SeqCst));
        assert!(t2.load(Ordering::SeqCst));
        assert!(t3.load(Ordering::SeqCst));
    }
}

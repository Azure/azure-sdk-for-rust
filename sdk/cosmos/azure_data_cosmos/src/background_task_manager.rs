// Copyright (c) Microsoft Corporation. All rights reserved.
// Licensed under the MIT License.

//! Manages background tasks spawned by a client.
//!
//! [`BackgroundTaskManager`] holds on to the [`SpawnedTask`] futures returned
//! by [`AsyncRuntime::spawn`]. Dropping the manager drops all stored futures,
//! which cancels the tasks — `drop(future)` is how Rust communicates
//! cancellation to futures.

use azure_core::async_runtime::{get_async_runtime, SpawnedTask, TaskFuture};
use std::sync::Mutex;
use tracing::debug;

/// Manages the lifecycle of background tasks spawned by a client.
///
/// Spawned tasks are kept alive by storing their [`SpawnedTask`] futures.
/// When the manager is dropped, all stored futures are dropped, cancelling the
/// associated tasks.
pub(crate) struct BackgroundTaskManager {
    /// Stored spawned task futures. Dropping these cancels the tasks.
    /// Uses a [`Mutex`] for interior mutability so that [`spawn`] can accept
    /// `&self`, which is required when the manager lives inside an `Arc`.
    tasks: Mutex<Vec<SpawnedTask>>,
}

impl std::fmt::Debug for BackgroundTaskManager {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let count = self.tasks.lock().map(|t| t.len()).unwrap_or(0);
        f.debug_struct("BackgroundTaskManager")
            .field("tasks_count", &count)
            .finish()
    }
}

impl BackgroundTaskManager {
    /// Creates a new [`BackgroundTaskManager`] with no active tasks.
    pub fn new() -> Self {
        Self {
            tasks: Mutex::new(Vec::new()),
        }
    }

    /// Spawns a background task on the async runtime and stores the handle.
    ///
    /// The task will remain alive as long as this manager is alive. When the
    /// manager is dropped, all stored futures are dropped, cancelling the tasks.
    pub fn spawn(&self, future: TaskFuture) {
        let spawned = get_async_runtime().spawn(future);
        self.tasks
            .lock()
            .expect("BackgroundTaskManager mutex poisoned")
            .push(spawned);
    }
}

impl Drop for BackgroundTaskManager {
    fn drop(&mut self) {
        let count = self.tasks.get_mut().map(|t| t.len()).unwrap_or(0);
        debug!(
            "BackgroundTaskManager: dropping {} background task(s).",
            count,
        );
        // Dropping the Vec<SpawnedTask> drops all futures, cancelling the tasks.
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn new_manager_has_no_tasks() {
        let manager = BackgroundTaskManager::new();
        assert_eq!(manager.tasks.lock().unwrap().len(), 0);
    }

    #[tokio::test]
    async fn drop_cleans_up_tasks() {
        let manager = BackgroundTaskManager::new();
        // Spawn a no-op task
        manager.spawn(Box::pin(async {}));
        assert_eq!(manager.tasks.lock().unwrap().len(), 1);
        drop(manager);
        // Manager dropped successfully — tasks are cancelled
    }
}

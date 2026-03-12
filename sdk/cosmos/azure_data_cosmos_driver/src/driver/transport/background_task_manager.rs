// Copyright (c) Microsoft Corporation. All rights reserved.
// Licensed under the MIT License.

//! Manages background tasks spawned by a client.
//!
//! [`BackgroundTaskManager`] holds on to [`tokio::task::JoinHandle`]s returned
//! by [`tokio::spawn`]. Dropping the manager aborts all stored tasks — unlike
//! raw `JoinHandle` (which detaches on drop), this manager calls
//! [`JoinHandle::abort`] to cancel each task.

use std::future::Future;
use std::sync::Mutex;
use tracing::debug;

/// Manages the lifecycle of background tasks spawned on the tokio runtime.
///
/// Spawned tasks are kept alive by storing their [`tokio::task::JoinHandle`]s.
/// When the manager is dropped, all handles are aborted, cancelling the
/// associated tasks (tokio `JoinHandle`s detach on drop rather than cancel,
/// so explicit abort is required).
pub(crate) struct BackgroundTaskManager {
    /// Stored task handles. Aborting these cancels the tasks.
    /// Uses a [`Mutex`] for interior mutability so that [`spawn`](Self::spawn)
    /// can accept `&self`, which is required when the manager lives inside an
    /// `Arc`.
    tasks: Mutex<Vec<tokio::task::JoinHandle<()>>>,
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

    /// Spawns a background task on the tokio runtime and stores the handle.
    ///
    /// The task will remain alive as long as this manager is alive. When the
    /// manager is dropped, all stored handles are aborted, cancelling the
    /// tasks.
    pub fn spawn<F>(&self, future: F)
    where
        F: Future<Output = ()> + Send + 'static,
    {
        let handle = tokio::spawn(future);
        self.tasks
            .lock()
            .expect("BackgroundTaskManager mutex poisoned")
            .push(handle);
    }
}

impl Drop for BackgroundTaskManager {
    fn drop(&mut self) {
        let tasks = self
            .tasks
            .get_mut()
            .expect("BackgroundTaskManager mutex poisoned");
        let count = tasks.len();
        debug!(
            "BackgroundTaskManager: aborting {} background task(s).",
            count,
        );
        for handle in tasks.drain(..) {
            handle.abort();
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::sync::atomic::{AtomicBool, AtomicU32, Ordering};
    use std::sync::Arc;
    use tokio::time::{sleep, Duration};

    #[test]
    fn new_manager_has_no_tasks() {
        let manager = BackgroundTaskManager::new();
        assert_eq!(manager.tasks.lock().unwrap().len(), 0);
    }

    #[tokio::test]
    async fn drop_cleans_up_tasks() {
        let manager = BackgroundTaskManager::new();
        // Spawn a no-op task
        manager.spawn(async {});
        assert_eq!(manager.tasks.lock().unwrap().len(), 1);
        drop(manager);
        // Manager dropped successfully — tasks are aborted
    }

    #[tokio::test]
    async fn task_runs_to_completion() {
        let counter = Arc::new(AtomicU32::new(0));
        let done = Arc::new(AtomicBool::new(false));

        let manager = BackgroundTaskManager::new();
        {
            let counter = Arc::clone(&counter);
            let done = Arc::clone(&done);
            manager.spawn(async move {
                for _ in 0..5 {
                    counter.fetch_add(1, Ordering::SeqCst);
                    sleep(Duration::from_millis(10)).await;
                }
                done.store(true, Ordering::SeqCst);
            });
        }

        // Wait enough time for the task to finish
        sleep(Duration::from_millis(200)).await;

        assert_eq!(counter.load(Ordering::SeqCst), 5);
        assert!(done.load(Ordering::SeqCst));
    }

    #[tokio::test]
    async fn drop_aborts_running_task() {
        let counter = Arc::new(AtomicU32::new(0));
        let running = Arc::new(AtomicBool::new(false));

        let manager = BackgroundTaskManager::new();
        {
            let counter = Arc::clone(&counter);
            let running = Arc::clone(&running);
            // Task increments counter every 10ms for ~500ms total
            manager.spawn(async move {
                running.store(true, Ordering::SeqCst);
                for _ in 0..50 {
                    sleep(Duration::from_millis(10)).await;
                    counter.fetch_add(1, Ordering::SeqCst);
                }
                running.store(false, Ordering::SeqCst);
            });
        }

        // Let the task run for ~50ms (should complete ~5 increments)
        sleep(Duration::from_millis(50)).await;
        assert!(running.load(Ordering::SeqCst));
        let count_before_drop = counter.load(Ordering::SeqCst);
        assert!(count_before_drop > 0, "task should have made progress");

        // Drop the manager — this aborts the task
        drop(manager);

        // Give a little time for the abort to propagate
        sleep(Duration::from_millis(50)).await;

        let count_after_drop = counter.load(Ordering::SeqCst);
        // The counter should have stopped (or at most advanced by 1 due to
        // a tick in flight at the moment of abort).
        assert!(
            count_after_drop <= count_before_drop + 1,
            "task should have stopped; before={count_before_drop}, after={count_after_drop}"
        );
        // The task should NOT have completed all 50 iterations
        assert!(
            count_after_drop < 50,
            "task should have been aborted before finishing; count={count_after_drop}"
        );
    }
}

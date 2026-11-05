// Copyright (c) Microsoft Corporation. All rights reserved.
// Licensed under the MIT License.

use super::{AsyncRuntime, SpawnedTask, TaskFuture};
use crate::async_runtime::AbortableTask;
use crate::time::Duration;
use futures::{executor::LocalPool, task::SpawnExt};
use std::error::Error;
use std::{
    future,
    sync::{Arc, Condvar, Mutex, MutexGuard, PoisonError},
    task::{Context, Poll, Waker},
    thread,
};
use std::{future::Future, pin::Pin};
use tracing::debug;

/// A future that completes when a thread join handle completes.
struct ThreadJoinFuture {
    join_state: Arc<Mutex<ThreadJoinState>>,
}

#[derive(Default)]
struct ThreadJoinState {
    join_handle:
        Option<thread::JoinHandle<std::result::Result<(), Box<dyn std::error::Error + Send>>>>,
    waker: Option<Waker>,
    thread_finished: bool,
}

impl AbortableTask for ThreadJoinFuture {
    fn abort(&self) {
        // We cannot actually abort the thread, but we can drop the join handle
        // to avoid blocking on it when the future is dropped.
        if let Ok(mut join_state) = self.join_state.lock() {
            join_state.thread_finished = true;
            join_state.join_handle = None;
        }
    }
}

impl Future for ThreadJoinFuture {
    type Output = std::result::Result<(), Box<dyn std::error::Error + Send>>;

    fn poll(self: Pin<&mut Self>, cx: &mut Context<'_>) -> Poll<Self::Output> {
        let mut join_state = self.join_state.lock().map_err(|e| {
            debug!("Failed to lock join state: {}", e);
            Box::new(crate::Error::with_message(
                crate::error::ErrorKind::Other,
                format!("Thread panicked: {:?}", e),
            )) as Box<dyn std::error::Error + Send>
        })?;

        // Join handle is present, so we can check if the thread has finished
        // and take the handle if it has.
        // This is safe because we are holding the lock on the join state.
        // We can safely take the handle and join it without blocking.
        // This allows us to retrieve the terminal state of the thread.
        if join_state.thread_finished {
            // Thread is finished, so we can safely take the handle
            let Some(join_handle) = join_state.join_handle.take() else {
                // The join handle was already removed from the state, we know we're done.
                return Poll::Ready(Ok(()));
            };

            // Since we know the thread is finished, we can safely take the handle
            // and join it. This allows us to retrieve the terminal state of the thread.
            //
            // Technically this might block (because the `thread_finished` flag
            // is set before the thread *actually* finishes), but it should be negligible.
            match join_handle.join() {
                Ok(_) => Poll::Ready(Ok(())),
                Err(e) => Poll::Ready(Err(Box::new(crate::Error::with_message(
                    crate::error::ErrorKind::Other,
                    format!("Thread panicked: {:?}", e),
                )) as Box<dyn std::error::Error + Send>)),
            }
        } else {
            // Thread is still running, so we need to register the waker
            // for when it completes.
            join_state.waker = Some(cx.waker().clone());
            Poll::Pending
        }
    }
}

struct ThreadJoinError {
    error: Option<Box<dyn Error + Send>>,
}

impl Future for ThreadJoinError {
    type Output = std::result::Result<(), Box<dyn std::error::Error + Send>>;

    fn poll(mut self: Pin<&mut Self>, _cx: &mut Context<'_>) -> Poll<Self::Output> {
        Poll::Ready(Err(self.error.take().unwrap()))
    }
}

impl AbortableTask for ThreadJoinError {
    fn abort(&self) {
        // No-op since the thread never started
    }
}

/// An [`AsyncRuntime`] using [`std::thread::spawn`].
pub(crate) struct StdRuntime;

impl AsyncRuntime for StdRuntime {
    fn spawn(&self, f: TaskFuture) -> SpawnedTask {
        let join_state = Arc::new(Mutex::new(ThreadJoinState::default()));
        {
            let Ok(mut js) = join_state.lock() else {
                // Set thread_finished to true and return a ThreadJoinFuture that will immediately return the error
                let error = Box::new(crate::Error::with_message(
                    crate::error::ErrorKind::Other,
                    "Thread panicked.",
                )) as Box<dyn std::error::Error + Send>;

                return Box::pin(ThreadJoinError { error: Some(error) });
            };

            // Clone the join state so it can be moved into the thread
            // and used to notify the waker when the thread finishes.
            let join_state_clone = join_state.clone();

            js.join_handle = Some(thread::spawn(move || {
                // Create a local executor
                let mut local_pool = LocalPool::new();
                let spawner = local_pool.spawner();

                // Spawn the future on the local executor
                let Ok(future_handle) = spawner.spawn_with_handle(f) else {
                    return Err(Box::new(crate::Error::with_message(
                        crate::error::ErrorKind::Other,
                        "Failed to spawn future.",
                    )) as Box<dyn std::error::Error + Send>);
                };
                // Drive the executor until the future completes
                local_pool.run_until(future_handle);

                let Ok(mut join_state) = join_state_clone.lock() else {
                    return Err(Box::new(crate::Error::with_message(
                        crate::error::ErrorKind::Other,
                        "Failed to lock join state",
                    )) as Box<dyn std::error::Error + Send>);
                };

                // The thread has finished, so we can take the waker
                // and notify it.
                join_state.thread_finished = true;
                if let Some(waker) = join_state.waker.take() {
                    waker.wake();
                }
                Ok(())
            }));
        }
        // Create a future that will complete when the thread joins
        let join_future = ThreadJoinFuture { join_state };
        Box::pin(join_future)
    }

    /// Creates a future that resolves after a specified duration of time.
    ///
    /// Uses a simple thread based implementation for sleep. A more efficient
    /// implementation is available by using the `tokio` crate feature.
    fn sleep(&self, duration: Duration) -> TaskFuture {
        Box::pin(Sleep {
            state: None,
            duration,
        })
    }

    fn yield_now(&self) -> TaskFuture {
        std::thread::yield_now();
        Box::pin(future::ready(()))
    }
}

#[derive(Debug)]
struct Sleep {
    state: Option<Arc<SleepState>>,
    duration: Duration,
}

#[derive(Debug, Default)]
struct SleepState {
    inner: Mutex<SleepInner>,
    condvar: Condvar,
    #[cfg(test)]
    thread_started: std::sync::atomic::AtomicBool,
    #[cfg(test)]
    thread_exited: std::sync::atomic::AtomicBool,
}

#[derive(Debug, Default)]
struct SleepInner {
    completed: bool,
    canceled: bool,
    waker: Option<Waker>,
}

impl SleepState {
    /// Locks the shared state, recovering from a poisoned mutex.
    ///
    /// A panic while the lock is held cannot leave the state inconsistent,
    /// and failing to lock would either hang the future or leak the thread.
    fn lock(&self) -> MutexGuard<'_, SleepInner> {
        self.inner.lock().unwrap_or_else(PoisonError::into_inner)
    }
}

impl Future for Sleep {
    type Output = ();

    fn poll(self: Pin<&mut Self>, cx: &mut Context<'_>) -> Poll<Self::Output> {
        if let Some(state) = &self.state {
            // Holding the lock while checking and registering the waker
            // guarantees the worker either sees the current waker or has
            // already marked the sleep completed.
            let mut inner = state.lock();
            if inner.completed {
                return Poll::Ready(());
            }
            inner.waker = Some(cx.waker().clone());
            return Poll::Pending;
        }

        let state = Arc::new(SleepState::default());
        state.lock().waker = Some(cx.waker().clone());

        let duration = self.duration;
        self.get_mut().state = Some(state.clone());
        thread::spawn(move || {
            #[cfg(test)]
            state.thread_started.store(true, Ordering::SeqCst);

            #[cfg(test)]
            let _thread_exit_guard = SleepThreadExitGuard {
                state: state.clone(),
            };

            let duration = duration.try_into().unwrap_or(std::time::Duration::ZERO);
            let (mut inner, _) = state
                .condvar
                .wait_timeout_while(state.lock(), duration, |inner| !inner.canceled)
                .unwrap_or_else(PoisonError::into_inner);

            inner.completed = true;
            let waker = inner.waker.take();
            drop(inner);

            if let Some(waker) = waker {
                waker.wake();
            }
        });
        Poll::Pending
    }
}

impl Drop for Sleep {
    fn drop(&mut self) {
        if let Some(state) = &self.state {
            state.lock().canceled = true;
            state.condvar.notify_one();
        }
    }
}

#[cfg(test)]
use std::sync::atomic::{AtomicUsize, Ordering};

#[cfg(test)]
struct SleepThreadExitGuard {
    state: Arc<SleepState>,
}

#[cfg(test)]
impl Drop for SleepThreadExitGuard {
    fn drop(&mut self) {
        self.state.thread_exited.store(true, Ordering::SeqCst);
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use futures::task::{waker, ArcWake};
    use std::time::{Duration as StdDuration, Instant};

    #[derive(Default)]
    struct CountingWaker {
        wake_count: AtomicUsize,
    }

    impl ArcWake for CountingWaker {
        fn wake_by_ref(arc_self: &Arc<Self>) {
            arc_self.wake_count.fetch_add(1, Ordering::SeqCst);
        }
    }

    fn wait_until(mut predicate: impl FnMut() -> bool) {
        let deadline = Instant::now() + StdDuration::from_secs(1);
        while !predicate() && Instant::now() < deadline {
            std::thread::sleep(StdDuration::from_millis(1));
        }
        assert!(predicate());
    }

    #[test]
    fn dropped_sleep_cancels_thread() {
        let mut sleep = Box::pin(Sleep {
            state: None,
            duration: Duration::seconds(30),
        });
        let waker = futures::task::noop_waker();
        let mut cx = Context::from_waker(&waker);
        assert!(matches!(sleep.as_mut().poll(&mut cx), Poll::Pending));

        let state = sleep.state.as_ref().unwrap().clone();
        wait_until(|| state.thread_started.load(Ordering::SeqCst));

        drop(sleep);

        wait_until(|| state.thread_exited.load(Ordering::SeqCst));
    }

    #[test]
    fn completed_sleep_wakes_latest_waker() {
        let duration = StdDuration::from_millis(50);
        let mut sleep = Box::pin(Sleep {
            state: None,
            duration: duration.try_into().unwrap(),
        });
        let first = Arc::new(CountingWaker::default());
        let first_waker = waker(first.clone());
        let mut first_cx = Context::from_waker(&first_waker);
        let started = Instant::now();
        assert!(matches!(sleep.as_mut().poll(&mut first_cx), Poll::Pending));

        let state = sleep.state.as_ref().unwrap().clone();
        let latest = Arc::new(CountingWaker::default());
        let latest_waker = waker(latest.clone());
        let mut latest_cx = Context::from_waker(&latest_waker);
        assert!(matches!(sleep.as_mut().poll(&mut latest_cx), Poll::Pending));

        wait_until(|| latest.wake_count.load(Ordering::SeqCst) > 0);

        assert_eq!(first.wake_count.load(Ordering::SeqCst), 0);
        assert!(started.elapsed() >= duration);
        assert!(matches!(
            sleep.as_mut().poll(&mut latest_cx),
            Poll::Ready(())
        ));
        wait_until(|| state.thread_exited.load(Ordering::SeqCst));
    }
}

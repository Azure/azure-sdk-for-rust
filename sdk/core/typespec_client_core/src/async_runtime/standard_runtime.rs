// Copyright (c) Microsoft Corporation. All rights reserved.
// Licensed under the MIT License.

use super::{AsyncRuntime, SpawnedTask, TaskFuture};
use crate::async_runtime::AbortableTask;
use crate::time::Duration;
use futures::{executor::LocalPool, task::SpawnExt};
use std::error::Error;
use std::sync::atomic::{AtomicBool, Ordering};
use std::{
    future,
    sync::{Arc, Condvar, Mutex},
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
    completed: AtomicBool,
    canceled: AtomicBool,
    thread_waker: Mutex<Option<Waker>>,
    wait_lock: Mutex<()>,
    wait_condvar: Condvar,
}

impl Future for Sleep {
    type Output = ();

    fn poll(self: Pin<&mut Self>, cx: &mut Context<'_>) -> Poll<Self::Output> {
        if let Some(state) = &self.state {
            if state.completed.load(Ordering::Acquire) {
                Poll::Ready(())
            } else {
                if let Ok(mut waker) = state.thread_waker.lock() {
                    *waker = Some(cx.waker().clone());
                }
                Poll::Pending
            }
        } else {
            let state = Arc::new(SleepState::default());
            if let Ok(mut waker) = state.thread_waker.lock() {
                *waker = Some(cx.waker().clone());
            }
            let duration = self.duration;
            self.get_mut().state = Some(state.clone());
            thread::spawn(move || {
                #[cfg(test)]
                sleep_thread_started_for_test();

                #[cfg(test)]
                let _thread_count_guard = SleepThreadCounterGuard;

                let wait_result = state.wait_lock.lock().ok().and_then(|wait_guard| {
                    state
                        .wait_condvar
                        .wait_timeout_while(
                            wait_guard,
                            duration.try_into().expect("Duration conversion failed"),
                            |_| !state.canceled.load(Ordering::Acquire),
                        )
                        .ok()
                });

                // If the mutex was poisoned we still complete the sleep to avoid hanging.
                let _ = wait_result;

                state.completed.store(true, Ordering::Release);
                if let Ok(mut waker) = state.thread_waker.lock() {
                    if let Some(waker) = waker.take() {
                        waker.wake();
                    }
                }
            });
            Poll::Pending
        }
    }
}

impl Drop for Sleep {
    fn drop(&mut self) {
        if let Some(state) = &self.state {
            state.canceled.store(true, Ordering::Release);
            state.wait_condvar.notify_one();
        }
    }
}

#[cfg(test)]
static ACTIVE_SLEEP_THREADS: std::sync::atomic::AtomicUsize =
    std::sync::atomic::AtomicUsize::new(0);

#[cfg(test)]
fn sleep_thread_started_for_test() {
    ACTIVE_SLEEP_THREADS.fetch_add(1, Ordering::SeqCst);
}

#[cfg(test)]
struct SleepThreadCounterGuard;

#[cfg(test)]
impl Drop for SleepThreadCounterGuard {
    fn drop(&mut self) {
        ACTIVE_SLEEP_THREADS.fetch_sub(1, Ordering::SeqCst);
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::time::{Duration as StdDuration, Instant};

    #[test]
    fn dropped_sleep_cancels_thread() {
        let runtime = StdRuntime;
        let baseline_threads = ACTIVE_SLEEP_THREADS.load(Ordering::SeqCst);
        let mut sleep = runtime.sleep(Duration::seconds(30));
        let waker = futures::task::noop_waker();
        let mut cx = Context::from_waker(&waker);
        assert!(matches!(sleep.as_mut().poll(&mut cx), Poll::Pending));

        let wait_for_spawn = Instant::now() + StdDuration::from_secs(1);
        while ACTIVE_SLEEP_THREADS.load(Ordering::SeqCst) <= baseline_threads
            && Instant::now() < wait_for_spawn
        {
            std::thread::sleep(StdDuration::from_millis(1));
        }
        assert!(ACTIVE_SLEEP_THREADS.load(Ordering::SeqCst) > baseline_threads);

        drop(sleep);

        let wait_for_exit = Instant::now() + StdDuration::from_secs(1);
        while ACTIVE_SLEEP_THREADS.load(Ordering::SeqCst) > baseline_threads
            && Instant::now() < wait_for_exit
        {
            std::thread::sleep(StdDuration::from_millis(1));
        }
        assert_eq!(
            ACTIVE_SLEEP_THREADS.load(Ordering::SeqCst),
            baseline_threads
        );
    }
}

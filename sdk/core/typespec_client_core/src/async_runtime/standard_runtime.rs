// Copyright (c) Microsoft Corporation. All rights reserved.
// Licensed under the MIT License.

use super::{AsyncRuntime, SpawnedTask, TaskFuture};
use crate::time::Duration;
use futures::{executor::LocalPool, task::SpawnExt};
use std::sync::atomic::{AtomicBool, Ordering};
use std::{
    future,
    sync::{Arc, Mutex},
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

/// An [`AsyncRuntime`] using [`std::thread::spawn`].
pub(crate) struct StdRuntime;

impl AsyncRuntime for StdRuntime {
    fn spawn(&self, f: TaskFuture) -> SpawnedTask {
        let join_state = Arc::new(Mutex::new(ThreadJoinState::default()));
        {
            let Ok(mut js) = join_state.lock() else {
                return Box::pin(future::ready(Err(Box::new(crate::Error::with_message(
                    crate::error::ErrorKind::Other,
                    "Thread panicked.",
                ))
                    as Box<dyn std::error::Error + Send>)));
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
            signal: None,
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
    signal: Option<Arc<AtomicBool>>,
    duration: Duration,
}

impl Future for Sleep {
    type Output = ();

    fn poll(self: Pin<&mut Self>, cx: &mut Context<'_>) -> Poll<Self::Output> {
        if let Some(signal) = &self.signal {
            if signal.load(Ordering::Acquire) {
                Poll::Ready(())
            } else {
                Poll::Pending
            }
        } else {
            let signal = Arc::new(AtomicBool::new(false));
            let waker = cx.waker().clone();
            let duration = self.duration;
            self.get_mut().signal = Some(signal.clone());
            thread::spawn(move || {
                thread::sleep(duration.try_into().expect("Duration conversion failed"));
                signal.store(true, Ordering::Release);
                waker.wake();
            });
            Poll::Pending
        }
    }
}

// Copyright (c) Microsoft Corporation. All rights reserved.
// Licensed under the MIT License.

use super::{SpawnedTask, TaskFuture, TaskSpawner};
#[cfg(not(target_arch = "wasm32"))]
use futures::executor::LocalPool;
#[cfg(not(target_arch = "wasm32"))]
use futures::task::SpawnExt;
use std::future;
#[cfg(not(target_arch = "wasm32"))]
use std::future::Future;
#[cfg(not(target_arch = "wasm32"))]
use std::pin::Pin;
#[cfg(not(target_arch = "wasm32"))]
use std::sync::{Arc, Mutex};
#[cfg(not(target_arch = "wasm32"))]
use std::task::Waker;
#[cfg(not(target_arch = "wasm32"))]
use std::task::{Context, Poll};
#[cfg(not(target_arch = "wasm32"))]
use std::thread;
use tracing::debug;

/// A future that completes when a thread join handle completes.
#[cfg(not(target_arch = "wasm32"))]
struct ThreadJoinFuture {
    join_state: Arc<Mutex<ThreadJoinState>>,
}

#[cfg(not(target_arch = "wasm32"))]
#[derive(Default)]
struct ThreadJoinState {
    join_handle:
        Option<thread::JoinHandle<std::result::Result<(), Box<dyn std::error::Error + Send>>>>,
    waker: Option<Waker>,
    thread_finished: bool,
}

#[cfg(not(target_arch = "wasm32"))]
impl Future for ThreadJoinFuture {
    type Output = std::result::Result<(), Box<dyn std::error::Error + Send>>;

    fn poll(self: Pin<&mut Self>, _cx: &mut Context<'_>) -> Poll<Self::Output> {
        let mut join_state = self.join_state.lock().map_err(|e| {
            debug!("Failed to lock join state: {}", e);
            Box::new(crate::Error::message(
                crate::error::ErrorKind::Other,
                format!("Thread panicked: {:?}", e),
            )) as Box<dyn std::error::Error + Send>
        })?;

        if join_state.join_handle.is_some() {
            // Join handle is present, so we can check if the thread has finished
            // and take the handle if it has.
            // This is safe because we are holding the lock on the join state.
            // We can safely take the handle and join it without blocking.
            // This allows us to retrieve the terminal state of the thread.
            //        if let Some(handle) = &join_state.join_handle {
            //            if handle.is_finished() {
            if join_state.thread_finished {
                // Thread is finished, so we can safely take the handle
                // Since we know the thread is finished, we can safely take the handle
                // and join it without blocking. This allows us to retrieve the terminal state of the thread.
                match join_state.join_handle.take().unwrap().join() {
                    Ok(_) => Poll::Ready(Ok(())),
                    Err(e) => Poll::Ready(Err(Box::new(crate::Error::message(
                        crate::error::ErrorKind::Other,
                        format!("Thread panicked: {:?}", e),
                    ))
                        as Box<dyn std::error::Error + Send>)),
                }
            } else {
                // Thread is still running, so we need to register the waker
                // for when it completes.
                join_state.waker = Some(_cx.waker().clone());
                Poll::Pending
            }
        } else {
            // Handle has been taken, so we've already completed
            Poll::Ready(Ok(()))
        }
    }
}

/// A [`TaskSpawner`] using [`std::thread::spawn`].
#[derive(Debug)]
pub struct StdSpawner;

//#[cfg_attr(not(target_arch = "wasm32"), async_trait)]
//#[cfg_attr(target_arch = "wasm32", async_trait(?Send))]
impl TaskSpawner for StdSpawner {
    #[cfg_attr(target_arch = "wasm32", allow(unused_variables))]
    fn spawn(&self, f: TaskFuture) -> SpawnedTask {
        #[cfg(target_arch = "wasm32")]
        {
            panic!("std::thread::spawn is not supported on wasm32")
        }
        #[cfg(not(target_arch = "wasm32"))]
        {
            let join_state = Arc::new(Mutex::new(ThreadJoinState::default()));
            {
                let js = join_state.lock();
                match js {
                    Ok(mut js) => {
                        let join_state_clone = join_state.clone();
                        js.join_handle = Some(thread::spawn(move || {
                            // Create a local executor
                            let mut local_pool = LocalPool::new();
                            let spawner = local_pool.spawner();

                            // Spawn the future on the local executor
                            let spawn_result = spawner.spawn_with_handle(f);
                            match spawn_result {
                                Err(err) => Err(Box::new(crate::Error::message(
                                    crate::error::ErrorKind::Other,
                                    format!("Failed to spawn future: {}", err),
                                ))
                                    as Box<dyn std::error::Error + Send>),
                                Ok(future_handle) => {
                                    // Drive the executor until the future completes
                                    local_pool.run_until(future_handle);

                                    let join_state = join_state_clone.lock();
                                    let Ok(mut join_state) = join_state else {
                                        return Err(Box::new(crate::Error::message(
                                            crate::error::ErrorKind::Other,
                                            "Failed to lock join state",
                                        ))
                                            as Box<dyn std::error::Error + Send>);
                                    };
                                    // The thread has finished, so we can take the waker
                                    // and notify it.
                                    join_state.thread_finished = true;
                                    if let Some(waker) = join_state.waker.take() {
                                        waker.wake();
                                    }
                                    Ok(())
                                }
                            }
                        }));
                    }
                    Err(err) => {
                        return Box::pin(future::ready(Err(Box::new(crate::Error::message(
                            crate::error::ErrorKind::Other,
                            format!("Thread panicked: {}", err),
                        ))
                            as Box<dyn std::error::Error + Send>)));
                    }
                }
            }
            // Create a future that will complete when the thread joins
            let join_future = ThreadJoinFuture { join_state };
            Box::pin(join_future)
        }
    }
}

// Copyright (c) Microsoft Corporation. All rights reserved.
// Licensed under the MIT License.

use super::{SpawnedTask, TaskFuture, TaskSpawner};
#[cfg(not(target_arch = "wasm32"))]
use futures::executor::LocalPool;
#[cfg(not(target_arch = "wasm32"))]
use futures::task::SpawnExt;
#[cfg(not(target_arch = "wasm32"))]
use futures::FutureExt;
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

/// A future that completes when a thread join handle completes.
#[cfg(not(target_arch = "wasm32"))]
struct ThreadJoinFuture {
    join_state: Arc<Mutex<ThreadJoinState>>,
}

#[cfg(not(target_arch = "wasm32"))]
#[derive(Default)]
struct ThreadJoinState {
    join_handle: Option<thread::JoinHandle<()>>,
    waker: Option<Waker>,
}

#[cfg(not(target_arch = "wasm32"))]
impl Future for ThreadJoinFuture {
    type Output = std::result::Result<(), Box<dyn std::error::Error>>;

    fn poll(self: Pin<&mut Self>, _cx: &mut Context<'_>) -> Poll<Self::Output> {
        // We need to check if the thread is done
        // Since there's no non-blocking way to check thread completion in std,
        // we attempt to join only once during polling
        let mut join_state = self
            .join_state
            .lock()
            .map_err(|e| format!("Failed to lock join state: {:?}", e))?;
        if let Some(handle) = &join_state.join_handle {
            if handle.is_finished() {
                // Since we know the thread is finished, we can safely take the handle
                // and join it without blocking.
                match join_state.join_handle.take().unwrap().join() {
                    Ok(_) => Poll::Ready(Ok(())),
                    Err(e) => Poll::Ready(Err(format!("Thread panicked: {:?}", e).into())),
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
                let join_state_clone = join_state.clone();
                let js = join_state
                    .lock()
                    .map_err(|e| format!("Failed to lock join state: {:?}", e));
                let Ok(mut js) = js else {
                    return Box::pin(async { Err("Failed to lock join state".into()) });
                };
                js.join_handle = Some(thread::spawn(move || {
                    // Create a local executor
                    let mut local_pool = LocalPool::new();
                    let spawner = local_pool.spawner();

                    // Spawn the future on the local executor
                    let future_handle = spawner
                        .spawn_with_handle(f)
                        .expect("Failed to spawn future");

                    // Drive the executor until the future completes
                    local_pool.run_until(future_handle);

                    let mut join_state = join_state_clone.lock().unwrap();
                    // Notify the waker that the thread has completed
                    if let Some(waker) = join_state.waker.take() {
                        waker.wake();
                    }
                }));
            }
            // Create a future that will complete when the thread joins
            let join_future = ThreadJoinFuture { join_state };
            join_future.boxed()
        }
    }
}

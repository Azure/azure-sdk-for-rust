// Copyright (c) Microsoft Corporation. All rights reserved.
// Licensed under the MIT License.

use super::{SpawnHandle, TaskFuture, TaskSpawner};
use async_trait::async_trait;
#[cfg(not(target_arch = "wasm32"))]
use futures::executor::LocalPool;
#[cfg(not(target_arch = "wasm32"))]
use futures::task::SpawnExt;
#[cfg(not(target_arch = "wasm32"))]
use std::thread;

/// A [`TaskSpawner`] using [`std::thread::spawn`].
#[derive(Debug)]
pub struct StdSpawner;

impl TaskSpawner for StdSpawner {
    #[cfg_attr(target_arch = "wasm32", allow(unused_variables))]
    fn spawn(&self, f: TaskFuture) -> Box<dyn SpawnHandle> {
        #[cfg(target_arch = "wasm32")]
        {
            panic!("std::thread::spawn is not supported on wasm32")
        }
        #[cfg(not(target_arch = "wasm32"))]
        {
            let th = thread::spawn(move || {
                // Create a local executor
                let mut local_pool = LocalPool::new();
                let spawner = local_pool.spawner();

                // Spawn the future on the local executor
                let future_handle = spawner
                    .spawn_with_handle(f)
                    .expect("Failed to spawn future");

                // Drive the executor until the future completes
                local_pool.run_until(future_handle);
            });

            // Return a handle that will await the result
            Box::new(StdSpawnHandle(th))
        }
    }
}

#[derive(Debug)]
pub struct StdSpawnHandle(std::thread::JoinHandle<()>);

#[cfg_attr(not(target_arch = "wasm32"), async_trait)]
#[cfg_attr(target_arch = "wasm32", async_trait(?Send))]
impl SpawnHandle for StdSpawnHandle {
    /// Wait for the task to complete and return the result.
    async fn wait(self: Box<Self>) -> crate::Result<()> {
        self.0.join().map_err(|e| {
            crate::Error::message(
                crate::error::ErrorKind::Other,
                format!("Task was cancelled before completion: {:?}", e),
            )
        })
    }
}

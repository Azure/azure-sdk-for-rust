// Copyright (c) Microsoft Corporation. All rights reserved.
// Licensed under the MIT License.

use super::{SpawnHandle, TaskFuture, TaskSpawner};
use futures::executor::LocalPool;
use futures::task::SpawnExt;
use std::thread;

/// A [`TaskSpawner`] using [`std::thread::spawn`].
#[derive(Debug)]
pub struct StdSpawner;

impl TaskSpawner for StdSpawner {
    fn spawn(&self, f: TaskFuture) -> SpawnHandle {
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
        SpawnHandle::Std(StdSpawnHandle(th))
    }
}

#[derive(Debug)]
pub struct StdSpawnHandle(std::thread::JoinHandle<()>);

impl StdSpawnHandle {
    /// Wait for the task to complete and return the result.
    pub(crate) async fn wait(self) -> crate::Result<()> {
        self.0.join().map_err(|_| {
            crate::Error::message(
                crate::error::ErrorKind::Other,
                "Task was cancelled before completion.",
            )
        })
    }
}

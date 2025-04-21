// Copyright (c) Microsoft Corporation. All rights reserved.
// Licensed under the MIT License.

use super::{SpawnHandle, TaskFuture, TaskSpawner};

/// A [`TaskSpawner`] using wasm.
#[derive(Debug)]
pub struct WasmSpawner;

impl TaskSpawner for WasmSpawner {
    fn spawn(&self, _f: TaskFuture) -> SpawnHandle {
        unimplemented!()
    }
}

#[derive(Debug)]
pub struct WasmSpawnHandle();

impl WasmSpawnHandle {
    /// Wait for the task to complete and return the result.
    pub(crate) async fn wait(self) -> crate::Result<()> {
        unimplemented!()
    }
}

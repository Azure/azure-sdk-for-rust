// Copyright (c) Microsoft Corporation. All rights reserved.
// Licensed under the MIT License.

use super::{SpawnHandleMethods, SpawnHandleT, TaskFuture, TaskSpawner};
use async_trait::async_trait;

/// A [`TaskSpawner`] using wasm.
#[derive(Debug)]
pub struct WasmSpawner;

impl TaskSpawner for WasmSpawner {
    fn spawn(&self, _f: TaskFuture) -> SpawnHandleT<WasmSpawnHandle> {
        unimplemented!()
    }
}

#[derive(Debug)]
pub struct WasmSpawnHandle();

#[async_trait(?Send)]
impl SpawnHandleMethods for WasmSpawnHandle {
    /// Wait for the task to complete and return the result.
    async fn wait(self) -> crate::Result<()> {
        unimplemented!()
    }
}

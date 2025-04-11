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

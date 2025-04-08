// Copyright (c) Microsoft Corporation. All rights reserved.
// Licensed under the MIT License.

use super::{SpawnHandle, TaskSpawner};
use std::future::Future;

/// A [`TaskSpawner`] using [`tokio::spawn`].
#[derive(Debug)]
pub struct TokioSpawner;

impl TaskSpawner for TokioSpawner {
    fn spawn_boxed(&self, f: Box<dyn Future<Output = ()> + Send + 'static>) -> SpawnHandle {
        let handle = ::tokio::spawn(Box::into_pin(f));
        SpawnHandle(handle)
    }
}

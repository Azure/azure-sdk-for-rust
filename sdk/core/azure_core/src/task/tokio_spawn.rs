// Copyright (c) Microsoft Corporation. All rights reserved.
// Licensed under the MIT License.

use super::{SpawnHandle, TaskSpawner};
use std::{future::Future, pin::Pin};

/// A [`TaskSpawner`] using [`tokio::spawn`].
#[derive(Debug)]
pub struct TokioSpawner;

impl TaskSpawner for TokioSpawner {
    fn spawn(&self, f: Pin<Box<dyn Future<Output = ()> + Send>>) -> SpawnHandle {
        let handle = ::tokio::spawn(f);
        SpawnHandle(handle)
    }
}

// Copyright (c) Microsoft Corporation. All rights reserved.
// Licensed under the MIT License.

use super::{AsyncRuntime, SpawnedTask, TaskFuture};
use crate::time::Duration;

/// An [`AsyncRuntime`] using `tokio` based APIs.
pub(crate) struct WebRuntime;

impl AsyncRuntime for WebRuntime {
    fn spawn(&self, f: TaskFuture) -> SpawnedTask {
        Box::pin(async {
            wasm_bindgen_futures::spawn_local(f);
            Ok(())
        })
    }

    fn sleep(&self, duration: Duration) -> TaskFuture {
        Box::pin(async move {
            if let Ok(d) = duration.try_into() {
                gloo_timers::future::sleep(d).await;
            } else {
                // This means the duration is negative, don't sleep at all.
                return;
            }
        })
    }
}

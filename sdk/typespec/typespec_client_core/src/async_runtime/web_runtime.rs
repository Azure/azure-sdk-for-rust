// Copyright (c) Microsoft Corporation. All rights reserved.
// Licensed under the MIT License.

use super::{AsyncRuntime, SpawnedTask, TaskFuture};
use crate::time::Duration;
use futures::channel::oneshot;

/// An [`AsyncRuntime`] using `tokio` based APIs.
pub(crate) struct WasmBindgenRuntime;

impl AsyncRuntime for WasmBindgenRuntime {
    fn spawn(&self, f: TaskFuture) -> SpawnedTask {
        let (tx, rx) = oneshot::channel();

        wasm_bindgen_futures::spawn_local(async move {
            let result = f.await;
            let _ = tx.send(result);
        });

        Box::pin(async {
            rx.await
                .map_err(|e| Box::new(e) as Box<dyn std::error::Error>)
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

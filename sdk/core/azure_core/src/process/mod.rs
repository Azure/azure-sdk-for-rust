// Copyright (c) Microsoft Corporation. All rights reserved.
// Licensed under the MIT License.

use async_trait::async_trait;
use std::{ffi::OsStr, fmt, io, process::Output, sync::Arc};

mod standard;
#[cfg(feature = "tokio_process")]
mod tokio;

pub use standard::StdExecutor;
#[cfg(feature = "tokio_process")]
pub use tokio::TokioExecutor;

/// Creates a new [`Executor`].
///
/// This returns a [`StdExecutor`] that spawns a [`std::process::Command`] in a separate thread unless `tokio_process` was enabled,
/// in which case an executor is returned that spawns a `tokio::process::Command`.
pub fn new_executor() -> Arc<dyn Executor> {
    #[cfg(not(feature = "tokio_process"))]
    {
        Arc::new(StdExecutor)
    }
    #[cfg(feature = "tokio_process")]
    {
        Arc::new(TokioExecutor)
    }
}

/// An async command runner.
#[cfg_attr(target_arch = "wasm32", async_trait(?Send))]
#[cfg_attr(not(target_arch = "wasm32"), async_trait)]
pub trait Executor: Send + Sync + fmt::Debug {
    /// Run a program with the given arguments until it terminates, returning the output.
    async fn run(&self, program: &OsStr, args: &[&OsStr]) -> io::Result<Output>;
}

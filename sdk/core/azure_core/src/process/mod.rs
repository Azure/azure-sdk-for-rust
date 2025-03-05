// Copyright (c) Microsoft Corporation. All rights reserved.
// Licensed under the MIT License.

use async_trait::async_trait;
use std::io::Error;
use std::{ffi::OsStr, process::Output};

#[cfg(not(feature = "tokio_process"))]
mod thread;

#[cfg(not(feature = "tokio_process"))]
pub use thread::ThreadRunner;

#[cfg(feature = "tokio_process")]
pub use tokio::TokioRunner;

#[cfg(feature = "tokio_process")]
mod tokio;

/// Obtains a new process runner
pub fn new_process_runner() -> Box<dyn ProcessRunner> {
    #[cfg(feature = "tokio_process")]
    {
        Box::new(TokioRunner)
    }
    #[cfg(not(feature = "tokio_process"))]
    {
        Box::new(ThreadRunner)
    }
}

/// An abstraction to run processes
#[cfg_attr(target_arch = "wasm32", async_trait(?Send))]
#[cfg_attr(not(target_arch = "wasm32"), async_trait)]
pub trait ProcessRunner: Send + Sync + std::fmt::Debug {
    /// Run a command with the given arguments until it terminates, returning the output
    async fn run_command(&self, program: &OsStr, args: &[&OsStr]) -> Result<Output, Error>;
}

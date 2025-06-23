// Copyright (c) Microsoft Corporation. All rights reserved.
// Licensed under the MIT License.

use super::Executor;
use async_trait::async_trait;
use std::{ffi::OsStr, io, process::Output};

/// An [`Executor`] using [`tokio::process::Command`].
#[derive(Debug)]
pub struct TokioExecutor;

#[cfg_attr(target_arch = "wasm32", async_trait(?Send))]
#[cfg_attr(not(target_arch = "wasm32"), async_trait)]
impl Executor for TokioExecutor {
    async fn run(&self, program: &OsStr, args: &[&OsStr]) -> io::Result<Output> {
        ::tokio::process::Command::new(program)
            .args(args)
            .output()
            .await
    }
}

// Copyright (c) Microsoft Corporation. All rights reserved.
// Licensed under the MIT License.

use super::ProcessRunner;
use async_trait::async_trait;
use std::io::Error;
use std::{ffi::OsStr, process::Output};

/// Implement [`ProcessRunner`] via [`::tokio::process`]
#[derive(Debug)]
pub struct TokioRunner;

#[cfg_attr(target_arch = "wasm32", async_trait(?Send))]
#[cfg_attr(not(target_arch = "wasm32"), async_trait)]
impl ProcessRunner for TokioRunner {
    async fn run_command(&self, program: &OsStr, args: &[&OsStr]) -> Result<Output, Error> {
        let mut cmd = ::tokio::process::Command::new(program);
        cmd.args(args);
        cmd.output().await
    }
}

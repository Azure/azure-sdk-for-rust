// Copyright (c) Microsoft Corporation. All rights reserved.
// Licensed under the MIT License.

use super::ProcessRunner;
use async_trait::async_trait;
use futures::channel::oneshot;
use std::io::Error;
use std::io::ErrorKind;
use std::{ffi::OsStr, process::Output};

/// Implement [`ProcessRunner`] via a thread and [`std::process`]
#[derive(Debug)]
pub struct ThreadRunner;

#[cfg_attr(target_arch = "wasm32", async_trait(?Send))]
#[cfg_attr(not(target_arch = "wasm32"), async_trait)]
impl ProcessRunner for ThreadRunner {
    async fn run_command(&self, program: &OsStr, args: &[&OsStr]) -> Result<Output, Error> {
        let (tx, rx) = oneshot::channel();
        let mut cmd = std::process::Command::new(program);
        cmd.args(args);
        std::thread::spawn(move || {
            let output = cmd.output();
            tx.send(output)
        });
        let output = rx
            .await
            .map_err(|err| Error::new(ErrorKind::Other, err))??;
        Ok(output)
    }
}

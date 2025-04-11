// Copyright (c) Microsoft Corporation. All rights reserved.
// Licensed under the MIT License.

use super::Executor;
use async_trait::async_trait;
use futures::channel::oneshot;
use std::{ffi::OsStr, io, process::Output, thread};

/// An [`Executor`] using [`std::process::Command`] from [`std::thread::spawn()`].
#[derive(Debug)]
pub struct StdExecutor;

#[cfg_attr(target_arch = "wasm32", async_trait(?Send))]
#[cfg_attr(not(target_arch = "wasm32"), async_trait)]
impl Executor for StdExecutor {
    async fn run(&self, program: &OsStr, args: &[&OsStr]) -> io::Result<Output> {
        let (tx, rx) = oneshot::channel();
        let mut cmd = std::process::Command::new(program);
        cmd.args(args);
        thread::spawn(move || {
            let output = cmd.output();
            tx.send(output)
        });
        let output = rx.await.map_err(io::Error::other)??;
        Ok(output)
    }
}

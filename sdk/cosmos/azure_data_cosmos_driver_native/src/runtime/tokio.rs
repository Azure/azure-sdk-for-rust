// Copyright (c) Microsoft Corporation. All rights reserved.
// Licensed under the MIT License.

//! Tokio-backed runtime context.

use std::sync::Arc;

use azure_data_cosmos_driver::CosmosDriverRuntime;
use tokio::runtime::{Builder, Runtime};

use crate::error::{CosmosErrorCode, Error};
use crate::runtime::RuntimeOptions;

/// Owns the Tokio runtime and a shared driver runtime.
///
/// Reads/writes are thread-safe; multiple `CallContext`s on different threads
/// may share a single `RuntimeContext` provided they each have their own
/// `CallContext`.
pub struct RuntimeContext {
    tokio: Runtime,
    driver_runtime: Arc<CosmosDriverRuntime>,
}

impl RuntimeContext {
    pub fn new(options: RuntimeOptions) -> Result<Self, Error> {
        let mut builder = Builder::new_multi_thread();
        builder.enable_all().thread_name("cosmos-driver-runtime");
        if options.worker_threads > 0 {
            builder.worker_threads(options.worker_threads as usize);
        }
        let tokio = builder.build().map_err(|e| {
            Error::with_detail(
                CosmosErrorCode::InternalError,
                c"Failed to build Tokio runtime",
                e,
            )
        })?;

        // Build the driver runtime on the tokio executor (build() is async).
        let driver_runtime = tokio
            .block_on(CosmosDriverRuntime::builder().build())
            .map_err(|e| {
                Error::with_detail(
                    CosmosErrorCode::InternalError,
                    c"Failed to build CosmosDriverRuntime",
                    e,
                )
            })?;

        Ok(Self {
            tokio,
            driver_runtime,
        })
    }

    /// Runs an async future to completion on the Tokio runtime.
    pub fn block_on<F: std::future::Future>(&self, fut: F) -> F::Output {
        self.tokio.block_on(fut)
    }

    /// Returns a clone of the shared driver runtime handle.
    pub fn driver_runtime(&self) -> Arc<CosmosDriverRuntime> {
        Arc::clone(&self.driver_runtime)
    }
}

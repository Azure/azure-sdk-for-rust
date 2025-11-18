// Copyright (c) Microsoft Corporation. All rights reserved.
// Licensed under the MIT License.

use tokio::runtime::{Builder, Runtime};

use crate::{
    error::{CosmosErrorCode, Error},
    runtime::RuntimeOptions,
};

/// Provides a RuntimeContext (see [`crate::runtime`]) implementation using the Tokio runtime.
pub struct RuntimeContext {
    runtime: Runtime,
}

impl RuntimeContext {
    pub fn new(_options: Option<&RuntimeOptions>) -> Result<Self, Error> {
        #[cfg(target_family = "wasm")]
        let runtime = Builder::new_current_thread()
            .enable_all()
            .thread_name("cosmos-sdk-runtime")
            .build()
            .map_err(|e| {
                Error::with_detail(
                    CosmosErrorCode::UnknownError,
                    c"Unknown error initializing Cosmos SDK runtime",
                    e,
                )
            })?;
        #[cfg(not(target_family = "wasm"))]
        let runtime = Builder::new_multi_thread()
            .enable_all()
            .thread_name("cosmos-sdk-runtime")
            .build()
            .map_err(|e| {
                Error::with_detail(
                    CosmosErrorCode::UnknownError,
                    c"Unknown error initializing Cosmos SDK runtime",
                    e,
                )
            })?;
        Ok(Self { runtime })
    }
}

impl RuntimeContext {
    pub fn block_on<F, R>(&self, future: F) -> R
    where
        F: std::future::Future<Output = R>,
    {
        self.runtime.block_on(async {
            let _span = tracing::trace_span!("block_on").entered();
            tracing::trace!("entered async runtime");
            let r = future.await;
            tracing::trace!("leaving async runtime");
            r
        })
    }
}

use tokio::runtime::{Builder, Runtime};

use crate::error::{CosmosErrorCode, Error};

/// Provides a RuntimeContext (see [`crate::runtime`]) implementation using the Tokio runtime.
pub struct RuntimeContext {
    runtime: Runtime,
}

impl RuntimeContext {
    pub fn new() -> Result<Self, Error> {
        let runtime = Builder::new_multi_thread()
            .enable_all()
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
        self.runtime.block_on(future)
    }
}

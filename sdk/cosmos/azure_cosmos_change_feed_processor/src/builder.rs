// Copyright (c) Microsoft Corporation. All rights reserved.
// Licensed under the MIT License.

use azure_core::http::StatusCode;
use azure_data_cosmos_driver::{CosmosError, CosmosErrorBuilder, CosmosStatus};
use futures::future::{select, Either};
use std::time::{Duration, Instant};

use crate::{
    container_binding::{invalid, PreparedContainer},
    ChangeFeedProcessor, ContainerBinding, ProcessorEngine,
};

/// Prepares two independent credential-bound container contexts without starting work.
///
/// Required group/feed/lease arguments are supplied to [`build()`](Self::build).
/// There is no processor-level credential, implicit default identity, callback,
/// bootstrap write, lease acquisition, or background processor task in build.
pub struct ChangeFeedProcessorBuilder {
    preparation_timeout: Duration,
}
impl Default for ChangeFeedProcessorBuilder {
    fn default() -> Self {
        Self {
            preparation_timeout: Duration::from_secs(60),
        }
    }
}
impl ChangeFeedProcessorBuilder {
    /// Sets the overall preparation budget for both bindings, independently of request defaults.
    pub fn with_preparation_timeout(mut self, timeout: Duration) -> Self {
        self.preparation_timeout = timeout;
        self
    }
    /// Returns the overall preparation timeout.
    pub fn preparation_timeout(&self) -> Duration {
        self.preparation_timeout
    }

    /// Validates configuration and prepares both independently credential-bound containers.
    ///
    /// Feed preparation runs before lease preparation. Each receives a fresh
    /// runtime/cache namespace and retains its account-bound driver. Metadata
    /// resolution may use caches and does not establish change-feed/write permission.
    /// This method performs no dummy authorization writes and starts no callbacks.
    ///
    /// # Errors
    ///
    /// Returns configuration, authentication, transport, or metadata errors with
    /// feed/lease preparation context and preserved Cosmos status/diagnostics.
    pub async fn build(
        self,
        group: impl Into<String>,
        feed: ContainerBinding,
        leases: ContainerBinding,
    ) -> azure_core::Result<ChangeFeedProcessor> {
        let group = group.into();
        if group.trim().is_empty()
            || self.preparation_timeout.is_zero()
            || self.preparation_timeout > Duration::from_secs(300)
        {
            return Err(invalid("processor preparation requires a group and a positive budget of at most five minutes").into());
        }
        let deadline = Instant::now() + self.preparation_timeout;
        let feed = prepare_side(feed, deadline, "preparing feed container binding").await?;
        let engine = ProcessorEngine::from_resolved(feed.driver, feed.container)
            .map_err(|error| context(error, "preparing feed container binding"))?;
        let leases = prepare_side(leases, deadline, "preparing lease container binding").await?;
        Ok(ChangeFeedProcessor {
            engine,
            prepared: Some(PreparedProcessor {
                group,
                feed: feed.binding,
                leases,
            }),
        })
    }
}

pub(crate) struct PreparedProcessor {
    pub group: String,
    pub feed: ContainerBinding,
    pub leases: PreparedContainer,
}

async fn prepare_side(
    binding: ContainerBinding,
    deadline: Instant,
    side: &'static str,
) -> azure_core::Result<PreparedContainer> {
    let budget = deadline.saturating_duration_since(Instant::now());
    if budget.is_zero() {
        return Err(preparation_expired(side));
    }
    let sleep_budget = budget
        .try_into()
        .map_err(|_| invalid("preparation budget is out of range"))?;
    match select(
        Box::pin(binding.prepare()),
        Box::pin(azure_core::sleep::sleep(sleep_budget)),
    )
    .await
    {
        Either::Left((result, _)) if Instant::now() < deadline => {
            result.map_err(|error| context(error, side))
        }
        _ => Err(preparation_expired(side)),
    }
}
fn preparation_expired(side: &'static str) -> azure_core::Error {
    context(
        CosmosError::builder()
            .with_status(CosmosStatus::new(StatusCode::RequestTimeout))
            .with_message("processor preparation exceeded its overall budget")
            .build(),
        side,
    )
}
fn context(error: CosmosError, side: &'static str) -> azure_core::Error {
    CosmosErrorBuilder::from_error(error)
        .with_context(side)
        .build()
        .into()
}

#[cfg(test)]
mod tests;

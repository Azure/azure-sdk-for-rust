// Copyright (c) Microsoft Corporation. All rights reserved.
// Licensed under the MIT License.

//! Shared runtime for one or more [`CosmosClient`](crate::CosmosClient) instances.
//!
//! A [`CosmosRuntime`] owns the background resources used by every
//! `CosmosClient` built on top of it — the HTTP client factory and
//! connection pool, the background CPU/memory sampler, the default
//! [`OperationOptions`], the User-Agent header, and any registered
//! throughput-control groups.
//!
//! Most applications never need to construct a runtime explicitly: every
//! [`CosmosClientBuilder::build`](crate::CosmosClientBuilder::build) call
//! falls back to [`CosmosRuntime::global`] when no runtime was supplied,
//! and the global runtime is sized appropriately for typical workloads.
//!
//! Construct a custom runtime via [`CosmosRuntime::builder`] when you need
//! to:
//!
//! - share a single transport across a large number of `CosmosClient`s,
//! - relax certificate validation against a local emulator,
//! - tunnel through an HTTP proxy,
//! - or configure runtime-wide throughput-control groups.
//!
//! Attach a custom runtime to a client with
//! [`CosmosClientBuilder::with_runtime`](crate::CosmosClientBuilder::with_runtime).

use std::sync::Arc;
use std::time::Duration;

use async_lock::OnceCell;

use azure_data_cosmos_driver::driver::{CosmosDriverRuntime, CosmosDriverRuntimeBuilder};

use crate::options::{
    ConnectionPoolOptions, OperationOptions, ThroughputControlGroupOptions, UserAgentSuffix,
};

/// A handle to a shared runtime for one or more [`CosmosClient`](crate::CosmosClient)s.
///
/// `CosmosRuntime` owns the HTTP client factory, connection pool, default
/// [`OperationOptions`], User-Agent header, and any registered
/// throughput-control groups that the clients built on top of it share.
///
/// This type is a handle to the runtime itself, cloning it will duplicate the handle
/// which will point to the same underlying runtime.
/// Use [`CosmosRuntime::global`] to obtain a process-wide shared default
/// runtime, or [`CosmosRuntime::builder`] to construct a customized one.
#[derive(Clone, Debug)]
pub struct CosmosRuntime(Arc<CosmosDriverRuntime>);

impl CosmosRuntime {
    /// Returns a new [`CosmosRuntimeBuilder`] for configuring a custom runtime.
    pub fn builder() -> CosmosRuntimeBuilder {
        CosmosRuntimeBuilder::new()
    }

    /// Returns the process-wide global runtime, initializing it on first call.
    ///
    /// This is the runtime
    /// [`CosmosClientBuilder::build`](crate::CosmosClientBuilder::build) falls
    /// back to when no runtime was supplied via
    /// [`CosmosClientBuilder::with_runtime`](crate::CosmosClientBuilder::with_runtime).
    ///
    /// # Errors
    ///
    /// Returns an error if the runtime fails to build (for example, if
    /// the HTTP client factory cannot be constructed).
    pub(crate) async fn global() -> crate::Result<Self> {
        static GLOBAL: OnceCell<CosmosRuntime> = OnceCell::new();
        GLOBAL
            .get_or_try_init(|| async { CosmosRuntimeBuilder::new().build().await })
            .await
            .cloned()
    }

    /// Consumes the runtime handle, returning a reference to the internal driver runtime.
    ///
    /// Used by the SDK's `CosmosClientBuilder::build` to wire the resolved
    /// runtime into a `CosmosDriver`.
    pub(crate) fn into_inner(self) -> Arc<CosmosDriverRuntime> {
        self.0
    }
}

/// Builder for constructing a customized [`CosmosRuntime`].
///
/// Use [`CosmosRuntime::builder`] (or [`CosmosRuntimeBuilder::new`]) to
/// start, configure with the `with_*` and `register_*` setters, then call
/// [`CosmosRuntimeBuilder::build`] to obtain the runtime. Attach it to one
/// or more clients via
/// [`CosmosClientBuilder::with_runtime`](crate::CosmosClientBuilder::with_runtime).
#[derive(Default, Debug, Clone)]
pub struct CosmosRuntimeBuilder(CosmosDriverRuntimeBuilder);

impl CosmosRuntimeBuilder {
    /// Returns a new builder with all default options.
    pub fn new() -> Self {
        Self::default()
    }

    /// Configures the connection pool used by the runtime's transport.
    ///
    /// Use [`ConnectionPoolOptions::builder`] (re-exported from this crate)
    /// to construct the pool. The pool controls TLS settings, proxy
    /// allowance, and emulator certificate-validation behavior.
    pub fn with_connection_pool(mut self, options: ConnectionPoolOptions) -> Self {
        self.0 = self.0.with_connection_pool(options);
        self
    }

    /// Sets the default [`OperationOptions`] applied to all requests on
    /// every client sharing this runtime, unless overridden at the client
    /// or per-request level.
    pub fn with_default_operation_options(mut self, options: OperationOptions) -> Self {
        self.0 = self.0.with_default_operation_options(options);
        self
    }

    /// Sets the runtime-wide default User-Agent suffix.
    ///
    /// A per-client override may be supplied via
    /// [`CosmosClientBuilder::with_user_agent_suffix`](crate::CosmosClientBuilder::with_user_agent_suffix);
    /// if absent, the runtime's suffix is used.
    pub fn with_user_agent_suffix(mut self, suffix: UserAgentSuffix) -> Self {
        self.0 = self.0.with_user_agent_suffix(suffix);
        self
    }

    /// Sets the CPU/memory sampler refresh interval.
    ///
    /// Controls how frequently the runtime's background sampler refreshes
    /// CPU and memory diagnostics. Defaults to the value of
    /// `AZURE_COSMOS_CPU_REFRESH_INTERVAL_MS`, or 5000 ms if unset. Valid
    /// range: 1000–60000 ms.
    pub fn with_cpu_refresh_interval(mut self, interval: Duration) -> Self {
        self.0 = self.0.with_cpu_refresh_interval(interval);
        self
    }

    /// Registers a runtime-wide [`ThroughputControlGroupOptions`].
    ///
    /// Groups registered here are shared by every client built on top of
    /// the runtime; per-client groups may be added via
    /// [`CosmosClientBuilder::register_throughput_control_group`](crate::CosmosClientBuilder::register_throughput_control_group)
    /// and are merged on top.
    ///
    /// # Errors
    ///
    /// Returns an error if a group with the same `(container, name)` key
    /// is already registered on this builder, or if another group is
    /// already marked as the default for the same container.
    pub fn register_throughput_control_group(
        mut self,
        group: ThroughputControlGroupOptions,
    ) -> crate::Result<Self> {
        self.0 = self
            .0
            .register_throughput_control_group(group)
            .map_err(crate::CosmosError::from)?;
        Ok(self)
    }

    /// Builds the [`CosmosRuntime`].
    ///
    /// Automatically applies the SDK's wrapping-SDK identifier
    /// (`azsdk-rust-cosmos/<version>`) so requests issued through clients
    /// built on this runtime can be attributed to this crate.
    ///
    /// # Errors
    ///
    /// Returns an error if the underlying driver runtime fails to build.
    pub async fn build(self) -> crate::Result<CosmosRuntime> {
        let mut inner = self.0;
        inner = inner.with_wrapping_sdk_identifier(format!(
            "azsdk-rust-cosmos/{}",
            env!("CARGO_PKG_VERSION")
        ));
        let runtime = inner.build().await.map_err(crate::CosmosError::from)?;
        Ok(CosmosRuntime(runtime))
    }
}

impl From<CosmosDriverRuntimeBuilder> for CosmosRuntimeBuilder {
    /// Constructs a `CosmosRuntimeBuilder` from a pre-configured
    /// [`CosmosDriverRuntimeBuilder`].
    ///
    /// Because this depends on directly creating a [`CosmosDriverRuntimeBuilder`], it is
    /// not an officially supported way to create a [`CosmosRuntime`]
    fn from(value: CosmosDriverRuntimeBuilder) -> Self {
        Self(value)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[tokio::test]
    async fn global_returns_same_runtime_across_calls() {
        let a = CosmosRuntime::global().await.expect("global builds");
        let b = CosmosRuntime::global().await.expect("global builds");
        assert!(
            Arc::ptr_eq(&a.0, &b.0),
            "global() must return the same Arc on repeated calls"
        );
    }

    #[tokio::test]
    async fn builder_applies_wrapping_sdk_identifier() {
        let runtime = CosmosRuntime::builder()
            .build()
            .await
            .expect("runtime builds");
        let ua = runtime.0.user_agent().as_str().to_string();
        assert!(
            ua.contains("azsdk-rust-cosmos/"),
            "user agent {ua:?} should contain the wrapping SDK identifier"
        );
    }

    #[tokio::test]
    async fn global_enables_per_partition_circuit_breaker_by_default() {
        let runtime = CosmosRuntime::global().await.expect("global builds");
        let ppcb = runtime
            .0
            .default_operation_options()
            .per_partition_circuit_breaker_enabled;
        assert_eq!(
            ppcb,
            Some(true),
            "global runtime must default to PPCB enabled"
        );
    }
}

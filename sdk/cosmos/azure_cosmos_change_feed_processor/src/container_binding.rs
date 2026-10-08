// Copyright (c) Microsoft Corporation. All rights reserved.
// Licensed under the MIT License.

use azure_core::{
    credentials::{Secret, TokenCredential},
    fmt::SafeDebug,
};
use azure_data_cosmos_driver::{
    models::{AccountReference, ContainerReference},
    options::{DriverOptions, OperationOptions, Region},
    CosmosDriver, CosmosDriverRuntimeBuilder, CosmosError, Result,
};
use std::sync::Arc;
use url::Url;

/// One named container, explicitly bound to its account credential and options.
///
/// A binding supplies authentication capabilities, not permanent authorization.
/// Each preparation creates a fresh runtime/cache namespace. Even cloned runtime
/// builder templates do not share resolved references between credentials.
#[derive(Clone, SafeDebug)]
pub struct ContainerBinding {
    account: AccountReference,
    database: String,
    container: String,
    operation: OperationOptions,
    preferred_regions: Vec<Region>,
    runtime: CosmosDriverRuntimeBuilder,
}
impl ContainerBinding {
    /// Binds required account credentials and resource names.
    ///
    /// # Errors
    ///
    /// Rejects empty database/container names. Other resource/authentication
    /// constraints are validated during preparation or by the service.
    pub fn new(
        account: AccountReference,
        database: impl Into<String>,
        container: impl Into<String>,
    ) -> Result<Self> {
        let database = database.into();
        let container = container.into();
        if database.trim().is_empty() || container.trim().is_empty() {
            return Err(invalid(
                "container binding requires database and container names",
            ));
        }
        Ok(Self {
            account,
            database,
            container,
            operation: OperationOptions::default(),
            preferred_regions: Vec::new(),
            runtime: CosmosDriverRuntimeBuilder::new(),
        })
    }
    /// Binds an application-supplied renewable token provider to one account.
    ///
    /// No default identity is chosen and no CFP token cache is introduced.
    ///
    /// # Errors
    ///
    /// Rejects empty resource names.
    pub fn with_token_credential(
        endpoint: Url,
        database: impl Into<String>,
        container: impl Into<String>,
        credential: Arc<dyn TokenCredential>,
    ) -> Result<Self> {
        Self::new(
            AccountReference::with_credential(endpoint, credential),
            database,
            container,
        )
    }
    /// Binds an explicitly supplied secret account key to this account only.
    ///
    /// Keys are immutable; automatic key rotation/rebinding is not implemented.
    ///
    /// # Errors
    ///
    /// Rejects empty resource names.
    pub fn with_key(
        endpoint: Url,
        database: impl Into<String>,
        container: impl Into<String>,
        key: Secret,
    ) -> Result<Self> {
        Self::new(
            AccountReference::with_master_key(endpoint, key),
            database,
            container,
        )
    }
    /// Sets this account's default request options, including latency budgets.
    pub fn with_operation_options(mut self, options: OperationOptions) -> Self {
        self.operation = options;
        self
    }
    /// Sets preferred routing regions for this account, independently of the other binding.
    pub fn with_preferred_regions(mut self, regions: Vec<Region>) -> Self {
        self.preferred_regions = regions;
        self
    }
    /// Sets a runtime-options template, not a live shared runtime.
    ///
    /// Every preparation builds a new runtime from this template. Connections,
    /// metadata caches, and credentials are retained for subsequent operations;
    /// no new driver is created for each lease or page.
    pub fn with_runtime_builder(mut self, builder: CosmosDriverRuntimeBuilder) -> Self {
        self.runtime = builder;
        self
    }
    /// Returns this account endpoint.
    pub fn endpoint(&self) -> &Url {
        self.account.endpoint()
    }
    /// Returns the explicitly credential-bound account reference.
    pub fn account(&self) -> &AccountReference {
        &self.account
    }
    /// Returns the database name.
    pub fn database(&self) -> &str {
        &self.database
    }
    /// Returns the container name.
    pub fn container(&self) -> &str {
        &self.container
    }
    /// Returns account-level request defaults.
    pub fn operation_options(&self) -> &OperationOptions {
        &self.operation
    }
    /// Returns account-level routing preferences.
    pub fn preferred_regions(&self) -> &[Region] {
        &self.preferred_regions
    }
    /// Returns the runtime template, not a resolved runtime or credential singleton.
    pub fn runtime_builder(&self) -> &CosmosDriverRuntimeBuilder {
        &self.runtime
    }

    pub(crate) async fn prepare(self) -> Result<PreparedContainer> {
        let runtime = self
            .runtime
            .clone()
            .with_wrapping_sdk_identifier(concat!(
                "azsdk-rust-cosmos-change-feed-processor/",
                env!("CARGO_PKG_VERSION"),
            ))
            .build()
            .await?;
        let driver = runtime
            .create_driver(
                DriverOptions::builder(self.account.clone())
                    .with_preferred_regions(self.preferred_regions.clone())
                    .with_operation_options(self.operation.clone())
                    .build(),
            )
            .await?;
        let container = driver
            .resolve_container(&self.database, &self.container, self.operation.clone())
            .await?;
        Ok(PreparedContainer {
            binding: self,
            driver,
            container,
        })
    }
}

pub(crate) struct PreparedContainer {
    pub binding: ContainerBinding,
    pub driver: Arc<CosmosDriver>,
    pub container: ContainerReference,
}

pub(crate) fn invalid(message: &'static str) -> CosmosError {
    CosmosError::builder()
        .with_status(azure_data_cosmos_driver::error::status_codes::CLIENT_BAD_REQUEST)
        .with_message(message)
        .build()
}

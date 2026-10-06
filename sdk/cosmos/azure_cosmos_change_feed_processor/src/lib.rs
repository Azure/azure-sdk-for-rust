// Copyright (c) Microsoft Corporation. All rights reserved.
// Licensed under the MIT License.

#![doc = include_str!("../README.md")]
#![warn(missing_docs)]

use std::{future::Future, num::NonZeroU32, sync::Arc};

use azure_core::{
    credentials::{Secret, TokenCredential},
    fmt::SafeDebug,
    http::StatusCode,
    Result,
};
use azure_cosmos_change_feed_processor_engine::ProcessorEngine;
pub use azure_cosmos_change_feed_processor_engine::{
    plan_equal_lease_balance, BalanceAction, BalanceActionKind, BalanceCycle, BalanceLease,
    BalanceRunOptions, BalanceRunReport, BatchState, BootstrapPlan, BootstrapReady,
    BootstrapStartPolicy, BootstrapStore, ChangeFeedMode, ChangeFeedReadOptions, CheckpointError,
    CheckpointStore, InitialLease, LeaseBalancer, LeaseControl, LeaseRunError, LeaseRunOptions,
    LeaseRunOutcome, LeaseRunPhase, LeaseRunReport, OwnedLease, RawChangeFeedPage,
};
pub use azure_cosmos_change_feed_processor_engine::{
    CosmosLeaseStore, LeaseObservation, LeaseOwnershipOptions, LeaseOwnershipRun,
    LeaseReleaseOutcome, LeaseSession,
};
use azure_data_cosmos_driver::{
    models::{AccountReference, ContinuationToken},
    CosmosError, CosmosErrorBuilder, ResponseBody,
};
use serde::{de::DeserializeOwned, Deserialize};
use url::Url;

mod builder;
mod container_binding;
pub use builder::ChangeFeedProcessorBuilder;
pub use container_binding::ContainerBinding;

pub mod models;
pub use models::{
    ChangeFeedItem, ChangeFeedMetadata, ChangeFeedOperationType, LogicalSequenceNumber,
};

/// Reads complete modeled change events and coordinates one already-owned lease.
///
/// Feed reads charge request units. An acquired Cosmos lease can be renewed
/// while processing. Bootstrap and balancing are explicit bounded operations;
/// an automatic start/stop supervisor is not provided.
pub struct ChangeFeedProcessor {
    engine: ProcessorEngine,
    prepared: Option<builder::PreparedProcessor>,
}

impl ChangeFeedProcessor {
    /// Creates a builder requiring independently credential-bound feed and lease containers.
    pub fn builder() -> ChangeFeedProcessorBuilder {
        ChangeFeedProcessorBuilder::default()
    }
    /// Returns the prepared processor group, absent for legacy source-only constructors.
    pub fn group(&self) -> Option<&str> {
        self.prepared
            .as_ref()
            .map(|prepared| prepared.group.as_str())
    }
    /// Returns the configured feed binding, absent for legacy source-only constructors.
    pub fn feed_binding(&self) -> Option<&ContainerBinding> {
        self.prepared.as_ref().map(|prepared| &prepared.feed)
    }
    /// Returns the independently configured lease binding.
    pub fn lease_binding(&self) -> Option<&ContainerBinding> {
        self.prepared
            .as_ref()
            .map(|prepared| &prepared.leases.binding)
    }
    /// Addresses a pre-created lease through the prepared lease account only.
    ///
    /// The account must be single-writer. This performs no I/O, name resolution,
    /// creation, or acquisition; operations reuse the prepared container reference.
    ///
    /// # Errors
    ///
    /// Rejects a source-only processor, invalid lease identity, or mismatched binding.
    pub async fn lease_store(
        &self,
        partition_key: azure_data_cosmos_driver::models::PartitionKey,
        id: impl Into<String>,
        options: LeaseOwnershipOptions,
    ) -> Result<CosmosLeaseStore> {
        let leases = self.configured_leases()?;
        CosmosLeaseStore::from_resolved_single_writer(
            leases.driver.clone(),
            leases.container.clone(),
            partition_key,
            id,
            options,
        )
        .map_err(|error| {
            CosmosErrorBuilder::from_error(error)
                .with_context("addressing configured lease container")
                .build()
                .into()
        })
    }
    /// Creates a validated source-bound bootstrap plan using the prepared group.
    ///
    /// # Errors
    ///
    /// Rejects a source-only processor or invalid source/mode/range seed positions.
    pub async fn bootstrap_plan(
        &self,
        mode: ChangeFeedMode,
        start: BootstrapStartPolicy,
        expected: azure_data_cosmos_driver::models::FeedRange,
        seeds: Vec<InitialLease>,
    ) -> Result<BootstrapPlan> {
        let group = self.group().ok_or_else(|| {
            container_binding::invalid("bootstrap preparation requires a two-binding processor")
        })?;
        self.engine
            .bootstrap_plan(group, mode, start, expected, seeds)
            .await
            .map_err(|error| {
                CosmosErrorBuilder::from_error(error)
                    .with_context("validating feed bootstrap plan")
                    .build()
                    .into()
            })
    }
    /// Addresses the configured lease container for explicit later bootstrap.
    ///
    /// This performs no I/O or name resolution and does not initialize, create
    /// records, or acquire authority.
    ///
    /// # Errors
    ///
    /// Rejects missing bindings, wrong source/group, or incompatible lease-account partitioning.
    pub async fn bootstrap_store(
        &self,
        plan: BootstrapPlan,
        policy: LeaseOwnershipOptions,
    ) -> Result<BootstrapStore> {
        let leases = self.configured_leases()?;
        if self.group() != Some(plan.group()) || plan.source() != self.engine.source_identity() {
            return Err(container_binding::invalid(
                "bootstrap plan source or group does not match this processor",
            )
            .into());
        }
        BootstrapStore::from_resolved_single_writer(
            leases.driver.clone(),
            leases.container.clone(),
            plan,
            policy,
        )
        .map_err(|error| {
            CosmosErrorBuilder::from_error(error)
                .with_context("addressing configured lease bootstrap store")
                .build()
                .into()
        })
    }
    fn configured_leases(&self) -> Result<&container_binding::PreparedContainer> {
        self.prepared
            .as_ref()
            .map(|prepared| &prepared.leases)
            .ok_or_else(|| {
                container_binding::invalid(
                    "no lease binding was supplied; source credentials are never reused implicitly",
                )
                .into()
            })
    }
    /// Runs an acquired Cosmos-backed lease with renewal during application work.
    ///
    /// Successful handler progress is persisted using the session's current ETag;
    /// obsolete workers cannot checkpoint or release replacement authority.
    /// Processing, renewal, and release outcomes are reported separately.
    pub async fn run_with_lease_session<T, H, F>(
        &self,
        session: &LeaseSession,
        read: ChangeFeedReadOptions,
        options: &LeaseRunOptions,
        mut handler: H,
    ) -> LeaseOwnershipRun
    where
        T: DeserializeOwned,
        H: FnMut(ChangeFeedPage<T>) -> F,
        F: Future<Output = azure_data_cosmos_driver::Result<()>>,
    {
        self.engine
            .run_with_lease_session(session, read, options, |raw| {
                let decoded: azure_data_cosmos_driver::Result<ChangeFeedPage<T>> = raw.try_into();
                let completion = decoded.map(&mut handler);
                async move { completion?.await }
            })
            .await
    }
    /// Connects using an Azure token credential and resolves the container.
    ///
    /// The credential needs Cosmos DB data-plane read permissions, including
    /// account metadata and change-feed access.
    ///
    /// # Errors
    ///
    /// Returns authentication, transport, or metadata-resolution errors.
    pub async fn connect(
        endpoint: Url,
        credential: Arc<dyn TokenCredential>,
        database: &str,
        container: &str,
    ) -> Result<Self> {
        Self::connect_account(
            AccountReference::with_credential(endpoint, credential),
            database,
            container,
        )
        .await
    }

    /// Connects using an account key and resolves the container.
    ///
    /// A read-only account key is sufficient. Load the key from a secure
    /// credential source rather than embedding it in code.
    ///
    /// # Errors
    ///
    /// Returns authentication, transport, or metadata-resolution errors.
    pub async fn connect_with_key(
        endpoint: Url,
        key: Secret,
        database: &str,
        container: &str,
    ) -> Result<Self> {
        Self::connect_account(
            AccountReference::with_master_key(endpoint, key),
            database,
            container,
        )
        .await
    }

    async fn connect_account(
        account: AccountReference,
        database: &str,
        container: &str,
    ) -> Result<Self> {
        Ok(Self {
            engine: ProcessorEngine::connect(account, database, container).await?,
            prepared: None,
        })
    }

    /// Reads one latest-version page across the monitored container.
    ///
    /// With no continuation, reading starts at the beginning. Save the returned
    /// token only after processing the page. An empty page means a partition is
    /// idle, not that the whole feed is exhausted. Page size is a service hint;
    /// planning admits at most 100 physical partitions for a fresh read.
    ///
    /// No cursor state is advanced in this object. Failed or cancelled calls
    /// can be retried with the same options without skipping a page.
    ///
    /// # Errors
    ///
    /// Returns invalid-token, service, transport, or typed-decoding errors.
    /// Driver failures retain Cosmos status and diagnostics in the error source.
    ///
    /// # Examples
    ///
    /// ```rust,no_run
    /// use azure_cosmos_change_feed_processor::{ChangeFeedProcessor, ReadChangesOptions};
    /// use serde::Deserialize;
    ///
    /// #[derive(Deserialize)]
    /// struct Document { id: String }
    ///
    /// # async fn example(processor: ChangeFeedProcessor) -> azure_core::Result<()> {
    /// let page = processor.read_page::<Document>(&ReadChangesOptions::default()).await?;
    /// for change in page.items() {
    ///     if let Some(document) = change.current() { println!("{}", document.id); }
    /// }
    /// let next = ReadChangesOptions::default()
    ///     .with_continuation(page.continuation());
    /// let _next_page = processor.read_page::<Document>(&next).await?;
    /// # Ok(())
    /// # }
    /// ```
    pub async fn read_page<T: DeserializeOwned>(
        &self,
        options: &ReadChangesOptions,
    ) -> Result<ChangeFeedPage<T>> {
        let continuation = options
            .continuation
            .as_ref()
            .map(|token| ContinuationToken::from_string(token.clone()));
        let raw = self
            .engine
            .read_page(options.max_item_count, continuation.as_ref())
            .await?;
        raw.try_into().map_err(Into::into)
    }

    /// Reads a typed page with explicit mode, range, start, and driver options.
    ///
    /// # Errors
    ///
    /// Returns planning, service, transport, or document-decoding errors.
    pub async fn read_page_with_options<T: DeserializeOwned>(
        &self,
        options: ChangeFeedReadOptions,
    ) -> Result<ChangeFeedPage<T>> {
        self.engine
            .open_reader(options)
            .await?
            .read_page()
            .await?
            .try_into()
            .map_err(Into::into)
    }

    /// Delivers one batch at a time and checkpoints only completed application work.
    ///
    /// The successful handler future is this slice's acknowledgment. The store
    /// must confirm conditional persistence before another batch is read.
    /// Ownership loss cancels local admission; stop allows bounded drain.
    ///
    /// Return success only after application work actually completes, not after
    /// scheduling it elsewhere. A batch is complete only after its checkpoint is
    /// confirmed. A crash between those steps can cause replay on recovery.
    ///
    /// # Errors
    ///
    /// Returns decoding, handler/unwinding-panic, or checkpoint failures with recovery positions
    /// and page diagnostics. Ambiguous persistence requires store reconciliation.
    pub async fn run_owned_lease<T, C, H, F>(
        &self,
        lease: OwnedLease,
        read: ChangeFeedReadOptions,
        control: &LeaseControl,
        options: &LeaseRunOptions,
        store: &mut C,
        mut handler: H,
    ) -> std::result::Result<LeaseRunReport, LeaseRunError>
    where
        T: DeserializeOwned,
        C: CheckpointStore,
        H: FnMut(ChangeFeedPage<T>) -> F,
        F: Future<Output = azure_data_cosmos_driver::Result<()>>,
    {
        self.engine
            .run_owned_lease(lease, read, control, options, store, |raw| {
                let decoded: azure_data_cosmos_driver::Result<ChangeFeedPage<T>> = raw.try_into();
                let completion = decoded.map(&mut handler);
                async move { completion?.await }
            })
            .await
    }
}

/// Options for a single latest-version change-feed page read.
#[derive(Clone, SafeDebug)]
pub struct ReadChangesOptions {
    max_item_count: NonZeroU32,
    continuation: Option<String>,
}

impl Default for ReadChangesOptions {
    fn default() -> Self {
        Self {
            max_item_count: NonZeroU32::new(100).expect("100 is non-zero"),
            continuation: None,
        }
    }
}

impl ReadChangesOptions {
    /// Sets the requested page size; the service may return fewer items.
    pub fn with_max_item_count(mut self, max_item_count: NonZeroU32) -> Self {
        self.max_item_count = max_item_count;
        self
    }

    /// Sets the opaque token returned by an earlier successful page read.
    pub fn with_continuation(mut self, continuation: impl Into<String>) -> Self {
        self.continuation = Some(continuation.into());
        self
    }

    /// Returns the requested page size.
    pub fn max_item_count(&self) -> NonZeroU32 {
        self.max_item_count
    }

    /// Returns the supplied continuation, if any.
    pub fn continuation(&self) -> Option<&str> {
        self.continuation.as_deref()
    }
}

/// A typed change-feed page and its next position.
#[derive(SafeDebug)]
pub struct ChangeFeedPage<T> {
    items: Vec<ChangeFeedItem<T>>,
    raw: RawChangeFeedPage,
}

impl<T> ChangeFeedPage<T> {
    /// Returns the changes in this page.
    pub fn items(&self) -> &[ChangeFeedItem<T>] {
        &self.items
    }

    /// Returns the opaque position after this page.
    ///
    /// Persist it after processing all changes, not before.
    pub fn continuation(&self) -> &str {
        self.raw.continuation().as_str()
    }

    /// Returns the page's HTTP status, including 304 for an idle partition.
    pub fn status(&self) -> StatusCode {
        self.raw.response().status().status_code()
    }

    /// Returns the page's request charge in request units, when available.
    pub fn request_charge(&self) -> Option<f64> {
        self.raw
            .response()
            .headers()
            .request_charge
            .map(|charge| charge.value())
    }

    /// Returns the service activity ID, when available.
    pub fn activity_id(&self) -> Option<&str> {
        self.raw
            .response()
            .headers()
            .activity_id
            .as_ref()
            .map(|id| id.as_str())
    }

    /// Returns complete raw payload and response context alongside the typed view.
    pub fn raw(&self) -> &RawChangeFeedPage {
        &self.raw
    }
    /// Returns idle versus non-idle response state, independently of item count.
    pub fn state(&self) -> BatchState {
        self.raw.state()
    }
    /// Returns page diagnostics without discarding them during typed decoding.
    pub fn diagnostics(&self) -> Arc<azure_data_cosmos_driver::DiagnosticsContext> {
        self.raw.response().diagnostics()
    }
    /// Returns all parsed driver response headers.
    pub fn headers(&self) -> &azure_data_cosmos_driver::models::CosmosResponseHeaders {
        self.raw.response().headers()
    }
}

#[derive(Deserialize)]
struct FeedEnvelope<T> {
    #[serde(rename = "Documents", alias = "items")]
    items: Vec<T>,
}

fn decode_page<T: DeserializeOwned>(
    raw: RawChangeFeedPage,
) -> azure_data_cosmos_driver::Result<ChangeFeedPage<T>> {
    let response = raw.response();
    let status = response.status().status_code();
    let diagnostics = response.diagnostics();
    let items = decode_items(response.body().clone(), status).map_err(|error| {
        azure_data_cosmos_driver::CosmosErrorBuilder::from_error(error)
            .with_diagnostics(diagnostics)
            .build()
    })?;
    Ok(ChangeFeedPage { items, raw })
}

fn decode_items<T: DeserializeOwned>(
    body: ResponseBody,
    status: StatusCode,
) -> azure_data_cosmos_driver::Result<Vec<ChangeFeedItem<T>>> {
    if status == StatusCode::NotModified {
        Ok(Vec::new())
    } else {
        match body {
            ResponseBody::Bytes(bytes) => ResponseBody::Bytes(bytes)
                .into_single::<FeedEnvelope<ChangeFeedItem<T>>>()
                .map(|envelope| envelope.items),
            ResponseBody::Items(items) => ResponseBody::Items(items).into_items::<ChangeFeedItem<T>>(),
            ResponseBody::NoPayload => Err(CosmosError::builder()
                .with_status(azure_data_cosmos_driver::error::status_codes::SERIALIZATION_RESPONSE_BODY_INVALID)
                .with_message("change-feed response has no payload without an idle status")
                .build()),
        }
    }
}

impl<T: DeserializeOwned> TryFrom<RawChangeFeedPage> for ChangeFeedPage<T> {
    type Error = CosmosError;
    fn try_from(raw: RawChangeFeedPage) -> std::result::Result<Self, Self::Error> {
        decode_page(raw)
    }
}

#[cfg(test)]
mod processing_tests;
#[cfg(test)]
mod tests;

// Copyright (c) Microsoft Corporation. All rights reserved.
// Licensed under the MIT License.

use std::sync::Arc;

use azure_core::http::StatusCode;
use azure_data_cosmos_driver::{
    models::{
        ChangeFeedStartFrom, ContainerReference, ContinuationToken, CosmosOperation, FeedRange,
        MaxItemCountHint, SessionToken,
    },
    options::{OperationOptions, PlanOptions},
    CosmosDriver, CosmosError, CosmosResponse, OperationPlan, Result,
};
use futures::future::BoxFuture;

/// The change-feed mode requested from the service.
#[derive(Clone, Copy, Debug, Default, PartialEq, Eq)]
#[non_exhaustive]
pub enum ChangeFeedMode {
    /// Latest versions of created and replaced items.
    #[default]
    LatestVersion,
    /// Intermediate versions and deletes within service retention.
    AllVersionsAndDeletes,
}

/// Configuration fixed for a retained change-feed reader.
#[derive(Clone, Debug)]
pub struct ChangeFeedReadOptions {
    range: FeedRange,
    start: ChangeFeedStartFrom,
    mode: ChangeFeedMode,
    continuation: Option<ContinuationToken>,
    max_item_count: Option<MaxItemCountHint>,
    session_token: Option<SessionToken>,
    operation: OperationOptions,
    planning: PlanOptions,
}

impl ChangeFeedReadOptions {
    /// Sets the required source range and initial position.
    ///
    /// A saved continuation takes precedence over the initial position.
    pub fn new(range: FeedRange, start: ChangeFeedStartFrom) -> Self {
        Self {
            range,
            start,
            mode: ChangeFeedMode::LatestVersion,
            continuation: None,
            max_item_count: None,
            session_token: None,
            operation: OperationOptions::default(),
            planning: PlanOptions::default(),
        }
    }

    /// Sets the feed mode. AVAD fresh starts are subject to service restrictions.
    pub fn with_mode(mut self, mode: ChangeFeedMode) -> Self {
        self.mode = mode;
        self
    }
    /// Sets a saved position; validation is performed by the driver.
    pub fn with_continuation(mut self, continuation: ContinuationToken) -> Self {
        self.continuation = Some(continuation);
        self
    }
    /// Sets a service page-size hint, not a strict client memory limit.
    pub fn with_max_item_count(mut self, hint: MaxItemCountHint) -> Self {
        self.max_item_count = Some(hint);
        self
    }
    /// Sets the session token.
    pub fn with_session_token(mut self, token: SessionToken) -> Self {
        self.session_token = Some(token);
        self
    }
    /// Sets request options, including latency and consistency controls.
    pub fn with_operation_options(mut self, options: OperationOptions) -> Self {
        self.operation = options;
        self
    }
    /// Sets planning options, including the admitted fan-out.
    pub fn with_plan_options(mut self, options: PlanOptions) -> Self {
        self.planning = options;
        self
    }
    /// Returns the source range.
    pub fn range(&self) -> &FeedRange {
        &self.range
    }
    /// Returns the initial position.
    pub fn start(&self) -> &ChangeFeedStartFrom {
        &self.start
    }
    /// Returns the feed mode.
    pub fn mode(&self) -> ChangeFeedMode {
        self.mode
    }
    /// Returns the saved position.
    pub fn continuation(&self) -> Option<&ContinuationToken> {
        self.continuation.as_ref()
    }
    /// Returns the page-size hint.
    pub fn max_item_count(&self) -> Option<MaxItemCountHint> {
        self.max_item_count
    }
    /// Returns the session token.
    pub fn session_token(&self) -> Option<&SessionToken> {
        self.session_token.as_ref()
    }
    /// Returns request options.
    pub fn operation_options(&self) -> &OperationOptions {
        &self.operation
    }
    /// Returns planning options.
    pub fn plan_options(&self) -> &PlanOptions {
        &self.planning
    }

    fn operation(&self, container: ContainerReference) -> CosmosOperation {
        let mut operation = match self.mode {
            ChangeFeedMode::LatestVersion => {
                CosmosOperation::change_feed(container, Some(self.range.clone()))
            }
            ChangeFeedMode::AllVersionsAndDeletes => {
                CosmosOperation::change_feed_all_versions_and_deletes(
                    container,
                    Some(self.range.clone()),
                )
            }
        }
        .with_change_feed_start(self.start.clone());
        if let Some(hint) = self.max_item_count {
            operation = operation.with_max_item_count(hint);
        }
        if let Some(token) = &self.session_token {
            operation = operation.with_session_token(token.clone());
        }
        operation
    }
}

/// Retains a driver plan across reads without decoding customer documents.
///
/// A failed or cancelled read invalidates this reader. Reopen from confirmed
/// durable progress rather than attempting to skip the uncertain page.
pub struct ChangeFeedReader {
    driver: Arc<CosmosDriver>,
    container: ContainerReference,
    plan: OperationPlan,
    options: OperationOptions,
    invalidated: bool,
}

impl ChangeFeedReader {
    /// Captures the reader's current position without fetching or acknowledging.
    ///
    /// # Errors
    ///
    /// Returns a driver snapshot error or rejects an invalidated reader.
    pub fn to_continuation_token(&self) -> Result<ContinuationToken> {
        if self.invalidated {
            return Err(CosmosError::builder()
                .with_status(azure_data_cosmos_driver::error::status_codes::CLIENT_BAD_REQUEST)
                .with_message("cannot snapshot an invalidated reader")
                .build());
        }
        self.plan.to_continuation_token()
    }
    pub(crate) async fn open(
        driver: Arc<CosmosDriver>,
        container: ContainerReference,
        options: ChangeFeedReadOptions,
    ) -> Result<Self> {
        let plan = driver
            .plan_operation(
                options.operation(container.clone()),
                &options.operation,
                options.continuation.as_ref(),
                &options.planning,
            )
            .await?;
        Ok(Self {
            driver,
            container,
            plan,
            options: options.operation,
            invalidated: false,
        })
    }

    /// Fetches a page and captures its post-page continuation candidate.
    ///
    /// Neither fetching nor snapshotting acknowledges processing.
    ///
    /// # Errors
    ///
    /// Returns driver errors or an invalid-reader error after a previous failure
    /// or cancellation. An unexpectedly drained change-feed plan is an error.
    pub async fn read_page(&mut self) -> Result<RawChangeFeedPage> {
        if self.invalidated {
            return Err(CosmosError::builder()
                .with_status(azure_data_cosmos_driver::error::status_codes::CLIENT_BAD_REQUEST)
                .with_message("change-feed reader is invalidated; reopen from durable progress")
                .build());
        }
        self.invalidated = true;
        let response = self.driver.execute_plan(
            &mut self.plan, Some(self.container.clone()), self.options.clone(),
        ).await?.ok_or_else(|| CosmosError::builder()
            .with_status(azure_data_cosmos_driver::error::status_codes::CLIENT_CHANGE_FEED_PIPELINE_UNEXPECTEDLY_DRAINED)
            .with_message("change feed unexpectedly ended instead of returning an idle page")
            .build())?;
        let continuation = self.plan.to_continuation_token().map_err(|error| {
            azure_data_cosmos_driver::CosmosErrorBuilder::from_error(error)
                .with_diagnostics(response.diagnostics())
                .build()
        })?;
        self.invalidated = false;
        Ok(RawChangeFeedPage {
            response,
            continuation,
        })
    }
}

/// A source of raw batches for the single-lease coordinator.
///
/// Implementations must preserve event bodies and associated post-batch tokens.
pub trait RawBatchSource: Send {
    /// Reads one batch; failures must not be represented as empty successful pages.
    fn read_batch(&mut self) -> BoxFuture<'_, Result<RawChangeFeedPage>>;
}

impl RawBatchSource for ChangeFeedReader {
    fn read_batch(&mut self) -> BoxFuture<'_, Result<RawChangeFeedPage>> {
        Box::pin(self.read_page())
    }
}

/// The response state, independent of the number of items.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum BatchState {
    /// HTTP 304: this range is currently idle, not terminated.
    Idle,
    /// A successful page, which can still contain zero items.
    Page,
}

/// A complete raw driver response and candidate position after that batch.
#[derive(Clone)]
pub struct RawChangeFeedPage {
    response: CosmosResponse,
    continuation: ContinuationToken,
}

impl RawChangeFeedPage {
    /// Returns the unmodified body, status, headers, and diagnostics.
    ///
    /// `Bytes` can contain a whole Documents envelope. `Items` already contains
    /// separate event buffers; `ResponseBody::items()` does not split an envelope.
    pub fn response(&self) -> &CosmosResponse {
        &self.response
    }
    /// Returns the candidate position. Persist only after processing succeeds.
    pub fn continuation(&self) -> &ContinuationToken {
        &self.continuation
    }
    /// Distinguishes idle status from an empty non-idle page.
    pub fn state(&self) -> BatchState {
        if self.response.status().status_code() == StatusCode::NotModified {
            BatchState::Idle
        } else {
            BatchState::Page
        }
    }
}

impl From<RawChangeFeedPage> for (CosmosResponse, ContinuationToken) {
    fn from(page: RawChangeFeedPage) -> Self {
        (page.response, page.continuation)
    }
}

impl From<(CosmosResponse, ContinuationToken)> for RawChangeFeedPage {
    /// Associates a complete driver response with its post-page candidate.
    fn from((response, continuation): (CosmosResponse, ContinuationToken)) -> Self {
        Self {
            response,
            continuation,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::{ChangeFeedMode, ChangeFeedReadOptions};
    use azure_data_cosmos_driver::models::{
        AccountReference, ChangeFeedStartFrom, FeedRange, MaxItemCountHint, SessionToken,
    };
    use azure_data_cosmos_driver::{
        in_memory_emulator::{InMemoryEmulatorHttpClient, VirtualAccountConfig, VirtualRegion},
        options::{DriverOptions, OperationOptions},
    };
    use std::num::NonZeroU32;

    #[tokio::test]
    async fn operation_mapping_reuses_driver_modes_range_start_and_hints() {
        let endpoint = "https://eastus.emulator.local".parse().unwrap();
        let account = AccountReference::with_master_key(endpoint, "dGVzdGtleQ==");
        let emulator = std::sync::Arc::new(InMemoryEmulatorHttpClient::new(
            VirtualAccountConfig::new(vec![VirtualRegion::new(
                "East US",
                account.endpoint().clone(),
            )])
            .unwrap(),
        ));
        emulator.store().create_database("db");
        emulator.store().create_container(
            "db",
            "feed",
            serde_json::from_value(serde_json::json!({"paths":["/pk"],"kind":"Hash","version":2}))
                .unwrap(),
        );
        let runtime = emulator.runtime_builder().build().await.unwrap();
        let driver = runtime
            .create_driver(DriverOptions::builder(account).build())
            .await
            .unwrap();
        let container = driver
            .resolve_container("db", "feed", OperationOptions::default())
            .await
            .unwrap();
        let range = FeedRange::new("40".try_into().unwrap(), "80".try_into().unwrap()).unwrap();
        let hint = MaxItemCountHint::Limit(NonZeroU32::new(7).unwrap());
        for mode in [
            ChangeFeedMode::LatestVersion,
            ChangeFeedMode::AllVersionsAndDeletes,
        ] {
            let operation = ChangeFeedReadOptions::new(range.clone(), ChangeFeedStartFrom::Now)
                .with_mode(mode)
                .with_max_item_count(hint)
                .with_session_token(SessionToken::new("0:1#1"))
                .operation(container.clone());
            let headers = operation.request_headers();
            assert_eq!(
                headers.incremental_feed,
                mode == ChangeFeedMode::LatestVersion
            );
            assert_eq!(
                headers.full_fidelity_feed,
                mode == ChangeFeedMode::AllVersionsAndDeletes
            );
            assert!(headers.changefeed_wire_format_version);
            assert_eq!(headers.max_item_count, Some(hint));
            assert_eq!(
                headers.session_token.as_ref().map(SessionToken::as_str),
                Some("0:1#1")
            );
            assert_eq!(operation.target(), Some(&range));
            assert_eq!(
                operation.change_feed_start(),
                Some(&ChangeFeedStartFrom::Now)
            );
        }
    }
}

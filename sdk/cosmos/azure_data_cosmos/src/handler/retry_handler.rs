// Copyright (c) Microsoft Corporation. All rights reserved.
// Licensed under the MIT License.

use crate::cosmos_request::CosmosRequest;
use crate::operation_context::OperationType;
use crate::retry_policies::client_retry_policy::ClientRetryPolicy;
use crate::retry_policies::metadata_request_retry_policy::MetadataRequestRetryPolicy;
use crate::retry_policies::{convert_gone_to_service_unavailable, RetryPolicy, RetryResult};
use crate::routing::global_endpoint_manager::GlobalEndpointManager;
use crate::routing::global_partition_endpoint_manager::GlobalPartitionEndpointManager;
use async_trait::async_trait;
use azure_core::error::ErrorKind;
use azure_core::time::Duration;
use azure_core::{async_runtime::get_async_runtime, http::RawResponse};
use futures::future::Either;
use std::sync::Arc;
use tracing::debug;

/// Environment variable used to override the detached-metadata hard deadline (in seconds).
const METADATA_DETACHED_HARD_DEADLINE_ENV: &str =
    "AZURE_COSMOS_METADATA_DETACHED_HARD_DEADLINE_SECONDS";

/// Default hard deadline bounding a detached metadata read (5 minutes).
const DEFAULT_METADATA_DETACHED_HARD_DEADLINE_SECS: u64 = 300;

/// Upper bound for the detached-metadata hard deadline (24 hours). Clamping guards
/// against a misconfigured environment value producing an unreasonably long-lived
/// background task.
const MAX_METADATA_DETACHED_HARD_DEADLINE_SECS: u64 = 86_400;

/// Defensive cap on the number of attempts a detached metadata read may make.
/// Guards against a misbehaving retry policy that keeps returning
/// [`RetryResult::Retry`] with zero backoff.
const METADATA_DETACHED_ATTEMPT_CAP: usize = 50;

/// Returns `true` when a request is a control-plane metadata read that is safe to run
/// on a detached task (see [`send_detached`]).
///
/// Detaching means the work continues even if the caller drops its future, so it must
/// be restricted to **idempotent** operations. [`ResourceType::is_meta_data`] alone is
/// insufficient because metadata resource types also carry mutating operations (e.g.
/// creating or deleting a database/container); detaching those could double-apply a
/// write if the caller drops and then retries. We therefore additionally require a
/// read-style (non-mutating) operation. This intentionally excludes `Create`,
/// `Replace`, `Delete`, `Upsert`, `Patch`, and `Batch`.
fn is_detachable_metadata_read(request: &CosmosRequest) -> bool {
    request.resource_type.is_meta_data()
        && matches!(
            request.operation_type,
            OperationType::Read
                | OperationType::ReadFeed
                | OperationType::Query
                | OperationType::SqlQuery
                | OperationType::QueryPlan
                | OperationType::Head
                | OperationType::HeadFeed
        )
}

/// Resolves the hard deadline that bounds a detached metadata read.
///
/// Reads [`METADATA_DETACHED_HARD_DEADLINE_ENV`] and delegates to
/// [`resolve_hard_deadline_secs`] for parsing, validation, and clamping.
fn metadata_detached_hard_deadline() -> Duration {
    let raw = std::env::var(METADATA_DETACHED_HARD_DEADLINE_ENV).ok();
    Duration::seconds(resolve_hard_deadline_secs(raw.as_deref()) as i64)
}

/// Parses, validates, and clamps a hard-deadline override value (in seconds).
///
/// A positive integer is honored up to [`MAX_METADATA_DETACHED_HARD_DEADLINE_SECS`];
/// anything missing, non-numeric, or zero falls back to
/// [`DEFAULT_METADATA_DETACHED_HARD_DEADLINE_SECS`]. The clamp keeps the value well
/// within range for constructing a timer and prevents an unbounded background task.
///
/// Split out from [`metadata_detached_hard_deadline`] so the logic can be unit-tested
/// without mutating the process-global environment.
fn resolve_hard_deadline_secs(raw: Option<&str>) -> u64 {
    raw.and_then(|value| value.trim().parse::<u64>().ok())
        .filter(|&v| v > 0)
        .unwrap_or(DEFAULT_METADATA_DETACHED_HARD_DEADLINE_SECS)
        .min(MAX_METADATA_DETACHED_HARD_DEADLINE_SECS)
}

/// Trait defining the interface for retry handlers in Cosmos DB operations
///
/// This trait provides a contract for implementing retry logic that wraps HTTP requests
/// with automatic retry capabilities. Implementations can inject custom retry policies
/// and handle both transient failures (errors) and non-success HTTP responses.
#[allow(dead_code)]
#[async_trait]
pub trait RetryHandler: Send + Sync {
    /// Sends an HTTP request with automatic retry logic
    ///
    /// This method wraps the provided sender callback with retry logic, automatically
    /// handling transient failures and implementing exponential backoff. The method
    /// will continue retrying until either:
    /// - The request succeeds (non-error 2xx status)
    /// - The retry policy determines no more retries should be attempted
    /// - Maximum retry attempts are exceeded
    ///
    /// # Arguments
    /// * `request` - Mutable reference to the HTTP request to send (may be modified by retry policy)
    /// * `sender` - Callback function that performs the actual HTTP request. This function
    ///              takes a mutable request reference and returns a future that resolves to
    ///              a `RawResponse` or error.
    ///
    /// # Type Parameters
    /// * `Sender` - Function type that takes `&mut Request` and returns a Future
    /// * `Fut` - Future type returned by the sender that resolves to `Result<RawResponse>`
    ///
    /// # Returns
    /// `Result<RawResponse>` - The final response (success or failure after all retry attempts)
    async fn send<Sender, Fut>(
        &self,
        request: &mut CosmosRequest,
        sender: Sender,
    ) -> azure_core::Result<RawResponse>
    where
        Sender: Fn(&mut CosmosRequest) -> Fut + Send + Sync + 'static,
        Fut: std::future::Future<Output = azure_core::Result<RawResponse>> + Send + 'static;
}

/// Concrete retry handler implementation with exponential back off.
/// This handler provides automatic retry capabilities for Cosmos DB operations using
/// a pluggable retry policy system. It wraps HTTP requests with intelligent retry logic
/// that handles both transient network errors and HTTP error responses.
#[derive(Debug, Clone)]
pub(crate) struct BackOffRetryHandler {
    global_endpoint_manager: Arc<GlobalEndpointManager>,
    global_partition_endpoint_manager: Arc<GlobalPartitionEndpointManager>,
}

impl BackOffRetryHandler {
    /// Returns the appropriate retry policy based on the request
    ///
    /// This method examines the underlying operation and resource types and determines
    /// which retry policy should be used for this specific request. Metadata operations
    /// use the MetadataRequestRetryPolicy, while data plane operations use the
    /// ClientRetryPolicy.
    ///
    /// # Arguments
    /// * `request` - The HTTP request to analyze
    ///
    /// # Returns
    /// A `RetryPolicy` enum variant appropriate for the request type
    pub fn retry_policy_for_request(&self, request: &CosmosRequest) -> RetryPolicy {
        if request.resource_type.is_meta_data() {
            RetryPolicy::Metadata(MetadataRequestRetryPolicy::new(
                self.global_endpoint_manager.clone(),
            ))
        } else {
            RetryPolicy::Client(Box::from(ClientRetryPolicy::new(
                self.global_endpoint_manager.clone(),
                self.global_partition_endpoint_manager.clone(),
                request.excluded_regions.clone(),
            )))
        }
    }

    pub fn new(
        global_endpoint_manager: Arc<GlobalEndpointManager>,
        global_partition_endpoint_manager: Arc<GlobalPartitionEndpointManager>,
    ) -> Self {
        Self {
            global_endpoint_manager,
            global_partition_endpoint_manager,
        }
    }
}

#[async_trait]
impl RetryHandler for BackOffRetryHandler {
    /// Sends an HTTP request with automatic retry and exponential back off
    ///
    /// This implementation of the `RetryHandler::send` method provides robust
    /// retry logic.
    ///
    /// # Arguments
    /// * `request` - Mutable HTTP request (may be modified by retry policy between attempts)
    /// * `sender` - Callback that performs the actual HTTP request
    async fn send<Sender, Fut>(
        &self,
        request: &mut CosmosRequest,
        sender: Sender,
    ) -> azure_core::Result<RawResponse>
    where
        Sender: Fn(&mut CosmosRequest) -> Fut + Send + Sync + 'static,
        Fut: std::future::Future<Output = azure_core::Result<RawResponse>> + Send + 'static,
    {
        // Get the appropriate retry policy based on the request
        let retry_policy = self.retry_policy_for_request(request);

        // Control-plane metadata reads are decoupled from caller cancellation:
        // dropping the caller's future must not preempt the retry policy's
        // cross-region failover decision (issue #4253). This is restricted to
        // idempotent metadata *reads* (see `is_detachable_metadata_read`); metadata
        // writes and all data-plane requests keep the in-line loop so that a dropped
        // caller still cancels in-flight work (important for non-idempotent writes).
        if is_detachable_metadata_read(request) {
            send_detached(
                retry_policy,
                request,
                sender,
                metadata_detached_hard_deadline(),
            )
            .await
        } else {
            drive_retry_loop(retry_policy, request, &sender, None).await
        }
    }
}

/// Drives the back-off retry loop for a single logical request.
///
/// Repeatedly invokes `sender`, consulting `retry_policy` after each attempt, until
/// the policy reports [`RetryResult::DoNotRetry`] (or `max_attempts`, when set, is
/// reached). The terminal partition-key-range Gone result is rewritten to a 503 when
/// the policy signals it.
async fn drive_retry_loop<Sender, Fut>(
    mut retry_policy: RetryPolicy,
    request: &mut CosmosRequest,
    sender: &Sender,
    max_attempts: Option<usize>,
) -> azure_core::Result<RawResponse>
where
    Sender: Fn(&mut CosmosRequest) -> Fut + Send + Sync,
    Fut: std::future::Future<Output = azure_core::Result<RawResponse>> + Send,
{
    let mut attempts = 0usize;

    loop {
        attempts += 1;
        retry_policy.before_send_request(request).await;

        // Log the endpoint URL being used for this request
        debug!(
            target: "azure_data_cosmos::retry_handler",
            "Sending request - endpoint: {:?}, region: {:?}, operation: {:?}, resource: {:?}",
            request.request_context.location_endpoint_to_route,
            request.request_context.region_name,
            request.operation_type,
            request.resource_type
        );

        // Invoke the provided sender callback instead of calling inner_send_async directly
        let result = sender(request).await;
        let retry_result = retry_policy.should_retry(&result).await;

        match retry_result {
            RetryResult::DoNotRetry => {
                // If the data-plane policy exhausted its partition-key-range Gone
                // budget, surface the terminal failure as 503 (preserving the
                // original sub-status) instead of letting a raw 410 escape.
                if let Some(sub_status) = retry_policy.terminal_gone_substatus() {
                    return convert_gone_to_service_unavailable(result, sub_status);
                }
                return result;
            }
            RetryResult::Retry { after } => {
                // Defensive cap: a misbehaving policy that always asks to retry with
                // zero backoff must not loop forever on a detached task.
                if let Some(cap) = max_attempts {
                    if attempts >= cap {
                        debug!(
                            target: "azure_data_cosmos::retry_handler",
                            "Detached metadata read hit the {cap}-attempt cap; surfacing the last result"
                        );
                        return result;
                    }
                }
                get_async_runtime().sleep(after).await;
            }
        }
    }
}

/// Runs the metadata retry loop on a detached background task so that dropping the
/// caller's future cannot preempt an in-flight cross-region failover.
///
/// The retry loop is spawned via the runtime abstraction (which detaches, rather than
/// aborts, when its handle is dropped) and its result is delivered back over a oneshot
/// channel. If the caller drops the returned future, the detached loop keeps running to
/// completion, so the retry policy's cross-region decision is always honored. The
/// detached work is bounded by [`metadata_detached_hard_deadline`] and
/// [`METADATA_DETACHED_ATTEMPT_CAP`] so it can never leak indefinitely.
async fn send_detached<Sender, Fut>(
    retry_policy: RetryPolicy,
    request: &mut CosmosRequest,
    sender: Sender,
    deadline: Duration,
) -> azure_core::Result<RawResponse>
where
    Sender: Fn(&mut CosmosRequest) -> Fut + Send + Sync + 'static,
    Fut: std::future::Future<Output = azure_core::Result<RawResponse>> + Send + 'static,
{
    let mut detached_request = request.clone();
    // Snapshot used only to write something coherent back if the hard deadline trips.
    let deadline_request = request.clone();

    let (tx, rx) = futures::channel::oneshot::channel();

    let task = async move {
        let loop_fut = async move {
            let result = drive_retry_loop(
                retry_policy,
                &mut detached_request,
                &sender,
                Some(METADATA_DETACHED_ATTEMPT_CAP),
            )
            .await;
            (result, detached_request)
        };
        let deadline_fut = get_async_runtime().sleep(deadline);

        futures::pin_mut!(loop_fut);
        let outcome = match futures::future::select(loop_fut, deadline_fut).await {
            Either::Left((completed, _)) => completed,
            // The hard deadline tripped while the loop was still in flight. The
            // detached loop is abandoned here; we write back the pre-spawn snapshot
            // (not its in-flight, possibly-mutated state) since this is a rare
            // defensive backstop and the caller only receives an error anyway.
            Either::Right(((), _)) => (
                Err(metadata_detached_deadline_error(deadline)),
                deadline_request,
            ),
        };

        // The receiver is gone when the caller dropped its future; the loop still ran
        // to completion (the point of detaching), so dropping the result is fine.
        let _ = tx.send(outcome);
    };

    // Dropping this handle detaches (does not abort) the task, so the loop continues
    // even after the caller's future is dropped.
    let _detached = get_async_runtime().spawn(Box::pin(task));

    match rx.await {
        Ok((result, completed_request)) => {
            *request = completed_request;
            result
        }
        // The detached task always sends before finishing, so a canceled receiver
        // means the task ended without producing a result: it panicked (the runtime's
        // default panic hook will have surfaced the backtrace) or the runtime is
        // shutting down. Degrade gracefully rather than propagating a panic into the
        // caller's task.
        Err(_canceled) => Err(azure_core::Error::with_message(
            ErrorKind::Other,
            "detached metadata read task ended without producing a result \
             (background task panicked or the runtime is shutting down)",
        )),
    }
}

/// Builds the error surfaced when a detached metadata read exceeds its hard deadline.
fn metadata_detached_deadline_error(deadline: Duration) -> azure_core::Error {
    azure_core::Error::with_message(
        ErrorKind::Other,
        format!(
            "detached metadata read exceeded its hard deadline of {} seconds",
            deadline.whole_seconds()
        ),
    )
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::cosmos_request::CosmosRequest;
    use crate::models::{AccountProperties, AccountRegion};
    use crate::operation_context::OperationType;
    use crate::regions::RegionName;
    use crate::resource_context::{ResourceLink, ResourceType};
    use crate::routing::global_endpoint_manager::GlobalEndpointManager;
    use crate::routing::global_partition_endpoint_manager::GlobalPartitionEndpointManager;
    use azure_core::error::ErrorKind;
    use azure_core::http::{headers::Headers, ClientOptions, Pipeline, StatusCode};
    use std::sync::atomic::{AtomicBool, AtomicUsize, Ordering};
    use std::time::Duration;
    use url::Url;

    const REGION_A: &str = "East US 2";
    const REGION_B: &str = "Central US";

    fn pipeline() -> Pipeline {
        Pipeline::new(
            option_env!("CARGO_PKG_NAME"),
            option_env!("CARGO_PKG_VERSION"),
            ClientOptions::default(),
            Vec::new(),
            Vec::new(),
            None,
        )
    }

    /// Builds a retry handler whose endpoint manager has two read regions
    /// (so the metadata retry policy will fail over from region A to region B)
    /// and whose account-properties cache is pre-seeded (so `before_send_request`
    /// never makes a live network call).
    async fn build_handler() -> BackOffRetryHandler {
        let endpoint = Url::parse("https://test.documents.azure.com/").unwrap();
        let gem = Arc::new(GlobalEndpointManager::new(
            endpoint,
            vec![RegionName::from(REGION_A), RegionName::from(REGION_B)],
            vec![],
            pipeline(),
        ));

        let regions = vec![
            AccountRegion {
                name: RegionName::from(REGION_A),
                database_account_endpoint: "https://test-a.documents.azure.com/".parse().unwrap(),
            },
            AccountRegion {
                name: RegionName::from(REGION_B),
                database_account_endpoint: "https://test-b.documents.azure.com/".parse().unwrap(),
            },
        ];
        gem.update_location_cache(regions.clone(), regions.clone());
        gem.seed_account_properties_cache(AccountProperties {
            writable_locations: regions.clone(),
            readable_locations: regions,
        })
        .await;

        let gpem = GlobalPartitionEndpointManager::new(gem.clone(), false, false);
        BackOffRetryHandler::new(gem, gpem)
    }

    /// A cold control-plane metadata read (ReadCollection-equivalent).
    fn metadata_request() -> CosmosRequest {
        let link = ResourceLink::root(ResourceType::Databases)
            .item("db")
            .feed(ResourceType::Containers)
            .item("c");
        let request = CosmosRequest::builder(OperationType::Read, link)
            .build()
            .unwrap();
        assert!(
            request.resource_type.is_meta_data(),
            "the proof scenario requires a metadata request"
        );
        request
    }

    fn service_unavailable() -> azure_core::Error {
        azure_core::Error::new(
            ErrorKind::HttpResponse {
                status: StatusCode::ServiceUnavailable,
                error_code: Some("ServiceUnavailable".to_string()),
                raw_response: None,
            },
            "region A is unhealthy",
        )
    }

    fn ok_response() -> RawResponse {
        RawResponse::from_bytes(StatusCode::Ok, Headers::new(), Vec::new())
    }

    /// Builds a two-region failover sender: the first attempt (region A) sleeps for
    /// `region_a_delay` then fails with a retryable 503; every later attempt (the
    /// cross-region failover to region B) records that it ran and succeeds.
    ///
    /// Returns the sender together with the attempt counter and the
    /// "region B reached" flag so tests can observe what actually executed.
    #[allow(clippy::type_complexity)]
    fn failover_sender(
        region_a_delay: Duration,
    ) -> (
        impl Fn(
                &mut CosmosRequest,
            ) -> std::pin::Pin<
                Box<dyn std::future::Future<Output = azure_core::Result<RawResponse>> + Send>,
            > + Send
            + Sync
            + 'static,
        Arc<AtomicUsize>,
        Arc<AtomicBool>,
    ) {
        let attempts = Arc::new(AtomicUsize::new(0));
        let region_b_reached = Arc::new(AtomicBool::new(false));

        let sender = {
            let attempts = attempts.clone();
            let region_b_reached = region_b_reached.clone();
            move |_req: &mut CosmosRequest| {
                let attempt = attempts.fetch_add(1, Ordering::SeqCst);
                let region_b_reached = region_b_reached.clone();
                Box::pin(async move {
                    if attempt == 0 {
                        // Region A is slow to fail (control-plane timeout escalation).
                        tokio::time::sleep(region_a_delay).await;
                        Err(service_unavailable())
                    } else {
                        // Region B (the cross-region failover target) succeeds.
                        region_b_reached.store(true, Ordering::SeqCst);
                        Ok(ok_response())
                    }
                })
                    as std::pin::Pin<
                        Box<
                            dyn std::future::Future<Output = azure_core::Result<RawResponse>>
                                + Send,
                        >,
                    >
            }
        };

        (sender, attempts, region_b_reached)
    }

    /// Regression test for issue #4253.
    ///
    /// When the caller future is dropped while the first metadata attempt (against the
    /// unhealthy region A) is still in flight, the detached executor keeps running, so
    /// the cross-region failover to region B is still attempted. The caller observes the
    /// cancellation, but the failover is no longer preempted.
    #[tokio::test]
    async fn caller_drop_does_not_preempt_cross_region_metadata_failover() {
        let handler = build_handler().await;
        let (sender, attempts, region_b_reached) = failover_sender(Duration::from_millis(200));

        let mut request = metadata_request();
        let outcome = tokio::time::timeout(
            Duration::from_millis(50),
            handler.send(&mut request, sender),
        )
        .await;

        assert!(
            outcome.is_err(),
            "the caller's timeout must fire and drop its view of the send future"
        );

        // The detached task continues past the caller's drop; give it time to fail over.
        tokio::time::sleep(Duration::from_millis(500)).await;

        assert_eq!(
            attempts.load(Ordering::SeqCst),
            2,
            "both the region-A attempt and the region-B failover must have executed"
        );
        assert!(
            region_b_reached.load(Ordering::SeqCst),
            "cross-region failover to region B must execute even though the caller was dropped"
        );
    }

    /// A caller that waits for the result receives the successful cross-region
    /// failover response, and the final (mutated) request state is written back.
    #[tokio::test]
    async fn patient_caller_receives_failover_result() {
        let handler = build_handler().await;
        let (sender, attempts, region_b_reached) = failover_sender(Duration::from_millis(10));

        let mut request = metadata_request();
        let result = handler.send(&mut request, sender).await;

        assert!(
            result.is_ok(),
            "the caller should receive the successful failover response"
        );
        assert_eq!(result.unwrap().status(), StatusCode::Ok);
        assert_eq!(attempts.load(Ordering::SeqCst), 2);
        assert!(region_b_reached.load(Ordering::SeqCst));
    }

    /// A metadata read that succeeds on the first attempt is returned to the caller
    /// without any failover.
    #[tokio::test]
    async fn successful_metadata_read_returns_to_caller() {
        let handler = build_handler().await;
        let attempts = Arc::new(AtomicUsize::new(0));

        let sender = {
            let attempts = attempts.clone();
            move |_req: &mut CosmosRequest| {
                attempts.fetch_add(1, Ordering::SeqCst);
                async move { Ok(ok_response()) }
            }
        };

        let mut request = metadata_request();
        let result = handler.send(&mut request, sender).await;

        assert_eq!(result.unwrap().status(), StatusCode::Ok);
        assert_eq!(attempts.load(Ordering::SeqCst), 1);
    }

    /// Data-plane requests are NOT detached: dropping the caller future cancels the
    /// in-flight work, so no background cross-region attempt is made. This keeps the
    /// fix scoped to idempotent control-plane metadata reads.
    #[tokio::test]
    async fn data_plane_request_is_not_detached() {
        let handler = build_handler().await;
        let (sender, attempts, region_b_reached) = failover_sender(Duration::from_millis(200));

        // A document read is a data-plane request, not metadata.
        let link = ResourceLink::root(ResourceType::Databases)
            .item("db")
            .feed(ResourceType::Containers)
            .item("c")
            .feed(ResourceType::Documents)
            .item("doc");
        let mut request = CosmosRequest::builder(OperationType::Read, link)
            .build()
            .unwrap();
        assert!(!request.resource_type.is_meta_data());

        let outcome = tokio::time::timeout(
            Duration::from_millis(50),
            handler.send(&mut request, sender),
        )
        .await;
        assert!(outcome.is_err(), "the caller's timeout must fire");

        tokio::time::sleep(Duration::from_millis(500)).await;

        assert_eq!(
            attempts.load(Ordering::SeqCst),
            1,
            "the in-flight data-plane attempt is dropped with the caller; no retry runs"
        );
        assert!(
            !region_b_reached.load(Ordering::SeqCst),
            "data-plane requests must not continue on a detached task after caller drop"
        );
    }

    /// Metadata *writes* (e.g. create container) must NOT be detached: detaching a
    /// non-idempotent write could double-apply it if the caller drops and retries.
    /// Dropping the caller therefore cancels the in-flight attempt with no background
    /// continuation.
    #[tokio::test]
    async fn metadata_write_is_not_detached() {
        let handler = build_handler().await;
        let (sender, attempts, region_b_reached) = failover_sender(Duration::from_millis(200));

        // Creating a container is a metadata resource type but a mutating operation.
        let link = ResourceLink::root(ResourceType::Databases)
            .item("db")
            .feed(ResourceType::Containers);
        let mut request = CosmosRequest::builder(OperationType::Create, link)
            .build()
            .unwrap();
        assert!(
            request.resource_type.is_meta_data(),
            "container create is a metadata resource type"
        );
        assert!(
            !is_detachable_metadata_read(&request),
            "a mutating metadata operation must not be treated as a detachable read"
        );

        let outcome = tokio::time::timeout(
            Duration::from_millis(50),
            handler.send(&mut request, sender),
        )
        .await;
        assert!(outcome.is_err(), "the caller's timeout must fire");

        tokio::time::sleep(Duration::from_millis(500)).await;

        assert_eq!(
            attempts.load(Ordering::SeqCst),
            1,
            "the in-flight metadata write is dropped with the caller; no retry runs"
        );
        assert!(
            !region_b_reached.load(Ordering::SeqCst),
            "metadata writes must not continue on a detached task after caller drop"
        );
    }

    /// When the detached hard deadline trips before the loop completes, the caller
    /// receives a deadline error rather than hanging on a stuck attempt.
    #[tokio::test]
    async fn detached_metadata_read_honors_hard_deadline() {
        let handler = build_handler().await;
        let retry_policy = handler.retry_policy_for_request(&metadata_request());

        let attempts = Arc::new(AtomicUsize::new(0));
        let sender = {
            let attempts = attempts.clone();
            move |_req: &mut CosmosRequest| {
                attempts.fetch_add(1, Ordering::SeqCst);
                async move {
                    // Never completes within the deadline below.
                    tokio::time::sleep(Duration::from_secs(30)).await;
                    Ok(ok_response())
                }
            }
        };

        let mut request = metadata_request();
        let result = send_detached(
            retry_policy,
            &mut request,
            sender,
            azure_core::time::Duration::milliseconds(50),
        )
        .await;

        let err = result.expect_err("the hard deadline must surface as an error");
        assert!(
            err.to_string().contains("hard deadline"),
            "deadline error should mention the hard deadline, got: {err}"
        );
    }

    /// Mirrors the .NET clamp fix: the default applies when the value is absent,
    /// an out-of-range value is clamped to the 24h maximum, and invalid/zero values
    /// fall back to the default so the timer can never be constructed with an
    /// overflowing duration.
    ///
    /// Tests the pure resolver directly so it never mutates the process-global
    /// environment (which would race other tests that read the deadline).
    #[test]
    fn hard_deadline_resolution_and_clamping() {
        assert_eq!(
            resolve_hard_deadline_secs(None),
            DEFAULT_METADATA_DETACHED_HARD_DEADLINE_SECS,
            "absent value should use the default"
        );

        // 60 days, well over the 24h clamp.
        assert_eq!(
            resolve_hard_deadline_secs(Some("5184000")),
            MAX_METADATA_DETACHED_HARD_DEADLINE_SECS,
            "an oversized value must be clamped to the 24h maximum"
        );

        // A valid in-range value is honored as-is.
        assert_eq!(resolve_hard_deadline_secs(Some("42")), 42);

        // Surrounding whitespace is tolerated.
        assert_eq!(resolve_hard_deadline_secs(Some("  600 ")), 600);

        // Zero is not a usable deadline; fall back to the default.
        assert_eq!(
            resolve_hard_deadline_secs(Some("0")),
            DEFAULT_METADATA_DETACHED_HARD_DEADLINE_SECS
        );

        // Non-numeric values fall back to the default.
        assert_eq!(
            resolve_hard_deadline_secs(Some("not-a-number")),
            DEFAULT_METADATA_DETACHED_HARD_DEADLINE_SECS
        );
    }
}

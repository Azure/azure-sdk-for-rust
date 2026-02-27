# Transport & Pipeline Refactoring Spec for `azure_data_cosmos_driver`

**Status**: Draft / Iterating
**Date**: 2026-02-27
**Authors**: (team)

---

## Table of Contents

1. [Goals & Motivation](#1-goals--motivation)
2. [Architectural Overview](#2-architectural-overview)
3. [Component Design: ECS-Inspired Pipeline State Model](#3-component-design-ecs-inspired-pipeline-state-model)
4. [Operation Pipeline](#4-operation-pipeline)
5. [Transport Pipeline](#5-transport-pipeline)
6. [HTTP/2 Connection Sharding Layer](#6-http2-connection-sharding-layer)
7. [Fault Injection Integration](#7-fault-injection-integration)
8. [Migration Plan (Step 1 → Step 2)](#8-migration-plan-step-1--step-2)
9. [Configuration Surface](#9-configuration-surface)
10. [Open Questions](#10-open-questions)

---

## 1. Goals & Motivation

### Why

The current `azure_data_cosmos` crate implements retry/failover/routing/pipeline logic that will
eventually live in `azure_data_cosmos_driver`. The goal is:

1. **Step 1**: Build the complete pipeline/transport stack in `azure_data_cosmos_driver` with a clean
   DOP-inspired architecture, making `execute_operation` the single entry point for all operation
   execution (including multi-region failover, hedging, partition-level circuit breaker, throttle
   retry, session consistency routing, and fault injection).

2. **Step 2**: Cut over `azure_data_cosmos` to call `driver.execute_operation()` instead of its own
   pipeline, then remove the duplicate logic from `azure_data_cosmos`.

### Design Principles

- **Data-Oriented Programming**: Follow the [DOP-in-Rust](https://analogrelay.github.io/dop-in-rust)
  "HTTP Pipeline, but with ECS flair" pattern. State is decomposed into focused component types.
  Pipeline stages are pure-ish functions over those components. No god-object mutable context.
- **Separate Operation pipeline from Transport pipeline**: Two distinct layers with different retry
  scopes and responsibilities.
- **Testability first**: Every pipeline stage function has a narrow signature
  (`fn(&ComponentA, &ComponentB) -> TransformedComponent`), making unit testing trivial without
  mocking.
- **No mutable context threading**: Instead of passing a `&mut RequestContext` that any policy can
  mutate in any order, state flows through well-typed transformations. Old state is consumed, new
  state replaces it.
- **Use `azure_core` abstractions**: The driver depends on `azure_core::http::HttpClient` trait and
  `azure_core::async_runtime` for pluggable transport and async runtime. No direct dependency on
  `reqwest` or `tokio` in production code — only in `dev-dependencies` for tests. This ensures
  future `HttpClient`/runtime implementations don't require a rewrite. Follows the same model
  already used by `azure_data_cosmos`.

---

## 2. Architectural Overview

```text
┌─────────────────────────────────────────────────────────────────────────────┐
│                      CosmosDriver.execute_operation()                       │
│                                                                             │
│  ┌───────────────────────────────────────────────────────────────────────┐  │
│  │                          OPERATION PIPELINE                           │  │
│  │                                                                       │  │
│  │  Responsibilities:                                                    │  │
│  │  ┌─ Region selection (AccountEndpointState + resolve_endpoint)        │  │
│  │  ├─ Hedging (parallel speculative execution in secondary region)      │  │
│  │  ├─ Cross-region failover (503/WriteForbidden/SessionNotAvailable)    │  │
│  │  ├─ 403.3 recovery (refresh AccountMetadataCache, rate-limited)       │  │
│  │  ├─ Partition-level circuit breaker (PPAF/PPCB)                       │  │
│  │  ├─ Session token resolution                                          │  │
│  │  └─ Operation-level diagnostics aggregation                           │  │
│  │                                                                       │  │
│  │  Input:  CosmosOperation + OperationOptions                           │  │
│  │  Output: CosmosResponse (includes DiagnosticsContext)                 │  │
│  │                                                                       │  │
│  │          ┌─────────────────┐   ┌─────────────────┐                    │  │
│  │          │  Attempt to     │   │ Hedged attempt  │                    │  │
│  │          │  Region A       │   │ to Region B     │  (optional)        │  │
│  │          └────────┬────────┘   └────────┬────────┘                    │  │
│  │                   │                     │                             │  │
│  └───────────────────┼─────────────────────┼─────────────────────────────┘  │
│                      ▼                     ▼                                │
│  ┌───────────────────────────────────────────────────────────────────────┐  │
│  │                          TRANSPORT PIPELINE                           │  │
│  │                                                                       │  │
│  │  Responsibilities (per-attempt, single region/endpoint):              │  │
│  │  ┌─ Transport-level retry (429 with backoff)                          │  │
│  │  ├─ Authorization header generation                                   │  │
│  │  ├─ Common headers (x-ms-version, User-Agent, Content-Type)           │  │
│  │  ├─ Request/response diagnostics capture (per-attempt events)         │  │
│  │  ├─ Request-sent-status tracking (for retry safety)                   │  │
│  │  └─ End-to-end deadline enforcement                                   │  │
│  │                                                                       │  │
│  │  Input:  TransportRequest (= operation snapshot + resolved endpoint)  │  │
│  │  Output: TransportResult (= raw response + attempt diagnostics)       │  │
│  │                                                                       │  │
│  └───────────────────────────────────────────────────────────────────────┘  │
│                      │                                                      │
│                      ▼                                                      │
│  ┌───────────────────────────────────────────────────────────────────────┐  │
│  │                     ADAPTIVE HTTP TRANSPORT LAYER                     │  │
│  │                                                                       │  │
│  │  Negotiated at client init based on gateway type:                     │  │
│  │  ┌─ HTTP/2 (ComputeGateway or Gateway 2.0):                           │  │
│  │  │    ShardedHttpTransport (see §6)                                   │  │
│  │  │    ┌─ Tracks inflight per HttpClient shard per endpoint            │  │
│  │  │    ├─ Load-balances across multiple HttpClient instances           │  │
│  │  │    ├─ Health monitoring per shard (consecutive failures)           │  │
│  │  │    └─ Proactive eviction of unhealthy shards                       │  │
│  │  └─ HTTP/1.1 (RoutingGateway):                                        │  │
│  │       Single HttpClient (no sharding needed)                          │  │
│  │                                                                       │  │
│  │  Uses azure_core::http::HttpClient trait (pluggable transport)        │  │
│  │  Input:  azure_core::http::Request                                    │  │
│  │  Output: azure_core::http::AsyncRawResponse                           │  │
│  └───────────────────────────────────────────────────────────────────────┘  │
│                      │                                                      │
│                      ▼                                                      │
│         [Arc<dyn HttpClient>]  (reqwest / future pluggable impl)            │
└─────────────────────────────────────────────────────────────────────────────┘
```

---

## 3. Component Design: ECS-Inspired Pipeline State Model

Following the DOP principle of "separate data from behavior," the pipeline state is decomposed into
focused component types. Each pipeline stage operates on only the components it needs.

### 3.1 Operation-Level Components

```rust
/// COMPONENT: Immutable snapshot of the operation intent.
/// Created once, never mutated during pipeline execution.
/// Already exists as `CosmosOperation`.
// (reuse existing CosmosOperation)

/// COMPONENT: Routing decision for the current attempt.
/// Produced by endpoint resolution, consumed by transport.
/// Replaced (not mutated) on retry/failover.
pub(crate) struct RoutingDecision {
    /// The resolved Cosmos DB service endpoint.
    pub endpoint: CosmosEndpoint,
    /// Whether partition-level override was applied.
    pub partition_override: Option<PartitionOverride>,
}

pub(crate) struct PartitionOverride {
    pub partition_key_range_id: String,
    pub original_region: RegionName,
    pub override_region: RegionName,
}

/// COMPONENT: Operation-level retry/failover state.
/// Accumulated across attempts. Replaced (not mutated) between attempts.
pub(crate) struct OperationRetryState {
    /// Index into the preferred endpoint list for round-robin failover.
    /// This indexes into an immutable `LocationSnapshot` (see §4.4).
    /// The snapshot is replaced atomically on refresh — it is never mutated
    /// in place — so the index remains stable for the lifetime of one
    /// operation attempt.
    pub location_index: usize,
    /// Regions/endpoints that have already been tried and failed.
    pub failed_endpoints: Vec<FailedEndpoint>,
    /// How many failover retries have been attempted.
    pub failover_retry_count: u32,
    /// Session token retry attempts.
    pub session_token_retry_count: u32,
    /// Whether multi-write locations are available.
    pub can_use_multiple_write_locations: bool,
    /// Excluded regions for this operation.
    pub excluded_regions: Vec<RegionName>,
}

pub(crate) struct FailedEndpoint {
    /// The endpoint that failed.
    pub endpoint: CosmosEndpoint,
    pub error: OperationError,
    pub request_sent: bool,
}

/// COMPONENT: Diagnostics accumulator for the operation.
/// Append-only across attempts.
// Reuse existing DiagnosticsContextBuilder (already mutable accumulator).

/// Newtype wrapper for a Cosmos DB Log Sequence Number (LSN).
///
/// LSNs are monotonically increasing 64-bit integers assigned by the
/// storage engine to each committed write within a partition. They are
/// used for session-consistency tracking, quorum reads, and
/// change-feed continuation.
#[derive(Clone, Copy, Debug, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub(crate) struct LogSequenceNumber(i64);

impl LogSequenceNumber {
    pub fn new(value: i64) -> Self {
        Self(value)
    }

    pub fn value(self) -> i64 {
        self.0
    }
}

impl std::fmt::Display for LogSequenceNumber {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.0)
    }
}

/// COMPONENT: Session state for consistency tracking.
pub(crate) struct SessionState {
    pub session_token: Option<SessionToken>,
    /// The effective read consistency strategy for this operation, taking
    /// into account the account-level default, any request-level override,
    /// and driver-level policies (e.g., session-read-from-write-region).
    pub effective_read_consistency_strategy: ReadConsistencyStrategy,
    /// LSN tracking for session consistency.
    pub quorum_selected_lsn: Option<LogSequenceNumber>,
    pub global_committed_selected_lsn: Option<LogSequenceNumber>,
}
```

### 3.2 Transport-Level Components

```rust
/// COMPONENT: A single transport attempt's identity and config.
/// Produced by the operation pipeline for each attempt.
pub(crate) struct TransportRequest {
    /// The HTTP method.
    pub method: Method,
    /// The fully resolved URL for this attempt.
    pub url: Url,
    /// Headers to send (includes any attempt-specific headers).
    pub headers: Headers,
    /// Request body bytes (schema-agnostic).
    pub body: Option<Bytes>,
    /// Authorization context for signing.
    pub auth_context: AuthorizationContext,
    /// The execution context (Initial/Retry/Hedging/Failover).
    pub execution_context: ExecutionContext,
    /// End-to-end deadline for the overall operation.
    /// Enforced in the transport pipeline by:
    /// 1. Checking `Instant::now() >= deadline` before each attempt.
    /// 2. Setting the per-request timeout on the underlying `HttpClient`
    ///    to `deadline - Instant::now()` (clamped to a minimum of 1ms)
    ///    so that the HTTP layer itself cancels the I/O if the deadline
    ///    is reached mid-flight.
    /// 3. On timeout, the response stream is dropped, which closes the
    ///    underlying HTTP/2 stream or TCP connection.
    pub deadline: Option<Instant>,
}

/// COMPONENT: Transport-level retry state (for 429 throttling and
/// connectivity errors that can be retried locally by switching to a
/// different HttpClient shard).
///
/// Scoped to a single attempt (including any regional failover within
/// that attempt from `ClientRetryPolicy`). When hedging is active,
/// each hedged request maintains its own independent `ThrottleRetryState`
/// to avoid one attempt's throttling causing the other to fail.
pub(crate) struct ThrottleRetryState {
    pub attempt_count: u32,
    pub max_attempts: u32,
    pub cumulative_delay: Duration,
    pub max_wait_time: Duration,
    pub backoff_factor: f64,
}

/// COMPONENT: Result of a single transport attempt.
/// Returned from the transport pipeline to the operation pipeline.
pub(crate) struct TransportResult {
    /// The outcome of this attempt.
    pub outcome: TransportOutcome,
    /// Per-attempt diagnostics.
    pub diagnostics: RequestDiagnostics,
}

pub(crate) enum TransportOutcome {
    /// Successful response.
    Success {
        status: CosmosStatus,
        headers: Headers,
        body: Vec<u8>,
    },
    /// Failed with an HTTP error that may be retryable at operation level.
    HttpError {
        status: CosmosStatus,
        headers: Headers,
        body: Vec<u8>,
        request_sent: RequestSentStatus,
    },
    /// Failed with a transport/connection error.
    TransportError {
        error: azure_core::Error,
        request_sent: RequestSentStatus,
    },
}
```

### 3.3 Pipeline Stage Functions (DOP Style)

Each stage is a function with an explicit, narrow signature:

```rust
// SYSTEM: Resolve which endpoint to send this attempt to.
// Pure function over immutable state snapshots — no manager references.
fn resolve_endpoint(
    operation: &CosmosOperation,
    retry_state: &OperationRetryState,
    account_state: &AccountEndpointState,
    partition_state: &PartitionEndpointState,
) -> RoutingDecision;

// SYSTEM: Evaluate the result of a transport attempt and decide the
// next OperationAction (complete, retry, failover, hedge, or abort).
fn evaluate_transport_result(
    operation: &CosmosOperation,
    result: &TransportResult,
    retry_state: &OperationRetryState,
    session_state: &SessionState,
) -> OperationAction;

// SYSTEM: Decide whether to retry a 429 or connectivity error at transport level.
// Extracted as a pure function (like `evaluate_transport_result`) so it can be
// unit-tested in isolation. Covers 429 backoff as well as transient I/O /
// timeout errors that can be retried on a different HttpClient shard.
fn evaluate_transport_retry(
    result: &TransportResult,
    throttle_state: &ThrottleRetryState,
) -> ThrottleAction;

// SYSTEM: Build the TransportRequest from operation + routing decision.
fn build_transport_request(
    operation: &CosmosOperation,
    routing: &RoutingDecision,
    session: &SessionState,
    options: &OperationOptions,
    deadline: Option<Instant>,
) -> TransportRequest;
```

### 3.4 Decision Enums

```rust
/// What the operation pipeline should do after an attempt.
pub(crate) enum OperationAction {
    /// Return the successful response.
    Complete(TransportResult),
    /// Retry in a different region. Contains the new retry state.
    FailoverRetry {
        new_state: OperationRetryState,
        delay: Option<Duration>,
    },
    /// Retry for session consistency (ReadSessionNotAvailable).
    SessionRetry {
        new_state: OperationRetryState,
    },
    /// Mark partition unavailable and retry with partition-level override.
    PartitionFailover {
        new_state: OperationRetryState,
        /// Identifies the partition + region to mark unavailable.
        unavailable_partition: UnavailablePartition,
    },
    /// Start a hedged attempt in a secondary region.
    Hedge {
        secondary_routing: RoutingDecision,
    },
    /// Abort the operation with this error.
    Abort(azure_core::Error),
}

/// What the transport pipeline should do after a 429.
pub(crate) enum ThrottleAction {
    /// Retry after a delay.
    Retry {
        delay: Duration,
        new_state: ThrottleRetryState,
    },
    /// Do not retry; propagate to operation pipeline.
    Propagate,
}
```

---

## 4. Operation Pipeline

### 4.1 Core Loop

The operation pipeline orchestrates attempts across regions. It is the replacement for
`ClientRetryPolicy` + `MetadataRequestRetryPolicy` + `BackOffRetryHandler` from `azure_data_cosmos`.

```rust
pub(crate) async fn execute_operation_pipeline(
    operation: &CosmosOperation,
    options: &OperationOptions,
    account_state_store: &AccountEndpointStateStore,
    partition_state_store: &PartitionEndpointStateStore,
    transport: &TransportPipeline,
    diagnostics: &mut DiagnosticsContextBuilder,
) -> Result<CosmosResponse> {
    let mut retry_state = OperationRetryState::initial(
        operation, options, &account_state_store.snapshot(),
    );
    let mut session_state = SessionState::from_operation(operation, options);
    let deadline = options.e2e_deadline();

    loop {
        // Acquire immutable snapshots of endpoint state for this attempt.
        // Each snapshot is an Arc clone — cheap, lock-free, and stable
        // for the lifetime of the attempt.
        let account_state = account_state_store.snapshot();
        let partition_state = partition_state_store.snapshot();

        // STAGE 1: Resolve endpoint for this attempt
        let routing = resolve_endpoint(
            operation,
            &retry_state,
            &account_state,
            &partition_state,
        );

        // STAGE 2: Build transport request
        let transport_request = build_transport_request(
            operation,
            &routing,
            &session_state,
            options,
            deadline,
        );

        // STAGE 3: Execute via transport pipeline
        let result = transport.execute(transport_request, diagnostics).await;

        // STAGE 4: Record attempt diagnostics
        diagnostics.record_attempt(&result);

        // STAGE 5: Evaluate result and decide next action
        let action = evaluate_transport_result(
            operation,
            &result,
            &retry_state,
            &session_state,
        );

        match action {
            OperationAction::Complete(result) => {
                return build_cosmos_response(result, diagnostics);
            }
            OperationAction::FailoverRetry { new_state, delay } => {
                if let Some(d) = delay { sleep(d).await; }
                retry_state = new_state;
            }
            OperationAction::SessionRetry { new_state } => {
                retry_state = new_state;
            }
            OperationAction::PartitionFailover { new_state, unavailable_partition } => {
                // SYSTEM: Apply partition failure — produces a new
                // PartitionEndpointState snapshot and swaps it into the store.
                let new_partition_state = mark_partition_unavailable(
                    &partition_state,
                    &unavailable_partition,
                );
                partition_state_store.swap(new_partition_state);
                retry_state = new_state;
            }
            OperationAction::Hedge { secondary_routing } => {
                // See §4.2 Hedging
                return execute_hedged(
                    operation, options, &routing, &secondary_routing,
                    &session_state, transport, diagnostics, deadline,
                ).await;
            }
            OperationAction::Abort(error) => {
                return Err(error);
            }
        }
    }
}
```

### 4.2 Hedging (Speculative Execution)

Hedging spawns a parallel attempt in a secondary region when the primary attempt has not responded
within a threshold. This is the Rust implementation of `ThresholdBasedAvailabilityStrategy` from the
Java SDK.

**Design**:

```rust
async fn execute_hedged(
    operation: &CosmosOperation,
    options: &OperationOptions,
    primary_routing: &RoutingDecision,
    secondary_routing: &RoutingDecision,
    session: &SessionState,
    transport: &TransportPipeline,
    diagnostics: &mut DiagnosticsContextBuilder,
    deadline: Option<Instant>,
) -> Result<CosmosResponse> {
    let primary_req = build_transport_request(
        operation, primary_routing, session, options, deadline,
    );

    let hedging_threshold = options.hedging_threshold(); // e.g. 50ms

    // Start primary attempt
    let primary_fut = transport.execute(primary_req, diagnostics);

    // Race: primary completes within threshold, or we start secondary
    tokio::select! {
        result = primary_fut => {
            // Primary completed (success or error) before hedging threshold
            diagnostics.record_attempt(&result);
            evaluate_and_return(operation, result, diagnostics)
        }
        _ = sleep(hedging_threshold) => {
            // Primary still pending - start secondary
            let secondary_req = build_transport_request(
                operation, secondary_routing, session, options, deadline,
            ).with_execution_context(ExecutionContext::Hedging);

            let secondary_fut = transport.execute(secondary_req, diagnostics);

            // Race primary vs secondary - first successful response wins
            // (or first to complete if both error)
            tokio::select! {
                primary_result = primary_fut => {
                    diagnostics.record_primary_attempt(&primary_result);
                    if primary_result.is_success() {
                        // Cancel secondary (drop future)
                        evaluate_and_return(operation, primary_result, diagnostics)
                    } else {
                        // Wait for secondary
                        let secondary_result = secondary_fut.await;
                        diagnostics.record_hedged_attempt(&secondary_result);
                        pick_best_result(primary_result, secondary_result, diagnostics)
                    }
                }
                secondary_result = secondary_fut => {
                    diagnostics.record_hedged_attempt(&secondary_result);
                    if secondary_result.is_success() {
                        // Cancel primary (drop future)
                        evaluate_and_return(operation, secondary_result, diagnostics)
                    } else {
                        // Wait for primary
                        let primary_result = primary_fut.await;
                        diagnostics.record_primary_attempt(&primary_result);
                        pick_best_result(primary_result, secondary_result, diagnostics)
                    }
                }
            }
        }
    }
}
```

**Key hedging rules**:
- Enabled by default for all operations. For write operations, hedging is only
  enabled on multi-write-region (MWR) accounts. An option to opt-out exists and
  is overridable at `DriverRuntime`, `Driver`, and per-operation levels.
- Threshold is **dynamic by default**, based on the observed P99 latency
  (preferring recent measurements for fast reaction). Hard-coded safety gates
  clamp the effective threshold to the range **50 ms – 4000 ms**; configurable
  min/max bounds can tighten this range further.
- The primary (first) attempt always uses `ExecutionContext::Initial` so that
  diagnostics can clearly distinguish whether hedging occurred. A hedged attempt
  uses `ExecutionContext::Hedging`. This makes it easy to identify concurrent
  execution caused by the original attempt not producing a terminal result
  within the configured threshold.
- Both attempts are tracked in `DiagnosticsContext`
- The hedged attempt's RU charge is always reported regardless of which wins

### 4.3 `evaluate_transport_result` Decision Tree

This pure function replaces the complex `ClientRetryPolicy::should_retry` method. Because it is a
pure function over data components, it is trivially unit-testable.

```rust
fn evaluate_transport_result(
    operation: &CosmosOperation,
    result: &TransportResult,
    retry_state: &OperationRetryState,
    session_state: &SessionState,
) -> OperationAction {
    match &result.outcome {
        TransportOutcome::Success { .. } => {
            OperationAction::Complete(result.clone())
        }

        TransportOutcome::HttpError { status, request_sent, .. } => {
            match *status {
                // 403/3 WriteForbidden → refresh AccountMetadataCache + endpoint failover
                // The account metadata cache is refreshed to obtain fresh account
                // properties (e.g., updated writable/readable locations). A rate
                // limiter ensures refreshes happen at most N times per minute to
                // avoid overwhelming the metadata endpoint during sustained
                // failover scenarios.
                CosmosStatus::WRITE_FORBIDDEN => {
                    if retry_state.failover_retry_count < MAX_FAILOVER_RETRIES {
                        OperationAction::FailoverRetry {
                            new_state: retry_state.advance_failover_with_cache_refresh(result),
                            delay: None,
                        }
                    } else {
                        OperationAction::Abort(build_error(status))
                    }
                }

                // 404/1002 ReadSessionNotAvailable → session retry
                CosmosStatus::READ_SESSION_NOT_AVAILABLE => {
                    if retry_state.session_token_retry_count < MAX_SESSION_RETRIES {
                        OperationAction::SessionRetry {
                            new_state: retry_state.advance_session_retry(),
                        }
                    } else {
                        OperationAction::Abort(build_error(status))
                    }
                }

                // 429/3092 SystemResourceUnavailable → treat as 503.
                // Also covers 503 (any sub-status) and 410 (any Gone sub-status).
                // These don't map to a single CosmosStatus constant, so we use
                // helper predicates on the status code + sub-status.
                _ if status.is_system_resource_unavailable()
                    || status.is_service_unavailable()
                    || status.is_gone() =>
                {
                    OperationAction::PartitionFailover {
                        new_state: retry_state.advance_partition_failover(result),
                    }
                }

                // 500 for reads → try next endpoint
                _ if status.is_internal_server_error() && operation.is_read_only() => {
                    OperationAction::FailoverRetry {
                        new_state: retry_state.advance_failover(result),
                        delay: None,
                    }
                }

                _ => OperationAction::Abort(build_error(status)),
            }
        }

        TransportOutcome::TransportError { request_sent, error, .. } => {
            match request_sent {
                RequestSentStatus::NotSent => {
                    // Safe to retry on any endpoint for any operation
                    if retry_state.failover_retry_count < MAX_CONNECTION_RETRIES {
                        OperationAction::FailoverRetry {
                            new_state: retry_state.advance_connection_retry(result),
                            delay: None,
                        }
                    } else {
                        OperationAction::Abort(error.clone())
                    }
                }
                _ if operation.is_read_only() || operation.is_idempotent() => {
                    OperationAction::FailoverRetry {
                        new_state: retry_state.advance_failover(result),
                        delay: None,
                    }
                }
                _ => OperationAction::Abort(error.clone()),
            }
        }
    }
}
```

### 4.4 Account Endpoint State & Routing Systems

Following the DOP principle, the account-level endpoint routing is split into:

- **Data (Component)**: `AccountEndpointState` — an immutable snapshot of everything the
  routing system needs to pick an endpoint. Replaced atomically on refresh.
- **Store (Infrastructure)**: `AccountEndpointStateStore` — a thin `RwLock<Arc<_>>` wrapper
  that provides thread-safe snapshot access and atomic swap.
- **Systems (Behavior)**: Pure functions that read state and produce `RoutingDecision`s, or
  take old state + an event and produce new state.

```rust
/// COMPONENT: Immutable snapshot of all account-level endpoint routing state.
///
/// Created from `AccountProperties` (fetched via `AccountMetadataCache`).
/// Replaced as a whole when account properties refresh — never mutated
/// in place. The operation pipeline clones an `Arc<AccountEndpointState>`
/// at the start of each attempt, giving it a stable view for the duration
/// of that attempt.
#[derive(Clone, Debug)]
pub(crate) struct AccountEndpointState {
    /// Ordered list of endpoints preferred for read operations, based on
    /// the account's configured preferred locations and available regions.
    pub preferred_read_endpoints: Vec<CosmosEndpoint>,
    /// Ordered list of endpoints preferred for write operations.
    pub preferred_write_endpoints: Vec<CosmosEndpoint>,
    /// Endpoints currently marked as unavailable, with the time they were
    /// marked and the reason. Used by `resolve_endpoint` to skip bad
    /// endpoints without mutating the snapshot.
    pub unavailable_endpoints: HashMap<Url, (Instant, UnavailableReason)>,
    /// Whether the account has multiple write regions enabled.
    pub multiple_write_locations_enabled: bool,
    /// The raw account properties from the last metadata fetch.
    pub account_properties: AccountProperties,
    /// Default (hub) endpoint — used as fallback when all regional
    /// endpoints are unavailable.
    pub default_endpoint: CosmosEndpoint,
}

/// Thread-safe holder for the current `AccountEndpointState` snapshot.
///
/// Read path: clone the `Arc` (lock-free via `RwLock::read`).
/// Write path: build a new `AccountEndpointState`, take the write lock,
/// and swap the `Arc`.
pub(crate) struct AccountEndpointStateStore {
    state: RwLock<Arc<AccountEndpointState>>,
}

impl AccountEndpointStateStore {
    /// Get a cheap clone of the current snapshot.
    pub fn snapshot(&self) -> Arc<AccountEndpointState> {
        Arc::clone(&self.state.read())
    }

    /// Atomically replace the current snapshot.
    pub fn swap(&self, new_state: AccountEndpointState) {
        *self.state.write() = Arc::new(new_state);
    }
}

/// A Cosmos DB service endpoint: region name, URL, and whether
/// the endpoint is global or regional.
///
/// This is the canonical representation used throughout routing,
/// failover tracking, and diagnostics.
///
/// - **Global** endpoints usually use the pattern `{account}.documents.azure.com`.
///   DNS resolves them to the hub (default) region.
/// - **Regional** endpoints usually use the pattern `{account}-{region}.documents.azure.com`
///   and resolve directly to that region.
#[derive(Clone, Debug)]
pub(crate) struct CosmosEndpoint {
    region: RegionName,
    url: Url,
    kind: EndpointKind,
}

/// Whether an endpoint targets a specific region or the global
/// (hub) entry point.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub(crate) enum EndpointKind {
    /// Global endpoint (`{account}.documents.azure.com`).
    /// DNS resolves to the hub region.
    Global,
    /// Regional endpoint (`{account}-{region}.documents.azure.com`).
    Regional,
}
```

**Systems (pure functions) operating on `AccountEndpointState`**:

```rust
// SYSTEM: Build an initial AccountEndpointState from account properties.
// Called at client init and on background metadata refresh.
fn build_account_endpoint_state(
    properties: &AccountProperties,
    preferred_locations: &[RegionName],
    default_endpoint: CosmosEndpoint,
) -> AccountEndpointState;

// SYSTEM: Produce a new state with an endpoint marked unavailable.
// The old state is not mutated — a new snapshot is returned.
fn mark_endpoint_unavailable(
    state: &AccountEndpointState,
    endpoint: &Url,
    reason: UnavailableReason,
) -> AccountEndpointState;

// SYSTEM: Produce a new state with expired unavailability entries removed.
// Called by the background health sweep.
fn expire_unavailable_endpoints(
    state: &AccountEndpointState,
    now: Instant,
    expiry_duration: Duration,
) -> AccountEndpointState;
```

**Key difference from previous design**: There is no `GlobalEndpointManager` struct with methods.
The `AccountEndpointState` is pure data, and each mutation is an explicit system function that
returns a new snapshot. The `AccountEndpointStateStore` is a trivial `RwLock<Arc<_>>` holder —
it has no routing logic. The `AccountMetadataCache` remains a separate infrastructure component
responsible for async-fetching and caching `AccountProperties`; on refresh, it calls
`build_account_endpoint_state` and swaps the result into the store.

This makes the read path lock-free (clone the `Arc`), keeps mutation explicit and traceable,
and means every system function can be unit-tested with a hand-constructed `AccountEndpointState`
— no mocking required.

### 4.5 Partition Endpoint State & Circuit-Breaker Systems

Same DOP split as §4.4: data component + state store + system functions.

```rust
/// COMPONENT: Immutable snapshot of partition-level circuit-breaker state.
///
/// Tracks which partitions in which regions are currently considered
/// unavailable and should be failed over to an alternate region.
/// Replaced atomically on mutation — never mutated in place.
#[derive(Clone, Debug)]
pub(crate) struct PartitionEndpointState {
    /// Partitions currently marked unavailable, keyed by
    /// (partition_key_range_id, region). Value is the circuit-breaker
    /// state for that partition/region pair.
    pub unavailable_partitions: HashMap<PartitionRegionKey, PartitionCircuitBreaker>,
    /// Circuit-breaker configuration thresholds.
    pub config: CircuitBreakerConfig,
}

/// Composite key for partition-level circuit-breaker tracking.
#[derive(Clone, Debug, PartialEq, Eq, Hash)]
pub(crate) struct PartitionRegionKey {
    pub partition_key_range_id: String,
    pub region: RegionName,
}

/// Per-partition circuit-breaker state.
#[derive(Clone, Debug)]
pub(crate) struct PartitionCircuitBreaker {
    /// When this partition was marked unavailable.
    pub marked_at: Instant,
    /// Consecutive read failure count.
    pub consecutive_read_failures: u32,
    /// Consecutive write failure count.
    pub consecutive_write_failures: u32,
    /// Last failure timestamp.
    pub last_failure: Instant,
    /// Whether the circuit is currently open (tripped).
    pub is_open: bool,
}

/// Identifies a partition + region to mark unavailable.
/// Carried in `OperationAction::PartitionFailover` so the operation
/// pipeline can apply the mutation without coupling to internals.
#[derive(Clone, Debug)]
pub(crate) struct UnavailablePartition {
    pub partition_key_range_id: String,
    pub region: RegionName,
    pub is_read: bool,
}

/// Circuit-breaker configuration thresholds.
#[derive(Clone, Debug)]
pub(crate) struct CircuitBreakerConfig {
    /// Consecutive read failures before tripping. Default: 2.
    pub read_failure_threshold: u32,
    /// Consecutive write failures before tripping. Default: 5.
    pub write_failure_threshold: u32,
    /// Window after which consecutive failure counters reset. Default: 5 min.
    pub counter_reset_window: Duration,
    /// How long a partition stays unavailable before a probe attempt. Default: 5s.
    pub unavailability_probe_delay: Duration,
    /// Background failback sweep interval. Default: 300s.
    pub failback_interval: Duration,
}

/// Thread-safe holder for the current `PartitionEndpointState` snapshot.
pub(crate) struct PartitionEndpointStateStore {
    state: RwLock<Arc<PartitionEndpointState>>,
}

impl PartitionEndpointStateStore {
    pub fn snapshot(&self) -> Arc<PartitionEndpointState> {
        Arc::clone(&self.state.read())
    }

    pub fn swap(&self, new_state: PartitionEndpointState) {
        *self.state.write() = Arc::new(new_state);
    }
}
```

**Systems (pure functions) operating on `PartitionEndpointState`**:

```rust
// SYSTEM: Produce a new state with the given partition marked unavailable.
// Called from the operation pipeline's PartitionFailover handling.
fn mark_partition_unavailable(
    state: &PartitionEndpointState,
    partition: &UnavailablePartition,
) -> PartitionEndpointState;

// SYSTEM: Record a failure against a partition/region.
// Increments the consecutive failure counter and trips the circuit
// if the threshold is exceeded.
fn record_partition_failure(
    state: &PartitionEndpointState,
    key: &PartitionRegionKey,
    is_read: bool,
    now: Instant,
) -> PartitionEndpointState;

// SYSTEM: Record a success — resets the consecutive failure counter
// and closes the circuit for the given partition/region.
fn record_partition_success(
    state: &PartitionEndpointState,
    key: &PartitionRegionKey,
    now: Instant,
) -> PartitionEndpointState;

// SYSTEM: Background failback sweep.
// Produces a new state with partitions eligible for probing or
// with expired counters reset. Called periodically by
// BackgroundTaskManager.
fn sweep_partition_health(
    state: &PartitionEndpointState,
    now: Instant,
) -> PartitionEndpointState;
```

**Key circuit-breaker thresholds** (from Java SDK reference, used as `CircuitBreakerConfig`
defaults):
- Consecutive read failures before trip: 2
- Consecutive write failures before trip: 5
- Counter reset window: 5 minutes
- Background failback interval: 300 seconds
- Partition unavailability duration before probe: 5 seconds

---

## 5. Transport Pipeline

### 5.1 Responsibilities

The transport pipeline handles a **single attempt** to execute an operation against a **specific
regional endpoint**. It is responsible for:

1. Adding authorization header
2. Adding common Cosmos headers
3. Transport-level retry (429 throttling AND connectivity/I/O errors via shard rotation)
4. Tracking request-sent-status for retry safety
5. Recording per-attempt diagnostics events
6. Enforcing end-to-end deadline (setting per-request timeout on the `HttpClient`)

The transport pipeline does **NOT** handle:
- Region failover (that's the operation pipeline)
- Session consistency routing (that's the operation pipeline)
- Partition-level circuit breaking (that's the operation pipeline)

### 5.2 Transport Pipeline Flow

```rust
pub(crate) struct TransportPipeline {
    headers_policy: CosmosHeadersPolicy,
    auth_policy: AuthorizationPolicy,
    http_transport: AdaptiveTransport, // See §6
}

impl TransportPipeline {
    pub async fn execute(
        &self,
        request: TransportRequest,
        diagnostics: &mut DiagnosticsContextBuilder,
    ) -> TransportResult {
        let mut throttle_state = ThrottleRetryState::new(
            MAX_THROTTLE_ATTEMPTS, MAX_THROTTLE_WAIT, BACKOFF_FACTOR,
        );

        loop {
            // Check deadline and set per-request timeout
            if let Some(deadline) = request.deadline {
                let remaining = deadline.saturating_duration_since(Instant::now());
                if remaining.is_zero() {
                    return TransportResult::deadline_exceeded(diagnostics);
                }
                // Clamp per-request timeout so the HTTP layer cancels I/O
                // when the operation-level deadline is reached mid-flight.
                http_request.set_timeout(remaining.max(Duration::from_millis(1)));
            }

            // Build HTTP request
            let mut http_request = self.build_http_request(&request);
            self.headers_policy.apply(&mut http_request);
            self.auth_policy.sign(&mut http_request, &request.auth_context)?;

            // Record transport start event
            let attempt_start = Instant::now();
            diagnostics.record_event(RequestEvent::TransportStart);

            // Execute via adaptive HTTP transport (sharded or plain)
            let http_result = self.http_transport.send(http_request).await;

            // Map to TransportResult
            let result = match http_result {
                Ok(response) => {
                    diagnostics.record_event(RequestEvent::ResponseHeadersReceived);
                    self.map_response(response, attempt_start, diagnostics).await
                }
                Err(error) => {
                    let sent_status = infer_request_sent_status(&error);
                    diagnostics.record_event(RequestEvent::TransportFailed(
                        error.to_string(),
                    ));
                    TransportResult::transport_error(error, sent_status, diagnostics)
                }
            };

            // Check for 429 throttling or connectivity errors → transport-level retry.
            // Connectivity errors (I/O errors, timeouts) are retried locally by
            // selecting a different HttpClient shard, which routes through a
            // different TCP connection. This is effective because Gateway has
            // multiple nodes and each backend partition has 4 replicas, so
            // retries via a different network path have a high success rate.
            let should_transport_retry = match &result.outcome {
                TransportOutcome::HttpError { status, .. } if status.is_throttled() => true,
                TransportOutcome::TransportError { .. }
                    if operation_is_idempotent_or_request_not_sent(&result) => true,
                _ => false,
            };

            if should_transport_retry {
                let action = evaluate_transport_retry(&result, &throttle_state);
                match action {
                    ThrottleAction::Retry { delay, new_state } => {
                        sleep(delay).await;
                        throttle_state = new_state;
                        continue;
                    }
                    ThrottleAction::Propagate => return result,
                }
            }

            return result;
        }
    }
}
```

### 5.3 Policy Application (Not a Chain)

Unlike the current design where policies are chained via `Policy` trait with `next[0].send()`, the
transport pipeline applies policies as direct function calls. This is simpler, more explicit, and
avoids the indirection of a policy chain.

```rust
// Instead of:
//   pipeline = [HeadersPolicy, AuthPolicy, TrackedTransport]
//   pipeline.send(request, context)  // each calls next

// We do:
//   headers_policy.apply(&mut request);        // mutates request headers
//   auth_policy.sign(&mut request, context);   // adds Authorization header
//   transport.send(request)                    // sends the HTTP request
```

**Rationale**: The Cosmos pipeline has a fixed, small set of policies. There's no user-extensible
policy chain. Direct function calls are easier to debug (no `next[0]` indirection), easier to
test (call the function directly), and have less overhead.

---

## 6. Adaptive HTTP Transport Layer

### 6.0 Gateway Flavors & Protocol Negotiation

Cosmos DB exposes **three gateway flavors** that differ in HTTP version support:

| Gateway            | Usage                | HTTP Version                   | Notes                  |
|--------------------|----------------------|--------------------------------|------------------------|
| **RoutingGateway** | Metadata + Dataplane | HTTP/1.1 only                  | Legacy, no ALPN        |
| **ComputeGateway** | Metadata + Dataplane | HTTP/2 via ALPN                | Preferred for metadata |
| **Gateway 2.0**    | Dataplane only       | HTTP/2 forced (different port) | No HTTP/1.1 fallback   |

At `CosmosClient` initialization, when fetching account properties via the metadata transport,
the driver **probes the gateway's protocol support** using ALPN negotiation. ALPN is sufficient
to decide between HTTP/1.1 and HTTP/2 (adaptive). This determines the transport strategy for
the lifetime of the client:

- **HTTP/2 detected** (ComputeGateway or Gateway 2.0): Use `ShardedHttpTransport` (§6.1+)
- **HTTP/1.1 only** (RoutingGateway): Use a plain `Arc<dyn HttpClient>` — no sharding needed
  since HTTP/1.1 uses one-request-per-connection and the underlying client already manages a
  connection pool via `pool_max_idle_per_host`.

**Gateway 2.0 detection**: The `AccountProperties` response from the metadata endpoint contains
`thinClientWritableLocations` and `thinClientReadableLocations` properties with regional
Gateway 2.0 endpoints. Their presence indicates that Gateway 2.0 should be used for the
dataplane transport (see Java reference:
`sdk/cosmos/azure-cosmos/src/main/java/com/azure/cosmos/implementation/DatabaseAccount.java`).

Both metadata and dataplane pipelines independently determine their transport strategy based on
their respective gateway endpoint. When deploying to a region that uses ComputeGateway, both
pipelines would use `ShardedHttpTransport` (with different timeout configurations). When
deploying to a region with only RoutingGateway, both would use the plain transport.

```rust
/// Transport strategy, determined at client initialization based on gateway probing.
pub(crate) enum AdaptiveTransport {
    /// HTTP/2 gateway — use sharded transport to manage stream limits.
    Sharded(ShardedHttpTransport),
    /// HTTP/1.1 gateway — single HttpClient, connection-per-request model.
    Plain(Arc<dyn HttpClient>),
}

impl AdaptiveTransport {
    pub async fn send(&self, request: &Request) -> Result<AsyncRawResponse> {
        match self {
            Self::Sharded(sharded) => sharded.send(request).await,
            Self::Plain(client) => client.execute_request(request).await,
        }
    }
}
```

### 6.1 Problem Statement (HTTP/2 Sharding)

When operating against an HTTP/2 gateway, the Cosmos DB service endpoint enforces a strict limit of **20 concurrent
streams per H2 connection**. Most `HttpClient` implementations (including reqwest) prefer
multiplexing over a single TCP connection in HTTP/2 mode and do NOT automatically allocate
more connections when the stream limit is reached. This causes:

1. **Head-of-line blocking**: Requests queue behind the 20-stream limit
2. **Blast radius**: A single bad TCP connection affects all multiplexed requests
3. **Unpredictable error propagation**: Inconsistent behavior on which errors close a stream
   vs. the entire connection

### 6.2 Design: `ShardedHttpTransport`

A layer above `Arc<dyn HttpClient>` that manages multiple client instances per endpoint and
load-balances requests based on inflight count and health. The sharded transport uses the
`azure_core::http::HttpClient` trait exclusively — it never references a concrete HTTP library.
All concrete client creation is delegated to a pluggable factory.

```rust
/// Factory trait for creating new HttpClient instances.
///
/// The driver provides a default factory that creates reqwest-backed clients
/// (via `azure_core::http::new_http_client()`), but callers can inject a
/// custom factory for testing or for using alternative HTTP implementations.
pub(crate) trait HttpClientFactory: Send + Sync {
    /// Create a new HttpClient with the given configuration.
    ///
    /// Each call should produce a fresh client that manages its own
    /// connection pool (i.e., separate underlying TCP connections).
    fn create(&self, config: &HttpClientConfig) -> Arc<dyn HttpClient>;
}

/// Configuration passed to the client factory.
pub(crate) struct HttpClientConfig {
    pub connect_timeout: Duration,
    pub request_timeout: Duration,
    pub pool_max_idle_per_host: usize,
    pub pool_idle_timeout: Duration,
    pub proxy: Option<ProxyConfig>,
    pub local_address: Option<IpAddr>,
    pub accept_invalid_certs: bool,
}

/// Manages a pool of HttpClient instances per target endpoint.
/// Distributes requests to avoid exceeding HTTP/2 stream limits.
pub(crate) struct ShardedHttpTransport {
    /// Configuration for the sharding behavior.
    config: ShardingConfig,
    /// Per-endpoint shard pools. Key = endpoint authority (host:port).
    pools: RwLock<HashMap<EndpointKey, Arc<EndpointShardPool>>>,
    /// Factory for creating new HttpClient instances.
    client_factory: Arc<dyn HttpClientFactory>,
}

/// Configuration for HTTP/2 connection sharding.
#[non_exhaustive]
pub struct ShardingConfig {
    /// Maximum concurrent streams per HttpClient per endpoint before
    /// load-balancing to another client. Should be <= 20 (H2 server limit).
    /// Default: 16 (leave headroom below the 20-stream limit).
    pub max_streams_per_client: u32,

    /// Maximum number of HttpClient instances per endpoint.
    /// Limits total connection fan-out.
    /// Default: `num_cpus * 2` (dynamically determined at runtime).
    /// A static fallback of 32 is used if CPU count cannot be detected.
    /// The value should be large enough to sustain thousands of point
    /// reads per second (`num_cpus * 2 * max_streams_per_client` concurrent requests).
    pub max_clients_per_endpoint: u32,

    /// Minimum number of HttpClient instances per endpoint.
    /// Maintained even under low load for quick ramp-up. Default: 1.
    pub min_clients_per_endpoint: u32,

    /// Target inflight utilization ratio for load spreading (0.0 - 1.0).
    /// Controls how aggressively load is concentrated on fewer shards to
    /// allow idle shards to drain and be reclaimed. Default: 0.5.
    /// See §6.4 for details.
    pub load_spread_ratio: f64,

    /// Number of consecutive failures on a client before considering
    /// it for eviction. Default: 5.
    pub consecutive_failure_threshold: u32,

    /// Grace period after last successful request before a client can
    /// be evicted due to consecutive failures. Prevents premature eviction
    /// during transient issues. Default: 2 seconds.
    pub eviction_grace_period: Duration,

    /// Idle timeout for HttpClient instances above min_clients_per_endpoint.
    /// Clients with no inflight requests for this duration are closed.
    /// Default: 60 seconds.
    /// **TODO @fabianm**: Align with the idle connection timeout used by the Cosmos DB
    /// Gateway. Follow up with the Gateway team for the recommended value.
    pub idle_client_timeout: Duration,

    /// How often to run the background health check/eviction sweep.
    /// Default: 10 seconds.
    pub health_check_interval: Duration,

    /// Scale-up threshold: when the average inflight across all shards
    /// exceeds `max_streams_per_client * scale_up_threshold_ratio`, a new
    /// shard is proactively created (up to max_clients_per_endpoint).
    /// Default: 0.75 (at 75% capacity, start scaling up).
    pub scale_up_threshold_ratio: f64,
}
```

### 6.3 Per-Endpoint Shard Pool

```rust
/// A pool of HttpClient instances for a single endpoint.
///
/// The shard list is stored as an immutable `Arc<Vec<...>>` and swapped
/// atomically on writes (clone-on-write). This makes the read path
/// (shard selection) lock-free — callers clone the `Arc` and iterate
/// without holding a lock. Mutations (add/remove shard) take a write
/// lock, build a new `Vec`, and swap the `Arc`.
struct EndpointShardPool {
    endpoint: EndpointKey,
    config: ShardingConfig,
    /// The shards. Stored as Arc<Vec<...>> for lock-free reads.
    /// Replaced atomically on shard add/remove.
    shards: RwLock<Arc<Vec<Arc<ClientShard>>>>,
    /// Factory for creating new HttpClient instances.
    client_factory: Arc<dyn HttpClientFactory>,
    /// HttpClient configuration for this pool.
    client_config: HttpClientConfig,
}

/// A single HttpClient with per-endpoint health/inflight tracking.
struct ClientShard {
    /// The HTTP client behind the azure_core abstraction.
    client: Arc<dyn HttpClient>,
    /// Per-endpoint inflight tracking. Key = endpoint authority.
    endpoint_stats: RwLock<HashMap<EndpointKey, EndpointStats>>,
    /// Shard-level health state.
    health: ShardHealth,
    /// When this shard was created.
    created_at: Instant,
    /// Shard ID for diagnostics.
    id: u64,
}

/// Per-endpoint statistics on a single shard.
/// Uses atomics for lock-free updates on the hot path.
struct EndpointStats {
    /// Currently inflight requests to this endpoint on this shard.
    inflight: AtomicU32,
    /// Timestamp of last successful response (nanos since epoch).
    last_success_nanos: AtomicU64,
    /// Timestamp of last request (success or failure).
    last_request_nanos: AtomicU64,
    /// Consecutive failure count (reset on success).
    consecutive_failures: AtomicU32,
    /// Total requests sent.
    total_requests: AtomicU64,
    /// Total failures.
    total_failures: AtomicU64,
}

/// Shard-level health tracking (across all endpoints on this shard).
struct ShardHealth {
    /// Total inflight across all endpoints.
    total_inflight: AtomicU32,
    /// Last successful response on any endpoint.
    last_success_nanos: AtomicU64,
    /// Whether this shard is marked for eviction.
    marked_for_eviction: AtomicBool,
}
```

### 6.4 Request Routing Algorithm & Elastic Scaling

The shard selection algorithm serves two goals simultaneously:

1. **Avoid exceeding the H2 stream limit** (scale up under load)
2. **Consolidate load to allow idle shards to drain and close** (scale down after spikes)

The key mechanism is `load_spread_ratio` (default: 0.5). When the pool has `N` shards, only the
top `ceil(N * load_spread_ratio)` shards are considered for new requests (the "active set").
The remaining shards receive no new work, allowing their inflight counts to drain to zero and
eventually hit the `idle_client_timeout`, at which point they are reclaimed by the health sweep.

**Example**: After a traffic spike, the pool has scaled up to 10 shards. With
`load_spread_ratio = 0.5`, only 5 shards are in the active set. The other 5 receive no new
requests. Once their inflight drains to 0 and `idle_client_timeout` (60s default) elapses,
the health sweep closes them. The pool shrinks back toward `min_clients_per_endpoint`.

```text
Traffic spike scaling:

  Time T1 (spike starts):
    Shard 1: [||||||||||||||||]  16/16 → at capacity
    Shard 2: [||||||||||||||||]  16/16 → at capacity
    → scale_up_threshold_ratio (0.75) exceeded → create Shard 3

  Time T2 (spike grows):
    Shard 1: [||||||||||||||||]  16/16
    Shard 2: [||||||||||||||||]  16/16
    Shard 3: [||||||||||||||||]  16/16 → all at capacity → create Shard 4
    Shard 4: [||||||||]           8/16 → absorbing overflow

  Time T3 (spike subsides):
    load_spread_ratio kicks in → active set = ceil(4 * 0.5) = 2 shards
    Shard 1: [||||||||||]        10/16  ← active (receives new requests)
    Shard 2: [|||||||||]          9/16  ← active (receives new requests)
    Shard 3: [||]                 2/16  ← draining (no new requests)
    Shard 4: []                   0/16  ← draining, idle timer started

  Time T4 (after idle_client_timeout):
    Shard 1: [||||]               4/16  ← active
    Shard 2: [|||]                3/16  ← active
    Shard 3: (closed by health sweep — idle_client_timeout exceeded)
    Shard 4: (closed by health sweep — idle_client_timeout exceeded)
```

**Proactive scale-up signal**: Rather than only scaling up reactively when a request finds all
shards at `max_streams_per_client`, the health sweep also monitors the **average inflight ratio**
across active shards. When `avg_inflight / max_streams_per_client > scale_up_threshold_ratio`
(default: 0.75), a new shard is created proactively. This avoids the latency spike of creating
a new shard + TCP handshake + TLS negotiation on the critical path of a request.

```rust
impl ShardedHttpTransport {
    pub async fn send(&self, request: &Request) -> Result<AsyncRawResponse> {
        let endpoint_key = EndpointKey::from_url(request.url());
        let pool = self.get_or_create_pool(&endpoint_key);
        let shard = pool.select_shard(&endpoint_key);

        // Increment inflight counter
        shard.endpoint_stats(&endpoint_key).inflight.fetch_add(1, Ordering::Relaxed);
        shard.health.total_inflight.fetch_add(1, Ordering::Relaxed);

        let result = shard.client.execute_request(request).await;

        // Decrement inflight counter
        shard.endpoint_stats(&endpoint_key).inflight.fetch_sub(1, Ordering::Relaxed);
        shard.health.total_inflight.fetch_sub(1, Ordering::Relaxed);

        // Update health stats
        match &result {
            Ok(_) => shard.record_success(&endpoint_key),
            Err(_) => shard.record_failure(&endpoint_key),
        }

        result
    }
}

impl EndpointShardPool {
    /// Select the best shard for a new request to the given endpoint.
    ///
    /// Algorithm:
    /// 1. Filter out shards marked for eviction.
    /// 2. Compute the "active set" size: ceil(total_shards * load_spread_ratio).
    ///    Only shards in the active set are candidates for new requests.
    /// 3. Among active shards, pick the one with the lowest inflight count
    ///    for this endpoint (least-loaded first).
    /// 4. If the best active shard's inflight >= max_streams_per_client,
    ///    create a new shard (if under max_clients_per_endpoint).
    /// 5. If at max clients, use the least-loaded shard from the FULL pool
    ///    (queue pressure, but bounded).
    fn select_shard(&self, endpoint: &EndpointKey) -> Arc<ClientShard> {
        let mut candidates = {
            let shards = self.shards.read();

            // Healthy shards, sorted by inflight count ascending
            let mut candidates: Vec<_> = shards.iter()
                .filter(|s| !s.health.marked_for_eviction.load(Ordering::Relaxed))
                .map(|s| {
                    let inflight = s.endpoint_inflight(endpoint);
                    (s.clone(), inflight)
                })
                .collect();

            candidates.sort_by_key(|(_, inflight)| *inflight);

            // Active set: only the top N shards receive new requests.
            // Remaining shards are left to drain (scale-down path).
            let active_count = (candidates.len() as f64 * self.config.load_spread_ratio)
                .ceil()
                .max(1.0) as usize;

            // Among active set, find least-loaded
            // Note: candidates are sorted ascending by inflight, so we want
            // to route to the least-loaded within the active set. The "active set"
            // is the first `active_count` shards (sorted ascending = least loaded first).
            let active_set = &candidates[..active_count.min(candidates.len())];

            if let Some((best, inflight)) = active_set.first() {
                if *inflight < self.config.max_streams_per_client {
                    return best.clone();
                }
            }

            candidates
        }; // read lock released here

        // All active shards at capacity → try to create a new one
        if let Some(new_shard) = self.try_create_shard() {
            return new_shard;
        }

        // At max clients → fall back to least-loaded across ALL shards
        // (including draining ones) to prevent request queuing
        candidates.first()
            .map(|(s, _)| s.clone())
            .unwrap_or_else(|| self.create_emergency_shard())
    }

    /// Proactively create a shard if average utilization is above threshold.
    /// Called by the background health sweep (off the request hot path).
    fn maybe_proactive_scale_up(&self, endpoint: &EndpointKey) {
        let shards = self.shards.read();
        if shards.len() >= self.config.max_clients_per_endpoint as usize {
            return;
        }

        let active_shards: Vec<_> = shards.iter()
            .filter(|s| !s.health.marked_for_eviction.load(Ordering::Relaxed))
            .collect();

        if active_shards.is_empty() {
            return;
        }

        let total_inflight: u32 = active_shards.iter()
            .map(|s| s.endpoint_inflight(endpoint))
            .sum();
        let avg_inflight = total_inflight as f64 / active_shards.len() as f64;
        let threshold = self.config.max_streams_per_client as f64
            * self.config.scale_up_threshold_ratio;

        if avg_inflight > threshold {
            drop(shards);
            let _ = self.try_create_shard();
        }
    }
}
```

### 6.5 Health Checking, Eviction & Scale-Down (Inspired by Java's `RntbdClientChannelHealthChecker`)

The health checker runs periodically and evaluates each shard. The check order follows the Java
pattern (ordered priority, first match returns):

```rust
/// Health check result for a single shard.
enum ShardHealthStatus {
    Healthy,
    Unhealthy { reason: EvictionReason },
    Idle,
}

enum EvictionReason {
    /// No successful response within read-delay limit despite having inflight requests.
    ReadHang,
    /// Consecutive failures exceeded threshold AND other shards to the same
    /// endpoint are succeeding (connection-level problem, not service-level).
    ConsecutiveFailuresWithHealthyPeers,
    /// No activity for longer than idle timeout.
    IdleTimeout,
}

fn check_shard_health(
    shard: &ClientShard,
    endpoint: &EndpointKey,
    peer_shards: &[Arc<ClientShard>],
    config: &ShardingConfig,
) -> ShardHealthStatus {
    let stats = shard.endpoint_stats(endpoint);
    let now_nanos = now_as_nanos();

    // 1. Recent success → Healthy (fast path)
    let last_success = stats.last_success_nanos.load(Ordering::Relaxed);
    if now_nanos - last_success < RECENT_SUCCESS_WINDOW_NANOS {
        return ShardHealthStatus::Healthy;
    }

    // 2. Read hang: inflight > 0 but no success for a long time
    let inflight = stats.inflight.load(Ordering::Relaxed);
    if inflight > 0 && (now_nanos - last_success) > READ_HANG_THRESHOLD_NANOS {
        return ShardHealthStatus::Unhealthy {
            reason: EvictionReason::ReadHang,
        };
    }

    // 3. Consecutive failures with healthy peers
    let consecutive = stats.consecutive_failures.load(Ordering::Relaxed);
    if consecutive >= config.consecutive_failure_threshold {
        // Check if peers are healthier (indicates connection problem, not service)
        let peers_healthy = peer_shards.iter().any(|peer| {
            let peer_stats = peer.endpoint_stats(endpoint);
            let peer_consecutive = peer_stats.consecutive_failures.load(Ordering::Relaxed);
            let peer_last_success = peer_stats.last_success_nanos.load(Ordering::Relaxed);
            peer_consecutive < consecutive && peer_last_success > last_success
        });

        if peers_healthy {
            // Grace period: don't evict immediately after recent success
            let grace_ok = (now_nanos - last_success)
                > config.eviction_grace_period.as_nanos() as u64;
            if grace_ok {
                return ShardHealthStatus::Unhealthy {
                    reason: EvictionReason::ConsecutiveFailuresWithHealthyPeers,
                };
            }
        }
    }

    // 4. Idle timeout (only for shards above minimum count)
    let last_request = stats.last_request_nanos.load(Ordering::Relaxed);
    if inflight == 0 && (now_nanos - last_request) > config.idle_client_timeout.as_nanos() as u64 {
        return ShardHealthStatus::Idle;
    }

    ShardHealthStatus::Healthy
}
```

**Eviction process**:
1. Mark shard for eviction (`marked_for_eviction = true`)
2. Stop routing new requests to it (the `load_spread_ratio` mechanism naturally does this
   by excluding draining shards from the active set)
3. Wait for inflight requests to drain (with a timeout)
4. Drop the `Arc<dyn HttpClient>` (the underlying client closes connections on final `Drop`)
5. If pool size drops below `min_clients_per_endpoint`, create a replacement

**Scale-down flow**: The `load_spread_ratio` and `idle_client_timeout` work together:

1. As load decreases, `select_shard` concentrates requests on the active set
   (`ceil(N * load_spread_ratio)` shards).
2. Shards outside the active set receive no new requests; their inflight drains to 0.
3. Once `idle_client_timeout` elapses with 0 inflight, the health sweep marks them `Idle`.
4. Idle shards above `min_clients_per_endpoint` are removed from the pool.
5. The active set shrinks proportionally (`ceil(smaller_N * load_spread_ratio)`).
6. The process continues until the pool stabilizes at `min_clients_per_endpoint`.

This provides smooth, gradual scale-down without abrupt connection drops that could
cause latency spikes.

**Key differences from Java `RntbdClientChannelHealthChecker`**:
- Java checks per-channel (per TCP connection); we check per-`HttpClient` shard (which manages its
  own connection pool internally)
- Java has CPU-aware guards (disable eviction under high CPU); we should adopt this using the
  existing `CpuMemoryMonitor` (with a 90% CPU threshold, aligned with Java)
- Java has transit-timeout counting; we use consecutive-failure counting as a proxy since the
  `HttpClient` trait doesn't expose per-stream timeout events separately

**TCP keepalive consideration**: Proactive shard eviction is most valuable when it is driven
by actual connectivity signals. The `reqwest` client supports TCP keepalive configuration
(`tcp_keepalive`). When TCP keepalive probes detect a dead connection, the resulting I/O error
feeds into the consecutive-failure counter, which then triggers eviction. Without keepalive,
idle connections may silently go stale (e.g., due to NAT/firewall timeouts) and only fail on
the next real request. **Recommended**: enable TCP keepalive on all `HttpClient` instances
created by the `HttpClientFactory` (e.g., 30-second interval). Research is needed on the
specific `reqwest` / `hyper` APIs for configuring keepalive intervals and probes.

### 6.6 Background Health Sweep

```rust
impl ShardedHttpTransport {
    /// Background task that periodically:
    /// 1. Checks shard health and evicts unhealthy shards
    /// 2. Reclaims idle shards above minimum
    /// 3. Proactively scales up if utilization is high
    async fn health_sweep_loop(self: Arc<Self>) {
        loop {
            sleep(self.config.health_check_interval).await;

            for (endpoint, pool) in self.pools.read().iter() {
                let shards = pool.shards.read().clone();
                let mut to_evict = Vec::new();
                let mut to_remove_idle = Vec::new();

                for shard in &shards {
                    let status = check_shard_health(
                        shard, endpoint, &shards, &self.config,
                    );
                    match status {
                        ShardHealthStatus::Unhealthy { reason } => {
                            shard.health.marked_for_eviction.store(true, Ordering::Relaxed);
                            to_evict.push((shard.id, reason));
                        }
                        ShardHealthStatus::Idle if shards.len() > pool.config.min_clients_per_endpoint as usize => {
                            to_remove_idle.push(shard.id);
                        }
                        _ => {}
                    }
                }

                // Remove evicted/idle shards
                if !to_evict.is_empty() || !to_remove_idle.is_empty() {
                    let mut shards_mut = pool.shards.write();
                    shards_mut.retain(|s| {
                        !to_evict.iter().any(|(id, _)| *id == s.id) &&
                        !to_remove_idle.contains(&s.id)
                    });

                    // Ensure minimum shard count
                    while shards_mut.len() < pool.config.min_clients_per_endpoint as usize {
                        if let Some(new) = pool.create_shard_internal() {
                            shards_mut.push(Arc::new(new));
                        }
                    }
                }

                // Proactive scale-up check
                pool.maybe_proactive_scale_up(endpoint);
            }
        }
    }
}
```

---

## 7. Fault Injection Integration

### 7.1 Architecture

Fault injection is integrated at the **`HttpClient` level** — *below* the `AdaptiveTransport`
(sharding) layer. Each shard's `Arc<dyn HttpClient>` is wrapped (or replaced) by a
`FaultInjectingHttpClient` that evaluates rules before delegating to the real client.

This placement is deliberate: it means injected faults are visible to the shard health tracking,
eviction logic, and scale-up/down algorithms in `ShardedHttpTransport`. A fault that triggers
consecutive failures on one shard will cause that shard to be evicted and replaced — exactly
the behavior we need to validate in tests. If fault injection sat *above* `AdaptiveTransport`,
the sharding layer would never see the injected errors and its management logic would be
untestable.

```text
  TransportPipeline
       │
       ▼
  ┌─────────────────────────────┐
  │  AdaptiveTransport          │  ← Sharded (H2) or Plain (H1.1)
  │                             │
  │  ┌────────────────────────┐ │
  │  │ ShardedHttpTransport   │ │  ← shard selection, health tracking
  │  │  ┌──────────────────┐  │ │
  │  │  │ ClientShard[0]   │  │ │
  │  │  │  ┌─────────────┐ │  │ │
  │  │  │  │FaultInject- │ │  │ │  ← evaluates rules, may short-circuit
  │  │  │  │ingHttpClient│ │  │ │
  │  │  │  │  ┌────────┐ │ │  │ │
  │  │  │  │  │ real   │ │ │  │ │  ← actual HTTP I/O (reqwest etc.)
  │  │  │  │  │HttpCli.│ │ │  │ │
  │  │  │  │  └────────┘ │ │  │ │
  │  │  │  └─────────────┘ │  │ │
  │  │  └──────────────────┘  │ │
  │  │  ┌──────────────────┐  │ │
  │  │  │ ClientShard[1]   │  │ │  ← same wrapping per shard
  │  │  │  ...             │  │ │
  │  │  └──────────────────┘  │ │
  │  └────────────────────────┘ │
  └─────────────────────────────┘
```

### 7.2 Injection via `HttpClientFactory`

The `HttpClientFactory` trait (§6.2) is the natural injection point. A
`FaultInjectingHttpClientFactory` wraps the real factory and produces
`FaultInjectingHttpClient` instances. Because the factory is called once per shard creation,
each shard gets its own fault-injecting wrapper — and the `ShardedHttpTransport` is completely
unaware that interception is happening.

```rust
/// Factory that wraps each produced HttpClient with fault injection.
/// Feature-gated behind `fault_injection`.
pub(crate) struct FaultInjectingHttpClientFactory {
    inner: Arc<dyn HttpClientFactory>,
    rules: Arc<RwLock<Vec<FaultInjectionRule>>>,
}

impl HttpClientFactory for FaultInjectingHttpClientFactory {
    fn create(&self, config: &HttpClientConfig) -> Arc<dyn HttpClient> {
        let real_client = self.inner.create(config);
        Arc::new(FaultInjectingHttpClient {
            inner: real_client,
            rules: Arc::clone(&self.rules),
        })
    }
}

/// HttpClient wrapper that evaluates fault injection rules before
/// delegating to the real client.
pub(crate) struct FaultInjectingHttpClient {
    inner: Arc<dyn HttpClient>,
    rules: Arc<RwLock<Vec<FaultInjectionRule>>>,
}

impl HttpClient for FaultInjectingHttpClient {
    async fn execute_request(
        &self,
        request: &Request,
    ) -> Result<AsyncRawResponse> {
        // Evaluate rules — first matching rule wins.
        if let Some(fault) = self.evaluate_rules(request) {
            return fault.apply(request);
        }
        // No rule matched — pass through to real client.
        self.inner.execute_request(request).await
    }
}
```

### 7.3 What This Enables

Because faults hit at the `HttpClient` level, tests can validate:

1. **Shard eviction**: Inject consecutive transport errors on one shard → verify it gets
   evicted after `consecutive_failure_threshold` and replaced with a healthy shard.
2. **Scale-up under failure**: Inject errors on all current shards → verify pool scales up
   to `max_clients_per_endpoint` as shards are marked unhealthy.
3. **Scale-down after recovery**: Remove fault rules → verify healthy shards drain idle
   ones and the pool shrinks back to `min_clients_per_endpoint`.
4. **Per-shard behavior**: Inject a fault on shard 0 only → verify requests route to
   other shards while shard 0 is evicted.
5. **Full pipeline integration**: Injected 429/503/transport errors still trigger the
   transport-level and operation-level retry/failover logic, since the `TransportPipeline`
   sees the error responses from the sharded transport as usual.

The existing `FaultInjectionRule`, `FaultInjectionCondition`, and `FaultInjectionResult` types
from `azure_data_cosmos` are moved into `azure_data_cosmos_driver` behind a `fault_injection`
feature flag.

---

## 8. Migration Plan

Each step below is designed as a **vertical slice** — a self-contained PR that delivers working
end-to-end functionality with clear, limited scope. Earlier steps may use hard-coded
timeouts/thresholds, skip advanced features, or support only HTTP/1.1. Later steps layer on
capabilities incrementally. This keeps PRs review-friendly and reduces risk.

### Step 1: Minimal Transport Pipeline (HTTP/1.1, single-region, basic retry)

Refactor `execute_operation` to use the new pipeline architecture with the absolute minimum
viable transport. No HTTP/2 sharding, no hedging, no circuit breaker, no session consistency —
just the structural refactoring with a plain `Arc<dyn HttpClient>`.

| Sub-step | Work Item                                                                                                                                                                                                                        | Files                                                          |
|----------|----------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------|----------------------------------------------------------------|
| 1.1      | **State components** — Define ECS-style component types (`RoutingDecision`, `OperationRetryState`, `TransportRequest`, `TransportResult`, `ThrottleRetryState`, decision enums)                                                  | `driver/pipeline/components.rs`                                |
| 1.2      | **Transport pipeline (slim)** — Implement `TransportPipeline` with auth header, common headers, and 429 throttle retry only. Uses a plain `Arc<dyn HttpClient>` (no `AdaptiveTransport` yet). Hard-coded retry limits + backoff. | `driver/transport/transport_pipeline.rs`                       |
| 1.3      | **Operation pipeline (slim)** — Implement `execute_operation_pipeline` core loop with single-region endpoint resolution. Only the happy path + 429 retry propagation. No failover, no hedging. Hard-coded deadline.              | `driver/pipeline/operation_pipeline.rs`, `retry_evaluation.rs` |
| 1.4      | **Wire `execute_operation`** — Connect `CosmosDriver::execute_operation` to the new operation pipeline for all operations. The old pipeline code path remains but is no longer called.                                           | `driver/cosmos_driver.rs`                                      |
| 1.5      | **Unit tests** — Tests for each pure function (`evaluate_transport_result` happy path, `evaluate_transport_retry` for 429, `build_transport_request`).                                                                           | `tests/`                                                       |

**What works after Step 1**: Operations flow through the new pipeline end-to-end against a
single region with basic 429 retry. The architecture is in place for incremental additions.

### Step 2: Multi-region failover & endpoint management

Add cross-region failover, `AccountEndpointState` + routing systems, and the `AccountMetadataCache` integration.

| Sub-step | Work Item                                                                                                                                                                                                                                                     | Files                                                                      |
|----------|---------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------|----------------------------------------------------------------------------|
| 2.1      | **Routing state & systems** — Implement `AccountEndpointState`, `AccountEndpointStateStore`, and the system functions (`build_account_endpoint_state`, `mark_endpoint_unavailable`, `expire_unavailable_endpoints`). Wire to existing `AccountMetadataCache`. | `driver/routing/mod.rs`, `account_endpoint_state.rs`, `routing_systems.rs` |
| 2.2      | **Expand `evaluate_transport_result`** — Add 403.3 (WriteForbidden + cache refresh with rate-limiting), 503, 404/1022, 429/3092, 500-for-reads. Add `FailoverRetry`, `SessionRetry`, `PartitionFailover` action handling in the operation loop.               | `driver/pipeline/retry_evaluation.rs`, `operation_pipeline.rs`             |
| 2.3      | **Deadline enforcement** — Implement active deadline enforcement in transport pipeline (per-request timeout clamping, stream drop).                                                                                                                           | `driver/transport/transport_pipeline.rs`                                   |
| 2.4      | **Config surface (initial)** — Add basic retry and failover options to `OperationOptions` / `RuntimeOptions`. Hard-coded defaults, environment variable overrides.                                                                                            | `options/retry.rs`, `options/availability.rs`                              |
| 2.5      | **Tests** — Unit tests for each retry scenario in `evaluate_transport_result`, integration tests for multi-region failover.                                                                                                                                   | `tests/`                                                                   |

**What works after Step 2**: Full multi-region failover, 403.3 recovery with cache refresh,
session retry, deadline enforcement. Still HTTP/1.1, no hedging, no circuit breaker.

### Step 3: Session consistency & partition-level circuit breaker

| Sub-step | Work Item                                                                                                                                                                                                                                                                                                                          | Files                                                                      |
|----------|------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------|----------------------------------------------------------------------------|
| 3.1      | **Session tracking** — Session token management (resolve, propagate, track LSN).                                                                                                                                                                                                                                                   | `driver/routing/session_manager.rs`                                        |
| 3.2      | **Partition endpoint state & circuit breaker** — Implement `PartitionEndpointState`, `PartitionEndpointStateStore`, `CircuitBreakerConfig`, and the system functions (`mark_partition_unavailable`, `record_partition_failure/success`, `sweep_partition_health`). Hard-coded default thresholds (read: 2, write: 5, reset: 5min). | `driver/routing/partition_endpoint_state.rs`, `circuit_breaker_systems.rs` |
| 3.3      | **`ReadConsistencyStrategy`** — Wire `effective_read_consistency_strategy` into `SessionState` and endpoint resolution.                                                                                                                                                                                                            | `driver/pipeline/components.rs`, `operation_pipeline.rs`                   |
| 3.4      | **Circuit breaker config** — Make thresholds configurable via `CircuitBreakerConfig`.                                                                                                                                                                                                                                              | `options/availability.rs`                                                  |
| 3.5      | **Tests** — Session consistency retry, circuit breaker trip/reset, partition failover.                                                                                                                                                                                                                                             | `tests/`                                                                   |

**What works after Step 3**: Full session consistency routing, partition-level circuit breaker
with failback. Still HTTP/1.1, no hedging.

### Step 4: Hedging (speculative execution)

| Sub-step | Work Item                                                                                                                                        | Files                        |
|----------|--------------------------------------------------------------------------------------------------------------------------------------------------|------------------------------|
| 4.1      | **Hedging implementation** — `execute_hedged` with `tokio::select!` racing. Initial/Hedging `ExecutionContext` tracking. Static threshold first. | `driver/pipeline/hedging.rs` |
| 4.2      | **Dynamic threshold** — P99 latency tracker with safety gates (50–4000 ms).                                                                      | `driver/pipeline/hedging.rs` |
| 4.3      | **Hedging config** — `HedgingThreshold` enum (Dynamic/Static), `hedging_enabled` flag, overridable at `DriverRuntime`/`Driver`/operation levels. | `options/availability.rs`    |
| 4.4      | **Tests** — Hedging fires after threshold, primary wins, secondary wins, both fail, write hedging on MWR only.                                   | `tests/`                     |

**What works after Step 4**: Full hedging with dynamic P99-based threshold. Still HTTP/1.1.

### Step 5: HTTP/2 support & adaptive transport (no sharding yet)

Add protocol detection and the `AdaptiveTransport` enum, but without the sharded transport —
HTTP/2 just uses a single `Arc<dyn HttpClient>` like HTTP/1.1.

| Sub-step | Work Item                                                                                                                                     | Files                                     |
|----------|-----------------------------------------------------------------------------------------------------------------------------------------------|-------------------------------------------|
| 5.1      | **Gateway probing** — ALPN negotiation to detect HTTP/2 vs HTTP/1.1. Gateway 2.0 detection via `thinClient*Locations` in `AccountProperties`. | `driver/transport/adaptive_transport.rs`  |
| 5.2      | **`AdaptiveTransport` enum** — `Sharded` / `Plain` dispatch. Initially both paths use a plain `Arc<dyn HttpClient>`.                          | `driver/transport/adaptive_transport.rs`  |
| 5.3      | **`HttpClientFactory` trait** — Pluggable factory + default reqwest-backed implementation. `HttpClientConfig` struct.                         | `driver/transport/http_client_factory.rs` |
| 5.4      | **Tests** — Protocol detection, factory creates clients, adaptive dispatch.                                                                   | `tests/`                                  |

**What works after Step 5**: HTTP/2 requests work (via a single connection). Gateway 2.0
endpoints are detected and used. No sharding yet — stream limit may be hit under high load.

### Step 6: HTTP/2 connection sharding

| Sub-step | Work Item                                                                                                                                                                                                       | Files                                                    |
|----------|-----------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------|----------------------------------------------------------|
| 6.1      | **`ShardedHttpTransport`** — Core shard pool with `EndpointShardPool`, `ClientShard`, inflight tracking via atomics. Immutable `Arc<Vec<...>>` snapshot for lock-free shard reads.                              | `driver/transport/sharded_transport.rs`, `shard_pool.rs` |
| 6.2      | **Shard selection algorithm** — `select_shard` with `load_spread_ratio`, active set, least-loaded routing, scale-up on capacity.                                                                                | `driver/transport/shard_pool.rs`                         |
| 6.3      | **`ShardingConfig`** — All knobs (`max_streams_per_client`, `max_clients_per_endpoint = num_cpus * 2`, `min_clients_per_endpoint`, `load_spread_ratio`, `idle_client_timeout`). Environment variable overrides. | `options/connection_pool.rs`                             |
| 6.4      | **Wire into `AdaptiveTransport`** — When HTTP/2 is detected, use `ShardedHttpTransport` instead of plain.                                                                                                       | `driver/transport/adaptive_transport.rs`                 |
| 6.5      | **Tests** — Shard selection under load, scale-up, inflight tracking, multi-endpoint pools.                                                                                                                      | `tests/`                                                 |

**What works after Step 6**: HTTP/2 with connection sharding and elastic scale-up. No health
checks or eviction yet — shards accumulate but do not get reclaimed.

### Step 7: Health checks, eviction, TCP keepalive & connectivity retry

| Sub-step | Work Item                                                                                                                                                          | Files                                     |
|----------|--------------------------------------------------------------------------------------------------------------------------------------------------------------------|-------------------------------------------|
| 7.1      | **Shard health checks** — `check_shard_health` (read-hang, consecutive failures with healthy peers, idle timeout). CPU-aware eviction guard (90% threshold).       | `driver/transport/shard_health.rs`        |
| 7.2      | **Background health sweep** — `health_sweep_loop` (evict unhealthy, reclaim idle, proactive scale-up). Scale-down via `load_spread_ratio` + `idle_client_timeout`. | `driver/transport/shard_health.rs`        |
| 7.3      | **TCP keepalive** — Enable TCP keepalive on all `HttpClient` instances (30s interval). Research reqwest/hyper APIs.                                                | `driver/transport/http_client_factory.rs` |
| 7.4      | **Connectivity retry in transport** — Expand `evaluate_transport_retry` to retry I/O errors and timeouts on a different shard (for idempotent/not-sent requests).  | `driver/transport/transport_pipeline.rs`  |
| 7.5      | **Tests** — Shard eviction, idle reclaim, scale-down flow, keepalive config, connectivity retry on different shard.                                                | `tests/`                                  |

**What works after Step 7**: Full HTTP/2 sharding with health monitoring, automatic eviction
& scale-down, TCP keepalive, and connectivity-level retry via shard rotation.

### Step 8: Fault injection

| Sub-step | Work Item                                                                                                                                                                                                                     | Files                                                                            |
|----------|-------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------|----------------------------------------------------------------------------------|
| 8.1      | **Move fault injection** — Move `FaultInjectionRule`, `FaultInjectionCondition`, `FaultInjectionResult` from `azure_data_cosmos` to driver behind `fault_injection` feature flag.                                             | `driver/fault_injection/mod.rs`, `rule.rs`, `condition.rs`, `result.rs`          |
| 8.2      | **`FaultInjectingHttpClientFactory`** — Wraps the real `HttpClientFactory`, producing `FaultInjectingHttpClient` instances that intercept at the `HttpClient` trait level (below `AdaptiveTransport`/`ShardedHttpTransport`). | `driver/fault_injection/fault_injecting_factory.rs`, `fault_injecting_client.rs` |
| 8.3      | **Tests** — Shard eviction under injected failures, scale-up/down validation, full pipeline retry/failover with injected faults, rule matching, feature gate.                                                                 | `tests/`                                                                         |

**What works after Step 8**: Complete driver pipeline with fault injection support.

### Step 9: Cut over `azure_data_cosmos` — Phase 1 (readItem + createItem)

Cut over `readItem` and `createItem` to use `driver.execute_operation()`. The existing pipeline
code in `azure_data_cosmos` remains in place — other operations still use it. This validates the
new pipeline under real workloads with the two most common operations before committing to a
full cut-over.

| Sub-step | Work Item                                                                                          | Files                                               |
|----------|----------------------------------------------------------------------------------------------------|-----------------------------------------------------|
| 9.1      | Wire `ContainerClient::read_item` to build `CosmosOperation` and call `driver.execute_operation()` | `azure_data_cosmos/src/clients/container_client.rs` |
| 9.2      | Wire `ContainerClient::create_item` similarly                                                      | `azure_data_cosmos/src/clients/container_client.rs` |
| 9.3      | Integration tests verifying read/create through the new pipeline                                   | `tests/`                                            |

**What works after Step 9**: `readItem` and `createItem` flow through the new driver pipeline.
All other operations still use the old pipeline. No code is removed yet.

### Step 10: Cut over `azure_data_cosmos` — Phase 2 (full cut-over + cleanup)

Cut over all remaining operations and remove the old pipeline code.

| Sub-step | Work Item                                                                                             | Files                                |
|----------|-------------------------------------------------------------------------------------------------------|--------------------------------------|
| 10.1     | Cut over all remaining operations in `azure_data_cosmos/src/clients/` to `driver.execute_operation()` | `azure_data_cosmos/src/clients/*.rs` |
| 10.2     | Remove `azure_data_cosmos/src/pipeline/`                                                              | —                                    |
| 10.3     | Remove `azure_data_cosmos/src/retry_policies/`                                                        | —                                    |
| 10.4     | Remove `azure_data_cosmos/src/handler/`                                                               | —                                    |
| 10.5     | Remove `azure_data_cosmos/src/routing/` (endpoint managers)                                           | —                                    |
| 10.6     | Remove `azure_data_cosmos/src/request_context.rs`                                                     | —                                    |
| 10.7     | Move fault injection tests to driver-level APIs                                                       | `tests/`                             |
| 10.8     | Full integration test pass                                                                            | `tests/`                             |

**What works after Step 10**: `azure_data_cosmos` is a thin client layer that builds
`CosmosOperation` values and delegates all execution to the driver. Duplicate pipeline,
retry, and routing code is removed.

---

## 9. Configuration Surface

### New Options in `ConnectionPoolOptions`

```rust
// Added to ConnectionPoolOptions or a new ShardingOptions struct:

/// Configuration for HTTP/2 connection sharding.
/// Only applies when using an HTTP/2 gateway (ComputeGateway or Gateway 2.0).
/// Ignored when the gateway only supports HTTP/1.1 (RoutingGateway).
pub sharding: ShardingConfig,
```

### New Options in `OperationOptions` or `RuntimeOptions`

```rust
/// Hedging configuration. Hedging is enabled by default with a dynamic
/// threshold based on observed P99 latency (clamped to 50-4000 ms).
/// Set `hedging_enabled` to `false` to disable hedging entirely.
/// Overridable at `DriverRuntime`, `Driver`, and per-operation levels.
pub hedging_enabled: bool,  // Default: true
pub hedging_threshold: HedgingThreshold,

/// Circuit breaker configuration for partition-level failover.
pub circuit_breaker: CircuitBreakerConfig,
```

```rust
pub enum HedgingThreshold {
    /// Dynamic threshold based on observed P99 latency with safety gates.
    /// This is the default.
    Dynamic {
        /// Minimum threshold (hard floor). Default: 50 ms.
        min: Duration,
        /// Maximum threshold (hard ceiling). Default: 4000 ms.
        max: Duration,
    },
    /// Static threshold (fixed duration).
    Static(Duration),
}
```

```rust
pub struct CircuitBreakerConfig {
    /// Consecutive read failures before tripping. Default: 2.
    pub read_failure_threshold: u32,
    /// Consecutive write failures before tripping. Default: 5.
    pub write_failure_threshold: u32,
    /// Counter reset window. Default: 5 minutes.
    pub counter_reset_window: Duration,
    /// Background failback interval. Default: 300 seconds.
    pub failback_interval: Duration,
    /// Duration a partition must be unavailable before probe. Default: 5 seconds.
    pub unavailability_probe_delay: Duration,
}
```

### Environment Variable Support

Following the existing pattern in `ConnectionPoolOptions`:

| Variable                                | Config Field                        |
|-----------------------------------------|-------------------------------------|
| `AZURE_COSMOS_MAX_STREAMS_PER_CLIENT`   | `sharding.max_streams_per_client`   |
| `AZURE_COSMOS_MAX_CLIENTS_PER_ENDPOINT` | `sharding.max_clients_per_endpoint` |
| `AZURE_COSMOS_SHARD_IDLE_TIMEOUT_SECS`  | `sharding.idle_client_timeout`      |
| `AZURE_COSMOS_HEDGING_THRESHOLD_MS`     | `hedging_threshold`                 |

---

## 10. Open Questions

1. ~~**Hedging scope**~~: **Resolved** — Hedging is **enabled by default** for all operations.
   For writes, hedging is only active on multi-write-region (MWR) accounts. An opt-out option
   is available and overridable at `DriverRuntime`, `Driver`, and per-operation levels.

2. ~~**Dynamic hedging threshold**~~: **Resolved** — The hedging threshold is **dynamic by
   default**, based on observed P99 latency (preferring recent measurements for fast reaction).
   Hard-coded safety gates clamp the effective threshold to **50 ms – 4000 ms**; configurable
   min/max bounds can tighten this range further.

3. ~~**Sharding granularity**~~: **Resolved** — Track inflight per `(HttpClient, endpoint)` pair.
   Per-endpoint tracking gives more precise load balancing. This is especially important for
   multi-tenant applications where a single shared-tenant app uses separate Cosmos DB accounts
   per tenant.

4. ~~**Transport pipeline vs. policy chain**~~: **Resolved** — Yes, direct function calls are
   acceptable. There is no need for caller/customer-side extensibility given the fixed nature
   of Cosmos policies.

5. **HTTP/2 stream limit detection**: Given that the Cosmos service endpoint's 20-stream limit
   is very conservative (most service implementations allow at least 100), hitting this limit
   should be infrequent. Eventually, higher latency/timeouts from stream exhaustion will
   produce health signals that trigger shard eviction/scaling. The `max_streams_per_client`
   configuration should remain configurable to allow tuning. No change to the approach.

6. ~~**CPU-aware eviction**~~: **Resolved** — Yes, adopt CPU-aware eviction using
   `CpuMemoryMonitor`. Use a 90% CPU threshold, aligned with the Java SDK.

7. ~~**Metadata vs. dataplane sharding**~~: **Resolved** — both metadata and dataplane pipelines
   use `AdaptiveTransport`, which automatically selects sharded or plain transport based on the
   gateway's protocol. If the metadata endpoint supports HTTP/2 (ComputeGateway), it gets
   sharding. If HTTP/1.1 only (RoutingGateway), it uses plain transport. Different timeout
   configs are passed via `HttpClientConfig`.

8. ~~**`HttpClient` reuse across endpoints**~~: **Resolved** — Share `HttpClient` instances across
   endpoints. This is important for multi-tenant applications using separate Cosmos DB accounts
   per tenant, where per-endpoint clients would waste resources. Note: .NET and Java use the
   DNS hostname as the connection pool key (not the resolved IP). Research is needed on whether
   Rust's reqwest/hyper uses the same approach — if it resolves IPs for the pool key, sharing
   clients across endpoints would better utilize connection pools for multi-tenant scenarios.

9. ~~**`HttpClientFactory` abstraction level**~~: **Resolved** — The HTTP version preference is
   determined externally by the gateway probing logic (ALPN negotiation + `AccountProperties`
   Gateway 2.0 detection). The `HttpClientFactory` does not need HTTP version knobs. The
   default should come from the probing logic, with overrides available as proposed in §9.

10. ~~**`load_spread_ratio` tuning**~~: **Resolved** — Make `load_spread_ratio` **adaptive based
    on observed behavior** (request rate variance). Start with the default of 0.5 and adjust
    dynamically.

---

## Appendix: File Layout After Step 1

```text
sdk/cosmos/azure_data_cosmos_driver/src/
├── lib.rs
├── diagnostics/
│   ├── mod.rs
│   └── diagnostics_context.rs
├── driver/
│   ├── mod.rs
│   ├── cosmos_driver.rs          # Updated to use operation pipeline
│   ├── runtime.rs
│   ├── cache/
│   │   ├── mod.rs
│   │   ├── async_cache.rs
│   │   ├── async_lazy.rs
│   │   ├── container_cache.rs
│   │   └── account_metadata_cache.rs
│   ├── pipeline/                 # NEW
│   │   ├── mod.rs
│   │   ├── components.rs         # ECS-style state component types
│   │   ├── operation_pipeline.rs # Operation-level orchestration loop
│   │   ├── hedging.rs            # Hedging/speculative execution
│   │   └── retry_evaluation.rs   # Pure fn: evaluate_transport_result
│   ├── routing/                  # NEW (moved from azure_data_cosmos)
│   │   ├── mod.rs
│   │   ├── global_endpoint_manager.rs
│   │   ├── location_cache.rs
│   │   ├── partition_endpoint_manager.rs
│   │   └── session_manager.rs
│   ├── transport/
│   │   ├── mod.rs                # CosmosTransport (updated)
│   │   ├── transport_pipeline.rs # Transport-level pipeline (refactored)
│   │   ├── adaptive_transport.rs # NEW: AdaptiveTransport (H2/H1.1 dispatch)
│   │   ├── http_client_factory.rs# NEW: HttpClientFactory trait + default impl
│   │   ├── authorization_policy.rs
│   │   ├── headers_policy.rs
│   │   ├── tracked_transport.rs
│   │   ├── emulator.rs
│   │   ├── sharded_transport.rs  # NEW: ShardedHttpTransport (H2 only)
│   │   ├── shard_pool.rs         # NEW: EndpointShardPool + elasticity
│   │   └── shard_health.rs       # NEW: Health checking & eviction
│   └── fault_injection/          # NEW (moved from azure_data_cosmos)
│       ├── mod.rs
│       ├── rule.rs
│       ├── condition.rs
│       ├── result.rs
│       └── client_builder.rs
├── models/
│   └── (unchanged)
├── options/
│   ├── mod.rs
│   ├── connection_pool.rs        # Updated with ShardingConfig
│   ├── driver_options.rs
│   ├── operation_options.rs      # Updated with hedging/CB config
│   ├── retry.rs                  # NEW: RetryOptions
│   └── availability.rs           # NEW: CircuitBreakerConfig, HedgingConfig
└── system/
    └── (unchanged)
```

### Dependency Model

The driver crate follows `azure_core`'s abstraction model (same pattern as `azure_data_cosmos`):

```toml
# Cargo.toml (azure_data_cosmos_driver)

[dependencies]
azure_core = { workspace = true, features = ["hmac_rust"] }
# No direct dependency on reqwest or tokio in production code.
# The driver uses:
#   - azure_core::http::HttpClient trait for HTTP transport
#   - azure_core::async_runtime for spawn/sleep

[dev-dependencies]
# Concrete implementations for testing:
azure_core = { workspace = true, features = ["reqwest_native_tls"] }
tokio = { workspace = true, features = ["macros", "rt-multi-thread", "time"] }
```

At runtime, the application (or the `azure_data_cosmos` crate) activates the concrete `reqwest`
feature via its own `azure_core` dependency, which registers the reqwest-backed `HttpClient` as
the default. The driver's `HttpClientFactory` default implementation calls
`azure_core::http::new_http_client()` to obtain instances.

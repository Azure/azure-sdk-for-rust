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
11. [Crossbeam & Lock-Free Data Structures](#11-crossbeam--lock-free-data-structures)

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

/// Transport mode for a routed attempt.
///
/// Determines which HTTP transport (and protocol version) is used
/// for a given request attempt.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub(crate) enum TransportMode {
    /// Standard gateway (HTTP/1.1 or HTTP/2 via ALPN).
    Gateway,
    /// Gateway 2.0 (HTTP/2 with prior knowledge).
    Gateway20,
}

/// COMPONENT: Routing decision for the current attempt.
/// Produced by endpoint resolution, consumed by transport.
/// Replaced (not mutated) on retry/failover.
pub(crate) struct RoutingDecision {
    /// The resolved Cosmos DB service endpoint for the chosen region.
    pub endpoint: CosmosEndpoint,
    /// The concrete URL selected for this attempt.
    ///
    /// For dataplane operations this may be the Gateway 2.0 URL when the
    /// endpoint exposes one and the runtime configuration allows it.
    pub selected_url: Url,
    /// The transport mode for this attempt.
    pub transport_mode: TransportMode,
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
    /// Type-safe index into the preferred endpoint list for round-robin
    /// failover. Bundles a numeric index with a generation counter so
    /// `resolve_endpoint` can detect stale references after an account
    /// metadata refresh (see `LocationIndex`).
    pub location: LocationIndex,
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

/// COMPONENT: Type-safe index into a preferred endpoint list.
///
/// Bundles the numeric position with a `generation` counter from the
/// `AccountEndpointState` that produced it. The generation advances
/// each time the preferred endpoint list is rebuilt (e.g., after an
/// account metadata refresh triggered by `LocationEffect::RefreshAccountProperties`).
///
/// `resolve_endpoint` compares the generation in the `LocationIndex`
/// with the generation in the current `LocationSnapshot`. On mismatch
/// (the endpoint list changed), it resets the index to 0 and uses the
/// fresh list — preventing out-of-bounds access or stale region ordering.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub(crate) struct LocationIndex {
    /// Position in the preferred endpoint list.
    index: usize,
    /// Monotonically increasing counter tied to the
    /// `AccountEndpointState` that produced this index.
    generation: u64,
}

impl LocationIndex {
    /// Create an index at position 0 for the given generation.
    pub fn initial(generation: u64) -> Self {
        Self { index: 0, generation }
    }

    /// Advance to the next endpoint in the list (wrapping).
    pub fn next(self, list_len: usize) -> Self {
        Self {
            index: (self.index + 1) % list_len,
            generation: self.generation,
        }
    }

    /// The numeric index.
    pub fn index(self) -> usize {
        self.index
    }

    /// The generation this index was created for.
    pub fn generation(self) -> u64 {
        self.generation
    }

    /// Whether this index is current for the given generation.
    /// When stale, `resolve_endpoint` resets to index 0.
    pub fn is_current(self, generation: u64) -> bool {
        self.generation == generation
    }
}

pub(crate) struct FailedEndpoint {
    /// The endpoint that failed.
    pub endpoint: CosmosEndpoint,
    pub error: OperationError,
    pub request_sent: bool,
}

/// COMPONENT: Composite snapshot of all location routing state for one
/// operation attempt. Acquired **once** at the top of each loop iteration
/// and immutable for the lifetime of that attempt. All routing decisions
/// within the attempt are based on this snapshot.
///
/// Mutations (marking endpoints/partitions unavailable, refreshing account
/// metadata) are expressed as `LocationEffect` values (see §3.4) and applied
/// in a single, well-defined stage (STAGE 6 of the operation loop). The next
/// iteration re-acquires a fresh snapshot that includes the applied mutations.
pub(crate) struct LocationSnapshot {
    /// Account-level endpoint routing (preferred regions, unavailable endpoints).
    pub account: Arc<AccountEndpointState>,
    /// Partition-level circuit-breaker state.
    pub partitions: Arc<PartitionEndpointState>,
}

/// COMPONENT: Diagnostics accumulator for the operation.
/// Append-only across attempts.
// Reuse existing DiagnosticsContextBuilder (already mutable accumulator).

/// Newtype wrapper for a Cosmos DB Log Sequence Number (LSN).
///
/// LSNs are monotonically increasing unsigned 64-bit integers assigned by
/// the storage engine to each committed write within a partition. They are
/// used for session-consistency tracking, quorum reads, and
/// change-feed continuation.
#[derive(Clone, Copy, Debug, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub(crate) struct LogSequenceNumber(u64);

impl LogSequenceNumber {
    pub fn new(value: u64) -> Self {
        Self(value)
    }

    pub fn value(self) -> u64 {
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
    /// The regional endpoint selected for this attempt.
    ///
    /// This preserves region identity for diagnostics and retry evaluation
    /// even when the concrete URL uses Gateway 2.0.
    pub endpoint: CosmosEndpoint,
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
// Pure function over an immutable LocationSnapshot — no manager or store
// references. The snapshot is acquired once per loop iteration.
//
// Stale-index handling: if `retry_state.location.generation` does not
// match `location.account.generation`, the endpoint list has changed
// since the LocationIndex was created. In that case, the function
// resets the index to 0 and uses the fresh list from the snapshot.
//
// The returned `RoutingDecision` carries both the regional endpoint and the
// concrete target chosen for the current attempt (`selected_url` plus
// `transport_mode`) so request construction and transport selection are driven
// by one routing decision.
fn resolve_endpoint(
    operation: &CosmosOperation,
    retry_state: &OperationRetryState,
    location: &LocationSnapshot,
) -> RoutingDecision;

// SYSTEM: Evaluate the result of a transport attempt and decide the
// next OperationAction (complete, retry, failover, hedge, or abort),
// plus any location-state mutations to apply before the next iteration.
//
// The returned `Vec<LocationEffect>` captures all side-effects on
// location state (mark unavailable, refresh metadata, etc.). These are
// applied in STAGE 6 of the operation loop — the only place where
// location state is mutated.
fn evaluate_transport_result(
    operation: &CosmosOperation,
    result: &TransportResult,
    retry_state: &OperationRetryState,
    session_state: &SessionState,
) -> (OperationAction, Vec<LocationEffect>);

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

// SYSTEM: Execute a single transport attempt — build HTTP request,
// apply headers, sign, send, retry 429s and connectivity errors.
// Pure function over its inputs (no struct state).
async fn execute_transport_pipeline(
    request: TransportRequest,
    transport: &AdaptiveTransport,
    credential: &Credential,
    diagnostics: &mut DiagnosticsContextBuilder,
) -> TransportResult;

// SYSTEM: Apply standard Cosmos headers (x-ms-version, User-Agent,
// Content-Type, etc.) to an outgoing HTTP request.
fn apply_cosmos_headers(request: &mut Request);

// SYSTEM: Generate and attach the Authorization header.
fn sign_request(
    request: &mut Request,
    credential: &Credential,
) -> Result<()>;
```

### 3.4 Decision Enums

```rust
/// What the operation pipeline should do after an attempt.
///
/// `OperationAction` is purely a control-flow decision. Any side-effects
/// on location state (marking endpoints/partitions unavailable, refreshing
/// account metadata) are expressed separately as `LocationEffect` values
/// returned alongside this action from `evaluate_transport_result`.
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
    /// Start a hedged attempt in a secondary region.
    Hedge {
        secondary_routing: RoutingDecision,
    },
    /// Abort the operation with this error.
    Abort(azure_core::Error),
}

/// A mutation to apply to location state.
///
/// Produced by `evaluate_transport_result` alongside an `OperationAction`.
/// Applied in STAGE 6 of the operation loop — the **only** place where
/// location state is mutated. This keeps the mutation scope explicit,
/// traceable, and testable.
pub(crate) enum LocationEffect {
    /// Mark an endpoint as temporarily unavailable so future routing
    /// skips it until the unavailability expires.
    MarkEndpointUnavailable {
        endpoint: CosmosEndpoint,
        reason: UnavailableReason,
    },
    /// Mark a partition as unavailable in a specific region, triggering
    /// the partition-level circuit breaker.
    MarkPartitionUnavailable(UnavailablePartition),
    /// Trigger an async refresh of the `AccountMetadataCache` to obtain
    /// fresh `AccountProperties` (e.g., updated writable/readable locations).
    /// Rate-limited internally to avoid overwhelming the metadata endpoint
    /// during sustained failover scenarios.
    RefreshAccountProperties,
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

**Design principle**: The loop acquires an immutable `LocationSnapshot` once per iteration.
All routing decisions within that iteration use this snapshot — the transport pipeline never
sees the stores. Any mutations triggered by the transport result (marking
endpoints/partitions unavailable, refreshing account metadata) are expressed as `LocationEffect`
values returned from `evaluate_transport_result` and applied in a single, well-defined stage
(STAGE 6). This ensures:

1. Routing within an attempt is always based on a consistent point-in-time view.
2. Mutations are explicit, traceable, and testable (returned from a pure function).
3. The next iteration re-acquires a fresh snapshot that includes the applied mutations.

```rust
pub(crate) async fn execute_operation_pipeline(
    operation: &CosmosOperation,
    options: &OperationOptions,
    location_store: &LocationStateStore,
    transport: &CosmosTransport,
    diagnostics: &mut DiagnosticsContextBuilder,
) -> Result<CosmosResponse> {
    let mut retry_state = OperationRetryState::initial(
        operation, options, &location_store.account_snapshot(),
    );
    let mut session_state = SessionState::from_operation(operation, options);
    let deadline = options.e2e_deadline();

    loop {
        // ── STAGE 1: Acquire LocationSnapshot ──────────────────────────
        // One immutable snapshot per iteration. All routing decisions in
        // this iteration are based on this view. Mutations (STAGE 6)
        // apply to the stores; the next iteration re-snapshots.
        let location = location_store.snapshot();

        // ── STAGE 2: Resolve endpoint ──────────────────────────────────
        let routing = resolve_endpoint(
            operation,
            &retry_state,
            &location,
        );

        // ── STAGE 3: Build transport request ───────────────────────────
        let transport_request = build_transport_request(
            operation,
            &routing,
            &session_state,
            options,
            deadline,
        );

        let selected_transport = if uses_dataplane_pipeline(operation) {
            transport.get_dataplane_transport(routing.transport_mode)
        } else {
            transport.get_metadata_transport()
        };

        // ── STAGE 4: Execute via transport pipeline ────────────────────
        let result = execute_transport_pipeline(
            transport_request, &selected_transport, diagnostics,
        ).await;

        // ── STAGE 5: Evaluate result → action + location effects ───────
        // Pure function: decides what to do next AND what location state
        // to mutate. No mutation happens here — effects are just data.
        diagnostics.record_attempt(&result);
        let (action, effects) = evaluate_transport_result(
            operation,
            &result,
            &retry_state,
            &session_state,
        );

        // ── STAGE 6: Apply location effects ────────────────────────────
        // This is the ONLY place in the loop where location state is
        // mutated. The unified `LocationStateStore::apply` method
        // processes all effects, applying each via a CAS loop against
        // the *current* state (not the stale STAGE 1 snapshot). It
        // handles `RefreshAccountProperties` internally by delegating
        // to the `AccountMetadataCache`.
        location_store.apply(&effects).await;

        // ── STAGE 7: Act on the control-flow decision ──────────────────
        match action {
            OperationAction::Complete(result) => {
                return build_cosmos_response(result, diagnostics);
            }
            OperationAction::FailoverRetry { new_state, delay } => {
                if let Some(d) = delay { sleep(d).await; }
                retry_state = new_state;
                // → next iteration re-snapshots with updated location state
            }
            OperationAction::SessionRetry { new_state } => {
                retry_state = new_state;
                // Advance to next preferred region. In Gateway mode, in-region
                // replica retries are already handled by Gateway, so the
                // operation-level session retry moves to the next suitable
                // region (hub/write region for single-write accounts,
                // round-robin for multi-write accounts).
                retry_state = retry_state.advance_location(endpoints_len);
                // Check deadline — same as FailoverRetry.
                check_deadline(deadline)?;
            }
            OperationAction::Hedge { secondary_routing } => {
                // See §4.2 Hedging
                return execute_hedged(
                    operation, options, &routing, &secondary_routing,
                    &session_state, transport, credential, diagnostics, deadline,
                ).await;
            }
            OperationAction::Abort(error) => {
                return Err(error);
            }
        }
    }
}

// NOTE: `apply_location_effects` is now the `LocationStateStore::apply`
// method (§4.6). See that section for the full implementation.
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
    transport: &AdaptiveTransport,
    credential: &Credential,
    diagnostics: &mut DiagnosticsContextBuilder,
    deadline: Option<Instant>,
) -> Result<CosmosResponse> {
    let primary_req = build_transport_request(
        operation, primary_routing, session, options, deadline,
    );

    let hedging_threshold = options.hedging_threshold(); // e.g. 50ms

    // Start primary attempt
    let primary_fut = execute_transport_pipeline(
        primary_req, transport, credential, diagnostics,
    );

    // Race: primary completes within threshold, or we start secondary
    tokio::select! {
        result = primary_fut => {
            // Primary completed (success or error) before hedging threshold
            diagnostics.record_attempt(&result);
            evaluate_and_return(operation, result, diagnostics)
        }
        _ = sleep(hedging_threshold) => {
            // Primary still pending — start secondary in another region.
            //
            // The hedged attempt inherits the *same* e2e deadline as the
            // primary. By the time we reach this branch at least
            // `hedging_threshold` has elapsed, so the remaining budget is
            // `deadline - Instant::now()`. We do NOT recompute a fresh
            // deadline here — `build_transport_request` + the transport
            // pipeline already enforce the original deadline by clamping
            // the per-request timeout to `deadline - now` (see §5.1).
            // If the deadline has already passed, the transport pipeline
            // will return immediately with a timeout error.
            let secondary_req = build_transport_request(
                operation, secondary_routing, session, options, deadline,
            ).with_execution_context(ExecutionContext::Hedging);

            let secondary_fut = execute_transport_pipeline(
                secondary_req, transport, credential, diagnostics,
            );

            // Race primary vs secondary — first successful response wins
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
- **Deadline enforcement**: The hedged attempt shares the original e2e deadline.
  By the time the hedged attempt starts, at least `hedging_threshold` has
  elapsed, so the remaining budget is `deadline − now`. The transport pipeline
  enforces this by clamping the per-request timeout to `deadline − Instant::now()`
  before each HTTP call. If the deadline has already passed, the transport
  pipeline returns immediately with a timeout error — no unnecessary network
  I/O is performed.
- **Single secondary region**: Currently hedging is limited to one secondary
  region per operation. Architecturally nothing prevents hedging to a 3rd or
  4th region, but in practice multi-region outages (where more than one region
  fails simultaneously) are rare, and the cost of additional speculative
  requests (extra RU charges, additional network traffic) grows linearly.
  Limiting to one hedged attempt keeps the blast radius predictable. This
  constraint can be relaxed in a future iteration if telemetry shows a need.

### 4.3 `evaluate_transport_result` Decision Tree

This pure function replaces the complex `ClientRetryPolicy::should_retry` method. Because it is a
pure function over data components, it is trivially unit-testable.

```rust
fn evaluate_transport_result(
    operation: &CosmosOperation,
    result: &TransportResult,
    retry_state: &OperationRetryState,
    session_state: &SessionState,
) -> (OperationAction, Vec<LocationEffect>) {
    match &result.outcome {
        TransportOutcome::Success { .. } => {
            (OperationAction::Complete(result.clone()), vec![])
        }

        TransportOutcome::HttpError { status, request_sent, .. } => {
            match *status {
                // 403/3 WriteForbidden → refresh account metadata + endpoint failover.
                //
                // The write-forbidden error indicates a regional failover has
                // occurred (or this endpoint is no longer the write region).
                // Two effects:
                //   1. Refresh AccountMetadataCache to pick up the new topology.
                //   2. Mark the current endpoint unavailable so the next routing
                //      attempt skips it.
                //
                // NOTE: `advance_failover` (not `advance_failover_with_cache_refresh`)
                // is used here because the cache refresh is now expressed as a
                // LocationEffect and applied in STAGE 6.
                CosmosStatus::WRITE_FORBIDDEN => {
                    if retry_state.failover_retry_count < MAX_FAILOVER_RETRIES {
                        (
                            OperationAction::FailoverRetry {
                                new_state: retry_state.advance_failover(result),
                                delay: None,
                            },
                            vec![
                                LocationEffect::RefreshAccountProperties,
                                LocationEffect::MarkEndpointUnavailable {
                                    endpoint: result.endpoint().clone(),
                                    reason: UnavailableReason::WriteForbidden,
                                },
                            ],
                        )
                    } else {
                        (OperationAction::Abort(build_error(status)), vec![])
                    }
                }

                // 404/1002 ReadSessionNotAvailable → session retry.
                // In Gateway mode, in-region replica retries are already
                // handled by Gateway. The operation-level session retry
                // advances to the next preferred region (hub/write region
                // for single-write accounts, round-robin for multi-write).
                //
                // Default max retries (Java SDK parity):
                //   - Single-write: 2 (try local + hub + one more)
                //   - Multi-write: endpoints.len() (try each region once)
                // For single-write accounts, abort after 2 retries even if
                // max_session_retries is higher.
                CosmosStatus::READ_SESSION_NOT_AVAILABLE => {
                    if retry_state.session_token_retry_count < MAX_SESSION_RETRIES {
                        (
                            OperationAction::SessionRetry {
                                new_state: retry_state.advance_session_retry(),
                            },
                            vec![],
                        )
                    } else {
                        (OperationAction::Abort(build_error(status)), vec![])
                    }
                }

                // 429/3092 SystemResourceUnavailable, 503 (any sub-status),
                // 410 (any Gone sub-status) → failover retry + mark partition
                // unavailable.
                //
                // These indicate that a specific partition on this endpoint is
                // unhealthy. Mark the partition unavailable (circuit-breaker)
                // AND mark the endpoint unavailable so routing prefers a
                // different region.
                _ if status.is_system_resource_unavailable()
                    || status.is_service_unavailable()
                    || status.is_gone() =>
                {
                    (
                        OperationAction::FailoverRetry {
                            new_state: retry_state.advance_partition_failover(result),
                            delay: None,
                        },
                        vec![
                            LocationEffect::MarkPartitionUnavailable(
                                UnavailablePartition::from_result(result),
                            ),
                            LocationEffect::MarkEndpointUnavailable {
                                endpoint: result.endpoint().clone(),
                                reason: UnavailableReason::ServiceUnavailable,
                            },
                        ],
                    )
                }

                // 500 for reads → try next endpoint.
                // Mark the current endpoint unavailable because an internal
                // server error suggests a regional issue.
                _ if status.is_internal_server_error() && operation.is_read_only() => {
                    (
                        OperationAction::FailoverRetry {
                            new_state: retry_state.advance_failover(result),
                            delay: None,
                        },
                        vec![LocationEffect::MarkEndpointUnavailable {
                            endpoint: result.endpoint().clone(),
                            reason: UnavailableReason::InternalServerError,
                        }],
                    )
                }

                _ => (OperationAction::Abort(build_error(status)), vec![]),
            }
        }

        TransportOutcome::TransportError { request_sent, error, .. } => {
            match request_sent {
                // Request was never sent → safe to retry on any endpoint
                // for any operation type. No location effects because we
                // don't know whether the endpoint itself is unhealthy (the
                // error may be a local DNS or socket issue).
                RequestSentStatus::NotSent => {
                    if retry_state.failover_retry_count < MAX_CONNECTION_RETRIES {
                        (
                            OperationAction::FailoverRetry {
                                new_state: retry_state.advance_connection_retry(result),
                                delay: None,
                            },
                            vec![],
                        )
                    } else {
                        (OperationAction::Abort(error.clone()), vec![])
                    }
                }

                // Request was (possibly) sent, but failed with a transport
                // error. For read-only / idempotent operations we can safely
                // retry on a different endpoint. Mark the current endpoint
                // unavailable because the transport error suggests the
                // region is unreachable or degraded.
                _ if operation.is_read_only() || operation.is_idempotent() => {
                    (
                        OperationAction::FailoverRetry {
                            new_state: retry_state.advance_failover(result),
                            delay: None,
                        },
                        vec![LocationEffect::MarkEndpointUnavailable {
                            endpoint: result.endpoint().clone(),
                            reason: UnavailableReason::TransportError,
                        }],
                    )
                }

                // Non-idempotent write whose request was sent → cannot safely
                // retry. No location effects — we don't mark the endpoint
                // because the error may be transient and the write may have
                // succeeded on the server.
                _ => (OperationAction::Abort(error.clone()), vec![]),
            }
        }
    }
}
```

### 4.4 Account Endpoint State & Routing Systems

Following the DOP principle, the account-level endpoint routing is split into:

- **Data (Component)**: `AccountEndpointState` — an immutable snapshot of everything the
  routing system needs to pick an endpoint. Replaced atomically on refresh.
  Together with `PartitionEndpointState`, it forms the `LocationSnapshot` (§3.1) that
  the operation loop acquires once per iteration and passes to `resolve_endpoint`.
- **Store (Infrastructure)**: `LocationStateStore` — a unified store (§4.6) that holds both
  account-level and partition-level state behind separate epoch-guarded atomic pointers (§11.1).
  Provides a single `apply(&self, effects, cache)` method that applies all `LocationEffect`s
  atomically via CAS loops. Mutated exclusively in STAGE 6 of the operation loop.
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
    /// Monotonically increasing counter, incremented each time the
    /// preferred endpoint lists are rebuilt (e.g., after a metadata
    /// refresh). Used by `LocationIndex` for stale-detection.
    pub generation: u64,
    /// Ordered list of endpoints preferred for read operations, based on
    /// the account's configured preferred locations and available regions.
    pub preferred_read_endpoints: Vec<CosmosEndpoint>,
    /// Ordered list of endpoints preferred for write operations.
    pub preferred_write_endpoints: Vec<CosmosEndpoint>,
    /// Endpoints currently marked as unavailable, with the time they were
    /// marked and the reason. Keyed by `CosmosEndpoint` (which carries the
    /// `RegionName`) so that unavailability tracking is region-aware —
    /// `resolve_endpoint` can skip an entire region, not just a raw URL.
    pub unavailable_endpoints: HashMap<CosmosEndpoint, (Instant, UnavailableReason)>,
    /// Whether the account has multiple write regions enabled.
    pub multiple_write_locations_enabled: bool,
    /// Default (hub) endpoint — used as fallback when all regional
    /// endpoints are unavailable.
    pub default_endpoint: CosmosEndpoint,
}

// NOTE: The thread-safe store for `AccountEndpointState` (and
// `PartitionEndpointState`) lives in the unified `LocationStateStore`
// defined in §4.6.  See that section for the epoch-guarded atomic
// pointers, snapshot access, CAS-loop `apply`, and unconditional `swap`.

/// A Cosmos DB service endpoint for one logical region.
///
/// This is the canonical representation used throughout routing,
/// failover tracking, and diagnostics. A regional endpoint may expose
/// both the standard gateway URL and an optional Gateway 2.0 URL.
///
/// - **Global** endpoints usually use the pattern `{account}.documents.azure.com`.
///   DNS resolves them to the hub (default) region.
/// - **Regional** endpoints usually use the pattern `{account}-{region}.documents.azure.com`
///   and resolve directly to that region.
/// - **Gateway 2.0** endpoints are optional per-region dataplane endpoints,
///   surfaced as `gateway20_url` instead of a separate endpoint object.
#[derive(Clone, Debug, PartialEq, Eq, Hash)]
pub(crate) struct CosmosEndpoint {
    region: Option<RegionName>,
    gateway_url: Url,
    gateway20_url: Option<Url>,
}
```

With this model, endpoint availability is tracked at the region level.
If either the standard gateway path or the Gateway 2.0 path is found to be
unavailable, the entire `CosmosEndpoint` is marked unavailable and routing
skips that region until the unavailability expires.

**Systems (pure functions) operating on `AccountEndpointState`**:

```rust
// SYSTEM: Build an initial AccountEndpointState from account properties.
// Called at client init and on background metadata refresh.
// `previous_generation` is `None` on first init (generation starts at 0)
// and `Some(prev.generation)` on refresh — the returned state uses
// `previous_generation.map_or(0, |g| g + 1)` so `LocationIndex` holders
// can detect stale references.
fn build_account_endpoint_state(
    properties: &AccountProperties,
    default_endpoint: CosmosEndpoint,
    previous_generation: Option<u64>,
    gateway20_enabled: bool,
) -> AccountEndpointState;

// SYSTEM: Produce a new state with an endpoint marked unavailable.
// The old state is not mutated — a new snapshot is returned.
fn mark_endpoint_unavailable(
    state: &AccountEndpointState,
    endpoint: &CosmosEndpoint,
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
returns a new snapshot. The `LocationStateStore` (§4.6) is a trivial epoch-guarded holder — it
has no routing logic. The `AccountMetadataCache` remains a separate infrastructure component
responsible for async-fetching and caching `AccountProperties`; on refresh, it calls
`build_account_endpoint_state` and swaps the result into the store via
`location_store.swap_account(new_state)`.

This makes the read path fully lock-free (epoch pin + atomic load), keeps mutation explicit and
traceable, and means every system function can be unit-tested with a hand-constructed
`AccountEndpointState` — no mocking required.

### 4.5 Partition Endpoint State & Circuit-Breaker Systems

Same DOP split as §4.4: data component + system functions.  The store is the unified
`LocationStateStore` (§4.6).

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
    /// Circuit-breaker configuration thresholds (resolved from `CircuitBreakerOptions` §9.4).
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
    /// Read failure count within the counter reset window.
    pub read_failure_count: u32,
    /// Write failure count within the counter reset window.
    pub write_failure_count: u32,
    /// Last failure timestamp.
    pub last_failure: Instant,
    /// Whether the circuit is currently open (tripped).
    pub is_open: bool,
}

/// Identifies a partition + region to mark unavailable.
/// Carried in `LocationEffect::MarkPartitionUnavailable` so the operation
/// pipeline can apply the mutation in STAGE 6 without coupling to internals.
#[derive(Clone, Debug)]
pub(crate) struct UnavailablePartition {
    pub partition_key_range_id: String,
    pub region: RegionName,
    pub is_read: bool,
}

/// Resolved circuit-breaker configuration thresholds.
///
/// This is a resolved snapshot built from `CircuitBreakerOptions` (§9.4)
/// after layered option resolution. All fields are concrete (non-`Option`)
/// with defaults applied. Named `Config` (not `Options`) to distinguish
/// from the user-facing `CircuitBreakerOptions` in §9.4.
#[derive(Clone, Debug)]
pub(crate) struct CircuitBreakerConfig {
    /// Read failures within the counter reset window before tripping. Default: 2.
    pub read_failure_threshold: u32,
    /// Write failures within the counter reset window before tripping. Default: 5.
    pub write_failure_threshold: u32,
    /// Window after which failure counters reset if no new failures occur. Default: 300 seconds.
    pub counter_reset_window: Duration,
    /// How long a partition stays unavailable before a probe attempt. Default: 5s.
    pub unavailability_probe_delay: Duration,
    /// Background failback sweep interval. Default: 300s.
    pub failback_interval: Duration,
}

// NOTE: The thread-safe store for `PartitionEndpointState` lives in the
// unified `LocationStateStore` (§4.6), alongside `AccountEndpointState`.
```

**Systems (pure functions) operating on `PartitionEndpointState`**:

```rust
// SYSTEM: Produce a new state with the given partition marked unavailable.
// Called from STAGE 6 via `LocationStateStore::apply` (§4.6) when a
// `LocationEffect::MarkPartitionUnavailable` effect is applied.
fn mark_partition_unavailable(
    state: &PartitionEndpointState,
    partition: &UnavailablePartition,
) -> PartitionEndpointState;

// SYSTEM: Record a failure against a partition/region.
// Increments the failure counter for the current window and trips
// the circuit if the threshold is exceeded.
fn record_partition_failure(
    state: &PartitionEndpointState,
    key: &PartitionRegionKey,
    is_read: bool,
    now: Instant,
) -> PartitionEndpointState;

// SYSTEM: Record a success — closes the circuit for the given
// partition/region. Failure counters are not reset on success;
// they expire naturally when the counter reset window elapses
// without new failures.
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

**Key circuit-breaker thresholds** (from Java SDK reference, used as `CircuitBreakerOptions`
defaults):
- Read failures (in window) before trip: 2
- Write failures (in window) before trip: 5
- Counter reset window: 300 seconds
- Background failback interval: 300 seconds
- Partition unavailability duration before probe: 5 seconds

### 4.6 Unified `LocationStateStore`

Both `AccountEndpointState` and `PartitionEndpointState` are held behind a single
`LocationStateStore`. Each inner state has its own `crossbeam::epoch::Atomic<T>` — they're
independent atomics (no shared lock), so a CAS on one never contends with a CAS on the other.

The store exposes:
- **`snapshot()`** — returns a `LocationSnapshot` with consistent (point-in-time) views of both
  states.
- **`apply(effects)`** — processes a `&[LocationEffect]` slice, applying each effect atomically
  via the appropriate inner CAS loop. This is the single mutation entry point used by STAGE 6 of
  the operation loop.
- **`swap_account(state)`** / **`swap_partitions(state)`** — unconditional replacement of one
  sub-state (used by `AccountMetadataCache` on refresh, or bulk partition reset).

```rust
/// Unified thread-safe holder for all location routing state.
///
/// Combines the account-level endpoint routing state and the partition-level
/// circuit-breaker state into a single store. Each sub-state is stored
/// behind its own `crossbeam::epoch::Atomic<T>` — reads and writes to
/// account state do not contend with reads or writes to partition state.
///
/// Having a single store simplifies call sites: instead of threading two
/// store references and an `AccountMetadataCache` through the pipeline,
/// callers pass one `&LocationStateStore` and call `store.apply(effects)`.
pub(crate) struct LocationStateStore {
    account: crossbeam::epoch::Atomic<AccountEndpointState>,
    partitions: crossbeam::epoch::Atomic<PartitionEndpointState>,
    /// Reference to the metadata cache for `RefreshAccountProperties`.
    account_metadata_cache: Arc<AccountMetadataCache>,
}

impl LocationStateStore {
    /// Acquire a consistent snapshot of both sub-states.
    ///
    /// Each load is a single epoch-pinned atomic read — fully lock-free.
    /// The two loads are NOT jointly atomic, but that is fine: routing
    /// decisions are tolerant of a one-iteration lag between account and
    /// partition state (the next loop iteration re-snapshots anyway).
    pub fn snapshot(&self) -> LocationSnapshot {
        let guard = crossbeam::epoch::pin();
        LocationSnapshot {
            account: unsafe {
                self.account.load(Ordering::Acquire, &guard).deref()
            }.clone().into(),
            partitions: unsafe {
                self.partitions.load(Ordering::Acquire, &guard).deref()
            }.clone().into(),
        }
    }

    /// Convenience accessor for account state only (used during init).
    pub fn account_snapshot(&self) -> Arc<AccountEndpointState> {
        let guard = crossbeam::epoch::pin();
        unsafe {
            self.account.load(Ordering::Acquire, &guard).deref()
        }.clone().into()
    }

    /// Apply a batch of `LocationEffect`s.
    ///
    /// Each effect is applied via a CAS loop against the *current* state
    /// of the relevant sub-store — not a stale snapshot. On contention
    /// (concurrent hedged attempt, background refresh), the CAS retries
    /// automatically, re-reading current state and re-applying the
    /// mutation until it succeeds.
    ///
    /// `RefreshAccountProperties` delegates to the `AccountMetadataCache`,
    /// which may eventually call `swap_account` with fully rebuilt state.
    pub async fn apply(&self, effects: &[LocationEffect]) {
        for effect in effects {
            match effect {
                LocationEffect::MarkEndpointUnavailable { endpoint, reason } => {
                    let endpoint = endpoint.clone();
                    let reason = reason.clone();
                    self.apply_account(|current| {
                        mark_endpoint_unavailable(current, &endpoint, reason.clone())
                    });
                }
                LocationEffect::MarkPartitionUnavailable(partition) => {
                    let partition = partition.clone();
                    self.apply_partitions(|current| {
                        mark_partition_unavailable(current, &partition)
                    });
                }
                LocationEffect::RefreshAccountProperties => {
                    // Rate-limited async refresh — may eventually call
                    // `swap_account` with a freshly built state.
                    self.account_metadata_cache.refresh_if_stale().await;
                }
            }
        }
    }

    // ── CAS-loop helpers (private) ─────────────────────────────────────

    fn apply_account(&self, mut f: impl FnMut(&AccountEndpointState) -> AccountEndpointState) {
        let guard = crossbeam::epoch::pin();
        loop {
            let current = self.account.load(Ordering::Acquire, &guard);
            let current_ref = unsafe { current.deref() };
            let new_state = f(current_ref);
            match self.account.compare_exchange(
                current,
                crossbeam::epoch::Owned::new(new_state),
                Ordering::AcqRel,
                Ordering::Acquire,
                &guard,
            ) {
                Ok(old) => {
                    unsafe { guard.defer_destroy(old); }
                    return;
                }
                Err(_) => continue,
            }
        }
    }

    fn apply_partitions(
        &self,
        mut f: impl FnMut(&PartitionEndpointState) -> PartitionEndpointState,
    ) {
        let guard = crossbeam::epoch::pin();
        loop {
            let current = self.partitions.load(Ordering::Acquire, &guard);
            let current_ref = unsafe { current.deref() };
            let new_state = f(current_ref);
            match self.partitions.compare_exchange(
                current,
                crossbeam::epoch::Owned::new(new_state),
                Ordering::AcqRel,
                Ordering::Acquire,
                &guard,
            ) {
                Ok(old) => {
                    unsafe { guard.defer_destroy(old); }
                    return;
                }
                Err(_) => continue,
            }
        }
    }

    // ── Unconditional swap (full refresh) ──────────────────────────────

    /// Unconditionally replace the account endpoint state.
    ///
    /// Use after a full metadata refresh that rebuilds the state from
    /// scratch. Concurrent `apply_account` CAS loops will simply fail
    /// and retry against the freshly swapped value.
    pub fn swap_account(&self, new_state: AccountEndpointState) {
        let guard = crossbeam::epoch::pin();
        let old = self.account.swap(
            crossbeam::epoch::Owned::new(new_state),
            Ordering::AcqRel,
            &guard,
        );
        unsafe { guard.defer_destroy(old); }
    }

    /// Unconditionally replace the partition endpoint state.
    pub fn swap_partitions(&self, new_state: PartitionEndpointState) {
        let guard = crossbeam::epoch::pin();
        let old = self.partitions.swap(
            crossbeam::epoch::Owned::new(new_state),
            Ordering::AcqRel,
            &guard,
        );
        unsafe { guard.defer_destroy(old); }
    }
}
```

---

## 5. Transport Pipeline

### 5.1 Responsibilities

The transport pipeline handles a **single attempt** to execute an operation against a **specific
selected target URL** derived from a regional endpoint. It is responsible for:

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

Following the same DOP pattern as the operation pipeline, the transport pipeline is a bare
async function — not a struct with methods. Its dependencies (`AdaptiveTransport`,
`Credential`) are passed as parameters. Header application and request signing are also bare
functions rather than policy objects.

```rust
// SYSTEM: Execute a single transport attempt.
// Applies headers, signs, sends, and handles 429 / connectivity retries.
pub(crate) async fn execute_transport_pipeline(
    request: TransportRequest,
    transport: &AdaptiveTransport,
    credential: &Credential,
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

        // Build HTTP request, apply standard headers, sign
        let mut http_request = build_http_request(&request);
        apply_cosmos_headers(&mut http_request);
        sign_request(&mut http_request, credential)?;

        // Record transport start event
        let attempt_start = Instant::now();
        diagnostics.record_event(RequestEvent::TransportStart);

        // Execute via adaptive HTTP transport (sharded or plain)
        let http_result = transport.send(http_request).await;

        // Map to TransportResult
        let result = match http_result {
            Ok(response) => {
                diagnostics.record_event(RequestEvent::ResponseHeadersReceived);
                map_response(response, attempt_start, diagnostics).await
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
```

### 5.3 Policy Application (Bare Functions, Not a Chain)

Unlike the current design where policies are chained via `Policy` trait with `next[0].send()`, the
transport pipeline uses bare functions for header application and request signing. There are no
policy objects — just functions that take and mutate the request.

```rust
// Instead of:
//   pipeline = [HeadersPolicy, AuthPolicy, TrackedTransport]
//   pipeline.send(request, context)  // each calls next

// We do:
//   apply_cosmos_headers(&mut request);       // bare fn: mutates request headers
//   sign_request(&mut request, auth_context); // bare fn: adds Authorization header
//   transport.send(request)                   // sends the HTTP request
```

**Rationale**: The Cosmos pipeline has a fixed, small set of header/auth concerns. There's no
user-extensible policy chain. Bare functions are easier to debug (no indirection), easier to
test (call the function directly), and have less overhead. They also align with the DOP
principle of separating behavior from state — there's no `CosmosHeadersPolicy` or
`AuthorizationPolicy` struct holding state; the functions take exactly the inputs they need.

---

## 6. Adaptive HTTP Transport Layer

### 6.0 Gateway Flavors & Protocol Negotiation

Cosmos DB exposes **three gateway flavors** that differ in HTTP version support:

| Gateway            | Usage                | HTTP Version                   | Notes                  |
| ------------------ | -------------------- | ------------------------------ | ---------------------- |
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
///
/// `pools` uses `crossbeam::epoch::Atomic` for lock-free reads (§11.1).
/// The map is read on every HTTP request and mutated only when a new
/// endpoint is first seen or during health-sweep eviction.
pub(crate) struct ShardedHttpTransport {
    /// Configuration for the sharding behavior.
    config: ShardingConfig,
    /// Per-endpoint shard pools. Key = endpoint authority (host:port).
    /// Lock-free reads via epoch; writers clone-and-swap the HashMap.
    pools: crossbeam::epoch::Atomic<HashMap<EndpointKey, Arc<EndpointShardPool>>>,
    /// Factory for creating new HttpClient instances.
    client_factory: Arc<dyn HttpClientFactory>,
}

/// Configuration for HTTP/2 connection sharding.
///
/// This is a driver-internal resolved snapshot built from the sharding
/// fields on `ConnectionPoolOptions` (§9.1) after layered option resolution.
/// All fields are concrete (non-`Option`) with defaults applied.
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
/// The shard list is stored in a `crossbeam::epoch::Atomic` and swapped
/// atomically on writes (clone-on-write).  The read path (`select_shard`)
/// is fully lock-free — pin the epoch, load the pointer, iterate.
/// Mutations (add/remove shard) build a new `Vec` and `swap` +
/// `defer_destroy` the old one.
struct EndpointShardPool {
    endpoint: EndpointKey,
    config: ShardingConfig,
    /// The shards. Epoch-guarded for lock-free reads (§11.1).
    /// Replaced atomically on shard add/remove.
    shards: crossbeam::epoch::Atomic<Vec<Arc<ClientShard>>>,
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
/// Each field is wrapped in `CachePadded` to prevent false sharing
/// between cores updating adjacent counters concurrently (§11.3).
struct EndpointStats {
    /// Currently inflight requests to this endpoint on this shard.
    inflight: CachePadded<AtomicU32>,
    /// Timestamp of last successful response (nanos since epoch).
    last_success_nanos: CachePadded<AtomicU64>,
    /// Timestamp of last request (success or failure).
    last_request_nanos: CachePadded<AtomicU64>,
    /// Consecutive failure count (reset on success).
    consecutive_failures: CachePadded<AtomicU32>,
    /// Total requests sent.
    total_requests: CachePadded<AtomicU64>,
    /// Total failures.
    total_failures: CachePadded<AtomicU64>,
}

/// Shard-level health tracking (across all endpoints on this shard).
/// Fields are `CachePadded` to avoid false sharing (§11.3).
struct ShardHealth {
    /// Total inflight across all endpoints.
    total_inflight: CachePadded<AtomicU32>,
    /// Last successful response on any endpoint.
    last_success_nanos: CachePadded<AtomicU64>,
    /// Whether this shard is marked for eviction.
    marked_for_eviction: CachePadded<AtomicBool>,
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
    /// 1. Filter out shards marked for eviction **and** shards already at
    ///    `max_streams_per_client` — both are ineligible for new requests.
    /// 2. Compute the "active set" size: `ceil(eligible * load_spread_ratio)`.
    ///    Only shards in the active set are preferred for new requests.
    /// 3. Among the active set, pick the shard with the lowest inflight
    ///    count for this endpoint (least-loaded first).  If found, return it.
    /// 4. If no active-set shard has capacity, check the **remaining**
    ///    (draining) eligible shards — they still have room below
    ///    `max_streams_per_client`.  Pick least-loaded.  This avoids
    ///    creating a new shard when an existing one can absorb the request.
    /// 5. If *all* eligible shards are at capacity, create a new shard
    ///    (if under `max_clients_per_endpoint`).
    /// 6. If at max clients, fall back to the least-loaded shard across
    ///    the full pool (including saturated ones) to bound queue depth.
    // NOTE: This pseudocode uses `.read()` / `.write()` shorthand for
    // readability. The actual implementation should use epoch-pinned loads
    // and clone-on-write swaps via `crossbeam::epoch::Atomic` (see §11.1).
    fn select_shard(&self, endpoint: &EndpointKey) -> Arc<ClientShard> {
        let fallback = {
            let shards = self.shards.read();

            // Eligible shards: healthy AND below the stream limit.
            let max = self.config.max_streams_per_client;
            let mut eligible: Vec<_> = shards.iter()
                .filter(|s| !s.health.marked_for_eviction.load(Ordering::Relaxed))
                .map(|s| {
                    let inflight = s.endpoint_inflight(endpoint);
                    (s.clone(), inflight)
                })
                .filter(|(_, inflight)| *inflight < max)
                .collect();

            eligible.sort_by_key(|(_, inflight)| *inflight);

            // Active set: only the top N eligible shards are preferred.
            // Remaining eligible shards are left to drain (scale-down path)
            // but can still absorb overflow before we create a new shard.
            let active_count = (eligible.len() as f64 * self.config.load_spread_ratio)
                .ceil()
                .max(1.0) as usize;

            // Step 3: try active set first (least-loaded).
            let active_set = &eligible[..active_count.min(eligible.len())];
            if let Some((best, _)) = active_set.first() {
                return best.clone();
            }

            // Step 4: active set empty (all active shards saturated) —
            // check remaining eligible (draining) shards before scaling up.
            if let Some((best, _)) = eligible.first() {
                return best.clone();
            }

            // No eligible shard has capacity.  Build a fallback from the
            // full (unfiltered, including saturated) pool for step 6.
            let mut all: Vec<_> = shards.iter()
                .filter(|s| !s.health.marked_for_eviction.load(Ordering::Relaxed))
                .map(|s| {
                    let inflight = s.endpoint_inflight(endpoint);
                    (s.clone(), inflight)
                })
                .collect();
            all.sort_by_key(|(_, inflight)| *inflight);
            all
        }; // read lock released here

        // Step 5: all eligible shards at capacity → try to create a new one.
        if let Some(new_shard) = self.try_create_shard() {
            return new_shard;
        }

        // Step 6: at max clients → fall back to least-loaded across ALL
        // shards (including saturated / draining ones) to bound queue depth.
        fallback.first()
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
    /// This shard is the designated probe candidate: it should be evicted
    /// and replaced with a fresh connection to test whether the network
    /// has recovered.  Only ONE shard per sweep iteration receives this
    /// status (the one with the most consecutive failures).
    UnhealthyProbeCandidate,
    Idle,
}

enum EvictionReason {
    /// No successful response within read-delay limit despite having
    /// inflight requests.
    ReadHang,
    /// Consecutive failures exceeded threshold AND other shards to the
    /// same endpoint are succeeding (connection-level problem, not
    /// service-level).
    ConsecutiveFailuresWithHealthyPeers,
    /// No activity for longer than idle timeout.
    IdleTimeout,
}

fn check_shard_health(
    shard: &ClientShard,
    endpoint: &EndpointKey,
    peer_shards: &[Arc<ClientShard>],
    config: &ShardingConfig,
    /// `true` when the caller (health sweep) has designated *this* shard
    /// as the probe candidate for the current sweep iteration.
    /// See the "all-shards-failing" protocol below.
    is_probe_candidate: bool,
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

    // 3. Consecutive failures — two sub-cases:
    let consecutive = stats.consecutive_failures.load(Ordering::Relaxed);
    if consecutive >= config.consecutive_failure_threshold {
        // 3a. At least one peer is healthier → connection-level problem
        //     on *this* shard.  Evict it (subject to grace period).
        let peers_healthy = peer_shards.iter().any(|peer| {
            let peer_stats = peer.endpoint_stats(endpoint);
            let peer_consecutive = peer_stats.consecutive_failures.load(Ordering::Relaxed);
            let peer_last_success = peer_stats.last_success_nanos.load(Ordering::Relaxed);
            peer_consecutive < consecutive && peer_last_success > last_success
        });

        if peers_healthy {
            let grace_ok = (now_nanos - last_success)
                > config.eviction_grace_period.as_nanos() as u64;
            if grace_ok {
                return ShardHealthStatus::Unhealthy {
                    reason: EvictionReason::ConsecutiveFailuresWithHealthyPeers,
                };
            }
        }

        // 3b. NO peer is healthier — every shard is failing equally.
        //     This happens when an external event (network component
        //     upgrade, firewall rule change, NAT timeout) corrupts ALL
        //     existing TCP connections simultaneously.
        //
        //     We must NOT evict all shards at once (that would drop all
        //     inflight traffic).  Instead, the health sweep designates
        //     exactly ONE shard — the one with the most consecutive
        //     failures — as the "probe candidate".  Only that shard is
        //     evicted and immediately replaced with a fresh connection.
        //
        //     On the NEXT sweep iteration, one of two things has happened:
        //       • The fresh shard is succeeding → it becomes a "healthy
        //         peer", and subsequent sweeps evict the remaining bad
        //         shards one-by-one via the normal 3a path.
        //       • The fresh shard is also failing → it's a service-level
        //         outage, not a connection-level problem.  No further
        //         evictions are triggered (the probe candidate rotates
        //         to the worst shard again, but the pool isn't drained).
        //
        //     This limits churn to at most one eviction + replacement per
        //     sweep interval when the entire pool is unhealthy.
        if is_probe_candidate {
            return ShardHealthStatus::UnhealthyProbeCandidate;
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

**Probe-candidate selection** (in the health sweep loop):

Before iterating over shards, the sweep checks whether the pool is in an "all-shards-failing"
state.  If every non-eviction-marked shard has `consecutive_failures >= threshold` AND no shard
has a recent success, the sweep designates the shard with the **highest** `consecutive_failures`
(ties broken by oldest `last_success_nanos`) as the probe candidate.  Only that shard receives
`is_probe_candidate = true`; all others get `false`.

```rust
/// Determine the probe candidate when all shards are failing.
/// Returns `Some(shard_id)` if — and only if — all shards exceed the
/// failure threshold with no healthy peer among them.
fn pick_probe_candidate(
    shards: &[Arc<ClientShard>],
    endpoint: &EndpointKey,
    config: &ShardingConfig,
) -> Option<u64> {
    let mut all_failing = true;
    let mut worst: Option<(u64, u32, u64)> = None; // (id, consecutive, last_success)

    for shard in shards {
        if shard.health.marked_for_eviction.load(Ordering::Relaxed) {
            continue;
        }
        let stats = shard.endpoint_stats(endpoint);
        let consecutive = stats.consecutive_failures.load(Ordering::Relaxed);
        let last_success = stats.last_success_nanos.load(Ordering::Relaxed);

        if consecutive < config.consecutive_failure_threshold {
            all_failing = false;
            break;
        }

        // Track the worst shard (most failures, then oldest success).
        let dominated = worst.map_or(true, |(_, wc, ws)| {
            consecutive > wc || (consecutive == wc && last_success < ws)
        });
        if dominated {
            worst = Some((shard.id, consecutive, last_success));
        }
    }

    if all_failing { worst.map(|(id, _, _)| id) } else { None }
}
```

**Eviction process**:
1. Mark shard for eviction (`marked_for_eviction = true`)
2. Stop routing new requests to it (the `select_shard` algorithm already excludes
   eviction-marked shards from the eligible set)
3. Wait for inflight requests to drain (with a timeout)
4. Drop the `Arc<dyn HttpClient>` (the underlying client closes connections on final `Drop`)
5. If pool size drops below `min_clients_per_endpoint`, create a replacement
6. **Probe-candidate eviction** (`UnhealthyProbeCandidate`): same steps 1–5, but the
   replacement shard is created **immediately** (not deferred) — this is the fresh
   connection that tests whether the network has recovered.  On the next sweep:
   - If the replacement is succeeding → it becomes a "healthy peer" and remaining bad
     shards are evicted one-by-one via the normal `ConsecutiveFailuresWithHealthyPeers` path.
   - If the replacement is also failing → it's a service-level outage; the sweep picks a
     new probe candidate (the current worst), limiting churn to one replacement per sweep.

**Scale-down flow**: The `load_spread_ratio` and `idle_client_timeout` work together:

1. As load decreases, `select_shard` concentrates requests on the active set
   (`ceil(N * load_spread_ratio)` shards).
2. Shards outside the active set receive no new requests; their inflight drains to 0.
3. Once `idle_client_timeout` elapses with 0 inflight, the health sweep marks them `Idle`.
4. The sweep runs in two phases: **Phase 1** evicts unhealthy shards and back-fills to
   `min_clients_per_endpoint`. **Phase 2** removes idle shards only down to the minimum —
   using the *post-eviction* pool size.  Because idle shards are already-warm connections,
   the sweep keeps them around to absorb the loss from Phase 1 instead of creating new
   shards (which are expensive: TLS handshake + TCP setup + H2 negotiation).
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
    /// 1. **Phase 1** — Evicts unhealthy shards and back-fills to minimum.
    /// 2. **Phase 2** — Reclaims idle shards *above* minimum, using the
    ///    post-eviction pool size so that idle shards absorb the loss
    ///    instead of being removed and immediately replaced.
    /// 3. Proactively scales up if utilization is high.
    // NOTE: This pseudocode uses `.read()` / `.write()` shorthand for
    // readability. The actual implementation should use epoch-pinned loads
    // and clone-on-write swaps via `crossbeam::epoch::Atomic` (see §11.1).
    async fn health_sweep_loop(self: Arc<Self>) {
        loop {
            sleep(self.config.health_check_interval).await;

            for (endpoint, pool) in self.pools.read().iter() {
                let shards = pool.shards.read().clone();
                let min = pool.config.min_clients_per_endpoint as usize;

                // ── Phase 1: identify & remove unhealthy shards ──────────
                let mut to_evict = Vec::new();
                let mut idle_candidates = Vec::new();
                let mut probe_replacement_needed = false;

                for shard in &shards {
                    let is_probe = pick_probe_candidate(&shards, endpoint, &self.config)
                        .map_or(false, |id| id == shard.id);
                    let status = check_shard_health(
                        shard, endpoint, &shards, &self.config, is_probe,
                    );
                    match status {
                        ShardHealthStatus::Unhealthy { reason } |
                        ShardHealthStatus::UnhealthyProbeCandidate { reason } => {
                            shard.health.marked_for_eviction.store(true, Ordering::Relaxed);
                            if matches!(status, ShardHealthStatus::UnhealthyProbeCandidate { .. }) {
                                probe_replacement_needed = true;
                            }
                            to_evict.push(shard.id);
                        }
                        ShardHealthStatus::Idle => {
                            // Collect idle candidates; we'll decide which to remove
                            // in Phase 2 based on post-eviction pool size.
                            idle_candidates.push(shard.id);
                        }
                        _ => {}
                    }
                }

                // Remove unhealthy shards and bring pool back to minimum.
                // Creating new shards is relatively expensive (TLS handshake,
                // TCP setup, H2 negotiation), so we defer idle removal until
                // after this step — keeping idle shards around avoids replacing
                // them with brand-new ones when the pool would otherwise drop
                // below `min_clients_per_endpoint`.
                if !to_evict.is_empty() {
                    let mut shards_mut = pool.shards.write();
                    shards_mut.retain(|s| !to_evict.contains(&s.id));

                    // For probe-candidate evictions, create a replacement
                    // immediately (the fresh connection tests recovery).
                    if probe_replacement_needed {
                        if let Some(new) = pool.create_shard_internal() {
                            shards_mut.push(Arc::new(new));
                        }
                    }

                    // Back-fill to minimum if still short.
                    while shards_mut.len() < min {
                        if let Some(new) = pool.create_shard_internal() {
                            shards_mut.push(Arc::new(new));
                        } else {
                            break; // factory failure; retry on next sweep
                        }
                    }
                }

                // ── Phase 2: remove idle shards (only those above minimum) ─
                // Decision is based on the *current* pool size, which already
                // accounts for the evictions (and any replacements) from Phase 1.
                // This avoids a pathological pattern where evicting N unhealthy
                // shards drops the pool below minimum, idle shards that could
                // have filled those slots were also removed, and N new shards
                // are created unnecessarily.
                if !idle_candidates.is_empty() {
                    let mut shards_mut = pool.shards.write();
                    let can_remove = shards_mut.len().saturating_sub(min);
                    if can_remove > 0 {
                        let remove_count = can_remove.min(idle_candidates.len());
                        let remove_set = &idle_candidates[..remove_count];
                        shards_mut.retain(|s| !remove_set.contains(&s.id));
                    }
                    // No back-fill needed: we only remove down to `min`.
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
  execute_transport_pipeline()
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
   transport-level and operation-level retry/failover logic, since
   `execute_transport_pipeline` sees the error responses from the sharded transport as usual.

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

| Sub-step | Work Item                                                                                                                                                                                                                                                                                                                                                                                                                            | Files                                                          |
| -------- | ------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------ | -------------------------------------------------------------- |
| 1.1      | **State components** — Define ECS-style component types (`RoutingDecision`, `OperationRetryState`, `TransportRequest`, `TransportResult`, `ThrottleRetryState`, decision enums)                                                                                                                                                                                                                                                      | `driver/pipeline/components.rs`                                |
| 1.2      | **Transport pipeline (slim)** — Implement `execute_transport_pipeline` function with `apply_cosmos_headers`, `sign_request`, and 429 throttle retry only. Uses a plain `Arc<dyn HttpClient>` (no `AdaptiveTransport` yet). Hard-coded retry limits + backoff.                                                                                                                                                                        | `driver/transport/transport_pipeline.rs`                       |
| 1.3      | **Operation pipeline (slim)** — Implement `execute_operation_pipeline` core loop with single-region endpoint resolution. Include happy path plus minimal operation-level transport retries: retry `TransportError` only when request is definitely not sent or operation is idempotent and retry budget remains; propagate non-429 HTTP errors as abort (429 remains transport-level). No failover, no hedging. Hard-coded deadline. | `driver/pipeline/operation_pipeline.rs`, `retry_evaluation.rs` |
| 1.4      | **Wire `execute_operation`** — Connect `CosmosDriver::execute_operation` to the new operation pipeline for all operations. The old pipeline code path remains but is no longer called.                                                                                                                                                                                                                                               | `driver/cosmos_driver.rs`                                      |
| 1.5      | **Unit tests** — Tests for each pure function (`evaluate_transport_result` happy path + minimal transport-error retry semantics, `evaluate_transport_retry` for 429, `build_transport_request`).                                                                                                                                                                                                                                     | `tests/`                                                       |

**What works after Step 1**: Operations flow through the new pipeline end-to-end against a
single region with basic 429 retry and minimal safe transport-error retries (not-sent or
idempotent-only within budget). The architecture is in place for incremental additions.

### Step 2: Multi-region failover & endpoint management

Add cross-region failover, `AccountEndpointState` + routing systems, and the `AccountMetadataCache` integration.

| Sub-step | Work Item                                                                                                                                                                                                                                                                                      | Files                                                                                                 |
| -------- | ---------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------- | ----------------------------------------------------------------------------------------------------- |
| 2.1      | **Routing state & systems** — Implement `AccountEndpointState`, the unified `LocationStateStore` (§4.6), and the system functions (`build_account_endpoint_state`, `mark_endpoint_unavailable`, `expire_unavailable_endpoints`). Wire to existing `AccountMetadataCache`.                      | `driver/routing/mod.rs`, `account_endpoint_state.rs`, `location_state_store.rs`, `routing_systems.rs` |
| 2.2      | **Expand `evaluate_transport_result`** — Add 403.3 (WriteForbidden + cache refresh), 503, 404/1022, 429/3092, 500-for-reads. Return `(OperationAction, Vec<LocationEffect>)` tuples; add `FailoverRetry`, `SessionRetry` action handling and STAGE 6 effect application in the operation loop. | `driver/pipeline/retry_evaluation.rs`, `operation_pipeline.rs`                                        |
| 2.3      | **Deadline enforcement** — Implement active deadline enforcement in transport pipeline (per-request timeout clamping, stream drop).                                                                                                                                                            | `driver/transport/transport_pipeline.rs`                                                              |
| 2.4      | **Config surface (initial)** — Add basic retry and failover options to `RetryOptions` / `CosmosRuntimeOptions` / `CosmosClientOptions` (§9). Hard-coded defaults, environment variable overrides.                                                                                              | `options/retry.rs`, `options/availability.rs`                                                         |
| 2.5      | **Tests** — Unit tests for each retry scenario in `evaluate_transport_result`, integration tests for multi-region failover.                                                                                                                                                                    | `tests/`                                                                                              |

**What works after Step 2**: Full multi-region failover, 403.3 recovery with cache refresh,
session retry, deadline enforcement. Still HTTP/1.1, no hedging, no circuit breaker.

### Step 3: Session consistency & partition-level circuit breaker

| Sub-step | Work Item                                                                                                                                                                                                                                                                                                                                                                                                   | Files                                                                                                 |
| -------- | ----------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------- | ----------------------------------------------------------------------------------------------------- |
| 3.1      | **Session tracking** — Session token management (resolve, propagate, track LSN).                                                                                                                                                                                                                                                                                                                            | `driver/routing/session_manager.rs`                                                                   |
| 3.2      | **Partition endpoint state & circuit breaker** — Implement `PartitionEndpointState`, wire it into the existing `LocationStateStore` (§4.6), add `CircuitBreakerConfig` (resolved from `CircuitBreakerOptions` §9.4), and the system functions (`mark_partition_unavailable`, `record_partition_failure/success`, `sweep_partition_health`). Hard-coded default thresholds (read: 2, write: 5, reset: 5min). | `driver/routing/partition_endpoint_state.rs`, `location_state_store.rs`, `circuit_breaker_systems.rs` |
| 3.3      | **`ReadConsistencyStrategy`** — Wire `effective_read_consistency_strategy` into `SessionState` and endpoint resolution.                                                                                                                                                                                                                                                                                     | `driver/pipeline/components.rs`, `operation_pipeline.rs`                                              |
| 3.4      | **Circuit breaker config** — Make thresholds configurable via `CircuitBreakerOptions`.                                                                                                                                                                                                                                                                                                                      | `options/availability.rs`                                                                             |
| 3.5      | **Tests** — Session consistency retry, circuit breaker trip/reset, partition failover.                                                                                                                                                                                                                                                                                                                      | `tests/`                                                                                              |

**What works after Step 3**: Full session consistency routing, partition-level circuit breaker
with failback. Still HTTP/1.1, no hedging.

### Step 4: Hedging (speculative execution)

| Sub-step | Work Item                                                                                                                                                                    | Files                        |
| -------- | ---------------------------------------------------------------------------------------------------------------------------------------------------------------------------- | ---------------------------- |
| 4.1      | **Hedging implementation** — `execute_hedged` with `tokio::select!` racing. Initial/Hedging `ExecutionContext` tracking. Static threshold first.                             | `driver/pipeline/hedging.rs` |
| 4.2      | **Dynamic threshold** — P99 latency tracker with safety gates (50–4000 ms).                                                                                                  | `driver/pipeline/hedging.rs` |
| 4.3      | **Hedging config** — `HedgingThreshold` enum (Dynamic/Static), `HedgingOptions` with `enabled` flag (§9.3). Nested in `RetryOptions`. Overridable at Runtime/Account layers. | `options/availability.rs`    |
| 4.4      | **Tests** — Hedging fires after threshold, primary wins, secondary wins, both fail, write hedging on MWR only.                                                               | `tests/`                     |

**What works after Step 4**: Full hedging with dynamic P99-based threshold. Still HTTP/1.1.

### Step 5: HTTP/2 support & adaptive transport (no sharding yet)

Add protocol detection and the `AdaptiveTransport` enum, but without the sharded transport —
HTTP/2 just uses a single `Arc<dyn HttpClient>` like HTTP/1.1.

| Sub-step | Work Item                                                                                                                                                                                                                 | Files                                     |
| -------- | ------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------- | ----------------------------------------- |
| 5.1      | **Gateway detection** — Gateway 2.0 detection via `thinClient*Locations` in `AccountProperties`. HTTP/2 vs HTTP/1.1 selection via `is_http2_allowed` configuration flag (runtime ALPN probing deferred — see note below). | `driver/transport/adaptive_transport.rs`  |
| 5.2      | **`AdaptiveTransport` enum** — `Sharded` / `Plain` dispatch. Initially both paths use a plain `Arc<dyn HttpClient>`.                                                                                                      | `driver/transport/adaptive_transport.rs`  |
| 5.3      | **`HttpClientFactory` trait** — Pluggable factory + default reqwest-backed implementation. `HttpClientConfig` struct.                                                                                                     | `driver/transport/http_client_factory.rs` |
| 5.4      | **Tests** — Protocol detection, factory creates clients, adaptive dispatch.                                                                                                                                               | `tests/`                                  |

**What works after Step 5**: HTTP/2 requests work (via a single connection). Gateway 2.0
endpoints are detected and used. No sharding yet — stream limit may be hit under high load.

> **Note on ALPN probing (§6.0):** The initial Step 5 implementation uses configuration flags
> (`is_http2_allowed`, `is_gateway20_allowed`) and `AccountProperties` metadata
> (`thinClient*Locations`) to determine the transport strategy, rather than runtime ALPN
> negotiation against the gateway. This is sufficient because:
> (1) reqwest with `http2` feature already performs ALPN automatically for `Http2Preferred`,
> (2) Gateway 2.0 is definitively identified by the presence of thin-client locations in
> account metadata, and (3) `http2_prior_knowledge()` for `Http2Only` skips ALPN entirely
> (h2 is guaranteed). Runtime probing may be revisited if a use case arises where the
> configuration-based approach is insufficient.

### Step 6: HTTP/2 connection sharding

| Sub-step | Work Item                                                                                                                                                                                                                                                     | Files                                                    |
| -------- | ------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------- | -------------------------------------------------------- |
| 6.1      | **`ShardedHttpTransport`** — Core shard pool with `EndpointShardPool`, `ClientShard`, inflight tracking via `CachePadded` atomics. Epoch-guarded (`crossbeam::epoch::Atomic`) snapshot-swap for lock-free shard reads (§11.1).                                | `driver/transport/sharded_transport.rs`, `shard_pool.rs` |
| 6.2      | **Shard selection algorithm** — `select_shard` with `load_spread_ratio`, active set, least-loaded routing, scale-up on capacity.                                                                                                                              | `driver/transport/shard_pool.rs`                         |
| 6.3      | **`ShardingConfig`** — All knobs (`max_streams_per_client`, `max_clients_per_endpoint = num_cpus * 2`, `min_clients_per_endpoint`, `load_spread_ratio`, `idle_client_timeout`). Resolved from `ConnectionPoolOptions` (§9.1). Environment variable overrides. | `options/connection_pool.rs`                             |
| 6.4      | **Wire into `AdaptiveTransport`** — When HTTP/2 is detected, use `ShardedHttpTransport` instead of plain.                                                                                                                                                     | `driver/transport/adaptive_transport.rs`                 |
| 6.5      | **Tests** — Shard selection under load, scale-up, inflight tracking, multi-endpoint pools.                                                                                                                                                                    | `tests/`                                                 |

**What works after Step 6**: HTTP/2 with connection sharding and elastic scale-up. No health
checks or eviction yet — shards accumulate but do not get reclaimed.

### Step 7: Health checks, eviction, TCP keepalive & connectivity retry

| Sub-step | Work Item                                                                                                                                                          | Files                                     |
| -------- | ------------------------------------------------------------------------------------------------------------------------------------------------------------------ | ----------------------------------------- |
| 7.1      | **Shard health checks** — `check_shard_health` (read-hang, consecutive failures with healthy peers, idle timeout). CPU-aware eviction guard (90% threshold).       | `driver/transport/shard_health.rs`        |
| 7.2      | **Background health sweep** — `health_sweep_loop` (evict unhealthy, reclaim idle, proactive scale-up). Scale-down via `load_spread_ratio` + `idle_client_timeout`. | `driver/transport/shard_health.rs`        |
| 7.3      | **TCP keepalive** — Enable TCP keepalive on all `HttpClient` instances (30s interval). Research reqwest/hyper APIs.                                                | `driver/transport/http_client_factory.rs` |
| 7.4      | **Connectivity retry in transport** — Expand `evaluate_transport_retry` to retry I/O errors and timeouts on a different shard (for idempotent/not-sent requests).  | `driver/transport/transport_pipeline.rs`  |
| 7.5      | **Tests** — Shard eviction, idle reclaim, scale-down flow, keepalive config, connectivity retry on different shard.                                                | `tests/`                                  |

**What works after Step 7**: Full HTTP/2 sharding with health monitoring, automatic eviction
& scale-down, TCP keepalive, and connectivity-level retry via shard rotation.

### Step 8: Fault injection

| Sub-step | Work Item                                                                                                                                                                                                                     | Files                                                                            |
| -------- | ----------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------- | -------------------------------------------------------------------------------- |
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
| -------- | -------------------------------------------------------------------------------------------------- | --------------------------------------------------- |
| 9.1      | Wire `ContainerClient::read_item` to build `CosmosOperation` and call `driver.execute_operation()` | `azure_data_cosmos/src/clients/container_client.rs` |
| 9.2      | Wire `ContainerClient::create_item` similarly                                                      | `azure_data_cosmos/src/clients/container_client.rs` |
| 9.3      | Integration tests verifying read/create through the new pipeline                                   | `tests/`                                            |

**What works after Step 9**: `readItem` and `createItem` flow through the new driver pipeline.
All other operations still use the old pipeline. No code is removed yet.

### Step 10: Cut over `azure_data_cosmos` — Phase 2 (full cut-over + cleanup)

Cut over all remaining operations and remove the old pipeline code.

| Sub-step | Work Item                                                                                             | Files                                |
| -------- | ----------------------------------------------------------------------------------------------------- | ------------------------------------ |
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

This section describes the new transport-pipeline options introduced by this spec. All option
types follow the conventions defined in the
[Configuration Options Specification](https://aka.ms/rust_azure_data_cosmos_ConfigurationOptions):

- All fields `pub`, all `Option<T>`, `#[non_exhaustive]`, `Default`, fluent `with_*` setters.
- `#[derive(CosmosOptions)]` generates `View` structs, `from_env()`, and builders.
- Environment variables use the `AZURE_COSMOS_` prefix + `SCREAMING_SNAKE_CASE`.
- Environment variables are read once at SDK init via `from_env()` and cached.

### 9.1 Additions to `ConnectionPoolOptions`

**Layers:** Runtime, Account *(nested inside `ConnectionOptions` via `#[option(nested)]`)*

The existing `ConnectionPoolOptions` (§3.3 of the Configuration Options spec) gains sharding
fields for HTTP/2 connection management. These are only effective when the `AdaptiveTransport`
selects the sharded path (HTTP/2 gateways — ComputeGateway or Gateway 2.0). They are ignored
when the gateway supports only HTTP/1.1 (RoutingGateway).

```rust
#[derive(CosmosOptions)]
#[options(layers(runtime, account))]
pub struct ConnectionPoolOptions {
    // --- Existing fields (from Configuration Options spec §3.3) ---

    /// How long idle connections are kept alive.
    pub idle_timeout: Option<Duration>,
    /// Maximum number of connections in the pool.
    pub max_connections: Option<usize>,

    // --- New fields for HTTP/2 sharding (this spec) ---

    /// Maximum concurrent HTTP/2 streams per `HttpClient` shard per endpoint
    /// before a new shard is created. Default: 16 (leaves headroom below the
    /// Cosmos gateway's 20-stream H2 limit; see `ShardingConfig` §6.2).
    pub max_streams_per_client: Option<usize>,
    /// Maximum number of `HttpClient` shards per endpoint. Default: `num_cpus * 2`.
    pub max_clients_per_endpoint: Option<usize>,
    /// Minimum number of `HttpClient` shards per endpoint. The pool never
    /// scales below this count. Default: 1.
    pub min_clients_per_endpoint: Option<usize>,
    /// Fraction of shards in the "active set" that receive new requests.
    /// Shards outside the active set drain and are eventually reclaimed.
    /// Default: 0.5.
    pub load_spread_ratio: Option<f64>,
    /// Consecutive failures on a shard (with healthy peers) before eviction.
    /// Default: 5.
    pub consecutive_failure_threshold: Option<u32>,
    /// Grace period after last successful request before a shard can be
    /// evicted due to consecutive failures. Prevents premature eviction
    /// during transient issues. Default: 2 s (see `ShardingConfig` §6.2).
    pub eviction_grace_period: Option<Duration>,
    /// Time a shard must be idle (0 inflight, 0 new requests) before
    /// the health sweep reclaims it. Default: 60 s.
    pub idle_client_timeout: Option<Duration>,
    /// Interval between background health-sweep iterations. Default: 5 s.
    pub health_check_interval: Option<Duration>,
    /// When average inflight / `max_streams_per_client` across active shards
    /// exceeds this ratio, a new shard is created proactively (off the
    /// request hot path). Default: 0.75.
    pub scale_up_threshold_ratio: Option<f64>,
}
```

| Option                          | Type               | Env Var                                           | Notes                                        |
| ------------------------------- | ------------------ | ------------------------------------------------- | -------------------------------------------- |
| `idle_timeout`                  | `Option<Duration>` | `AZURE_COSMOS_POOL_IDLE_TIMEOUT`                  | *(existing)*                                 |
| `max_connections`               | `Option<usize>`    | `AZURE_COSMOS_POOL_MAX_CONNECTIONS`               | *(existing)*                                 |
| `max_streams_per_client`        | `Option<usize>`    | `AZURE_COSMOS_POOL_MAX_STREAMS_PER_CLIENT`        | **New.** H2 stream limit per shard.          |
| `max_clients_per_endpoint`      | `Option<usize>`    | `AZURE_COSMOS_POOL_MAX_CLIENTS_PER_ENDPOINT`      | **New.** Upper bound on shards per endpoint. |
| `min_clients_per_endpoint`      | `Option<usize>`    | `AZURE_COSMOS_POOL_MIN_CLIENTS_PER_ENDPOINT`      | **New.** Lower bound on shards per endpoint. |
| `load_spread_ratio`             | `Option<f64>`      | `AZURE_COSMOS_POOL_LOAD_SPREAD_RATIO`             | **New.** Active-set fraction for scale-down. |
| `consecutive_failure_threshold` | `Option<u32>`      | `AZURE_COSMOS_POOL_CONSECUTIVE_FAILURE_THRESHOLD` | **New.** Eviction trigger.                   |
| `eviction_grace_period`         | `Option<Duration>` | `AZURE_COSMOS_POOL_EVICTION_GRACE_PERIOD`         | **New.**                                     |
| `idle_client_timeout`           | `Option<Duration>` | `AZURE_COSMOS_POOL_IDLE_CLIENT_TIMEOUT`           | **New.** Per-shard idle reclaim.             |
| `health_check_interval`         | `Option<Duration>` | `AZURE_COSMOS_POOL_HEALTH_CHECK_INTERVAL`         | **New.** Sweep cadence.                      |
| `scale_up_threshold_ratio`      | `Option<f64>`      | `AZURE_COSMOS_POOL_SCALE_UP_THRESHOLD_RATIO`      | **New.** Proactive scale-up trigger.         |

### 9.2 Additions to `RetryOptions`

**Layers:** Runtime, Account

The existing `RetryOptions` (§3.5 of the Configuration Options spec) gains nested groups for
hedging and circuit-breaker threshold tuning. The top-level `enable_partition_level_circuit_breaker`
and `disable_partition_level_failover` flags are already defined there; the new nested groups
provide fine-grained knobs.

```rust
#[derive(CosmosOptions)]
#[options(layers(runtime, account))]
pub struct RetryOptions {
    // --- Existing fields (from Configuration Options spec §3.5) ---

    /// Nested group for session-consistency retry behavior on 404/1002 errors.
    #[option(nested)]
    pub session_retry: Option<SessionRetryOptions>,
    /// Enable partition-level circuit breaker for transient failure isolation.
    pub enable_partition_level_circuit_breaker: Option<bool>,
    /// Disable automatic partition-level failover to other replicas.
    pub disable_partition_level_failover: Option<bool>,

    // --- New nested groups (this spec) ---

    /// Hedging (speculative execution) configuration.
    #[option(nested)]
    pub hedging: Option<HedgingOptions>,
    /// Fine-grained circuit-breaker threshold tuning.
    #[option(nested)]
    pub circuit_breaker: Option<CircuitBreakerOptions>,
}
```

### 9.3 `HedgingOptions` *(new nested group)*

**Layers:** Runtime, Account *(nested inside `RetryOptions` via `#[option(nested)]`)*

Controls speculative hedging of requests. Hedging is enabled by default with a dynamic
threshold based on observed P99 latency (clamped to safety gates). For writes, hedging is
only active on multi-write-region (MWR) accounts.

```rust
#[derive(CosmosOptions)]
#[options(layers(runtime, account))]
pub struct HedgingOptions {
    /// Whether hedging is enabled. Default: `true`.
    pub enabled: Option<bool>,
    /// Threshold strategy. Default: `HedgingThreshold::Dynamic { min: 50ms, max: 4000ms }`.
    pub threshold: Option<HedgingThreshold>,
}
```

| Option      | Type                       | Env Var                             | Notes                                                                     |
| ----------- | -------------------------- | ----------------------------------- | ------------------------------------------------------------------------- |
| `enabled`   | `Option<bool>`             | `AZURE_COSMOS_HEDGING_ENABLED`      | Set `false` to disable hedging entirely.                                  |
| `threshold` | `Option<HedgingThreshold>` | `AZURE_COSMOS_HEDGING_THRESHOLD_MS` | Static threshold in ms; dynamic threshold is configured programmatically. |

```rust
/// Strategy for computing the hedging delay.
#[derive(Clone, Debug, PartialEq)]
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

> **Environment variable note:** When `AZURE_COSMOS_HEDGING_THRESHOLD_MS` is set to a numeric
> value, the SDK uses `HedgingThreshold::Static(Duration::from_millis(value))`. The dynamic
> threshold with custom bounds is only configurable programmatically.

### 9.4 `CircuitBreakerOptions` *(new nested group)*

**Layers:** Runtime, Account *(nested inside `RetryOptions` via `#[option(nested)]`)*

Fine-grained thresholds for the partition-level circuit breaker. The top-level
`RetryOptions.enable_partition_level_circuit_breaker` controls whether the circuit breaker is
active; these options tune its behavior when enabled.

```rust
#[derive(CosmosOptions)]
#[options(layers(runtime, account))]
pub struct CircuitBreakerOptions {
    /// Read failures within the counter reset window before tripping. Default: 2.
    pub read_failure_threshold: Option<u32>,
    /// Write failures within the counter reset window before tripping. Default: 5.
    pub write_failure_threshold: Option<u32>,
    /// Window after which failure counters reset if no new failures occur. Default: 300 seconds.
    pub counter_reset_window: Option<Duration>,
    /// Background failback interval. Default: 300 seconds.
    pub failback_interval: Option<Duration>,
    /// Duration a partition must be unavailable before probe. Default: 5 seconds.
    pub unavailability_probe_delay: Option<Duration>,
}
```

| Option                       | Type               | Env Var                                      | Notes                                                         |
| ---------------------------- | ------------------ | -------------------------------------------- | ------------------------------------------------------------- |
| `read_failure_threshold`     | `Option<u32>`      | `AZURE_COSMOS_CB_READ_FAILURE_THRESHOLD`     | Trips after N read failures within the counter reset window.  |
| `write_failure_threshold`    | `Option<u32>`      | `AZURE_COSMOS_CB_WRITE_FAILURE_THRESHOLD`    | Trips after N write failures within the counter reset window. |
| `counter_reset_window`       | `Option<Duration>` | `AZURE_COSMOS_CB_COUNTER_RESET_WINDOW`       | Resets failure counters after this period of no new failures. |
| `failback_interval`          | `Option<Duration>` | `AZURE_COSMOS_CB_FAILBACK_INTERVAL`          | How often the background sweep probes tripped partitions.     |
| `unavailability_probe_delay` | `Option<Duration>` | `AZURE_COSMOS_CB_UNAVAILABILITY_PROBE_DELAY` | Time a partition must be unavailable before the first probe.  |

### 9.5 Integration with Layer Structs

The new option groups nest inside existing groups already aggregated by `CosmosRuntimeOptions`
and `CosmosClientOptions` (§4 of the Configuration Options spec). No new top-level fields are
needed on the layer structs:

```text
CosmosRuntimeOptions / CosmosClientOptions
├── connection: Arc<ConnectionOptions>
│   └── connection_pool: Option<ConnectionPoolOptions>   ← sharding fields added here (§9.1)
├── retry: Arc<RetryOptions>
│   ├── session_retry: Option<SessionRetryOptions>       ← existing
│   ├── hedging: Option<HedgingOptions>                  ← new (§9.3)
│   └── circuit_breaker: Option<CircuitBreakerOptions>   ← new (§9.4)
├── request: Arc<RequestOptions>
├── regions: Arc<RegionOptions>
├── account: Arc<CosmosAccountOptions>
└── quirks: Arc<QuirkOptions>
```

Resolution follows the standard layered walk: **Operation → Account → Runtime → Environment**
(highest to lowest priority). Because sharding, hedging, and circuit-breaker options participate
at the Runtime and Account layers only (not Operation), the effective walk is
**Account → Runtime → Environment**.

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

## 11. Crossbeam & Lock-Free Data Structures

The `crossbeam` crate is available as a workspace dependency (`default-features = false`)
and enabled in `azure_data_cosmos_driver` with the `crossbeam-epoch` and `crossbeam-utils`
features.  This section describes where crossbeam primitives should replace (or augment)
existing synchronization, and the general migration pattern for each case.

### 11.1 Snapshot-Swap State Stores (`RwLock<Arc<T>>` → epoch-guarded atomic pointer)

**Pattern**: Multiple structures in this spec use a `RwLock<Arc<T>>` where the write path
builds a new `T`, wraps it in `Arc`, and swaps the pointer.  The read path acquires a read
lock only to clone the `Arc`.  While `RwLock::read()` is cheap on uncontended paths, under
high concurrency it still performs an atomic CAS on the lock word — and on some platforms
(Windows `SRWLock`) read-locks can starve writers or introduce cache-line bouncing.

**Crossbeam replacement**: Use `crossbeam::epoch::Atomic<T>` (or a thin wrapper around it)
to replace these `RwLock<Arc<T>>` holders.  The read path becomes:

```rust
use std::sync::atomic::Ordering;

use crossbeam::epoch::{self, Atomic, Guard, Owned};

struct StateStore<T> {
    state: Atomic<T>,
}

/// RAII wrapper that keeps the epoch guard alive for as long as the
/// snapshot reference is used.  Derefs to `&T`.
struct Snapshot<'g, T> {
    _guard: Guard,
    ptr: &'g T,
}

impl<T> std::ops::Deref for Snapshot<'_, T> {
    type Target = T;
    fn deref(&self) -> &T {
        self.ptr
    }
}

impl<T> StateStore<T> {
    /// Lock-free snapshot — costs a single atomic load + epoch pin.
    ///
    /// Returns a `Snapshot` that keeps the epoch guard alive.  The
    /// caller can deref into `&T` for as long as it holds the
    /// `Snapshot`.  Do **not** hold the snapshot across `.await`
    /// points — the pinned epoch blocks garbage collection of
    /// retired pointers while alive.
    ///
    /// The concrete `LocationStateStore` (§4.6) clones the inner
    /// value into an `Arc<T>` so the snapshot can outlive the guard;
    /// that is the preferred approach when the data must cross an
    /// await boundary.
    fn snapshot(&self) -> Snapshot<'_, T> {
        let guard = epoch::pin();
        // SAFETY: Writers use `swap`/`apply` + `defer_destroy`, so
        // the pointee is valid for the lifetime of the guard.
        let ptr = unsafe {
            self.state
                .load(Ordering::Acquire, &guard)
                .deref()
        };
        Snapshot { _guard: guard, ptr }
    }

    /// Atomically apply a mutation to the current state (CAS loop).
    ///
    /// Reads the current value, applies `f`, and attempts a
    /// compare-and-swap.  On contention (another writer changed the
    /// state between load and CAS), re-reads and re-applies until
    /// the CAS succeeds — no concurrent mutation is lost.
    ///
    /// Use this when the new state depends on the current state.
    fn apply(&self, mut f: impl FnMut(&T) -> T) {
        let guard = epoch::pin();
        loop {
            let current = self.state.load(Ordering::Acquire, &guard);
            let current_ref = unsafe { current.deref() };
            let new_val = f(current_ref);
            match self.state.compare_exchange(
                current,
                Owned::new(new_val),
                Ordering::AcqRel,
                Ordering::Acquire,
                &guard,
            ) {
                Ok(old) => {
                    unsafe { guard.defer_destroy(old); }
                    return;
                }
                Err(_) => continue, // another writer won — retry
            }
        }
    }

    /// Unconditionally replace the current snapshot.
    ///
    /// Use when the new state does NOT depend on the current state
    /// (e.g., full refresh from an external source).  Concurrent
    /// `apply` calls are safe — their CAS will fail and retry
    /// against the freshly swapped value.
    fn swap(&self, new_state: T) {
        let guard = epoch::pin();
        let old = self.state.swap(
            Owned::new(new_state),
            Ordering::AcqRel,
            &guard,
        );
        unsafe { guard.defer_destroy(old); }
    }
}
```

Alternatively, `arc_swap::ArcSwap<T>` provides a
higher-level API with `load()` returning a `Guard` that derefs to `Arc<T>` - but this has more overhead.
Prefer `Atomic<T>` for minimal overhead when the reader does not need to
outlive the epoch guard, or `ArcSwap` when the caller wants a long-lived `Arc<T>`.

**Already applied to spec structures** (see §4.6, §6.2, §6.3):

| Structure              | Field        | Section | Hot-path?                                  | Notes                            |
| ---------------------- | ------------ | ------- | ------------------------------------------ | -------------------------------- |
| `LocationStateStore`   | `account`    | §4.6    | Yes — every request reads routing state    | `Atomic<AccountEndpointState>`   |
| `LocationStateStore`   | `partitions` | §4.6    | Yes — every request checks circuit-breaker | `Atomic<PartitionEndpointState>` |
| `ShardedHttpTransport` | `pools`      | §6.2    | Yes — every HTTP request looks up the pool | `Atomic<HashMap<…>>`             |
| `EndpointShardPool`    | `shards`     | §6.3    | Yes — every HTTP request selects a shard   | `Atomic<Vec<…>>`                 |
| `EndpointStats`        | all          | §6.3    | Yes — updated per HTTP request             | `CachePadded` per field          |
| `ShardHealth`          | all          | §6.3    | Yes — updated per HTTP request             | `CachePadded` per field          |

**Applies to (existing driver code)**:

| Structure                       | Field                          | File                            | Notes                                                                                                                                                   |
| ------------------------------- | ------------------------------ | ------------------------------- | ------------------------------------------------------------------------------------------------------------------------------------------------------- |
| `AsyncCache<K, V>`              | inner `RwLock<HashMap>`        | `driver/cache/async_cache.rs`   | The outer map is read-heavy; writes only on cache miss. Wrap in `Atomic<HashMap<…>>` or `ArcSwap`. The per-entry `AsyncLazy` also benefits (see §11.2). |
| `AsyncLazy<T>`                  | inner `RwLock<Option<Arc<T>>>` | `driver/cache/async_lazy.rs`    | After initialization the value is essentially immutable. Replace with `Atomic<T>` for truly zero-cost reads post-init. See §11.2.                       |
| `DriverRuntime`                 | `driver_registry`              | `driver/runtime.rs`             | Read-heavy; new drivers created rarely.                                                                                                                 |
| `SharedRuntimeOptions`          | inner `RwLock<RuntimeOptions>` | `options/runtime_options.rs`    | Read on every operation; mutated only by reconfiguration.                                                                                               |
| `ThroughputControlGroupOptions` | mutable-value fields           | `options/throughput_control.rs` | Read per-request for throughput gating; written by background refresh.                                                                                  |
| `CpuMemoryMonitorInner`         | `buffer` / `listener_count`    | `system/cpu_memory.rs`          | Lower priority — the monitor runs on a timer, not per-request. Still benefits from eliminating reader contention on the sample buffer.                  |

### 11.2 `AsyncCache` / `AsyncLazy` — Epoch-Guarded Reads

`AsyncCache` wraps `async_lock::RwLock<HashMap<K, Arc<AsyncLazy<V>>>>`.  The outer map
lock is held for reads on every cache lookup (the hot path) and for writes only on cache
miss.  This is the textbook crossbeam case:

1. **Outer map**: Replace with `crossbeam::epoch::Atomic<HashMap<K, Arc<AsyncLazy<V>>>>`.
   Readers pin the epoch, load the map pointer, do the lookup, and unpin.  Writers clone
   the map, insert the new entry, and `swap` + `defer_destroy` the old map.
2. **`AsyncLazy<T>` inner value**: After the one-time initialization completes, the value
   never changes (or changes only on explicit invalidation).  Use
   `crossbeam::epoch::Atomic<T>` for the read path (load + deref under epoch guard).  The
   initialization path still needs an async mutex for single-flight, but the *read* path
   after init becomes zero-cost.

### 11.3 `CachePadded` for Atomics (False-Sharing Prevention)

`crossbeam::utils::CachePadded<T>` pads `T` to a cache-line boundary, preventing false
sharing when adjacent atomics are updated by different cores.

**Applies to**:

- `EndpointStats` (§6.3): Fields like `inflight: AtomicU32`, `consecutive_failures: AtomicU32`,
  and `total_requests: AtomicU64` are updated concurrently from different request tasks.
  Wrapping each field (or the whole struct) in `CachePadded` eliminates cross-core
  invalidation traffic.
- `ShardHealth` (§6.3): `total_inflight`, `last_success_nanos`, `marked_for_eviction` are
  updated from multiple tasks. `CachePadded` avoids false sharing.
- `CpuMemoryMonitorInner` atomics (`system/cpu_memory.rs`): `last_refresh`, `cached_cpu_usage`,
  `cached_memory_usage` are adjacent `AtomicU64` / `AtomicU32` fields that benefit from
  padding.
- General guideline: Any struct with multiple `Atomic*` fields that are written by
  independent tasks should wrap those fields in `CachePadded`.

### 11.4 Migration Priority

Recommended implementation order based on hot-path impact:

1. **`LocationStateStore`** (§4.6) — unified account + partition state store,
   read on every single Cosmos request.
2. **`EndpointShardPool.shards`** and **`ShardedHttpTransport.pools`** (§6.2, §6.3) —
   read on every HTTP send.
3. **`AsyncCache` / `AsyncLazy`** — read on every operation for container metadata lookup.
4. **`CachePadded` on `EndpointStats` / `ShardHealth`** — reduces cache-line bouncing
   under high concurrency.
5. **`DriverRuntime.driver_registry`**, **`SharedRuntimeOptions`**,
   **`ThroughputControlGroupOptions`** — moderate impact; operations read these for
   configuration but contention is lower because the data is small.
6. **`CpuMemoryMonitorInner`** — lowest priority; timer-driven, not per-request.

### 11.5 Crossbeam Feature Selection

The driver crate enables only the features it needs:

```toml
# sdk/cosmos/azure_data_cosmos_driver/Cargo.toml
[dependencies]
crossbeam = { workspace = true, features = ["crossbeam-epoch"] }
```

- **`crossbeam-epoch`** (optional dep): Provides `Atomic<T>`, epoch-based memory reclamation,
  `Guard`, and `Owned` / `Shared` pointer types.  Used for all snapshot-swap state stores.
- **`crossbeam-utils`** (always included): Provides `CachePadded<T>` for false-sharing prevention.
  Not listed as a feature because it is a non-optional dependency of `crossbeam`.

Other crossbeam features (`crossbeam-channel`, `crossbeam-deque`, `crossbeam-queue`) are
**not** enabled — the driver uses runtime-agnostic primitives and `azure_core` abstractions
for async coordination and does not need lock-free work-stealing queues at this time.

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
│   │   ├── transport_pipeline.rs # execute_transport_pipeline fn (refactored)
│   │   ├── adaptive_transport.rs # NEW: AdaptiveTransport (H2/H1.1 dispatch)
│   │   ├── http_client_factory.rs# NEW: HttpClientFactory trait + default impl
│   │   ├── cosmos_headers.rs     # apply_cosmos_headers bare fn
│   │   ├── request_signing.rs    # sign_request bare fn
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
│   ├── connection_pool.rs        # Updated with sharding fields on ConnectionPoolOptions (§9.1)
│   ├── driver_options.rs
│   ├── operation_options.rs      # Unchanged (hedging/CB in RetryOptions §9.2–§9.4)
│   ├── retry.rs                  # NEW: RetryOptions
│   └── availability.rs           # NEW: CircuitBreakerOptions, HedgingOptions
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

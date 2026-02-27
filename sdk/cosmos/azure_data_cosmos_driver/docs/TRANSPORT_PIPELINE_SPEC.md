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
│  │  ┌─ Region selection (GlobalEndpointManager)                          │  │
│  │  ├─ Hedging (parallel speculative execution in secondary region)      │  │
│  │  ├─ Cross-region failover (503/WriteForbidden/SessionNotAvailable)    │  │
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
    /// The resolved regional endpoint URL.
    pub endpoint: Url,
    /// The region name (for diagnostics and failover tracking).
    pub region: RegionName,
    /// Whether this is the hub (default) endpoint or a regional endpoint.
    pub endpoint_kind: EndpointKind,
    /// Whether partition-level override was applied.
    pub partition_override: Option<PartitionOverride>,
}

pub(crate) enum EndpointKind {
    Hub,
    Regional,
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
    pub endpoint: Url,
    pub region: RegionName,
    pub error: OperationError,
    pub request_sent: bool,
}

/// COMPONENT: Diagnostics accumulator for the operation.
/// Append-only across attempts.
// Reuse existing DiagnosticsContextBuilder (already mutable accumulator).

/// COMPONENT: Session state for consistency tracking.
pub(crate) struct SessionState {
    pub session_token: Option<String>,
    pub consistency_level: Option<ConsistencyLevel>,
    /// LSN tracking for session consistency.
    pub quorum_selected_lsn: Option<i64>,
    pub global_committed_selected_lsn: Option<i64>,
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
    pub deadline: Option<Instant>,
}

/// COMPONENT: Transport-level retry state (for 429 throttling).
/// Tracked per-attempt, not shared across operation-level retries.
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
        status: StatusCode,
        headers: Headers,
        body: Vec<u8>,
    },
    /// Failed with an HTTP error that may be retryable at operation level.
    HttpError {
        status: StatusCode,
        sub_status: Option<u32>,
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
fn resolve_endpoint(
    operation: &CosmosOperation,
    retry_state: &OperationRetryState,
    endpoint_manager: &GlobalEndpointManager,
    partition_manager: &GlobalPartitionEndpointManager,
) -> RoutingDecision;

// SYSTEM: Decide what to do after a transport attempt completes.
fn evaluate_operation_retry(
    operation: &CosmosOperation,
    result: &TransportResult,
    retry_state: &OperationRetryState,
    session_state: &SessionState,
) -> OperationAction;

// SYSTEM: Decide whether to retry a 429 at transport level.
fn evaluate_throttle_retry(
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
    endpoint_manager: &GlobalEndpointManager,
    partition_manager: &GlobalPartitionEndpointManager,
    transport: &TransportPipeline,
    diagnostics: &mut DiagnosticsContextBuilder,
) -> Result<CosmosResponse> {
    let mut retry_state = OperationRetryState::initial(operation, options, endpoint_manager);
    let mut session_state = SessionState::from_operation(operation, options);
    let deadline = options.e2e_deadline();

    loop {
        // STAGE 1: Resolve endpoint for this attempt
        let routing = resolve_endpoint(
            operation,
            &retry_state,
            endpoint_manager,
            partition_manager,
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
        let action = evaluate_operation_retry(
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
            OperationAction::PartitionFailover { new_state } => {
                partition_manager.mark_partition_unavailable(/* ... */);
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
                    diagnostics.record_attempt_primary(&primary_result);
                    if primary_result.is_success() {
                        // Cancel secondary (drop future)
                        evaluate_and_return(operation, primary_result, diagnostics)
                    } else {
                        // Wait for secondary
                        let secondary_result = secondary_fut.await;
                        diagnostics.record_attempt_hedged(&secondary_result);
                        pick_best_result(primary_result, secondary_result, diagnostics)
                    }
                }
                secondary_result = secondary_fut => {
                    diagnostics.record_attempt_hedged(&secondary_result);
                    if secondary_result.is_success() {
                        // Cancel primary (drop future)
                        evaluate_and_return(operation, secondary_result, diagnostics)
                    } else {
                        // Wait for primary
                        let primary_result = primary_fut.await;
                        diagnostics.record_attempt_primary(&primary_result);
                        pick_best_result(primary_result, secondary_result, diagnostics)
                    }
                }
            }
        }
    }
}
```

**Key hedging rules**:
- Only for read-only operations (or idempotent operations if configured)
- Threshold is configurable per-operation (default: 50ms, or dynamic based on P99 latency)
- Both attempts are tracked in `DiagnosticsContext` with `ExecutionContext::Hedging`
- The hedged attempt's RU charge is always reported regardless of which wins

### 4.3 `evaluate_operation_retry` Decision Tree

This pure function replaces the complex `ClientRetryPolicy::should_retry` method. Because it is a
pure function over data components, it is trivially unit-testable.

```rust
fn evaluate_operation_retry(
    operation: &CosmosOperation,
    result: &TransportResult,
    retry_state: &OperationRetryState,
    session_state: &SessionState,
) -> OperationAction {
    match &result.outcome {
        TransportOutcome::Success { .. } => {
            OperationAction::Complete(result.clone())
        }

        TransportOutcome::HttpError { status, sub_status, request_sent, .. } => {
            match (*status, *sub_status) {
                // 403.3 WriteForbidden → endpoint failover + cache refresh
                (403, Some(3)) => {
                    if retry_state.failover_retry_count < MAX_FAILOVER_RETRIES {
                        OperationAction::FailoverRetry {
                            new_state: retry_state.advance_failover(result),
                            delay: None,
                        }
                    } else {
                        OperationAction::Abort(build_error(status, sub_status))
                    }
                }

                // 404.1022 ReadSessionNotAvailable → session retry
                (404, Some(1022)) => {
                    if retry_state.session_token_retry_count < MAX_SESSION_RETRIES {
                        OperationAction::SessionRetry {
                            new_state: retry_state.advance_session_retry(),
                        }
                    } else {
                        OperationAction::Abort(build_error(status, sub_status))
                    }
                }

                // 429.3092 SystemResourceUnavailable → treat as 503
                (429, Some(3092)) | (503, _) | (410, Some(_)) => {
                    OperationAction::PartitionFailover {
                        new_state: retry_state.advance_partition_failover(result),
                    }
                }

                // 500 for reads → try next endpoint
                (500, _) if operation.is_read_only() => {
                    OperationAction::FailoverRetry {
                        new_state: retry_state.advance_failover(result),
                        delay: None,
                    }
                }

                _ => OperationAction::Abort(build_error(status, sub_status)),
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

### 4.4 GlobalEndpointManager & LocationCache

These are moved from `azure_data_cosmos` into `azure_data_cosmos_driver` (under
`driver/routing/`). The design remains similar but the mutable `LocationCache` behind a `Mutex`
gets replaced with an immutable-swap pattern:

```rust
/// Thread-safe endpoint routing. Uses RwLock with immutable snapshots.
pub(crate) struct GlobalEndpointManager {
    /// Current snapshot of location info. Replaced atomically on refresh.
    locations: RwLock<Arc<LocationSnapshot>>,
    /// Account properties cache (TTL-based).
    account_cache: AsyncCache<String, AccountProperties>,
    /// Default (hub) endpoint.
    default_endpoint: Url,
}

/// Immutable snapshot of all location routing state.
/// Replaced as a whole when account properties refresh.
struct LocationSnapshot {
    preferred_read_endpoints: Vec<RegionalEndpoint>,
    preferred_write_endpoints: Vec<RegionalEndpoint>,
    unavailable_endpoints: HashMap<Url, (Instant, UnavailableReason)>,
    account_properties: AccountProperties,
}

struct RegionalEndpoint {
    region: RegionName,
    endpoint: Url,
}
```

**Key difference from current design**: Instead of mutating fields on `LocationCache` behind a
`Mutex`, we create a new `LocationSnapshot` and swap it atomically via `RwLock<Arc<_>>`.
This makes the read path lock-free (clone the `Arc`) and mutations are serialized through the
write lock with snapshot replacement.

### 4.5 GlobalPartitionEndpointManager

Moved from `azure_data_cosmos`. Same circuit-breaker logic with per-partition failover tracking.
Background failback loop via `BackgroundTaskManager`.

**Key circuit-breaker thresholds** (from Java SDK reference):
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
3. Transport-level retry (429 throttling ONLY)
4. Tracking request-sent-status for retry safety
5. Recording per-attempt diagnostics events
6. Enforcing end-to-end deadline

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
            // Check deadline
            if let Some(deadline) = request.deadline {
                if Instant::now() >= deadline {
                    return TransportResult::deadline_exceeded(diagnostics);
                }
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

            // Check for 429 throttling → transport-level retry
            if let TransportOutcome::HttpError { status: 429, .. } = &result.outcome {
                let action = evaluate_throttle_retry(&result, &throttle_state);
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

| Gateway | Usage | HTTP Version | Notes |
|---------|-------|-------------|-------|
| **RoutingGateway** | Metadata + Dataplane | HTTP/1.1 only | Legacy, no ALPN |
| **ComputeGateway** | Metadata + Dataplane | HTTP/2 via ALPN | Preferred for metadata |
| **Gateway 2.0** | Dataplane only | HTTP/2 forced (different port) | No HTTP/1.1 fallback |

At `CosmosClient` initialization, when fetching account properties via the metadata transport,
the driver **probes the gateway's protocol support** (via ALPN negotiation or the well-known
Gateway 2.0 port). This determines the transport strategy for the lifetime of the client:

- **HTTP/2 detected** (ComputeGateway or Gateway 2.0): Use `ShardedHttpTransport` (§6.1+)
- **HTTP/1.1 only** (RoutingGateway): Use a plain `Arc<dyn HttpClient>` — no sharding needed
  since HTTP/1.1 uses one-request-per-connection and the underlying client already manages a
  connection pool via `pool_max_idle_per_host`.

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

When operating against an HTTP/2 gateway, the server enforces a strict limit of **20 concurrent
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
    /// Limits total connection fan-out. Default: 32.
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
struct EndpointShardPool {
    endpoint: EndpointKey,
    config: ShardingConfig,
    /// The shards (each wrapping an HttpClient).
    shards: RwLock<Vec<Arc<ClientShard>>>,
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

        // All active shards at capacity → try to create a new one
        drop(shards); // release read lock
        if let Some(new_shard) = self.try_create_shard() {
            return new_shard;
        }

        // At max clients → fall back to least-loaded across ALL shards
        // (including draining ones) to prevent request queuing
        let shards = self.shards.read();
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
  existing `CpuMemoryMonitor`
- Java has transit-timeout counting; we use consecutive-failure counting as a proxy since the
  `HttpClient` trait doesn't expose per-stream timeout events separately

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

Fault injection is integrated at the **HTTP transport level**, below the transport pipeline's retry
logic but above actual HTTP I/O. This ensures injected faults trigger the full retry/failover
pipeline just like real service errors.

```text
  TransportPipeline
       │
       ▼
  ┌─────────────────────────────┐
  │  FaultInjectionLayer        │  ← evaluates rules BEFORE sending
  │  (optional, feature-gated)  │
  └─────────────┬───────────────┘
                │
                ▼
  ┌─────────────────────────────┐
  │  AdaptiveTransport          │  ← Sharded (H2) or Plain (H1.1)
  └─────────────────────────────┘
```

### 7.2 Fault Injection in the Driver

The existing `azure_data_cosmos` fault injection model (`FaultInjectionRule`, `FaultInjectionCondition`,
`FaultInjectionResult`) is moved into `azure_data_cosmos_driver` behind a `fault_injection` feature
flag.

```rust
/// Wraps AdaptiveTransport with fault injection capability.
pub(crate) struct FaultInjectionTransport {
    inner: AdaptiveTransport,
    rules: Arc<RwLock<Vec<FaultInjectionRule>>>,
}

impl FaultInjectionTransport {
    pub async fn send(&self, request: &Request) -> Result<AsyncRawResponse> {
        // Check rules
        if let Some(fault) = self.evaluate_rules(request) {
            return fault.apply();
        }
        self.inner.send(request).await
    }
}
```

The existing `FaultClient` pattern (wrapping `HttpClient`) maps cleanly to this design. The key
difference is that it wraps `AdaptiveTransport` instead of a raw `HttpClient`.

---

## 8. Migration Plan (Step 1 → Step 2)

### Step 1: Build in `azure_data_cosmos_driver`

| Phase | Work Item | New Modules/Files |
|-------|-----------|-------------------|
| 1a | **Routing** — Move `GlobalEndpointManager`, `LocationCache`, `GlobalPartitionEndpointManager` | `driver/routing/mod.rs`, `global_endpoint_manager.rs`, `location_cache.rs`, `partition_endpoint_manager.rs` |
| 1b | **Operation pipeline** — Implement `execute_operation_pipeline`, `evaluate_operation_retry`, hedging | `driver/pipeline/mod.rs`, `operation_pipeline.rs`, `hedging.rs`, `retry_evaluation.rs` |
| 1c | **Transport pipeline** — Implement `TransportPipeline`, throttle retry | `driver/transport/transport_pipeline.rs` (refactor existing `pipeline.rs`) |
| 1d | **Adaptive transport** — Implement `AdaptiveTransport`, `ShardedHttpTransport`, gateway probing | `driver/transport/adaptive_transport.rs`, `sharded_transport.rs`, `shard_pool.rs`, `shard_health.rs` |
| 1e | **State components** — Define all ECS-style component types | `driver/pipeline/components.rs` |
| 1f | **Fault injection** — Move from `azure_data_cosmos`, integrate with sharded transport | `driver/fault_injection/mod.rs`, `rule.rs`, `condition.rs`, `result.rs` |
| 1g | **Session tracking** — Session token management | `driver/routing/session_manager.rs` |
| 1h | **Update `execute_operation`** — Wire `CosmosDriver::execute_operation` to use the operation pipeline | Modify `driver/cosmos_driver.rs` |
| 1i | **Config surface** — Add `ShardingConfig`, hedging threshold, circuit breaker configs to options | Modify `options/connection_pool.rs`, add `options/retry.rs`, `options/availability.rs` |
| 1j | **Testing** — Unit tests for each pure function + integration tests | `tests/` |

### Step 2: Cut Over `azure_data_cosmos`

| Phase | Work Item |
|-------|-----------|
| 2a | Replace `GatewayPipeline` + `BackOffRetryHandler` with `driver.execute_operation()` calls |
| 2b | Remove `azure_data_cosmos/src/pipeline/` |
| 2c | Remove `azure_data_cosmos/src/retry_policies/` |
| 2d | Remove `azure_data_cosmos/src/handler/` |
| 2e | Remove `azure_data_cosmos/src/routing/` (endpoint managers) |
| 2f | Remove `azure_data_cosmos/src/request_context.rs` |
| 2g | Move fault injection tests to use driver-level APIs |
| 2h | Update `azure_data_cosmos/src/clients/` to build `CosmosOperation` and call driver |

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
/// Hedging threshold: if the primary attempt has not completed
/// within this duration, start a hedged attempt in a secondary region.
/// None = hedging disabled. Default: None (opt-in).
pub hedging_threshold: Option<Duration>,

/// Circuit breaker configuration for partition-level failover.
pub circuit_breaker: CircuitBreakerConfig,
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

| Variable | Config Field |
|----------|-------------|
| `AZURE_COSMOS_MAX_STREAMS_PER_CLIENT` | `sharding.max_streams_per_client` |
| `AZURE_COSMOS_MAX_CLIENTS_PER_ENDPOINT` | `sharding.max_clients_per_endpoint` |
| `AZURE_COSMOS_SHARD_IDLE_TIMEOUT_SECS` | `sharding.idle_client_timeout` |
| `AZURE_COSMOS_HEDGING_THRESHOLD_MS` | `hedging_threshold` |

---

## 10. Open Questions

1. **Hedging scope**: Should hedging be opt-in or default-enabled? The Java SDK uses
   `ThresholdBasedAvailabilityStrategy` which is opt-in. The design principles doc suggests
   force-enabling. What's the right default for the Rust driver?

2. **Dynamic hedging threshold**: Should the hedging threshold be static (configured) or dynamic
   (based on observed P99 latency)? Dynamic is more robust but adds complexity.

3. **Sharding granularity**: Should `ShardedHttpTransport` track inflight per
   `(HttpClient, endpoint)` pair (proposed) or per `HttpClient` globally? Per-endpoint
   gives more precise load balancing but more bookkeeping.

4. **Transport pipeline vs. policy chain**: The spec proposes replacing the `Policy` chain with
   direct function calls. This is simpler but loses extensibility. Is that acceptable given the
   fixed nature of Cosmos policies?

5. **HTTP/2 stream limit detection**: Can we detect when the underlying `HttpClient` implementation
   is actually hitting the H2 stream limit? Or do we rely purely on the configured
   `max_streams_per_client` threshold? The `HttpClient` trait doesn't expose H2 frame-level events.

6. **CPU-aware eviction**: The Java health checker disables eviction under high CPU (to avoid
   making things worse). Should we adopt this using `CpuMemoryMonitor`? What CPU threshold?

7. ~~**Metadata vs. dataplane sharding**~~: **Resolved** — both metadata and dataplane pipelines
   use `AdaptiveTransport`, which automatically selects sharded or plain transport based on the
   gateway's protocol. If the metadata endpoint supports HTTP/2 (ComputeGateway), it gets
   sharding. If HTTP/1.1 only (RoutingGateway), it uses plain transport. Different timeout
   configs are passed via `HttpClientConfig`.

8. **`HttpClient` reuse across endpoints**: A single `HttpClient` implementation might maintain
   connections to multiple endpoints. Should we use one `HttpClient` per endpoint (simpler
   health tracking) or share clients across endpoints (fewer client instances)?

9. **`HttpClientFactory` abstraction level**: The proposed `HttpClientFactory` trait takes an
   `HttpClientConfig` struct with pool/timeout/TLS knobs. Should these knobs be extended to
   cover HTTP version preferences (force H2, prefer H1.1, etc.)? Or is that determined
   externally by the gateway probing logic?

10. **`load_spread_ratio` tuning**: The default of 0.5 means only half of available shards are
    in the active set. Is this too aggressive (causes premature scale-down during variable load)
    or too conservative (holds onto shards too long)? Should it be adaptive based on observed
    request rate variance?

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
│   │   └── retry_evaluation.rs   # Pure fn: evaluate_operation_retry
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

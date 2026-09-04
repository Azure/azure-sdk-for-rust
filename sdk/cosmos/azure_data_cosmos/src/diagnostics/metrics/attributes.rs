// Copyright (c) Microsoft Corporation. All rights reserved.
// Licensed under the MIT License.

//! Centralized metric- and attribute-name string literals for Cosmos DB OTel
//! metrics.
//!
//! The OpenTelemetry semantic conventions for database clients define the exact
//! metric and attribute names emitted here. Metric names and instrument units are
//! metrics-specific and defined here; the shared **attribute** names are re-exported
//! from the crate-internal `diagnostics::attributes` module so metrics and tracing
//! can't drift.
//!
//! Two tiers, matching the semantic conventions:
//! - **Stable** names — emitted unconditionally (operation-scope, low cardinality).
//! - **Optional** names — emitted only when the matching
//!   [`MetricsOptions`](super::MetricsOptions) toggle is opted into.

use crate::diagnostics::attributes;

// =========================================================================
// Metric names
// =========================================================================

/// Stable histogram (seconds): total client-observed duration of an operation.
///
/// This is the primary Cosmos metric — the one to graph for latency SLOs.
pub const METRIC_OPERATION_DURATION: &str = "db.client.operation.duration";

/// Optional histogram (request units): request charge (RU) for an operation.
pub const METRIC_OPERATION_REQUEST_CHARGE: &str = "azure.cosmosdb.client.operation.request_charge";

/// Optional histogram (rows): number of rows/items returned by an operation.
pub const METRIC_RESPONSE_RETURNED_ROWS: &str = "db.client.response.returned_rows";

/// Optional up-down counter (instances): number of live
/// [`CosmosMetricsHandler`](super::CosmosMetricsHandler) instances (one per
/// instrumented client, under the intended one-handler-per-client registration).
pub const METRIC_ACTIVE_INSTANCE_COUNT: &str = "azure.cosmosdb.client.active_instance.count";

/// Optional counter (operations): number of operations that dispatched a
/// cross-region hedge fan-out.
pub const METRIC_OPERATION_HEDGED: &str = "azure.cosmosdb.client.operation.hedged";

// =========================================================================
// Instrument units
// =========================================================================
//
// Unit strings follow the Unified Code for Units of Measure (UCUM):
// <https://ucum.org/ucum>. OpenTelemetry adopts UCUM for instrument units; see
// <https://opentelemetry.io/docs/specs/semconv/general/metrics/#instrument-units>.

/// Unit for [`METRIC_OPERATION_DURATION`] — seconds.
pub const UNIT_SECONDS: &str = "s";

/// Unit for [`METRIC_OPERATION_REQUEST_CHARGE`] — Cosmos request units.
pub const UNIT_REQUEST_UNIT: &str = "{request_unit}";

/// Unit for [`METRIC_RESPONSE_RETURNED_ROWS`] — rows.
pub const UNIT_ROW: &str = "{row}";

/// Unit for [`METRIC_ACTIVE_INSTANCE_COUNT`] — client instances.
pub const UNIT_INSTANCE: &str = "{instance}";

/// Unit for [`METRIC_OPERATION_HEDGED`] — operations.
pub const UNIT_OPERATION: &str = "{operation}";

// =========================================================================
// Explicit histogram bucket boundaries
// =========================================================================
//
// These MUST be set on every histogram we create. OpenTelemetry's default
// boundaries are `[0, 5, 10, 25, 50, 75, 100, 250, 500, 750, 1000, 2500, 5000,
// 7500, 10000]`, which are scaled for *milliseconds*. `db.client.operation.duration`
// is recorded in *seconds*, so a typical few-millisecond Cosmos operation
// (0.003 s) lands in the very first bucket and every observation piles up there.
// A histogram whose observations all share one bucket carries no information:
// `histogram_quantile` degenerates to linear interpolation within that bucket
// and returns a constant that depends only on the requested quantile, not on
// the data. Latency percentiles then look plausible while being unable to
// register any change at all — so latency dashboards and alerts silently stop
// working rather than visibly breaking.
//
// The boundaries below come from the OpenTelemetry semantic conventions'
// per-instrument bucket *advice*, which exists for exactly this reason. Using
// the advised values (rather than hand-picked ones) also keeps these histograms
// directly comparable with the other Azure Cosmos DB SDKs.

/// Bucket boundaries for [`METRIC_OPERATION_DURATION`], in seconds.
///
/// Semconv advice for `db.client.operation.duration`. Spans sub-millisecond
/// (cache/emulator) through 10 s (a badly degraded or retried request).
pub const BUCKETS_OPERATION_DURATION_SECONDS: &[f64] =
    &[0.001, 0.005, 0.01, 0.05, 0.1, 0.5, 1.0, 5.0, 10.0];

/// Bucket boundaries for [`METRIC_OPERATION_REQUEST_CHARGE`], in request units.
///
/// Cosmos-specific, so there is no semconv advice to follow. Chosen to give
/// resolution where Cosmos operations actually sit: a point read is ~1 RU and a
/// small write ~5-10 RU, so the low end is finely divided, while cross-partition
/// queries reaching into the thousands still land in a meaningful bucket rather
/// than overflowing.
pub const BUCKETS_REQUEST_CHARGE_RU: &[f64] = &[
    1.0, 2.5, 5.0, 10.0, 25.0, 50.0, 100.0, 250.0, 500.0, 1000.0, 2500.0, 5000.0,
];

/// Bucket boundaries for [`METRIC_RESPONSE_RETURNED_ROWS`], in rows.
///
/// Semconv advice for `db.client.response.returned_rows`.
pub const BUCKETS_RETURNED_ROWS: &[f64] = &[
    1.0,
    10.0,
    100.0,
    1000.0,
    10000.0,
    100_000.0,
    1_000_000.0,
    10_000_000.0,
];

// =========================================================================
// Stable attributes (always emitted; operation scope, low cardinality)
//
// These alias the shared semconv literals in `crate::diagnostics::attributes`
// so the string lives in exactly one place.
// =========================================================================

/// `db.system.name` — identifies the database system.
pub const ATTR_DB_SYSTEM_NAME: &str = attributes::DB_SYSTEM_NAME;

/// Value for [`ATTR_DB_SYSTEM_NAME`] on every Cosmos metric.
pub const DB_SYSTEM_NAME_VALUE: &str = attributes::DB_SYSTEM_NAME_VALUE;

/// `db.operation.name` — canonical operation name (e.g. `read_item`).
pub const ATTR_DB_OPERATION_NAME: &str = attributes::DB_OPERATION_NAME;

/// `db.collection.name` — the container name.
pub const ATTR_DB_COLLECTION_NAME: &str = attributes::DB_COLLECTION_NAME;

/// `db.namespace` — the database name.
pub const ATTR_DB_NAMESPACE: &str = attributes::DB_NAMESPACE;

/// `db.response.status_code` — the HTTP status code of the response.
pub const ATTR_DB_RESPONSE_STATUS_CODE: &str = attributes::DB_RESPONSE_STATUS_CODE;

/// `error.type` — present only when the operation failed.
pub const ATTR_ERROR_TYPE: &str = attributes::ERROR_TYPE;

/// `server.address` — host of the contacted endpoint.
pub const ATTR_SERVER_ADDRESS: &str = attributes::SERVER_ADDRESS;

/// `server.port` — port of the contacted endpoint.
///
/// Conditionally required: emitted only when the endpoint uses a non-default
/// port (i.e. anything other than 443 for HTTPS).
pub const ATTR_SERVER_PORT: &str = attributes::SERVER_PORT;

/// Fallback value for [`ATTR_ERROR_TYPE`] when the error is otherwise unknown
/// (per semantic conventions).
pub const ERROR_TYPE_OTHER: &str = attributes::ERROR_TYPE_OTHER;

// =========================================================================
// Extended attributes (opt-in; may be higher cardinality)
// =========================================================================

/// `azure.cosmosdb.consistency.level` — effective consistency level.
pub const ATTR_CONSISTENCY_LEVEL: &str = attributes::CONSISTENCY_LEVEL;

/// `azure.cosmosdb.operation.contacted_regions` — regions contacted.
pub const ATTR_CONTACTED_REGIONS: &str = attributes::CONTACTED_REGIONS;

/// `azure.cosmosdb.response.sub_status_code` — Cosmos sub-status code.
pub const ATTR_SUB_STATUS_CODE: &str = attributes::SUB_STATUS_CODE;

/// `azure.cosmosdb.connection.mode` — gateway vs. direct connection mode.
pub const ATTR_CONNECTION_MODE: &str = attributes::CONNECTION_MODE;

/// `azure.cosmosdb.operation.hedge_terminal_state` — how the hedging race ended.
/// Low cardinality (one value per terminal state), attached to the hedged
/// counter unconditionally.
pub const ATTR_HEDGE_TERMINAL_STATE: &str = attributes::HEDGE_TERMINAL_STATE;

/// `azure.cosmosdb.operation.hedge_region` — the alternate hedge region. Higher
/// cardinality, so attached only under the extended-attribute opt-in.
pub const ATTR_HEDGE_REGION: &str = attributes::HEDGE_REGION;
